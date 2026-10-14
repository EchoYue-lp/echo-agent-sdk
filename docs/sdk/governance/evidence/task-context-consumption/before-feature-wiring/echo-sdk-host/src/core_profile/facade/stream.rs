//! Pull-based facade streams over the unified [`HandleRegistry`].
//!
//! Framework stream producers run against their existing Rust authority and
//! publish into a capacity-one queue. An SDK `stream.next` request consumes
//! one item, which provides transport backpressure without buffering an
//! unbounded response. Handle identity, owner, generation, sequence,
//! cancellation and close state remain exclusively in `HandleRegistry`;
//! this module stores only the live receiver and its producer task.

use echo_sdk_protocol::error::{EchoSdkError, ExtensionErrorCode, Retryability};
use echo_sdk_protocol::handle::WireHandle;
use echo_sdk_protocol::methods::FeatureOperationRequest;
use echo_sdk_protocol::scalar::{WireField, WireU64, WireValue};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use super::super::handles::HandleRegistry;
use super::super::wire;

const STREAM_EVENT_TYPE_ID: &str = "echo_sdk::FacadeStreamEvent";

pub(crate) type FacadeStreamItem = Result<WireValue, EchoSdkError>;

pub(crate) struct FacadeStreamRecord {
    owner_session: String,
    family: String,
    resource_id: String,
    close_resource_on_finish: bool,
    abort_on_close_deadline: bool,
    receiver: tokio::sync::Mutex<tokio::sync::mpsc::Receiver<FacadeStreamItem>>,
    cancel: tokio_util::sync::CancellationToken,
    background: Mutex<Option<tokio::task::AbortHandle>>,
    // The collector retains the producer's actual abnormal exit. This is
    // transport completion evidence, never a framework Run terminal state.
    producer_failure: Mutex<Option<tokio::task::JoinError>>,
    // Received failure evidence stays owned here until its terminal is
    // actually returned. A dropped pull waiter must not discard that cause.
    pending_failure: Mutex<Option<EchoSdkError>>,
    settled: std::sync::atomic::AtomicBool,
    settled_notify: tokio::sync::Notify,
}

pub(crate) struct FacadeStreamProducer {
    pub handle: WireHandle,
    pub sender: tokio::sync::mpsc::Sender<FacadeStreamItem>,
    /// Semantic cancellation shared with the registry's state commit.
    pub cancel: tokio_util::sync::CancellationToken,
    /// Local producer shutdown also wakes on parent cancellation, while
    /// cleanup after a source failure never rewrites its terminal cause.
    pub cleanup: tokio_util::sync::CancellationToken,
    record: Arc<FacadeStreamRecord>,
}

#[cfg(test)]
type TerminalCommitBarriers = (Arc<std::sync::Barrier>, Arc<std::sync::Barrier>);

pub(crate) struct FacadeStreamRuntime {
    streams: Mutex<HashMap<String, Arc<FacadeStreamRecord>>>,
    max_open_streams: usize,
    shutdown_timeout: std::time::Duration,
    #[cfg(test)]
    terminal_commit_barriers: Mutex<Option<TerminalCommitBarriers>>,
}

impl FacadeStreamRuntime {
    pub fn new(max_open_streams: usize, shutdown_timeout_secs: u64) -> Self {
        Self {
            streams: Mutex::new(HashMap::new()),
            max_open_streams,
            shutdown_timeout: std::time::Duration::from_secs(shutdown_timeout_secs),
            #[cfg(test)]
            terminal_commit_barriers: Mutex::new(None),
        }
    }

    pub fn open(
        &self,
        handles: &HandleRegistry,
        resource: &WireHandle,
        owner_session: &str,
        operation: &str,
    ) -> Result<FacadeStreamProducer, EchoSdkError> {
        self.open_with_ownership(handles, resource, owner_session, operation, false, true)
    }

    /// Channel producers retain a framework callback/driver owner after a
    /// local task abort; only their actual cooperative exit proves cleanup.
    pub fn open_cooperative(
        &self,
        handles: &HandleRegistry,
        resource: &WireHandle,
        owner_session: &str,
        operation: &str,
    ) -> Result<FacadeStreamProducer, EchoSdkError> {
        self.open_with_ownership(handles, resource, owner_session, operation, false, false)
    }

    /// Open a stream whose facade resource exists only to anchor this stream.
    /// Natural terminal or explicit close tombstones both handles together.
    #[cfg(feature = "sdk-extension-bridge")]
    pub fn open_ephemeral(
        &self,
        handles: &HandleRegistry,
        resource: &WireHandle,
        owner_session: &str,
        operation: &str,
    ) -> Result<FacadeStreamProducer, EchoSdkError> {
        self.open_with_ownership(handles, resource, owner_session, operation, true, true)
    }

    /// External Sandbox callbacks may outlive the local producer task just
    /// like Channel handlers. Keep the ephemeral address until true exit.
    #[cfg(feature = "sdk-extension-bridge")]
    pub fn open_ephemeral_cooperative(
        &self,
        handles: &HandleRegistry,
        resource: &WireHandle,
        owner_session: &str,
        operation: &str,
    ) -> Result<FacadeStreamProducer, EchoSdkError> {
        self.open_with_ownership(handles, resource, owner_session, operation, true, false)
    }

