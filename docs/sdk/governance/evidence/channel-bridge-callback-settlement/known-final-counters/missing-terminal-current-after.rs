//! Bidirectional `_echo_agent/extension/*` bridge (supreme plan 06).
//!
//! The bridge connects two directions over one ACP connection:
//!
//! - *Host → SDK reverse invocation*: framework trait proxies acquire a
//!   lease from the shared [`ExtensionInvocationAuthority`], send one typed
//!   [`ExtensionInvokeCall`] through the official `ConnectionTo` transport
//!   and settle exactly once. Deadline, cancellation and disconnect all
//!   settle locally with typed errors; a late response can only be
//!   discarded. Channel handlers use a driven adapter: cancellation signals
//!   the callback, while the original connection response task retains its
//!   lease until the callback actually returns. There is no built-in fallback.
//! - *SDK → Host stream delivery*: streaming callbacks answer with the
//!   Host-minted stream handle and deliver chunks through
//!   `_echo_agent/extension/stream` notifications routed to a bounded sink
//!   with monotonic sequences and exactly one terminal.
//!
//! Trait proxies are thin by construction: they convert Rust trait
//! arguments to the canonical per-operation payload selected by
//! kind + operation and restore the result. Framework authorities
//! (ToolManager policy, run terminals, event envelopes, the session
//! registry) stay untouched.

use agent_client_protocol::{Client, ConnectionTo};
#[cfg(feature = "framework-channels")]
use async_trait::async_trait;
use base64::Engine as _;
use echo_agent::agent::{Agent, AgentEvent, CancellationToken};
#[cfg(feature = "framework-channels")]
use echo_agent::channels::{
    AttachmentKind, ChannelCapabilities, ChannelPlugin, ChatType as ChannelChatType,
    InboundMessage, MessageAttachment, MessageHandler, OutboundMessage,
};
use echo_agent::error::ReactError;
use echo_agent::tools::{ToolContext, ToolParameters, ToolResult, ToolStreamEvent};
use futures::{Stream, StreamExt as _};
use std::collections::{HashMap, VecDeque};
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, OnceLock, Weak};
use std::task::{Context, Poll};
use std::time::Duration;
use std::time::Instant;
use tokio::sync::{Notify, mpsc};

use echo_sdk_protocol::error::{EchoSdkError, ExtensionErrorCode, Retryability};
use echo_sdk_protocol::handle::{HandleKind, WireHandle};
use echo_sdk_protocol::methods::{
    ActivateSkillWire, AgentComponentCallInputWire, AgentComponentCallResultWire,
    AgentComponentCallWire, AgentComponentCapabilitiesWire, AgentComponentKindWire,
    AgentComponentOperationWire, AgentComponentStreamChunkWire, AgentComponentStreamCompleteWire,
    AgentEventWire, AgentFactoryConfigWire, AgentMessageInput, AgentStreamChunkWire,
    AgentStreamTerminalWire, AgentTaskInput, ApprovalScopeWire, CallbackFinalAnswerInput,
    CallbackIterationInput, CallbackThinkEndInput, CallbackThinkStartInput, CallbackToolEndInput,
    CallbackToolErrorInput, CallbackToolInterruptedInput, CallbackToolStartInput,
    CompressionInputWire, CompressionOutputWire, CritiqueInput, CritiqueWire,
    CustomAgentDescriptorWire, ExtensionDescriptor, ExtensionInvocation,
    ExtensionInvocationContext, ExtensionInvokeCall, ExtensionInvokeOutcome, ExtensionKind,
    ExtensionOperation, ExtensionResult, ExtensionStreamChunkValue, ExtensionStreamCompleteValue,
    ExtensionStreamEvent, ExtensionUnit, HookResultWire, HookRunInput, HumanLoopKindWire,
    HumanLoopRequestWire, HumanLoopResponseWire, HumanRiskLevelWire, InterventionResultWire,
    LlmChatChunkWire, LlmChatRequestWire, LlmChatResponseWire, LlmDeltaToolCallWire,
    LlmMessageWire, LlmReasoningBlockWire, LlmStreamChunkWire, LlmStreamCompleteWire,
    LlmToolCallWire, LlmToolDefinitionWire, PermissionDecisionWire, SandboxOutputChannelWire,
    SandboxStreamChunkWire, SandboxStreamCompleteWire, SandboxStreamFailureWire,
    SkillDescriptorPolicyWire, StepTypeWire, StoreItemWire, StoreKeyInput,
    StoreListNamespacesInput, StoreNamespaceInput, StorePutInput, StoreSearchInput,
    StoreSearchModeWire, StoreSearchQueryWire, StoreSearchWithInput, TokenizerReferenceWire,
    ToolContextWire, ToolExecuteInput, ToolOutputArtifactConfigWire, ToolStreamChunkWire,
    ToolStreamEventWire, ToolValidateInput, WorkflowStreamChunkWire, WorkflowStreamCompleteWire,
};
#[cfg(feature = "framework-channels")]
use echo_sdk_protocol::methods::{
    ChannelAttachmentWire, ChannelChatTypeWire, ChannelHandleInput, ChannelInboundMessageWire,
    ChannelOutboundMessageWire, ChannelPluginDescriptorWire, ChannelReplyInput, ChannelSendInput,
    ChannelStartInput,
};
#[cfg(test)]
use echo_sdk_protocol::methods::{LlmDeltaFunctionWire, LlmUsageWire};
use echo_sdk_protocol::scalar::{WireI64, WirePath, WireU64, WireValue};

use super::state::CoreProfileState;
use super::wire::managed::Projection as _;
use super::wire::{sdk_error, tool_result_from_wire, tool_result_to_wire};
use echo_sdk_protocol::managed::{
    ConversationProjectionCapabilityWire, PersistenceCallCapabilityWire,
    PersistenceCallContextWire, RuntimeStateCapabilityWire,
};

/// Bounded stream-channel capacity for one reverse callback stream. The
/// consumer (the Rust stream proxy) paces delivery; a full channel makes the
/// notification handler wait (never the reader loop — routing is spawned).
#[cfg(test)]
const STREAM_CHANNEL_CAPACITY: usize = 128;
const MCP_NOTIFICATION_QUEUE_CAPACITY: usize = 64;

// ── Shared bridge state ─────────────────────────────────────────────────────

/// Connection-captured state shared by every proxy of one connection.
pub(crate) struct ExtensionBridgeShared {
    /// The official transport handle, captured from the first extension
    /// handler invocation. Every proxy sends through this one connection.
    connection: OnceLock<ConnectionTo<Client>>,
    /// Live callback stream sinks by stream id.
    streams: Mutex<HashMap<String, Arc<ExtensionStreamSink>>>,
    /// Number of reverse callbacks currently awaiting the SDK for each
    /// Session. A callback may issue independent SDK requests, but an
    /// exclusive mutation of the same Session must fail immediately rather
    /// than wait on the Agent lock held by the originating run.
    active_session_callbacks: Mutex<HashMap<String, usize>>,
    /// Cleanup jobs remain owned by this connection, including while they
    /// back off between reverse invocations. Teardown aborts and removes
    /// them before releasing the corresponding extension handles.
    factory_cleanup_tasks: Mutex<Vec<tokio::task::JoinHandle<()>>>,
    cleanup_closed: AtomicBool,
}

impl ExtensionBridgeShared {
    pub(crate) fn new() -> Self {
        Self {
            connection: OnceLock::new(),
            streams: Mutex::new(HashMap::new()),
            active_session_callbacks: Mutex::new(HashMap::new()),
            factory_cleanup_tasks: Mutex::new(Vec::new()),
            cleanup_closed: AtomicBool::new(false),
        }
    }

    fn enter_session_callback(
        self: &Arc<Self>,
        session_id: Option<&str>,
    ) -> Option<SessionCallbackGuard> {
        let session_id = session_id?.to_string();
        let mut active = self
            .active_session_callbacks
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let count = active.entry(session_id.clone()).or_default();
        *count = count.saturating_add(1);
        drop(active);
        Some(SessionCallbackGuard {
            shared: self.clone(),
            session_id,
        })
    }

    pub(crate) fn has_active_session_callback(&self, session_id: &str) -> bool {
        self.active_session_callbacks
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .get(session_id)
            .is_some_and(|count| *count > 0)
    }

    /// Capture the connection once. Later calls with the same connection are
    /// no-ops (the stdio Host serves exactly one connection per process).
    pub(crate) fn bind_connection(&self, connection: ConnectionTo<Client>) {
        let _ = self.connection.set(connection);
    }

    fn connection(&self) -> std::result::Result<ConnectionTo<Client>, EchoSdkError> {
        self.connection.get().cloned().ok_or_else(|| {
            sdk_error(
                ExtensionErrorCode::ExtensionDisconnected,
                "extension transport is not bound to a connection",
                Retryability::Never,
                "_echo_agent/extension/invoke",
            )
        })
    }

    fn register_stream(&self, sink: Arc<ExtensionStreamSink>) {
        self.streams
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .insert(sink.stream_id().to_string(), sink);
    }

    fn remove_stream(&self, stream_id: &str) {
        if let Some(sink) = self
            .streams
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .remove(stream_id)
        {
            sink.close();
        }
    }

    pub(crate) fn remove_streams_for_extension(&self, extension_id: &str) {
        let sinks = {
            let mut streams = self
                .streams
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            let ids = streams
                .iter()
                .filter(|(_, sink)| sink.extension_id() == extension_id)
                .map(|(id, _)| id.clone())
                .collect::<Vec<_>>();
            ids.into_iter()
                .filter_map(|id| streams.remove(&id))
                .collect::<Vec<_>>()
        };
        for sink in sinks {
            sink.close();
        }
    }

    pub(crate) fn close_all_streams(&self) {
        self.cleanup_closed.store(true, Ordering::Release);
        let tasks = self
            .factory_cleanup_tasks
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .drain(..)
            .collect::<Vec<_>>();
        for task in tasks {
            task.abort();
        }
        let sinks = self
            .streams
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .drain()
            .map(|(_, sink)| sink)
            .collect::<Vec<_>>();
        for sink in sinks {
            sink.close();
        }
    }

    fn track_factory_cleanup(&self, task: tokio::task::JoinHandle<()>) {
        let mut tasks = self
            .factory_cleanup_tasks
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        tasks.retain(|task| !task.is_finished());
        if self.cleanup_closed.load(Ordering::Acquire) {
            task.abort();
        } else {
            tasks.push(task);
        }
    }

    fn stream_sink(&self, stream_id: &str) -> Option<Arc<ExtensionStreamSink>> {
        self.streams
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .get(stream_id)
            .cloned()
    }

    /// Match the original already-admitted callback stream during teardown.
    /// This lookup grants no new registration or invocation admission.
    pub(crate) fn owns_settlement_stream(&self, stream: &WireHandle, extension_id: &str) -> bool {
        self.stream_sink(&stream.id).is_some_and(|sink| {
            sink.stream_handle() == stream && sink.extension_id() == extension_id
        })
    }

    /// Route one `_echo_agent/extension/stream` notification. Unknown or
    /// already-terminal streams discard the event with a bounded diagnostic
    /// (late delivery never overrides settled state). Delivery is
    /// non-blocking: a full bounded mailbox closes the stream and reports the
    /// backpressure failure to the notification handler.
    pub(crate) fn deliver_stream_event(
        &self,
        event: ExtensionStreamEvent,
    ) -> std::result::Result<(), String> {
        if let Err(reason) = event.validate() {
            self.fail_stream(event.stream(), reason);
            return Err(format!("stream event violated the contract: {reason}"));
        }
        let sink = {
            let streams = self
                .streams
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            streams.get(event.stream().id.as_str()).cloned()
        };
        let Some(sink) = sink else {
            tracing::warn!(
                stream = event.stream().id,
                "discarded stream event for an unknown or released stream"
            );
            return Ok(());
        };

        if event.stream() != sink.stream_handle() {
            return Err(
                "extension stream handle does not match the original owner/generation".to_string(),
            );
        }
        let payload_kind = match &event {
            ExtensionStreamEvent::Chunk { value, .. } => Some(value.operation_kind()),
            ExtensionStreamEvent::Complete { value, .. } => Some(value.operation_kind()),
            ExtensionStreamEvent::Failed { .. } | ExtensionStreamEvent::Cancelled { .. } => None,
        };
        if payload_kind.is_some_and(|kind| kind != sink.value_kind()) {
            self.fail_stream(
                sink.stream_handle(),
                "extension stream payload kind does not match the invocation",
            );
            return Err("extension stream payload kind does not match the invocation".to_string());
        }
        let is_terminal = event.is_terminal();
        if !sink.observe_remote_sequence(&event) {
            self.fail_stream(
                sink.stream_handle(),
                "extension stream remote sequence is not contiguous",
            );
            return Err("extension stream remote sequence is not contiguous".to_string());
        }
        if is_terminal {
            sink.remote_terminal_received.store(true, Ordering::Release);
            sink.remote_terminal_notify.notify_waiters();
        }
        let sequence = event.sequence().to_u64().unwrap_or_default();
        if !sink.admit_event(&event) {
            let reason = format!(
                "extension stream sequence {sequence} is not the next contiguous value or follows a terminal"
            );
            self.fail_stream(sink.stream_handle(), &reason);
            return Err(reason);
        }
        match sink.sender().try_send(event) {
            Ok(()) => {
                if is_terminal {
                    sink.clear_terminal();
                }
                Ok(())
            }
            Err(mpsc::error::TrySendError::Closed(_)) => {
                if !sink.retain_until_remote_terminal.load(Ordering::Acquire) {
                    self.remove_stream(sink.stream_id());
                }
                Ok(())
            }
            Err(mpsc::error::TrySendError::Full(event)) => {
                let reason = "extension stream consumer exceeded its bounded mailbox";
                sink.fail_undelivered(event.sequence().clone(), reason);
                if !sink.retain_until_remote_terminal.load(Ordering::Acquire) {
                    self.remove_stream(sink.stream_id());
                }
                Err(reason.to_string())
            }
        }
    }

    pub(crate) fn fail_stream(&self, stream: &WireHandle, reason: &str) {
        let sink = self.stream_sink(&stream.id);
        if let Some(sink) = sink {
            let message = reason.to_string();
            sink.terminate(|sequence| ExtensionStreamEvent::Failed {
                stream: stream.clone(),
                sequence,
                error: sdk_error(
                    ExtensionErrorCode::SerializationViolation,
                    message,
                    Retryability::Never,
                    "_echo_agent/extension/stream",
                ),
            });
            if !sink.retain_until_remote_terminal.load(Ordering::Acquire) {
                self.remove_stream(sink.stream_id());
            }
        }
    }
}

struct SessionCallbackGuard {
    shared: Arc<ExtensionBridgeShared>,
    session_id: String,
}

impl Drop for SessionCallbackGuard {
    fn drop(&mut self) {
        let mut active = self
            .shared
            .active_session_callbacks
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        if let Some(count) = active.get_mut(&self.session_id) {
            *count = count.saturating_sub(1);
            if *count == 0 {
                active.remove(&self.session_id);
            }
        }
    }
}

// ── Stream sink ─────────────────────────────────────────────────────────────

/// One live callback stream: bounded mailbox plus the monotonic-sequence and
/// exactly-one-terminal ledger.
pub(crate) struct ExtensionStreamSink {
    stream: WireHandle,
    extension_id: String,
    value_kind: ExtensionKind,
    sender: mpsc::Sender<ExtensionStreamEvent>,
    inner: Mutex<StreamSinkInner>,
    closed: AtomicBool,
    closed_notify: Notify,
    session_callback: Mutex<Option<SessionCallbackGuard>>,
    retain_until_remote_terminal: AtomicBool,
    remote_terminal_received: AtomicBool,
    remote_terminal_notify: Notify,
}

struct StreamSinkInner {
    last_sequence: u64,
    remote_last_sequence: u64,
    terminal_seen: bool,
    terminal: Option<ExtensionStreamEvent>,
}

impl ExtensionStreamSink {
    fn new(
        stream: WireHandle,
        extension_id: String,
        value_kind: ExtensionKind,
        capacity: usize,
        session_callback: Option<SessionCallbackGuard>,
    ) -> (Arc<Self>, mpsc::Receiver<ExtensionStreamEvent>) {
        let (sender, receiver) = mpsc::channel(capacity.max(1));
        (
            Arc::new(Self {
                stream,
                extension_id,
                value_kind,
                sender,
                inner: Mutex::new(StreamSinkInner {
                    last_sequence: 0,
                    remote_last_sequence: 0,
                    terminal_seen: false,
                    terminal: None,
                }),
                closed: AtomicBool::new(false),
                closed_notify: Notify::new(),
                session_callback: Mutex::new(session_callback),
                retain_until_remote_terminal: AtomicBool::new(false),
                remote_terminal_received: AtomicBool::new(false),
                remote_terminal_notify: Notify::new(),
            }),
            receiver,
        )
    }

    fn stream_id(&self) -> &str {
        &self.stream.id
    }

    fn stream_handle(&self) -> &WireHandle {
        &self.stream
    }

    fn extension_id(&self) -> &str {
        &self.extension_id
    }

    fn value_kind(&self) -> ExtensionKind {
        self.value_kind
    }

    fn sender(&self) -> &mpsc::Sender<ExtensionStreamEvent> {
        &self.sender
    }

    fn close(&self) {
        self.terminate(|sequence| ExtensionStreamEvent::Cancelled {
            stream: self.stream.clone(),
            sequence,
        });
        if !self.closed.swap(true, Ordering::AcqRel) {
            self.closed_notify.notify_waiters();
        }
        self.session_callback
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .take();
    }

    async fn wait_closed(&self) {
        loop {
            let notified = self.closed_notify.notified();
            if self.closed.load(Ordering::Acquire) {
                return;
            }
            notified.await;
        }
    }

    /// Observes a validated terminal received from the remote producer. Local
    /// timeout/backpressure terminals are not evidence that that producer ended.
    async fn wait_remote_terminal(&self) {
        loop {
            let notified = self.remote_terminal_notify.notified();
            if self.remote_terminal_received.load(Ordering::Acquire) {
                return;
            }
            notified.await;
        }
    }

    /// Remote receive watermark shares this same sink ledger. Locally
    /// synthesized terminal sequences never advance what the peer sent.
    fn observe_remote_sequence(&self, event: &ExtensionStreamEvent) -> bool {
        let Some(sequence) = event.sequence().to_u64() else {
            return false;
        };
        let mut inner = self.inner.lock().unwrap_or_else(|error| error.into_inner());
        if self.remote_terminal_received.load(Ordering::Acquire)
            || inner.remote_last_sequence.checked_add(1) != Some(sequence)
        {
            return false;
        }
        inner.remote_last_sequence = sequence;
        true
    }

    /// Sequence/terminal admission. Returns false for events that must be
    /// discarded (non-contiguous, post-terminal, or duplicate terminal).
    fn admit_event(&self, event: &ExtensionStreamEvent) -> bool {
        let sequence = event.sequence().to_u64().unwrap_or_default();
        let is_terminal = event.is_terminal();
        let Ok(mut inner) = self.inner.lock() else {
            return false;
        };
        let Some(expected) = inner.last_sequence.checked_add(1) else {
            return false;
        };
        if inner.terminal_seen || sequence != expected {
            return false;
        }
        inner.last_sequence = sequence;
        inner.terminal_seen = is_terminal;
        if is_terminal {
            inner.terminal = Some(event.clone());
        }
        true
    }

    /// Replace an admitted event that could not enter the bounded mailbox
    /// with a failed terminal at the same sequence. Keeping that terminal in
    /// the sink lets the consumer observe the failure after it drains the
    /// already-buffered prefix, without fabricating a sequence gap.
    fn fail_undelivered(&self, sequence: echo_sdk_protocol::scalar::WireNonZeroU64, reason: &str) {
        let event = ExtensionStreamEvent::Failed {
            stream: self.stream.clone(),
            sequence,
            error: sdk_error(
                ExtensionErrorCode::ExtensionFailed,
                reason,
                Retryability::Never,
                "_echo_agent/extension/stream",
            ),
        };
        let Ok(mut inner) = self.inner.lock() else {
            return;
        };
        inner.terminal_seen = true;
        inner.terminal = Some(event.clone());
        drop(inner);
        if self.sender.try_send(event).is_ok() {
            self.clear_terminal();
        }
    }

    fn terminate(
        &self,
        build: impl FnOnce(echo_sdk_protocol::scalar::WireNonZeroU64) -> ExtensionStreamEvent,
    ) {
        let Ok(mut inner) = self.inner.lock() else {
            return;
        };
        if inner.terminal_seen {
            return;
        }
        let Some(sequence) = inner.last_sequence.checked_add(1).and_then(|value| {
            echo_sdk_protocol::scalar::WireNonZeroU64::try_from(value.to_string()).ok()
        }) else {
            return;
        };
        inner.last_sequence = sequence.to_u64().unwrap_or_default();
        inner.terminal_seen = true;
        let event = build(sequence);
        inner.terminal = Some(event.clone());
        drop(inner);
        if self.sender.try_send(event).is_ok() {
            self.clear_terminal();
        }
    }

    fn take_terminal(&self) -> Option<ExtensionStreamEvent> {
        self.inner.lock().ok()?.terminal.take()
    }

    fn clear_terminal(&self) {
        if let Ok(mut inner) = self.inner.lock() {
            inner.terminal = None;
        }
    }
}

// ── Bridge handle ───────────────────────────────────────────────────────────

/// Per-call bridge over the shared state. Cheap to construct; proxies hold
/// one for their lifetime. The profile state binds itself after
/// construction (the bridge is created first because the default Agent
/// definition must capture it before any Session exists).
#[derive(Clone)]
pub(crate) struct ExtensionBridge {
    // The profile owns this bridge (also through PreparedAgentDefinition).
    // A strong back-reference would keep the complete Host generation and
    // persistence resources alive after every embedded Host close/drop.
    state: OnceLock<Weak<CoreProfileState>>,
    shared: Arc<ExtensionBridgeShared>,
}

/// Transport lifetime facts for a reverse call. The optional submission
/// observer lets a factory release a reservation only before it was handed
/// to the official outgoing actor; it is not an execution state authority.
struct ReverseInvocationScope<'a> {
    cancellation: CancellationToken,
    resolve_run_cancellation: bool,
    submitted: Option<&'a AtomicBool>,
}

/// Kinds whose callbacks mutate the implementation exclusively: a second
/// in-flight invocation on the same registration is a typed conflict, never
/// a wait (design §12.3).
/// AgentFactory creates independent instances and follows the framework's
/// Send + Sync factory contract. Exclusivity starts at each CustomAgent child,
/// not at its shared constructor registration.
fn is_exclusive_invocation(kind: ExtensionKind, invocation: &ExtensionInvocation) -> bool {
    if kind == ExtensionKind::AgentComponent {
        return matches!(
            invocation,
            ExtensionInvocation::AgentComponentCall(AgentComponentCallWire {
                component: AgentComponentKindWire::Workflow,
                ..
            }) | ExtensionInvocation::AgentComponentCallStream(AgentComponentCallWire {
                component: AgentComponentKindWire::Workflow,
                ..
            })
        );
    }
    matches!(
        kind,
        ExtensionKind::HumanLoopProvider
            | ExtensionKind::CustomAgent
            | ExtensionKind::Hook
            | ExtensionKind::ChannelPlugin
            | ExtensionKind::ContextCompressor
    )
}

fn lease_error(error: echo_agent::acp::ExtensionLeaseError) -> EchoSdkError {
    match error {
        echo_agent::acp::ExtensionLeaseError::AdmissionClosed => sdk_error(
            ExtensionErrorCode::HostShuttingDown,
            "the Host is shutting down and refuses new extension invocations",
            Retryability::Never,
            "_echo_agent/extension/invoke",
        ),
        echo_agent::acp::ExtensionLeaseError::ConcurrencyLimit => sdk_error(
            ExtensionErrorCode::ExtensionRejected,
            "extension callback concurrency limit reached",
            Retryability::AfterDelay,
            "_echo_agent/extension/invoke",
        ),
        echo_agent::acp::ExtensionLeaseError::ExclusiveConflict => sdk_error(
            ExtensionErrorCode::ExtensionConflict,
            "extension is already executing an exclusive invocation",
            Retryability::Never,
            "_echo_agent/extension/invoke",
        ),
    }
}

fn transport_error(error: &agent_client_protocol::Error) -> EchoSdkError {
    if agent_client_protocol::is_incoming_transport_closed(error) {
        sdk_error(
            ExtensionErrorCode::ExtensionDisconnected,
            "the SDK connection closed before the callback answered",
            Retryability::Never,
            "_echo_agent/extension/invoke",
        )
    } else if let Ok(decoded) = EchoSdkError::from_jsonrpc_data(error.data.as_ref()) {
        decoded
    } else {
        sdk_error(
            ExtensionErrorCode::ExtensionFailed,
            format!("reverse invocation failed: {error}"),
            Retryability::Never,
            "_echo_agent/extension/invoke",
        )
    }
}

impl ExtensionBridge {
    pub(crate) fn unbound(shared: Arc<ExtensionBridgeShared>) -> Self {
        Self {
            state: OnceLock::new(),
            shared,
        }
    }

    /// Bind the profile state once; every proxy resolves it per invocation.
    pub(crate) fn bind_state(&self, state: Arc<CoreProfileState>) {
        let _ = self.state.set(Arc::downgrade(&state));
    }

    fn state(&self) -> std::result::Result<Arc<CoreProfileState>, EchoSdkError> {
        self.state.get().and_then(Weak::upgrade).ok_or_else(|| {
            sdk_error(
                ExtensionErrorCode::HostShuttingDown,
                "extension bridge profile state is not live",
                Retryability::Never,
                "_echo_agent/extension/invoke",
            )
        })
    }

    fn connection_cancellation(&self) -> CancellationToken {
        self.state()
            .ok()
            .and_then(|state| state.services().ok())
            .map(|services| services.extensions().connection_cancellation())
            .unwrap_or_default()
    }

    async fn cancellation_for_context(
        &self,
        context: Option<&ExtensionInvocationContext>,
        fallback: CancellationToken,
    ) -> CancellationToken {
        let Some(state) = self.state().ok() else {
            return fallback;
        };
        let Ok(services) = state.services() else {
            return fallback;
        };
        if let Some(run_id) = context.and_then(|context| context.run_id.as_deref())
            && let Some(run) = services.run(run_id).await
        {
            return run.cancellation();
        }
        if let Some(session_id) = context.and_then(|context| context.session_id.as_deref())
            && let Some(cancel) = services.active_run_cancellation(session_id).await
        {
            return cancel;
        }
        fallback
    }

    fn deadline_of(&self, record: &super::handles::ExtensionRecord) -> Duration {
        record
            .timeout
            .as_ref()
            .and_then(|timeout| {
                timeout.validate().ok().and_then(|_| {
                    timeout
                        .seconds
                        .to_u64()
                        .map(|seconds| Duration::new(seconds, timeout.nanos))
                })
            })
            .unwrap_or_else(|| {
                Duration::from_secs(
                    self.state()
                        .map(|state| state.limits.callback_timeout_secs)
                        .unwrap_or_default()
                        .max(1),
                )
            })
    }

    /// Acquire a lease and resolve the extension record through the fixed
    /// admission ladder. Shared by both invoke paths.
    #[allow(clippy::type_complexity)]
    fn prepare(
        &self,
        extension: &WireHandle,
        operation: ExtensionOperation,
        invocation: &ExtensionInvocation,
        framework_cancellation: CancellationToken,
    ) -> std::result::Result<
        (
            echo_agent::acp::ExtensionInvocationLease,
            Arc<super::handles::ExtensionRecord>,
            Arc<echo_agent::acp::ExtensionInvocationAuthority>,
        ),
        EchoSdkError,
    > {
        const OPERATION: &str = "_echo_agent/extension/invoke";
        let state = self.state()?;
        state
            .handles
            .check_shape_and_generation(extension, HandleKind::Extension, OPERATION)?;
        if let Err(reason) = extension.validate() {
            return Err(sdk_error(
                ExtensionErrorCode::InvalidValue,
                reason,
                Retryability::Never,
                OPERATION,
            ));
        }
        let record = if operation == ExtensionOperation::AgentClose {
            state.handles.extension_for_cleanup(extension)?
        } else {
            state.handles.extension(extension)?
        };
        if record.kind != operation.kind() {
            return Err(sdk_error(
                ExtensionErrorCode::InvalidValue,
                format!(
                    "operation {} does not address extension kind {}",
                    operation.as_str(),
                    record.kind.as_str()
                ),
                Retryability::Never,
                OPERATION,
            )
            .with_handle(extension.clone()));
        }
        let services = state.services().map_err(|error| {
            sdk_error(
                ExtensionErrorCode::HostShuttingDown,
                error.to_string(),
                Retryability::Never,
                OPERATION,
            )
        })?;
        let authority = services.extensions().clone();
        let exclusive_key = is_exclusive_invocation(record.kind, invocation)
            .then(|| format!("{}/{}", extension.id, record.kind.as_str()));
        let lease = authority
            .lease(exclusive_key.as_deref(), framework_cancellation)
            .map_err(lease_error)?;
        Ok((lease, record, authority))
    }

    /// Drive one non-streaming reverse invocation to its single settlement.
    pub(crate) async fn invoke_once(
        &self,
        extension: &WireHandle,
        context: Option<ExtensionInvocationContext>,
        invocation: ExtensionInvocation,
        framework_cancellation: CancellationToken,
    ) -> std::result::Result<ExtensionResult, EchoSdkError> {
        self.invoke_once_with_scope(
            extension,
            context,
            invocation,
            ReverseInvocationScope {
                cancellation: framework_cancellation,
                resolve_run_cancellation: true,
                submitted: None,
            },
        )
        .await
    }

    async fn invoke_once_connection_scoped(
        &self,
        extension: &WireHandle,
        context: Option<ExtensionInvocationContext>,
        invocation: ExtensionInvocation,
    ) -> std::result::Result<ExtensionResult, EchoSdkError> {
        self.invoke_once_with_scope(
            extension,
            context,
            invocation,
            ReverseInvocationScope {
                cancellation: self.connection_cancellation(),
                resolve_run_cancellation: false,
                submitted: None,
            },
        )
        .await
    }

    async fn invoke_once_with_scope(
        &self,
        extension: &WireHandle,
        context: Option<ExtensionInvocationContext>,
        invocation: ExtensionInvocation,
        scope: ReverseInvocationScope<'_>,
    ) -> std::result::Result<ExtensionResult, EchoSdkError> {
        let operation = invocation.operation();
        let framework_cancellation = if scope.resolve_run_cancellation {
            self.cancellation_for_context(context.as_ref(), scope.cancellation)
                .await
        } else {
            scope.cancellation
        };
        let (mut lease, record, _authority) =
            self.prepare(extension, operation, &invocation, framework_cancellation)?;
        let connection = self.shared.connection()?;
        self.ensure_payload_bound(&invocation, false)?;
        let mut deadline = self.deadline_of(&record);
        if let ExtensionInvocation::AgentComponentCall(call) = &invocation
            && let Some(context) = managed_call_context(&call.call)
        {
            let absolute_deadline_unix_ms =
                context.absolute_deadline_unix_ms.to_i64().ok_or_else(|| {
                    sdk_error(
                        ExtensionErrorCode::InvalidValue,
                        "invalid managed deadline",
                        Retryability::Never,
                        "_echo_agent/extension/invoke",
                    )
                })?;
            let remaining = echo_agent::memory::PersistenceCallContext {
                absolute_deadline_unix_ms,
            }
            .remaining_duration()
            .map_err(|error| {
                let mut error = sdk_error(
                    ExtensionErrorCode::ExtensionTimeout,
                    error.to_string(),
                    Retryability::Never,
                    "_echo_agent/extension/invoke",
                );
                // This rejection is before send_request. The backend
                // cannot have committed, unlike a response timeout.
                error.details = Some(echo_sdk_protocol::error::ErrorDetails {
                    fields: Some(vec![echo_sdk_protocol::error::DetailField {
                        key: "persistence_error_class".to_string(),
                        value: "deadline_exceeded".to_string(),
                    }]),
                    ..Default::default()
                });
                error
            })?;
            deadline = deadline.min(remaining);
        }
        let call = ExtensionInvokeCall {
            extension: extension.clone(),
            factory_origin: record.factory_origin.clone(),
            invocation_id: lease.identity().to_string(),
            context,
            invocation,
            deadline: wire_duration(deadline),
            stream: None,
        };
        if let Err(reason) = call.validate() {
            return Err(sdk_error(
                ExtensionErrorCode::SerializationViolation,
                reason,
                Retryability::Never,
                "_echo_agent/extension/invoke",
            ));
        }
        let _session_callback = self.shared.enter_session_callback(
            call.context
                .as_ref()
                .and_then(|value| value.session_id.as_deref()),
        );
        let sent = connection.send_request(call);
        // The official transport enqueues eagerly in send_request (not when
        // SentRequest is polled). After submission the constructor outcome
        // is unknown until its typed acknowledgement; before this point no
        // remote constructor could have observed the reserved child.
        if let Some(submitted) = scope.submitted {
            submitted.store(true, Ordering::Release);
        }
        let cancellation = lease.cancellation();
        let mut drop_notice = InvocationDropNotice::new(self.clone(), lease.identity());
        let result = tokio::select! {
            answer = sent.block_task() => {
                match answer {
                    Ok(outcome) => self
                        .settle_outcome(&mut lease, outcome, operation, None)
                        .and_then(|result| result.ok_or_else(|| sdk_error(
                            ExtensionErrorCode::ExtensionFailed,
                            "non-streaming invocation returned no result",
                            Retryability::Never,
                            "_echo_agent/extension/invoke",
                        ))),
                    Err(error) => {
                        let typed = transport_error(&error);
                        if typed.code == ExtensionErrorCode::ExtensionDisconnected {
                            lease.settle(echo_agent::acp::ExtensionSettlement::Disconnected);
                        } else {
                            lease.settle(echo_agent::acp::ExtensionSettlement::Answered);
                        }
                        Err(typed)
                    }
                }
            }
            () = cancellation.cancelled() => {
                let _ = self.send_cancel_notice(lease.identity(), "cancelled");
                drop_notice.disarm();
                if !lease.settle(echo_agent::acp::ExtensionSettlement::Cancelled) {
                    return Err(settlement_error(lease.settlement()));
                }
                Err(sdk_error(
                    ExtensionErrorCode::Cancelled,
                    "extension invocation was cancelled",
                    Retryability::Never,
                    "_echo_agent/extension/invoke",
                ))
            }
            _ = tokio::time::sleep(deadline) => {
                let _ = self.send_cancel_notice(lease.identity(), "timeout");
                drop_notice.disarm();
                if !lease.settle_timeout() {
                    return Err(settlement_error(lease.settlement()));
                }
                Err(sdk_error(
                    ExtensionErrorCode::ExtensionTimeout,
                    "extension invocation exceeded its deadline",
                    Retryability::Never,
                    "_echo_agent/extension/invoke",
                ))
            }
        };
        drop_notice.disarm();
        result
    }

    /// Channel calls opt into the framework's driven cancellation contract.
    /// The official connection response task owns the original lease even if
    /// the caller drops its wait; the Channel driver waits for that same real
    /// callback response before releasing its entry invocation receipt.
    fn submit_owned_invocation(
        &self,
        call: ExtensionInvokeCall,
        mut lease: echo_agent::acp::ExtensionInvocationLease,
        call_cancellation: CancellationToken,
        policy: CallbackStreamPolicy,
        owner: Arc<CallbackInvocationOwner>,
        sink: Option<Arc<ExtensionStreamSink>>,
    ) -> std::result::Result<
        tokio::sync::oneshot::Receiver<std::result::Result<Option<ExtensionResult>, EchoSdkError>>,
        EchoSdkError,
    > {
        let connection = self.shared.connection()?;
        let operation = call.invocation.operation();
        let expected_stream = call.stream.clone();
        let deadline = Duration::new(
            call.deadline.seconds.to_u64().ok_or_else(|| {
                sdk_error(
                    ExtensionErrorCode::SerializationViolation,
                    "invalid channel callback deadline",
                    Retryability::Never,
                    "_echo_agent/extension/invoke",
                )
            })?,
            call.deadline.nanos,
        );
        let cancellation = lease.cancellation();
        let invocation_id = lease.identity().to_string();
        let interrupted = Arc::new(Mutex::new(None::<EchoSdkError>));
        let (sender, receiver) = tokio::sync::oneshot::channel();
        let response_interrupted = interrupted.clone();
        let response_owner = owner.clone();
        let response_policy = policy;
        let response_sink = sink.clone();
        let response_cancellation = cancellation.clone();
        let notice_owner = owner.clone();
        let bridge = self.clone();
        let finished = owner.finished.clone();
        connection
            .spawn(async move {
                let timeout = tokio::select! {
                    biased;
                    () = finished.cancelled() => return Ok(()),
                    () = cancellation.cancelled() => false,
                    _ = tokio::time::sleep(deadline) => true,
                };
                let error = sdk_error(
                    if timeout {
                        ExtensionErrorCode::ExtensionTimeout
                    } else {
                        ExtensionErrorCode::Cancelled
                    },
                    if timeout {
                        "channel callback exceeded its deadline"
                    } else {
                        "channel callback was cancelled"
                    },
                    Retryability::Never,
                    "_echo_agent/extension/invoke",
                );
                *interrupted
                    .lock()
                    .unwrap_or_else(|error| error.into_inner()) = Some(error.clone());
                if policy == CallbackStreamPolicy::CustomAgent {
                    *notice_owner
                        .pending_failure
                        .lock()
                        .unwrap_or_else(|error| error.into_inner()) = Some(error.clone());
                }
                if timeout {
                    call_cancellation.cancel();
                }
                let _ = bridge.send_cancel_notice(
                    &invocation_id,
                    if timeout { "timeout" } else { "cancelled" },
                );
                if policy == CallbackStreamPolicy::Channel
                    && let Some(sink) = sink
                {
                    sink.terminate(|sequence| ExtensionStreamEvent::Failed {
                        stream: sink.stream_handle().clone(),
                        sequence,
                        error,
                    });
                }
                // Neither notice nor synthetic terminal releases the real owner.
                Ok(())
            })
            .map_err(|error| transport_error(&error))?;
        if policy == CallbackStreamPolicy::CustomAgent
            && let Some(sink) = response_sink.clone()
        {
            let tracked_owner = owner.clone();
            let tracked_connection = connection.clone();
            connection
                .spawn(async move {
                    tokio::select! {
                        biased;
                        () = tracked_owner.finished.cancelled() => return Ok(()),
                        _ = tracked_connection.incoming_closed() => {},
                        () = tracked_owner.response_completed.cancelled() => {
                            tokio::select! {
                                biased;
                                () = tracked_owner.finished.cancelled() => return Ok(()),
                                _ = tracked_connection.incoming_closed() => {},
                                () = sink.wait_remote_terminal() => {},
                            }
                        }
                    }
                    if tracked_connection.is_incoming_closed() {
                        sink.terminate(|sequence| ExtensionStreamEvent::Failed {
                            stream: sink.stream_handle().clone(),
                            sequence,
                            error: sdk_error(
                                ExtensionErrorCode::ExtensionDisconnected,
                                "custom Agent producer connection closed before its terminal",
                                Retryability::Never,
                                "_echo_agent/extension/stream",
                            ),
                        });
                    }
                    tracked_owner.finished.cancel();
                    tracked_owner
                        .retained_lease
                        .lock()
                        .unwrap_or_else(|error| error.into_inner())
                        .take();
                    Ok(())
                })
                .map_err(|error| transport_error(&error))?;
        }
        let bridge = self.clone();
        // Do not forward cancellation into the transport request: dropping the
        // language callback task can finish its RPC before asynchronous cleanup.
        // The typed notice signals its original cooperative callback instead.
        connection
            .send_request(call)
            .on_receiving_result(async move |answer| {
                let actual_setup_ack = response_policy == CallbackStreamPolicy::CustomAgent
                    && matches!(&answer, Ok(ExtensionInvokeOutcome::Stream { stream })
                        if expected_stream.as_ref() == Some(stream) && stream.validate().is_ok());
                let interruption = response_interrupted
                    .lock()
                    .unwrap_or_else(|error| error.into_inner())
                    .clone()
                    .or_else(|| {
                        response_cancellation.is_cancelled().then(|| {
                            sdk_error(
                                ExtensionErrorCode::Cancelled,
                                "channel callback was cancelled",
                                Retryability::Never,
                                "_echo_agent/extension/invoke",
                            )
                        })
                    });
                let result = if let Some(error) = interruption {
                    let settlement = if error.code == ExtensionErrorCode::ExtensionTimeout {
                        echo_agent::acp::ExtensionSettlement::TimedOut
                    } else {
                        echo_agent::acp::ExtensionSettlement::Cancelled
                    };
                    lease.settle(settlement);
                    if actual_setup_ack {
                        *response_owner
                            .pending_failure
                            .lock()
                            .unwrap_or_else(|error| error.into_inner()) = Some(error);
                        Ok(None)
                    } else {
                        Err(error)
                    }
                } else {
                    match answer {
                        Ok(outcome) => bridge.settle_outcome(
                            &mut lease,
                            outcome,
                            operation,
                            expected_stream.as_ref(),
                        ),
                        Err(error) => {
                            let typed = transport_error(&error);
                            lease.settle(
                                if typed.code == ExtensionErrorCode::ExtensionDisconnected {
                                    echo_agent::acp::ExtensionSettlement::Disconnected
                                } else {
                                    echo_agent::acp::ExtensionSettlement::Answered
                                },
                            );
                            Err(typed)
                        }
                    }
                };
                // A response is a callback-return fact. The lease and session guard
                // must be released before the framework driver observes that fact.
                // Prior stream notifications are routed inline by the ACP
                // dispatch loop before this ordered response callback. At this
                // point callback return without a terminal is an invalid flow,
                // not permission to wait forever for user-detached work.
                if response_policy == CallbackStreamPolicy::Channel
                    && matches!(&result, Ok(None))
                    && let Some(sink) = response_sink
                {
                    sink.terminate(|sequence| ExtensionStreamEvent::Failed {
                        stream: sink.stream_handle().clone(),
                        sequence,
                        error: sdk_error(
                            ExtensionErrorCode::SerializationViolation,
                            "channel callback returned without a stream terminal",
                            Retryability::Never,
                            "_echo_agent/extension/stream",
                        ),
                    });
                }
                if response_policy == CallbackStreamPolicy::CustomAgent
                    && matches!(&result, Ok(None))
                {
                    // The existing CustomAgent protocol permits a setup answer
                    // before its producer sends terminal. Retain this same lease
                    // until both the real answer and declared terminal exist.
                    *response_owner
                        .retained_lease
                        .lock()
                        .unwrap_or_else(|error| error.into_inner()) = Some(lease);
                } else {
                    response_owner.finished.cancel();
                    drop(lease);
                }
                response_owner.response_completed.cancel();
                drop(response_owner);
                let _ = sender.send(result);
                Ok(())
            })
            .map_err(|error| transport_error(&error))?;
        Ok(receiver)
    }

    #[cfg(feature = "framework-channels")]
    async fn invoke_channel_once(
        &self,
        extension: &WireHandle,
        invocation: ExtensionInvocation,
        cancellation: CancellationToken,
    ) -> std::result::Result<ExtensionResult, EchoSdkError> {
        let operation = invocation.operation();
        let (lease, record, _authority) =
            self.prepare(extension, operation, &invocation, cancellation.clone())?;
        self.ensure_payload_bound(&invocation, false)?;
        let call = ExtensionInvokeCall {
            extension: extension.clone(),
            factory_origin: record.factory_origin.clone(),
            invocation_id: lease.identity().to_string(),
            context: None,
            invocation,
            deadline: wire_duration(self.deadline_of(&record)),
            stream: None,
        };
        call.validate().map_err(|reason| {
            sdk_error(
                ExtensionErrorCode::SerializationViolation,
                reason,
                Retryability::Never,
                "_echo_agent/extension/invoke",
            )
        })?;
        let owner = Arc::new(CallbackInvocationOwner {
            _stream: None,
            _session_callback: None,
            _agent_invocation: None,
            retained_lease: Mutex::new(None),
            response_completed: CancellationToken::new(),
            pending_failure: Mutex::new(None),
            finished: CancellationToken::new(),
        });
        let response = self.submit_owned_invocation(
            call,
            lease,
            cancellation,
            CallbackStreamPolicy::Channel,
            owner,
            None,
        )?;
        response
            .await
            .map_err(callback_response_lost)??
            .ok_or_else(|| {
                sdk_error(
                    ExtensionErrorCode::ExtensionFailed,
                    "channel callback returned no single result",
                    Retryability::Never,
                    "_echo_agent/extension/invoke",
                )
            })
    }

    #[cfg(feature = "framework-channels")]
    async fn invoke_channel_stream(
        &self,
        extension: &WireHandle,
        invocation: ExtensionInvocation,
        cancellation: CancellationToken,
    ) -> std::result::Result<
        futures::stream::BoxStream<'static, echo_agent::error::Result<OutboundMessage>>,
        EchoSdkError,
    > {
        let operation = invocation.operation();
        let (lease, record, _authority) =
            self.prepare(extension, operation, &invocation, cancellation.clone())?;
        self.ensure_payload_bound(&invocation, false)?;
        let state = self.state()?;
        let stream = state.handles.register_extension_stream(&extension.id)?;
        let owner = Arc::new(CallbackInvocationOwner {
            _stream: Some(PendingExtensionStream {
                bridge: self.clone(),
                stream: Some(stream.clone()),
            }),
            _session_callback: None,
            _agent_invocation: None,
            retained_lease: Mutex::new(None),
            response_completed: CancellationToken::new(),
            pending_failure: Mutex::new(None),
            finished: CancellationToken::new(),
        });
        let (sink, receiver) = ExtensionStreamSink::new(
            stream.clone(),
            extension.id.clone(),
            operation.kind(),
            state.limits.max_outstanding_live_events,
            None,
        );
        self.shared.register_stream(sink.clone());
        let drop_notice = CallbackStreamDropNotice {
            cancellation: cancellation.clone(),
            armed: true,
        };
        let call = ExtensionInvokeCall {
            extension: extension.clone(),
            factory_origin: record.factory_origin.clone(),
            invocation_id: lease.identity().to_string(),
            context: None,
            invocation,
            deadline: wire_duration(self.deadline_of(&record)),
            stream: Some(stream),
        };
        call.validate().map_err(|reason| {
            sdk_error(
                ExtensionErrorCode::SerializationViolation,
                reason,
                Retryability::Never,
                "_echo_agent/extension/invoke",
            )
        })?;
        let completion = self.submit_owned_invocation(
            call,
            lease,
            cancellation,
            CallbackStreamPolicy::Channel,
            owner.clone(),
            Some(sink.clone()),
        )?;
        // Setup does not wait for the RPC response. Native callbacks may await
        // their entire producer/finally before returning that response.
        Ok(channel_callback_event_stream(
            owner,
            drop_notice,
            sink,
            receiver,
            completion,
        ))
    }

    /// CustomAgent permits the existing setup ACK before stream terminal.
    /// Its explicit framework invocation token never resolves through another
    /// active Run of the registered parent Session.
    async fn invoke_owned_producer_stream(
        &self,
        extension: &WireHandle,
        context: ExtensionInvocationContext,
        invocation: ExtensionInvocation,
        cancellation: CancellationToken,
        agent_invocation: Option<echo_agent::agent::AgentInvocationContext>,
    ) -> std::result::Result<
        futures::stream::BoxStream<
            'static,
            std::result::Result<ExtensionStreamEvent, EchoSdkError>,
        >,
        EchoSdkError,
    > {
        let operation = invocation.operation();
        let (lease, record, _authority) =
            self.prepare(extension, operation, &invocation, cancellation.clone())?;
        self.ensure_payload_bound(&invocation, false)?;
        let state = self.state()?;
        let stream = state.handles.register_extension_stream(&extension.id)?;
        let owner = Arc::new(CallbackInvocationOwner {
            _stream: Some(PendingExtensionStream {
                bridge: self.clone(),
                stream: Some(stream.clone()),
            }),
            _session_callback: self
                .shared
                .enter_session_callback(context.session_id.as_deref()),
            _agent_invocation: agent_invocation,
            retained_lease: Mutex::new(None),
            response_completed: CancellationToken::new(),
            pending_failure: Mutex::new(None),
            finished: CancellationToken::new(),
        });
        let (sink, receiver) = ExtensionStreamSink::new(
            stream.clone(),
            extension.id.clone(),
            operation.kind(),
            state.limits.max_outstanding_live_events,
            None,
        );
        sink.retain_until_remote_terminal
            .store(true, Ordering::Release);
        self.shared.register_stream(sink.clone());
        let drop_notice = CallbackStreamDropNotice {
            cancellation: cancellation.clone(),
            armed: true,
        };
        let call = ExtensionInvokeCall {
            extension: extension.clone(),
            factory_origin: record.factory_origin.clone(),
            invocation_id: lease.identity().to_string(),
            context: Some(context),
            invocation,
            deadline: wire_duration(self.deadline_of(&record)),
            stream: Some(stream),
        };
        call.validate().map_err(|reason| {
            sdk_error(
                ExtensionErrorCode::SerializationViolation,
                reason,
                Retryability::Never,
                "_echo_agent/extension/invoke",
            )
        })?;
        let completion = self.submit_owned_invocation(
            call,
            lease,
            cancellation,
            CallbackStreamPolicy::CustomAgent,
            owner.clone(),
            Some(sink.clone()),
        )?;
        Ok(owned_callback_event_stream(
            owner,
            drop_notice,
            sink,
            receiver,
            completion,
        ))
    }

    /// Drive one streaming reverse invocation: mint the stream handle,
    /// subscribe the sink, return the bounded receiver the Rust stream
    /// proxy consumes.
    pub(crate) async fn invoke_stream(
        &self,
        extension: &WireHandle,
        context: Option<ExtensionInvocationContext>,
        invocation: ExtensionInvocation,
        framework_cancellation: CancellationToken,
    ) -> std::result::Result<
        (
            mpsc::Receiver<ExtensionStreamEvent>,
            WireHandle,
            String,
            CancellationToken,
            Arc<Mutex<Option<echo_agent::acp::ExtensionInvocationLease>>>,
            Arc<ExtensionStreamSink>,
        ),
        EchoSdkError,
    > {
        const OPERATION_STR: &str = "_echo_agent/extension/invoke";
        let operation = invocation.operation();
        debug_assert!(operation.is_streaming());
        let started_at = Instant::now();
        let framework_cancellation = self
            .cancellation_for_context(context.as_ref(), framework_cancellation)
            .await;
        let (mut lease, record, _authority) =
            self.prepare(extension, operation, &invocation, framework_cancellation)?;
        let connection = self.shared.connection()?;
        self.ensure_payload_bound(&invocation, false)?;
        let deadline = self.deadline_of(&record);
        let state = self.state()?;
        let stream = state.handles.register_extension_stream(&extension.id)?;
        let session_callback = self.shared.enter_session_callback(
            context
                .as_ref()
                .and_then(|value| value.session_id.as_deref()),
        );
        let (sink, receiver) = ExtensionStreamSink::new(
            stream.clone(),
            extension.id.clone(),
            operation.kind(),
            state.limits.max_outstanding_live_events,
            session_callback,
        );
        self.shared.register_stream(sink.clone());
        // The enclosing framework timeout can drop this future before the
        // reverse request answers. Until the receiver is handed off, this
        // owner must release both handle quota and the session reentry guard.
        let mut pending_stream = PendingExtensionStream {
            bridge: self.clone(),
            stream: Some(stream.clone()),
        };
        let call = ExtensionInvokeCall {
            extension: extension.clone(),
            factory_origin: record.factory_origin.clone(),
            invocation_id: lease.identity().to_string(),
            context,
            invocation,
            deadline: wire_duration(deadline),
            stream: Some(stream.clone()),
        };
        if let Err(reason) = call.validate() {
            return Err(sdk_error(
                ExtensionErrorCode::SerializationViolation,
                reason,
                Retryability::Never,
                OPERATION_STR,
            ));
        }
        let sent = connection.send_request(call);
        let cancellation = lease.cancellation();
        let mut drop_notice = InvocationDropNotice::new(self.clone(), lease.identity());
        let outcome = tokio::select! {
            answer = sent.block_task() => {
                match answer {
                    Ok(outcome) => outcome,
                    Err(error) => {
                        let typed = transport_error(&error);
                        if typed.code == ExtensionErrorCode::ExtensionDisconnected {
                            lease.settle(echo_agent::acp::ExtensionSettlement::Disconnected);
                        } else {
                            lease.settle(echo_agent::acp::ExtensionSettlement::Answered);
                        }
                        drop_notice.disarm();
                                    return Err(typed);
                    }
                }
            }
            () = cancellation.cancelled() => {
                let _ = self.send_cancel_notice(lease.identity(), "cancelled");
                drop_notice.disarm();
                if !lease.settle(echo_agent::acp::ExtensionSettlement::Cancelled) {
                            return Err(settlement_error(lease.settlement()));
                }
                    return Err(sdk_error(
                    ExtensionErrorCode::Cancelled,
                    "extension stream invocation was cancelled",
                    Retryability::Never,
                    OPERATION_STR,
                ));
            }
            _ = tokio::time::sleep(deadline) => {
                let _ = self.send_cancel_notice(lease.identity(), "timeout");
                drop_notice.disarm();
                if !lease.settle_timeout() {
                            return Err(settlement_error(lease.settlement()));
                }
                    return Err(sdk_error(
                    ExtensionErrorCode::ExtensionTimeout,
                    "extension stream invocation exceeded its deadline",
                    Retryability::Never,
                    OPERATION_STR,
                ));
            }
        };
        drop_notice.disarm();
        let invocation_id = lease.identity().to_string();
        match self.settle_outcome(&mut lease, outcome, operation, Some(&stream)) {
            Ok(None) => {
                let lease = Arc::new(Mutex::new(Some(lease)));
                self.watch_stream(
                    stream.clone(),
                    invocation_id.clone(),
                    cancellation.clone(),
                    deadline.saturating_sub(started_at.elapsed()),
                    lease.clone(),
                );
                pending_stream.stream.take();
                Ok((receiver, stream, invocation_id, cancellation, lease, sink))
            }
            Ok(Some(_)) => Err(sdk_error(
                ExtensionErrorCode::ExtensionFailed,
                "streaming invocation returned a single result",
                Retryability::Never,
                OPERATION_STR,
            )),
            Err(error) => Err(error),
        }
    }

    /// Validate and settle one answer. `expected_stream` enforces the
    /// echoed stream handle for streaming operations.
    fn settle_outcome(
        &self,
        lease: &mut echo_agent::acp::ExtensionInvocationLease,
        outcome: ExtensionInvokeOutcome,
        operation: ExtensionOperation,
        expected_stream: Option<&WireHandle>,
    ) -> std::result::Result<Option<ExtensionResult>, EchoSdkError> {
        if !lease.settle(echo_agent::acp::ExtensionSettlement::Answered) {
            return Err(settlement_error(lease.settlement()));
        }
        if let Err(reason) = outcome.validate() {
            return Err(sdk_error(
                ExtensionErrorCode::SerializationViolation,
                reason,
                Retryability::Never,
                "_echo_agent/extension/invoke",
            ));
        }
        match outcome {
            ExtensionInvokeOutcome::Result { result } => {
                self.ensure_payload_bound(&result, false)?;
                if operation.is_streaming() {
                    return Err(sdk_error(
                        ExtensionErrorCode::ExtensionFailed,
                        "streaming operation answered with a single result",
                        Retryability::Never,
                        "_echo_agent/extension/invoke",
                    ));
                }
                if result.operation() != operation {
                    return Err(sdk_error(
                        ExtensionErrorCode::SerializationViolation,
                        "extension result operation does not match the invocation",
                        Retryability::Never,
                        "_echo_agent/extension/invoke",
                    ));
                }
                Ok(Some(result))
            }
            ExtensionInvokeOutcome::Stream { stream } => {
                let Some(expected) = expected_stream else {
                    return Err(sdk_error(
                        ExtensionErrorCode::ExtensionFailed,
                        "non-streaming operation answered with a stream",
                        Retryability::Never,
                        "_echo_agent/extension/invoke",
                    ));
                };
                if stream != *expected {
                    return Err(sdk_error(
                        ExtensionErrorCode::ExtensionFailed,
                        "stream outcome did not echo the Host-minted stream handle",
                        Retryability::Never,
                        "_echo_agent/extension/invoke",
                    ));
                }
                Ok(None)
            }
            ExtensionInvokeOutcome::Error { error } => Err(error),
        }
    }

    fn release_stream(&self, stream: &WireHandle) {
        self.shared.remove_stream(&stream.id);
        if let Ok(state) = self.state() {
            state.handles.remove_extension_stream(&stream.id);
        }
    }

    fn register_factory_instance(
        &self,
        factory: &WireHandle,
        instance: &WireHandle,
        descriptor: &CustomAgentDescriptorWire,
    ) -> std::result::Result<WireHandle, EchoSdkError> {
        let state = self.state()?;
        let factory_record = state.handles.extension(factory)?;
        if factory_record.kind != ExtensionKind::AgentFactory {
            return Err(sdk_error(
                ExtensionErrorCode::InvalidValue,
                "factory instance creation requires an AgentFactory handle",
                Retryability::Never,
                "_echo_agent/extension/invoke",
            ));
        }
        let custom_descriptor = ExtensionDescriptor::CustomAgent {
            descriptor_version: 1,
            name: descriptor.name.clone(),
            model_name: descriptor.model_name.clone(),
            system_prompt: descriptor.system_prompt.clone(),
            tool_names: descriptor.tool_names.clone(),
        };
        state
            .handles
            .complete_factory_instance_reserved(
                super::handles::ExtensionRegistrationLimits {
                    max_extensions: state.limits.max_registered_extensions,
                    max_descriptor_bytes: state.limits.max_extension_descriptor_bytes,
                },
                instance,
                factory,
                custom_descriptor,
            )
            .map(|_| instance.clone())
    }

    fn release_extension(&self, extension: &WireHandle) {
        if let Ok(state) = self.state()
            && let Ok(true) = state.handles.close_extension(extension)
        {
            self.shared.remove_streams_for_extension(&extension.id);
        }
    }

    fn ensure_payload_bound<T: serde::Serialize>(
        &self,
        value: &T,
        stream_chunk: bool,
    ) -> std::result::Result<(), EchoSdkError> {
        let encoded = serde_json::to_vec(value).map_err(|error| {
            sdk_error(
                ExtensionErrorCode::SerializationViolation,
                format!("extension payload is not encodable: {error}"),
                Retryability::Never,
                "_echo_agent/extension/invoke",
            )
        })?;
        let state = self.state()?;
        let limit = if stream_chunk {
            state.limits.max_extension_stream_bytes
        } else {
            state.limits.max_extension_payload_bytes
        };
        if encoded.len() > limit {
            return Err(sdk_error(
                ExtensionErrorCode::PayloadTooLarge,
                "extension payload exceeds the configured byte bound",
                Retryability::Never,
                "_echo_agent/extension/invoke",
            ));
        }
        Ok(())
    }

    /// Best-effort typed cancel notice; the official `$/cancel_request` is
    /// sent automatically when the `SentRequest` handle drops.
    fn send_cancel_notice(&self, invocation_id: &str, reason: &str) -> Result<(), String> {
        let notice = echo_sdk_protocol::methods::ExtensionCancelNotice {
            invocation_id: invocation_id.to_string(),
            reason: reason.to_string(),
        };
        if let Err(reason) = notice.validate() {
            return Err(reason.to_string());
        }
        let connection = self.shared.connection().map_err(|error| error.message)?;
        connection
            .send_notification(notice)
            .map(|_| ())
            .map_err(|error| error.to_string())
    }

    fn watch_stream(
        &self,
        stream: WireHandle,
        invocation_id: String,
        cancellation: CancellationToken,
        deadline: Duration,
        lease: Arc<Mutex<Option<echo_agent::acp::ExtensionInvocationLease>>>,
    ) {
        let bridge = self.clone();
        let sink = self.shared.stream_sink(&stream.id);
        tokio::spawn(async move {
            let Some(sink) = sink else {
                return;
            };
            tokio::select! {
                _ = sink.wait_closed() => {
                    drop_stream_lease(&lease);
                }
                _ = cancellation.cancelled() => {
                    sink.terminate(|sequence| ExtensionStreamEvent::Cancelled {
                        stream: stream.clone(),
                        sequence,
                    });
                    let _ = bridge.send_cancel_notice(&invocation_id, "cancelled");
                    drop_stream_lease(&lease);
                    bridge.release_stream(&stream);
                }
                _ = tokio::time::sleep(deadline) => {
                    cancellation.cancel();
                    sink.terminate(|sequence| ExtensionStreamEvent::Failed {
                        stream: stream.clone(),
                        sequence,
                        error: sdk_error(
                            ExtensionErrorCode::ExtensionTimeout,
                            "extension stream invocation exceeded its deadline",
                            Retryability::Never,
                            "_echo_agent/extension/stream",
                        ),
                    });
                    let _ = bridge.send_cancel_notice(&invocation_id, "timeout");
                    drop_stream_lease(&lease);
                    bridge.release_stream(&stream);
                }
            }
        });
    }
}

fn wire_duration(duration: Duration) -> echo_sdk_protocol::scalar::WireDuration {
    echo_sdk_protocol::scalar::WireDuration {
        seconds: WireU64::from_u64(duration.as_secs()),
        nanos: duration.subsec_nanos(),
    }
}

/// Sends the extension-level cancellation notice when an invocation future is
/// dropped by its owning run before the normal select branches can settle.
struct InvocationDropNotice {
    bridge: ExtensionBridge,
    invocation_id: String,
    armed: bool,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum CallbackStreamPolicy {
    Channel,
    CustomAgent,
}

struct CallbackStreamDropNotice {
    cancellation: CancellationToken,
    armed: bool,
}

impl Drop for CallbackStreamDropNotice {
    fn drop(&mut self) {
        if self.armed {
            self.cancellation.cancel();
        }
    }
}

struct CallbackInvocationOwner {
    _stream: Option<PendingExtensionStream>,
    _session_callback: Option<SessionCallbackGuard>,
    _agent_invocation: Option<echo_agent::agent::AgentInvocationContext>,
    retained_lease: Mutex<Option<echo_agent::acp::ExtensionInvocationLease>>,
    response_completed: CancellationToken,
    pending_failure: Mutex<Option<EchoSdkError>>,
    finished: CancellationToken,
}

impl Drop for CallbackInvocationOwner {
    fn drop(&mut self) {
        self.finished.cancel();
    }
}

struct CallbackStreamState {
    _owner: Arc<CallbackInvocationOwner>,
    drop_notice: CallbackStreamDropNotice,
    sink: Arc<ExtensionStreamSink>,
    receiver: mpsc::Receiver<ExtensionStreamEvent>,
    completion:
        tokio::sync::oneshot::Receiver<std::result::Result<Option<ExtensionResult>, EchoSdkError>>,
    answered: bool,
    terminal: Option<ExtensionStreamEvent>,
    sink_ended: bool,
    finished: bool,
}

#[cfg(feature = "framework-channels")]
fn channel_callback_event_stream(
    owner: Arc<CallbackInvocationOwner>,
    drop_notice: CallbackStreamDropNotice,
    sink: Arc<ExtensionStreamSink>,
    receiver: mpsc::Receiver<ExtensionStreamEvent>,
    completion: tokio::sync::oneshot::Receiver<
        std::result::Result<Option<ExtensionResult>, EchoSdkError>,
    >,
) -> futures::stream::BoxStream<'static, echo_agent::error::Result<OutboundMessage>> {
    owned_callback_event_stream(owner, drop_notice, sink, receiver, completion)
        .map(|event| event.map_err(react_error).and_then(channel_stream_message))
        .boxed()
}

fn owned_callback_event_stream(
    owner: Arc<CallbackInvocationOwner>,
    drop_notice: CallbackStreamDropNotice,
    sink: Arc<ExtensionStreamSink>,
    receiver: mpsc::Receiver<ExtensionStreamEvent>,
    completion: tokio::sync::oneshot::Receiver<
        std::result::Result<Option<ExtensionResult>, EchoSdkError>,
    >,
) -> futures::stream::BoxStream<'static, std::result::Result<ExtensionStreamEvent, EchoSdkError>> {
    Box::pin(futures::stream::unfold(
        CallbackStreamState {
            _owner: owner,
            drop_notice,
            sink,
            receiver,
            completion,
            answered: false,
            terminal: None,
            sink_ended: false,
            finished: false,
        },
        async move |mut state| {
            if state.finished {
                return None;
            }
            loop {
                if (state.terminal.is_some() || state.sink_ended)
                    && state.answered
                    && (!state
                        .sink
                        .retain_until_remote_terminal
                        .load(Ordering::Acquire)
                        || state.sink.remote_terminal_received.load(Ordering::Acquire)
                        || state._owner.finished.is_cancelled())
                {
                    let pending_failure = state
                        ._owner
                        .pending_failure
                        .lock()
                        .unwrap_or_else(|error| error.into_inner())
                        .take();
                    let item = if let Some(error) = pending_failure {
                        Err(error)
                    } else {
                        state.terminal.take().ok_or_else(|| {
                            sdk_error(
                                ExtensionErrorCode::ExtensionFailed,
                                "callback stream sink ended without a terminal",
                                Retryability::Never,
                                "_echo_agent/extension/stream",
                            )
                        })
                    };
                    state.finished = true;
                    state.drop_notice.armed = false;
                    return Some((item, state));
                }
                if state.receiver.is_empty()
                    && let Some(event) = state.sink.take_terminal()
                {
                    state.terminal = Some(event);
                    continue;
                }
                tokio::select! {
                    response = &mut state.completion, if !state.answered => {
                        state.answered = true;
                        match response.map_err(callback_response_lost).and_then(|result| result) {
                            Ok(None) => {},
                            result => {
                                state.finished = true;
                                state.drop_notice.armed = false;
                                let error = match result {
                                    Err(error) => error,
                                    Ok(_) => sdk_error(ExtensionErrorCode::ExtensionFailed,
                                        "callback stream returned a single result", Retryability::Never,
                                        "_echo_agent/extension/invoke"),
                                };
                                return Some((Err(error), state));
                            }
                        }
                    }
                    () = state.sink.wait_remote_terminal(), if state.answered
                        && state.sink.retain_until_remote_terminal.load(Ordering::Acquire)
                        && !state.sink.remote_terminal_received.load(Ordering::Acquire) => {},
                    () = state._owner.finished.cancelled(), if state.answered
                        && state.sink.retain_until_remote_terminal.load(Ordering::Acquire)
                        && !state._owner.finished.is_cancelled() => {},
                    event = state.receiver.recv(), if state.terminal.is_none() && !state.sink_ended => {
                        match event {
                            Some(event) if event.is_terminal() => { state.terminal = Some(event); }
                            Some(event) => { return Some((Ok(event), state)); }
                            None => {
                                // Closed notification delivery is not callback completion.
                                state.terminal = state.sink.take_terminal();
                                state.sink_ended = true;
                            }
                        }
                    }
                }
            }
        },
    ))
}

fn callback_response_lost(error: tokio::sync::oneshot::error::RecvError) -> EchoSdkError {
    sdk_error(
        ExtensionErrorCode::ExtensionDisconnected,
        format!("channel response owner ended without a callback response: {error}"),
        Retryability::Never,
        "_echo_agent/extension/invoke",
    )
}

#[cfg(feature = "framework-channels")]
fn channel_stream_message(
    event: ExtensionStreamEvent,
) -> echo_agent::error::Result<OutboundMessage> {
    match event {
        ExtensionStreamEvent::Chunk {
            value: ExtensionStreamChunkValue::Channel(message),
            ..
        }
        | ExtensionStreamEvent::Complete {
            value: ExtensionStreamCompleteValue::Channel(message),
            ..
        } => channel_outbound_from_wire(message),
        ExtensionStreamEvent::Failed { error, .. } => Err(react_error(error)),
        ExtensionStreamEvent::Cancelled { .. } => Err(react_error(sdk_error(
            ExtensionErrorCode::Cancelled,
            "channel handler stream was cancelled",
            Retryability::Never,
            "_echo_agent/extension/stream",
        ))),
        _ => Err(ReactError::Other(
            "channel handler stream received a non-channel payload".to_string(),
        )),
    }
}

struct PendingExtensionStream {
    bridge: ExtensionBridge,
    stream: Option<WireHandle>,
}

impl Drop for PendingExtensionStream {
    fn drop(&mut self) {
        if let Some(stream) = self.stream.take() {
            self.bridge.release_stream(&stream);
        }
    }
}

impl InvocationDropNotice {
    fn new(bridge: ExtensionBridge, invocation_id: &str) -> Self {
        Self {
            bridge,
            invocation_id: invocation_id.to_string(),
            armed: true,
        }
    }

    fn disarm(&mut self) {
        self.armed = false;
    }
}

impl Drop for InvocationDropNotice {
    fn drop(&mut self) {
        if self.armed {
            let _ = self
                .bridge
                .send_cancel_notice(&self.invocation_id, "cancelled");
        }
    }
}

// ── Wire value conversions ──────────────────────────────────────────────────

fn to_wire(value: impl serde::Serialize) -> std::result::Result<WireValue, String> {
    let json = serde_json::to_value(value).map_err(|error| error.to_string())?;
    WireValue::from_json(json).map_err(|error| error.to_string())
}

fn from_wire<T: serde::de::DeserializeOwned>(value: WireValue) -> std::result::Result<T, String> {
    let json = value.into_json().map_err(|error| error.to_string())?;
    serde_json::from_value(json).map_err(|error| error.to_string())
}

/// Build the callback-event stream that ends exactly at the terminal.
///
/// The sink keeps its bounded `Sender` registered for late-delivery
/// admission, so the raw receiver would never close on its own: ending at
/// the terminal (and releasing the stream resources then) is what makes the
/// exactly-one-terminal contract observable to the Rust consumer.
fn extension_event_stream(
    bridge: Arc<ExtensionBridge>,
    stream: WireHandle,
    invocation_id: String,
    cancellation: CancellationToken,
    lease: Arc<Mutex<Option<echo_agent::acp::ExtensionInvocationLease>>>,
    sink: Arc<ExtensionStreamSink>,
    receiver: mpsc::Receiver<ExtensionStreamEvent>,
) -> ExtensionEventStream {
    ExtensionEventStream {
        bridge,
        stream,
        invocation_id,
        cancellation,
        lease,
        sink,
        receiver,
        finished: false,
    }
}

struct ExtensionEventStream {
    bridge: Arc<ExtensionBridge>,
    stream: WireHandle,
    invocation_id: String,
    cancellation: CancellationToken,
    lease: Arc<Mutex<Option<echo_agent::acp::ExtensionInvocationLease>>>,
    sink: Arc<ExtensionStreamSink>,
    receiver: mpsc::Receiver<ExtensionStreamEvent>,
    finished: bool,
}

impl Stream for ExtensionEventStream {
    type Item = ExtensionStreamEvent;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.finished {
            return Poll::Ready(None);
        }
        if self.receiver.is_empty()
            && let Some(event) = self.sink.take_terminal()
        {
            self.finished = true;
            drop_stream_lease(&self.lease);
            self.bridge.release_stream(&self.stream);
            return Poll::Ready(Some(event));
        }
        match Pin::new(&mut self.receiver).poll_recv(cx) {
            Poll::Ready(Some(event)) => {
                if event.is_terminal() {
                    self.finished = true;
                    self.sink.clear_terminal();
                    drop_stream_lease(&self.lease);
                    self.bridge.release_stream(&self.stream);
                }
                Poll::Ready(Some(event))
            }
            Poll::Ready(None) => {
                self.finished = true;
                drop_stream_lease(&self.lease);
                self.bridge.release_stream(&self.stream);
                Poll::Ready(None)
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

impl Drop for ExtensionEventStream {
    fn drop(&mut self) {
        if !self.finished {
            self.cancellation.cancel();
            let _ = self
                .bridge
                .send_cancel_notice(&self.invocation_id, "cancelled");
            drop_stream_lease(&self.lease);
            self.bridge.release_stream(&self.stream);
        }
    }
}

fn drop_stream_lease(lease: &Arc<Mutex<Option<echo_agent::acp::ExtensionInvocationLease>>>) {
    if let Ok(mut lease) = lease.lock() {
        lease.take();
    }
}

fn react_error(error: EchoSdkError) -> ReactError {
    let message = format!(
        "extension bridge failure {}: {}",
        error.code.as_str(),
        error.message
    );
    match error.code {
        ExtensionErrorCode::Cancelled => echo_agent::error::AgentError::Cancelled(message).into(),
        ExtensionErrorCode::ExtensionTimeout => {
            echo_agent::error::AgentError::Timeout(message).into()
        }
        _ => ReactError::Other(message),
    }
}

fn tool_react_error(error: EchoSdkError) -> ReactError {
    let message = format!(
        "extension bridge failure {}: {}",
        error.code.as_str(),
        error.message
    );
    match error.code {
        ExtensionErrorCode::Cancelled => echo_agent::error::ToolError::Cancelled(message).into(),
        ExtensionErrorCode::ExtensionTimeout => {
            echo_agent::error::ToolError::Timeout(message).into()
        }
        _ => react_error(error),
    }
}

fn managed_call_context(call: &AgentComponentCallInputWire) -> Option<&PersistenceCallContextWire> {
    match call {
        AgentComponentCallInputWire::ConversationEnsureProjectionEpochWithContext {
            context,
            ..
        } => Some(context),
        AgentComponentCallInputWire::ConversationGetProjectionAuthorityWithContext {
            context,
            ..
        } => Some(context),
        AgentComponentCallInputWire::ConversationApplyTranscriptProjectionWithContext {
            context,
            ..
        } => Some(context),
        AgentComponentCallInputWire::ConversationDeleteManagedWithContext { context, .. } => {
            Some(context)
        }
        AgentComponentCallInputWire::RuntimeLoadManagedStateWithContext { context, .. } => {
            Some(context)
        }
        AgentComponentCallInputWire::RuntimeLoadScopeAuthorityWithContext { context, .. } => {
            Some(context)
        }
        AgentComponentCallInputWire::RuntimeCompareAndSaveCheckpointWithContext {
            context, ..
        } => Some(context),
        AgentComponentCallInputWire::RuntimeAcknowledgeTranscriptProjectionWithContext {
            context,
            ..
        } => Some(context),
        AgentComponentCallInputWire::RuntimeRetireGenerationWithContext { context, .. } => {
            Some(context)
        }
        AgentComponentCallInputWire::RuntimeBeginScopeRetirementWithContext { context, .. } => {
            Some(context)
        }
        AgentComponentCallInputWire::RuntimeContinueScopeRetirementWithContext {
            context, ..
        } => Some(context),
        _ => None,
    }
}

fn managed_persistence_error(error: EchoSdkError, component: AgentComponentKindWire) -> ReactError {
    let class = error
        .details
        .as_ref()
        .and_then(|details| details.fields.as_ref())
        .and_then(|fields| {
            fields
                .iter()
                .find(|field| field.key == "persistence_error_class")
        })
        .map(|field| field.value.as_str());
    let message = format!(
        "extension bridge failure {}: {}",
        error.code.as_str(),
        error.message
    );
    match (component, class) {
        (AgentComponentKindWire::ConversationStore, Some("transient_no_commit")) => {
            echo_agent::error::MemoryError::TransientNoCommit(message).into()
        }
        (AgentComponentKindWire::ConversationStore, Some("deadline_exceeded")) => {
            echo_agent::error::MemoryError::DeadlineExceeded(message).into()
        }
        (AgentComponentKindWire::ConversationStore, Some("unsupported")) => {
            echo_agent::error::MemoryError::Unsupported(message).into()
        }
        (AgentComponentKindWire::ConversationStore, Some("corrupt_state")) => {
            echo_agent::error::MemoryError::SerializationError(message).into()
        }
        (AgentComponentKindWire::ConversationStore, Some("invalid_configuration")) => {
            echo_agent::error::ConfigError::ConfigFileError(message).into()
        }
        (AgentComponentKindWire::RuntimeStateStore, Some("transient_no_commit")) => {
            echo_agent::error::RuntimeStateError::TransientNoCommit(message).into()
        }
        (AgentComponentKindWire::RuntimeStateStore, Some("deadline_exceeded")) => {
            echo_agent::error::RuntimeStateError::DeadlineExceeded(message).into()
        }
        (AgentComponentKindWire::RuntimeStateStore, Some("unsupported")) => {
            echo_agent::error::RuntimeStateError::Unsupported(message).into()
        }
        (AgentComponentKindWire::RuntimeStateStore, Some("corrupt_state")) => {
            echo_agent::error::RuntimeStateError::SerializationError(message).into()
        }
        (AgentComponentKindWire::RuntimeStateStore, Some("invalid_configuration")) => {
            echo_agent::error::ConfigError::ConfigFileError(message).into()
        }
        // Conflict is a domain result with an authoritative receipt. An
        // exception claiming conflict lacks that receipt and remains unknown.
        _ => ReactError::Other(message),
    }
}

fn sandbox_react_error(error: EchoSdkError) -> ReactError {
    use echo_agent::error::SandboxError;
    let message = format!(
        "extension bridge failure {}: {}",
        error.code.as_str(),
        error.message
    );
    let error = match error.code {
        ExtensionErrorCode::Cancelled => SandboxError::Cancelled(message),
        ExtensionErrorCode::ExtensionTimeout => SandboxError::Timeout(message),
        ExtensionErrorCode::FeatureUnavailable => SandboxError::Unavailable(message),
        ExtensionErrorCode::ExtensionRejected => SandboxError::PermissionDenied(message),
        _ => SandboxError::IoError(message),
    };
    ReactError::Sandbox(Box::new(error))
}

fn settlement_error(settlement: Option<echo_agent::acp::ExtensionSettlement>) -> EchoSdkError {
    match settlement {
        Some(echo_agent::acp::ExtensionSettlement::Cancelled) => sdk_error(
            ExtensionErrorCode::Cancelled,
            "extension invocation was cancelled before its response was applied",
            Retryability::Never,
            "_echo_agent/extension/invoke",
        ),
        Some(echo_agent::acp::ExtensionSettlement::TimedOut) => sdk_error(
            ExtensionErrorCode::ExtensionTimeout,
            "extension invocation exceeded its deadline before its response was applied",
            Retryability::Never,
            "_echo_agent/extension/invoke",
        ),
        Some(echo_agent::acp::ExtensionSettlement::Disconnected) => sdk_error(
            ExtensionErrorCode::ExtensionDisconnected,
            "extension connection closed before its response was applied",
            Retryability::Never,
            "_echo_agent/extension/invoke",
        ),
        Some(echo_agent::acp::ExtensionSettlement::Answered) | None => sdk_error(
            ExtensionErrorCode::ExtensionFailed,
            "extension invocation settled concurrently without a usable response",
            Retryability::Never,
            "_echo_agent/extension/invoke",
        ),
    }
}

fn cancelled_token() -> CancellationToken {
    CancellationToken::new()
}

fn cancelled_token_arc() -> Arc<CancellationToken> {
    Arc::new(CancellationToken::new())
}

// ── Tool proxy ──────────────────────────────────────────────────────────────

/// Serializable projection of [`ToolContext`]: the identity facts a callback
/// needs. Closures, sinks and guards never cross the wire; cancellation is
/// carried by the invocation envelope, not the context.
fn tool_context_wire(context: &ToolContext) -> std::result::Result<ToolContextWire, String> {
    Ok(ToolContextWire {
        working_dir: context.working_dir.as_deref().map(wire_path),
        conversation_id: context.conversation_id.clone(),
        run_id: context.run_id.clone(),
        turn_id: context.turn_id.clone(),
        message_id: context.message_id.clone(),
        execution_id: context.execution_id.clone(),
        call_id: context.call_id.clone(),
        active_message: context
            .active_message
            .as_ref()
            .map(message_wire)
            .transpose()?,
        output_artifacts: context.output_artifacts.as_ref().map(|config| {
            ToolOutputArtifactConfigWire {
                root_dir: wire_path(&config.root_dir),
                retention: config.retention.clone(),
                threshold_bytes: WireU64::from_u64(
                    u64::try_from(config.threshold_bytes).unwrap_or(u64::MAX),
                ),
                max_age_secs: config.max_age_secs.map(WireU64::from_u64),
            }
        }),
    })
}

fn wire_path(path: &std::path::Path) -> WirePath {
    #[cfg(unix)]
    {
        use std::os::unix::ffi::OsStrExt as _;
        let bytes = path.as_os_str().as_bytes();
        WirePath::Unix {
            bytes_base64: base64::engine::general_purpose::STANDARD_NO_PAD.encode(bytes),
            display: path.to_str().map(str::to_string),
        }
    }
    #[cfg(windows)]
    {
        use std::os::windows::ffi::OsStrExt as _;
        let units = path
            .as_os_str()
            .encode_wide()
            .flat_map(u16::to_le_bytes)
            .collect::<Vec<_>>();
        WirePath::Windows {
            utf16_base64: base64::engine::general_purpose::STANDARD_NO_PAD.encode(units),
            display: path.to_str().map(str::to_string),
        }
    }
    #[cfg(not(any(unix, windows)))]
    {
        WirePath::Utf8 {
            path: path.to_string_lossy().into_owned(),
        }
    }
}

fn invocation_context(context: &ToolContext) -> Option<ExtensionInvocationContext> {
    let value = ExtensionInvocationContext {
        session_id: context.conversation_id.clone(),
        run_id: context.run_id.clone(),
        stream_id: None,
        turn_id: context.turn_id.clone(),
        message_id: context.message_id.clone(),
        execution_id: context.execution_id.clone(),
        call_id: context.call_id.clone(),
    };
    (value.session_id.is_some()
        || value.run_id.is_some()
        || value.turn_id.is_some()
        || value.message_id.is_some()
        || value.execution_id.is_some()
        || value.call_id.is_some())
    .then_some(value)
}

fn session_invocation_context(session_id: &str) -> ExtensionInvocationContext {
    ExtensionInvocationContext {
        session_id: Some(session_id.to_string()),
        run_id: None,
        stream_id: None,
        turn_id: None,
        message_id: None,
        execution_id: None,
        call_id: None,
    }
}

fn tool_stream_event_from_wire(
    wire: ToolStreamEventWire,
) -> std::result::Result<ToolStreamEvent, String> {
    Ok(match wire {
        ToolStreamEventWire::Progress { message, percent } => {
            ToolStreamEvent::Progress { message, percent }
        }
        ToolStreamEventWire::Output { channel, chunk } => ToolStreamEvent::Output {
            channel: match channel.as_str() {
                "stdout" => echo_agent::tools::ToolOutputChannel::Stdout,
                "stderr" => echo_agent::tools::ToolOutputChannel::Stderr,
                "log" => echo_agent::tools::ToolOutputChannel::Log,
                _ => return Err("unknown tool output channel".to_string()),
            },
            chunk,
        },
        ToolStreamEventWire::Complete { result } => {
            ToolStreamEvent::Complete(tool_result_from_wire(result)?)
        }
    })
}

fn tool_stream_chunk_from_wire(
    wire: ToolStreamChunkWire,
) -> std::result::Result<ToolStreamEvent, String> {
    match wire {
        ToolStreamChunkWire::Progress { message, percent } => {
            Ok(ToolStreamEvent::Progress { message, percent })
        }
        ToolStreamChunkWire::Output { channel, chunk } => {
            tool_stream_event_from_wire(ToolStreamEventWire::Output { channel, chunk })
        }
    }
}

#[allow(dead_code)]
fn tool_stream_event_wire_from_framework(
    event: &ToolStreamEvent,
) -> std::result::Result<ToolStreamEventWire, String> {
    Ok(match event {
        ToolStreamEvent::Progress { message, percent } => ToolStreamEventWire::Progress {
            message: message.clone(),
            percent: *percent,
        },
        ToolStreamEvent::Output { channel, chunk } => ToolStreamEventWire::Output {
            channel: match channel {
                echo_agent::tools::ToolOutputChannel::Stdout => "stdout",
                echo_agent::tools::ToolOutputChannel::Stderr => "stderr",
                echo_agent::tools::ToolOutputChannel::Log => "log",
            }
            .to_string(),
            chunk: chunk.clone(),
        },
        ToolStreamEvent::Complete(result) => ToolStreamEventWire::Complete {
            result: tool_result_to_wire(result)?,
        },
    })
}

#[allow(dead_code)]
fn agent_event_wire_from_framework(
    event: &AgentEvent,
) -> std::result::Result<AgentEventWire, String> {
    Ok(match event {
        AgentEvent::Token(text) => AgentEventWire::Token { text: text.clone() },
        AgentEvent::ThinkStart => AgentEventWire::ThinkStart,
        AgentEvent::ThinkEnd {
            prompt_tokens,
            completion_tokens,
        } => AgentEventWire::ThinkEnd {
            prompt_tokens: WireU64::from_u64(u64::try_from(*prompt_tokens).unwrap_or(u64::MAX)),
            completion_tokens: WireU64::from_u64(
                u64::try_from(*completion_tokens).unwrap_or(u64::MAX),
            ),
        },
        AgentEvent::LlmUsage {
            model,
            prompt_tokens,
            completion_tokens,
            total_tokens,
            cached_prompt_tokens,
            cache_creation_prompt_tokens,
            usage_reported,
        } => AgentEventWire::LlmUsage {
            model: model.clone(),
            prompt_tokens: WireU64::from_u64(u64::try_from(*prompt_tokens).unwrap_or(u64::MAX)),
            completion_tokens: WireU64::from_u64(
                u64::try_from(*completion_tokens).unwrap_or(u64::MAX),
            ),
            total_tokens: WireU64::from_u64(u64::try_from(*total_tokens).unwrap_or(u64::MAX)),
            cached_prompt_tokens: WireU64::from_u64(
                u64::try_from(*cached_prompt_tokens).unwrap_or(u64::MAX),
            ),
            cache_creation_prompt_tokens: WireU64::from_u64(
                u64::try_from(*cache_creation_prompt_tokens).unwrap_or(u64::MAX),
            ),
            usage_reported: *usage_reported,
        },
        AgentEvent::BudgetDecision {
            decision,
            reason,
            iteration,
            reported_model_tokens,
            usage_complete,
        } => AgentEventWire::BudgetDecision {
            decision: to_wire(decision)?,
            reason: reason.clone(),
            iteration: WireU64::from_u64(u64::try_from(*iteration).unwrap_or(u64::MAX)),
            reported_model_tokens: WireU64::from_u64(
                u64::try_from(*reported_model_tokens).unwrap_or(u64::MAX),
            ),
            usage_complete: *usage_complete,
        },
        AgentEvent::ToolCall {
            call_id,
            invocation,
        } => AgentEventWire::ToolCall {
            call_id: call_id.clone(),
            invocation: to_wire(invocation)?,
        },
        AgentEvent::ToolResult {
            call_id,
            name,
            result,
        } => AgentEventWire::ToolResult {
            call_id: call_id.clone(),
            name: name.clone(),
            result: tool_result_to_wire(result)?,
        },
        AgentEvent::ToolStream {
            call_id,
            name,
            event,
        } => AgentEventWire::ToolStream {
            call_id: call_id.clone(),
            name: name.clone(),
            event: tool_stream_event_wire_from_framework(event)?,
        },
        AgentEvent::ToolBatchStart { tool_count } => AgentEventWire::ToolBatchStart {
            tool_count: WireU64::from_u64(u64::try_from(*tool_count).unwrap_or(u64::MAX)),
        },
        AgentEvent::ToolBatchEnd => AgentEventWire::ToolBatchEnd,
        AgentEvent::GuardTriggered { guard, blocked } => AgentEventWire::GuardTriggered {
            guard: guard.clone(),
            blocked: *blocked,
        },
        AgentEvent::TranscriptProjectionSettlement(settlement) => {
            AgentEventWire::TranscriptProjectionSettlement {
                settlement: settlement.clone().project()?,
            }
        }
        AgentEvent::MemoryRecalled { count } => AgentEventWire::MemoryRecalled {
            count: WireU64::from_u64(u64::try_from(*count).unwrap_or(u64::MAX)),
        },
        AgentEvent::ContextCompressed {
            before_count,
            after_count,
            before_tokens,
            after_tokens,
        } => AgentEventWire::ContextCompressed {
            before_count: WireU64::from_u64(u64::try_from(*before_count).unwrap_or(u64::MAX)),
            after_count: WireU64::from_u64(u64::try_from(*after_count).unwrap_or(u64::MAX)),
            before_tokens: WireU64::from_u64(u64::try_from(*before_tokens).unwrap_or(u64::MAX)),
            after_tokens: WireU64::from_u64(u64::try_from(*after_tokens).unwrap_or(u64::MAX)),
        },
        AgentEvent::Chart { spec } => AgentEventWire::Chart {
            spec: to_wire(spec)?,
        },
        AgentEvent::Error {
            source,
            message,
            failure,
        } => AgentEventWire::Error {
            source: source.clone(),
            message: message.clone(),
            failure: to_wire(failure)?,
        },
        AgentEvent::SafetyNotice {
            action,
            reason,
            risk,
            permission,
        } => AgentEventWire::SafetyNotice {
            action: action.clone(),
            reason: reason.clone(),
            risk: risk.clone(),
            permission: permission.clone(),
        },
        AgentEvent::ParameterError {
            tool,
            parameter,
            expected,
            got,
        } => AgentEventWire::ParameterError {
            tool: tool.clone(),
            parameter: parameter.clone(),
            expected: expected.clone(),
            got: got.clone(),
        },
        AgentEvent::FinalAnswer(text) => AgentEventWire::FinalAnswer { text: text.clone() },
        AgentEvent::Cancelled => AgentEventWire::Cancelled,
        _ => return Err("unsupported framework AgentEvent variant".to_string()),
    })
}

fn agent_event_from_wire(wire: AgentEventWire) -> std::result::Result<AgentEvent, String> {
    Ok(match wire {
        AgentEventWire::Token { text } => AgentEvent::Token(text),
        AgentEventWire::ThinkStart => AgentEvent::ThinkStart,
        AgentEventWire::ThinkEnd {
            prompt_tokens,
            completion_tokens,
        } => AgentEvent::ThinkEnd {
            prompt_tokens: wire_usize(prompt_tokens, "prompt token count")?,
            completion_tokens: wire_usize(completion_tokens, "completion token count")?,
        },
        AgentEventWire::LlmUsage {
            model,
            prompt_tokens,
            completion_tokens,
            total_tokens,
            cached_prompt_tokens,
            cache_creation_prompt_tokens,
            usage_reported,
        } => AgentEvent::LlmUsage {
            model,
            prompt_tokens: wire_usize(prompt_tokens, "prompt token count")?,
            completion_tokens: wire_usize(completion_tokens, "completion token count")?,
            total_tokens: wire_usize(total_tokens, "total token count")?,
            cached_prompt_tokens: wire_usize(cached_prompt_tokens, "cached prompt token count")?,
            cache_creation_prompt_tokens: wire_usize(
                cache_creation_prompt_tokens,
                "cache creation prompt token count",
            )?,
            usage_reported,
        },
        AgentEventWire::BudgetDecision {
            decision,
            reason,
            iteration,
            reported_model_tokens,
            usage_complete,
        } => AgentEvent::BudgetDecision {
            decision: from_wire(decision)?,
            reason,
            iteration: wire_usize(iteration, "budget iteration")?,
            reported_model_tokens: wire_usize(reported_model_tokens, "reported model token count")?,
            usage_complete,
        },
        AgentEventWire::ToolCall {
            call_id,
            invocation,
        } => AgentEvent::ToolCall {
            call_id,
            invocation: from_wire(invocation)?,
        },
        AgentEventWire::ToolResult {
            call_id,
            name,
            result,
        } => AgentEvent::ToolResult {
            call_id,
            name,
            result: tool_result_from_wire(result)?,
        },
        AgentEventWire::ToolStream {
            call_id,
            name,
            event,
        } => AgentEvent::ToolStream {
            call_id,
            name,
            event: tool_stream_event_from_wire(event)?,
        },
        AgentEventWire::ToolBatchStart { tool_count } => AgentEvent::ToolBatchStart {
            tool_count: wire_usize(tool_count, "tool batch count")?,
        },
        AgentEventWire::ToolBatchEnd => AgentEvent::ToolBatchEnd,
        AgentEventWire::GuardTriggered { guard, blocked } => {
            AgentEvent::GuardTriggered { guard, blocked }
        }
        AgentEventWire::TranscriptProjectionSettlement { settlement } => {
            AgentEvent::TranscriptProjectionSettlement(super::wire::managed::Projection::restore(
                settlement,
            )?)
        }
        AgentEventWire::MemoryRecalled { count } => AgentEvent::MemoryRecalled {
            count: wire_usize(count, "recalled memory count")?,
        },
        AgentEventWire::ContextCompressed {
            before_count,
            after_count,
            before_tokens,
            after_tokens,
        } => AgentEvent::ContextCompressed {
            before_count: wire_usize(before_count, "pre-compression message count")?,
            after_count: wire_usize(after_count, "post-compression message count")?,
            before_tokens: wire_usize(before_tokens, "pre-compression token count")?,
            after_tokens: wire_usize(after_tokens, "post-compression token count")?,
        },
        AgentEventWire::Chart { spec } => AgentEvent::Chart {
            spec: from_wire(spec)?,
        },
        AgentEventWire::Error {
            source,
            message,
            failure,
        } => AgentEvent::Error {
            source,
            message,
            failure: from_wire(failure)?,
        },
        AgentEventWire::SafetyNotice {
            action,
            reason,
            risk,
            permission,
        } => AgentEvent::SafetyNotice {
            action,
            reason,
            risk,
            permission,
        },
        AgentEventWire::ParameterError {
            tool,
            parameter,
            expected,
            got,
        } => AgentEvent::ParameterError {
            tool,
            parameter,
            expected,
            got,
        },
        AgentEventWire::FinalAnswer { text } => AgentEvent::FinalAnswer(text),
        AgentEventWire::Cancelled => AgentEvent::Cancelled,
    })
}

fn agent_stream_chunk_from_wire(
    wire: AgentStreamChunkWire,
) -> std::result::Result<AgentEvent, String> {
    let value = serde_json::to_value(wire).map_err(|error| error.to_string())?;
    let event = serde_json::from_value(value).map_err(|error| error.to_string())?;
    agent_event_from_wire(event)
}

fn agent_stream_terminal_from_wire(
    wire: AgentStreamTerminalWire,
) -> std::result::Result<AgentEvent, String> {
    let value = serde_json::to_value(wire).map_err(|error| error.to_string())?;
    let event = serde_json::from_value(value).map_err(|error| error.to_string())?;
    agent_event_from_wire(event)
}

fn wire_usize(value: WireU64, field: &str) -> std::result::Result<usize, String> {
    let value = value.to_u64().ok_or_else(|| format!("invalid {field}"))?;
    usize::try_from(value).map_err(|_| format!("{field} exceeds this platform's usize range"))
}

/// Thin `Tool` proxy: descriptor facts answered locally, execution and
/// streaming delegated to the registered implementation. The ToolManager
/// keeps owning permissions, retry and sandbox policy.
pub(crate) struct ExtensionToolProxy {
    bridge: Arc<ExtensionBridge>,
    extension: WireHandle,
    descriptor: ExtensionDescriptor,
    session_id: String,
}

impl ExtensionToolProxy {
    pub(crate) fn new(
        bridge: Arc<ExtensionBridge>,
        extension: WireHandle,
        session_id: String,
    ) -> Option<Self> {
        let descriptor = bridge
            .state()
            .ok()?
            .handles
            .extension(&extension)
            .ok()?
            .descriptor
            .clone();
        matches!(descriptor, ExtensionDescriptor::Tool { .. }).then(|| Self {
            bridge,
            extension,
            descriptor,
            session_id,
        })
    }

    fn as_tool(&self) -> (String, String, serde_json::Value, u64, bool) {
        let ExtensionDescriptor::Tool {
            name,
            description,
            parameters,
            schema_revision,
            supports_streaming,
            ..
        } = &self.descriptor
        else {
            return (
                String::new(),
                String::new(),
                serde_json::Value::Null,
                0,
                false,
            );
        };
        let parameters = parameters
            .clone()
            .into_json()
            .unwrap_or(serde_json::Value::Null);
        (
            name.clone(),
            description.clone(),
            parameters,
            schema_revision.to_u64().unwrap_or_default(),
            *supports_streaming,
        )
    }

    async fn execute_over(
        &self,
        operation: ExtensionOperation,
        parameters: ToolParameters,
        context: Option<&ToolContext>,
    ) -> echo_agent::error::Result<ToolResult> {
        let input = ToolExecuteInput {
            parameters: to_wire(parameters).map_err(ReactError::Other)?,
            context: context
                .map(tool_context_wire)
                .transpose()
                .map_err(ReactError::Other)?,
        };
        let invocation = match operation {
            ExtensionOperation::ToolExecute => ExtensionInvocation::ToolExecute(input),
            _ => {
                return Err(ReactError::Other(
                    "invalid non-streaming tool operation".to_string(),
                ));
            }
        };
        let value = self
            .bridge
            .invoke_once(
                &self.extension,
                context.and_then(invocation_context),
                invocation,
                context
                    .and_then(|context| context.cancel.clone())
                    .unwrap_or_else(cancelled_token_arc)
                    .as_ref()
                    .clone(),
            )
            .await
            .map_err(tool_react_error)?;
        let ExtensionResult::ToolExecute(result) = value else {
            return Err(ReactError::Other(
                "tool extension returned the wrong result variant".to_string(),
            ));
        };
        tool_result_from_wire(result).map_err(ReactError::Other)
    }
}

impl echo_agent::tools::Tool for ExtensionToolProxy {
    fn name(&self) -> &str {
        match &self.descriptor {
            ExtensionDescriptor::Tool { name, .. } => name,
            _ => "",
        }
    }

    fn description(&self) -> &str {
        match &self.descriptor {
            ExtensionDescriptor::Tool { description, .. } => description,
            _ => "",
        }
    }

    fn parameters(&self) -> serde_json::Value {
        self.as_tool().2
    }

    fn schema_revision(&self) -> u64 {
        self.as_tool().3
    }

    fn supports_streaming(&self) -> bool {
        self.as_tool().4
    }

    fn permissions(&self) -> Vec<echo_agent::tools::permission::ToolPermission> {
        match &self.descriptor {
            ExtensionDescriptor::Tool {
                required_permissions,
                ..
            } => required_permissions
                .iter()
                .map(|permission| match permission {
                    echo_sdk_protocol::methods::ToolPermissionWire::Read => {
                        echo_agent::tools::permission::ToolPermission::Read
                    }
                    echo_sdk_protocol::methods::ToolPermissionWire::Write => {
                        echo_agent::tools::permission::ToolPermission::Write
                    }
                    echo_sdk_protocol::methods::ToolPermissionWire::Network => {
                        echo_agent::tools::permission::ToolPermission::Network
                    }
                    echo_sdk_protocol::methods::ToolPermissionWire::Execute => {
                        echo_agent::tools::permission::ToolPermission::Execute
                    }
                    echo_sdk_protocol::methods::ToolPermissionWire::Sensitive => {
                        echo_agent::tools::permission::ToolPermission::Sensitive
                    }
                })
                .collect(),
            _ => Vec::new(),
        }
    }

    fn risk_level(&self) -> echo_agent::tools::ToolRiskLevel {
        match &self.descriptor {
            ExtensionDescriptor::Tool { risk_level, .. } => match risk_level {
                echo_sdk_protocol::methods::ToolRiskLevelWire::ReadOnly => {
                    echo_agent::tools::ToolRiskLevel::ReadOnly
                }
                echo_sdk_protocol::methods::ToolRiskLevelWire::Standard => {
                    echo_agent::tools::ToolRiskLevel::Standard
                }
                echo_sdk_protocol::methods::ToolRiskLevelWire::Dangerous => {
                    echo_agent::tools::ToolRiskLevel::Dangerous
                }
            },
            _ => echo_agent::tools::ToolRiskLevel::Standard,
        }
    }

    #[doc(hidden)]
    fn required_input_modalities_owned(&self) -> Vec<echo_agent::llm::ModelInputModality> {
        match &self.descriptor {
            ExtensionDescriptor::Tool {
                required_input_modalities,
                ..
            } => required_input_modalities
                .iter()
                .map(|modality| match modality {
                    echo_sdk_protocol::methods::ModelModalityWire::Text => {
                        echo_agent::llm::ModelInputModality::Text
                    }
                    echo_sdk_protocol::methods::ModelModalityWire::Image => {
                        echo_agent::llm::ModelInputModality::Image
                    }
                    echo_sdk_protocol::methods::ModelModalityWire::Audio => {
                        echo_agent::llm::ModelInputModality::Audio
                    }
                    echo_sdk_protocol::methods::ModelModalityWire::Video => {
                        echo_agent::llm::ModelInputModality::Video
                    }
                })
                .collect(),
            _ => Vec::new(),
        }
    }

    fn exempt_from_batch_timeout(&self) -> bool {
        match &self.descriptor {
            ExtensionDescriptor::Tool {
                exempt_from_batch_timeout,
                ..
            } => *exempt_from_batch_timeout,
            _ => false,
        }
    }

    fn allows_parallel_batch_execution(&self) -> bool {
        match &self.descriptor {
            ExtensionDescriptor::Tool {
                allows_parallel_batch_execution,
                ..
            } => *allows_parallel_batch_execution,
            _ => true,
        }
    }

    fn manages_own_timeout(&self) -> bool {
        match &self.descriptor {
            ExtensionDescriptor::Tool {
                manages_own_timeout,
                ..
            } => *manages_own_timeout,
            _ => false,
        }
    }

    fn execute_with_context<'a>(
        &'a self,
        parameters: ToolParameters,
        context: &'a ToolContext,
    ) -> futures::future::BoxFuture<'a, echo_agent::error::Result<ToolResult>> {
        Box::pin(async move {
            self.execute_over(ExtensionOperation::ToolExecute, parameters, Some(context))
                .await
        })
    }

    fn execute_stream_with_context<'a>(
        &'a self,
        parameters: ToolParameters,
        context: &ToolContext,
    ) -> futures::future::BoxFuture<
        'a,
        echo_agent::error::Result<
            std::pin::Pin<Box<dyn futures::Stream<Item = ToolStreamEvent> + Send + 'a>>,
        >,
    > {
        let context = context.clone();
        Box::pin(async move {
            let input = ToolExecuteInput {
                parameters: to_wire(parameters).map_err(ReactError::Other)?,
                context: Some(tool_context_wire(&context).map_err(ReactError::Other)?),
            };
            let cancellation = context
                .cancel
                .clone()
                .unwrap_or_else(cancelled_token_arc)
                .as_ref()
                .clone();
            let (receiver, stream, invocation_id, cancellation, lease, sink) = self
                .bridge
                .invoke_stream(
                    &self.extension,
                    Some(ExtensionInvocationContext {
                        session_id: context.conversation_id.clone(),
                        run_id: context.run_id.clone(),
                        stream_id: None,
                        turn_id: context.turn_id.clone(),
                        message_id: context.message_id.clone(),
                        execution_id: context.execution_id.clone(),
                        call_id: context.call_id.clone(),
                    })
                    .filter(|value| {
                        value.session_id.is_some()
                            || value.run_id.is_some()
                            || value.turn_id.is_some()
                            || value.message_id.is_some()
                            || value.execution_id.is_some()
                            || value.call_id.is_some()
                    }),
                    ExtensionInvocation::ToolExecuteStream(input),
                    cancellation,
                )
                .await
                .map_err(tool_react_error)?;
            Ok(Box::pin(futures::StreamExt::map(
                extension_event_stream(
                    self.bridge.clone(),
                    stream,
                    invocation_id,
                    cancellation,
                    lease,
                    sink,
                    receiver,
                ),
                |event: ExtensionStreamEvent| match event {
                    ExtensionStreamEvent::Chunk { value, .. } => {
                        let ExtensionStreamChunkValue::Tool(event) = value else {
                            return ToolStreamEvent::Complete(ToolResult::error(
                                "tool stream received a non-tool payload",
                            ));
                        };
                        tool_stream_chunk_from_wire(event).unwrap_or_else(|error| {
                            ToolStreamEvent::Complete(ToolResult::error(error))
                        })
                    }
                    ExtensionStreamEvent::Complete { value, .. } => {
                        let ExtensionStreamCompleteValue::Tool(result) = value else {
                            return ToolStreamEvent::Complete(ToolResult::error(
                                "tool stream received a non-tool terminal",
                            ));
                        };
                        ToolStreamEvent::Complete(
                            tool_result_from_wire(result).unwrap_or_else(ToolResult::error),
                        )
                    }
                    ExtensionStreamEvent::Failed { error, .. } => {
                        ToolStreamEvent::Complete(ToolResult::error(&error.message))
                    }
                    ExtensionStreamEvent::Cancelled { .. } => {
                        ToolStreamEvent::Complete(ToolResult::error("stream was cancelled"))
                    }
                },
            ))
                as std::pin::Pin<
                    Box<dyn futures::Stream<Item = ToolStreamEvent> + Send>,
                >)
        })
    }

    fn validate_parameters<'a>(
        &'a self,
        parameters: &'a ToolParameters,
    ) -> futures::future::BoxFuture<'a, echo_agent::error::Result<()>> {
        Box::pin(async move {
            let input = ToolValidateInput {
                parameters: to_wire(parameters).map_err(ReactError::Other)?,
            };
            let value = self
                .bridge
                .invoke_once(
                    &self.extension,
                    Some(session_invocation_context(&self.session_id)),
                    ExtensionInvocation::ToolValidateParameters(input),
                    self.bridge.connection_cancellation(),
                )
                .await
                .map_err(tool_react_error)?;
            // The validation contract is `null` when the parameters are
            // valid and a bounded rejection reason otherwise.
            let ExtensionResult::ToolValidateParameters(result) = value else {
                return Err(ReactError::Other(
                    "tool validation returned the wrong result variant".to_string(),
                ));
            };
            match result {
                Some(reason) => Err(ReactError::Other(reason)),
                None => Ok(()),
            }
        })
    }
}

// ── LlmClient proxy ─────────────────────────────────────────────────────────

fn cache_hints_json(hints: &echo_agent::llm::cache::CacheHints) -> serde_json::Value {
    let breakpoints = hints
        .breakpoints
        .iter()
        .map(|target| match target {
            echo_agent::llm::cache::BreakpointTarget::SystemLastBlock => {
                serde_json::json!({"kind": "system_last_block"})
            }
            echo_agent::llm::cache::BreakpointTarget::ToolsLastTool => {
                serde_json::json!({"kind": "tools_last_tool"})
            }
            echo_agent::llm::cache::BreakpointTarget::HistoryIndex(index) => {
                serde_json::json!({"kind": "history_index", "index": index})
            }
            echo_agent::llm::cache::BreakpointTarget::HistoryLastStable => {
                serde_json::json!({"kind": "history_last_stable"})
            }
        })
        .collect::<Vec<_>>();
    let range = |range: echo_agent::llm::cache::SegmentRange| serde_json::json!({"start": range.start, "end": range.end});
    serde_json::json!({
        "breakpoints": breakpoints,
        "stable_prefix_hash": hints.stable_prefix_hash,
        "segments": {
            "system": range(hints.segments.system),
            "canonical": range(hints.segments.canonical),
            "history": range(hints.segments.history),
            "runtime_context": range(hints.segments.runtime_context),
        }
    })
}

fn provider_capabilities(
    capabilities: &echo_sdk_protocol::methods::LlmCapabilitiesWire,
) -> echo_agent::llm::ProviderCapabilities {
    let tokenizer_name = match capabilities.tokenizer_name.as_deref() {
        Some("cl100k_base") => Some("cl100k_base"),
        Some("o200k_base") => Some("o200k_base"),
        Some("claude") => Some("claude"),
        _ => None,
    };
    echo_agent::llm::ProviderCapabilities {
        streaming_tool_calls: capabilities.streaming_tool_calls,
        named_sse_events: capabilities.named_sse_events,
        reasoning_content: capabilities.reasoning_content,
        image_input: capabilities.image_input,
        system_as_top_level: capabilities.system_as_top_level,
        ndjson_streaming: capabilities.ndjson_streaming,
        tool_support: capabilities.tool_support,
        structured_output: capabilities.structured_output,
        requires_version_header: capabilities.requires_version_header,
        supports_parallel_tool_calls: capabilities.supports_parallel_tool_calls,
        supports_tool_choice_none: capabilities.supports_tool_choice_none,
        tokenizer_name,
    }
}

fn reasoning_block_wire(block: &echo_agent::llm::types::ReasoningBlock) -> LlmReasoningBlockWire {
    match block {
        echo_agent::llm::types::ReasoningBlock::Signed {
            thinking,
            signature,
        } => LlmReasoningBlockWire::Signed {
            thinking: thinking.clone(),
            signature: signature.clone(),
        },
        echo_agent::llm::types::ReasoningBlock::Redacted { data } => {
            LlmReasoningBlockWire::Redacted { data: data.clone() }
        }
        echo_agent::llm::types::ReasoningBlock::Opaque {
            provider,
            id,
            data,
            summary,
        } => LlmReasoningBlockWire::Opaque {
            provider: provider.clone(),
            id: id.clone(),
            data: data.clone(),
            summary: summary.clone(),
        },
    }
}

fn reasoning_block_from_wire(
    block: LlmReasoningBlockWire,
) -> echo_agent::llm::types::ReasoningBlock {
    match block {
        LlmReasoningBlockWire::Signed {
            thinking,
            signature,
        } => echo_agent::llm::types::ReasoningBlock::Signed {
            thinking,
            signature,
        },
        LlmReasoningBlockWire::Redacted { data } => {
            echo_agent::llm::types::ReasoningBlock::Redacted { data }
        }
        LlmReasoningBlockWire::Opaque {
            provider,
            id,
            data,
            summary,
        } => echo_agent::llm::types::ReasoningBlock::Opaque {
            provider,
            id,
            data,
            summary,
        },
    }
}

fn message_wire(
    message: &echo_agent::llm::types::Message,
) -> std::result::Result<LlmMessageWire, String> {
    Ok(LlmMessageWire {
        role: message.role.as_str().to_string(),
        content: to_wire(&message.content)?,
        tool_calls: message.tool_calls.as_ref().map(|calls| {
            calls
                .iter()
                .map(|call| LlmToolCallWire {
                    id: call.id.clone(),
                    call_type: call.call_type.clone(),
                    function_name: call.function.name.clone(),
                    arguments: call.function.arguments.clone(),
                })
                .collect()
        }),
        name: message.name.clone(),
        tool_call_id: message.tool_call_id.clone(),
        reasoning_content: message.reasoning_content.clone(),
        reasoning_blocks: message
            .reasoning_blocks
            .as_ref()
            .map(|blocks| blocks.iter().map(reasoning_block_wire).collect()),
    })
}

fn message_from_wire(
    message: LlmMessageWire,
) -> std::result::Result<echo_agent::llm::types::Message, String> {
    Ok(echo_agent::llm::types::Message {
        role: message.role.as_str().into(),
        content: from_wire(message.content)?,
        tool_calls: message.tool_calls.map(|calls| {
            calls
                .into_iter()
                .map(|call| echo_agent::llm::types::ToolCall {
                    id: call.id,
                    call_type: call.call_type,
                    function: echo_agent::llm::types::FunctionCall {
                        name: call.function_name,
                        arguments: call.arguments,
                    },
                })
                .collect()
        }),
        name: message.name,
        tool_call_id: message.tool_call_id,
        reasoning_content: message.reasoning_content,
        reasoning_blocks: message
            .reasoning_blocks
            .map(|blocks| blocks.into_iter().map(reasoning_block_from_wire).collect()),
    })
}

#[cfg(test)]
fn delta_tool_call_wire(delta: &echo_agent::llm::types::DeltaToolCall) -> LlmDeltaToolCallWire {
    LlmDeltaToolCallWire {
        index: delta.index,
        id: delta.id.clone(),
        call_type: delta.call_type.clone(),
        function: delta
            .function
            .as_ref()
            .map(|function| LlmDeltaFunctionWire {
                name: function.name.clone(),
                arguments: function.arguments.clone(),
            }),
    }
}

fn delta_tool_call_from_wire(wire: LlmDeltaToolCallWire) -> echo_agent::llm::types::DeltaToolCall {
    echo_agent::llm::types::DeltaToolCall {
        index: wire.index,
        id: wire.id,
        call_type: wire.call_type,
        function: wire
            .function
            .map(|function| echo_agent::llm::types::DeltaFunctionCall {
                name: function.name,
                arguments: function.arguments,
            }),
    }
}

#[cfg(test)]
fn chat_chunk_wire(
    chunk: &echo_agent::llm::ChatChunk,
) -> std::result::Result<LlmChatChunkWire, String> {
    Ok(LlmChatChunkWire {
        role: chunk.delta.role.clone(),
        content: chunk.delta.content.clone(),
        reasoning_content: chunk.delta.reasoning_content.clone(),
        reasoning_blocks: chunk
            .delta
            .reasoning_blocks
            .as_ref()
            .map(|blocks| blocks.iter().map(reasoning_block_wire).collect()),
        tool_calls: chunk
            .delta
            .tool_calls
            .as_ref()
            .map(|calls| calls.iter().map(delta_tool_call_wire).collect()),
        finish_reason: chunk.finish_reason.clone(),
        usage: chunk
            .usage
            .as_ref()
            .map(|usage| to_wire(usage).map(|value| LlmUsageWire { value }))
            .transpose()?,
    })
}

fn chat_chunk_from_wire(
    wire: LlmChatChunkWire,
) -> std::result::Result<echo_agent::llm::ChatChunk, String> {
    let delta = echo_agent::llm::types::DeltaMessage {
        role: wire.role,
        content: wire.content,
        reasoning_content: wire.reasoning_content,
        reasoning_blocks: wire
            .reasoning_blocks
            .map(|blocks| blocks.into_iter().map(reasoning_block_from_wire).collect()),
        tool_calls: wire
            .tool_calls
            .map(|calls| calls.into_iter().map(delta_tool_call_from_wire).collect()),
    };
    Ok(echo_agent::llm::ChatChunk {
        delta,
        finish_reason: wire.finish_reason,
        usage: wire.usage.map(|usage| from_wire(usage.value)).transpose()?,
    })
}

fn chat_chunk_from_stream_chunk(
    wire: LlmStreamChunkWire,
) -> std::result::Result<echo_agent::llm::ChatChunk, String> {
    chat_chunk_from_wire(LlmChatChunkWire {
        role: wire.role,
        content: wire.content,
        reasoning_content: wire.reasoning_content,
        reasoning_blocks: wire.reasoning_blocks,
        tool_calls: wire.tool_calls,
        finish_reason: None,
        usage: wire.usage,
    })
}

fn chat_chunk_from_stream_complete(
    wire: LlmStreamCompleteWire,
) -> std::result::Result<echo_agent::llm::ChatChunk, String> {
    chat_chunk_from_wire(LlmChatChunkWire {
        role: wire.role,
        content: wire.content,
        reasoning_content: wire.reasoning_content,
        reasoning_blocks: wire.reasoning_blocks,
        tool_calls: wire.tool_calls,
        finish_reason: Some(wire.finish_reason),
        usage: wire.usage,
    })
}

fn chat_response_from_wire(
    wire: LlmChatResponseWire,
) -> std::result::Result<echo_agent::llm::ChatResponse, String> {
    Ok(echo_agent::llm::ChatResponse {
        message: message_from_wire(wire.message)?,
        finish_reason: wire.finish_reason,
        usage: wire.usage.map(|usage| from_wire(usage.value)).transpose()?,
        raw: from_wire(wire.raw)?,
    })
}

fn chat_response_into_chunk(
    response: echo_agent::llm::ChatResponse,
) -> std::result::Result<echo_agent::llm::ChatChunk, String> {
    let finish_reason = response
        .finish_reason
        .filter(|reason| !reason.trim().is_empty() && reason.chars().count() <= 256)
        .ok_or_else(|| {
            "non-streaming LLM response cannot be adapted without a bounded finish reason"
                .to_string()
        })?;
    let tool_calls = response
        .message
        .tool_calls
        .map(|calls| {
            calls
                .into_iter()
                .enumerate()
                .map(|(index, call)| {
                    Ok(echo_agent::llm::types::DeltaToolCall {
                        index: u32::try_from(index)
                            .map_err(|_| "LLM response has too many tool calls".to_string())?,
                        id: Some(call.id),
                        call_type: Some(call.call_type),
                        function: Some(echo_agent::llm::types::DeltaFunctionCall {
                            name: Some(call.function.name),
                            arguments: Some(call.function.arguments),
                        }),
                    })
                })
                .collect::<std::result::Result<Vec<_>, String>>()
        })
        .transpose()?;
    Ok(echo_agent::llm::ChatChunk {
        delta: echo_agent::llm::types::DeltaMessage {
            role: Some(response.message.role.as_str().to_string()),
            content: response.message.content.as_text(),
            reasoning_content: response.message.reasoning_content,
            reasoning_blocks: response.message.reasoning_blocks,
            tool_calls,
        },
        finish_reason: Some(finish_reason),
        usage: response.usage,
    })
}

/// Thin `LlmClient` proxy: chat and chat_stream delegate to the registered
/// implementation; the descriptor answers model identity locally. The
/// AgentTurnDriver keeps deciding run terminals from the returned chunks.
pub(crate) struct ExtensionLlmClientProxy {
    bridge: Arc<ExtensionBridge>,
    extension: WireHandle,
    model_name: String,
    capabilities: echo_agent::llm::ProviderCapabilities,
    supports_streaming: bool,
    session_id: String,
}

impl ExtensionLlmClientProxy {
    pub(crate) fn new(
        bridge: Arc<ExtensionBridge>,
        extension: WireHandle,
        session_id: String,
    ) -> Option<Self> {
        let record = bridge.state().ok()?.handles.extension(&extension).ok()?;
        let ExtensionDescriptor::LlmClient {
            model_name,
            supports_streaming,
            capabilities,
            ..
        } = &record.descriptor
        else {
            return None;
        };
        Some(Self {
            bridge,
            extension,
            model_name: model_name.clone(),
            capabilities: provider_capabilities(capabilities),
            supports_streaming: *supports_streaming,
            session_id,
        })
    }

    fn request_wire(
        request: &echo_agent::llm::ChatRequest,
    ) -> std::result::Result<LlmChatRequestWire, String> {
        Ok(LlmChatRequestWire {
            messages: request
                .messages
                .iter()
                .map(message_wire)
                .collect::<std::result::Result<Vec<_>, _>>()?,
            temperature: request.temperature.map(f64::from),
            max_tokens: request.max_tokens,
            tools: request
                .tools
                .as_ref()
                .map(|tools| {
                    tools
                        .iter()
                        .map(|tool| {
                            Ok(LlmToolDefinitionWire {
                                tool_type: tool.tool_type.clone(),
                                name: tool.function.name.clone(),
                                description: tool.function.description.clone(),
                                parameters: to_wire(&tool.function.parameters)?,
                            })
                        })
                        .collect::<std::result::Result<Vec<_>, String>>()
                })
                .transpose()?,
            tool_choice: request.tool_choice.clone(),
            user_id: request.user_id.clone(),
            response_format: request.response_format.as_ref().map(to_wire).transpose()?,
            thinking: request.thinking.as_ref().map(to_wire).transpose()?,
            timeouts: request.timeouts.as_ref().map(to_wire).transpose()?,
            cache_hints: request
                .cache_hints
                .as_ref()
                .map(cache_hints_json)
                .map(to_wire)
                .transpose()?,
        })
    }
}

impl echo_agent::llm::LlmClient for ExtensionLlmClientProxy {
    fn chat(
        &self,
        request: echo_agent::llm::ChatRequest,
    ) -> futures::future::BoxFuture<'_, echo_agent::error::Result<echo_agent::llm::ChatResponse>>
    {
        Box::pin(async move {
            let cancellation = request.cancel_token.clone().unwrap_or_else(cancelled_token);
            let value = self
                .bridge
                .invoke_once(
                    &self.extension,
                    Some(session_invocation_context(&self.session_id)),
                    ExtensionInvocation::LlmChat(
                        Self::request_wire(&request).map_err(ReactError::Other)?,
                    ),
                    cancellation,
                )
                .await
                .map_err(react_error)?;
            let ExtensionResult::LlmChat(response) = value else {
                return Err(ReactError::Other(
                    "llm extension returned the wrong result variant".to_string(),
                ));
            };
            chat_response_from_wire(response).map_err(ReactError::Other)
        })
    }

    fn chat_stream(
        &self,
        request: echo_agent::llm::ChatRequest,
    ) -> futures::future::BoxFuture<
        '_,
        echo_agent::error::Result<
            futures::stream::BoxStream<
                'static,
                echo_agent::error::Result<echo_agent::llm::ChatChunk>,
            >,
        >,
    > {
        Box::pin(async move {
            if !self.supports_streaming {
                let response = self.chat(request).await?;
                let chunk = chat_response_into_chunk(response).map_err(ReactError::Other)?;
                return Ok(futures::stream::once(async move { Ok(chunk) }).boxed());
            }
            let cancellation = request.cancel_token.clone().unwrap_or_else(cancelled_token);
            let (receiver, stream, invocation_id, cancellation, lease, sink) = self
                .bridge
                .invoke_stream(
                    &self.extension,
                    Some(session_invocation_context(&self.session_id)),
                    ExtensionInvocation::LlmChatStream(
                        Self::request_wire(&request).map_err(ReactError::Other)?,
                    ),
                    cancellation,
                )
                .await
                .map_err(react_error)?;
            Ok(futures::StreamExt::map(
                extension_event_stream(
                    self.bridge.clone(),
                    stream,
                    invocation_id,
                    cancellation,
                    lease,
                    sink,
                    receiver,
                ),
                |event: ExtensionStreamEvent| {
                    let result: echo_agent::error::Result<echo_agent::llm::ChatChunk> = match event
                    {
                        ExtensionStreamEvent::Chunk { value, .. } => {
                            let ExtensionStreamChunkValue::Llm(chunk) = value else {
                                return Err(ReactError::Other(
                                    "llm stream received a non-llm payload".to_string(),
                                ));
                            };
                            chat_chunk_from_stream_chunk(chunk).map_err(ReactError::Other)
                        }
                        ExtensionStreamEvent::Complete { value, .. } => {
                            let ExtensionStreamCompleteValue::Llm(chunk) = value else {
                                return Err(ReactError::Other(
                                    "llm stream received a non-llm terminal".to_string(),
                                ));
                            };
                            chat_chunk_from_stream_complete(chunk).map_err(ReactError::Other)
                        }
                        ExtensionStreamEvent::Failed { error, .. } => Err(react_error(error)),
                        ExtensionStreamEvent::Cancelled { .. } => {
                            Err(ReactError::Other("llm stream was cancelled".to_string()))
                        }
                    };
                    result
                },
            )
            .boxed())
        })
    }

    fn model_name(&self) -> &str {
        &self.model_name
    }

    fn capabilities(&self) -> echo_agent::llm::ProviderCapabilities {
        self.capabilities
    }
}

// ── Context-compressor proxy ────────────────────────────────────────────────

/// Thin custom-compressor proxy. The Host retains cancellation and tokenizer
/// authority; the callback receives the message/input fields and returns the
/// framework's message, eviction and checkpoint result.
pub(crate) struct ExtensionContextCompressorProxy {
    bridge: Arc<ExtensionBridge>,
    extension: WireHandle,
    session_id: String,
    name: String,
}

struct TokenizerResourceGuard {
    state: Arc<CoreProfileState>,
    owner: String,
    handle: WireHandle,
}

impl TokenizerResourceGuard {
    fn new(state: Arc<CoreProfileState>, owner: String, handle: WireHandle) -> Self {
        Self {
            state,
            owner,
            handle,
        }
    }
}

impl Drop for TokenizerResourceGuard {
    fn drop(&mut self) {
        crate::core_profile::facade::source_operations::release_tokenizer_authority(
            &self.state,
            &self.owner,
            &self.handle,
        );
    }
}

impl ExtensionContextCompressorProxy {
    pub(crate) fn new(
        bridge: Arc<ExtensionBridge>,
        extension: WireHandle,
        session_id: String,
    ) -> Option<Self> {
        let record = bridge.state().ok()?.handles.extension(&extension).ok()?;
        let ExtensionDescriptor::ContextCompressor { name, .. } = &record.descriptor else {
            return None;
        };
        Some(Self {
            bridge,
            extension,
            session_id,
            name: name.clone(),
        })
    }
}

impl echo_agent::compression::ContextCompressor for ExtensionContextCompressorProxy {
    fn compress(
        &self,
        input: echo_agent::compression::CompressionInput,
    ) -> futures::future::BoxFuture<
        '_,
        echo_agent::error::Result<echo_agent::compression::CompressionOutput>,
    > {
        Box::pin(async move {
            let cancellation = input.cancel_token.clone().unwrap_or_else(cancelled_token);
            let tokenizer = input.tokenizer();
            let token_limit = u64::try_from(input.token_limit).map_err(|_| {
                ReactError::Other("compression token_limit exceeds WireU64".to_string())
            })?;
            let messages = input
                .messages
                .iter()
                .map(message_wire)
                .collect::<std::result::Result<Vec<_>, _>>()
                .map_err(ReactError::Other)?;
            let state = self.bridge.state().map_err(react_error)?;
            let tokenizer_resource =
                crate::core_profile::facade::source_operations::register_tokenizer_authority(
                    &state,
                    &self.session_id,
                    tokenizer,
                )
                .map_err(react_error)?;
            let tokenizer_resource = TokenizerResourceGuard::new(
                state.clone(),
                self.session_id.clone(),
                tokenizer_resource,
            );
            let result = self
                .bridge
                .invoke_once(
                    &self.extension,
                    Some(session_invocation_context(&self.session_id)),
                    ExtensionInvocation::CompressorCompress(CompressionInputWire {
                        messages,
                        token_limit: WireU64::from_u64(token_limit),
                        current_query: input.current_query,
                        focus_instructions: input.focus_instructions,
                        tokenizer: TokenizerReferenceWire {
                            resource: tokenizer_resource.handle.clone(),
                            owner_session_id: self.session_id.clone(),
                        },
                    }),
                    cancellation,
                )
                .await;
            let result = result.map_err(react_error)?;
            let ExtensionResult::CompressorCompress(CompressionOutputWire {
                messages,
                evicted,
                checkpoint,
            }) = result
            else {
                return Err(ReactError::Other(
                    "context compressor extension returned the wrong result variant".to_string(),
                ));
            };
            let messages = messages
                .into_iter()
                .map(message_from_wire)
                .collect::<std::result::Result<Vec<_>, _>>()
                .map_err(ReactError::Other)?;
            let evicted = evicted
                .into_iter()
                .map(message_from_wire)
                .collect::<std::result::Result<Vec<_>, _>>()
                .map_err(ReactError::Other)?;
            let checkpoint = checkpoint
                .map(|checkpoint| {
                    checkpoint
                        .into_json()
                        .map_err(|error| error.to_string())
                        .and_then(|value| {
                            serde_json::from_value(value).map_err(|error| error.to_string())
                        })
                })
                .transpose()
                .map_err(ReactError::Other)?;
            Ok(echo_agent::compression::CompressionOutput {
                messages,
                evicted,
                checkpoint,
            })
        })
    }

    fn name(&self) -> &str {
        &self.name
    }
}

// ── Live Agent component proxies ────────────────────────────────────────────

/// Shared reverse bridge for application-supplied Agent infrastructure. The
/// descriptor component and each operation form a closed pair; concrete trait
/// impls below only translate values and never own persistence or policy.
#[derive(Clone)]
pub(crate) struct ExtensionAgentComponentProxy {
    bridge: Arc<ExtensionBridge>,
    extension: WireHandle,
    session_id: String,
    component: AgentComponentKindWire,
    name: String,
    capabilities: AgentComponentCapabilitiesWire,
    notification_queue:
        Arc<Mutex<VecDeque<echo_agent::mcp::integration::types::JsonRpcNotification>>>,
    notification_cancel: CancellationToken,
    notification_started: Arc<std::sync::atomic::AtomicBool>,
}

impl ExtensionAgentComponentProxy {
    pub(crate) fn new(
        bridge: Arc<ExtensionBridge>,
        extension: WireHandle,
        session_id: String,
    ) -> Option<Self> {
        let record = bridge.state().ok()?.handles.extension(&extension).ok()?;
        let ExtensionDescriptor::AgentComponent {
            component,
            ref name,
            ref capabilities,
            ..
        } = record.descriptor
        else {
            return None;
        };
        let proxy = Self {
            bridge,
            extension,
            session_id,
            component,
            name: name.clone(),
            capabilities: capabilities.clone(),
            notification_queue: Arc::new(Mutex::new(VecDeque::new())),
            notification_cancel: CancellationToken::new(),
            notification_started: Arc::new(std::sync::atomic::AtomicBool::new(false)),
        };
        Some(proxy)
    }

    pub(crate) fn component(&self) -> AgentComponentKindWire {
        self.component
    }

    fn start_notification_poll(&self) {
        if !self.capabilities.supports_notifications
            || self
                .notification_started
                .swap(true, std::sync::atomic::Ordering::AcqRel)
        {
            return;
        }
        let proxy = self.clone();
        let _ = tokio::runtime::Handle::try_current().map(|runtime| {
            runtime.spawn(async move {
                loop {
                    let result = tokio::select! {
                        () = proxy.notification_cancel.cancelled() => return,
                        result = proxy.invoke(AgentComponentCallInputWire::McpTransportTryNotification) => result,
                    };
                    match result {
                        Ok(AgentComponentCallResultWire::McpTransportTryNotification {
                            notification: Some(notification),
                        }) => {
                            let Ok(notification) = component_decode(notification) else {
                                return;
                            };
                            let mut queue = proxy
                                .notification_queue
                                .lock()
                                .unwrap_or_else(|error| error.into_inner());
                            if queue.len() >= MCP_NOTIFICATION_QUEUE_CAPACITY {
                                queue.pop_front();
                                tracing::warn!(
                                    extension = %proxy.extension.id,
                                    capacity = MCP_NOTIFICATION_QUEUE_CAPACITY,
                                    "MCP extension notification queue dropped its oldest item"
                                );
                            }
                            queue.push_back(notification);
                        }
                        Ok(AgentComponentCallResultWire::McpTransportTryNotification {
                            notification: None,
                        }) => tokio::time::sleep(Duration::from_millis(25)).await,
                        _ => return,
                    }
                }
            });
        });
    }

    fn require_managed_persistence(&self) -> echo_agent::error::Result<()> {
        let supported = self.capabilities.persistence_call
            == PersistenceCallCapabilityWire::AbsoluteDeadlineV1
            && match self.component {
                AgentComponentKindWire::ConversationStore => {
                    self.capabilities.conversation_projection
                        == ConversationProjectionCapabilityWire::AtomicV1
                }
                AgentComponentKindWire::RuntimeStateStore => {
                    self.capabilities.runtime_state == RuntimeStateCapabilityWire::RevisionedV1
                }
                _ => false,
            };
        if !supported {
            return Err(match self.component {
                AgentComponentKindWire::RuntimeStateStore => {
                    echo_agent::error::RuntimeStateError::Unsupported(
                        "extension lacks revisioned/deadline persistence capability".to_string(),
                    )
                    .into()
                }
                _ => echo_agent::error::MemoryError::Unsupported(
                    "extension lacks atomic/deadline persistence capability".to_string(),
                )
                .into(),
            });
        }
        Ok(())
    }

    fn managed_context(
        &self,
        context: echo_agent::memory::PersistenceCallContext,
    ) -> echo_agent::error::Result<PersistenceCallContextWire> {
        if let Err(error) = context.ensure_not_expired() {
            return Err(match self.component {
                AgentComponentKindWire::RuntimeStateStore => {
                    echo_agent::error::RuntimeStateError::DeadlineExceeded(error.to_string()).into()
                }
                _ => error.into(),
            });
        }
        Ok(PersistenceCallContextWire {
            absolute_deadline_unix_ms: WireI64::from_i64(context.absolute_deadline_unix_ms),
        })
    }

    async fn invoke_task_store(
        &self,
        call: AgentComponentCallInputWire,
    ) -> Result<AgentComponentCallResultWire, echo_agent::tasks::RevisionedTaskStoreError> {
        let operation = call.operation();
        let result = self
            .bridge
            .invoke_once_connection_scoped(
                &self.extension,
                Some(session_invocation_context(&self.session_id)),
                ExtensionInvocation::AgentComponentCall(AgentComponentCallWire {
                    component: self.component,
                    call,
                }),
            )
            .await
            .map_err(|error| {
                if error.code == ExtensionErrorCode::FeatureUnavailable {
                    return echo_agent::tasks::RevisionedTaskStoreError::Unsupported {
                        operation: format!("{operation:?}"),
                    };
                }
                let fields = error
                    .details
                    .as_ref()
                    .and_then(|details| details.fields.as_ref());
                let field = |name: &str| {
                    fields
                        .and_then(|fields| fields.iter().find(|field| field.key == name))
                        .map(|field| field.value.as_str())
                };
                if field("task_store_error_class") == Some("conflict") {
                    let revision = |name: &str| match field(name) {
                        Some("absent") => Ok(None),
                        Some(value) => value
                            .parse::<u64>()
                            .map(Some)
                            .map_err(|error| error.to_string()),
                        None => Err(format!("missing {name}")),
                    };
                    if let (Ok(expected), Ok(current)) =
                        (revision("expected_revision"), revision("current_revision"))
                    {
                        return echo_agent::tasks::RevisionedTaskStoreError::Conflict {
                            expected,
                            current,
                        };
                    }
                }
                echo_agent::tasks::RevisionedTaskStoreError::Backend {
                    message: error.message,
                }
            })?;
        match result {
            ExtensionResult::AgentComponentCall(result)
                if result.component == self.component && result.result.operation() == operation =>
            {
                Ok(result.result)
            }
            _ => Err(echo_agent::tasks::RevisionedTaskStoreError::Backend {
                message: "task store returned mismatched component or operation".to_string(),
            }),
        }
    }

    async fn invoke(
        &self,
        call: AgentComponentCallInputWire,
    ) -> echo_agent::error::Result<AgentComponentCallResultWire> {
        self.invoke_scoped(call, true).await
    }

    async fn invoke_connection_scoped(
        &self,
        call: AgentComponentCallInputWire,
    ) -> echo_agent::error::Result<AgentComponentCallResultWire> {
        self.invoke_scoped(call, false).await
    }

    async fn invoke_scoped(
        &self,
        call: AgentComponentCallInputWire,
        resolve_run_cancellation: bool,
    ) -> echo_agent::error::Result<AgentComponentCallResultWire> {
        let operation = call.operation();
        let managed = managed_call_context(&call).is_some();
        if operation.component() != self.component {
            return Err(ReactError::Other(
                "agent component operation does not match its descriptor".to_string(),
            ));
        }
        let result = self
            .bridge
            .invoke_once_with_scope(
                &self.extension,
                Some(session_invocation_context(&self.session_id)),
                ExtensionInvocation::AgentComponentCall(AgentComponentCallWire {
                    component: self.component,
                    call,
                }),
                ReverseInvocationScope {
                    cancellation: self.bridge.connection_cancellation(),
                    resolve_run_cancellation,
                    submitted: None,
                },
            )
            .await
            .map_err(|error| {
                if managed {
                    managed_persistence_error(error, self.component)
                } else {
                    react_error(error)
                }
            })?;
        let ExtensionResult::AgentComponentCall(result) = result else {
            return Err(ReactError::Other(
                "agent component extension returned the wrong result variant".to_string(),
            ));
        };
        if result.component != self.component || result.result.operation() != operation {
            return Err(ReactError::Other(
                "agent component extension returned a mismatched operation".to_string(),
            ));
        }
        Ok(result.result)
    }

    async fn invoke_with_cancellation(
        &self,
        call: AgentComponentCallInputWire,
        cancellation: CancellationToken,
    ) -> echo_agent::error::Result<AgentComponentCallResultWire> {
        let operation = call.operation();
        if operation.component() != self.component {
            return Err(ReactError::Other(
                "agent component operation does not match its descriptor".to_string(),
            ));
        }
        let result = self
            .bridge
            .invoke_once(
                &self.extension,
                Some(session_invocation_context(&self.session_id)),
                ExtensionInvocation::AgentComponentCall(AgentComponentCallWire {
                    component: self.component,
                    call,
                }),
                cancellation,
            )
            .await
            .map_err(sandbox_react_error)?;
        let ExtensionResult::AgentComponentCall(result) = result else {
            return Err(ReactError::Other(
                "agent component extension returned the wrong result variant".to_string(),
            ));
        };
        if result.component != self.component || result.result.operation() != operation {
            return Err(ReactError::Other(
                "agent component extension returned a mismatched operation".to_string(),
            ));
        }
        Ok(result.result)
    }
}

pub(crate) fn latest_agent_component_proxy(
    state: &CoreProfileState,
    bridge: Arc<ExtensionBridge>,
    session_id: &str,
    component: AgentComponentKindWire,
) -> Option<ExtensionAgentComponentProxy> {
    let extension = state
        .handles
        .extensions_of_kind(ExtensionKind::AgentComponent)
        .into_iter()
        .filter(|(_, record)| {
            matches!(
                &record.descriptor,
                ExtensionDescriptor::AgentComponent {
                    component: descriptor_component,
                    ..
                } if *descriptor_component == component
            )
        })
        .max_by_key(|(_, record)| record.registration_order)?
        .0;
    ExtensionAgentComponentProxy::new(bridge, extension, session_id.to_string())
}

fn component_value(value: impl serde::Serialize) -> echo_agent::error::Result<WireValue> {
    WireValue::from_json(
        serde_json::to_value(value).map_err(|error| ReactError::Other(error.to_string()))?,
    )
    .map_err(|error| ReactError::Other(error.to_string()))
}

fn component_decode<T: serde::de::DeserializeOwned>(
    value: WireValue,
) -> echo_agent::error::Result<T> {
    serde_json::from_value(
        value
            .into_json()
            .map_err(|error| ReactError::Other(error.to_string()))?,
    )
    .map_err(|error| ReactError::Other(error.to_string()))
}

fn skill_descriptor_policy_wire(
    descriptor: &echo_agent::skills::external::SkillDescriptor,
) -> echo_agent::error::Result<SkillDescriptorPolicyWire> {
    Ok(SkillDescriptorPolicyWire {
        name: descriptor.name.clone(),
        description: descriptor.description.clone(),
        location: super::wire::path_to_wire(&descriptor.location)
            .map_err(|error| ReactError::Other(error.to_string()))?,
        license: descriptor.license.clone(),
        compatibility: descriptor.compatibility.clone(),
        metadata: descriptor
            .metadata
            .iter()
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect(),
        source: descriptor.source.clone(),
        allowed_tools: descriptor.allowed_tools.clone(),
        shell: descriptor.shell.clone(),
        paths: descriptor.paths.clone(),
        triggers: descriptor.triggers.clone(),
        hooks: descriptor.hooks.as_ref().map(component_value).transpose()?,
        sandbox: descriptor
            .sandbox
            .as_ref()
            .map(component_value)
            .transpose()?,
        depends_on: descriptor.depends_on.clone(),
    })
}

fn component_wire_usize(value: &WireU64, field: &str) -> echo_agent::error::Result<usize> {
    value
        .to_u64()
        .and_then(|value| usize::try_from(value).ok())
        .ok_or_else(|| ReactError::Other(format!("{field} exceeds usize")))
}

fn component_duration(
    value: echo_sdk_protocol::scalar::WireDuration,
) -> echo_agent::error::Result<Duration> {
    let seconds = value
        .seconds
        .to_u64()
        .ok_or_else(|| ReactError::Other("duration seconds exceed u64".to_string()))?;
    if value.nanos >= 1_000_000_000 {
        return Err(ReactError::Other(
            "duration nanos must be below one second".to_string(),
        ));
    }
    Ok(Duration::new(seconds, value.nanos))
}

fn sandbox_stream_chunk(
    value: AgentComponentStreamChunkWire,
) -> echo_agent::error::Result<echo_agent::sandbox::SandboxStreamEvent> {
    match value {
        AgentComponentStreamChunkWire::Sandbox(SandboxStreamChunkWire::Output {
            channel,
            chunk,
        }) => Ok(echo_agent::sandbox::SandboxStreamEvent::Output {
            channel: match channel {
                SandboxOutputChannelWire::Stdout => {
                    echo_agent::sandbox::SandboxOutputChannel::Stdout
                }
                SandboxOutputChannelWire::Stderr => {
                    echo_agent::sandbox::SandboxOutputChannel::Stderr
                }
            },
            chunk,
        }),
        AgentComponentStreamChunkWire::Workflow(_) => Err(ReactError::Other(
            "sandbox extension returned a workflow stream chunk".to_string(),
        )),
    }
}

fn sandbox_stream_complete(
    value: AgentComponentStreamCompleteWire,
) -> echo_agent::error::Result<echo_agent::sandbox::SandboxStreamEvent> {
    match value {
        AgentComponentStreamCompleteWire::Sandbox(SandboxStreamCompleteWire::Complete {
            result,
        }) => Ok(echo_agent::sandbox::SandboxStreamEvent::Complete(
            component_decode(result)?,
        )),
        AgentComponentStreamCompleteWire::Sandbox(SandboxStreamCompleteWire::Failed {
            failure,
        }) => Ok(echo_agent::sandbox::SandboxStreamEvent::Failed {
            failure: match failure {
                SandboxStreamFailureWire::Cancelled { message } => {
                    echo_agent::sandbox::SandboxStreamFailure::Cancelled { message }
                }
                SandboxStreamFailureWire::IoError { message } => {
                    echo_agent::sandbox::SandboxStreamFailure::IoError { message }
                }
            },
        }),
        AgentComponentStreamCompleteWire::Workflow(_) => Err(ReactError::Other(
            "sandbox extension returned a workflow stream terminal".to_string(),
        )),
    }
}

fn workflow_stream_chunk(
    value: AgentComponentStreamChunkWire,
) -> echo_agent::error::Result<echo_agent::workflow::WorkflowEvent> {
    let AgentComponentStreamChunkWire::Workflow(event) = value else {
        return Err(ReactError::Other(
            "workflow extension returned a sandbox stream chunk".to_string(),
        ));
    };
    match event {
        WorkflowStreamChunkWire::NodeStart {
            node_name,
            step_index,
        } => Ok(echo_agent::workflow::WorkflowEvent::NodeStart {
            node_name,
            step_index: component_wire_usize(&step_index, "workflow step_index")?,
        }),
        WorkflowStreamChunkWire::NodeEnd {
            node_name,
            step_index,
            elapsed,
        } => Ok(echo_agent::workflow::WorkflowEvent::NodeEnd {
            node_name,
            step_index: component_wire_usize(&step_index, "workflow step_index")?,
            elapsed: component_duration(elapsed)?,
        }),
        WorkflowStreamChunkWire::Token { node_name, token } => {
            Ok(echo_agent::workflow::WorkflowEvent::Token { node_name, token })
        }
        WorkflowStreamChunkWire::NodeError { node_name, error } => {
            Ok(echo_agent::workflow::WorkflowEvent::NodeError { node_name, error })
        }
    }
}

fn workflow_stream_complete(
    value: AgentComponentStreamCompleteWire,
) -> echo_agent::error::Result<echo_agent::workflow::WorkflowEvent> {
    let AgentComponentStreamCompleteWire::Workflow(WorkflowStreamCompleteWire {
        result,
        total_steps,
        elapsed,
    }) = value
    else {
        return Err(ReactError::Other(
            "workflow extension returned a sandbox stream terminal".to_string(),
        ));
    };
    Ok(echo_agent::workflow::WorkflowEvent::Completed {
        result,
        total_steps: component_wire_usize(&total_steps, "workflow total_steps")?,
        elapsed: component_duration(elapsed)?,
    })
}

fn component_decode_many<T: serde::de::DeserializeOwned>(
    values: Vec<WireValue>,
) -> echo_agent::error::Result<Vec<T>> {
    values.into_iter().map(component_decode).collect()
}

fn component_mismatch(operation: AgentComponentOperationWire) -> ReactError {
    ReactError::Other(format!(
        "agent component returned the wrong payload for {operation:?}",
    ))
}

fn usize_wire(value: usize, field: &str) -> echo_agent::error::Result<WireU64> {
    u64::try_from(value)
        .map(WireU64::from_u64)
        .map_err(|_| ReactError::Other(format!("{field} exceeds WireU64")))
}

impl echo_agent::memory::ConversationStore for ExtensionAgentComponentProxy {
    fn projection_capability(&self) -> echo_agent::memory::ConversationProjectionCapability {
        match self.capabilities.conversation_projection {
            ConversationProjectionCapabilityWire::Unsupported => {
                echo_agent::memory::ConversationProjectionCapability::Unsupported
            }
            ConversationProjectionCapabilityWire::AtomicV1 => {
                echo_agent::memory::ConversationProjectionCapability::AtomicV1
            }
        }
    }
    fn persistence_call_capability(&self) -> echo_agent::memory::PersistenceCallCapability {
        match self.capabilities.persistence_call {
            PersistenceCallCapabilityWire::Unsupported => {
                echo_agent::memory::PersistenceCallCapability::Unsupported
            }
            PersistenceCallCapabilityWire::AbsoluteDeadlineV1 => {
                echo_agent::memory::PersistenceCallCapability::AbsoluteDeadlineV1
            }
        }
    }
    fn ensure_projection_epoch_with_context<'a>(
        &'a self,
        context: echo_agent::memory::PersistenceCallContext,
        request: echo_agent::memory::EnsureConversationProjectionRequest,
    ) -> futures::future::BoxFuture<
        'a,
        echo_agent::error::Result<echo_agent::memory::ConversationProjectionEpochReceipt>,
    > {
        Box::pin(async move {
            self.require_managed_persistence()?;
            let context = self.managed_context(context)?;
            let result = self
                .invoke_connection_scoped(
                    AgentComponentCallInputWire::ConversationEnsureProjectionEpochWithContext {
                        context,
                        request: request.project().map_err(ReactError::Other)?,
                    },
                )
                .await?;
            match result {
                AgentComponentCallResultWire::ConversationEnsureProjectionEpochWithContext {
                    receipt,
                } => super::wire::managed::Projection::restore(receipt).map_err(ReactError::Other),
                _ => Err(component_mismatch(
                    AgentComponentOperationWire::ConversationEnsureProjectionEpochWithContext,
                )),
            }
        })
    }
    fn get_projection_authority_with_context<'a>(
        &'a self,
        context: echo_agent::memory::PersistenceCallContext,
        conversation_id: &'a str,
    ) -> futures::future::BoxFuture<
        'a,
        echo_agent::error::Result<Option<echo_agent::memory::ConversationProjectionAuthority>>,
    > {
        Box::pin(async move {
            self.require_managed_persistence()?;
            let context = self.managed_context(context)?;
            let result = self
                .invoke_connection_scoped(
                    AgentComponentCallInputWire::ConversationGetProjectionAuthorityWithContext {
                        context,
                        conversation_id: conversation_id.to_string(),
                    },
                )
                .await?;
            match result {
                AgentComponentCallResultWire::ConversationGetProjectionAuthorityWithContext {
                    authority,
                } => {
                    super::wire::managed::Projection::restore(authority).map_err(ReactError::Other)
                }
                _ => Err(component_mismatch(
                    AgentComponentOperationWire::ConversationGetProjectionAuthorityWithContext,
                )),
            }
        })
    }
    fn apply_transcript_projection_with_context<'a>(
        &'a self,
        context: echo_agent::memory::PersistenceCallContext,
        batch: echo_agent::memory::TranscriptProjectionBatch,
    ) -> futures::future::BoxFuture<
        'a,
        echo_agent::error::Result<echo_agent::memory::TranscriptProjectionApplyReceipt>,
    > {
        Box::pin(async move {
            self.require_managed_persistence()?;
            let context = self.managed_context(context)?;
            let result = self
                .invoke_connection_scoped(
                    AgentComponentCallInputWire::ConversationApplyTranscriptProjectionWithContext {
                        context,
                        batch: batch.project().map_err(ReactError::Other)?,
                    },
                )
                .await?;
            match result {
                AgentComponentCallResultWire::ConversationApplyTranscriptProjectionWithContext { receipt } => {
                    super::wire::managed::Projection::restore(receipt).map_err(ReactError::Other)
                },
                _ => Err(component_mismatch(AgentComponentOperationWire::ConversationApplyTranscriptProjectionWithContext)),
            }
        })
    }
    fn delete_managed_conversation_with_context<'a>(
        &'a self,
        context: echo_agent::memory::PersistenceCallContext,
        request: echo_agent::memory::ManagedConversationDelete,
    ) -> futures::future::BoxFuture<
        'a,
        echo_agent::error::Result<echo_agent::memory::ManagedConversationDeleteReceipt>,
    > {
        Box::pin(async move {
            self.require_managed_persistence()?;
            let context = self.managed_context(context)?;
            let result = self
                .invoke_connection_scoped(
                    AgentComponentCallInputWire::ConversationDeleteManagedWithContext {
                        context,
                        request: request.project().map_err(ReactError::Other)?,
                    },
                )
                .await?;
            match result {
                AgentComponentCallResultWire::ConversationDeleteManagedWithContext { receipt } => {
                    super::wire::managed::Projection::restore(receipt).map_err(ReactError::Other)
                }
                _ => Err(component_mismatch(
                    AgentComponentOperationWire::ConversationDeleteManagedWithContext,
                )),
            }
        })
    }

    fn create_conversation<'a>(
        &'a self,
        conversation: echo_agent::memory::NewConversation,
    ) -> futures::future::BoxFuture<'a, echo_agent::error::Result<echo_agent::memory::Conversation>>
    {
        Box::pin(async move {
            let result = self
                .invoke(AgentComponentCallInputWire::ConversationCreate {
                    conversation: component_value(conversation)?,
                })
                .await?;
            match result {
                AgentComponentCallResultWire::ConversationCreate { conversation } => {
                    component_decode(conversation)
                }
                _ => Err(component_mismatch(
                    AgentComponentOperationWire::ConversationCreate,
                )),
            }
        })
    }

    fn get_conversation<'a>(
        &'a self,
        conversation_id: &'a str,
    ) -> futures::future::BoxFuture<
        'a,
        echo_agent::error::Result<Option<echo_agent::memory::Conversation>>,
    > {
        let conversation_id = conversation_id.to_string();
        Box::pin(async move {
            let result = self
                .invoke(AgentComponentCallInputWire::ConversationGet { conversation_id })
                .await?;
            match result {
                AgentComponentCallResultWire::ConversationGet { conversation } => {
                    conversation.map(component_decode).transpose()
                }
                _ => Err(component_mismatch(
                    AgentComponentOperationWire::ConversationGet,
                )),
            }
        })
    }

    fn list_conversations<'a>(
        &'a self,
        filter: echo_agent::memory::ConversationFilter,
    ) -> futures::future::BoxFuture<
        'a,
        echo_agent::error::Result<Vec<echo_agent::memory::ConversationMeta>>,
    > {
        Box::pin(async move {
            let limit = filter
                .limit
                .map(|value| usize_wire(value, "limit"))
                .transpose()?;
            let offset = filter
                .offset
                .map(|value| usize_wire(value, "offset"))
                .transpose()?;
            let result = self
                .invoke(AgentComponentCallInputWire::ConversationList {
                    user_id: filter.user_id,
                    agent_type: filter.agent_type,
                    limit,
                    offset,
                })
                .await?;
            match result {
                AgentComponentCallResultWire::ConversationList { conversations } => {
                    component_decode_many(conversations)
                }
                _ => Err(component_mismatch(
                    AgentComponentOperationWire::ConversationList,
                )),
            }
        })
    }

    fn update_conversation<'a>(
        &'a self,
        conversation_id: &'a str,
        title: Option<&'a str>,
        summary: Option<&'a str>,
        compressed_before_id: Option<i64>,
    ) -> futures::future::BoxFuture<'a, echo_agent::error::Result<()>> {
        let conversation_id = conversation_id.to_string();
        let title = title.map(str::to_string);
        let summary = summary.map(str::to_string);
        Box::pin(async move {
            let result = self
                .invoke(AgentComponentCallInputWire::ConversationUpdate {
                    conversation_id,
                    title,
                    summary,
                    compressed_before_id: compressed_before_id.map(WireI64::from_i64),
                })
                .await?;
            match result {
                AgentComponentCallResultWire::ConversationUpdate => Ok(()),
                _ => Err(component_mismatch(
                    AgentComponentOperationWire::ConversationUpdate,
                )),
            }
        })
    }

    fn delete_conversation<'a>(
        &'a self,
        conversation_id: &'a str,
    ) -> futures::future::BoxFuture<'a, echo_agent::error::Result<()>> {
        let conversation_id = conversation_id.to_string();
        Box::pin(async move {
            let result = self
                .invoke(AgentComponentCallInputWire::ConversationDelete { conversation_id })
                .await?;
            match result {
                AgentComponentCallResultWire::ConversationDelete => Ok(()),
                _ => Err(component_mismatch(
                    AgentComponentOperationWire::ConversationDelete,
                )),
            }
        })
    }

    fn save_messages<'a>(
        &'a self,
        conversation_id: &'a str,
        messages: &'a [echo_agent::memory::StoredMessage],
    ) -> futures::future::BoxFuture<'a, echo_agent::error::Result<()>> {
        let conversation_id = conversation_id.to_string();
        let messages = messages
            .iter()
            .map(component_value)
            .collect::<echo_agent::error::Result<Vec<_>>>();
        Box::pin(async move {
            let result = self
                .invoke(AgentComponentCallInputWire::ConversationSaveMessages {
                    conversation_id,
                    messages: messages?,
                })
                .await?;
            match result {
                AgentComponentCallResultWire::ConversationSaveMessages => Ok(()),
                _ => Err(component_mismatch(
                    AgentComponentOperationWire::ConversationSaveMessages,
                )),
            }
        })
    }

    fn get_messages<'a>(
        &'a self,
        conversation_id: &'a str,
    ) -> futures::future::BoxFuture<
        'a,
        echo_agent::error::Result<Vec<echo_agent::memory::StoredMessage>>,
    > {
        let conversation_id = conversation_id.to_string();
        Box::pin(async move {
            let result = self
                .invoke(AgentComponentCallInputWire::ConversationGetMessages { conversation_id })
                .await?;
            match result {
                AgentComponentCallResultWire::ConversationGetMessages { messages } => {
                    component_decode_many(messages)
                }
                _ => Err(component_mismatch(
                    AgentComponentOperationWire::ConversationGetMessages,
                )),
            }
        })
    }

    fn count_messages<'a>(
        &'a self,
        conversation_id: &'a str,
    ) -> futures::future::BoxFuture<'a, echo_agent::error::Result<usize>> {
        let conversation_id = conversation_id.to_string();
        Box::pin(async move {
            let result = self
                .invoke(AgentComponentCallInputWire::ConversationCountMessages { conversation_id })
                .await?;
            match result {
                AgentComponentCallResultWire::ConversationCountMessages { count } => count
                    .to_u64()
                    .and_then(|value| usize::try_from(value).ok())
                    .ok_or_else(|| ReactError::Other("message count exceeds usize".to_string())),
                _ => Err(component_mismatch(
                    AgentComponentOperationWire::ConversationCountMessages,
                )),
            }
        })
    }

    fn ensure_conversation<'a>(
        &'a self,
        conv: echo_agent::memory::NewConversation,
    ) -> futures::future::BoxFuture<'a, echo_agent::error::Result<echo_agent::memory::Conversation>>
    {
        Box::pin(async move {
            let result = self
                .invoke(AgentComponentCallInputWire::ConversationEnsure {
                    conversation: component_value(conv)?,
                })
                .await?;
            match result {
                AgentComponentCallResultWire::ConversationEnsure { conversation } => {
                    component_decode(conversation)
                }
                _ => Err(component_mismatch(
                    AgentComponentOperationWire::ConversationEnsure,
                )),
            }
        })
    }

    fn search_conversations<'a>(
        &'a self,
        query: &'a str,
        limit: usize,
    ) -> futures::future::BoxFuture<
        'a,
        echo_agent::error::Result<Vec<echo_agent::memory::ConversationMeta>>,
    > {
        let query = query.to_string();
        Box::pin(async move {
            let result = self
                .invoke(AgentComponentCallInputWire::ConversationSearch {
                    query,
                    limit: usize_wire(limit, "conversation search limit")?,
                })
                .await?;
            match result {
                AgentComponentCallResultWire::ConversationSearch { conversations } => {
                    component_decode_many(conversations)
                }
                _ => Err(component_mismatch(
                    AgentComponentOperationWire::ConversationSearch,
                )),
            }
        })
    }
}

#[async_trait::async_trait]
impl echo_agent::trace::RunStore for ExtensionAgentComponentProxy {
    async fn save(&self, run: echo_agent::trace::Run) -> echo_agent::error::Result<()> {
        let result = self
            .invoke(AgentComponentCallInputWire::RunSave {
                run: component_value(run)?,
            })
            .await?;
        match result {
            AgentComponentCallResultWire::RunSave => Ok(()),
            _ => Err(component_mismatch(AgentComponentOperationWire::RunSave)),
        }
    }

    async fn load(
        &self,
        run_id: &str,
    ) -> echo_agent::error::Result<Option<echo_agent::trace::Run>> {
        let result = self
            .invoke(AgentComponentCallInputWire::RunLoad {
                run_id: run_id.to_string(),
            })
            .await?;
        match result {
            AgentComponentCallResultWire::RunLoad { run } => run.map(component_decode).transpose(),
            _ => Err(component_mismatch(AgentComponentOperationWire::RunLoad)),
        }
    }

    async fn list_by_session(
        &self,
        session_id: &str,
    ) -> echo_agent::error::Result<Vec<echo_agent::trace::RunSummary>> {
        let result = self
            .invoke(AgentComponentCallInputWire::RunListBySession {
                session_id: session_id.to_string(),
            })
            .await?;
        match result {
            AgentComponentCallResultWire::RunListBySession { runs } => component_decode_many(runs),
            _ => Err(component_mismatch(
                AgentComponentOperationWire::RunListBySession,
            )),
        }
    }

    async fn list_all(
        &self,
        limit: usize,
    ) -> echo_agent::error::Result<Vec<echo_agent::trace::RunSummary>> {
        let result = self
            .invoke(AgentComponentCallInputWire::RunListAll {
                limit: usize_wire(limit, "limit")?,
            })
            .await?;
        match result {
            AgentComponentCallResultWire::RunListAll { runs } => component_decode_many(runs),
            _ => Err(component_mismatch(AgentComponentOperationWire::RunListAll)),
        }
    }

    async fn append_event(
        &self,
        run_id: &str,
        event: echo_agent::trace::RunEvent,
    ) -> echo_agent::error::Result<()> {
        let result = self
            .invoke(AgentComponentCallInputWire::RunAppendEvent {
                run_id: run_id.to_string(),
                event: component_value(event)?,
            })
            .await?;
        match result {
            AgentComponentCallResultWire::RunAppendEvent => Ok(()),
            _ => Err(component_mismatch(
                AgentComponentOperationWire::RunAppendEvent,
            )),
        }
    }

    async fn list_by_parent_run(
        &self,
        parent_run_id: &str,
    ) -> echo_agent::error::Result<Vec<echo_agent::trace::RunSummary>> {
        let result = self
            .invoke(AgentComponentCallInputWire::RunListByParent {
                parent_run_id: parent_run_id.to_string(),
            })
            .await?;
        match result {
            AgentComponentCallResultWire::RunListByParent { runs } => component_decode_many(runs),
            _ => Err(component_mismatch(
                AgentComponentOperationWire::RunListByParent,
            )),
        }
    }
}

impl echo_agent::state::RuntimeStateStore for ExtensionAgentComponentProxy {
    fn runtime_state_capability(&self) -> echo_agent::state::RuntimeStateCapability {
        match self.capabilities.runtime_state {
            RuntimeStateCapabilityWire::Unsupported => {
                echo_agent::state::RuntimeStateCapability::Unsupported
            }
            RuntimeStateCapabilityWire::RevisionedV1 => {
                echo_agent::state::RuntimeStateCapability::RevisionedV1
            }
        }
    }
    fn persistence_call_capability(&self) -> echo_agent::memory::PersistenceCallCapability {
        match self.capabilities.persistence_call {
            PersistenceCallCapabilityWire::Unsupported => {
                echo_agent::memory::PersistenceCallCapability::Unsupported
            }
            PersistenceCallCapabilityWire::AbsoluteDeadlineV1 => {
                echo_agent::memory::PersistenceCallCapability::AbsoluteDeadlineV1
            }
        }
    }
    fn load_runtime_state_with_context<'a>(
        &'a self,
        context: echo_agent::memory::PersistenceCallContext,
        scope_id: &'a str,
        runtime_state_id: &'a str,
    ) -> futures::future::BoxFuture<
        'a,
        echo_agent::error::Result<Option<echo_agent::state::ManagedRuntimeStateSnapshot>>,
    > {
        Box::pin(async move {
            self.require_managed_persistence()?;
            let context = self.managed_context(context)?;
            let result = self
                .invoke_connection_scoped(
                    AgentComponentCallInputWire::RuntimeLoadManagedStateWithContext {
                        context,
                        scope_id: scope_id.to_string(),
                        runtime_state_id: runtime_state_id.to_string(),
                    },
                )
                .await?;
            match result {
                AgentComponentCallResultWire::RuntimeLoadManagedStateWithContext { snapshot } => {
                    super::wire::managed::Projection::restore(snapshot).map_err(ReactError::Other)
                }
                _ => Err(component_mismatch(
                    AgentComponentOperationWire::RuntimeLoadManagedStateWithContext,
                )),
            }
        })
    }
    fn load_scope_authority_with_context<'a>(
        &'a self,
        context: echo_agent::memory::PersistenceCallContext,
        scope_id: &'a str,
    ) -> futures::future::BoxFuture<
        'a,
        echo_agent::error::Result<Option<echo_agent::state::RuntimeScopeAuthority>>,
    > {
        Box::pin(async move {
            self.require_managed_persistence()?;
            let context = self.managed_context(context)?;
            let result = self
                .invoke_connection_scoped(
                    AgentComponentCallInputWire::RuntimeLoadScopeAuthorityWithContext {
                        context,
                        scope_id: scope_id.to_string(),
                    },
                )
                .await?;
            match result {
                AgentComponentCallResultWire::RuntimeLoadScopeAuthorityWithContext {
                    authority,
                } => {
                    super::wire::managed::Projection::restore(authority).map_err(ReactError::Other)
                }
                _ => Err(component_mismatch(
                    AgentComponentOperationWire::RuntimeLoadScopeAuthorityWithContext,
                )),
            }
        })
    }
    fn compare_and_save_checkpoint_with_context<'a>(
        &'a self,
        context: echo_agent::memory::PersistenceCallContext,
        request: echo_agent::state::RuntimeCheckpointCasRequest,
    ) -> futures::future::BoxFuture<
        'a,
        echo_agent::error::Result<echo_agent::state::RuntimeCheckpointCasReceipt>,
    > {
        Box::pin(async move {
            self.require_managed_persistence()?;
            let context = self.managed_context(context)?;
            let result = self
                .invoke_connection_scoped(
                    AgentComponentCallInputWire::RuntimeCompareAndSaveCheckpointWithContext {
                        context,
                        request: request.project().map_err(ReactError::Other)?,
                    },
                )
                .await?;
            match result {
                AgentComponentCallResultWire::RuntimeCompareAndSaveCheckpointWithContext {
                    receipt,
                } => super::wire::managed::Projection::restore(receipt).map_err(ReactError::Other),
                _ => Err(component_mismatch(
                    AgentComponentOperationWire::RuntimeCompareAndSaveCheckpointWithContext,
                )),
            }
        })
    }
    fn acknowledge_transcript_projection_with_context<'a>(
        &'a self,
        context: echo_agent::memory::PersistenceCallContext,
        request: echo_agent::state::RuntimeTranscriptAckRequest,
    ) -> futures::future::BoxFuture<
        'a,
        echo_agent::error::Result<echo_agent::state::RuntimeCheckpointCasReceipt>,
    > {
        Box::pin(async move {
            self.require_managed_persistence()?;
            let context = self.managed_context(context)?;
            let result = self.invoke_connection_scoped(AgentComponentCallInputWire::RuntimeAcknowledgeTranscriptProjectionWithContext {
                context,
                request: request.project().map_err(ReactError::Other)?,
            }).await?;
            match result {
                AgentComponentCallResultWire::RuntimeAcknowledgeTranscriptProjectionWithContext { receipt } => {
                    super::wire::managed::Projection::restore(receipt).map_err(ReactError::Other)
                },
                _ => Err(component_mismatch(AgentComponentOperationWire::RuntimeAcknowledgeTranscriptProjectionWithContext)),
            }
        })
    }
    fn retire_runtime_generation_with_context<'a>(
        &'a self,
        context: echo_agent::memory::PersistenceCallContext,
        request: echo_agent::state::RuntimeGenerationRetireRequest,
    ) -> futures::future::BoxFuture<
        'a,
        echo_agent::error::Result<echo_agent::state::RuntimeGenerationRetireReceipt>,
    > {
        Box::pin(async move {
            self.require_managed_persistence()?;
            let context = self.managed_context(context)?;
            let result = self
                .invoke_connection_scoped(
                    AgentComponentCallInputWire::RuntimeRetireGenerationWithContext {
                        context,
                        request: request.project().map_err(ReactError::Other)?,
                    },
                )
                .await?;
            match result {
                AgentComponentCallResultWire::RuntimeRetireGenerationWithContext { receipt } => {
                    super::wire::managed::Projection::restore(receipt).map_err(ReactError::Other)
                }
                _ => Err(component_mismatch(
                    AgentComponentOperationWire::RuntimeRetireGenerationWithContext,
                )),
            }
        })
    }
    fn begin_scope_retirement_with_context<'a>(
        &'a self,
        context: echo_agent::memory::PersistenceCallContext,
        request: echo_agent::state::ScopeRetirementRequest,
    ) -> futures::future::BoxFuture<
        'a,
        echo_agent::error::Result<echo_agent::state::ScopeRetirementReceipt>,
    > {
        Box::pin(async move {
            self.require_managed_persistence()?;
            let context = self.managed_context(context)?;
            let result = self
                .invoke_connection_scoped(
                    AgentComponentCallInputWire::RuntimeBeginScopeRetirementWithContext {
                        context,
                        request: request.project().map_err(ReactError::Other)?,
                    },
                )
                .await?;
            match result {
                AgentComponentCallResultWire::RuntimeBeginScopeRetirementWithContext {
                    receipt,
                } => super::wire::managed::Projection::restore(receipt).map_err(ReactError::Other),
                _ => Err(component_mismatch(
                    AgentComponentOperationWire::RuntimeBeginScopeRetirementWithContext,
                )),
            }
        })
    }
    fn continue_scope_retirement_with_context<'a>(
        &'a self,
        context: echo_agent::memory::PersistenceCallContext,
        scope_id: &'a str,
        delete_operation_id: &'a str,
        expected_scope_revision: u64,
        advance: echo_agent::state::ScopeRetirementAdvance,
    ) -> futures::future::BoxFuture<
        'a,
        echo_agent::error::Result<echo_agent::state::ScopeRetirementReceipt>,
    > {
        Box::pin(async move {
            self.require_managed_persistence()?;
            let context = self.managed_context(context)?;
            let result = self
                .invoke_connection_scoped(
                    AgentComponentCallInputWire::RuntimeContinueScopeRetirementWithContext {
                        context,
                        scope_id: scope_id.to_string(),
                        delete_operation_id: delete_operation_id.to_string(),
                        expected_scope_revision: expected_scope_revision
                            .project()
                            .map_err(ReactError::Other)?,
                        advance: advance.project().map_err(ReactError::Other)?,
                    },
                )
                .await?;
            match result {
                AgentComponentCallResultWire::RuntimeContinueScopeRetirementWithContext {
                    receipt,
                } => super::wire::managed::Projection::restore(receipt).map_err(ReactError::Other),
                _ => Err(component_mismatch(
                    AgentComponentOperationWire::RuntimeContinueScopeRetirementWithContext,
                )),
            }
        })
    }

    fn get_checkpoint<'a>(
        &'a self,
        conversation_id: &'a str,
    ) -> futures::future::BoxFuture<
        'a,
        echo_agent::error::Result<Option<echo_agent::state::AgentCheckpoint>>,
    > {
        let conversation_id = conversation_id.to_string();
        Box::pin(async move {
            let result = self
                .invoke(AgentComponentCallInputWire::RuntimeGetCheckpoint { conversation_id })
                .await?;
            match result {
                AgentComponentCallResultWire::RuntimeGetCheckpoint { checkpoint } => {
                    checkpoint.map(component_decode).transpose()
                }
                _ => Err(component_mismatch(
                    AgentComponentOperationWire::RuntimeGetCheckpoint,
                )),
            }
        })
    }

    fn save_checkpoint<'a>(
        &'a self,
        checkpoint: &'a echo_agent::state::AgentCheckpoint,
    ) -> futures::future::BoxFuture<'a, echo_agent::error::Result<()>> {
        let checkpoint = checkpoint.clone();
        Box::pin(async move {
            let result = self
                .invoke(AgentComponentCallInputWire::RuntimeSaveCheckpoint {
                    checkpoint: component_value(checkpoint)?,
                })
                .await?;
            match result {
                AgentComponentCallResultWire::RuntimeSaveCheckpoint => Ok(()),
                _ => Err(component_mismatch(
                    AgentComponentOperationWire::RuntimeSaveCheckpoint,
                )),
            }
        })
    }

    fn save_checkpoint_for_scope<'a>(
        &'a self,
        scope_id: &'a str,
        checkpoint: &'a echo_agent::state::AgentCheckpoint,
    ) -> futures::future::BoxFuture<'a, echo_agent::error::Result<()>> {
        let scope_id = scope_id.to_string();
        let checkpoint = checkpoint.clone();
        Box::pin(async move {
            let result = self
                .invoke(AgentComponentCallInputWire::RuntimeSaveCheckpointForScope {
                    scope_id,
                    checkpoint: component_value(checkpoint)?,
                })
                .await?;
            match result {
                AgentComponentCallResultWire::RuntimeSaveCheckpointForScope => Ok(()),
                _ => Err(component_mismatch(
                    AgentComponentOperationWire::RuntimeSaveCheckpointForScope,
                )),
            }
        })
    }

    fn runtime_state_ids<'a>(
        &'a self,
        scope_id: &'a str,
    ) -> futures::future::BoxFuture<'a, echo_agent::error::Result<Vec<String>>> {
        let scope_id = scope_id.to_string();
        Box::pin(async move {
            let result = self
                .invoke(AgentComponentCallInputWire::RuntimeStateIds { scope_id })
                .await?;
            match result {
                AgentComponentCallResultWire::RuntimeStateIds { state_ids } => Ok(state_ids),
                _ => Err(component_mismatch(
                    AgentComponentOperationWire::RuntimeStateIds,
                )),
            }
        })
    }

    fn clear_runtime_state<'a>(
        &'a self,
        scope_id: &'a str,
        runtime_state_id: &'a str,
    ) -> futures::future::BoxFuture<
        'a,
        echo_agent::error::Result<echo_agent::state::RuntimeStateClearReceipt>,
    > {
        let scope_id = scope_id.to_string();
        let runtime_state_id = runtime_state_id.to_string();
        Box::pin(async move {
            let result = self
                .invoke(AgentComponentCallInputWire::RuntimeClearState {
                    scope_id,
                    runtime_state_id,
                })
                .await?;
            match result {
                AgentComponentCallResultWire::RuntimeClearState { receipt } => {
                    component_decode(receipt)
                }
                _ => Err(component_mismatch(
                    AgentComponentOperationWire::RuntimeClearState,
                )),
            }
        })
    }

    fn clear_runtime_state_scope<'a>(
        &'a self,
        scope_id: &'a str,
    ) -> futures::future::BoxFuture<
        'a,
        echo_agent::error::Result<echo_agent::state::RuntimeStateScopeClearReceipt>,
    > {
        let scope_id = scope_id.to_string();
        Box::pin(async move {
            let result = self
                .invoke(AgentComponentCallInputWire::RuntimeClearScope { scope_id })
                .await?;
            match result {
                AgentComponentCallResultWire::RuntimeClearScope { receipt } => {
                    component_decode(receipt)
                }
                _ => Err(component_mismatch(
                    AgentComponentOperationWire::RuntimeClearScope,
                )),
            }
        })
    }

    fn clear_conversation<'a>(
        &'a self,
        conversation_id: &'a str,
    ) -> futures::future::BoxFuture<'a, echo_agent::error::Result<()>> {
        let conversation_id = conversation_id.to_string();
        Box::pin(async move {
            let result = self
                .invoke(AgentComponentCallInputWire::RuntimeClearConversation { conversation_id })
                .await?;
            match result {
                AgentComponentCallResultWire::RuntimeClearConversation => Ok(()),
                _ => Err(component_mismatch(
                    AgentComponentOperationWire::RuntimeClearConversation,
                )),
            }
        })
    }
}

impl echo_agent::audit::AuditLogger for ExtensionAgentComponentProxy {
    fn log<'a>(
        &'a self,
        event: echo_agent::audit::AuditEvent,
    ) -> futures::future::BoxFuture<'a, echo_agent::error::Result<()>> {
        Box::pin(async move {
            let result = self
                .invoke(AgentComponentCallInputWire::AuditLog {
                    event: component_value(event)?,
                })
                .await?;
            match result {
                AgentComponentCallResultWire::AuditLog => Ok(()),
                _ => Err(component_mismatch(AgentComponentOperationWire::AuditLog)),
            }
        })
    }

    fn query<'a>(
        &'a self,
        filter: echo_agent::audit::AuditFilter,
    ) -> futures::future::BoxFuture<'a, echo_agent::error::Result<Vec<echo_agent::audit::AuditEvent>>>
    {
        Box::pin(async move {
            let limit = filter
                .limit
                .map(|value| usize_wire(value, "limit"))
                .transpose()?;
            let result = self
                .invoke(AgentComponentCallInputWire::AuditQuery {
                    session_id: filter.session_id,
                    agent_name: filter.agent_name,
                    from: filter.from.map(|value| value.to_rfc3339()),
                    to: filter.to.map(|value| value.to_rfc3339()),
                    limit,
                })
                .await?;
            match result {
                AgentComponentCallResultWire::AuditQuery { events } => {
                    component_decode_many(events)
                }
                _ => Err(component_mismatch(AgentComponentOperationWire::AuditQuery)),
            }
        })
    }
}

impl echo_agent::compression::PreModelContextProjector for ExtensionAgentComponentProxy {
    fn project<'a>(
        &'a self,
        context: &'a echo_agent::compression::ProjectionContext,
    ) -> futures::future::BoxFuture<
        'a,
        echo_agent::error::Result<Vec<echo_agent::compression::ContextProjection>>,
    > {
        let iteration = match usize_wire(context.iteration, "iteration") {
            Ok(value) => value,
            Err(error) => return Box::pin(async move { Err(error) }),
        };
        let agent_name = context.agent_name.clone();
        let session_id = context.session_id.clone();
        let conversation_id = context.conversation_id.clone();
        let run_id = context.run_id.clone();
        let turn_id = context.turn_id.clone();
        Box::pin(async move {
            #[derive(serde::Deserialize)]
            struct ProjectionWire {
                marker: String,
                message: Option<LlmMessageWire>,
            }
            let result = self
                .invoke(AgentComponentCallInputWire::ContextProject {
                    iteration,
                    agent_name,
                    session_id,
                    conversation_id,
                    run_id,
                    turn_id,
                })
                .await?;
            let AgentComponentCallResultWire::ContextProject { projections } = result else {
                return Err(component_mismatch(
                    AgentComponentOperationWire::ContextProject,
                ));
            };
            let values: Vec<ProjectionWire> = component_decode_many(projections)?;
            values
                .into_iter()
                .map(|value| {
                    Ok(echo_agent::compression::ContextProjection {
                        marker: value.marker,
                        message: value
                            .message
                            .map(message_from_wire)
                            .transpose()
                            .map_err(ReactError::Other)?,
                    })
                })
                .collect()
        })
    }
}

impl echo_agent::evolution::MemoryTriggerSink for ExtensionAgentComponentProxy {
    fn on_trigger<'a>(
        &'a self,
        trigger: &'a echo_agent::evolution::TriggerMatch,
    ) -> futures::future::BoxFuture<
        'a,
        std::result::Result<echo_agent::evolution::MemoryTriggerDisposition, String>,
    > {
        let input = serde_json::json!({
            "content": trigger.content,
            "memory_type": trigger.memory_type,
            "source": trigger.source,
            "confidence": trigger.confidence,
            "topic": trigger.topic,
            "trust_level": trigger.trust_level,
            "suggested_key": trigger.suggested_key,
            "evidence": trigger.evidence.iter().map(|evidence| serde_json::json!({
                "source_role": evidence.source_role,
                "quote": evidence.quote,
            })).collect::<Vec<_>>(),
        });
        Box::pin(async move {
            let result = self
                .invoke(AgentComponentCallInputWire::MemoryTrigger {
                    trigger: component_value(input).map_err(|error| error.to_string())?,
                })
                .await
                .map_err(|error| error.to_string())?;
            let AgentComponentCallResultWire::MemoryTrigger { disposition } = result else {
                return Err(
                    component_mismatch(AgentComponentOperationWire::MemoryTrigger).to_string(),
                );
            };
            match disposition.as_str() {
                "persist" => Ok(echo_agent::evolution::MemoryTriggerDisposition::Persist),
                "captured" => Ok(echo_agent::evolution::MemoryTriggerDisposition::Captured),
                _ => Err("memory trigger disposition must be persist or captured".to_string()),
            }
        })
    }
}

impl echo_agent::guard::Guard for ExtensionAgentComponentProxy {
    fn name(&self) -> &str {
        &self.name
    }

    fn check<'a>(
        &'a self,
        content: &'a str,
        direction: echo_agent::guard::GuardDirection,
    ) -> futures::future::BoxFuture<'a, echo_agent::error::Result<echo_agent::guard::GuardResult>>
    {
        let content = content.to_string();
        let direction = direction.to_string();
        Box::pin(async move {
            let result = self
                .invoke(AgentComponentCallInputWire::GuardCheck { content, direction })
                .await?;
            match result {
                AgentComponentCallResultWire::GuardCheck { result } => component_decode(result),
                _ => Err(component_mismatch(AgentComponentOperationWire::GuardCheck)),
            }
        })
    }
}

#[cfg(feature = "framework-web")]
#[async_trait::async_trait]
impl echo_agent::tools::web::providers::SearchProvider for ExtensionAgentComponentProxy {
    fn name(&self) -> &str {
        &self.name
    }

    async fn search(
        &self,
        query: &str,
        max_results: usize,
    ) -> echo_agent::error::Result<Vec<echo_agent::tools::web::providers::SearchResult>> {
        let result = self
            .invoke(AgentComponentCallInputWire::SearchProviderSearch {
                query: query.to_string(),
                max_results: usize_wire(max_results, "max_results")?,
            })
            .await?;
        match result {
            AgentComponentCallResultWire::SearchProviderSearch { results } => {
                component_decode_many(results)
            }
            _ => Err(component_mismatch(
                AgentComponentOperationWire::SearchProviderSearch,
            )),
        }
    }
}

#[async_trait::async_trait]
impl echo_agent::workflow::CheckpointStore for ExtensionAgentComponentProxy {
    async fn save(
        &self,
        checkpoint: &echo_agent::workflow::Checkpoint,
    ) -> echo_agent::error::Result<()> {
        let result = self
            .invoke(AgentComponentCallInputWire::WorkflowCheckpointSave {
                checkpoint: component_value(checkpoint)?,
            })
            .await?;
        match result {
            AgentComponentCallResultWire::WorkflowCheckpointSave => Ok(()),
            _ => Err(component_mismatch(
                AgentComponentOperationWire::WorkflowCheckpointSave,
            )),
        }
    }

    async fn load(
        &self,
        id: &str,
    ) -> echo_agent::error::Result<Option<echo_agent::workflow::Checkpoint>> {
        let result = self
            .invoke(AgentComponentCallInputWire::WorkflowCheckpointLoad {
                checkpoint_id: id.to_string(),
            })
            .await?;
        match result {
            AgentComponentCallResultWire::WorkflowCheckpointLoad { checkpoint } => {
                checkpoint.map(component_decode).transpose()
            }
            _ => Err(component_mismatch(
                AgentComponentOperationWire::WorkflowCheckpointLoad,
            )),
        }
    }

    async fn claim(
        &self,
        id: &str,
    ) -> echo_agent::error::Result<Option<echo_agent::workflow::Checkpoint>> {
        let result = self
            .invoke(AgentComponentCallInputWire::WorkflowCheckpointClaim {
                checkpoint_id: id.to_string(),
            })
            .await?;
        match result {
            AgentComponentCallResultWire::WorkflowCheckpointClaim { checkpoint } => {
                checkpoint.map(component_decode).transpose()
            }
            _ => Err(component_mismatch(
                AgentComponentOperationWire::WorkflowCheckpointClaim,
            )),
        }
    }

    async fn ack_claim(&self, id: &str, attempt_id: &str) -> echo_agent::error::Result<()> {
        let result = self
            .invoke(AgentComponentCallInputWire::WorkflowCheckpointAckClaim {
                checkpoint_id: id.to_string(),
                attempt_id: attempt_id.to_string(),
            })
            .await?;
        match result {
            AgentComponentCallResultWire::WorkflowCheckpointAckClaim => Ok(()),
            _ => Err(component_mismatch(
                AgentComponentOperationWire::WorkflowCheckpointAckClaim,
            )),
        }
    }

    async fn requeue_claim(&self, id: &str, attempt_id: &str) -> echo_agent::error::Result<()> {
        let result = self
            .invoke(
                AgentComponentCallInputWire::WorkflowCheckpointRequeueClaim {
                    checkpoint_id: id.to_string(),
                    attempt_id: attempt_id.to_string(),
                },
            )
            .await?;
        match result {
            AgentComponentCallResultWire::WorkflowCheckpointRequeueClaim => Ok(()),
            _ => Err(component_mismatch(
                AgentComponentOperationWire::WorkflowCheckpointRequeueClaim,
            )),
        }
    }

    async fn renew_claim(&self, id: &str, attempt_id: &str) -> echo_agent::error::Result<()> {
        let result = self
            .invoke(AgentComponentCallInputWire::WorkflowCheckpointRenewClaim {
                checkpoint_id: id.to_string(),
                attempt_id: attempt_id.to_string(),
            })
            .await?;
        match result {
            AgentComponentCallResultWire::WorkflowCheckpointRenewClaim => Ok(()),
            _ => Err(component_mismatch(
                AgentComponentOperationWire::WorkflowCheckpointRenewClaim,
            )),
        }
    }

    fn claim_heartbeat_interval(&self) -> Option<Duration> {
        self.capabilities
            .claim_heartbeat_interval_ms
            .as_ref()
            .and_then(WireU64::to_u64)
            .map(Duration::from_millis)
    }

    fn supports_claim_settlement(&self) -> bool {
        true
    }

    async fn save_if_generation(
        &self,
        checkpoint: &echo_agent::workflow::Checkpoint,
        expected_generation: u64,
    ) -> echo_agent::error::Result<bool> {
        let result = self
            .invoke(
                AgentComponentCallInputWire::WorkflowCheckpointSaveIfGeneration {
                    checkpoint: component_value(checkpoint)?,
                    expected_generation: WireU64::from_u64(expected_generation),
                },
            )
            .await?;
        match result {
            AgentComponentCallResultWire::WorkflowCheckpointSaveIfGeneration { committed } => {
                Ok(committed)
            }
            _ => Err(component_mismatch(
                AgentComponentOperationWire::WorkflowCheckpointSaveIfGeneration,
            )),
        }
    }

    async fn list(&self) -> echo_agent::error::Result<Vec<echo_agent::workflow::CheckpointInfo>> {
        let result = self
            .invoke(AgentComponentCallInputWire::WorkflowCheckpointList)
            .await?;
        match result {
            AgentComponentCallResultWire::WorkflowCheckpointList { checkpoints } => {
                component_decode_many(checkpoints)
            }
            _ => Err(component_mismatch(
                AgentComponentOperationWire::WorkflowCheckpointList,
            )),
        }
    }

    async fn list_by_graph(
        &self,
        graph_name: &str,
    ) -> echo_agent::error::Result<Vec<echo_agent::workflow::CheckpointInfo>> {
        let result = self
            .invoke(AgentComponentCallInputWire::WorkflowCheckpointListByGraph {
                graph_name: graph_name.to_string(),
            })
            .await?;
        match result {
            AgentComponentCallResultWire::WorkflowCheckpointListByGraph { checkpoints } => {
                component_decode_many(checkpoints)
            }
            _ => Err(component_mismatch(
                AgentComponentOperationWire::WorkflowCheckpointListByGraph,
            )),
        }
    }

    async fn list_filtered(
        &self,
        filter: &echo_agent::workflow::orchestration::checkpoint_store::CheckpointFilter,
    ) -> echo_agent::error::Result<Vec<echo_agent::workflow::CheckpointInfo>> {
        let filter = WireValue::from_json(serde_json::json!({
            "graph_name": filter.graph_name,
            "branch": filter.branch,
            "tag": filter.tag,
            "limit": filter.limit.map(|value| value.to_string()),
        }))
        .map_err(|error| ReactError::Other(error.to_string()))?;
        let result = self
            .invoke(AgentComponentCallInputWire::WorkflowCheckpointListFiltered { filter })
            .await?;
        match result {
            AgentComponentCallResultWire::WorkflowCheckpointListFiltered { checkpoints } => {
                component_decode_many(checkpoints)
            }
            _ => Err(component_mismatch(
                AgentComponentOperationWire::WorkflowCheckpointListFiltered,
            )),
        }
    }

    async fn delete(&self, id: &str) -> echo_agent::error::Result<()> {
        let result = self
            .invoke(AgentComponentCallInputWire::WorkflowCheckpointDelete {
                checkpoint_id: id.to_string(),
            })
            .await?;
        match result {
            AgentComponentCallResultWire::WorkflowCheckpointDelete => Ok(()),
            _ => Err(component_mismatch(
                AgentComponentOperationWire::WorkflowCheckpointDelete,
            )),
        }
    }

    async fn clear(&self) -> echo_agent::error::Result<()> {
        let result = self
            .invoke(AgentComponentCallInputWire::WorkflowCheckpointClear)
            .await?;
        match result {
            AgentComponentCallResultWire::WorkflowCheckpointClear => Ok(()),
            _ => Err(component_mismatch(
                AgentComponentOperationWire::WorkflowCheckpointClear,
            )),
        }
    }
}

#[async_trait::async_trait]
impl echo_agent::tasks::RevisionedTaskStore for ExtensionAgentComponentProxy {
    fn supports_runtime_execution(&self) -> bool {
        self.capabilities.execution_cas_v1
    }
    async fn load(
        &self,
        scope_id: &str,
    ) -> std::result::Result<
        Option<echo_agent::tasks::RevisionedTaskGraph>,
        echo_agent::tasks::RevisionedTaskStoreError,
    > {
        let result = self
            .invoke_task_store(AgentComponentCallInputWire::RevisionedTaskLoad {
                scope_id: scope_id.to_string(),
            })
            .await?;
        match result {
            AgentComponentCallResultWire::RevisionedTaskLoad { graph } => {
                super::wire::managed::Projection::restore(graph).map_err(|error| {
                    echo_agent::tasks::RevisionedTaskStoreError::Backend {
                        message: error.to_string(),
                    }
                })
            }
            _ => Err(echo_agent::tasks::RevisionedTaskStoreError::Backend {
                message: component_mismatch(AgentComponentOperationWire::RevisionedTaskLoad)
                    .to_string(),
            }),
        }
    }

    async fn compare_and_commit(
        &self,
        scope_id: &str,
        commit: echo_agent::tasks::TaskGraphCommit,
    ) -> std::result::Result<
        echo_agent::tasks::RevisionedTaskGraph,
        echo_agent::tasks::RevisionedTaskStoreError,
    > {
        let commit = commit.project().map_err(|error| {
            echo_agent::tasks::RevisionedTaskStoreError::Backend {
                message: error.to_string(),
            }
        })?;
        let result = self
            .invoke_task_store(
                AgentComponentCallInputWire::RevisionedTaskCompareAndCommit {
                    scope_id: scope_id.to_string(),
                    commit,
                },
            )
            .await?;
        match result {
            AgentComponentCallResultWire::RevisionedTaskCompareAndCommit { graph } => {
                super::wire::managed::Projection::restore(graph).map_err(|error| {
                    echo_agent::tasks::RevisionedTaskStoreError::Backend {
                        message: error.to_string(),
                    }
                })
            }
            _ => Err(echo_agent::tasks::RevisionedTaskStoreError::Backend {
                message: component_mismatch(
                    AgentComponentOperationWire::RevisionedTaskCompareAndCommit,
                )
                .to_string(),
            }),
        }
    }
    async fn compare_and_commit_execution(
        &self,
        scope_id: &str,
        commit: echo_agent::tasks::TaskExecutionCommit,
    ) -> Result<echo_agent::tasks::RevisionedTaskGraph, echo_agent::tasks::RevisionedTaskStoreError>
    {
        let commit = commit
            .project()
            .map_err(|message| echo_agent::tasks::RevisionedTaskStoreError::Backend { message })?;
        let result = self
            .invoke_task_store(
                AgentComponentCallInputWire::RevisionedTaskCompareAndCommitExecution {
                    scope_id: scope_id.to_string(),
                    commit,
                },
            )
            .await?;
        match result {
            AgentComponentCallResultWire::RevisionedTaskCompareAndCommitExecution { graph } => {
                super::wire::managed::Projection::restore(graph).map_err(|message| {
                    echo_agent::tasks::RevisionedTaskStoreError::Backend { message }
                })
            }
            _ => Err(echo_agent::tasks::RevisionedTaskStoreError::Backend {
                message: component_mismatch(
                    AgentComponentOperationWire::RevisionedTaskCompareAndCommitExecution,
                )
                .to_string(),
            }),
        }
    }
}

impl ExtensionAgentComponentProxy {
    pub(crate) fn execute_stream_with_cancel<'a>(
        &'a self,
        command: echo_agent::sandbox::SandboxCommand,
        cancellation: CancellationToken,
    ) -> futures::future::BoxFuture<
        'a,
        echo_agent::error::Result<
            std::pin::Pin<
                Box<
                    dyn futures::Stream<Item = echo_agent::sandbox::SandboxStreamEvent> + Send + 'a,
                >,
            >,
        >,
    > {
        Box::pin(async move {
            let invocation =
                ExtensionInvocation::AgentComponentCallStream(AgentComponentCallWire {
                    component: self.component,
                    call: AgentComponentCallInputWire::SandboxExecuteStream {
                        command: component_value(command)?,
                    },
                });
            let source = self
                .bridge
                .invoke_owned_producer_stream(
                    &self.extension,
                    session_invocation_context(&self.session_id),
                    invocation,
                    cancellation.clone(),
                    None,
                )
                .await
                .map_err(react_error)?;
            Ok(futures::stream::unfold(
                (source, cancellation, false),
                async move |(mut source, cancellation, finished)| {
                    if finished {
                        return None;
                    }
                    let event = source.next().await?;
                    let converted = match event {
                        Ok(ExtensionStreamEvent::Chunk {
                            value: ExtensionStreamChunkValue::AgentComponent(value),
                            ..
                        }) => sandbox_stream_chunk(value),
                        Ok(ExtensionStreamEvent::Complete {
                            value: ExtensionStreamCompleteValue::AgentComponent(value),
                            ..
                        }) => sandbox_stream_complete(value),
                        Ok(ExtensionStreamEvent::Cancelled { .. }) => {
                            Ok(echo_agent::sandbox::SandboxStreamEvent::Failed {
                                failure: echo_agent::sandbox::SandboxStreamFailure::Cancelled {
                                    message: "sandbox extension stream cancelled".to_string(),
                                },
                            })
                        }
                        Ok(ExtensionStreamEvent::Failed { error, .. }) | Err(error) => {
                            Err(react_error(error))
                        }
                        _ => Err(ReactError::Other(
                            "sandbox extension returned a mismatched stream value".to_string(),
                        )),
                    };
                    match converted {
                        Ok(event) => Some((event, (source, cancellation, false))),
                        Err(error) => {
                            cancellation.cancel();
                            while source.next().await.is_some() {}
                            Some((
                                echo_agent::sandbox::SandboxStreamEvent::Failed {
                                    failure: echo_agent::sandbox::SandboxStreamFailure::IoError {
                                        message: error.to_string(),
                                    },
                                },
                                (source, cancellation, true),
                            ))
                        }
                    }
                },
            )
            .boxed())
        })
    }
}

impl echo_agent::sandbox::SandboxExecutor for ExtensionAgentComponentProxy {
    fn name(&self) -> &str {
        &self.name
    }

    fn isolation_level(&self) -> echo_agent::sandbox::IsolationLevel {
        match self.capabilities.isolation_level.as_deref() {
            Some("process") => echo_agent::sandbox::IsolationLevel::Process,
            Some("os-sandbox") => echo_agent::sandbox::IsolationLevel::OsSandbox,
            Some("container") => echo_agent::sandbox::IsolationLevel::Container,
            Some("orchestrated") => echo_agent::sandbox::IsolationLevel::Orchestrated,
            _ => echo_agent::sandbox::IsolationLevel::None,
        }
    }

    fn is_available(&self) -> futures::future::BoxFuture<'_, bool> {
        Box::pin(async move {
            match self
                .invoke(AgentComponentCallInputWire::SandboxIsAvailable)
                .await
            {
                Ok(AgentComponentCallResultWire::SandboxIsAvailable { available }) => available,
                _ => false,
            }
        })
    }

    fn execute(
        &self,
        command: echo_agent::sandbox::SandboxCommand,
    ) -> futures::future::BoxFuture<
        '_,
        echo_agent::error::Result<echo_agent::sandbox::ExecutionResult>,
    > {
        Box::pin(async move {
            let result = self
                .invoke(AgentComponentCallInputWire::SandboxExecute {
                    command: component_value(command)?,
                })
                .await?;
            match result {
                AgentComponentCallResultWire::SandboxExecute { result } => component_decode(result),
                _ => Err(component_mismatch(
                    AgentComponentOperationWire::SandboxExecute,
                )),
            }
        })
    }

    fn execute_stream<'a>(
        &'a self,
        command: echo_agent::sandbox::SandboxCommand,
    ) -> futures::future::BoxFuture<
        'a,
        echo_agent::error::Result<
            std::pin::Pin<
                Box<
                    dyn futures::Stream<Item = echo_agent::sandbox::SandboxStreamEvent> + Send + 'a,
                >,
            >,
        >,
    > {
        Box::pin(async move {
            if !self.capabilities.supports_streaming {
                let result = self.execute(command).await?;
                return Ok(Box::pin(futures::stream::once(async move {
                    echo_agent::sandbox::SandboxStreamEvent::Complete(result)
                }))
                    as std::pin::Pin<Box<dyn futures::Stream<Item = _> + Send + 'a>>);
            }
            let (receiver, stream, invocation_id, cancellation, lease, sink) = self
                .bridge
                .invoke_stream(
                    &self.extension,
                    Some(session_invocation_context(&self.session_id)),
                    ExtensionInvocation::AgentComponentCallStream(AgentComponentCallWire {
                        component: self.component,
                        call: AgentComponentCallInputWire::SandboxExecuteStream {
                            command: component_value(command)?,
                        },
                    }),
                    self.bridge.connection_cancellation(),
                )
                .await
                .map_err(react_error)?;
            let events = futures::StreamExt::map(
                extension_event_stream(
                    self.bridge.clone(),
                    stream,
                    invocation_id,
                    cancellation,
                    lease,
                    sink,
                    receiver,
                ),
                |event| match event {
                    ExtensionStreamEvent::Chunk {
                        value: ExtensionStreamChunkValue::AgentComponent(value),
                        ..
                    } => sandbox_stream_chunk(value).unwrap_or_else(|error| {
                        echo_agent::sandbox::SandboxStreamEvent::Failed {
                            failure: echo_agent::sandbox::SandboxStreamFailure::IoError {
                                message: error.to_string(),
                            },
                        }
                    }),
                    ExtensionStreamEvent::Complete {
                        value: ExtensionStreamCompleteValue::AgentComponent(value),
                        ..
                    } => sandbox_stream_complete(value).unwrap_or_else(|error| {
                        echo_agent::sandbox::SandboxStreamEvent::Failed {
                            failure: echo_agent::sandbox::SandboxStreamFailure::IoError {
                                message: error.to_string(),
                            },
                        }
                    }),
                    ExtensionStreamEvent::Cancelled { .. } => {
                        echo_agent::sandbox::SandboxStreamEvent::Failed {
                            failure: echo_agent::sandbox::SandboxStreamFailure::Cancelled {
                                message: "sandbox extension stream cancelled".to_string(),
                            },
                        }
                    }
                    ExtensionStreamEvent::Failed { error, .. } => {
                        echo_agent::sandbox::SandboxStreamEvent::Failed {
                            failure: echo_agent::sandbox::SandboxStreamFailure::IoError {
                                message: error.message,
                            },
                        }
                    }
                    _ => echo_agent::sandbox::SandboxStreamEvent::Failed {
                        failure: echo_agent::sandbox::SandboxStreamFailure::IoError {
                            message: "sandbox extension returned a mismatched stream value"
                                .to_string(),
                        },
                    },
                },
            );
            Ok(Box::pin(events)
                as std::pin::Pin<
                    Box<dyn futures::Stream<Item = _> + Send + 'a>,
                >)
        })
    }

    fn execute_with_limits(
        &self,
        command: echo_agent::sandbox::SandboxCommand,
        limits: echo_agent::sandbox::ResourceLimits,
    ) -> futures::future::BoxFuture<
        '_,
        echo_agent::error::Result<echo_agent::sandbox::ExecutionResult>,
    > {
        Box::pin(async move {
            let result = self
                .invoke(AgentComponentCallInputWire::SandboxExecuteWithLimits {
                    command: component_value(command)?,
                    limits: component_value(limits)?,
                })
                .await?;
            match result {
                AgentComponentCallResultWire::SandboxExecuteWithLimits { result } => {
                    component_decode(result)
                }
                _ => Err(component_mismatch(
                    AgentComponentOperationWire::SandboxExecuteWithLimits,
                )),
            }
        })
    }

    fn execute_with_limits_and_cancel(
        &self,
        command: echo_agent::sandbox::SandboxCommand,
        limits: echo_agent::sandbox::ResourceLimits,
        cancel: Option<Arc<CancellationToken>>,
    ) -> futures::future::BoxFuture<
        '_,
        echo_agent::error::Result<echo_agent::sandbox::ExecutionResult>,
    > {
        Box::pin(async move {
            let cancellation = cancel
                .as_deref()
                .cloned()
                .unwrap_or_else(|| self.bridge.connection_cancellation());
            let result = self
                .invoke_with_cancellation(
                    AgentComponentCallInputWire::SandboxExecuteWithLimitsAndCancel {
                        command: component_value(command)?,
                        limits: component_value(limits)?,
                    },
                    cancellation.clone(),
                )
                .await;
            if cancellation.is_cancelled() {
                // Cleanup settles local resources after Run cancellation;
                // inheriting that already-cancelled token would drop its
                // response before the SDK acknowledges the actual cleanup.
                let cleanup = self
                    .invoke_connection_scoped(AgentComponentCallInputWire::SandboxCleanup)
                    .await;
                if !matches!(cleanup, Ok(AgentComponentCallResultWire::SandboxCleanup)) {
                    return Err(ReactError::Sandbox(Box::new(
                        echo_agent::error::SandboxError::IoError(
                            "sandbox extension cancellation cleanup failed".to_string(),
                        ),
                    )));
                }
                return Err(ReactError::Sandbox(Box::new(
                    echo_agent::error::SandboxError::Cancelled(
                        "owning run cancelled sandbox extension execution".to_string(),
                    ),
                )));
            }
            match result? {
                AgentComponentCallResultWire::SandboxExecuteWithLimitsAndCancel { result } => {
                    component_decode(result)
                }
                _ => Err(component_mismatch(
                    AgentComponentOperationWire::SandboxExecuteWithLimitsAndCancel,
                )),
            }
        })
    }

    fn supports_streaming(&self) -> bool {
        self.capabilities.supports_streaming
    }

    fn cleanup(&self) -> futures::future::BoxFuture<'_, echo_agent::error::Result<()>> {
        Box::pin(async move {
            let result = self
                .invoke_connection_scoped(AgentComponentCallInputWire::SandboxCleanup)
                .await?;
            match result {
                AgentComponentCallResultWire::SandboxCleanup => Ok(()),
                _ => Err(component_mismatch(
                    AgentComponentOperationWire::SandboxCleanup,
                )),
            }
        })
    }
}

#[cfg(feature = "framework-mcp")]
impl echo_agent::mcp::integration::transport::McpTransport for ExtensionAgentComponentProxy {
    fn send(
        &self,
        request: echo_agent::mcp::integration::types::JsonRpcRequest,
    ) -> futures::future::BoxFuture<
        '_,
        echo_agent::error::Result<echo_agent::mcp::integration::types::JsonRpcResponse>,
    > {
        Box::pin(async move {
            let result = self
                .invoke(AgentComponentCallInputWire::McpTransportSend {
                    request: component_value(request)?,
                })
                .await?;
            match result {
                AgentComponentCallResultWire::McpTransportSend { response } => {
                    component_decode(response)
                }
                _ => Err(component_mismatch(
                    AgentComponentOperationWire::McpTransportSend,
                )),
            }
        })
    }

    fn notify(
        &self,
        notification: echo_agent::mcp::integration::types::JsonRpcNotification,
    ) -> futures::future::BoxFuture<'_, echo_agent::error::Result<()>> {
        Box::pin(async move {
            let result = self
                .invoke(AgentComponentCallInputWire::McpTransportNotify {
                    notification: component_value(notification)?,
                })
                .await?;
            match result {
                AgentComponentCallResultWire::McpTransportNotify => Ok(()),
                _ => Err(component_mismatch(
                    AgentComponentOperationWire::McpTransportNotify,
                )),
            }
        })
    }

    fn close(&self) -> futures::future::BoxFuture<'_, echo_agent::error::Result<()>> {
        Box::pin(async move {
            self.notification_cancel.cancel();
            match self
                .invoke_connection_scoped(AgentComponentCallInputWire::McpTransportClose)
                .await?
            {
                AgentComponentCallResultWire::McpTransportClose => Ok(()),
                _ => Err(component_mismatch(
                    AgentComponentOperationWire::McpTransportClose,
                )),
            }
        })
    }

    fn notification_rx(
        &self,
    ) -> Option<Arc<dyn echo_agent::mcp::integration::types::JsonRpcNotificationReceiver>> {
        self.capabilities.supports_notifications.then(|| {
            self.start_notification_poll();
            Arc::new(self.clone())
                as Arc<dyn echo_agent::mcp::integration::types::JsonRpcNotificationReceiver>
        })
    }
}

#[cfg(feature = "framework-mcp")]
impl echo_agent::mcp::integration::types::JsonRpcNotificationReceiver
    for ExtensionAgentComponentProxy
{
    fn try_recv(&self) -> Option<echo_agent::mcp::integration::types::JsonRpcNotification> {
        self.notification_queue
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .pop_front()
    }
}

impl echo_agent::memory::Embedder for ExtensionAgentComponentProxy {
    fn embed<'a>(
        &'a self,
        text: &'a str,
    ) -> futures::future::BoxFuture<'a, echo_agent::error::Result<Vec<f32>>> {
        let text = text.to_string();
        Box::pin(async move {
            let result = self
                .invoke(AgentComponentCallInputWire::EmbedderEmbed { text })
                .await?;
            match result {
                AgentComponentCallResultWire::EmbedderEmbed { vector } => Ok(vector),
                _ => Err(component_mismatch(
                    AgentComponentOperationWire::EmbedderEmbed,
                )),
            }
        })
    }
}

impl echo_agent::compression::MemoryPromoter for ExtensionAgentComponentProxy {
    fn promote(
        &self,
        evicted: &[echo_agent::llm::types::Message],
    ) -> futures::future::BoxFuture<
        '_,
        echo_agent::error::Result<echo_agent::compression::MemoryPromotionReceipt>,
    > {
        let evicted = evicted.to_vec();
        Box::pin(async move {
            let evicted = evicted
                .iter()
                .map(message_wire)
                .collect::<std::result::Result<Vec<_>, _>>()
                .map_err(ReactError::Other)?;
            let result = self
                .invoke(AgentComponentCallInputWire::MemoryPromoterPromote { evicted })
                .await?;
            match result {
                AgentComponentCallResultWire::MemoryPromoterPromote {
                    submitted,
                    promoted,
                    deduplicated,
                } => Ok(echo_agent::compression::MemoryPromotionReceipt {
                    submitted: wire_usize(submitted, "submitted").map_err(ReactError::Other)?,
                    promoted: wire_usize(promoted, "promoted").map_err(ReactError::Other)?,
                    deduplicated: wire_usize(deduplicated, "deduplicated")
                        .map_err(ReactError::Other)?,
                }),
                _ => Err(component_mismatch(
                    AgentComponentOperationWire::MemoryPromoterPromote,
                )),
            }
        })
    }
}

impl echo_agent::workflow::Workflow for ExtensionAgentComponentProxy {
    fn run<'a>(
        &'a mut self,
        input: &'a str,
    ) -> futures::future::BoxFuture<
        'a,
        echo_agent::error::Result<echo_agent::workflow::WorkflowOutput>,
    > {
        let input = input.to_string();
        Box::pin(async move {
            let result = self
                .invoke(AgentComponentCallInputWire::WorkflowRun { input })
                .await?;
            match result {
                AgentComponentCallResultWire::WorkflowRun { output } => component_decode(output),
                _ => Err(component_mismatch(AgentComponentOperationWire::WorkflowRun)),
            }
        })
    }

    fn run_stream<'a>(
        &'a mut self,
        input: &'a str,
    ) -> futures::future::BoxFuture<
        'a,
        echo_agent::error::Result<
            futures::stream::BoxStream<
                'a,
                echo_agent::error::Result<echo_agent::workflow::WorkflowEvent>,
            >,
        >,
    > {
        let input = input.to_string();
        Box::pin(async move {
            if !self.capabilities.supports_streaming {
                let output = self.run(&input).await?;
                let event = echo_agent::workflow::WorkflowEvent::Completed {
                    result: output.result,
                    total_steps: output.steps.len(),
                    elapsed: output.elapsed,
                };
                return Ok(futures::StreamExt::boxed(futures::stream::once(
                    async move { Ok(event) },
                )));
            }
            let (receiver, stream, invocation_id, cancellation, lease, sink) = self
                .bridge
                .invoke_stream(
                    &self.extension,
                    Some(session_invocation_context(&self.session_id)),
                    ExtensionInvocation::AgentComponentCallStream(AgentComponentCallWire {
                        component: self.component,
                        call: AgentComponentCallInputWire::WorkflowRunStream { input },
                    }),
                    self.bridge.connection_cancellation(),
                )
                .await
                .map_err(react_error)?;
            Ok(futures::StreamExt::boxed(futures::StreamExt::map(
                extension_event_stream(
                    self.bridge.clone(),
                    stream,
                    invocation_id,
                    cancellation,
                    lease,
                    sink,
                    receiver,
                ),
                |event| match event {
                    ExtensionStreamEvent::Chunk {
                        value: ExtensionStreamChunkValue::AgentComponent(value),
                        ..
                    } => workflow_stream_chunk(value),
                    ExtensionStreamEvent::Complete {
                        value: ExtensionStreamCompleteValue::AgentComponent(value),
                        ..
                    } => workflow_stream_complete(value),
                    ExtensionStreamEvent::Cancelled { .. } => Err(ReactError::Other(
                        "workflow extension stream cancelled".to_string(),
                    )),
                    ExtensionStreamEvent::Failed { error, .. } => {
                        Err(ReactError::Other(error.message))
                    }
                    _ => Err(ReactError::Other(
                        "workflow extension returned a mismatched stream value".to_string(),
                    )),
                },
            )))
        })
    }
}

impl echo_agent::intent::IntentClassifier for ExtensionAgentComponentProxy {
    fn classify<'a>(
        &'a self,
        user_input: &'a str,
        context: &'a [echo_agent::llm::types::Message],
    ) -> futures::future::BoxFuture<'a, echo_agent::intent::Intent> {
        let user_input = user_input.to_string();
        let context = context
            .iter()
            .map(message_wire)
            .collect::<std::result::Result<Vec<_>, _>>();
        Box::pin(async move {
            let Ok(context) = context else {
                return echo_agent::intent::Intent::Fallback;
            };
            match self
                .invoke(AgentComponentCallInputWire::IntentClassify {
                    user_input,
                    context,
                })
                .await
            {
                Ok(AgentComponentCallResultWire::IntentClassify { intent }) => {
                    component_decode(intent).unwrap_or(echo_agent::intent::Intent::Fallback)
                }
                _ => echo_agent::intent::Intent::Fallback,
            }
        })
    }
}

impl echo_agent::skills::external::SkillLoadPolicy for ExtensionAgentComponentProxy {
    fn allows<'a>(
        &'a self,
        descriptor: &'a echo_agent::skills::external::SkillDescriptor,
    ) -> futures::future::BoxFuture<'a, bool> {
        let descriptor = skill_descriptor_policy_wire(descriptor);
        Box::pin(async move {
            let Ok(descriptor) = descriptor else {
                return false;
            };
            match self
                .invoke(AgentComponentCallInputWire::SkillLoadAllows {
                    descriptor: Box::new(descriptor),
                })
                .await
            {
                Ok(AgentComponentCallResultWire::SkillLoadAllows { allowed }) => allowed,
                _ => false,
            }
        })
    }
}

// ── Store proxy ─────────────────────────────────────────────────────────────

fn store_item_from_wire(
    item: StoreItemWire,
) -> std::result::Result<echo_agent::memory::StoreItem, String> {
    Ok(echo_agent::memory::StoreItem {
        namespace: item.namespace,
        key: item.key,
        value: item.value.into_json().map_err(|error| error.to_string())?,
        created_at: item
            .created_at
            .to_u64()
            .ok_or_else(|| "invalid created_at".to_string())?,
        updated_at: item
            .updated_at
            .to_u64()
            .ok_or_else(|| "invalid updated_at".to_string())?,
        score: item.score,
        importance: item.importance,
        last_accessed: item.last_accessed.and_then(|value| value.to_u64()),
        expires_at: item.expires_at.and_then(|value| value.to_u64()),
    })
}

/// Thin `Store` proxy over the six store operations. Semantic or hybrid
/// searches against an implementation that did not declare them fail before
/// any callback leaves the process.
pub(crate) struct ExtensionStoreProxy {
    bridge: Arc<ExtensionBridge>,
    extension: WireHandle,
    declared_modes: Vec<echo_sdk_protocol::methods::SearchModeWire>,
    session_id: String,
}

impl ExtensionStoreProxy {
    pub(crate) fn new(
        bridge: Arc<ExtensionBridge>,
        extension: WireHandle,
        session_id: String,
    ) -> Option<Self> {
        let record = bridge.state().ok()?.handles.extension(&extension).ok()?;
        let ExtensionDescriptor::Store { search_modes, .. } = &record.descriptor else {
            return None;
        };
        Some(Self {
            bridge,
            extension,
            declared_modes: search_modes.clone(),
            session_id,
        })
    }

    fn declares(&self, mode: echo_sdk_protocol::methods::SearchModeWire) -> bool {
        (self.declared_modes.is_empty()
            && mode == echo_sdk_protocol::methods::SearchModeWire::Keyword)
            || self.declared_modes.contains(&mode)
    }

    async fn call_op(
        &self,
        invocation: ExtensionInvocation,
    ) -> echo_agent::error::Result<ExtensionResult> {
        self.bridge
            .invoke_once(
                &self.extension,
                Some(session_invocation_context(&self.session_id)),
                invocation,
                self.bridge.connection_cancellation(),
            )
            .await
            .map_err(react_error)
    }
}

impl echo_agent::memory::Store for ExtensionStoreProxy {
    fn put<'a>(
        &'a self,
        namespace: &'a [&'a str],
        key: &'a str,
        value: serde_json::Value,
    ) -> futures::future::BoxFuture<'a, echo_agent::error::Result<()>> {
        Box::pin(async move {
            let result = self
                .call_op(ExtensionInvocation::StorePut(StorePutInput {
                    namespace: namespace.iter().map(|part| (*part).to_string()).collect(),
                    key: key.to_string(),
                    value: to_wire(value).map_err(ReactError::Other)?,
                }))
                .await?;
            matches!(result, ExtensionResult::StorePut(_))
                .then_some(())
                .ok_or_else(|| {
                    ReactError::Other("store put returned the wrong result variant".to_string())
                })
        })
    }

    fn get<'a>(
        &'a self,
        namespace: &'a [&'a str],
        key: &'a str,
    ) -> futures::future::BoxFuture<
        'a,
        echo_agent::error::Result<Option<echo_agent::memory::StoreItem>>,
    > {
        Box::pin(async move {
            let result = self
                .call_op(ExtensionInvocation::StoreGet(StoreKeyInput {
                    namespace: namespace.iter().map(|part| (*part).to_string()).collect(),
                    key: key.to_string(),
                }))
                .await?;
            let ExtensionResult::StoreGet(item) = result else {
                return Err(ReactError::Other(
                    "store get returned the wrong result variant".to_string(),
                ));
            };
            item.map(store_item_from_wire)
                .transpose()
                .map_err(ReactError::Other)
        })
    }

    fn search<'a>(
        &'a self,
        namespace: &'a [&'a str],
        query: &'a str,
        limit: usize,
    ) -> futures::future::BoxFuture<'a, echo_agent::error::Result<Vec<echo_agent::memory::StoreItem>>>
    {
        Box::pin(async move {
            let result = self
                .call_op(ExtensionInvocation::StoreSearch(StoreSearchInput {
                    namespace: namespace.iter().map(|part| (*part).to_string()).collect(),
                    query: query.to_string(),
                    limit: WireU64::from_u64(u64::try_from(limit).unwrap_or(u64::MAX)),
                }))
                .await?;
            let ExtensionResult::StoreSearch(items) = result else {
                return Err(ReactError::Other(
                    "store search returned the wrong result variant".to_string(),
                ));
            };
            items
                .into_iter()
                .map(store_item_from_wire)
                .collect::<std::result::Result<Vec<_>, _>>()
                .map_err(ReactError::Other)
        })
    }

    fn search_with<'a>(
        &'a self,
        namespace: &'a [&'a str],
        query: echo_agent::memory::SearchQuery<'a>,
    ) -> futures::future::BoxFuture<'a, echo_agent::error::Result<Vec<echo_agent::memory::StoreItem>>>
    {
        Box::pin(async move {
            let (mode, wire_mode) = match &query.mode {
                echo_agent::memory::SearchMode::Keyword => (
                    echo_sdk_protocol::methods::SearchModeWire::Keyword,
                    StoreSearchModeWire::Keyword,
                ),
                echo_agent::memory::SearchMode::Semantic => (
                    echo_sdk_protocol::methods::SearchModeWire::Semantic,
                    StoreSearchModeWire::Semantic,
                ),
                echo_agent::memory::SearchMode::Hybrid { vector_weight } => (
                    echo_sdk_protocol::methods::SearchModeWire::Hybrid,
                    StoreSearchModeWire::Hybrid {
                        vector_weight: Some(*vector_weight),
                    },
                ),
            };
            if !self.declares(mode) {
                return Err(ReactError::Other(format!(
                    "store extension does not declare {:?} search",
                    mode
                )));
            }
            let payload = StoreSearchQueryWire {
                text: query.text.to_string(),
                limit: WireU64::from_u64(u64::try_from(query.limit).unwrap_or(u64::MAX)),
                mode: wire_mode,
            };
            let result = self
                .call_op(ExtensionInvocation::StoreSearchWith(StoreSearchWithInput {
                    namespace: namespace.iter().map(|part| (*part).to_string()).collect(),
                    query: payload,
                }))
                .await?;
            let ExtensionResult::StoreSearchWith(items) = result else {
                return Err(ReactError::Other(
                    "store search_with returned the wrong result variant".to_string(),
                ));
            };
            items
                .into_iter()
                .map(store_item_from_wire)
                .collect::<std::result::Result<Vec<_>, _>>()
                .map_err(ReactError::Other)
        })
    }

    fn delete<'a>(
        &'a self,
        namespace: &'a [&'a str],
        key: &'a str,
    ) -> futures::future::BoxFuture<'a, echo_agent::error::Result<bool>> {
        Box::pin(async move {
            let result = self
                .call_op(ExtensionInvocation::StoreDelete(StoreKeyInput {
                    namespace: namespace.iter().map(|part| (*part).to_string()).collect(),
                    key: key.to_string(),
                }))
                .await?;
            match result {
                ExtensionResult::StoreDelete(deleted) => Ok(deleted),
                _ => Err(ReactError::Other(
                    "store delete returned the wrong result variant".to_string(),
                )),
            }
        })
    }

    fn list_namespaces<'a>(
        &'a self,
        prefix: Option<&'a [&'a str]>,
    ) -> futures::future::BoxFuture<'a, echo_agent::error::Result<Vec<Vec<String>>>> {
        Box::pin(async move {
            let result = self
                .call_op(ExtensionInvocation::StoreListNamespaces(
                    StoreListNamespacesInput {
                        prefix: prefix
                            .map(|parts| parts.iter().map(|part| (*part).to_string()).collect()),
                    },
                ))
                .await?;
            match result {
                ExtensionResult::StoreListNamespaces(namespaces) => Ok(namespaces),
                _ => Err(ReactError::Other(
                    "store list_namespaces returned the wrong result variant".to_string(),
                )),
            }
        })
    }

    fn list<'a>(
        &'a self,
        namespace: &'a [&'a str],
    ) -> futures::future::BoxFuture<'a, echo_agent::error::Result<Vec<echo_agent::memory::StoreItem>>>
    {
        Box::pin(async move {
            let result = self
                .call_op(ExtensionInvocation::StoreList(StoreNamespaceInput {
                    namespace: namespace.iter().map(|part| (*part).to_string()).collect(),
                }))
                .await?;
            let ExtensionResult::StoreList(items) = result else {
                return Err(ReactError::Other(
                    "store list returned the wrong result variant".to_string(),
                ));
            };
            items
                .into_iter()
                .map(store_item_from_wire)
                .collect::<std::result::Result<Vec<_>, _>>()
                .map_err(ReactError::Other)
        })
    }

    fn prune_expired<'a>(
        &'a self,
        namespace: &'a [&'a str],
    ) -> futures::future::BoxFuture<'a, echo_agent::error::Result<u64>> {
        Box::pin(async move {
            let result = self
                .call_op(ExtensionInvocation::StorePruneExpired(
                    StoreNamespaceInput {
                        namespace: namespace.iter().map(|part| (*part).to_string()).collect(),
                    },
                ))
                .await?;
            match result {
                ExtensionResult::StorePruneExpired(count) => count
                    .to_u64()
                    .ok_or_else(|| ReactError::Other("invalid prune count".to_string())),
                _ => Err(ReactError::Other(
                    "store prune returned the wrong result variant".to_string(),
                )),
            }
        })
    }

    fn dedup_by_content<'a>(
        &'a self,
        namespace: &'a [&'a str],
    ) -> futures::future::BoxFuture<'a, echo_agent::error::Result<u64>> {
        Box::pin(async move {
            let result = self
                .call_op(ExtensionInvocation::StoreDedupByContent(
                    StoreNamespaceInput {
                        namespace: namespace.iter().map(|part| (*part).to_string()).collect(),
                    },
                ))
                .await?;
            match result {
                ExtensionResult::StoreDedupByContent(count) => count
                    .to_u64()
                    .ok_or_else(|| ReactError::Other("invalid dedup count".to_string())),
                _ => Err(ReactError::Other(
                    "store dedup returned the wrong result variant".to_string(),
                )),
            }
        })
    }
}

// ── Critic proxy ─────────────────────────────────────────────────────────────

fn critique_from_wire(
    wire: CritiqueWire,
) -> std::result::Result<echo_agent::agent::critic::Critique, String> {
    wire.validate()?;
    Ok(echo_agent::agent::critic::Critique {
        score: wire.score,
        passed: wire.passed,
        feedback: wire.feedback,
        suggestions: wire.suggestions,
    })
}

/// Thin `Critic` proxy. Registration only supplies the implementation; the
/// framework's verifier remains disabled unless its own configuration enables
/// it, so this bridge never changes verification policy implicitly.
pub(crate) struct ExtensionCriticProxy {
    bridge: Arc<ExtensionBridge>,
    extension: WireHandle,
    name: String,
    session_id: String,
}

impl ExtensionCriticProxy {
    pub(crate) fn new(
        bridge: Arc<ExtensionBridge>,
        extension: WireHandle,
        session_id: String,
    ) -> Option<Self> {
        let record = bridge.state().ok()?.handles.extension(&extension).ok()?;
        let ExtensionDescriptor::Critic { name, .. } = &record.descriptor else {
            return None;
        };
        Some(Self {
            bridge,
            extension,
            name: name.clone(),
            session_id,
        })
    }
}

impl echo_agent::agent::critic::Critic for ExtensionCriticProxy {
    fn critique<'a>(
        &'a self,
        task: &'a str,
        answer: &'a str,
        context: &'a str,
    ) -> futures::future::BoxFuture<
        'a,
        echo_agent::error::Result<echo_agent::agent::critic::Critique>,
    > {
        Box::pin(async move {
            let input = CritiqueInput {
                task: task.to_string(),
                answer: answer.to_string(),
                context: context.to_string(),
            };
            input
                .validate()
                .map_err(|error| ReactError::Other(error.to_string()))?;
            let value = self
                .bridge
                .invoke_once(
                    &self.extension,
                    Some(session_invocation_context(&self.session_id)),
                    ExtensionInvocation::CriticCritique(input),
                    self.bridge.connection_cancellation(),
                )
                .await
                .map_err(react_error)?;
            let ExtensionResult::CriticCritique(critique) = value else {
                return Err(ReactError::Other(
                    "critic extension returned the wrong result variant".to_string(),
                ));
            };
            critique_from_wire(critique).map_err(ReactError::Other)
        })
    }

    fn name(&self) -> &str {
        &self.name
    }
}

// ── HumanLoopProvider proxy ─────────────────────────────────────────────────

fn scope_from_wire(scope: ApprovalScopeWire) -> echo_agent::human_loop::ApprovalScope {
    match scope {
        ApprovalScopeWire::Once => echo_agent::human_loop::ApprovalScope::Once,
        ApprovalScopeWire::Session => echo_agent::human_loop::ApprovalScope::Session,
        ApprovalScopeWire::SessionTool => echo_agent::human_loop::ApprovalScope::SessionTool,
    }
}

fn human_response_from_wire(
    wire: HumanLoopResponseWire,
) -> std::result::Result<echo_agent::human_loop::HumanLoopResponse, String> {
    use echo_agent::human_loop::HumanLoopResponse as Framework;
    Ok(match wire {
        HumanLoopResponseWire::Approved => Framework::Approved,
        HumanLoopResponseWire::ApprovedWithScope { scope } => Framework::ApprovedWithScope {
            scope: scope_from_wire(scope),
        },
        HumanLoopResponseWire::ModifiedArgs { args, scope } => Framework::ModifiedArgs {
            args: args.into_json().map_err(|error| error.to_string())?,
            scope: scope_from_wire(scope),
        },
        HumanLoopResponseWire::Rejected { reason } => Framework::Rejected { reason },
        HumanLoopResponseWire::Text { text } => Framework::Text(text),
        HumanLoopResponseWire::Timeout => Framework::Timeout,
        HumanLoopResponseWire::Deferred => Framework::Deferred,
        HumanLoopResponseWire::Selection {
            selection,
            instructions,
        } => Framework::Selection {
            selection,
            instructions,
        },
    })
}

/// Thin `HumanLoopProvider` proxy: request identity is preserved, the
/// response settles exactly once per invocation, and disconnect or timeout
/// becomes an explicit framework error instead of a fallback provider.
pub(crate) struct ExtensionHumanLoopProxy {
    bridge: Arc<ExtensionBridge>,
    extension: WireHandle,
    session_id: String,
}

impl ExtensionHumanLoopProxy {
    pub(crate) fn new(
        bridge: Arc<ExtensionBridge>,
        extension: WireHandle,
        session_id: String,
    ) -> Option<Self> {
        let record = bridge.state().ok()?.handles.extension(&extension).ok()?;
        matches!(
            record.descriptor,
            ExtensionDescriptor::HumanLoopProvider { .. }
        )
        .then(|| Self {
            bridge,
            extension,
            session_id,
        })
    }
}

impl echo_agent::human_loop::HumanLoopProvider for ExtensionHumanLoopProxy {
    fn request(
        &self,
        request: echo_agent::human_loop::HumanLoopRequest,
    ) -> futures::future::BoxFuture<
        '_,
        echo_agent::error::Result<echo_agent::human_loop::HumanLoopResponse>,
    > {
        Box::pin(async move {
            let payload = HumanLoopRequestWire {
                request_id: request.request_id.clone(),
                session_id: request.session_id.clone(),
                agent_name: request.agent_name.clone(),
                kind: match request.kind {
                    echo_agent::human_loop::HumanLoopKind::Approval => HumanLoopKindWire::Approval,
                    echo_agent::human_loop::HumanLoopKind::Input => HumanLoopKindWire::Input,
                    echo_agent::human_loop::HumanLoopKind::Selection => {
                        HumanLoopKindWire::Selection
                    }
                },
                prompt: request.prompt.clone(),
                tool_name: request.tool_name.clone(),
                args: request
                    .args
                    .as_ref()
                    .map(to_wire)
                    .transpose()
                    .map_err(ReactError::Other)?,
                risk_level: request.risk_level.map(|risk| match risk {
                    echo_agent::human_loop::RiskLevel::Low => HumanRiskLevelWire::Low,
                    echo_agent::human_loop::RiskLevel::Medium => HumanRiskLevelWire::Medium,
                    echo_agent::human_loop::RiskLevel::High => HumanRiskLevelWire::High,
                    echo_agent::human_loop::RiskLevel::Critical => HumanRiskLevelWire::Critical,
                }),
                approval_context: request
                    .approval_context
                    .as_ref()
                    .map(to_wire)
                    .transpose()
                    .map_err(ReactError::Other)?,
                suggestions: request
                    .suggestions
                    .iter()
                    .map(to_wire)
                    .collect::<std::result::Result<Vec<_>, _>>()
                    .map_err(ReactError::Other)?,
                timeout: request.timeout.map(wire_duration),
                task_id: request.task_id.clone(),
                options: request.options.clone(),
                context: request
                    .context
                    .as_ref()
                    .map(to_wire)
                    .transpose()
                    .map_err(ReactError::Other)?,
                phase: request.phase.clone(),
            };
            let value = self
                .bridge
                .invoke_once(
                    &self.extension,
                    Some(session_invocation_context(&self.session_id)),
                    ExtensionInvocation::HumanLoopRequest(payload),
                    self.bridge.connection_cancellation(),
                )
                .await
                .map_err(react_error)?;
            let ExtensionResult::HumanLoopRequest(response) = value else {
                return Err(ReactError::Other(
                    "human-loop extension returned the wrong result variant".to_string(),
                ));
            };
            human_response_from_wire(response).map_err(ReactError::Other)
        })
    }
}

// ── Hook bridge ─────────────────────────────────────────────────────────────

fn hook_result_from_wire(
    wire: HookResultWire,
) -> std::result::Result<echo_agent::hooks::HookResult, String> {
    let permission_mode_override = wire
        .permission_mode_override
        .map(|mode| {
            serde_json::from_value(serde_json::Value::String(mode))
                .map_err(|error| format!("invalid permission mode override: {error}"))
        })
        .transpose()?;
    Ok(echo_agent::hooks::HookResult {
        block: wire.block,
        block_reason: wire.block_reason,
        updated_input: wire
            .updated_input
            .map(|value| value.into_json().map_err(|error| error.to_string()))
            .transpose()?,
        messages: wire.messages,
        stop_propagation: wire.stop_propagation,
        permission_decision: wire.permission_decision.map(|decision| match decision {
            PermissionDecisionWire::Allow => {
                echo_agent::tools::permission::PermissionDecision::Allow
            }
            PermissionDecisionWire::Deny { reason } => {
                echo_agent::tools::permission::PermissionDecision::Deny { reason }
            }
            PermissionDecisionWire::RequireApproval => {
                echo_agent::tools::permission::PermissionDecision::RequireApproval
            }
            PermissionDecisionWire::Ask { suggestions } => {
                echo_agent::tools::permission::PermissionDecision::Ask { suggestions }
            }
        }),
        permission_mode_override,
        continue_reason: wire.continue_reason,
        injected_context: wire.injected_context,
        retry: wire.retry,
        metadata: wire
            .metadata
            .map(|value| value.into_json().map_err(|error| error.to_string()))
            .transpose()?,
        activate_skill: wire
            .activate_skill
            .map(|ActivateSkillWire { name, reason }| (name, reason)),
    })
}

/// Build the framework programmatic-hook executor for one Hook extension.
/// The closure preserves hook semantics: the wire result maps back to
/// `HookResult` (block/mutation/propagation) unchanged.
pub(crate) fn hook_executor(
    bridge: Arc<ExtensionBridge>,
    extension: WireHandle,
) -> echo_agent::skills::hooks::ProgrammaticHookFn {
    Arc::new(move |context: echo_agent::hooks::HookContext| {
        let bridge = bridge.clone();
        let extension = extension.clone();
        let context_identity = ExtensionInvocationContext {
            session_id: Some(context.session_id.clone()),
            run_id: context.run_id.clone(),
            stream_id: None,
            turn_id: None,
            message_id: None,
            execution_id: None,
            call_id: None,
        };
        Box::pin(async move {
            let input = match to_wire(&context) {
                Ok(context) => HookRunInput { context },
                Err(error) => {
                    tracing::warn!("hook context could not be projected: {error}");
                    return echo_agent::hooks::HookResult::deny(format!(
                        "extension hook context projection failed: {error}"
                    ));
                }
            };
            match bridge
                .invoke_once(
                    &extension,
                    Some(context_identity),
                    ExtensionInvocation::HookRun(input),
                    bridge.connection_cancellation(),
                )
                .await
            {
                Ok(ExtensionResult::HookRun(value)) => {
                    hook_result_from_wire(value).unwrap_or_else(|error| {
                        echo_agent::hooks::HookResult::deny(format!(
                            "extension hook returned an invalid result: {error}"
                        ))
                    })
                }
                Ok(_) => echo_agent::hooks::HookResult::deny(
                    "extension hook returned the wrong result variant".to_string(),
                ),
                Err(error) => {
                    tracing::warn!("hook extension {} failed: {}", extension.id, error.message);
                    echo_agent::hooks::HookResult::deny(format!(
                        "extension hook failed: {}",
                        error.message
                    ))
                }
            }
        })
    })
}

// ── AgentCallback proxy ─────────────────────────────────────────────────────

/// Observational callback proxy. Callback failures are logged with bounded
/// diagnostics and never fail the run — matching the Rust trait contract
/// (`BoxFuture<'a, ()>` cannot propagate errors).
pub(crate) struct ExtensionAgentCallbackProxy {
    bridge: Arc<ExtensionBridge>,
    extension: WireHandle,
    session_id: String,
}

impl ExtensionAgentCallbackProxy {
    pub(crate) fn new(
        bridge: Arc<ExtensionBridge>,
        extension: WireHandle,
        session_id: String,
    ) -> Option<Self> {
        let record = bridge.state().ok()?.handles.extension(&extension).ok()?;
        matches!(record.descriptor, ExtensionDescriptor::AgentCallback { .. }).then(|| Self {
            bridge,
            extension,
            session_id,
        })
    }

    fn observe(
        &self,
        invocation: std::result::Result<ExtensionInvocation, String>,
    ) -> futures::future::BoxFuture<'_, ()> {
        self.observe_scoped(invocation, true)
    }

    fn observe_connection_scoped(
        &self,
        invocation: Result<ExtensionInvocation, String>,
    ) -> futures::future::BoxFuture<'_, ()> {
        self.observe_scoped(invocation, false)
    }

    fn observe_scoped(
        &self,
        invocation: Result<ExtensionInvocation, String>,
        resolve_run_cancellation: bool,
    ) -> futures::future::BoxFuture<'_, ()> {
        let bridge = self.bridge.clone();
        let extension = self.extension.clone();
        let session_id = self.session_id.clone();
        Box::pin(async move {
            let invocation = match invocation {
                Ok(invocation) => invocation,
                Err(error) => {
                    tracing::warn!("agent callback input projection failed: {error}");
                    return;
                }
            };
            if let Err(error) = bridge
                .invoke_once_with_scope(
                    &extension,
                    Some(session_invocation_context(&session_id)),
                    invocation,
                    ReverseInvocationScope {
                        cancellation: bridge.connection_cancellation(),
                        resolve_run_cancellation,
                        submitted: None,
                    },
                )
                .await
            {
                tracing::warn!(
                    "agent callback extension {} failed: {}",
                    extension.id,
                    error.message
                );
            }
        })
    }
}

impl echo_agent::agent::AgentCallback for ExtensionAgentCallbackProxy {
    fn callback_kind(&self) -> Option<&'static str> {
        Some("sdk_extension")
    }

    fn callback_id(&self) -> Option<&str> {
        Some(&self.extension.id)
    }

    fn on_think_start<'a>(
        &'a self,
        agent: &'a str,
        messages: &'a [echo_agent::llm::types::Message],
    ) -> futures::future::BoxFuture<'a, ()> {
        self.observe(
            messages
                .iter()
                .map(message_wire)
                .collect::<std::result::Result<Vec<_>, _>>()
                .map(|messages| {
                    ExtensionInvocation::CallbackOnThinkStart(CallbackThinkStartInput {
                        agent: agent.to_string(),
                        messages,
                    })
                }),
        )
    }

    fn on_think_end<'a>(
        &'a self,
        agent: &'a str,
        steps: &'a [echo_agent::agent::StepType],
        prompt_tokens: usize,
        completion_tokens: usize,
    ) -> futures::future::BoxFuture<'a, ()> {
        self.observe(
            steps
                .iter()
                .map(step_type_wire)
                .collect::<std::result::Result<Vec<_>, _>>()
                .map(|steps| {
                    ExtensionInvocation::CallbackOnThinkEnd(CallbackThinkEndInput {
                        agent: agent.to_string(),
                        steps,
                        prompt_tokens: WireU64::from_u64(
                            u64::try_from(prompt_tokens).unwrap_or(u64::MAX),
                        ),
                        completion_tokens: WireU64::from_u64(
                            u64::try_from(completion_tokens).unwrap_or(u64::MAX),
                        ),
                    })
                }),
        )
    }

    fn on_tool_start<'a>(
        &'a self,
        agent: &'a str,
        tool: &'a str,
        args: &'a serde_json::Value,
    ) -> futures::future::BoxFuture<'a, ()> {
        self.observe(to_wire(args).map(|args| {
            ExtensionInvocation::CallbackOnToolStart(CallbackToolStartInput {
                agent: agent.to_string(),
                call_id: None,
                tool: tool.to_string(),
                args,
            })
        }))
    }

    fn on_tool_end<'a>(
        &'a self,
        agent: &'a str,
        tool: &'a str,
        result: &'a str,
    ) -> futures::future::BoxFuture<'a, ()> {
        self.observe_connection_scoped(Ok(ExtensionInvocation::CallbackOnToolEnd(
            CallbackToolEndInput {
                agent: agent.to_string(),
                call_id: None,
                tool: tool.to_string(),
                result: result.to_string(),
            },
        )))
    }

    fn on_tool_error<'a>(
        &'a self,
        agent: &'a str,
        tool: &'a str,
        error: &'a ReactError,
    ) -> futures::future::BoxFuture<'a, ()> {
        self.observe_connection_scoped(Ok(ExtensionInvocation::CallbackOnToolError(
            CallbackToolErrorInput {
                agent: agent.to_string(),
                call_id: None,
                tool: tool.to_string(),
                error: error.to_string(),
            },
        )))
    }

    fn on_tool_start_with_id<'a>(
        &'a self,
        agent: &'a str,
        call_id: &'a str,
        tool: &'a str,
        args: &'a serde_json::Value,
    ) -> futures::future::BoxFuture<'a, ()> {
        self.observe(to_wire(args).map(|args| {
            ExtensionInvocation::CallbackOnToolStart(CallbackToolStartInput {
                agent: agent.to_string(),
                call_id: Some(call_id.to_string()),
                tool: tool.to_string(),
                args,
            })
        }))
    }

    fn on_tool_end_with_id<'a>(
        &'a self,
        agent: &'a str,
        call_id: &'a str,
        tool: &'a str,
        result: &'a str,
    ) -> futures::future::BoxFuture<'a, ()> {
        self.observe_connection_scoped(Ok(ExtensionInvocation::CallbackOnToolEnd(
            CallbackToolEndInput {
                agent: agent.to_string(),
                call_id: Some(call_id.to_string()),
                tool: tool.to_string(),
                result: result.to_string(),
            },
        )))
    }

    fn on_tool_error_with_id<'a>(
        &'a self,
        agent: &'a str,
        call_id: &'a str,
        tool: &'a str,
        error: &'a ReactError,
    ) -> futures::future::BoxFuture<'a, ()> {
        self.observe_connection_scoped(Ok(ExtensionInvocation::CallbackOnToolError(
            CallbackToolErrorInput {
                agent: agent.to_string(),
                call_id: Some(call_id.to_string()),
                tool: tool.to_string(),
                error: error.to_string(),
            },
        )))
    }

    fn on_tool_interrupted_with_id<'a>(
        &'a self,
        agent: &'a str,
        call_id: &'a str,
        tool: &'a str,
        input: &'a serde_json::Value,
        error: &'a ReactError,
    ) -> futures::future::BoxFuture<'a, ()> {
        self.observe_connection_scoped(to_wire(input).map(|input| {
            ExtensionInvocation::CallbackOnToolInterrupted(CallbackToolInterruptedInput {
                agent: agent.to_string(),
                call_id: call_id.to_string(),
                tool: tool.to_string(),
                input,
                error: error.to_string(),
            })
        }))
    }

    fn on_final_answer<'a>(
        &'a self,
        agent: &'a str,
        answer: &'a str,
    ) -> futures::future::BoxFuture<'a, ()> {
        self.observe_connection_scoped(Ok(ExtensionInvocation::CallbackOnFinalAnswer(
            CallbackFinalAnswerInput {
                agent: agent.to_string(),
                answer: answer.to_string(),
            },
        )))
    }

    fn on_iteration<'a>(
        &'a self,
        agent: &'a str,
        iteration: usize,
    ) -> futures::future::BoxFuture<'a, ()> {
        self.observe(Ok(ExtensionInvocation::CallbackOnIteration(
            CallbackIterationInput {
                agent: agent.to_string(),
                iteration: WireU64::from_u64(u64::try_from(iteration).unwrap_or(u64::MAX)),
            },
        )))
    }
}

fn step_type_wire(step: &echo_agent::agent::StepType) -> std::result::Result<StepTypeWire, String> {
    Ok(match step {
        echo_agent::agent::StepType::Thought(text) => StepTypeWire::Thought { text: text.clone() },
        echo_agent::agent::StepType::Call {
            tool_call_id,
            function_name,
            arguments,
        } => StepTypeWire::Call {
            tool_call_id: tool_call_id.clone(),
            function_name: function_name.clone(),
            arguments: to_wire(arguments)?,
        },
    })
}

// ── InterventionCallback proxy ──────────────────────────────────────────────

fn intervention_result_from_wire(
    wire: InterventionResultWire,
) -> std::result::Result<echo_agent::agent::InterventionResult, String> {
    Ok(echo_agent::agent::InterventionResult {
        block: wire.block,
        block_reason: wire.block_reason,
        injected_context: wire.injected_context,
        redirect_to: wire.redirect_to,
        cancel: wire.cancel,
        modified_args: wire
            .modified_args
            .map(|value| value.into_json().map_err(|error| error.to_string()))
            .transpose()?,
    })
}

/// Behavior-controlling intervention proxy. It is its own extension kind —
/// never an alias of the observational AgentCallback. Decode, transport and
/// deadline failures return `cancel` so a missing policy decision cannot
/// silently authorize a behavior-changing action.
pub(crate) struct ExtensionInterventionProxy {
    bridge: Arc<ExtensionBridge>,
    extension: WireHandle,
    session_id: String,
}

impl ExtensionInterventionProxy {
    pub(crate) fn new(
        bridge: Arc<ExtensionBridge>,
        extension: WireHandle,
        session_id: String,
    ) -> Option<Self> {
        let record = bridge.state().ok()?.handles.extension(&extension).ok()?;
        matches!(
            record.descriptor,
            ExtensionDescriptor::InterventionCallback { .. }
        )
        .then(|| Self {
            bridge,
            extension,
            session_id,
        })
    }

    fn decide(
        &self,
        invocation: std::result::Result<ExtensionInvocation, String>,
    ) -> futures::future::BoxFuture<'_, echo_agent::agent::InterventionResult> {
        let bridge = self.bridge.clone();
        let extension = self.extension.clone();
        let context = session_invocation_context(&self.session_id);
        Box::pin(async move {
            let Ok(invocation) = invocation else {
                return echo_agent::agent::InterventionResult::cancel();
            };
            match bridge
                .invoke_once(
                    &extension,
                    Some(context),
                    invocation,
                    bridge.connection_cancellation(),
                )
                .await
            {
                Ok(ExtensionResult::InterventionOnToolCall(value))
                | Ok(ExtensionResult::InterventionOnThinkStart(value))
                | Ok(ExtensionResult::InterventionOnFinalAnswer(value)) => {
                    intervention_result_from_wire(value).unwrap_or_else(|error| {
                        tracing::warn!(
                            "intervention extension {} returned an invalid result: {}",
                            extension.id,
                            error
                        );
                        echo_agent::agent::InterventionResult::cancel()
                    })
                }
                Ok(_) => echo_agent::agent::InterventionResult::cancel(),
                Err(error) => {
                    tracing::warn!(
                        "intervention extension {} failed: {}",
                        extension.id,
                        error.message
                    );
                    echo_agent::agent::InterventionResult::cancel()
                }
            }
        })
    }
}

impl echo_agent::agent::InterventionCallback for ExtensionInterventionProxy {
    fn on_tool_call<'a>(
        &'a self,
        agent: &'a str,
        tool: &'a str,
        args: &'a serde_json::Value,
    ) -> futures::future::BoxFuture<'a, echo_agent::agent::InterventionResult> {
        self.decide(to_wire(args).map(|args| {
            ExtensionInvocation::InterventionOnToolCall(CallbackToolStartInput {
                agent: agent.to_string(),
                call_id: None,
                tool: tool.to_string(),
                args,
            })
        }))
    }

    fn on_think_start<'a>(
        &'a self,
        agent: &'a str,
        messages: &'a [echo_agent::llm::types::Message],
    ) -> futures::future::BoxFuture<'a, echo_agent::agent::InterventionResult> {
        self.decide(
            messages
                .iter()
                .map(message_wire)
                .collect::<std::result::Result<Vec<_>, _>>()
                .map(|messages| {
                    ExtensionInvocation::InterventionOnThinkStart(CallbackThinkStartInput {
                        agent: agent.to_string(),
                        messages,
                    })
                }),
        )
    }

    fn on_final_answer<'a>(
        &'a self,
        agent: &'a str,
        answer: &'a str,
    ) -> futures::future::BoxFuture<'a, echo_agent::agent::InterventionResult> {
        self.decide(Ok(ExtensionInvocation::InterventionOnFinalAnswer(
            CallbackFinalAnswerInput {
                agent: agent.to_string(),
                answer: answer.to_string(),
            },
        )))
    }
}

// ── AgentFactory + custom Agent proxies ─────────────────────────────────────

/// Async subagent-factory adapter: the framework's lazy subagent
/// construction calls `create()`, which forwards to the same factory
/// operation with a minimal construction config.
#[derive(Clone)]
pub(crate) struct SubagentFactoryAdapter {
    bridge: Arc<ExtensionBridge>,
    extension: WireHandle,
    subagent_name: String,
    session_id: String,
}

impl SubagentFactoryAdapter {
    #[cfg(any(feature = "framework-eval", feature = "framework-improve"))]
    pub(crate) fn for_registration(
        bridge: Arc<ExtensionBridge>,
        extension: WireHandle,
        subagent_name: String,
        session_id: String,
    ) -> Option<Self> {
        let record = bridge.state().ok()?.handles.extension(&extension).ok()?;
        matches!(record.descriptor, ExtensionDescriptor::AgentFactory { .. }).then_some(Self {
            bridge,
            extension,
            subagent_name,
            session_id,
        })
    }
}

impl echo_agent::agent::subagent::AgentFactory for SubagentFactoryAdapter {
    fn create(
        &self,
    ) -> futures::future::BoxFuture<'static, echo_agent::error::Result<Box<dyn Agent>>> {
        let bridge = self.bridge.clone();
        let extension = self.extension.clone();
        let subagent_name = self.subagent_name.clone();
        let session_id = self.session_id.clone();
        Box::pin(async move {
            let state = bridge.state().map_err(react_error)?;
            let factory_record = state.handles.extension(&extension).map_err(react_error)?;
            let implementation_id = format!(
                "{}::instance-{}",
                factory_record.implementation_id,
                uuid::Uuid::new_v4()
            );
            let custom_extension = state
                .handles
                .reserve_factory_instance(
                    super::handles::ExtensionRegistrationLimits {
                        max_extensions: state.limits.max_registered_extensions,
                        max_descriptor_bytes: state.limits.max_extension_descriptor_bytes,
                    },
                    &extension,
                    &implementation_id,
                    factory_record.timeout.clone(),
                )
                .map_err(react_error)?;
            // Establish cleanup ownership before the remote constructor can
            // see its identity. Outer cancellation drops this owner too.
            let factory_lease = Arc::new(FactoryInstanceLease {
                bridge: bridge.clone(),
                extension: custom_extension.clone(),
                session_id: session_id.clone(),
                close_result: tokio::sync::OnceCell::new(),
                construction_submitted: AtomicBool::new(false),
            });
            let payload = AgentFactoryConfigWire {
                instance: custom_extension.clone(),
                model: String::new(),
                name: subagent_name,
                system_prompt: String::new(),
                tool_count: WireU64::from_u64(0),
            };
            let value = bridge
                .invoke_once_with_scope(
                    &extension,
                    Some(session_invocation_context(&session_id)),
                    ExtensionInvocation::FactoryCreateAgent(payload),
                    ReverseInvocationScope {
                        cancellation: bridge.connection_cancellation(),
                        resolve_run_cancellation: true,
                        submitted: Some(&factory_lease.construction_submitted),
                    },
                )
                .await
                .map_err(react_error);
            let descriptor = match value {
                Ok(ExtensionResult::FactoryCreateAgent(descriptor)) => descriptor,
                other => {
                    if let Err(error) = factory_lease.close().await {
                        tracing::warn!(
                            "factory construction cleanup retained pending owner: {error}"
                        );
                    }
                    return Err(match other {
                        Err(error) => error,
                        _ => ReactError::Other(
                            "agent factory returned the wrong result variant".to_string(),
                        ),
                    });
                }
            };
            if let Err(error) =
                bridge.register_factory_instance(&extension, &custom_extension, &descriptor)
            {
                if let Err(error) = factory_lease.close().await {
                    tracing::warn!("factory publication cleanup retained pending owner: {error}");
                }
                return Err(react_error(error));
            }
            Ok(Box::new(ExtensionCustomAgentProxy {
                bridge,
                extension: custom_extension,
                descriptor,
                session_id,
                factory_lease: Some(factory_lease),
            }) as Box<dyn Agent>)
        })
    }
}

struct FactoryInstanceLease {
    bridge: Arc<ExtensionBridge>,
    extension: WireHandle,
    session_id: String,
    close_result: tokio::sync::OnceCell<std::result::Result<(), String>>,
    construction_submitted: AtomicBool,
}

impl FactoryInstanceLease {
    async fn close(&self) -> echo_agent::error::Result<()> {
        self.close_result
            .get_or_init(|| async {
                if !self.construction_submitted.load(Ordering::Acquire) {
                    self.bridge.release_extension(&self.extension);
                    return Ok(());
                }

                close_factory_instance(
                    self.bridge.clone(),
                    self.extension.clone(),
                    self.session_id.clone(),
                )
                .await
                .map_err(|error| error.to_string())
            })
            .await
            .clone()
            .map_err(ReactError::Other)
    }
}

impl Drop for FactoryInstanceLease {
    fn drop(&mut self) {
        if self.close_result.get().is_some_and(Result::is_ok) {
            return;
        }
        if !self.construction_submitted.load(Ordering::Acquire) {
            self.bridge.release_extension(&self.extension);
            return;
        }
        let bridge = self.bridge.clone();
        let extension = self.extension.clone();
        let session_id = self.session_id.clone();
        if let Ok(runtime) = tokio::runtime::Handle::try_current() {
            let task = runtime.spawn(async move {
                // A failed close retains the reserved child and its parent
                // relationship. Retry under the connection's bounded scope;
                // no failed acknowledgement becomes a successful release.
                let connection = bridge.connection_cancellation();
                let mut retry_delay = Duration::from_millis(100);
                loop {
                    if connection.is_cancelled() {
                        return;
                    }
                    match close_factory_instance(
                        bridge.clone(),
                        extension.clone(),
                        session_id.clone(),
                    )
                    .await
                    {
                        Ok(()) => return,
                        Err(error) => {
                            tracing::warn!("factory-created custom agent close failed: {error}")
                        }
                    }
                    tokio::select! {
                        () = connection.cancelled() => return,
                        () = tokio::time::sleep(retry_delay) => {}
                    }
                    retry_delay = retry_delay.saturating_mul(2).min(Duration::from_secs(5));
                }
            });
            self.bridge.shared.track_factory_cleanup(task);
        } else {
            tracing::warn!("factory child cleanup requires a live runtime; retaining owned handle");
        }
    }
}

async fn close_factory_instance(
    bridge: Arc<ExtensionBridge>,
    extension: WireHandle,
    session_id: String,
) -> echo_agent::error::Result<()> {
    let deadline = Duration::from_secs(
        bridge
            .state()
            .map(|state| state.limits.shutdown_timeout_secs)
            .unwrap_or(5),
    );
    let close = bridge.invoke_once_connection_scoped(
        &extension,
        Some(session_invocation_context(&session_id)),
        ExtensionInvocation::AgentClose(ExtensionUnit),
    );
    let result = tokio::time::timeout(deadline, close)
        .await
        .map_err(|_| {
            ReactError::Other("factory child cleanup exceeded its bounded deadline".to_string())
        })
        .and_then(|result| result.map_err(react_error))
        .and_then(|result| {
            matches!(result, ExtensionResult::AgentClose(_))
                .then_some(())
                .ok_or_else(|| {
                    ReactError::Other(
                        "factory-created custom agent close returned the wrong result variant"
                            .to_string(),
                    )
                })
        });
    if result.is_ok() {
        bridge.release_extension(&extension);
    }
    result
}

/// Thin custom `Agent` proxy: execute/chat (and their streaming forms) run
/// through the bridge; events the SDK emits are plain `AgentEvent` values
/// the existing run observers already project — the proxy never fabricates
/// sequence numbers or terminals.
#[derive(Clone)]
pub(crate) struct ExtensionCustomAgentProxy {
    bridge: Arc<ExtensionBridge>,
    extension: WireHandle,
    descriptor: CustomAgentDescriptorWire,
    session_id: String,
    factory_lease: Option<Arc<FactoryInstanceLease>>,
}

impl ExtensionCustomAgentProxy {
    async fn run_once(
        &self,
        invocation: ExtensionInvocation,
    ) -> echo_agent::error::Result<ExtensionResult> {
        let value = self
            .bridge
            .invoke_once(
                &self.extension,
                Some(session_invocation_context(&self.session_id)),
                invocation,
                self.bridge.connection_cancellation(),
            )
            .await
            .map_err(react_error)?;
        Ok(value)
    }

    async fn run_stream(
        &self,
        invocation: ExtensionInvocation,
    ) -> echo_agent::error::Result<
        futures::stream::BoxStream<'static, echo_agent::error::Result<AgentEvent>>,
    > {
        self.run_stream_scoped(
            invocation,
            self.bridge.connection_cancellation().child_token(),
            None,
        )
        .await
    }

    async fn run_stream_scoped(
        &self,
        invocation: ExtensionInvocation,
        cancellation: CancellationToken,
        context: Option<echo_agent::agent::AgentInvocationContext>,
    ) -> echo_agent::error::Result<
        futures::stream::BoxStream<'static, echo_agent::error::Result<AgentEvent>>,
    > {
        let wire_context = custom_agent_invocation_context(&self.session_id, context.as_ref());
        let source = self
            .bridge
            .invoke_owned_producer_stream(
                &self.extension,
                wire_context,
                invocation,
                cancellation.clone(),
                context,
            )
            .await
            .map_err(react_error)?;
        Ok(futures::stream::unfold(
            (source, cancellation, false),
            async move |(mut source, cancellation, finished)| {
                if finished {
                    return None;
                }
                let event = source.next().await?;
                let result = event
                    .map_err(react_error)
                    .and_then(custom_agent_stream_event);
                if result.is_err() {
                    cancellation.cancel();
                    // A conversion failure is local. Keep driving the original
                    // owner until its RPC/producer receipt before exposing an error
                    // that the canonical turn driver treats as terminal.
                    while source.next().await.is_some() {}
                    return Some((result, (source, cancellation, true)));
                }
                Some((result, (source, cancellation, false)))
            },
        )
        .boxed())
    }

    async fn finish_factory_result<T>(
        &self,
        result: echo_agent::error::Result<T>,
    ) -> echo_agent::error::Result<T> {
        let Some(lease) = &self.factory_lease else {
            return result;
        };
        let close = lease.close().await;
        match (result, close) {
            (Err(error), _) => Err(error),
            (Ok(value), Ok(())) => Ok(value),
            (Ok(_), Err(error)) => Err(error),
        }
    }
}

fn custom_agent_invocation_context(
    session_id: &str,
    invocation: Option<&echo_agent::agent::AgentInvocationContext>,
) -> ExtensionInvocationContext {
    let mut context = session_invocation_context(session_id);
    if let Some(runtime) = invocation.and_then(|invocation| invocation.runtime.as_ref()) {
        context.run_id = runtime.run_id.clone();
        context.turn_id = runtime.turn_id.clone();
        context.message_id = runtime.message_id.clone();
        context.execution_id = runtime.execution_id.clone();
    }
    context
}

fn custom_agent_stream_event(event: ExtensionStreamEvent) -> echo_agent::error::Result<AgentEvent> {
    match event {
        ExtensionStreamEvent::Chunk {
            value: ExtensionStreamChunkValue::Agent(value),
            ..
        } => agent_stream_chunk_from_wire(value).map_err(ReactError::Other),
        ExtensionStreamEvent::Complete {
            value: ExtensionStreamCompleteValue::Agent(value),
            ..
        } => agent_stream_terminal_from_wire(value).map_err(ReactError::Other),
        ExtensionStreamEvent::Failed { error, .. } => Err(react_error(error)),
        ExtensionStreamEvent::Cancelled { .. } => Ok(AgentEvent::Cancelled),
        _ => Err(ReactError::Other(
            "custom agent stream received a non-agent payload".to_string(),
        )),
    }
}

fn close_factory_stream(
    stream: futures::stream::BoxStream<'static, echo_agent::error::Result<AgentEvent>>,
    lease: Arc<FactoryInstanceLease>,
) -> futures::stream::BoxStream<'static, echo_agent::error::Result<AgentEvent>> {
    futures::stream::unfold(
        (stream, Some(lease)),
        |(mut stream, mut lease)| async move {
            match stream.next().await {
                Some(item) => {
                    let terminal = item.as_ref().is_err()
                        || matches!(
                            item.as_ref(),
                            Ok(AgentEvent::FinalAnswer(_)
                                | AgentEvent::Error { .. }
                                | AgentEvent::Cancelled)
                        );
                    if terminal
                        && let Some(owner) = lease.take()
                        && let Err(close_error) = owner.close().await
                    {
                        return Some((Err(close_error), (stream, lease)));
                    }
                    Some((item, (stream, lease)))
                }
                None => {
                    if let Some(owner) = lease.take()
                        && let Err(close_error) = owner.close().await
                    {
                        return Some((Err(close_error), (stream, lease)));
                    }
                    None
                }
            }
        },
    )
    .boxed()
}

impl Agent for ExtensionCustomAgentProxy {
    fn name(&self) -> &str {
        &self.descriptor.name
    }

    fn model_name(&self) -> &str {
        &self.descriptor.model_name
    }

    fn system_prompt(&self) -> &str {
        &self.descriptor.system_prompt
    }

    fn tool_names(&self) -> Vec<String> {
        self.descriptor.tool_names.clone()
    }

    fn close<'a>(&'a self) -> futures::future::BoxFuture<'a, echo_agent::error::Result<()>> {
        Box::pin(async move {
            if let Some(lease) = &self.factory_lease {
                return lease.close().await;
            }
            let result = self
                .run_once(ExtensionInvocation::AgentClose(ExtensionUnit))
                .await
                .and_then(|result| {
                    matches!(result, ExtensionResult::AgentClose(_))
                        .then_some(())
                        .ok_or_else(|| {
                            ReactError::Other(
                                "custom agent close returned the wrong result variant".to_string(),
                            )
                        })
                });
            self.bridge.release_extension(&self.extension);
            result
        })
    }

    fn execute<'a>(
        &'a self,
        task: &'a str,
    ) -> futures::future::BoxFuture<'a, echo_agent::error::Result<String>> {
        Box::pin(async move {
            let result = match self
                .run_once(ExtensionInvocation::AgentExecute(AgentTaskInput {
                    task: task.to_string(),
                }))
                .await
            {
                Ok(ExtensionResult::AgentExecute(output)) => Ok(output),
                Ok(_) => Err(ReactError::Other(
                    "custom agent execute returned the wrong result variant".to_string(),
                )),
                Err(error) => Err(error),
            };
            self.finish_factory_result(result).await
        })
    }

    fn execute_stream<'a>(
        &'a self,
        task: &'a str,
    ) -> futures::future::BoxFuture<
        'a,
        echo_agent::error::Result<
            futures::stream::BoxStream<'a, echo_agent::error::Result<AgentEvent>>,
        >,
    > {
        Box::pin(async move {
            let stream = match self
                .run_stream(ExtensionInvocation::AgentExecuteStream(AgentTaskInput {
                    task: task.to_string(),
                }))
                .await
            {
                Ok(stream) => stream,
                Err(error) => return self.finish_factory_result(Err(error)).await,
            };
            Ok(match &self.factory_lease {
                Some(lease) => close_factory_stream(stream, lease.clone()),
                None => stream,
            })
        })
    }

    fn execute_stream_with_cancel<'a>(
        &'a self,
        task: &'a str,
        cancel: CancellationToken,
    ) -> futures::future::BoxFuture<
        'a,
        echo_agent::error::Result<
            futures::stream::BoxStream<'a, echo_agent::error::Result<AgentEvent>>,
        >,
    > {
        self.execute_stream_with_invocation_context(
            task,
            cancel,
            echo_agent::agent::AgentInvocationContext::default(),
        )
    }

    fn execute_stream_with_invocation_context<'a>(
        &'a self,
        task: &'a str,
        cancel: CancellationToken,
        invocation: echo_agent::agent::AgentInvocationContext,
    ) -> futures::future::BoxFuture<
        'a,
        echo_agent::error::Result<
            futures::stream::BoxStream<'a, echo_agent::error::Result<AgentEvent>>,
        >,
    > {
        Box::pin(async move {
            let stream = match self
                .run_stream_scoped(
                    ExtensionInvocation::AgentExecuteStream(AgentTaskInput {
                        task: task.to_string(),
                    }),
                    cancel,
                    Some(invocation),
                )
                .await
            {
                Ok(stream) => stream,
                Err(error) => return self.finish_factory_result(Err(error)).await,
            };
            Ok(match &self.factory_lease {
                Some(lease) => close_factory_stream(stream, lease.clone()),
                None => stream,
            })
        })
    }

    fn chat_stream_with_cancel<'a>(
        &'a self,
        message: &'a str,
        cancel: CancellationToken,
    ) -> futures::future::BoxFuture<
        'a,
        echo_agent::error::Result<
            futures::stream::BoxStream<'a, echo_agent::error::Result<AgentEvent>>,
        >,
    > {
        Box::pin(async move {
            let stream = match self
                .run_stream_scoped(
                    ExtensionInvocation::AgentChatStream(AgentMessageInput {
                        message: message.to_string(),
                    }),
                    cancel,
                    None,
                )
                .await
            {
                Ok(stream) => stream,
                Err(error) => return self.finish_factory_result(Err(error)).await,
            };
            Ok(match &self.factory_lease {
                Some(lease) => close_factory_stream(stream, lease.clone()),
                None => stream,
            })
        })
    }

    fn execute_stream_message_with_cancel<'a>(
        &'a self,
        message: echo_agent::llm::types::Message,
        cancel: CancellationToken,
    ) -> futures::future::BoxFuture<
        'a,
        echo_agent::error::Result<
            futures::stream::BoxStream<'a, echo_agent::error::Result<AgentEvent>>,
        >,
    > {
        self.execute_stream_message_with_invocation_context(
            message,
            cancel,
            echo_agent::agent::AgentInvocationContext::default(),
        )
    }

    fn execute_stream_message_with_invocation_context<'a>(
        &'a self,
        message: echo_agent::llm::types::Message,
        cancel: CancellationToken,
        invocation: echo_agent::agent::AgentInvocationContext,
    ) -> futures::future::BoxFuture<
        'a,
        echo_agent::error::Result<
            futures::stream::BoxStream<'a, echo_agent::error::Result<AgentEvent>>,
        >,
    > {
        Box::pin(async move {
            let text = match message.content {
                echo_agent::llm::types::MessageContent::Text(text) => text,
                _ => return Err(ReactError::Other("custom Agent accepted task wire supports text only; multimodal input is unsupported".to_string())),
            };
            let stream = match self
                .run_stream_scoped(
                    ExtensionInvocation::AgentExecuteStream(AgentTaskInput { task: text }),
                    cancel,
                    Some(invocation),
                )
                .await
            {
                Ok(stream) => stream,
                Err(error) => return self.finish_factory_result(Err(error)).await,
            };
            Ok(match &self.factory_lease {
                Some(lease) => close_factory_stream(stream, lease.clone()),
                None => stream,
            })
        })
    }

    fn chat_stream_message_with_cancel<'a>(
        &'a self,
        message: echo_agent::llm::types::Message,
        cancel: CancellationToken,
    ) -> futures::future::BoxFuture<
        'a,
        echo_agent::error::Result<
            futures::stream::BoxStream<'a, echo_agent::error::Result<AgentEvent>>,
        >,
    > {
        self.chat_stream_message_with_invocation_context(
            message,
            cancel,
            echo_agent::agent::AgentInvocationContext::default(),
        )
    }

    fn chat_stream_message_with_invocation_context<'a>(
        &'a self,
        message: echo_agent::llm::types::Message,
        cancel: CancellationToken,
        invocation: echo_agent::agent::AgentInvocationContext,
    ) -> futures::future::BoxFuture<
        'a,
        echo_agent::error::Result<
            futures::stream::BoxStream<'a, echo_agent::error::Result<AgentEvent>>,
        >,
    > {
        Box::pin(async move {
            let text = match message.content {
                echo_agent::llm::types::MessageContent::Text(text) => text,
                _ => return Err(ReactError::Other("custom Agent accepted message wire supports text only; multimodal input is unsupported".to_string())),
            };
            let stream = match self
                .run_stream_scoped(
                    ExtensionInvocation::AgentChatStream(AgentMessageInput { message: text }),
                    cancel,
                    Some(invocation),
                )
                .await
            {
                Ok(stream) => stream,
                Err(error) => return self.finish_factory_result(Err(error)).await,
            };
            Ok(match &self.factory_lease {
                Some(lease) => close_factory_stream(stream, lease.clone()),
                None => stream,
            })
        })
    }

    fn chat<'a>(
        &'a self,
        message: &'a str,
    ) -> futures::future::BoxFuture<'a, echo_agent::error::Result<String>> {
        Box::pin(async move {
            let result = match self
                .run_once(ExtensionInvocation::AgentChat(AgentMessageInput {
                    message: message.to_string(),
                }))
                .await
            {
                Ok(ExtensionResult::AgentChat(output)) => Ok(output),
                Ok(_) => Err(ReactError::Other(
                    "custom agent chat returned the wrong result variant".to_string(),
                )),
                Err(error) => Err(error),
            };
            self.finish_factory_result(result).await
        })
    }

    fn chat_stream<'a>(
        &'a self,
        message: &'a str,
    ) -> futures::future::BoxFuture<
        'a,
        echo_agent::error::Result<
            futures::stream::BoxStream<'a, echo_agent::error::Result<AgentEvent>>,
        >,
    > {
        Box::pin(async move {
            let stream = match self
                .run_stream(ExtensionInvocation::AgentChatStream(AgentMessageInput {
                    message: message.to_string(),
                }))
                .await
            {
                Ok(stream) => stream,
                Err(error) => return self.finish_factory_result(Err(error)).await,
            };
            Ok(match &self.factory_lease {
                Some(lease) => close_factory_stream(stream, lease.clone()),
                None => stream,
            })
        })
    }
}

#[cfg(feature = "framework-channels")]
#[allow(dead_code)]
fn channel_chat_type_to_wire(value: ChannelChatType) -> ChannelChatTypeWire {
    match value {
        ChannelChatType::Direct => ChannelChatTypeWire::Direct,
        ChannelChatType::Group => ChannelChatTypeWire::Group,
    }
}

#[cfg(feature = "framework-channels")]
#[allow(dead_code)]
fn channel_chat_type_from_wire(value: ChannelChatTypeWire) -> ChannelChatType {
    match value {
        ChannelChatTypeWire::Direct => ChannelChatType::Direct,
        ChannelChatTypeWire::Group => ChannelChatType::Group,
    }
}

#[cfg(feature = "framework-channels")]
#[allow(dead_code)]
fn channel_attachment_to_wire(value: &MessageAttachment) -> ChannelAttachmentWire {
    let (kind, data) = match value.kind {
        AttachmentKind::Image => ("image", &value.data),
        AttachmentKind::File => ("file", &value.data),
        AttachmentKind::Audio => ("audio", &value.data),
        AttachmentKind::Video => ("video", &value.data),
    };
    ChannelAttachmentWire {
        kind: kind.to_string(),
        data_base64: base64::engine::general_purpose::STANDARD.encode(data),
        filename: value.filename.clone(),
    }
}

#[cfg(feature = "framework-channels")]
#[allow(dead_code)]
fn channel_inbound_to_wire(value: &InboundMessage) -> ChannelInboundMessageWire {
    ChannelInboundMessageWire {
        channel_id: value.channel_id.clone(),
        sender_id: value.sender_id.clone(),
        chat_id: value.chat_id.clone(),
        chat_type: channel_chat_type_to_wire(value.chat_type),
        text: value.text.clone(),
        message_id: value.message_id.clone(),
        timestamp: WireU64::from_u64(value.timestamp),
        attachments: value
            .attachments
            .iter()
            .map(channel_attachment_to_wire)
            .collect(),
    }
}

#[cfg(feature = "framework-channels")]
#[allow(dead_code)]
pub(crate) fn channel_outbound_to_wire(value: &OutboundMessage) -> ChannelOutboundMessageWire {
    ChannelOutboundMessageWire {
        channel_id: value.channel_id.clone(),
        to: value.to.clone(),
        chat_type: channel_chat_type_to_wire(value.chat_type),
        text: value.text.clone(),
        reply_to: value.reply_to.clone(),
        attachments: value
            .attachments
            .iter()
            .map(channel_attachment_to_wire)
            .collect(),
    }
}

#[cfg(feature = "framework-channels")]
#[allow(dead_code)]
fn channel_attachment_from_wire(
    value: ChannelAttachmentWire,
) -> echo_agent::error::Result<MessageAttachment> {
    let data = base64::engine::general_purpose::STANDARD
        .decode(value.data_base64)
        .map_err(|error| ReactError::Other(format!("channel attachment base64: {error}")))?;
    let kind = match value.kind.as_str() {
        "image" => AttachmentKind::Image,
        "file" => AttachmentKind::File,
        "audio" => AttachmentKind::Audio,
        "video" => AttachmentKind::Video,
        other => {
            return Err(ReactError::Other(format!(
                "unknown channel attachment kind {other}"
            )));
        }
    };
    Ok(MessageAttachment {
        kind,
        data,
        filename: value.filename,
    })
}

#[cfg(feature = "framework-channels")]
#[allow(dead_code)]
pub(crate) fn channel_inbound_from_wire(
    value: ChannelInboundMessageWire,
) -> echo_agent::error::Result<InboundMessage> {
    if value.channel_id.trim().is_empty()
        || value.sender_id.trim().is_empty()
        || value.chat_id.trim().is_empty()
    {
        return Err(ReactError::Other(
            "channel/sender/chat identities must be nonempty".to_string(),
        ));
    }
    Ok(InboundMessage {
        channel_id: value.channel_id,
        sender_id: value.sender_id,
        chat_id: value.chat_id,
        chat_type: channel_chat_type_from_wire(value.chat_type),
        text: value.text,
        message_id: value.message_id,
        timestamp: value.timestamp.to_u64().ok_or_else(|| {
            ReactError::Other("channel timestamp is not a canonical u64".to_string())
        })?,
        attachments: value
            .attachments
            .into_iter()
            .map(channel_attachment_from_wire)
            .collect::<echo_agent::error::Result<Vec<_>>>()?,
    })
}

#[cfg(feature = "framework-channels")]
#[allow(dead_code)]
pub(crate) fn channel_outbound_from_wire(
    value: ChannelOutboundMessageWire,
) -> echo_agent::error::Result<OutboundMessage> {
    let mut message = OutboundMessage::new(
        value.channel_id,
        value.to,
        channel_chat_type_from_wire(value.chat_type),
        value.text,
    )
    .with_attachments(
        value
            .attachments
            .into_iter()
            .map(channel_attachment_from_wire)
            .collect::<echo_agent::error::Result<Vec<_>>>()?,
    );
    message.reply_to = value.reply_to;
    Ok(message)
}

#[cfg(feature = "framework-channels")]
#[allow(dead_code)]
pub(crate) struct ExtensionChannelMessageHandlerProxy {
    bridge: Arc<ExtensionBridge>,
    extension: WireHandle,
}

#[cfg(feature = "framework-channels")]
#[allow(dead_code)]
impl ExtensionChannelMessageHandlerProxy {
    pub(crate) fn new(bridge: Arc<ExtensionBridge>, extension: WireHandle) -> Self {
        Self { bridge, extension }
    }

    async fn invoke(
        &self,
        invocation: ExtensionInvocation,
        cancellation: CancellationToken,
    ) -> echo_agent::error::Result<ExtensionResult> {
        self.bridge
            .invoke_channel_once(&self.extension, invocation, cancellation)
            .await
            .map_err(react_error)
    }
}

#[cfg(feature = "framework-channels")]
#[async_trait]
impl MessageHandler for ExtensionChannelMessageHandlerProxy {
    async fn handle(&self, message: InboundMessage) -> echo_agent::error::Result<OutboundMessage> {
        self.handle_with_cancel(message, self.bridge.connection_cancellation().child_token())
            .await
    }

    async fn handle_with_cancel(
        &self,
        message: InboundMessage,
        cancellation: CancellationToken,
    ) -> echo_agent::error::Result<OutboundMessage> {
        let result = self
            .invoke(
                ExtensionInvocation::ChannelHandle(ChannelHandleInput {
                    message: channel_inbound_to_wire(&message),
                }),
                cancellation,
            )
            .await?;
        let ExtensionResult::ChannelHandle(value) = result else {
            return Err(ReactError::Other(
                "channel handler returned the wrong result variant".to_string(),
            ));
        };
        channel_outbound_from_wire(value)
    }

    async fn handle_stream<'a>(
        &'a self,
        message: InboundMessage,
    ) -> echo_agent::error::Result<
        futures::stream::BoxStream<'a, echo_agent::error::Result<OutboundMessage>>,
    > {
        self.handle_stream_with_cancel(message, self.bridge.connection_cancellation().child_token())
            .await
    }

    async fn handle_stream_with_cancel<'a>(
        &'a self,
        message: InboundMessage,
        cancellation: CancellationToken,
    ) -> echo_agent::error::Result<
        futures::stream::BoxStream<'a, echo_agent::error::Result<OutboundMessage>>,
    > {
        self.bridge
            .invoke_channel_stream(
                &self.extension,
                ExtensionInvocation::ChannelHandleStream(ChannelHandleInput {
                    message: channel_inbound_to_wire(&message),
                }),
                cancellation,
            )
            .await
            .map_err(react_error)
    }

    fn settles_on_cancel(&self) -> bool {
        true
    }

    async fn reply(&self, message: OutboundMessage) -> echo_agent::error::Result<()> {
        self.reply_with_cancel(message, self.bridge.connection_cancellation().child_token())
            .await
    }

    async fn reply_with_cancel(
        &self,
        message: OutboundMessage,
        cancellation: CancellationToken,
    ) -> echo_agent::error::Result<()> {
        let result = self
            .invoke(
                ExtensionInvocation::ChannelReply(ChannelReplyInput {
                    message: channel_outbound_to_wire(&message),
                }),
                cancellation,
            )
            .await?;
        if matches!(result, ExtensionResult::ChannelReply(_)) {
            Ok(())
        } else {
            Err(ReactError::Other(
                "channel handler reply returned the wrong result variant".to_string(),
            ))
        }
    }
}

#[cfg(feature = "framework-channels")]
#[allow(dead_code)]
pub(crate) struct ExtensionChannelPluginProxy {
    bridge: Arc<ExtensionBridge>,
    extension: WireHandle,
    descriptor: ChannelPluginDescriptorWire,
    capabilities: ChannelCapabilities,
}

#[cfg(feature = "framework-channels")]
#[allow(dead_code)]
impl ExtensionChannelPluginProxy {
    pub(crate) fn new(
        bridge: Arc<ExtensionBridge>,
        extension: WireHandle,
        descriptor: ChannelPluginDescriptorWire,
    ) -> Self {
        static DIRECT: &[ChannelChatType] = &[ChannelChatType::Direct];
        static GROUP: &[ChannelChatType] = &[ChannelChatType::Group];
        static BOTH: &[ChannelChatType] = &[ChannelChatType::Direct, ChannelChatType::Group];
        let chat_types = match descriptor.capabilities.chat_types.as_slice() {
            [ChannelChatTypeWire::Direct] => DIRECT,
            [ChannelChatTypeWire::Group] => GROUP,
            _ => BOTH,
        };
        let capabilities = ChannelCapabilities {
            chat_types,
            supports_media: descriptor.capabilities.supports_media,
            supports_threads: descriptor.capabilities.supports_threads,
        };
        Self {
            bridge,
            extension,
            descriptor,
            capabilities,
        }
    }

    async fn invoke(
        &self,
        invocation: ExtensionInvocation,
    ) -> echo_agent::error::Result<ExtensionResult> {
        self.bridge
            .invoke_once_connection_scoped(&self.extension, None, invocation)
            .await
            .map_err(react_error)
    }
}

#[cfg(feature = "framework-channels")]
#[async_trait]
impl ChannelPlugin for ExtensionChannelPluginProxy {
    fn id(&self) -> &str {
        &self.descriptor.channel_id
    }

    fn label(&self) -> &str {
        &self.descriptor.label
    }

    fn capabilities(&self) -> &ChannelCapabilities {
        &self.capabilities
    }

    async fn start(&mut self, _handler: Arc<dyn MessageHandler>) -> echo_agent::error::Result<()> {
        // The original framework entry retains the handler. The proxy sends
        // only its declared name; all forward calls return to that entry.
        let result = self
            .invoke(ExtensionInvocation::ChannelStart(ChannelStartInput {
                handler_id: self.descriptor.handler_id.clone(),
            }))
            .await?;
        if matches!(result, ExtensionResult::ChannelStart(_)) {
            Ok(())
        } else {
            Err(ReactError::Other(
                "channel plugin start returned the wrong result variant".to_string(),
            ))
        }
    }

    async fn stop(&mut self) -> echo_agent::error::Result<()> {
        let result = self
            .invoke(ExtensionInvocation::ChannelStop(ExtensionUnit))
            .await?;
        if matches!(result, ExtensionResult::ChannelStop(_)) {
            Ok(())
        } else {
            Err(ReactError::Other(
                "channel plugin stop returned the wrong result variant".to_string(),
            ))
        }
    }

    async fn send(&self, message: OutboundMessage) -> echo_agent::error::Result<()> {
        let result = self
            .invoke(ExtensionInvocation::ChannelSend(ChannelSendInput {
                message: channel_outbound_to_wire(&message),
            }))
            .await?;
        if matches!(result, ExtensionResult::ChannelSend(_)) {
            Ok(())
        } else {
            Err(ReactError::Other(
                "channel plugin send returned the wrong result variant".to_string(),
            ))
        }
    }

    async fn health_check(&self) -> echo_agent::error::Result<()> {
        let result = self
            .invoke(ExtensionInvocation::ChannelHealth(ExtensionUnit))
            .await?;
        if matches!(result, ExtensionResult::ChannelHealth(_)) {
            Ok(())
        } else {
            Err(ReactError::Other(
                "channel plugin health returned the wrong result variant".to_string(),
            ))
        }
    }
}

// ── Session Agent construction integration ──────────────────────────────────

/// Inject every currently registered extension into one Session Agent at
/// construction. Called by `PreparedAgentDefinition::create_agent` so all
/// Sessions — standard `session/new` and `_echo_agent/session/create`
/// alike — share one bridge wiring path. Sessions created before a
/// registration simply never see it: registration is connection-owned and
/// takes effect for Agents constructed afterwards.
pub(crate) async fn apply_extensions_to_agent(
    bridge: &Arc<ExtensionBridge>,
    agent: &mut echo_agent::agent::ReactAgent,
    session_id: &str,
) -> std::result::Result<(), String> {
    let state = bridge.state().map_err(|error| error.message)?;
    // LlmClient: the most recently registered implementation becomes the
    // Session's model client.
    if let Some((extension, _)) = state
        .handles
        .extensions_of_kind(ExtensionKind::LlmClient)
        .into_iter()
        .max_by_key(|(_, record)| record.registration_order)
        && let Some(proxy) =
            ExtensionLlmClientProxy::new(bridge.clone(), extension, session_id.to_string())
    {
        agent.set_llm_client(Arc::new(proxy));
    }
    if let Some((extension, _)) = state
        .handles
        .extensions_of_kind(ExtensionKind::ContextCompressor)
        .into_iter()
        .max_by_key(|(_, record)| record.registration_order)
        && let Some(proxy) =
            ExtensionContextCompressorProxy::new(bridge.clone(), extension, session_id.to_string())
    {
        agent.set_compressor(proxy).await;
    }
    for component in [
        AgentComponentKindWire::ConversationStore,
        AgentComponentKindWire::RunStore,
        AgentComponentKindWire::RuntimeStateStore,
        AgentComponentKindWire::AuditLogger,
        AgentComponentKindWire::ContextProjector,
        AgentComponentKindWire::MemoryTriggerSink,
        AgentComponentKindWire::RevisionedTaskStore,
        AgentComponentKindWire::SandboxExecutor,
        AgentComponentKindWire::MemoryPromoter,
        AgentComponentKindWire::Embedder,
        AgentComponentKindWire::Workflow,
        AgentComponentKindWire::IntentClassifier,
        AgentComponentKindWire::SkillLoadPolicy,
    ] {
        let selected = state
            .handles
            .extensions_of_kind(ExtensionKind::AgentComponent)
            .into_iter()
            .filter(|(_, record)| {
                matches!(
                    &record.descriptor,
                    ExtensionDescriptor::AgentComponent {
                        component: descriptor_component,
                        ..
                    } if *descriptor_component == component
                )
            })
            .max_by_key(|(_, record)| record.registration_order);
        let Some((extension, _)) = selected else {
            continue;
        };
        let Some(proxy) =
            ExtensionAgentComponentProxy::new(bridge.clone(), extension, session_id.to_string())
        else {
            continue;
        };
        match component {
            AgentComponentKindWire::ConversationStore => {
                agent.set_conversation_store(Arc::new(proxy));
            }
            AgentComponentKindWire::RunStore => agent.set_run_store(Arc::new(proxy)),
            AgentComponentKindWire::RuntimeStateStore => agent.set_state_store(Arc::new(proxy)),
            AgentComponentKindWire::AuditLogger => agent.set_audit_logger(Arc::new(proxy)),
            AgentComponentKindWire::ContextProjector => {
                agent.set_pre_model_context_projector(Some(Arc::new(proxy)));
            }
            AgentComponentKindWire::MemoryTriggerSink => {
                agent.set_memory_trigger_sink(Some(Arc::new(proxy)));
            }
            AgentComponentKindWire::Guard => {}
            AgentComponentKindWire::SearchProvider => {}
            AgentComponentKindWire::WorkflowCheckpointStore => {}
            AgentComponentKindWire::RevisionedTaskStore => {
                echo_agent::tasks::register_task_tools(
                    agent,
                    Arc::new(echo_agent::tasks::TaskRevisionService::new(
                        Arc::new(proxy),
                        Arc::new(echo_agent::tasks::DefaultTaskToolPolicy::default()),
                    )),
                );
            }
            AgentComponentKindWire::SandboxExecutor => {
                agent.set_sandbox_executor(Arc::new(proxy));
            }
            AgentComponentKindWire::McpTransport => {}
            AgentComponentKindWire::MemoryPromoter => {
                agent.set_memory_promoter(Arc::new(proxy)).await;
            }
            AgentComponentKindWire::Embedder => {}
            AgentComponentKindWire::Workflow => {}
            AgentComponentKindWire::IntentClassifier => {
                agent.set_intent_router(echo_agent::intent::IntentRouter::new(
                    Box::new(proxy),
                    echo_agent::intent::IntentRouterConfig::default(),
                ));
            }
            AgentComponentKindWire::SkillLoadPolicy => {
                agent.set_skill_load_policy(Some(Arc::new(proxy)));
            }
        }
    }
    let mut guards = state
        .handles
        .extensions_of_kind(ExtensionKind::AgentComponent)
        .into_iter()
        .filter(|(_, record)| {
            matches!(
                &record.descriptor,
                ExtensionDescriptor::AgentComponent {
                    component: AgentComponentKindWire::Guard,
                    ..
                }
            )
        })
        .collect::<Vec<_>>();
    guards.sort_by_key(|(_, record)| record.registration_order);
    let guards = guards
        .into_iter()
        .filter_map(|(extension, _)| {
            ExtensionAgentComponentProxy::new(bridge.clone(), extension, session_id.to_string())
                .map(|proxy| Arc::new(proxy) as Arc<dyn echo_agent::guard::Guard>)
        })
        .collect::<Vec<_>>();
    if !guards.is_empty() {
        agent.set_guard_manager(echo_agent::guard::GuardManager::from_guards(guards));
    }
    #[cfg(feature = "framework-web")]
    if let Some((extension, _)) = state
        .handles
        .extensions_of_kind(ExtensionKind::AgentComponent)
        .into_iter()
        .filter(|(_, record)| {
            matches!(
                &record.descriptor,
                ExtensionDescriptor::AgentComponent {
                    component: AgentComponentKindWire::SearchProvider,
                    ..
                }
            )
        })
        .max_by_key(|(_, record)| record.registration_order)
        && let Some(proxy) =
            ExtensionAgentComponentProxy::new(bridge.clone(), extension, session_id.to_string())
    {
        agent.replace_tool(Box::new(echo_agent::tools::web::WebSearchTool::new(
            Box::new(proxy),
        )));
    }
    // Tools.
    for (extension, _) in state.handles.extensions_of_kind(ExtensionKind::Tool) {
        if let Some(proxy) =
            ExtensionToolProxy::new(bridge.clone(), extension, session_id.to_string())
        {
            agent.add_tool(Box::new(proxy));
        }
    }
    // Memory store: set_memory_store re-registers the remember/recall/forget
    // tools against the extension-backed store, so model-driven memory calls
    // become real reverse invocations.
    if let Some((extension, _)) = state
        .handles
        .extensions_of_kind(ExtensionKind::Store)
        .first()
        .cloned()
        && let Some(proxy) =
            ExtensionStoreProxy::new(bridge.clone(), extension, session_id.to_string())
    {
        let mut store: Arc<dyn echo_agent::memory::Store> = Arc::new(proxy);
        if let Some(embedder) = latest_agent_component_proxy(
            &state,
            bridge.clone(),
            session_id,
            AgentComponentKindWire::Embedder,
        ) {
            store = Arc::new(echo_agent::memory::EmbeddingStore::new(
                store,
                Arc::new(embedder),
            ));
        }
        agent
            .set_memory_store(store)
            .map_err(|error| error.to_string())?;
    }
    // Critic: deterministic latest-registration ownership. Installing the
    // proxy does not enable the framework verifier; that remains config-owned.
    if let Some((extension, _)) = state
        .handles
        .extensions_of_kind(ExtensionKind::Critic)
        .into_iter()
        .max_by_key(|(_, record)| record.registration_order)
        && let Some(proxy) =
            ExtensionCriticProxy::new(bridge.clone(), extension, session_id.to_string())
    {
        agent.set_critic(Arc::new(proxy));
    }
    // Human-in-the-loop provider: swap the approval channel and register the
    // appeal tool so model-initiated approvals also reach the extension.
    if let Some((extension, _)) = state
        .handles
        .extensions_of_kind(ExtensionKind::HumanLoopProvider)
        .first()
        .cloned()
        && let Some(proxy) =
            ExtensionHumanLoopProxy::new(bridge.clone(), extension, session_id.to_string())
    {
        let shared = Arc::new(proxy);
        agent.set_approval_provider(shared.clone());
        agent.add_need_appeal_tool(Box::new(
            echo_agent::tools::builtin::human_in_loop::HumanInLoop::new(shared),
        ));
    }
    // Hooks: programmatic sources in the agent's hook registry.
    {
        let registry = agent.hook_registry().clone();
        let mut registry = registry.write().await;
        for (extension, record) in state.handles.extensions_of_kind(ExtensionKind::Hook) {
            let executor = hook_executor(bridge.clone(), extension);
            let events = match &record.descriptor {
                ExtensionDescriptor::Hook { events, .. } => events
                    .iter()
                    .filter_map(|event| echo_agent::hooks::HookEvent::from_name(event))
                    .collect::<Vec<_>>(),
                _ => Vec::new(),
            };
            registry.set_programmatic_hook(&record.implementation_id, &events, executor);
        }
    }
    // Observational callbacks.
    for (extension, _) in state
        .handles
        .extensions_of_kind(ExtensionKind::AgentCallback)
    {
        if let Some(proxy) =
            ExtensionAgentCallbackProxy::new(bridge.clone(), extension, session_id.to_string())
        {
            agent.add_callback(Arc::new(proxy));
        }
    }
    // Intervention callbacks.
    for (extension, _) in state
        .handles
        .extensions_of_kind(ExtensionKind::InterventionCallback)
    {
        if let Some(proxy) =
            ExtensionInterventionProxy::new(bridge.clone(), extension, session_id.to_string())
        {
            agent.add_intervention_callback(Arc::new(proxy));
        }
    }
    // Custom agents register for subagent dispatch by name.
    for (extension, _) in state
        .handles
        .session_extensions_of_kind(ExtensionKind::CustomAgent)
    {
        if let Some(proxy) = ExtensionCustomAgentProxy::for_registration(
            bridge.clone(),
            extension,
            session_id.to_string(),
        ) {
            agent.register_agent(Box::new(proxy));
        }
    }
    // Agent factories register lazily-constructed subagents by name.
    for (extension, record) in state
        .handles
        .extensions_of_kind(ExtensionKind::AgentFactory)
    {
        agent.register_subagent_factory(
            echo_agent::agent::subagent::SubagentDefinition::simple_sync(&record.implementation_id),
            Arc::new(SubagentFactoryAdapter {
                bridge: bridge.clone(),
                extension,
                subagent_name: record.implementation_id.clone(),
                session_id: session_id.to_string(),
            }),
        );
    }
    Ok(())
}

impl ExtensionCustomAgentProxy {
    /// Build a proxy for a directly registered custom agent (no factory
    /// round-trip): identity facts come from the registration descriptor.
    pub(crate) fn for_registration(
        bridge: Arc<ExtensionBridge>,
        extension: WireHandle,
        session_id: String,
    ) -> Option<Self> {
        let record = bridge.state().ok()?.handles.extension(&extension).ok()?;
        if record.factory_instance {
            return None;
        }
        let ExtensionDescriptor::CustomAgent {
            name,
            model_name,
            system_prompt,
            tool_names,
            ..
        } = &record.descriptor
        else {
            return None;
        };
        Some(Self {
            bridge,
            extension,
            descriptor: CustomAgentDescriptorWire {
                name: name.clone(),
                model_name: model_name.clone(),
                system_prompt: system_prompt.clone(),
                tool_names: tool_names.clone(),
            },
            session_id,
            factory_lease: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "framework-channels")]
    fn channel_owner_outbound(text: &str) -> ChannelOutboundMessageWire {
        ChannelOutboundMessageWire {
            channel_id: "original-channel".to_string(),
            to: "chat".to_string(),
            chat_type: ChannelChatTypeWire::Direct,
            text: text.to_string(),
            reply_to: None,
            attachments: Vec::new(),
        }
    }

    /// Real official ACP requests, rather than a mock lease-settlement future.
    /// The remote callback sends its stream terminal before it finishes cleanup.
    #[cfg(feature = "framework-channels")]
    async fn channel_callback_owner_case(streaming: bool, mode: &str) -> Result<(), String> {
        use agent_client_protocol::{Agent as AcpAgent, ByteStreams, Error as RpcError, Responder};
        use tokio_util::compat::{TokioAsyncReadCompatExt as _, TokioAsyncWriteCompatExt as _};
        let root = tempfile::tempdir().map_err(|error| error.to_string())?;
        let config: crate::SdkHostConfig = serde_json::from_value(serde_json::json!({
            "schema_version":1,"default_agent":{
                "model":{"provider":"fixture","name":"fixture","base_url":"http://127.0.0.1:1/v1","api_protocol":"chat_completions","auth_token":"fixture"},
                "agent":{"name":"fixture","enable_tools":true}
            }
        })).map_err(|error| error.to_string())?;
        let prepared = config.prepare().map_err(|error| error.to_string())?;
        let state = CoreProfileState::install(
            root.path(),
            crate::config::SdkProfileLimits::default(),
            prepared.framework,
            prepared.llm_client,
        )
        .map_err(|error| error.to_string())?;
        let bridge = state.extension_bridge.clone();
        let authority = echo_agent::acp::ExtensionInvocationAuthority::new(4)
            .map_err(|error| format!("{error:?}"))?;
        let custom_eof = mode == "custom_eof";
        let custom_late_ack = mode == "custom_late_ack" || custom_eof;
        let remote_notice = Arc::new(Notify::new());
        let setup_answered = Arc::new(Notify::new());
        let entered = Arc::new(Notify::new());
        let release = Arc::new(Notify::new());
        let noticed = Arc::new(Notify::new());
        let callback_finished = Arc::new(AtomicBool::new(false));
        let finish = CancellationToken::new();
        let sdk_finish = finish.clone();
        let callback_finish = finish.clone();
        let (host_io, sdk_io) = tokio::io::duplex(16_384);
        let (host_read, host_write) = tokio::io::split(host_io);
        let (sdk_read, sdk_write) = tokio::io::split(sdk_io);
        let missing_terminal = mode == "missing_terminal";
        let callback_error = mode == "callback_error";
        let sdk = Client.builder()
            .on_receive_request({
                let entered = entered.clone();
                let release = release.clone();
                let callback_finished = callback_finished.clone();
                let remote_notice = remote_notice.clone();
                let setup_answered = setup_answered.clone();
                let callback_finish = callback_finish.clone();
                async move |call: ExtensionInvokeCall,
                            responder: Responder<ExtensionInvokeOutcome>,
                            connection: ConnectionTo<AcpAgent>| {
                    if matches!(&call.invocation,
                        ExtensionInvocation::ChannelReply(input) if input.message.text == "independent B") {
                        return responder.respond(ExtensionInvokeOutcome::Result {
                            result: ExtensionResult::ChannelReply(ExtensionUnit),
                        });
                    }
                    let entered = entered.clone();
                    let release = release.clone();
                    let callback_finished = callback_finished.clone();
                    let callback_connection = connection.clone();
                    let remote_notice = remote_notice.clone();
                    let setup_answered = setup_answered.clone();
                    let callback_finish = callback_finish.clone();
                    connection.spawn(async move {
                    let connection = callback_connection;
                    if custom_late_ack {
                        let stream = call.stream.ok_or_else(|| RpcError::internal_error().data("missing original stream"))?;
                        entered.notify_one();
                        if !custom_eof { remote_notice.notified().await; }
                        responder.respond(ExtensionInvokeOutcome::Stream { stream: stream.clone() })?;
                        setup_answered.notify_one();
                        release.notified().await;
                        callback_finished.store(true, Ordering::Release);
                        if custom_eof { callback_finish.cancel(); return Ok(()); }
                        return connection.send_notification(ExtensionStreamEvent::Complete {
                            stream, sequence: echo_sdk_protocol::scalar::WireNonZeroU64::try_from("1".to_string())
                                .map_err(|error| RpcError::internal_error().data(error.to_string()))?,
                            value: ExtensionStreamCompleteValue::Agent(AgentStreamTerminalWire::Cancelled),
                        });
                    }
                    let outcome = if callback_error {
                        ExtensionInvokeOutcome::Error { error: sdk_error(
                            ExtensionErrorCode::ExtensionFailed, "original callback failure", Retryability::Never,
                            "_echo_agent/extension/invoke",
                        ) }
                    } else if missing_terminal {
                        ExtensionInvokeOutcome::Stream { stream: call.stream.clone().ok_or_else(||
                            RpcError::internal_error().data("missing callback stream handle"))? }
                    } else if let Some(stream) = call.stream.clone() {
                        connection.send_notification(ExtensionStreamEvent::Chunk {
                            stream: stream.clone(),
                            sequence: echo_sdk_protocol::scalar::WireNonZeroU64::try_from("1".to_string())
                                .map_err(|error| RpcError::internal_error().data(error.to_string()))?,
                            value: ExtensionStreamChunkValue::Channel(channel_owner_outbound("first")),
                        })?;
                        connection.send_notification(ExtensionStreamEvent::Complete {
                            stream: stream.clone(),
                            sequence: echo_sdk_protocol::scalar::WireNonZeroU64::try_from("2".to_string())
                                .map_err(|error| RpcError::internal_error().data(error.to_string()))?,
                            value: ExtensionStreamCompleteValue::Channel(channel_owner_outbound("last")),
                        })?;
                        ExtensionInvokeOutcome::Stream { stream }
                    } else {
                        ExtensionInvokeOutcome::Result { result: ExtensionResult::ChannelReply(ExtensionUnit) }
                    };
                    entered.notify_one();
                    // Represents the native callback's asynchronous finally;
                    // no typed RPC answer is sent until this actual await ends.
                    release.notified().await;
                    callback_finished.store(true, Ordering::Release);
                    responder.respond(outcome)
                    })
                }
            }, agent_client_protocol::on_receive_request!())
            .on_receive_notification({
                let noticed = noticed.clone();
                let remote_notice = remote_notice.clone();
                async move |_notice: echo_sdk_protocol::methods::ExtensionCancelNotice,
                            _connection: ConnectionTo<AcpAgent>| {
                    noticed.notify_one(); remote_notice.notify_one(); Ok(())
                }
            }, agent_client_protocol::on_receive_notification!())
            .connect_with(ByteStreams::new(sdk_write.compat_write(), sdk_read.compat()), async move |_| {
                sdk_finish.cancelled().await;
                Ok(())
            });
        let host = AcpAgent
            .builder()
            .on_receive_notification(
                {
                    let shared = state.extension_shared.clone();
                    async move |event: ExtensionStreamEvent, _connection: ConnectionTo<Client>| {
                        shared
                            .deliver_stream_event(event)
                            .map_err(|error| RpcError::internal_error().data(error))
                    }
                },
                agent_client_protocol::on_receive_notification!(),
            )
            .connect_with(
                ByteStreams::new(host_write.compat_write(), host_read.compat()),
                async move |connection| {
                    bridge.shared.bind_connection(connection);
                    let ancestor = CancellationToken::new();
                    let cancellation = ancestor.child_token();
                    let lease = authority
                        .lease(None, cancellation.clone())
                        .map_err(|error| RpcError::internal_error().data(format!("{error:?}")))?;
                    let child = cancellation.clone();
                    let stream = streaming.then(|| test_stream_handle("original-channel-stream"));
                    let owner = Arc::new(CallbackInvocationOwner {
                        _stream: stream.clone().map(|stream| PendingExtensionStream {
                            bridge: bridge.as_ref().clone(),
                            stream: Some(stream),
                        }),
                        _session_callback: None,
                        _agent_invocation: None,
                        retained_lease: Mutex::new(None),
                        response_completed: CancellationToken::new(),
                        pending_failure: Mutex::new(None),
                        finished: CancellationToken::new(),
                    });
                    let (sink, receiver) = ExtensionStreamSink::new(
                        test_stream_handle("original-channel-stream"),
                        "handler".to_string(),
                        if custom_late_ack {
                            ExtensionKind::CustomAgent
                        } else {
                            ExtensionKind::ChannelMessageHandler
                        },
                        2,
                        None,
                    );
                    if custom_late_ack {
                        sink.retain_until_remote_terminal
                            .store(true, Ordering::Release);
                    }
                    if streaming {
                        bridge.shared.register_stream(sink.clone());
                    }
                    let invocation = if custom_late_ack {
                        ExtensionInvocation::AgentExecuteStream(AgentTaskInput {
                            task: "late-setup".to_string(),
                        })
                    } else if streaming {
                        ExtensionInvocation::ChannelHandleStream(ChannelHandleInput {
                            message: ChannelInboundMessageWire {
                                channel_id: "original-channel".to_string(),
                                sender_id: "sender".to_string(),
                                chat_id: "chat".to_string(),
                                chat_type: ChannelChatTypeWire::Direct,
                                text: "stream".to_string(),
                                message_id: "message".to_string(),
                                timestamp: WireU64::from_u64(1),
                                attachments: Vec::new(),
                            },
                        })
                    } else {
                        ExtensionInvocation::ChannelReply(ChannelReplyInput {
                            message: channel_owner_outbound("once"),
                        })
                    };
                    let call = ExtensionInvokeCall {
                        extension: WireHandle {
                            id: "handler".to_string(),
                            generation: WireU64::from_u64(1),
                            kind: HandleKind::Extension,
                        },
                        factory_origin: None,
                        invocation_id: lease.identity().to_string(),
                        context: None,
                        invocation,
                        deadline: wire_duration(if mode == "timeout" {
                            Duration::from_millis(50)
                        } else if mode == "answered_slow" {
                            Duration::from_millis(150)
                        } else {
                            Duration::from_secs(5)
                        }),
                        stream: stream.clone(),
                    };
                    let mut response = Some(
                        bridge
                            .submit_owned_invocation(
                                call,
                                lease,
                                cancellation.clone(),
                                if custom_late_ack {
                                    CallbackStreamPolicy::CustomAgent
                                } else {
                                    CallbackStreamPolicy::Channel
                                },
                                owner.clone(),
                                streaming.then(|| sink.clone()),
                            )
                            .map_err(|error| RpcError::internal_error().data(error.message))?,
                    );
                    let mut events = if streaming {
                        Some(channel_callback_event_stream(
                            owner.clone(),
                            CallbackStreamDropNotice {
                                cancellation: child,
                                armed: true,
                            },
                            sink.clone(),
                            receiver,
                            response.take().ok_or_else(|| {
                                RpcError::internal_error().data("missing stream response owner")
                            })?,
                        ))
                    } else {
                        None
                    };
                    // The once branch keeps its actual response receiver; the stream
                    // branch consumes that receiver through the production adapter.
                    if !missing_terminal
                        && !callback_error
                        && !custom_late_ack
                        && let Some(events) = events.as_mut()
                    {
                        let item = events
                            .next()
                            .await
                            .ok_or_else(|| RpcError::internal_error().data("missing first item"))?
                            .map_err(|error| RpcError::internal_error().data(error.to_string()))?;
                        assert_eq!(item.text, "first");
                    }
                    entered.notified().await;
                    if mode == "cancel" || (custom_late_ack && !custom_eof) {
                        cancellation.cancel();
                    }
                    if mode == "drop" {
                        drop(events.take());
                        drop(response.take());
                    }
                    if mode != "normal"
                        && mode != "answered_slow"
                        && !missing_terminal
                        && !callback_error
                        && !custom_eof
                    {
                        tokio::time::timeout(Duration::from_secs(2), noticed.notified())
                            .await
                            .map_err(|_| {
                                RpcError::internal_error().data("missing cancellation notice")
                            })?;
                    }
                    if custom_late_ack {
                        setup_answered.notified().await;
                        tokio::time::sleep(Duration::from_millis(20)).await;
                    }
                    assert!(!callback_finished.load(Ordering::Acquire));
                    assert_eq!(authority.in_flight(), 1);
                    assert!(!is_exclusive_invocation(
                        ExtensionKind::ChannelMessageHandler,
                        &ExtensionInvocation::ChannelReply(ChannelReplyInput {
                            message: channel_owner_outbound("same handler")
                        })
                    ));

                    if let Some(events) = events.as_mut() {
                        assert!(
                            tokio::time::timeout(Duration::from_millis(20), events.next())
                                .await
                                .is_err(),
                            "terminal notification must not bypass the real callback response"
                        );
                        assert!(
                            bridge
                                .shared
                                .stream_sink("original-channel-stream")
                                .is_some()
                        );
                    } else if mode != "drop" {
                        assert!(matches!(
                            response
                                .as_mut()
                                .ok_or_else(|| RpcError::internal_error()
                                    .data("missing once response owner"))?
                                .try_recv(),
                            Err(tokio::sync::oneshot::error::TryRecvError::Empty)
                        ));
                    }
                    // B settles independently while A's native finally is held.
                    let independent = authority
                        .lease(None, CancellationToken::new())
                        .map_err(|error| RpcError::internal_error().data(format!("{error:?}")))?;
                    let independent_call = ExtensionInvokeCall {
                        extension: WireHandle {
                            id: "handler".to_string(),
                            generation: WireU64::from_u64(1),
                            kind: HandleKind::Extension,
                        },
                        factory_origin: None,
                        invocation_id: independent.identity().to_string(),
                        context: None,
                        invocation: ExtensionInvocation::ChannelReply(ChannelReplyInput {
                            message: channel_owner_outbound("independent B"),
                        }),
                        deadline: wire_duration(Duration::from_secs(5)),
                        stream: None,
                    };
                    let independent_owner = Arc::new(CallbackInvocationOwner {
                        _stream: None,
                        _session_callback: None,
                        _agent_invocation: None,
                        retained_lease: Mutex::new(None),
                        response_completed: CancellationToken::new(),
                        pending_failure: Mutex::new(None),
                        finished: CancellationToken::new(),
                    });
                    let independent_response = bridge
                        .submit_owned_invocation(
                            independent_call,
                            independent,
                            CancellationToken::new(),
                            CallbackStreamPolicy::Channel,
                            independent_owner,
                            None,
                        )
                        .map_err(|error| RpcError::internal_error().data(error.message))?;
                    assert!(matches!(
                        independent_response
                            .await
                            .map_err(|error| RpcError::internal_error().data(error.to_string()))?,
                        Ok(Some(ExtensionResult::ChannelReply(_)))
                    ));
                    assert_eq!(authority.in_flight(), 1);
                    if mode == "drop" {
                        assert!(
                            !ancestor.is_cancelled(),
                            "dropping A must not cancel its parent"
                        );
                    }
                    release.notify_one();
                    if mode == "answered_slow" {
                        assert_eq!(authority.drain(Duration::from_secs(2)).await, 0);
                        tokio::time::sleep(Duration::from_millis(200)).await;
                        assert!(
                            !cancellation.is_cancelled(),
                            "callback deadline must stop at its actual RPC answer"
                        );
                    }
                    if let Some(events) = events.as_mut() {
                        let terminal =
                            tokio::time::timeout(Duration::from_millis(200), events.next())
                                .await
                                .map_err(|_| {
                                    RpcError::internal_error()
                                        .data("callback already returned but stream never ended")
                                })?
                                .ok_or_else(|| {
                                    RpcError::internal_error().data("missing final result")
                                })?;
                        if mode == "normal" || mode == "answered_slow" {
                            assert_eq!(
                                terminal
                                    .map_err(
                                        |error| RpcError::internal_error().data(error.to_string())
                                    )?
                                    .text,
                                "last"
                            );
                        } else {
                            let error = terminal.err().ok_or_else(|| {
                                RpcError::internal_error().data("missing stream failure")
                            })?;
                            if missing_terminal {
                                assert!(error.to_string().contains("without a stream terminal"));
                            }
                            if custom_eof {
                                assert!(error.to_string().contains("producer connection closed"));
                            }
                            if callback_error {
                                assert!(error.to_string().contains("original callback failure"));
                            }
                        }
                        assert!(events.next().await.is_none());
                    } else if !streaming {
                        let result = response
                            .take()
                            .ok_or_else(|| {
                                RpcError::internal_error().data("missing once response owner")
                            })?
                            .await
                            .map_err(|error| RpcError::internal_error().data(error.to_string()))?;
                        assert_eq!(
                            result.err().map(|error| error.code),
                            Some(if mode == "timeout" {
                                ExtensionErrorCode::ExtensionTimeout
                            } else {
                                ExtensionErrorCode::Cancelled
                            })
                        );
                    }
                    let remaining = authority.drain(Duration::from_secs(2)).await;
                    assert_eq!(remaining, 0);
                    assert!(callback_finished.load(Ordering::Acquire));
                    drop(owner);
                    if streaming {
                        assert!(
                            bridge
                                .shared
                                .stream_sink("original-channel-stream")
                                .is_none()
                        );
                    }
                    finish.cancel();
                    Ok(())
                },
            );
        let (host, sdk) =
            tokio::time::timeout(Duration::from_secs(10), async { tokio::join!(host, sdk) })
                .await
                .map_err(|_| "official ACP Channel owner test timed out".to_string())?;
        host.map_err(|error| error.to_string())?;
        sdk.map_err(|error| error.to_string())?;
        Ok(())
    }

    #[cfg(feature = "framework-channels")]
    #[tokio::test]
    async fn channel_callback_cancel_retains_actual_response_owner() -> Result<(), String> {
        channel_callback_owner_case(false, "cancel").await
    }

    #[cfg(feature = "framework-channels")]
    #[tokio::test]
    async fn channel_callback_timeout_retains_actual_response_owner() -> Result<(), String> {
        channel_callback_owner_case(false, "timeout").await
    }

    #[cfg(feature = "framework-channels")]
    #[tokio::test]
    async fn channel_stream_terminal_waits_for_actual_callback_return() -> Result<(), String> {
        channel_callback_owner_case(true, "normal").await
    }

    #[cfg(feature = "framework-channels")]
    #[tokio::test]
    async fn channel_stream_cancel_waits_for_actual_callback_return() -> Result<(), String> {
        channel_callback_owner_case(true, "cancel").await
    }

    #[cfg(feature = "framework-channels")]
    #[tokio::test]
    async fn channel_stream_timeout_waits_for_actual_callback_return() -> Result<(), String> {
        channel_callback_owner_case(true, "timeout").await
    }

    #[cfg(feature = "framework-channels")]
    #[tokio::test]
    async fn channel_stream_drop_retains_connection_response_owner() -> Result<(), String> {
        channel_callback_owner_case(true, "drop").await
    }

    #[cfg(feature = "framework-channels")]
    #[tokio::test]
    async fn channel_stream_answer_stops_deadline_before_slow_terminal_consumption()
    -> Result<(), String> {
        channel_callback_owner_case(true, "answered_slow").await
    }

    #[cfg(feature = "framework-channels")]
    #[tokio::test]
    async fn channel_stream_callback_return_without_terminal_fails_instead_of_hanging()
    -> Result<(), String> {
        channel_callback_owner_case(true, "missing_terminal").await
    }

    #[cfg(feature = "framework-channels")]
    #[tokio::test]
    async fn channel_stream_callback_error_keeps_original_cause_until_actual_return()
    -> Result<(), String> {
        channel_callback_owner_case(true, "callback_error").await
    }

    #[cfg(feature = "framework-channels")]
    async fn channel_once_public_helper_owner_case() -> Result<(), String> {
        use agent_client_protocol::{Agent as AcpAgent, ByteStreams, Error as RpcError, Responder};
        use tokio_util::compat::{TokioAsyncReadCompatExt as _, TokioAsyncWriteCompatExt as _};
        let root = tempfile::tempdir().map_err(|error| error.to_string())?;
        let config: crate::SdkHostConfig = serde_json::from_value(serde_json::json!({
            "schema_version":1,"default_agent":{
                "model":{"provider":"fixture","name":"fixture","base_url":"http://127.0.0.1:1/v1","api_protocol":"chat_completions","auth_token":"fixture"},
                "agent":{"name":"fixture","enable_tools":true}
            }
        })).map_err(|error| error.to_string())?;
        let prepared = config.prepare().map_err(|error| error.to_string())?;
        let state = CoreProfileState::install(
            root.path(),
            crate::config::SdkProfileLimits::default(),
            prepared.framework,
            prepared.llm_client,
        )
        .map_err(|error| error.to_string())?;
        let bridge = state.extension_bridge.clone();
        let registry = Arc::new(echo_agent::acp::SessionRegistry::new(
            Arc::new(state.session_factory.clone()),
            4,
        ));
        let services = Arc::new(echo_agent::acp::AcpConnectionServices::new(
            registry,
            Arc::new(echo_agent::acp::AcpAdapterConfig::default()),
        ));
        state.bind_services(services.clone());
        let authority = services.extensions().clone();
        let (extension, _) = state
            .handles
            .register_extension(
                super::super::handles::ExtensionRegistrationLimits {
                    max_extensions: 4,
                    max_descriptor_bytes: 65_536,
                },
                ExtensionKind::ChannelMessageHandler,
                "actual-shared-handler",
                ExtensionDescriptor::ChannelMessageHandler(
                    echo_sdk_protocol::methods::ChannelMessageHandlerDescriptorWire {
                        descriptor_version: 1,
                        handler_id: "actual-shared-handler".to_string(),
                    },
                ),
                None,
                false,
            )
            .map_err(|error| error.message)?;
        let entered = Arc::new(Notify::new());
        let release = Arc::new(Notify::new());
        let noticed = Arc::new(Notify::new());
        let callback_finished = Arc::new(AtomicBool::new(false));
        let finish = CancellationToken::new();
        let sdk_finish = finish.clone();
        let (host_io, sdk_io) = tokio::io::duplex(16_384);
        let (host_read, host_write) = tokio::io::split(host_io);
        let (sdk_read, sdk_write) = tokio::io::split(sdk_io);
        let sdk = Client.builder()
            .on_receive_request({
                let entered = entered.clone();
                let release = release.clone();
                let callback_finished = callback_finished.clone();
                async move |call: ExtensionInvokeCall,
                            responder: Responder<ExtensionInvokeOutcome>,
                            connection: ConnectionTo<AcpAgent>| {
                    if matches!(&call.invocation,
                        ExtensionInvocation::ChannelReply(input) if input.message.text == "independent B") {
                        return responder.respond(ExtensionInvokeOutcome::Result {
                            result: ExtensionResult::ChannelReply(ExtensionUnit),
                        });
                    }
                    let entered = entered.clone();
                    let release = release.clone();
                    let callback_finished = callback_finished.clone();
                    connection.spawn(async move {
                    let outcome = ExtensionInvokeOutcome::Result {
                    result: ExtensionResult::ChannelReply(ExtensionUnit),
                };
                entered.notify_one();
                    // Represents the native callback's asynchronous finally;
                    // no typed RPC answer is sent until this actual await ends.
                    release.notified().await;
                    callback_finished.store(true, Ordering::Release);
                    responder.respond(outcome)
                    })
                }
            }, agent_client_protocol::on_receive_request!())
            .on_receive_notification({
                let noticed = noticed.clone();
                async move |_notice: echo_sdk_protocol::methods::ExtensionCancelNotice,
                            _connection: ConnectionTo<AcpAgent>| {
                    noticed.notify_one(); Ok(())
                }
            }, agent_client_protocol::on_receive_notification!())
            .connect_with(ByteStreams::new(sdk_write.compat_write(), sdk_read.compat()), async move |_| {
                sdk_finish.cancelled().await;
                Ok(())
            });
        let host = AcpAgent
            .builder()
            .on_receive_notification(
                {
                    let shared = state.extension_shared.clone();
                    async move |event: ExtensionStreamEvent, _connection: ConnectionTo<Client>| {
                        shared
                            .deliver_stream_event(event)
                            .map_err(|error| RpcError::internal_error().data(error))
                    }
                },
                agent_client_protocol::on_receive_notification!(),
            )
            .connect_with(
                ByteStreams::new(host_write.compat_write(), host_read.compat()),
                async move |connection| {
                    bridge.shared.bind_connection(connection);
                    let ancestor = CancellationToken::new();
                    let cancellation = ancestor.child_token();
                    let call = ExtensionInvocation::ChannelReply(ChannelReplyInput {
                        message: channel_owner_outbound("once"),
                    });
                    let original_bridge = bridge.clone();
                    let original_extension = extension.clone();
                    let original_cancellation = cancellation.clone();
                    let mut response = tokio::spawn(async move {
                        original_bridge
                            .invoke_channel_once(&original_extension, call, original_cancellation)
                            .await
                    });
                    entered.notified().await;
                    cancellation.cancel();
                    tokio::time::timeout(Duration::from_secs(2), noticed.notified())
                        .await
                        .map_err(|_| {
                            RpcError::internal_error().data("missing cancellation notice")
                        })?;
                    assert!(!callback_finished.load(Ordering::Acquire));
                    assert_eq!(authority.in_flight(), 1);
                    assert!(!is_exclusive_invocation(
                        ExtensionKind::ChannelMessageHandler,
                        &ExtensionInvocation::ChannelReply(ChannelReplyInput {
                            message: channel_owner_outbound("same handler")
                        })
                    ));

                    assert!(
                        tokio::time::timeout(Duration::from_millis(20), &mut response)
                            .await
                            .is_err(),
                        "cancelled helper must keep its original callback until actual RPC answer"
                    );
                    let independent = bridge
                        .invoke_channel_once(
                            &extension,
                            ExtensionInvocation::ChannelReply(ChannelReplyInput {
                                message: channel_owner_outbound("independent B"),
                            }),
                            CancellationToken::new(),
                        )
                        .await
                        .map_err(|error| RpcError::internal_error().data(error.message))?;
                    assert!(matches!(independent, ExtensionResult::ChannelReply(_)));
                    assert_eq!(authority.in_flight(), 1);
                    assert!(!ancestor.is_cancelled());
                    release.notify_one();
                    let result = response
                        .await
                        .map_err(|error| RpcError::internal_error().data(error.to_string()))?;
                    assert_eq!(
                        result.err().map(|error| error.code),
                        Some(ExtensionErrorCode::Cancelled)
                    );
                    let remaining = authority.drain(Duration::from_secs(2)).await;
                    assert_eq!(remaining, 0);
                    assert!(callback_finished.load(Ordering::Acquire));
                    finish.cancel();
                    Ok(())
                },
            );
        let (host, sdk) =
            tokio::time::timeout(Duration::from_secs(10), async { tokio::join!(host, sdk) })
                .await
                .map_err(|_| "official ACP Channel owner test timed out".to_string())?;
        host.map_err(|error| error.to_string())?;
        sdk.map_err(|error| error.to_string())?;
        Ok(())
    }

    #[cfg(feature = "framework-channels")]
    #[tokio::test]
    async fn channel_once_public_helper_keeps_real_callback_after_cancel() -> Result<(), String> {
        channel_once_public_helper_owner_case().await
    }
    #[test]
    fn custom_agent_transport_failure_retains_exact_remote_terminal_observation()
    -> Result<(), String> {
        let shared = ExtensionBridgeShared::new();
        let stream = test_stream_handle("custom-actual-producer");
        let (sink, mut receiver) = ExtensionStreamSink::new(
            stream.clone(),
            "custom-original-extension".to_string(),
            ExtensionKind::CustomAgent,
            1,
            None,
        );
        sink.retain_until_remote_terminal
            .store(true, Ordering::Release);
        shared.register_stream(sink.clone());
        shared.fail_stream(&stream, "original transport payload failure");
        assert!(
            shared.stream_sink(&stream.id).is_some(),
            "failure cannot remove the only live producer address"
        );
        assert!(!sink.remote_terminal_received.load(Ordering::Acquire));
        let mut wrong = stream.clone();
        wrong.generation = WireU64::from_u64(2);
        assert!(
            shared
                .deliver_stream_event(ExtensionStreamEvent::Complete {
                    stream: wrong,
                    sequence: echo_sdk_protocol::scalar::WireNonZeroU64::try_from("1".to_string())
                        .map_err(|error| error.to_string())?,
                    value: ExtensionStreamCompleteValue::Agent(AgentStreamTerminalWire::Cancelled),
                })
                .is_err()
        );
        assert!(
            !sink.remote_terminal_received.load(Ordering::Acquire),
            "another generation cannot end this producer"
        );
        assert!(
            shared
                .deliver_stream_event(ExtensionStreamEvent::Complete {
                    stream: stream.clone(),
                    sequence: echo_sdk_protocol::scalar::WireNonZeroU64::try_from("2".to_string())
                        .map_err(|error| error.to_string())?,
                    value: ExtensionStreamCompleteValue::Agent(AgentStreamTerminalWire::Cancelled),
                })
                .is_err()
        );
        assert!(
            !sink.remote_terminal_received.load(Ordering::Acquire),
            "wrong remote sequence cannot release the owner"
        );
        // The existing ledger still rejects a terminal after the local failure;
        // the validated original producer's declaration is a distinct physical
        // observation and must not replace the already-preserved failure cause.
        assert!(
            shared
                .deliver_stream_event(ExtensionStreamEvent::Complete {
                    stream: stream.clone(),
                    sequence: echo_sdk_protocol::scalar::WireNonZeroU64::try_from("1".to_string())
                        .map_err(|error| error.to_string())?,
                    value: ExtensionStreamCompleteValue::Agent(AgentStreamTerminalWire::Cancelled),
                })
                .is_err()
        );
        assert!(sink.remote_terminal_received.load(Ordering::Acquire));
        assert!(
            shared
                .deliver_stream_event(ExtensionStreamEvent::Complete {
                    stream: stream.clone(),
                    sequence: echo_sdk_protocol::scalar::WireNonZeroU64::try_from("1".to_string())
                        .map_err(|error| error.to_string())?,
                    value: ExtensionStreamCompleteValue::Agent(AgentStreamTerminalWire::Cancelled),
                })
                .is_err(),
            "duplicate declaration cannot replace the original failure"
        );
        let terminal = receiver.try_recv().map_err(|error| error.to_string())?;
        assert!(
            matches!(terminal, ExtensionStreamEvent::Failed { error, .. }
            if error.message == "original transport payload failure")
        );
        shared.remove_stream(&stream.id);
        Ok(())
    }

    #[test]
    fn custom_agent_invocation_context_preserves_original_correlation_without_new_wire_fields() {
        let invocation = echo_agent::agent::AgentInvocationContext {
            runtime: Some(echo_agent::tools::ExternalRunContext {
                conversation_id: Some("external-eval-not-an-acp-session".to_string()),
                run_id: Some("canonical-run".to_string()),
                turn_id: Some("canonical-turn".to_string()),
                execution_id: Some("canonical-execution".to_string()),
                message_id: Some("canonical-message".to_string()),
                ..echo_agent::tools::ExternalRunContext::default()
            }),
            working_dir: Some(std::path::PathBuf::from("/原框架/workspace")),
            ..echo_agent::agent::AgentInvocationContext::default()
        };
        let context =
            custom_agent_invocation_context("original-registered-session", Some(&invocation));
        assert_eq!(
            context.session_id.as_deref(),
            Some("original-registered-session")
        );
        assert_eq!(context.run_id.as_deref(), Some("canonical-run"));
        assert_eq!(context.turn_id.as_deref(), Some("canonical-turn"));
        assert_eq!(context.execution_id.as_deref(), Some("canonical-execution"));
        assert_eq!(context.message_id.as_deref(), Some("canonical-message"));
        assert_eq!(context.call_id, None);
        assert_eq!(context.stream_id, None);
        assert_eq!(
            invocation.working_dir,
            Some(std::path::PathBuf::from("/原框架/workspace"))
        );
    }

    #[cfg(feature = "framework-channels")]
    #[tokio::test]
    async fn custom_late_setup_ack_after_cancel_waits_for_remote_terminal() -> Result<(), String> {
        channel_callback_owner_case(true, "custom_late_ack").await
    }

    #[cfg(feature = "framework-channels")]
    #[tokio::test]
    async fn custom_setup_ack_then_transport_eof_returns_disconnected_failure() -> Result<(), String>
    {
        channel_callback_owner_case(true, "custom_eof").await
    }

    async fn teardown_admitted_prefix_keeps_original_remote_watermark_case() -> Result<(), String> {
        use agent_client_protocol::{ByteStreams, Error as RpcError};
        use tokio_util::compat::{TokioAsyncReadCompatExt as _, TokioAsyncWriteCompatExt as _};
        let root = tempfile::tempdir().map_err(|error| error.to_string())?;
        let config: crate::SdkHostConfig = serde_json::from_value(serde_json::json!({
            "schema_version":1,"default_agent":{
                "model":{"provider":"fixture","name":"fixture","base_url":"http://127.0.0.1:1/v1","api_protocol":"chat_completions","auth_token":"fixture"},
                "agent":{"name":"fixture","enable_tools":true}
            }
        })).map_err(|error| error.to_string())?;
        let prepared = config.prepare().map_err(|error| error.to_string())?;
        let state = CoreProfileState::install(
            root.path(),
            crate::config::SdkProfileLimits::default(),
            prepared.framework,
            prepared.llm_client,
        )
        .map_err(|error| error.to_string())?;
        let factory = state.session_factory.clone();
        let profile = super::super::SdkCoreProfile {
            state: state.clone(),
        };
        let adapter = echo_agent::acp::AcpAgentAdapter::new(factory).with_profile(profile);
        let close_owner = adapter.close_owner();
        let (host_io, sdk_io) = tokio::io::duplex(16_384);
        let (host_read, host_write) = tokio::io::split(host_io);
        let (sdk_read, sdk_write) = tokio::io::split(sdk_io);
        let host = agent_client_protocol::ConnectTo::connect_to(
            adapter,
            ByteStreams::new(host_write.compat_write(), host_read.compat()),
        );
        let client = Client.builder().connect_with(
            ByteStreams::new(sdk_write.compat_write(), sdk_read.compat()),
            async move |connection| {
                let mut initialize = agent_client_protocol::schema::v1::InitializeRequest::new(
                    agent_client_protocol::schema::ProtocolVersion::V1,
                );
                let mut meta = agent_client_protocol::schema::v1::Meta::new();
                meta.insert(
                    "echo_agent".to_string(),
                    serde_json::to_value(echo_sdk_protocol::capability::EchoAgentClientHello {
                        extension_protocol_version: echo_sdk_protocol::EXTENSION_PROTOCOL_VERSION,
                        contract_digest: state.advertisement.contract_digest.clone(),
                        source_contract_digest: state.advertisement.source_contract_digest.clone(),
                        required_features: Vec::new(),
                        required_capabilities: Vec::new(),
                    })
                    .map_err(|error| RpcError::internal_error().data(error.to_string()))?,
                );
                initialize.client_capabilities.meta = Some(meta);
                connection.send_request(initialize).block_task().await?;
                let services = state
                    .services()
                    .map_err(|error| RpcError::internal_error().data(error.to_string()))?;
                assert!(
                    services.is_extended().await,
                    "official initialize must negotiate real Extended mode"
                );
                let (extension, _) = state
                    .handles
                    .register_extension(
                        super::super::handles::ExtensionRegistrationLimits {
                            max_extensions: 4,
                            max_descriptor_bytes: 65_536,
                        },
                        ExtensionKind::ChannelMessageHandler,
                        "teardown-original-handler",
                        ExtensionDescriptor::ChannelMessageHandler(
                            echo_sdk_protocol::methods::ChannelMessageHandlerDescriptorWire {
                                descriptor_version: 1,
                                handler_id: "teardown-original-handler".to_string(),
                            },
                        ),
                        None,
                        false,
                    )
                    .map_err(|error| RpcError::internal_error().data(error.message))?;
                let stream = state
                    .handles
                    .register_extension_stream(&extension.id)
                    .map_err(|error| RpcError::internal_error().data(error.message))?;
                let (sink, mut receiver) = ExtensionStreamSink::new(
                    stream.clone(),
                    extension.id.clone(),
                    ExtensionKind::ChannelMessageHandler,
                    2,
                    None,
                );
                sink.retain_until_remote_terminal
                    .store(true, Ordering::Release);
                state.extension_shared.register_stream(sink.clone());
                services.close_admission();
                assert!(services.ensure_admission().is_err());
                // These two notifications traverse the actual Host handler,
                // not shared.deliver directly; only the original admitted
                // identity may advance this receive ledger after close.
                super::super::handler::extension_stream(
                    state.clone(),
                    ExtensionStreamEvent::Chunk {
                        stream: stream.clone(),
                        sequence: echo_sdk_protocol::scalar::WireNonZeroU64::try_from(
                            "1".to_string(),
                        )
                        .map_err(|error| RpcError::internal_error().data(error.to_string()))?,
                        value: ExtensionStreamChunkValue::Channel(channel_owner_outbound(
                            "queued-before-cancel",
                        )),
                    },
                )
                .await
                .map_err(|error| RpcError::internal_error().data(error))?;
                super::super::handler::extension_stream(
                    state.clone(),
                    ExtensionStreamEvent::Complete {
                        stream: stream.clone(),
                        sequence: echo_sdk_protocol::scalar::WireNonZeroU64::try_from(
                            "2".to_string(),
                        )
                        .map_err(|error| RpcError::internal_error().data(error.to_string()))?,
                        value: ExtensionStreamCompleteValue::Channel(channel_owner_outbound(
                            "real-end",
                        )),
                    },
                )
                .await
                .map_err(|error| RpcError::internal_error().data(error))?;
                assert!(
                    sink.remote_terminal_received.load(Ordering::Acquire),
                    "admitted prefix must advance the original remote watermark before terminal"
                );
                assert!(matches!(
                    receiver.try_recv(),
                    Ok(ExtensionStreamEvent::Chunk { .. })
                ));
                assert!(matches!(
                    receiver.try_recv(),
                    Ok(ExtensionStreamEvent::Complete { .. })
                ));
                state.extension_shared.remove_stream(&stream.id);
                state.handles.remove_extension_stream(&stream.id);
                Ok(())
            },
        );
        let (host, client) =
            tokio::time::timeout(Duration::from_secs(5), async { tokio::join!(host, client) })
                .await
                .map_err(|_| "official teardown prefix scenario timed out".to_string())?;
        host.map_err(|error| error.to_string())?;
        client.map_err(|error| error.to_string())?;
        drop(close_owner);
        Ok(())
    }

    #[cfg(feature = "framework-channels")]
    #[tokio::test]
    async fn teardown_admitted_prefix_keeps_original_remote_watermark() -> Result<(), String> {
        teardown_admitted_prefix_keeps_original_remote_watermark_case().await
    }
    fn test_stream_handle(id: &str) -> WireHandle {
        WireHandle {
            id: id.to_string(),
            generation: WireU64::from_u64(1),
            kind: HandleKind::Stream,
        }
    }

    #[test]
    fn dropped_pending_stream_releases_sink_and_session_reentry_guard() -> Result<(), String> {
        let shared = Arc::new(ExtensionBridgeShared::new());
        let bridge = ExtensionBridge::unbound(shared.clone());
        let stream = test_stream_handle("pending-stream");
        let guard = shared.enter_session_callback(Some("pending-session"));
        let (sink, _receiver) = ExtensionStreamSink::new(
            stream.clone(),
            "extension".to_string(),
            ExtensionKind::LlmClient,
            1,
            guard,
        );
        shared.register_stream(sink);
        assert!(shared.has_active_session_callback("pending-session"));
        drop(PendingExtensionStream {
            bridge,
            stream: Some(stream.clone()),
        });
        assert!(shared.stream_sink(&stream.id).is_none());
        assert!(!shared.has_active_session_callback("pending-session"));
        Ok(())
    }

    #[tokio::test]
    async fn connection_teardown_aborts_owned_factory_cleanup_jobs() -> Result<(), String> {
        let shared = ExtensionBridgeShared::new();
        let blocked = Arc::new(Notify::new());
        let task = tokio::spawn({
            let blocked = blocked.clone();
            async move {
                blocked.notified().await;
            }
        });
        let abort = task.abort_handle();
        shared.track_factory_cleanup(task);
        shared.close_all_streams();
        tokio::task::yield_now().await;
        assert!(abort.is_finished());
        assert!(
            shared
                .factory_cleanup_tasks
                .lock()
                .unwrap_or_else(|error| error.into_inner())
                .is_empty()
        );
        let task = tokio::spawn(async move {
            blocked.notified().await;
        });
        let abort = task.abort_handle();
        shared.track_factory_cleanup(task);
        tokio::task::yield_now().await;
        assert!(
            abort.is_finished(),
            "late cleanup admission must obey the connection closed fence"
        );
        Ok(())
    }

    #[test]
    fn context_compressor_proxy_preserves_descriptor_name() {
        let proxy = ExtensionContextCompressorProxy {
            bridge: Arc::new(ExtensionBridge::unbound(Arc::new(
                ExtensionBridgeShared::new(),
            ))),
            extension: WireHandle {
                id: "compressor".to_string(),
                generation: WireU64::from_u64(1),
                kind: HandleKind::Extension,
            },
            session_id: "session".to_string(),
            name: "sdk-compressor".to_string(),
        };
        assert_eq!(
            echo_agent::compression::ContextCompressor::name(&proxy),
            "sdk-compressor"
        );
    }

    #[test]
    fn agent_component_exclusivity_is_operation_scoped() {
        assert!(!is_exclusive_invocation(
            ExtensionKind::AgentFactory,
            &ExtensionInvocation::FactoryCreateAgent(AgentFactoryConfigWire {
                instance: WireHandle {
                    id: "reserved".to_string(),
                    generation: WireU64::from_u64(1),
                    kind: HandleKind::Extension
                },
                model: String::new(),
                name: "independent".to_string(),
                system_prompt: String::new(),
                tool_count: WireU64::from_u64(0),
            })
        ));
        let audit = ExtensionInvocation::AgentComponentCall(AgentComponentCallWire {
            component: AgentComponentKindWire::AuditLogger,
            call: AgentComponentCallInputWire::AuditLog {
                event: WireValue::Null,
            },
        });
        assert!(!is_exclusive_invocation(
            ExtensionKind::AgentComponent,
            &audit
        ));

        let workflow = ExtensionInvocation::AgentComponentCall(AgentComponentCallWire {
            component: AgentComponentKindWire::Workflow,
            call: AgentComponentCallInputWire::WorkflowRun {
                input: "run".to_string(),
            },
        });
        assert!(is_exclusive_invocation(
            ExtensionKind::AgentComponent,
            &workflow
        ));

        let workflow_stream =
            ExtensionInvocation::AgentComponentCallStream(AgentComponentCallWire {
                component: AgentComponentKindWire::Workflow,
                call: AgentComponentCallInputWire::WorkflowRunStream {
                    input: "stream".to_string(),
                },
            });
        assert!(is_exclusive_invocation(
            ExtensionKind::AgentComponent,
            &workflow_stream
        ));
    }

    #[test]
    fn reverse_errors_preserve_cancellation_and_tool_timeout_domains() {
        let cancelled = sdk_error(
            ExtensionErrorCode::Cancelled,
            "cancel",
            Retryability::Never,
            "extension/invoke",
        );
        assert_eq!(
            echo_agent::error::AgentFailure::from(&react_error(cancelled.clone())).terminal_kind,
            echo_agent::error::AgentTerminalKind::Cancelled
        );
        let tool = tool_react_error(cancelled);
        let failure = echo_agent::tools::ToolFailure::from_error(&tool, true);
        assert_eq!(
            failure.category,
            echo_agent::tools::ToolFailureCategory::Cancelled
        );
        assert_eq!(
            failure.side_effect,
            echo_agent::tools::ToolSideEffect::Possible
        );
        let timeout = tool_react_error(sdk_error(
            ExtensionErrorCode::ExtensionTimeout,
            "timeout",
            Retryability::Never,
            "extension/invoke",
        ));
        assert_eq!(
            echo_agent::tools::ToolFailure::from_error(&timeout, true).category,
            echo_agent::tools::ToolFailureCategory::Timeout
        );
    }

    fn test_sequence(
        value: u64,
    ) -> std::result::Result<echo_sdk_protocol::scalar::WireNonZeroU64, String> {
        echo_sdk_protocol::scalar::WireNonZeroU64::try_from(value.to_string())
            .map_err(|error| error.to_string())
    }

    fn tool_progress(
        stream: &WireHandle,
        sequence: u64,
    ) -> std::result::Result<ExtensionStreamEvent, String> {
        Ok(ExtensionStreamEvent::Chunk {
            stream: stream.clone(),
            sequence: test_sequence(sequence)?,
            value: ExtensionStreamChunkValue::Tool(ToolStreamChunkWire::Progress {
                message: format!("chunk {sequence}"),
                percent: None,
            }),
        })
    }

    #[test]
    fn chat_chunk_wire_round_trips_fields() -> Result<(), String> {
        let chunk = echo_agent::llm::ChatChunk {
            delta: echo_agent::llm::types::DeltaMessage {
                role: Some("assistant".to_string()),
                content: Some("hello".to_string()),
                reasoning_content: None,
                reasoning_blocks: None,
                tool_calls: None,
            },
            finish_reason: Some("stop".to_string()),
            usage: None,
        };
        let wire = chat_chunk_wire(&chunk)?;
        let restored = chat_chunk_from_wire(wire)?;
        assert_eq!(restored.delta.content.as_deref(), Some("hello"));
        assert_eq!(restored.finish_reason.as_deref(), Some("stop"));
        Ok(())
    }

    #[cfg(unix)]
    #[test]
    fn tool_context_keeps_non_utf8_working_directory_lossless() -> Result<(), String> {
        use std::os::unix::ffi::OsStringExt as _;

        let path =
            std::path::PathBuf::from(std::ffi::OsString::from_vec(b"/tmp/bridge-\xff".to_vec()));
        let context = ToolContext {
            working_dir: Some(path.clone()),
            ..ToolContext::default()
        };
        let wire = tool_context_wire(&context)?;
        let Some(WirePath::Unix { bytes_base64, .. }) = wire.working_dir else {
            return Err("unix paths must use the unix wire encoding".to_string());
        };
        let decoded = base64::engine::general_purpose::STANDARD_NO_PAD
            .decode(bytes_base64)
            .map_err(|error| error.to_string())?;
        use std::os::unix::ffi::OsStrExt as _;
        assert_eq!(decoded, path.as_os_str().as_bytes());
        Ok(())
    }

    #[test]
    fn approval_scope_maps_all_closed_variants() {
        assert_eq!(
            scope_from_wire(ApprovalScopeWire::Once),
            echo_agent::human_loop::ApprovalScope::Once
        );
        assert_eq!(
            scope_from_wire(ApprovalScopeWire::Session),
            echo_agent::human_loop::ApprovalScope::Session
        );
        assert_eq!(
            scope_from_wire(ApprovalScopeWire::SessionTool),
            echo_agent::human_loop::ApprovalScope::SessionTool
        );
    }

    #[tokio::test]
    async fn duplicate_stream_sequence_settles_a_failed_terminal() -> Result<(), String> {
        let shared = ExtensionBridgeShared::new();
        let stream = test_stream_handle("stream-duplicate");
        let (sink, mut receiver) = ExtensionStreamSink::new(
            stream.clone(),
            "extension-1".to_string(),
            ExtensionKind::Tool,
            STREAM_CHANNEL_CAPACITY,
            None,
        );
        shared.register_stream(sink);

        shared.deliver_stream_event(tool_progress(&stream, 1)?)?;
        assert!(
            shared
                .deliver_stream_event(tool_progress(&stream, 1)?)
                .is_err()
        );
        assert!(matches!(
            receiver.recv().await,
            Some(ExtensionStreamEvent::Chunk { .. })
        ));
        assert!(matches!(
            receiver.recv().await,
            Some(ExtensionStreamEvent::Failed { .. })
        ));
        assert!(shared.stream_sink(&stream.id).is_none());
        Ok(())
    }

    #[test]
    fn full_stream_mailbox_retains_a_failed_terminal_without_a_gap() -> Result<(), String> {
        let shared = ExtensionBridgeShared::new();
        let stream = test_stream_handle("stream-backpressure");
        let (sink, _receiver) = ExtensionStreamSink::new(
            stream.clone(),
            "extension-1".to_string(),
            ExtensionKind::Tool,
            STREAM_CHANNEL_CAPACITY,
            None,
        );
        shared.register_stream(sink.clone());

        for sequence in 1..=STREAM_CHANNEL_CAPACITY {
            let sequence = u64::try_from(sequence).map_err(|error| error.to_string())?;
            shared.deliver_stream_event(tool_progress(&stream, sequence)?)?;
        }
        let failed_sequence = u64::try_from(STREAM_CHANNEL_CAPACITY)
            .map_err(|error| error.to_string())?
            .saturating_add(1);
        assert!(
            shared
                .deliver_stream_event(tool_progress(&stream, failed_sequence)?)
                .is_err()
        );
        assert!(matches!(
            sink.take_terminal(),
            Some(ExtensionStreamEvent::Failed { sequence, .. })
                if sequence.to_u64() == Some(failed_sequence)
        ));
        assert!(shared.stream_sink(&stream.id).is_none());
        Ok(())
    }

    #[test]
    fn every_public_agent_event_variant_round_trips() -> Result<(), String> {
        let events = vec![
            AgentEvent::Token("token".to_string()),
            AgentEvent::ThinkStart,
            AgentEvent::ThinkEnd {
                prompt_tokens: 2,
                completion_tokens: 3,
            },
            AgentEvent::LlmUsage {
                model: "fixture-model".to_string(),
                prompt_tokens: 2,
                completion_tokens: 3,
                total_tokens: 5,
                cached_prompt_tokens: 1,
                cache_creation_prompt_tokens: 1,
                usage_reported: true,
            },
            AgentEvent::BudgetDecision {
                decision: echo_agent::agent::BudgetDecision::WindDown,
                reason: "near_limit".to_string(),
                iteration: 4,
                reported_model_tokens: 5,
                usage_complete: true,
            },
            AgentEvent::ToolCall {
                call_id: "call-1".to_string(),
                invocation: echo_agent::agent::ToolInvocation {
                    requested_name: "search".to_string(),
                    requested_args: serde_json::json!({"query": "bridge"}),
                    name: "search".to_string(),
                    args: serde_json::json!({"query": "bridge"}),
                    rewrites: Vec::new(),
                },
            },
            AgentEvent::ToolResult {
                call_id: "call-1".to_string(),
                name: "search".to_string(),
                result: ToolResult::success("found"),
            },
            AgentEvent::ToolStream {
                call_id: "call-1".to_string(),
                name: "search".to_string(),
                event: ToolStreamEvent::Progress {
                    message: "halfway".to_string(),
                    percent: Some(50),
                },
            },
            AgentEvent::ToolBatchStart { tool_count: 1 },
            AgentEvent::ToolBatchEnd,
            AgentEvent::GuardTriggered {
                guard: "fixture".to_string(),
                blocked: true,
            },
            AgentEvent::MemoryRecalled { count: 2 },
            AgentEvent::TranscriptProjectionSettlement(
                echo_agent::memory::TranscriptProjectionSettlement {
                    status: echo_agent::memory::TranscriptProjectionSettlementStatus::Deferred,
                    operation_id: Some("operation-中文🙂".to_string()),
                    conversation_id: Some("conversation".to_string()),
                    generation_id: Some("generation".to_string()),
                    attempt: 7,
                    error_class: Some(
                        echo_agent::memory::TranscriptProjectionErrorClass::OutcomeUnknown,
                    ),
                    detail: Some("committed response lost".to_string()),
                },
            ),
            AgentEvent::ContextCompressed {
                before_count: 4,
                after_count: 2,
                before_tokens: 40,
                after_tokens: 20,
            },
            AgentEvent::Chart {
                spec: serde_json::json!({"mark": "bar"}),
            },
            AgentEvent::Error {
                source: "fixture".to_string(),
                message: "failed".to_string(),
                failure: echo_agent::error::AgentFailure {
                    category: echo_agent::error::AgentFailureCategory::Other,
                    terminal_kind: echo_agent::error::AgentTerminalKind::Failed,
                    retryable: false,
                    code: "fixture".to_string(),
                    http_status: None,
                    message: "failed".to_string(),
                },
            },
            AgentEvent::SafetyNotice {
                action: "write".to_string(),
                reason: "fixture".to_string(),
                risk: "low".to_string(),
                permission: "default".to_string(),
            },
            AgentEvent::ParameterError {
                tool: "search".to_string(),
                parameter: "query".to_string(),
                expected: "string".to_string(),
                got: "number".to_string(),
            },
            AgentEvent::FinalAnswer("done".to_string()),
            AgentEvent::Cancelled,
        ];

        for event in events {
            let expected = serde_json::to_value(&event).map_err(|error| error.to_string())?;
            let wire = agent_event_wire_from_framework(&event)?;
            let restored = agent_event_from_wire(wire)?;
            let actual = serde_json::to_value(restored).map_err(|error| error.to_string())?;
            assert_eq!(actual, expected);
        }
        Ok(())
    }

    #[test]
    fn custom_agent_tool_stream_chunk_stays_non_terminal() -> Result<(), String> {
        let event = agent_stream_chunk_from_wire(AgentStreamChunkWire::ToolStream {
            call_id: "call-1".to_string(),
            name: "search".to_string(),
            event: ToolStreamChunkWire::Progress {
                message: "halfway".to_string(),
                percent: Some(50),
            },
        })?;
        assert!(matches!(
            event,
            AgentEvent::ToolStream {
                event: ToolStreamEvent::Progress {
                    percent: Some(50),
                    ..
                },
                ..
            }
        ));
        Ok(())
    }
}

#[cfg(all(test, feature = "framework-channels"))]
mod channel_forward_conversion_tests {
    use super::*;

    fn inbound() -> serde_json::Value {
        serde_json::json!({"channel_id":"通道🙂", "sender_id":"sender中文", "chat_id":"chat🧪", "chat_type":"group", "text":"输入 中文🙂", "message_id":"消息", "timestamp":"18446744073709551615"})
    }

    #[test]
    fn channel_inbound_maximum_timestamp_and_attachment_bytes_are_lossless()
    -> Result<(), Box<dyn std::error::Error>> {
        let mut input = inbound();
        input["attachments"] = serde_json::json!([
            {"kind":"image","data_base64":"AP9BgA==","filename":"文件🙂"},
            {"kind":"file","data_base64":"AP9BgA==","filename":""},
            {"kind":"audio","data_base64":"AP9BgA==","filename":null},
            {"kind":"video","data_base64":"AP9BgA=="}
        ]);
        let wire: ChannelInboundMessageWire = serde_json::from_value(input)?;
        let native = channel_inbound_from_wire(wire)?;
        assert_eq!(native.timestamp, u64::MAX);
        assert_eq!(native.text, "输入 中文🙂");
        for attachment in &native.attachments {
            assert_eq!(attachment.data, vec![0, 255, 65, 128]);
        }
        let roundtrip = serde_json::to_value(channel_inbound_to_wire(&native))?;
        assert_eq!(
            roundtrip.get("timestamp"),
            Some(&serde_json::json!("18446744073709551615"))
        );
        let names = native
            .attachments
            .iter()
            .map(|attachment| attachment.filename.as_deref())
            .collect::<Vec<_>>();
        assert_eq!(names, vec![Some("文件🙂"), Some(""), None, None]);
        Ok(())
    }

    #[test]
    fn channel_inbound_rejects_noncanonical_required_fields_before_forward()
    -> Result<(), Box<dyn std::error::Error>> {
        for timestamp in [
            serde_json::json!(0),
            serde_json::json!(-1),
            serde_json::json!(null),
            serde_json::json!("01"),
            serde_json::json!("18446744073709551616"),
        ] {
            let mut input = inbound();
            input["timestamp"] = timestamp;
            assert!(serde_json::from_value::<ChannelInboundMessageWire>(input).is_err());
        }
        for field in [
            "channel_id",
            "sender_id",
            "chat_id",
            "chat_type",
            "text",
            "message_id",
            "timestamp",
        ] {
            let mut input = inbound();
            if let Some(object) = input.as_object_mut() {
                object.remove(field);
            }
            assert!(serde_json::from_value::<ChannelInboundMessageWire>(input).is_err());
        }
        let mut input = inbound();
        input["sender"] = serde_json::json!("invented");
        assert!(serde_json::from_value::<ChannelInboundMessageWire>(input).is_err());
        let mut input = inbound();
        input["attachments"] = serde_json::Value::Null;
        assert!(serde_json::from_value::<ChannelInboundMessageWire>(input).is_err());
        for (kind, data) in [("unknown", "AP9BgA=="), ("image", "invalid base64!")] {
            let mut input = inbound();
            input["attachments"] = serde_json::json!([{"kind":kind,"data_base64":data}]);
            let wire: ChannelInboundMessageWire = serde_json::from_value(input)?;
            assert!(channel_inbound_from_wire(wire).is_err());
        }
        Ok(())
    }
}
