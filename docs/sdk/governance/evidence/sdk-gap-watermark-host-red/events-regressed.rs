//! Bounded live event delivery for negotiated streams (supreme plan 05,
//! todo `deliver-events-replay-recovery`).
//!
//! Contract: every committed envelope is delivered live while the ACK
//! window is open. When the outstanding count/bytes bound is reached, the
//! stream sends exactly one [`GapNotification`], pauses live delivery, and
//! the Client recovers through bounded `_echo_agent/run/replay` plus
//! `_echo_agent/event/ack`. Host memory stays bounded because live delivery
//! is admission-controlled — the durable journal is the authority, not the
//! outgoing queue. A single event whose serialized size exceeds the live
//! event bound is announced as a one-sequence gap (it remains in the journal
//! for snapshot/watermark semantics) instead of failing the run.

use agent_client_protocol::{Client, ConnectionTo};
use echo_agent::acp::RunEventObserver;
use echo_agent::agent::EventEnvelope;
use echo_agent::error::{ReactError, Result as EchoResult};
use echo_agent::state::journal::EventJournal;
use echo_sdk_protocol::error::{EchoSdkError, Retryability};
use echo_sdk_protocol::event::{EventAck, EventNotification, GapNotification};
use echo_sdk_protocol::scalar::WireNonZeroU64;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::sync::Mutex as AsyncMutex;

use super::wire::{sdk_error, wire_envelope};

/// Reasons sent inside gap notifications. Stable diagnostic strings, not
/// error codes.
pub(crate) const GAP_REASON_ACK_WINDOW: &str = "ack window exceeded; live delivery paused";
pub(crate) const GAP_REASON_EVENT_TOO_LARGE: &str = "event exceeds the live delivery bound";

/// Shared delivery state of one stream.
struct DeliveryInner {
    /// Sent-but-unacknowledged `(sequence, serialized bytes)` FIFO.
    unacked: VecDeque<(u64, usize)>,
    outstanding_bytes: usize,
    paused: bool,
    /// Watermark of the outstanding gap, independent of replay ACK coverage.
    paused_sequence: u64,
    last_sent_sequence: u64,
    /// Highest sequence covered by an enqueued event, replay page or gap. A gap's
    /// snapshot watermark is ACK-able without sending its covered event live.
    ackable_sequence: u64,
}

impl DeliveryInner {
    fn outstanding(&self) -> usize {
        self.unacked.len()
    }

    fn acknowledge(&mut self, acked: u64, max_outstanding: usize) -> EchoResult<Option<u64>> {
        if acked > self.ackable_sequence {
            return Err(ReactError::Other(
                "acknowledgement is ahead of the announced event or gap watermark".to_string(),
            ));
        }
        let mut released_bytes = 0usize;
        while let Some((sequence, bytes)) = self.unacked.front() {
            if *sequence > acked {
                break;
            }
            released_bytes = released_bytes.saturating_add(*bytes);
            self.unacked.pop_front();
        }
        self.outstanding_bytes = self.outstanding_bytes.saturating_sub(released_bytes);
        // An accepted snapshot ACK covers journal facts just like a processed
        // live event. Late/duplicate ACKs must never move this cursor backwards.
        // A gap is recovered by ACKing its watermark or a later consumed replay
        // page. Earlier live ACKs cannot retract the already announced gap.
        if self.paused && acked >= self.paused_sequence && self.outstanding() < max_outstanding {
            self.paused = false;
            self.paused_sequence = 0;
            Ok(Some(self.last_sent_sequence))
        } else {
            Ok(None)
        }
    }

    fn publish_replay<T, E>(
        &mut self,
        response: echo_sdk_protocol::event::ReplayResponse,
        send: impl FnOnce(echo_sdk_protocol::event::ReplayResponse) -> std::result::Result<T, E>,
    ) -> std::result::Result<T, E> {
        // An empty page echoes the caller's cursor. It is never evidence of
        // delivery. Only returned journal envelopes or a retention gap prove
        // coverage; publication failure grants no new ACK permission.
        let coverage = response
            .events
            .last()
            .and_then(|event| event.sequence.to_u64())
            .or_else(|| {
                response
                    .gap
                    .as_ref()
                    .and_then(|gap| gap.snapshot_watermark.to_u64())
            });
        let result = send(response)?;
        if let Some(coverage) = coverage {
            self.ackable_sequence = self.ackable_sequence.max(coverage);
        }
        Ok(result)
    }

