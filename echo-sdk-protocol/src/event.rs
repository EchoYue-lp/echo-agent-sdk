//! Lossless event, replay and gap contracts for the SDK extension profile.

use agent_client_protocol::{JsonRpcNotification, JsonRpcRequest, JsonRpcResponse};
use serde::{Deserialize, Serialize};

use crate::handle::{HandleKind, WireHandle};
use crate::scalar::{ScalarError, WireNonZeroU64, WireTimestamp, WireU64, WireValue};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
pub struct WireEventPayload {
    #[schemars(length(min = 1, max = 256))]
    pub event_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<WireValue>,
}

impl WireEventPayload {
    pub fn validate(&self) -> Result<(), EventWireError> {
        if self.event_type.trim().is_empty() {
            return Err(EventWireError::InvalidIdentity("event_type"));
        }
        if let Some(data) = &self.data {
            data.validate()?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
pub struct WireEventEnvelope {
    pub schema_version: u16,
    #[schemars(length(min = 1, max = 256))]
    pub event_id: String,
    #[schemars(regex(pattern = "^sha256:[0-9a-fA-F]{64}$"))]
    pub content_hash: String,
    pub sequence: WireNonZeroU64,
    #[schemars(length(min = 1, max = 256))]
    pub stream_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(length(min = 1, max = 256))]
    pub conversation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(length(min = 1, max = 256))]
    pub run_id: Option<String>,
    #[schemars(length(min = 1, max = 256))]
    pub turn_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(length(min = 1, max = 256))]
    pub message_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(length(min = 1, max = 256))]
    pub execution_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(length(min = 1, max = 256))]
    pub parent_event_id: Option<String>,
    pub timestamp: WireTimestamp,
    pub payload: WireEventPayload,
}

impl WireEventEnvelope {
    pub fn validate(&self) -> Result<(), EventWireError> {
        for (name, value) in [
            ("event_id", self.event_id.as_str()),
            ("stream_id", self.stream_id.as_str()),
            ("turn_id", self.turn_id.as_str()),
            ("content_hash", self.content_hash.as_str()),
        ] {
            if value.trim().is_empty() {
                return Err(EventWireError::InvalidIdentity(name));
            }
        }
        let hash_is_valid = self
            .content_hash
            .strip_prefix("sha256:")
            .is_some_and(|hex| {
                hex.chars().count() == 64
                    && hex.chars().all(|character| character.is_ascii_hexdigit())
            });
        if !hash_is_valid {
            return Err(EventWireError::InvalidIdentity("content_hash"));
        }
        if self.sequence.to_u64().is_none_or(|sequence| sequence == 0) {
            return Err(EventWireError::InvalidSequence);
        }
        self.timestamp.validate()?;
        self.payload.validate()
    }
}

#[derive(Debug)]
pub enum EventWireError {
    InvalidIdentity(&'static str),
    InvalidSequence,
    InvalidTimestamp,
    InvalidPayload(String),
    Scalar(ScalarError),
}

impl std::fmt::Display for EventWireError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidIdentity(name) => write!(formatter, "{name} must be non-empty"),
            Self::InvalidSequence => write!(formatter, "event sequence must start at one"),
            Self::InvalidTimestamp => write!(formatter, "event timestamp is out of range"),
            Self::InvalidPayload(message) => {
                write!(formatter, "invalid AgentEvent payload: {message}")
            }
            Self::Scalar(error) => write!(formatter, "invalid wire scalar: {error}"),
        }
    }
}

impl std::error::Error for EventWireError {}

impl From<ScalarError> for EventWireError {
    fn from(error: ScalarError) -> Self {
        Self::Scalar(error)
    }
}

#[derive(
    Debug, Clone, PartialEq, Serialize, Deserialize, schemars::JsonSchema, JsonRpcNotification,
)]
#[notification(method = "_echo_agent/event")]
#[serde(deny_unknown_fields)]
pub struct EventNotification {
    /// Live event stream the envelope belongs to; the generation fence
    /// prevents a pre-restart subscriber from consuming a newer stream.
    pub stream: WireHandle,
    pub envelope: WireEventEnvelope,
}

impl EventNotification {
    pub fn validate(&self) -> Result<(), EventWireError> {
        self.stream
            .validate()
            .map_err(|_| EventWireError::InvalidIdentity("stream handle"))?;
        if self.stream.kind != HandleKind::Stream {
            return Err(EventWireError::InvalidIdentity("stream handle kind"));
        }
        if self.stream.id != self.envelope.stream_id {
            return Err(EventWireError::InvalidIdentity(
                "notification stream must match the envelope stream_id",
            ));
        }
        self.envelope.validate()
    }
}