    fn open_with_ownership(
        &self,
        handles: &HandleRegistry,
        resource: &WireHandle,
        owner_session: &str,
        operation: &str,
        close_resource_on_finish: bool,
        abort_on_close_deadline: bool,
    ) -> Result<FacadeStreamProducer, EchoSdkError> {
        let resource_record = handles.facade_resource(resource, operation)?;
        let handle = handles.register_facade_stream(
            self.max_open_streams,
            resource,
            Some(owner_session),
            operation,
        )?;
        let (sender, receiver) = tokio::sync::mpsc::channel(1);
        let cancel = handles
            .facade_stream(&handle, Some(owner_session), operation)?
            .facade_cancellation
            .clone()
            .ok_or_else(|| {
                wire::sdk_error(
                    ExtensionErrorCode::FrameworkError,
                    "facade stream lacks its original cancellation authority",
                    Retryability::Never,
                    operation,
                )
            })?;
        let cleanup = cancel.child_token();
        let record = Arc::new(FacadeStreamRecord {
            owner_session: owner_session.to_string(),
            family: resource_record.family.clone(),
            resource_id: resource.id.clone(),
            close_resource_on_finish,
            abort_on_close_deadline,
            receiver: tokio::sync::Mutex::new(receiver),
            cancel: cleanup.clone(),
            background: Mutex::new(None),
            producer_failure: Mutex::new(None),
            pending_failure: Mutex::new(None),
            settled: std::sync::atomic::AtomicBool::new(true),
            settled_notify: tokio::sync::Notify::new(),
        });
        self.streams
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .insert(handle.id.clone(), record.clone());
        Ok(FacadeStreamProducer {
            handle,
            sender,
            cancel,
            cleanup,
            record,
        })
    }

    pub fn attach_background(
        &self,
        producer: &FacadeStreamProducer,
        background: tokio::task::JoinHandle<()>,
    ) {
        producer
            .record
            .settled
            .store(false, std::sync::atomic::Ordering::Release);
        *producer
            .record
            .background
            .lock()
            .unwrap_or_else(|e| e.into_inner()) = Some(background.abort_handle());
        let record = producer.record.clone();
        // The collector is the sole JoinHandle owner. Close may be cancelled
        // without detaching the producer or losing settlement visibility.
        tokio::spawn(async move {
            if let Err(error) = background.await {
                *record
                    .producer_failure
                    .lock()
                    .unwrap_or_else(|error| error.into_inner()) = Some(error);
            }
            record
                .settled
                .store(true, std::sync::atomic::Ordering::Release);
            record.settled_notify.notify_waiters();
        });
    }

    pub async fn next(
        &self,
        handles: &HandleRegistry,
        stream: &WireHandle,
        owner_session: &str,
        family: &str,
        operation: &str,
    ) -> Result<WireValue, EchoSdkError> {
        let record = self.record(handles, stream, owner_session, family, operation)?;
        // Serialise all pulls for this stream. The registry sequence read,
        // dequeue and sequence/terminal transition stay inside this one
        // consumer critical section, so concurrent `next` calls cannot remove
        // two items using the same pre-dequeue watermark.
        let mut receiver = record.receiver.lock().await;
        let (last_sequence, _) =
            handles.facade_stream_state(stream, Some(owner_session), operation)?;
        let pending = record
            .pending_failure
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .clone();
        let mut item = match pending {
            Some(error) => Some(Err(error)),
            None => tokio::select! {
                item = receiver.recv() => item,
                () = record.cancel.cancelled() => None,
            },
        };
        if item.is_none() {
            // Queue EOF alone cannot prove successful source completion: a
            // producer may drop its sender before failing. Cancellation also
            // wakes an initial receive even while external code holds sender.
            tokio::select! {
                () = wait_settled(&record) => {},
                () = record.cancel.cancelled() => {},
            }
            let (_, cancelled) =
                handles.facade_stream_state(stream, Some(owner_session), operation)?;
            if !cancelled {
                wait_settled(&record).await;
                if let Some(failure) = record
                    .producer_failure
                    .lock()
                    .unwrap_or_else(|error| error.into_inner())
                    .as_ref()
                {
                    let mut error = wire::handle_error(
                        ExtensionErrorCode::FrameworkError,
                        wire::bounded_message(&format!("facade stream producer failed: {failure}")),
                        operation,
                        stream,
                    );
                    error.details = Some(echo_sdk_protocol::error::ErrorDetails {
                        fields: Some(vec![echo_sdk_protocol::error::DetailField {
                            key: "producer_exit".to_string(),
                            value: if failure.is_panic() { "panic" } else { "abort" }.to_string(),
                        }]),
                        ..Default::default()
                    });
                    item = Some(Err(error));
                }
            }
        }
        if let Some(Err(error)) = &item {
            *record
                .pending_failure
                .lock()
                .unwrap_or_else(|error| error.into_inner()) = Some(error.clone());
        }
        let (_, cancelled) = handles.facade_stream_state(stream, Some(owner_session), operation)?;
        if !cancelled && let Some(Ok(value)) = item.as_ref() {
            let sequence = last_sequence.checked_add(1).ok_or_else(|| {
                wire::sdk_error(
                    ExtensionErrorCode::FrameworkError,
                    "facade stream sequence exhausted",
                    Retryability::Never,
                    operation,
                )
            })?;
            match handles.advance_facade_stream(stream, Some(owner_session), sequence, operation) {
                Ok(()) => return Ok(stream_event(stream, sequence, "item", Some(value.clone()))),
                Err(error) if error.code == ExtensionErrorCode::Cancelled => {}
                Err(error) => return Err(error),
            }
        }
        let error_value = match item {
            Some(Err(error)) => Some(
                WireValue::from_json(serde_json::to_value(error).map_err(|error| {
                    wire::sdk_error(
                        ExtensionErrorCode::FrameworkError,
                        format!("facade stream error projection failed: {error}"),
                        Retryability::Never,
                        operation,
                    )
                })?)
                .map_err(|error| {
                    wire::sdk_error(
                        ExtensionErrorCode::FrameworkError,
                        format!("facade stream error projection failed: {error}"),
                        Retryability::Never,
                        operation,
                    )
                })?,
            ),
            _ => None,
        };
        // Complete every asynchronous producer cleanup before committing the
        // registry terminal. If this waiter drops, the received cause remains
        // in pending_failure and the next pull can finish the same settlement.
        settle_record(&record, self.shutdown_timeout).await?;
        #[cfg(test)]
        if let Some((ready, proceed)) = self
            .terminal_commit_barriers
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .take()
        {
            // Test-only external thread fixes the exact pre-commit race order.
            ready.wait();
            proceed.wait();
        }
        let (sequence, cancelled) =
            handles.settle_facade_stream(stream, Some(owner_session), false, operation)?;
        let terminal = if cancelled {
            stream_event(stream, sequence, "cancelled", None)
        } else if let Some(error) = error_value {
            stream_event(stream, sequence, "failed", Some(error))
        } else {
            stream_event(stream, sequence, "complete", None)
        };
        // No await may follow terminal publication. Synchronous cleanup errors
        // retain the same close owner for retry; they cannot hide this event.
        let _ = self.finish_record(handles, stream, owner_session, operation, &record);
        Ok(terminal)
    }