    /// Enqueue first, then commit transport accounting. The enclosing stream
    /// serializes this whole operation; failed event/gap sends therefore leave
    /// the ACK cursor, outstanding budget and pause state exactly unchanged.
    fn publish(
        &mut self,
        sequence: u64,
        bytes: usize,
        max_outstanding: usize,
        max_buffer_bytes: usize,
        max_event_bytes: usize,
        send: impl FnOnce(&Reserve) -> EchoResult<()>,
    ) -> EchoResult<Reserve> {
        if sequence <= self.last_sent_sequence {
            return Ok(Reserve::AlreadySent);
        }
        if self.paused {
            return Ok(Reserve::AlreadyPaused);
        }
        let next_bytes = self
            .outstanding_bytes
            .checked_add(bytes)
            .ok_or_else(|| ReactError::Other("live window byte accounting overflow".to_string()))?;
        let next_outstanding = self
            .outstanding()
            .checked_add(1)
            .ok_or_else(|| ReactError::Other("live window count overflow".to_string()))?;
        let reservation = if bytes > max_event_bytes {
            Reserve::SkipOversized
        } else if next_outstanding > max_outstanding || next_bytes > max_buffer_bytes {
            let from_sequence = self
                .last_sent_sequence
                .checked_add(1)
                .ok_or_else(|| ReactError::Other("live sequence overflow".to_string()))?;
            Reserve::GapAndPause {
                from_sequence,
                to_sequence: sequence.max(from_sequence),
                snapshot_watermark: sequence.max(from_sequence),
            }
        } else {
            Reserve::Deliver
        };
        send(&reservation)?;
        match reservation {
            Reserve::Deliver => {
                self.unacked.push_back((sequence, bytes));
                self.outstanding_bytes = next_bytes;
                self.last_sent_sequence = sequence;
                self.ackable_sequence = self.ackable_sequence.max(sequence);
            }
            Reserve::GapAndPause {
                snapshot_watermark, ..
            } => {
                self.paused = true;
                self.paused_sequence = snapshot_watermark;
                self.ackable_sequence = self.ackable_sequence.max(snapshot_watermark);
            }
            Reserve::SkipOversized => {
                self.paused = true;
                self.paused_sequence = sequence;
                self.ackable_sequence = self.ackable_sequence.max(sequence);
            }
            Reserve::AlreadyPaused | Reserve::AlreadySent => {}
        }
        Ok(reservation)
    }
}

/// Live delivery of one run's committed events, gated by the negotiated ACK
/// window. The observer is registered on the shared run sink, so it observes
/// exactly the envelopes the ledger committed, in order.
pub(crate) struct StreamDelivery {
    state: Arc<super::state::CoreProfileState>,
    run_id: String,
    connection: ConnectionTo<Client>,
    stream_handle: echo_sdk_protocol::handle::WireHandle,
    inner: Mutex<DeliveryInner>,
    send_serial: AsyncMutex<()>,
    max_outstanding: usize,
    max_buffer_bytes: usize,
    max_event_bytes: usize,
}

impl StreamDelivery {
    pub(crate) fn new(
        state: Arc<super::state::CoreProfileState>,
        run_id: &str,
        connection: ConnectionTo<Client>,
        stream_handle: echo_sdk_protocol::handle::WireHandle,
        max_outstanding: usize,
        max_buffer_bytes: usize,
        max_event_bytes: usize,
    ) -> Arc<Self> {
        Arc::new(Self {
            state,
            run_id: run_id.to_string(),
            connection,
            stream_handle,
            inner: Mutex::new(DeliveryInner {
                unacked: VecDeque::new(),
                outstanding_bytes: 0,
                paused: false,
                paused_sequence: 0,
                last_sent_sequence: 0,
                ackable_sequence: 0,
            }),
            send_serial: AsyncMutex::new(()),
            max_outstanding,
            max_buffer_bytes,
            max_event_bytes,
        })
    }

    pub(crate) fn stream_handle(&self) -> &echo_sdk_protocol::handle::WireHandle {
        &self.stream_handle
    }

    pub(crate) fn run_id(&self) -> &str {
        &self.run_id
    }

    /// Atomically enqueue a validated replay page and expose only its actual
    /// journal coverage to ACK admission. Shares the live/ACK serialization
    /// lock so a client ACK cannot race ahead of the successful response.
    pub(crate) async fn publish_replay(
        &self,
        stream: &echo_sdk_protocol::handle::WireHandle,
        response: echo_sdk_protocol::event::ReplayResponse,
        send: impl FnOnce(
            echo_sdk_protocol::event::ReplayResponse,
        ) -> std::result::Result<(), agent_client_protocol::Error>,
    ) -> std::result::Result<(), agent_client_protocol::Error> {
        let _serial = self.send_serial.lock().await;
        if stream != &self.stream_handle || response.next_cursor.stream_id != self.stream_handle.id
        {
            return Err(agent_client_protocol::Error::internal_error()
                .data("replay stream identity or generation mismatch"));
        }
        response.validate().map_err(|error| {
            agent_client_protocol::Error::internal_error().data(error.to_string())
        })?;
        let mut inner = self.inner.lock().map_err(|_| {
            agent_client_protocol::Error::internal_error()
                .data("stream delivery state lock poisoned")
        })?;
        inner.publish_replay(response, send)
    }

