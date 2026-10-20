package com.echoagent.sdk;

import com.fasterxml.jackson.databind.JsonNode;
import java.math.BigInteger;

/** Checks replay transport facts without taking over framework recovery or client ACK policy. */
public final class ReplayResponse {
    private ReplayResponse() {}

    /** Preserve the response, including its plain EventGap and raw WireValue payloads, after validation. */
    public static JsonNode validate(JsonNode response, WireHandle stream, String requestedAfter) {
        try {
            if (stream == null || !"stream".equals(stream.kind())) throw invalid("replay requires a stream handle");
            BigInteger requested = integer(requestedAfter, false);
            object(response, "replay response");
            if (!requested.equals(integer(text(response, "requested_after_sequence"), false))) {
                throw invalid("replay request cursor differs from response");
            }
            JsonNode events = response.get("events");
            if (events == null || !events.isArray()) throw invalid("replay events must be an array");
            JsonNode cursor = response.get("next_cursor");
            object(cursor, "replay cursor");
            if (!stream.id().equals(identity(cursor, "stream_id", 256))) throw invalid("replay cursor belongs to another stream");
            BigInteger previous = requested;
            JsonNode gap = response.get("gap");
            if (gap != null && !gap.isNull()) {
                object(gap, "replay gap");
                BigInteger from = integer(text(gap, "from_sequence"), true);
                BigInteger to = integer(text(gap, "to_sequence"), true);
                BigInteger watermark = integer(text(gap, "snapshot_watermark"), true);
                identity(gap, "reason", 1024);
                if (!from.equals(requested.add(BigInteger.ONE)) || to.compareTo(from) < 0 || watermark.compareTo(to) < 0) {
                    throw invalid("replay gap bounds do not follow request or snapshot watermark");
                }
                previous = watermark;
            }
            for (JsonNode event : events) {
                validateEvent(event, stream);
                BigInteger sequence = integer(text(event, "sequence"), true);
                if (!sequence.equals(previous.add(BigInteger.ONE))) throw invalid("replay events are not contiguous");
                previous = sequence;
            }
            if (!previous.equals(integer(text(cursor, "last_processed_sequence"), false))) {
                throw invalid("replay cursor differs from returned events or snapshot watermark");
            }
            return response.deepCopy();
        } catch (IllegalArgumentException error) {
            throw invalid("replay contains malformed scalar: " + error.getMessage());
        }
    }

    private static void validateEvent(JsonNode event, WireHandle stream) {
        object(event, "replay event");
        if (!stream.id().equals(identity(event, "stream_id", 256))) throw invalid("replay event belongs to another stream");
        identity(event, "event_id", 256);
        identity(event, "turn_id", 256);
        String hash = text(event, "content_hash");
        if (!hash.matches("sha256:[0-9a-fA-F]{64}")) throw invalid("event content_hash must be SHA-256 text");
        JsonNode schema = event.get("schema_version");
        if (schema == null || !schema.isIntegralNumber() || !schema.canConvertToInt()
                || schema.intValue() <= 0 || schema.intValue() > 65535) throw invalid("event schema_version must be positive u16");
        integer(text(event, "sequence"), true);
        for (String field : new String[]{"conversation_id", "run_id", "message_id", "execution_id", "parent_event_id"}) {
            JsonNode value = event.get(field);
            if (value != null && !value.isNull()) identity(event, field, 256);
        }
        JsonNode timestamp = event.get("timestamp");
        object(timestamp, "event timestamp");
        WireValues.i64(text(timestamp, "unix_seconds"));
        JsonNode nanos = timestamp.get("nanos");
        if (nanos == null || !nanos.isIntegralNumber() || !nanos.canConvertToInt()
                || nanos.intValue() < 0 || nanos.intValue() >= 1_000_000_000) throw invalid("event timestamp nanos out of range");
        JsonNode rfc3339 = timestamp.get("rfc3339");
        if (rfc3339 != null && !rfc3339.isNull() && !rfc3339.isTextual()) throw invalid("event timestamp rfc3339 must be text");
        JsonNode payload = event.get("payload");
        object(payload, "event payload");
        identity(payload, "event_type", 256);
        JsonNode data = payload.get("data");
        if (data != null && !data.isNull()) JsonSupport.validateWire(data);
    }

    private static BigInteger integer(String value, boolean positive) {
        WireValues.u64(value);
        BigInteger number = new BigInteger(value);
        if (positive && number.signum() == 0) throw invalid("event/gap sequence must be positive");
        return number;
    }
    private static String text(JsonNode value, String field) {
        object(value, "replay field owner");
        JsonNode node = value.get(field);
        if (node == null || !node.isTextual()) throw invalid(field + " must be text");
        return node.textValue();
    }
    private static String identity(JsonNode value, String field, int maxCodePoints) {
        String text = text(value, field);
        if (text.isBlank() || text.codePointCount(0, text.length()) > maxCodePoints) throw invalid(field + " must be non-empty and bounded");
        return text;
    }
    private static void object(JsonNode value, String name) {
        if (value == null || !value.isObject()) throw invalid(name + " must be object");
    }
    private static EchoAgentException invalid(String message) {
        return new EchoAgentException("serialization_violation", message, "never", "_echo_agent/run/replay", null);
    }
}