    pub fn cancel(
        &self,
        handles: &HandleRegistry,
        stream: &WireHandle,
        owner_session: &str,
        family: &str,
        operation: &str,
    ) -> Result<bool, EchoSdkError> {
        let _ = self.record(handles, stream, owner_session, family, operation)?;
        let changed = handles.cancel_facade_stream(stream, Some(owner_session), operation)?;
        if changed
            && let Some(record) = self
                .streams
                .lock()
                .unwrap_or_else(|error| error.into_inner())
                .get(&stream.id)
                .cloned()
        {
            record.cancel.cancel();
        }
        Ok(changed)
    }

    pub async fn close(
        &self,
        handles: &HandleRegistry,
        stream: &WireHandle,
        owner_session: &str,
        family: &str,
        operation: &str,
    ) -> Result<bool, EchoSdkError> {
        if !handles.is_closed(stream) {
            let _ = self.record(handles, stream, owner_session, family, operation)?;
            // Close fences further items through the same registry authority,
            // even if cooperative cleanup outlives this request's deadline.
            handles.cancel_facade_stream(stream, Some(owner_session), operation)?;
        }
        let record = self
            .streams
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .get(&stream.id)
            .cloned();
        if let Some(record) = record {
            settle_record(&record, self.shutdown_timeout).await?;
            return self.finish_record(handles, stream, owner_session, operation, &record);
        }
        handles.close_facade_stream(stream, Some(owner_session), operation)
    }

    fn finish_record(
        &self,
        handles: &HandleRegistry,
        stream: &WireHandle,
        owner_session: &str,
        operation: &str,
        record: &FacadeStreamRecord,
    ) -> Result<bool, EchoSdkError> {
        if record.close_resource_on_finish {
            let resource = WireHandle {
                id: record.resource_id.clone(),
                kind: echo_sdk_protocol::handle::HandleKind::FacadeResource,
                generation: stream.generation.clone(),
            };
            handles.close_facade_resource(&resource, operation)?;
        }
        let closed = handles.close_facade_stream(stream, Some(owner_session), operation)?;
        self.streams
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .remove(&stream.id);
        Ok(closed)
    }

    async fn close_matching(
        &self,
        matches: impl Fn(&FacadeStreamRecord) -> bool,
    ) -> Result<(), EchoSdkError> {
        let records = self
            .streams
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .iter()
            .filter(|(_, record)| matches(record))
            .map(|(id, record)| (id.clone(), record.clone()))
            .collect::<Vec<_>>();
        for (_, record) in &records {
            record.cancel.cancel();
        }
        let mut pending = None;
        for (id, record) in records {
            if let Err(error) = settle_record(&record, self.shutdown_timeout).await {
                pending.get_or_insert(error);
                continue;
            }
            self.streams
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .remove(&id);
        }
        match pending {
            Some(error) => Err(error),
            None => Ok(()),
        }
    }

    pub async fn close_owner(&self, owner_session: &str) -> Result<(), EchoSdkError> {
        self.close_matching(|record| record.owner_session == owner_session)
            .await
    }