    /// Acknowledge one contiguous cursor: retire outstanding entries, and —
    /// if delivery was paused — resume by flushing the events the Client has
    /// not yet seen (bounded by the window, from the durable journal).
    pub(crate) async fn acknowledge(self: &Arc<Self>, ack: &EventAck) -> EchoResult<()> {
        let _serial = self.send_serial.lock().await;
        if ack.stream != self.stream_handle {
            return Err(ReactError::Other(
                "ACK stream identity or generation mismatch".to_string(),
            ));
        }
        let acked = ack
            .last_processed_sequence
            .to_u64()
            .ok_or_else(|| ReactError::Other("acknowledgement sequence is invalid".to_string()))?;
        let resume_from = self
            .inner
            .lock()
            .map_err(|_| lock_poisoned())?
            .acknowledge(acked, self.max_outstanding)?;
        let Some(after_sequence) = resume_from else {
            return Ok(());
        };
        let resumed = self.resume_after(after_sequence);
        if resumed.is_err() {
            // Preserve the accepted ACK, but allow the same ACK to retry a
            // failed journal read or transport enqueue. The journal remains
            // the recovery authority; failed publication advances no cursor.
            let mut inner = self.inner.lock().map_err(|_| lock_poisoned())?;
            inner.paused = true;
            inner.paused_sequence = inner.last_sent_sequence;
        }
        resumed
    }

    fn resume_after(&self, after_sequence: u64) -> EchoResult<()> {
        let journal = self.state.journal(&self.run_id)?;
        let records = journal
            // One lookahead record lets the existing admission check announce a
            // new gap when replay fills the window. Without it the stream
            // looks unpaused and a later event ACK cannot drain durable tail.
            .replay_after(after_sequence, self.max_outstanding.saturating_add(1))
            .map_err(|error| ReactError::Other(format!("failed to resume live stream: {error}")))?;
        for record in records {
            match self.publish(record.event.as_ref())? {
                Reserve::Deliver | Reserve::AlreadySent => {}
                // Every replay gap stays paused until that gap's watermark is
                // ACKed. Oversized records do not silently advance recovery.
                Reserve::GapAndPause { .. } | Reserve::SkipOversized | Reserve::AlreadyPaused => {
                    break;
                }
            }
        }
        Ok(())
    }

    fn publish(&self, envelope: &EventEnvelope) -> EchoResult<Reserve> {
        let bytes = self.envelope_bytes(envelope)?;
        let mut inner = self.inner.lock().map_err(|_| lock_poisoned())?;
        inner.publish(
            envelope.sequence,
            bytes,
            self.max_outstanding,
            self.max_buffer_bytes,
            self.max_event_bytes,
            |reservation| match reservation {
                Reserve::Deliver => self.send_event(envelope),
                Reserve::SkipOversized => self.send_gap(
                    envelope.sequence,
                    envelope.sequence,
                    envelope.sequence,
                    GAP_REASON_EVENT_TOO_LARGE,
                ),
                Reserve::GapAndPause {
                    from_sequence,
                    to_sequence,
                    snapshot_watermark,
                } => self.send_gap(
                    *from_sequence,
                    *to_sequence,
                    *snapshot_watermark,
                    GAP_REASON_ACK_WINDOW,
                ),
                Reserve::AlreadyPaused | Reserve::AlreadySent => Ok(()),
            },
        )
    }

    fn send_event(&self, envelope: &EventEnvelope) -> EchoResult<()> {
        let wire = wire_envelope(envelope).map_err(|error| ReactError::Other(error.message))?;
        let notification = EventNotification {
            stream: self.stream_handle.clone(),
            envelope: wire,
        };
        notification.validate().map_err(|error| {
            ReactError::Other(format!("event notification is invalid: {error}"))
        })?;
        self.connection
            .send_notification(notification)
            .map_err(|error| {
                ReactError::Other(format!("failed to send _echo_agent/event: {error}"))
            })
    }