#[derive(
    Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema, JsonRpcNotification,
)]
#[notification(method = "_echo_agent/event/ack")]
#[serde(deny_unknown_fields)]
pub struct EventAckNotification {
    pub ack: EventAck,
}

/// Client acknowledgement of consumed live events. The Host counts
/// outstanding notifications per stream against the negotiated
/// `max_outstanding_live_events` window; acknowledgements retire them and
/// resume live delivery after a gap pause. Hosts ignore unknown or
/// un-negotiated ACK notifications without producing a response (ACP
/// notification rules).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct EventAck {
    pub stream: WireHandle,
    /// Highest contiguous sequence the Client has consumed.
    pub last_processed_sequence: WireNonZeroU64,
}

impl EventAck {
    pub fn validate(&self) -> Result<(), EventWireError> {
        self.stream
            .validate()
            .map_err(|_| EventWireError::InvalidIdentity("stream handle"))?;
        if self.stream.kind != HandleKind::Stream {
            return Err(EventWireError::InvalidIdentity("stream handle kind"));
        }
        if self
            .last_processed_sequence
            .to_u64()
            .is_none_or(|sequence| sequence == 0)
        {
            return Err(EventWireError::InvalidSequence);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
pub struct EventCursor {
    #[schemars(length(min = 1, max = 256))]
    pub stream_id: String,
    pub last_processed_sequence: WireU64,
}

impl EventCursor {
    pub fn validate(&self) -> Result<(), EventWireError> {
        if self.stream_id.trim().is_empty() {
            Err(EventWireError::InvalidIdentity("stream_id"))
        } else {
            Ok(())
        }
    }
}

#[derive(
    Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema, JsonRpcRequest,
)]
#[request(method = "_echo_agent/run/replay", response = ReplayResponse)]
#[serde(deny_unknown_fields)]
pub struct ReplayRequest {
    /// Event stream to replay. The handle's generation must match the
    /// currently served stream incarnation; stale or wrong-kind handles
    /// fail with typed errors before any event is read.
    pub stream: WireHandle,
    /// Replay events strictly after this sequence (0 = from the retained
    /// beginning).
    pub after_sequence: WireU64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_events: Option<WireNonZeroU64>,
}

impl ReplayRequest {
    pub fn validate(&self) -> Result<(), EventWireError> {
        self.stream
            .validate()
            .map_err(|_| EventWireError::InvalidIdentity("stream handle"))?;
        if self.stream.kind != HandleKind::Stream {
            return Err(EventWireError::InvalidIdentity("stream handle kind"));
        }
        if self
            .max_events
            .as_ref()
            .is_some_and(|maximum| maximum.to_u64() == Some(0))
        {
            return Err(EventWireError::InvalidSequence);
        }
        Ok(())
    }
}

#[derive(
    Debug, Clone, PartialEq, Serialize, Deserialize, schemars::JsonSchema, JsonRpcResponse,
)]
pub struct ReplayResponse {
    /// Cursor supplied by the request. This makes an empty response and a
    /// retention gap independently verifiable.
    pub requested_after_sequence: WireU64,
    pub events: Vec<WireEventEnvelope>,
    pub next_cursor: EventCursor,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gap: Option<EventGap>,
}