    pub async fn close_resource(&self, resource_id: &str) -> Result<(), EchoSdkError> {
        self.close_matching(|record| record.resource_id == resource_id)
            .await
    }

    pub async fn close_all(&self) -> Result<(), EchoSdkError> {
        self.close_matching(|_| true).await
    }

    fn record(
        &self,
        handles: &HandleRegistry,
        stream: &WireHandle,
        owner_session: &str,
        family: &str,
        operation: &str,
    ) -> Result<Arc<FacadeStreamRecord>, EchoSdkError> {
        handles.facade_stream(stream, Some(owner_session), operation)?;
        let record = self
            .streams
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .get(&stream.id)
            .cloned()
            .ok_or_else(|| {
                wire::sdk_error(
                    ExtensionErrorCode::ClosedHandle,
                    "facade stream receiver is no longer available",
                    Retryability::Never,
                    operation,
                )
            })?;
        if record.family != family {
            return Err(wire::sdk_error(
                ExtensionErrorCode::InvalidValue,
                format!(
                    "facade stream belongs to family {}, not {family}",
                    record.family
                ),
                Retryability::Never,
                operation,
            ));
        }
        Ok(record)
    }
}

pub(crate) async fn dispatch_control(
    runtime: &FacadeStreamRuntime,
    handles: &HandleRegistry,
    owner_session: &str,
    family: &str,
    request: &FeatureOperationRequest,
) -> Result<WireValue, EchoSdkError> {
    if request.arguments.len() != 1 {
        return Err(wire::sdk_error(
            ExtensionErrorCode::InvalidValue,
            "facade stream control accepts exactly one Stream handle argument",
            Retryability::Never,
            &request.operation,
        ));
    }
    let stream = match request.arguments.first() {
        Some(WireValue::Handle(handle)) => handle,
        _ => {
            return Err(wire::sdk_error(
                ExtensionErrorCode::InvalidValue,
                "facade stream control requires a Stream handle",
                Retryability::Never,
                &request.operation,
            ));
        }
    };
    if request.operation.ends_with(".stream.next") {
        runtime
            .next(handles, stream, owner_session, family, &request.operation)
            .await
    } else if request.operation.ends_with(".stream.cancel") {
        runtime
            .cancel(handles, stream, owner_session, family, &request.operation)
            .map(WireValue::Bool)
    } else if request.operation.ends_with(".stream.close") {
        runtime
            .close(handles, stream, owner_session, family, &request.operation)
            .await
            .map(WireValue::Bool)
    } else {
        Err(wire::sdk_error(
            ExtensionErrorCode::InvalidValue,
            "unknown facade stream control operation",
            Retryability::Never,
            &request.operation,
        ))
    }
}

fn stream_event(
    stream: &WireHandle,
    sequence: u64,
    variant: &str,
    value: Option<WireValue>,
) -> WireValue {
    let mut fields = vec![
        WireField {
            name: "stream".to_string(),
            value: WireValue::Handle(stream.clone()),
        },
        WireField {
            name: "sequence".to_string(),
            value: WireValue::U64(WireU64::from_u64(sequence)),
        },
    ];
    if let Some(value) = value {
        fields.push(WireField {
            name: "value".to_string(),
            value,
        });
    }
    WireValue::Variant {
        type_id: STREAM_EVENT_TYPE_ID.to_string(),
        variant: variant.to_string(),
        fields,
    }
}

async fn wait_settled(record: &FacadeStreamRecord) {
    loop {
        let changed = record.settled_notify.notified();
        tokio::pin!(changed);
        changed.as_mut().enable();
        if record.settled.load(std::sync::atomic::Ordering::Acquire) {
            return;
        }
        changed.await;
    }
}

async fn settle_record(
    record: &FacadeStreamRecord,
    timeout: std::time::Duration,
) -> Result<(), EchoSdkError> {
    record.cancel.cancel();
    if tokio::time::timeout(timeout, wait_settled(record))
        .await
        .is_err()
    {
        if !record.abort_on_close_deadline {
            return Err(wire::sdk_error(
                ExtensionErrorCode::FrameworkError,
                "channel stream cleanup is still awaiting its original callback/driver",
                Retryability::AfterDelay,
                "channels.stream.close",
            ));
        }
        if let Some(background) = record
            .background
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .as_ref()
        {
            background.abort();
        }
        wait_settled(record).await;
    }
    Ok(())
}

#[cfg(test)]
mod cleanup_tests {
    use super::*;
    use std::future::Future as _;
    use std::sync::atomic::{AtomicBool, Ordering};