    fn send_gap(
        &self,
        from_sequence: u64,
        to_sequence: u64,
        snapshot_watermark: u64,
        reason: &str,
    ) -> EchoResult<()> {
        let from = echo_sdk_protocol::scalar::WireNonZeroU64::try_from(from_sequence.to_string())
            .map_err(|_| ReactError::Other("gap sequence must be positive".to_string()))?;
        let to = echo_sdk_protocol::scalar::WireNonZeroU64::try_from(to_sequence.to_string())
            .map_err(|_| ReactError::Other("gap sequence must be positive".to_string()))?;
        let watermark =
            echo_sdk_protocol::scalar::WireNonZeroU64::try_from(snapshot_watermark.to_string())
                .map_err(|_| ReactError::Other("gap watermark must be positive".to_string()))?;
        let notification = GapNotification {
            stream: self.stream_handle.clone(),
            gap: echo_sdk_protocol::event::EventGap {
                from_sequence: from,
                to_sequence: to,
                reason: reason.to_string(),
                snapshot_watermark: watermark,
            },
        };
        notification
            .validate()
            .map_err(|error| ReactError::Other(format!("gap notification is invalid: {error}")))?;
        self.connection
            .send_notification(notification)
            .map_err(|error| ReactError::Other(format!("failed to send _echo_agent/gap: {error}")))
    }
}

#[derive(Debug, PartialEq)]
enum Reserve {
    Deliver,
    GapAndPause {
        from_sequence: u64,
        to_sequence: u64,
        snapshot_watermark: u64,
    },
    SkipOversized,
    AlreadyPaused,
    AlreadySent,
}

impl StreamDelivery {
    fn envelope_bytes(&self, envelope: &EventEnvelope) -> EchoResult<usize> {
        let wire = wire_envelope(envelope).map_err(|error| ReactError::Other(error.message))?;
        serde_json::to_vec(&EventNotification {
            stream: self.stream_handle.clone(),
            envelope: wire,
        })
        .map(|bytes| bytes.len())
        .map_err(|error| ReactError::Other(format!("event notification is not encodable: {error}")))
    }
}

fn lock_poisoned() -> ReactError {
    ReactError::Other("stream delivery state lock poisoned".to_string())
}

#[async_trait::async_trait]
impl RunEventObserver for StreamDelivery {
    async fn on_committed_event(&self, envelope: &EventEnvelope) -> EchoResult<()> {
        let _serial = self.send_serial.lock().await;
        self.publish(envelope).map(|_| ())
    }
}

/// Bounded wait helper shared by `run/wait`.
pub(crate) async fn wait_with_timeout<T>(
    future: impl std::future::Future<Output = T>,
    timeout: Option<Duration>,
) -> Option<T> {
    match timeout {
        Some(bound) => tokio::time::timeout(bound, future).await.ok(),
        None => Some(future.await),
    }
}

/// Bounded journal replay with typed gap (todo `deliver-events-replay-
/// recovery` step 2). Reads strictly after `after_sequence` from the
/// retained floor, validates journal/envelope sequence alignment, and
/// enforces the event and byte bounds. The response is validated with the
/// contract's own `ReplayResponse::validate` before it is returned.
pub(crate) fn bounded_replay(
    limits_max_replay_bytes: usize,
    journal: &echo_agent::state::journal::SegmentedFileEventJournal<EventEnvelope>,
    stream: &echo_sdk_protocol::handle::WireHandle,
    after_sequence: u64,
    max_events: usize,
) -> std::result::Result<echo_sdk_protocol::event::ReplayResponse, EchoSdkError> {
    use echo_sdk_protocol::{error::ExtensionErrorCode, event::EventCursor};

    let operation = "_echo_agent/run/replay";
    let floor = journal.retention_metadata().retained_floor;
    let next_sequence = after_sequence.saturating_add(1);
    let gap = (after_sequence < u64::MAX && next_sequence < floor)
        .then(|| {
            let missing_to = floor.saturating_sub(1).max(next_sequence);
            Ok(echo_sdk_protocol::event::EventGap {
                from_sequence: nonzero(next_sequence)?,
                to_sequence: nonzero(missing_to)?,
                reason: "replay below the journal retention floor".to_string(),
                snapshot_watermark: nonzero(missing_to)?,
            })
        })
        .transpose()
        .map_err(|error: ReactError| {
            sdk_error(
                ExtensionErrorCode::SerializationViolation,
                error.to_string(),
                Retryability::Never,
                "_echo_agent/run/replay",
            )
        })?;
    let read_from = gap.as_ref().map_or(after_sequence, |gap| {
        gap.to_sequence.to_u64().unwrap_or_default()
    });
    let records = journal
        .replay_after(read_from, max_events)
        .map_err(|error| {
            sdk_error(
                ExtensionErrorCode::ReplayUnavailable,
                format!("journal replay failed: {error}"),
                Retryability::AfterDelay,
                operation,
            )
        })?;
    let mut events = Vec::with_capacity(records.len());
    let mut bytes = 0usize;
    let mut last_sequence = read_from;
    for record in records {
        let envelope = record.event.as_ref().clone();
        if record.sequence != envelope.sequence {
            return Err(sdk_error(
                ExtensionErrorCode::SerializationViolation,
                format!(
                    "journal sequence {} does not align with envelope sequence {}",
                    record.sequence, envelope.sequence
                ),
                Retryability::Never,
                operation,
            ));
        }
        let wire_event = wire_envelope(&envelope).map_err(|error| {
            sdk_error(
                ExtensionErrorCode::SerializationViolation,
                error.message,
                Retryability::Never,
                operation,
            )
        })?;
        let envelope_size = serde_json::to_vec(&wire_event)
            .map(|bytes| bytes.len())
            .map_err(|error| {
                sdk_error(
                    ExtensionErrorCode::SerializationViolation,
                    error.to_string(),
                    Retryability::Never,
                    operation,
                )
            })?;
        let next_bytes = bytes.checked_add(envelope_size).ok_or_else(|| {
            sdk_error(
                ExtensionErrorCode::PayloadTooLarge,
                "replay byte accounting overflow",
                Retryability::Never,
                operation,
            )
        })?;
        if events.len() >= max_events || next_bytes > limits_max_replay_bytes {
            if events.is_empty() {
                return Err(sdk_error(
                    ExtensionErrorCode::PayloadTooLarge,
                    "first replay event exceeds max_replay_bytes",
                    Retryability::Never,
                    operation,
                ));
            }
            break;
        }
        bytes = next_bytes;
        last_sequence = envelope.sequence;
        events.push(wire_event);
    }
    // Contract: with a gap and no deliverable events, the cursor equals the
    // gap watermark; otherwise it is the last delivered sequence.
    let cursor_sequence = match &gap {
        Some(gap) => events
            .last()
            .and_then(|event| event.sequence.to_u64())
            .unwrap_or_else(|| gap.snapshot_watermark.to_u64().unwrap_or_default()),
        None => last_sequence,
    };
    let response = echo_sdk_protocol::event::ReplayResponse {
        requested_after_sequence: echo_sdk_protocol::scalar::WireU64::from_u64(after_sequence),
        events,
        next_cursor: EventCursor {
            stream_id: stream.id.clone(),
            last_processed_sequence: echo_sdk_protocol::scalar::WireU64::from_u64(cursor_sequence),
        },
        gap,
    };
    response.validate().map_err(|error| {
        sdk_error(
            ExtensionErrorCode::SerializationViolation,
            format!("replay response failed contract validation: {error}"),
            Retryability::Never,
            operation,
        )
    })?;
    Ok(response)
}