impl ReplayResponse {
    pub fn validate(&self) -> Result<(), EventWireError> {
        self.next_cursor.validate()?;
        let requested = self
            .requested_after_sequence
            .to_u64()
            .ok_or(EventWireError::InvalidSequence)?;
        let gap_watermark = match &self.gap {
            Some(gap) => Some(
                gap.snapshot_watermark
                    .to_u64()
                    .ok_or(EventWireError::InvalidSequence)?,
            ),
            None => None,
        };
        let mut previous: Option<u64> = None;
        for event in &self.events {
            event.validate()?;
            if event.stream_id != self.next_cursor.stream_id {
                return Err(EventWireError::InvalidIdentity("replay stream_id"));
            }
            let sequence = event
                .sequence
                .to_u64()
                .ok_or(EventWireError::InvalidSequence)?;
            if previous.is_none() {
                let base = gap_watermark.unwrap_or(requested);
                if base.checked_add(1) != Some(sequence) {
                    return Err(EventWireError::InvalidSequence);
                }
            }
            if let Some(previous) = previous
                && previous.checked_add(1) != Some(sequence)
            {
                return Err(EventWireError::InvalidSequence);
            }
            previous = Some(sequence);
        }
        let cursor = self
            .next_cursor
            .last_processed_sequence
            .to_u64()
            .ok_or(EventWireError::InvalidSequence)?;
        let expected_cursor = previous.or(gap_watermark).unwrap_or(requested);
        if cursor != expected_cursor {
            return Err(EventWireError::InvalidSequence);
        }
        if let Some(gap) = &self.gap {
            gap.validate()?;
            let from = gap
                .from_sequence
                .to_u64()
                .ok_or(EventWireError::InvalidSequence)?;
            if requested.checked_add(1) != Some(from) {
                return Err(EventWireError::InvalidSequence);
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
pub struct EventGap {
    pub from_sequence: WireNonZeroU64,
    pub to_sequence: WireNonZeroU64,
    #[schemars(length(min = 1, max = 1024))]
    pub reason: String,
    pub snapshot_watermark: WireNonZeroU64,
}

impl EventGap {
    pub fn validate(&self) -> Result<(), EventWireError> {
        let from = self
            .from_sequence
            .to_u64()
            .ok_or(EventWireError::InvalidSequence)?;
        let to = self
            .to_sequence
            .to_u64()
            .ok_or(EventWireError::InvalidSequence)?;
        let watermark = self
            .snapshot_watermark
            .to_u64()
            .ok_or(EventWireError::InvalidSequence)?;
        if from == 0 || to < from || watermark < to || self.reason.trim().is_empty() {
            Err(EventWireError::InvalidSequence)
        } else {
            Ok(())
        }
    }
}

#[derive(
    Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema, JsonRpcNotification,
)]
#[notification(method = "_echo_agent/gap")]
#[serde(deny_unknown_fields)]
pub struct GapNotification {
    /// Event stream that hit its live-delivery bound.
    pub stream: WireHandle,
    pub gap: EventGap,
}

impl GapNotification {
    pub fn validate(&self) -> Result<(), EventWireError> {
        self.stream
            .validate()
            .map_err(|_| EventWireError::InvalidIdentity("stream handle"))?;
        if self.stream.kind != HandleKind::Stream {
            return Err(EventWireError::InvalidIdentity("stream handle kind"));
        }
        self.gap.validate()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn event(sequence: u64, event_type: &str) -> Result<WireEventEnvelope, ScalarError> {
        Ok(WireEventEnvelope {
            schema_version: 4,
            event_id: format!("event-{sequence}"),
            content_hash: format!("sha256:{}", "a".repeat(64)),
            sequence: WireNonZeroU64::try_from(sequence.to_string())?,
            stream_id: "stream-1".to_string(),
            conversation_id: None,
            run_id: None,
            turn_id: "turn-1".to_string(),
            message_id: None,
            execution_id: None,
            parent_event_id: None,
            timestamp: WireTimestamp {
                unix_seconds: crate::scalar::WireI64::from_i64(0),
                nanos: 0,
                rfc3339: None,
            },
            payload: WireEventPayload {
                event_type: event_type.to_string(),
                data: None,
            },
        })
    }

    #[test]
    fn sequence_zero_is_invalid() {
        let parsed: Result<WireEventEnvelope, _> = serde_json::from_value(serde_json::json!({
            "schema_version": 4,
            "event_id": "event-1",
            "content_hash": format!("sha256:{}", "a".repeat(64)),
            "sequence": "0",
            "stream_id": "stream-1",
            "turn_id": "turn-1",
            "timestamp": {"unix_seconds": "0", "nanos": 0},
            "payload": {"event_type": "think_start"}
        }));
        assert!(parsed.is_err());
    }

    #[test]
    fn replay_rejects_non_contiguous_events() -> Result<(), Box<dyn std::error::Error>> {
        let first = event(1, "think_start")?;
        let third = event(3, "think_end")?;
        let response = ReplayResponse {
            requested_after_sequence: WireU64::from_u64(0),
            events: vec![first, third],
            next_cursor: EventCursor {
                stream_id: "stream-1".to_string(),
                last_processed_sequence: WireU64::from_u64(3),
            },
            gap: None,
        };
        assert!(response.validate().is_err());
        Ok(())
    }

    #[test]
    fn replay_rejects_cursor_ahead_of_delivery() -> Result<(), Box<dyn std::error::Error>> {
        let event = event(1, "think_start")?;
        let response = ReplayResponse {
            requested_after_sequence: WireU64::from_u64(0),
            events: vec![event],
            next_cursor: EventCursor {
                stream_id: "stream-1".to_string(),
                last_processed_sequence: WireU64::from_u64(100),
            },
            gap: None,
        };
        assert!(response.validate().is_err());
        Ok(())
    }
}