    #[tokio::test]
    async fn cancelled_stream_close_retains_producer_until_collector_settles()
    -> Result<(), Box<dyn std::error::Error>> {
        let runtime = FacadeStreamRuntime::new(4, 5);
        let (sender, receiver) = tokio::sync::mpsc::channel(1);
        let cancel = tokio_util::sync::CancellationToken::new();
        let record = Arc::new(FacadeStreamRecord {
            owner_session: "session-a".to_string(),
            family: "fixture".to_string(),
            resource_id: "resource-a".to_string(),
            close_resource_on_finish: false,
            abort_on_close_deadline: true,
            receiver: tokio::sync::Mutex::new(receiver),
            cancel: cancel.clone(),
            background: Mutex::new(None),
            producer_failure: Mutex::new(None),
            pending_failure: Mutex::new(None),
            settled: AtomicBool::new(true),
            settled_notify: tokio::sync::Notify::new(),
        });
        let producer = FacadeStreamProducer {
            handle: WireHandle {
                id: "stream-fixture".to_string(),
                kind: echo_sdk_protocol::handle::HandleKind::Stream,
                generation: echo_sdk_protocol::scalar::WireU64::from_u64(1),
            },
            sender,
            cancel: cancel.clone(),
            cleanup: cancel.clone(),
            record: record.clone(),
        };
        runtime
            .streams
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .insert(producer.handle.id.clone(), record);
        let release = Arc::new(tokio::sync::Notify::new());
        let finished = Arc::new(AtomicBool::new(false));
        let background = tokio::spawn({
            let release = release.clone();
            let finished = finished.clone();
            async move {
                cancel.cancelled().await;
                release.notified().await;
                finished.store(true, Ordering::Release);
            }
        });
        runtime.attach_background(&producer, background);
        assert!(
            tokio::time::timeout(std::time::Duration::from_millis(20), runtime.close_all())
                .await
                .is_err()
        );
        assert_eq!(
            runtime
                .streams
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .len(),
            1
        );
        assert!(!finished.load(Ordering::Acquire));
        release.notify_one();
        tokio::time::timeout(std::time::Duration::from_secs(2), runtime.close_all())
            .await?
            .map_err(|error| error.message)?;
        assert!(finished.load(Ordering::Acquire));
        assert!(
            runtime
                .streams
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .is_empty()
        );
        Ok(())
    }

    fn owned_fixture() -> Result<
        (HandleRegistry, FacadeStreamRuntime, FacadeStreamProducer),
        Box<dyn std::error::Error>,
    > {
        let handles = HandleRegistry::new(1, 16);
        let llm_config = echo_agent::llm::LlmConfig::for_provider(
            "local",
            "http://127.0.0.1:11434/v1",
            "",
            "unused",
            echo_agent::llm::LlmApiProtocol::ChatCompletions,
        )?;
        let definition = crate::factory::PreparedAgentDefinition::new(
            Default::default(),
            Arc::new(echo_agent::llm::OpenAiClient::new(llm_config)?),
            None,
            false,
        );
        let (agent, _) = handles
            .register_agent(Arc::new(definition), String::new())
            .map_err(|error| std::io::Error::other(error.message))?;
        handles
            .register_session_with_cwd("session-a".to_string(), agent.id, None)
            .map_err(|error| std::io::Error::other(error.message))?;
        let (resource, _) = handles
            .register_facade_resource(
                4,
                "fixture",
                "fixture.resource",
                Some("session-a"),
                "fixture.open",
            )
            .map_err(|error| std::io::Error::other(error.message))?;
        let runtime = FacadeStreamRuntime::new(4, 0);
        let producer = runtime
            .open(&handles, &resource, "session-a", "fixture.open")
            .map_err(|error| std::io::Error::other(error.message))?;
        Ok((handles, runtime, producer))
    }

    fn assert_failed(
        value: WireValue,
        handle: &WireHandle,
        cause: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let WireValue::Variant {
            variant, fields, ..
        } = value
        else {
            return Err("expected stream variant".into());
        };
        assert_eq!(
            variant, "failed",
            "abnormal producer exit must never project complete"
        );
        let error_value = fields
            .into_iter()
            .find(|field| field.name == "value")
            .ok_or("failed stream has no cause")?
            .value;
        let error: EchoSdkError = serde_json::from_value(error_value.into_json()?)?;
        assert_eq!(error.code, ExtensionErrorCode::FrameworkError);
        assert_eq!(error.retryable, Retryability::Never);
        assert_eq!(error.handle.as_ref(), Some(handle));
        assert!(error.message.chars().count() <= 2048);
        let details = error
            .details
            .ok_or("producer failure lacks typed exit fact")?;
        assert!(
            details
                .fields
                .ok_or("producer failure lacks facts")?
                .iter()
                .any(|fact| fact.key == "producer_exit" && fact.value == cause)
        );
        Ok(())
    }

    #[tokio::test]
    async fn producer_unwind_yields_failed_terminal_not_complete()
    -> Result<(), Box<dyn std::error::Error>> {
        let (handles, runtime, producer) = owned_fixture()?;
        let handle = producer.handle.clone();
        let sender = producer.sender.clone();
        let background = tokio::spawn(async move {
            let _sender = sender;
            // Simulate an external source unwind through Tokio's real JoinError.
            std::panic::resume_unwind(Box::new("外部生产源故障🙂".repeat(4096)));
        });
        runtime.attach_background(&producer, background);
        drop(producer);
        assert_failed(
            runtime
                .next(
                    &handles,
                    &handle,
                    "session-a",
                    "fixture",
                    "fixture.stream.next",
                )
                .await
                .map_err(|error| std::io::Error::other(error.message))?,
            &handle,
            "panic",
        )?;
        assert!(
            runtime
                .next(
                    &handles,
                    &handle,
                    "session-a",
                    "fixture",
                    "fixture.stream.next"
                )
                .await
                .is_err_and(|error| error.code == ExtensionErrorCode::ClosedHandle)
        );
        assert!(
            !runtime
                .close(
                    &handles,
                    &handle,
                    "session-a",
                    "fixture",
                    "fixture.stream.close"
                )
                .await
                .map_err(|error| std::io::Error::other(error.message))?
        );
        Ok(())
    }