/// Positive sequences are the only legal gap components, and the non-zero
/// scalar contract accepts exactly their decimal strings; failures become
/// typed errors instead of panicking in the delivery path.
fn nonzero(value: u64) -> EchoResult<WireNonZeroU64> {
    WireNonZeroU64::try_from(value.max(1).to_string())
        .map_err(|_| ReactError::Other("gap sequence must be a positive integer".to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gap_reasons_are_stable_strings() {
        assert!(!GAP_REASON_ACK_WINDOW.is_empty());
        assert!(!GAP_REASON_EVENT_TOO_LARGE.is_empty());
    }

    fn empty_delivery() -> DeliveryInner {
        DeliveryInner {
            unacked: VecDeque::new(),
            outstanding_bytes: 0,
            paused: false,
            paused_sequence: 0,
            last_sent_sequence: 0,
            ackable_sequence: 0,
        }
    }

    #[test]
    fn failed_publications_do_not_create_ackable_facts_or_retire_budget() -> EchoResult<()> {
        for (max_count, max_bytes, max_event) in
            [(1, 100, 100), (0, 100, 100), (1, 1, 100), (1, 100, 1)]
        {
            let mut inner = empty_delivery();
            let failed = inner.publish(1, 10, max_count, max_bytes, max_event, |_| {
                Err(ReactError::Other("transport enqueue failed".to_string()))
            });
            assert!(failed.is_err());
            assert_eq!(inner.last_sent_sequence, 0);
            assert_eq!(inner.ackable_sequence, 0);
            assert_eq!(inner.outstanding_bytes, 0);
            assert_eq!(inner.outstanding(), 0);
            assert!(!inner.paused);
            assert!(
                inner.acknowledge(1, 1).is_err(),
                "unsent watermark cannot be ACKed"
            );
            assert_eq!(
                inner.publish(1, 10, 1, 100, 100, |_| Ok(()))?,
                Reserve::Deliver
            );
        }
        Ok(())
    }

    #[test]
    fn failed_gap_enqueue_preserves_existing_event_budget_and_ack_cursor() -> EchoResult<()> {
        let mut inner = empty_delivery();
        inner.publish(1, 10, 1, 100, 100, |_| Ok(()))?;
        let failed = inner.publish(2, 10, 1, 100, 100, |_| {
            Err(ReactError::Other("gap enqueue failed".to_string()))
        });
        assert!(failed.is_err());
        assert_eq!(inner.unacked.front(), Some(&(1, 10)));
        assert_eq!(inner.outstanding_bytes, 10);
        assert_eq!(inner.last_sent_sequence, 1);
        assert_eq!(inner.ackable_sequence, 1);
        assert!(!inner.paused);
        assert!(inner.acknowledge(2, 1).is_err());
        assert_eq!(inner.acknowledge(1, 1)?, None);
        assert_eq!(
            inner.publish(2, 10, 1, 100, 100, |_| Ok(()))?,
            Reserve::Deliver
        );
        Ok(())
    }

    #[test]
    fn future_late_and_duplicate_acks_cannot_retract_an_announced_gap() -> EchoResult<()> {
        let mut inner = empty_delivery();
        inner.publish(1, 10, 1, 100, 100, |_| Ok(()))?;
        assert!(matches!(
            inner.publish(2, 10, 1, 100, 100, |_| Ok(()))?,
            Reserve::GapAndPause { .. }
        ));
        assert!(inner.acknowledge(3, 1).is_err());
        assert_eq!(inner.outstanding_bytes, 10);
        assert_eq!(inner.acknowledge(1, 1)?, None);
        assert!(inner.paused, "ordinary ACK cannot retract the snapshot gap");
        assert_eq!(inner.acknowledge(2, 1)?, Some(2));
        assert_eq!(inner.outstanding_bytes, 0);
        assert_eq!(inner.acknowledge(1, 1)?, None);
        assert_eq!(inner.last_sent_sequence, 2);
        assert_eq!(inner.acknowledge(2, 1)?, None);
        assert_eq!(
            inner.publish(2, 10, 1, 100, 100, |_| Ok(()))?,
            Reserve::AlreadySent
        );
        assert_eq!(
            inner.publish(3, 10, 1, 100, 100, |_| Ok(()))?,
            Reserve::Deliver
        );
        Ok(())
    }

    #[test]
    fn byte_bound_gap_retires_only_after_snapshot_ack() -> EchoResult<()> {
        let mut inner = empty_delivery();
        inner.publish(1, 6, 10, 10, 100, |_| Ok(()))?;
        assert!(matches!(
            inner.publish(2, 6, 10, 10, 100, |_| Ok(()))?,
            Reserve::GapAndPause { .. }
        ));
        assert_eq!(inner.outstanding_bytes, 6);
        assert_eq!(inner.acknowledge(2, 10)?, Some(2));
        assert_eq!(inner.outstanding_bytes, 0);
        assert_eq!(
            inner.publish(3, 6, 10, 10, 100, |_| Ok(()))?,
            Reserve::Deliver
        );
        Ok(())
    }

    #[test]
    fn replay_oversized_gap_waits_for_its_ack_and_suppresses_covered_records() -> EchoResult<()> {
        let mut inner = empty_delivery();
        assert_eq!(
            inner.publish(1, 10, 10, 100, 1, |_| Ok(()))?,
            Reserve::SkipOversized
        );
        assert_eq!(inner.acknowledge(1, 10)?, Some(1));
        assert_eq!(
            inner.publish(1, 10, 10, 100, 1, |_| Ok(()))?,
            Reserve::AlreadySent
        );
        assert_eq!(
            inner.publish(2, 10, 10, 100, 1, |_| Ok(()))?,
            Reserve::SkipOversized
        );
        assert_eq!(
            inner.publish(3, 10, 10, 100, 1, |_| Ok(()))?,
            Reserve::AlreadyPaused
        );
        assert_eq!(inner.acknowledge(1, 10)?, None);
        assert_eq!(inner.acknowledge(2, 10)?, Some(2));
        assert_eq!(
            inner.publish(3, 10, 10, 100, 1, |_| Ok(()))?,
            Reserve::SkipOversized
        );
        Ok(())
    }

    #[tokio::test]
    async fn closed_acp_transport_cannot_publish_or_make_a_gap_ackable() -> EchoResult<()> {
        let connection = agent_client_protocol::Agent
            .builder()
            .connect_with(
                agent_client_protocol::ByteStreams::new(futures::io::sink(), futures::io::empty()),
                async |connection| Ok(connection),
            )
            .await
            .map_err(|error| ReactError::Other(error.to_string()))?;
        let gap = GapNotification {
            stream: echo_sdk_protocol::handle::WireHandle {
                id: "closed-stream".to_string(),
                generation: echo_sdk_protocol::scalar::WireU64::from_u64(1),
                kind: echo_sdk_protocol::handle::HandleKind::Stream,
            },
            gap: echo_sdk_protocol::event::EventGap {
                from_sequence: nonzero(1)?,
                to_sequence: nonzero(1)?,
                snapshot_watermark: nonzero(1)?,
                reason: GAP_REASON_EVENT_TOO_LARGE.to_string(),
            },
        };
        let mut inner = empty_delivery();
        let publication = inner.publish(1, 10, 1, 100, 1, |_| {
            connection
                .send_notification(gap)
                .map_err(|error| ReactError::Other(error.to_string()))
        });
        assert!(
            publication.is_err(),
            "a closed real ACP output queue must reject the send"
        );
        assert!(!inner.paused);
        assert_eq!(inner.ackable_sequence, 0);
        assert_eq!(inner.last_sent_sequence, 0);
        assert_eq!(inner.outstanding_bytes, 0);
        assert!(inner.acknowledge(1, 1).is_err());
        Ok(())
    }

    #[test]
    fn replay_window_lookahead_keeps_the_durable_tail_recoverable() -> EchoResult<()> {
        let mut inner = empty_delivery();
        inner.publish(1, 10, 1, 100, 100, |_| Ok(()))?;
        inner.publish(2, 10, 1, 100, 100, |_| Ok(()))?;
        assert_eq!(inner.acknowledge(2, 1)?, Some(2));
        assert_eq!(
            inner.publish(3, 10, 1, 100, 100, |_| Ok(()))?,
            Reserve::Deliver
        );
        assert!(matches!(
            inner.publish(4, 10, 1, 100, 100, |_| Ok(()))?,
            Reserve::GapAndPause { .. }
        ));
        assert_eq!(inner.acknowledge(3, 1)?, None);
        assert!(inner.paused);
        assert_eq!(inner.acknowledge(4, 1)?, Some(4));
        assert_eq!(
            inner.publish(5, 10, 1, 100, 100, |_| Ok(()))?,
            Reserve::Deliver
        );
        Ok(())
    }

    fn replay_page(
        after: u64,
        sequences: std::ops::RangeInclusive<u64>,
    ) -> EchoResult<echo_sdk_protocol::event::ReplayResponse> {
        let fixture: serde_json::Value = serde_json::from_str(include_str!(
            "../../../contracts/sdk/fixtures/extension/v1/event-envelope-full-valid.json"
        ))
        .map_err(|error| ReactError::Other(error.to_string()))?;
        let payload = fixture
            .get("payload")
            .cloned()
            .ok_or_else(|| ReactError::Other("event fixture payload missing".to_string()))?;
        let template: echo_sdk_protocol::event::WireEventEnvelope = serde_json::from_value(payload)
            .map_err(|error| ReactError::Other(error.to_string()))?;
        let mut events = Vec::new();
        for sequence in sequences {
            let mut event = template.clone();
            event.sequence = nonzero(sequence)?;
            events.push(event);
        }
        let cursor = events
            .last()
            .and_then(|event| event.sequence.to_u64())
            .unwrap_or(after);
        Ok(echo_sdk_protocol::event::ReplayResponse {
            requested_after_sequence: echo_sdk_protocol::scalar::WireU64::from_u64(after),
            events,
            next_cursor: echo_sdk_protocol::event::EventCursor {
                stream_id: template.stream_id,
                last_processed_sequence: echo_sdk_protocol::scalar::WireU64::from_u64(cursor),
            },
            gap: None,
        })
    }

    #[test]
    fn consumed_replay_cursor_resumes_after_the_whole_page_not_the_initial_gap() -> EchoResult<()> {
        let mut inner = empty_delivery();
        inner.publish(1, 10, 1, 100, 100, |_| Ok(()))?;
        inner.publish(2, 10, 1, 100, 100, |_| Ok(()))?;
        let page = replay_page(2, 3..=8)?;
        page.validate()
            .map_err(|error| ReactError::Other(error.to_string()))?;
        inner.publish_replay(page, |_| Ok::<(), ReactError>(()))?;
        assert_eq!(inner.acknowledge(8, 1)?, Some(8));
        assert_eq!(inner.last_sent_sequence, 8);
        assert_eq!(
            inner.publish(8, 10, 1, 100, 100, |_| Ok(()))?,
            Reserve::AlreadySent
        );
        assert_eq!(
            inner.publish(9, 10, 1, 100, 100, |_| Ok(()))?,
            Reserve::Deliver
        );
        Ok(())
    }

    #[test]
    fn replay_publication_failure_and_empty_future_page_grant_no_ack_coverage() -> EchoResult<()> {
        let mut inner = empty_delivery();
        inner.publish(1, 10, 1, 100, 100, |_| Ok(()))?;
        inner.publish(2, 10, 1, 100, 100, |_| Ok(()))?;
        let failed = inner.publish_replay(replay_page(2, 3..=8)?, |_| {
            Err::<(), _>(ReactError::Other("response enqueue failed".to_string()))
        });
        assert!(failed.is_err());
        assert_eq!(inner.ackable_sequence, 2);
        assert!(inner.acknowledge(8, 1).is_err());
        let mut empty = replay_page(100, 3..=3)?;
        empty.events.clear();
        empty.next_cursor.last_processed_sequence =
            echo_sdk_protocol::scalar::WireU64::from_u64(100);
        empty
            .validate()
            .map_err(|error| ReactError::Other(error.to_string()))?;
        inner.publish_replay(empty, |_| Ok::<(), ReactError>(()))?;
        assert_eq!(inner.ackable_sequence, 2);
        assert!(inner.acknowledge(100, 1).is_err());
        assert!(inner.paused);
        Ok(())
    }

    #[test]
    fn multiple_replay_pages_and_retention_gap_only_grant_returned_coverage() -> EchoResult<()> {
        let mut inner = empty_delivery();
        inner.publish(1, 10, 1, 100, 100, |_| Ok(()))?;
        inner.publish(2, 10, 1, 100, 100, |_| Ok(()))?;
        for (after, start, end) in [(2, 3, 4), (4, 5, 6), (6, 7, 8)] {
            inner.publish_replay(replay_page(after, start..=end)?, |_| {
                Ok::<(), ReactError>(())
            })?;
            assert_eq!(inner.ackable_sequence, end);
        }
        assert_eq!(inner.acknowledge(8, 1)?, Some(8));
        inner.publish(9, 10, 1, 100, 1, |_| Ok(()))?;
        let fixture: serde_json::Value = serde_json::from_str(include_str!(
            "../../../contracts/sdk/fixtures/extension/v1/event-replay-gap-valid.json"
        ))
        .map_err(|error| ReactError::Other(error.to_string()))?;
        let page: echo_sdk_protocol::event::ReplayResponse = serde_json::from_value(
            fixture
                .get("payload")
                .cloned()
                .ok_or_else(|| ReactError::Other("gap fixture payload missing".to_string()))?,
        )
        .map_err(|error| ReactError::Other(error.to_string()))?;
        page.validate()
            .map_err(|error| ReactError::Other(error.to_string()))?;
        inner.publish_replay(page, |_| Ok::<(), ReactError>(()))?;
        assert_eq!(inner.acknowledge(9, 1)?, Some(9));
        assert_eq!(
            inner.publish(9, 10, 1, 100, 1, |_| Ok(()))?,
            Reserve::AlreadySent
        );
        Ok(())
    }

    #[test]
    fn real_journal_retention_page_authorizes_only_its_returned_watermark() -> EchoResult<()> {
        use echo_agent::agent::{AgentEvent, EventIdentity};
        use echo_agent::state::journal::SegmentedFileEventJournal;
        use echo_agent::utils::fs::FileDurability;
        let work = tempfile::tempdir().map_err(|error| ReactError::Other(error.to_string()))?;
        let identity = EventIdentity::new("stream-1", "turn-retention")?;
        // Each committed envelope rotates the previous tiny segment. Prune is
        // the framework's native operation, never an edited marker fixture.
        let journal = SegmentedFileEventJournal::<EventEnvelope>::open(
            work.path(),
            1,
            FileDurability::Flush,
        )?;
        for sequence in 1..=6 {
            journal
                .append(EventEnvelope::new(
                    &identity,
                    sequence,
                    None,
                    AgentEvent::Token("retained 中文🌊".to_string()),
                )?)
                .map_err(|error| {
                    ReactError::Other(format!("retention fixture journal append failed: {error}"))
                })?;
        }
        journal.prune_closed_segments_before(6)?;
        assert_eq!(journal.retention_metadata().retained_floor, 6);
        let stream = echo_sdk_protocol::handle::WireHandle {
            id: "stream-1".to_string(),
            generation: echo_sdk_protocol::scalar::WireU64::from_u64(7),
            kind: echo_sdk_protocol::handle::HandleKind::Stream,
        };
        let page = bounded_replay(1024 * 1024, &journal, &stream, 0, 8)
            .map_err(|error| ReactError::Other(error.message))?;
        page.validate()
            .map_err(|error| ReactError::Other(error.to_string()))?;
        assert_eq!(
            page.gap
                .as_ref()
                .and_then(|gap| gap.snapshot_watermark.to_u64()),
            Some(5)
        );
        assert_eq!(page.next_cursor.stream_id, stream.id);
        assert_eq!(page.next_cursor.last_processed_sequence.to_u64(), Some(6));
        let mut inner = empty_delivery();
        inner.publish(1, 10, 1, 100, 1, |_| Ok(()))?;
        inner.publish_replay(page, |_| Ok::<(), ReactError>(()))?;
        assert_eq!(inner.acknowledge(6, 1)?, Some(6));
        assert!(inner.acknowledge(7, 1).is_err());
        assert_eq!(inner.last_sent_sequence, 6);
        Ok(())
    }
}