    #[tokio::test]
    async fn producer_abort_yields_failed_terminal_not_complete()
    -> Result<(), Box<dyn std::error::Error>> {
        let (handles, runtime, producer) = owned_fixture()?;
        let handle = producer.handle.clone();
        let sender = producer.sender.clone();
        let background = tokio::spawn(async move {
            let _sender = sender;
            std::future::pending::<()>().await;
        });
        let abort = background.abort_handle();
        runtime.attach_background(&producer, background);
        drop(producer);
        abort.abort();
        assert_failed(
            runtime
                .next(
                    &handles,
                    &handle,
                    "session-a",
                    "fixture",
                    "fixture.stream.next",
                )
                .await
                .map_err(|error| std::io::Error::other(error.message))?,
            &handle,
            "abort",
        )?;
        Ok(())
    }

    #[tokio::test]
    async fn eof_waits_for_producer_failure_and_cancelled_pull_retries_same_cause()
    -> Result<(), Box<dyn std::error::Error>> {
        let (handles, runtime, producer) = owned_fixture()?;
        let handle = producer.handle.clone();
        let sender = producer.sender.clone();
        let entered = Arc::new(tokio::sync::Notify::new());
        let release = Arc::new(tokio::sync::Notify::new());
        let background = tokio::spawn({
            let entered = entered.clone();
            let release = release.clone();
            async move {
                drop(sender);
                entered.notify_one();
                release.notified().await;
                // External source fails after closing its output queue.
                std::panic::resume_unwind(Box::new("delayed external failure"));
            }
        });
        runtime.attach_background(&producer, background);
        let record = producer.record.clone();
        drop(producer);
        entered.notified().await;
        assert!(
            tokio::time::timeout(
                std::time::Duration::from_millis(20),
                runtime.next(
                    &handles,
                    &handle,
                    "session-a",
                    "fixture",
                    "fixture.stream.next"
                )
            )
            .await
            .is_err()
        );
        assert_eq!(
            handles
                .facade_stream_state(&handle, Some("session-a"), "fixture.next")
                .map_err(|error| std::io::Error::other(error.message))?,
            (0, false)
        );
        release.notify_one();
        assert_failed(
            runtime
                .next(
                    &handles,
                    &handle,
                    "session-a",
                    "fixture",
                    "fixture.stream.next",
                )
                .await
                .map_err(|error| std::io::Error::other(error.message))?,
            &handle,
            "panic",
        )?;
        assert!(record.settled.load(Ordering::Acquire));
        Ok(())
    }

    #[tokio::test]
    async fn close_deadline_aborts_producer_and_collector_retains_abort_fact()
    -> Result<(), Box<dyn std::error::Error>> {
        let (handles, runtime, producer) = owned_fixture()?;
        let handle = producer.handle.clone();
        let record = producer.record.clone();
        let sender = producer.sender.clone();
        let background = tokio::spawn(async move {
            let _sender = sender;
            std::future::pending::<()>().await;
        });
        runtime.attach_background(&producer, background);
        drop(producer);
        assert!(
            runtime
                .close(
                    &handles,
                    &handle,
                    "session-a",
                    "fixture",
                    "fixture.stream.close"
                )
                .await
                .map_err(|error| std::io::Error::other(error.message))?
        );
        assert!(record.settled.load(Ordering::Acquire));
        assert!(
            record
                .producer_failure
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .as_ref()
                .is_some_and(tokio::task::JoinError::is_cancelled)
        );
        assert!(
            !runtime
                .close(
                    &handles,
                    &handle,
                    "session-a",
                    "fixture",
                    "fixture.stream.close"
                )
                .await
                .map_err(|error| std::io::Error::other(error.message))?
        );
        Ok(())
    }

    #[tokio::test]
    async fn cancellation_wakes_initial_receive_while_external_source_keeps_sender()
    -> Result<(), Box<dyn std::error::Error>> {
        let (handles, runtime, producer) = owned_fixture()?;
        let handle = producer.handle.clone();
        let sender = producer.sender.clone();
        let background = tokio::spawn(async move {
            let _sender = sender;
            // Model an external run_stream/execute_stream call that has not
            // produced its first item and does not observe our cancel token.
            std::future::pending::<()>().await;
        });
        runtime.attach_background(&producer, background);
        drop(producer);
        let pull = runtime.next(
            &handles,
            &handle,
            "session-a",
            "fixture",
            "fixture.stream.next",
        );
        tokio::pin!(pull);
        std::future::poll_fn(|context| {
            assert!(pull.as_mut().poll(context).is_pending());
            std::task::Poll::Ready(())
        })
        .await;
        assert!(
            runtime
                .cancel(
                    &handles,
                    &handle,
                    "session-a",
                    "fixture",
                    "fixture.stream.cancel"
                )
                .map_err(|error| std::io::Error::other(error.message))?
        );
        let terminal = tokio::time::timeout(std::time::Duration::from_secs(1), pull)
            .await?
            .map_err(|error| std::io::Error::other(error.message))?;
        let WireValue::Variant {
            variant, fields, ..
        } = terminal
        else {
            return Err("expected cancelled stream event".into());
        };
        assert_eq!(variant, "cancelled");
        assert!(
            fields.iter().any(|field| field.name == "sequence"
                && field.value == WireValue::U64(WireU64::from_u64(1)))
        );
        assert!(
            runtime
                .next(
                    &handles,
                    &handle,
                    "session-a",
                    "fixture",
                    "fixture.stream.next"
                )
                .await
                .is_err_and(|error| error.code == ExtensionErrorCode::ClosedHandle)
        );
        Ok(())
    }

    #[tokio::test]
    async fn cancelled_pull_during_failed_cleanup_preserves_error_and_terminal_sequence()
    -> Result<(), Box<dyn std::error::Error>> {
        let (handles, mut runtime, producer) = owned_fixture()?;
        runtime.shutdown_timeout = std::time::Duration::from_secs(5);
        let handle = producer.handle.clone();
        let sender = producer.sender.clone();
        let release = Arc::new(tokio::sync::Notify::new());
        let cleanup_started = Arc::new(tokio::sync::Notify::new());
        let cancel = producer.cleanup.clone();
        let source_error = wire::handle_error(
            ExtensionErrorCode::FrameworkError,
            "真实源错误🙂",
            "external.source",
            &handle,
        );
        let expected_error = source_error.clone();
        let background = tokio::spawn({
            let release = release.clone();
            let cleanup_started = cleanup_started.clone();
            async move {
                let _ = sender.send(Err(source_error)).await;
                cancel.cancelled().await;
                cleanup_started.notify_one();
                // Keep sender and owned cleanup alive until the external
                // source really completes, even if the pull waiter drops.
                release.notified().await;
                drop(sender);
            }
        });
        runtime.attach_background(&producer, background);
        drop(producer);
        assert!(
            tokio::time::timeout(
                std::time::Duration::from_millis(40),
                runtime.next(
                    &handles,
                    &handle,
                    "session-a",
                    "fixture",
                    "fixture.stream.next"
                )
            )
            .await
            .is_err()
        );
        cleanup_started.notified().await;
        assert_eq!(
            handles
                .facade_stream_state(&handle, Some("session-a"), "fixture.next")
                .map_err(|error| std::io::Error::other(error.message))?,
            (0, false),
            "undelivered terminal cannot consume a registry sequence"
        );
        release.notify_one();
        let terminal = runtime
            .next(
                &handles,
                &handle,
                "session-a",
                "fixture",
                "fixture.stream.next",
            )
            .await
            .map_err(|error| std::io::Error::other(error.message))?;
        let WireValue::Variant {
            variant, fields, ..
        } = terminal
        else {
            return Err("expected failed stream event".into());
        };
        assert_eq!(variant, "failed");
        assert!(
            fields.iter().any(|field| field.name == "sequence"
                && field.value == WireValue::U64(WireU64::from_u64(1)))
        );
        let error_value = fields
            .into_iter()
            .find(|field| field.name == "value")
            .ok_or("missing failed cause")?
            .value;
        let actual_error: EchoSdkError = serde_json::from_value(error_value.into_json()?)?;
        assert_eq!(
            actual_error, expected_error,
            "retry must return the exact source error, not producer EOF success"
        );
        assert!(
            runtime
                .next(
                    &handles,
                    &handle,
                    "session-a",
                    "fixture",
                    "fixture.stream.next"
                )
                .await
                .is_err_and(|error| error.code == ExtensionErrorCode::ClosedHandle)
        );
        assert!(
            !runtime
                .close(
                    &handles,
                    &handle,
                    "session-a",
                    "fixture",
                    "fixture.stream.close"
                )
                .await
                .map_err(|error| std::io::Error::other(error.message))?
        );
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn accepted_cancel_between_snapshot_and_terminal_commit_returns_cancelled()
    -> Result<(), Box<dyn std::error::Error>> {
        let (handles, runtime, producer) = owned_fixture()?;
        let handles = Arc::new(handles);
        let runtime = Arc::new(runtime);
        let handle = producer.handle.clone();
        let sender = producer.sender.clone();
        runtime.attach_background(
            &producer,
            tokio::spawn(async move {
                drop(sender);
            }),
        );
        drop(producer);
        let ready = Arc::new(std::sync::Barrier::new(2));
        let proceed = Arc::new(std::sync::Barrier::new(2));
        *runtime
            .terminal_commit_barriers
            .lock()
            .unwrap_or_else(|error| error.into_inner()) = Some((ready.clone(), proceed.clone()));
        let cancel_thread = std::thread::spawn({
            let handles = handles.clone();
            let runtime = runtime.clone();
            let handle = handle.clone();
            move || {
                ready.wait();
                let accepted = runtime.cancel(
                    &handles,
                    &handle,
                    "session-a",
                    "fixture",
                    "fixture.stream.cancel",
                );
                proceed.wait();
                accepted
            }
        });
        let terminal = runtime
            .next(
                &handles,
                &handle,
                "session-a",
                "fixture",
                "fixture.stream.next",
            )
            .await
            .map_err(|error| std::io::Error::other(error.message))?;
        let accepted = cancel_thread
            .join()
            .map_err(|_| "external cancel thread failed")?
            .map_err(|error| std::io::Error::other(error.message))?;
        assert!(
            accepted,
            "cancel committed before terminal must be accepted"
        );
        let WireValue::Variant {
            variant, fields, ..
        } = terminal
        else {
            return Err("expected stream terminal".into());
        };
        assert_eq!(
            variant, "cancelled",
            "terminal must project the state committed under Registry's lock, not a pre-commit snapshot"
        );
        assert!(
            fields.iter().any(|field| field.name == "sequence"
                && field.value == WireValue::U64(WireU64::from_u64(1)))
        );
        assert!(
            runtime
                .next(
                    &handles,
                    &handle,
                    "session-a",
                    "fixture",
                    "fixture.stream.next"
                )
                .await
                .is_err_and(|error| error.code == ExtensionErrorCode::ClosedHandle)
        );
        Ok(())
    }

    #[test]
    fn cancel_after_registry_terminal_commit_is_not_accepted()
    -> Result<(), Box<dyn std::error::Error>> {
        let (handles, _runtime, producer) = owned_fixture()?;
        handles
            .settle_facade_stream(
                &producer.handle,
                Some("session-a"),
                false,
                "fixture.stream.next",
            )
            .map_err(|error| std::io::Error::other(error.message))?;
        let changed = handles
            .cancel_facade_stream(&producer.handle, Some("session-a"), "fixture.stream.cancel")
            .map_err(|error| std::io::Error::other(error.message))?;
        assert!(
            !changed,
            "cancel cannot mutate a terminal that was committed first"
        );
        assert_eq!(
            handles
                .facade_stream_state(&producer.handle, Some("session-a"), "fixture.stream.state")
                .map_err(|error| std::io::Error::other(error.message))?,
            (1, false)
        );
        Ok(())
    }

    #[tokio::test]
    async fn producer_token_cancellation_does_not_commit_a_buffered_item()
    -> Result<(), Box<dyn std::error::Error>> {
        let (handles, runtime, producer) = owned_fixture()?;
        producer
            .sender
            .send(Ok(WireValue::String(
                "retired-generation-output".to_string(),
            )))
            .await?;
        // This is the same original token the Channel invocation/generation
        // producer observes, not a second runtime.cancel wire request.
        producer.cancel.cancel();
        let event = runtime
            .next(
                &handles,
                &producer.handle,
                "session-a",
                "fixture",
                "fixture.stream.next",
            )
            .await
            .map_err(|error| error.message)?;
        let WireValue::Variant { variant, .. } = event else {
            return Err("expected facade stream event".into());
        };
        assert_eq!(
            variant, "cancelled",
            "a producer cancellation fact must prevent a retired buffered item from committing"
        );
        assert!(handles.is_closed(&producer.handle));
        Ok(())
    }

    #[tokio::test]
    async fn cooperative_close_deadline_retains_same_address_until_original_producer_exits()
    -> Result<(), Box<dyn std::error::Error>> {
        let (handles, runtime, original) = owned_fixture()?;
        runtime
            .close(
                &handles,
                &original.handle,
                "session-a",
                "fixture",
                "fixture.stream.close",
            )
            .await
            .map_err(|error| error.message)?;
        let resource = handles
            .register_facade_resource(4, "fixture", "resource", Some("session-a"), "fixture.open")
            .map_err(|error| error.message)?
            .0;
        let producer = runtime
            .open_cooperative(&handles, &resource, "session-a", "fixture.open")
            .map_err(|error| error.message)?;
        let handle = producer.handle.clone();
        let release = Arc::new(tokio::sync::Notify::new());
        let exited = Arc::new(AtomicBool::new(false));
        let sender = producer.sender.clone();
        let background = tokio::spawn({
            let release = release.clone();
            let exited = exited.clone();
            async move {
                release.notified().await;
                drop(sender);
                exited.store(true, Ordering::Release);
            }
        });
        runtime.attach_background(&producer, background);
        let error = runtime
            .close(
                &handles,
                &handle,
                "session-a",
                "fixture",
                "fixture.stream.close",
            )
            .await
            .err()
            .ok_or("close falsely succeeded before source exit")?;
        assert_eq!(error.code, ExtensionErrorCode::FrameworkError);
        assert_eq!(error.retryable, Retryability::AfterDelay);
        assert!(!handles.is_closed(&handle));
        assert!(!exited.load(Ordering::Acquire));
        assert!(
            handles
                .facade_stream(&handle, Some("session-a"), "fixture.retry")
                .is_ok()
        );
        release.notify_one();
        let record = runtime
            .streams
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .get(&handle.id)
            .cloned()
            .ok_or("missing original retry owner")?;
        wait_settled(&record).await;
        assert!(
            runtime
                .close(
                    &handles,
                    &handle,
                    "session-a",
                    "fixture",
                    "fixture.stream.close"
                )
                .await
                .map_err(|error| error.message)?
        );
        assert!(exited.load(Ordering::Acquire));
        assert!(handles.is_closed(&handle));
        Ok(())
    }
}
