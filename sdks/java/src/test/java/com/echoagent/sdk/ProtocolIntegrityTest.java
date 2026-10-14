package com.echoagent.sdk;
import org.junit.jupiter.api.Test;
import java.nio.file.Path;
import static org.junit.jupiter.api.Assertions.*;
class ProtocolIntegrityTest {
    @Test void rejectsMalformedNativeProjection() throws Exception {
        var corpus = JsonSupport.MAPPER.readTree(Path.of("../shared/fixtures/native-projection-invalid.json").toFile());
        for (var item : corpus) {
            var error = assertThrows(EchoAgentException.class, () -> JsonSupport.fromWire(item.get("wire")), item.path("name").asText());
            assertEquals("serialization_violation", error.code());
        }
    }
    @Test void sdkErrorRetainsHandleAndRetryMetadata() throws Exception {
        var handle = new WireHandle("run-中文", "18446744073709551615", "run");
        var data = JsonSupport.MAPPER.createObjectNode().put("code", "stale_handle").put("message", "stale").put("retryable", "after_delay").put("operation", "Host operation");
        data.set("handle", handle.toJson());
        var acp = new com.agentclientprotocol.sdk.spec.AcpClientSession.AcpError(new com.agentclientprotocol.sdk.spec.AcpSchema.JSONRPCError(-32000, "failure", data));
        var error = EchoAgentException.from(new java.util.concurrent.CompletionException(acp), "client fallback");
        assertEquals("stale_handle", error.code());
        assertEquals("after_delay", error.retryable());
        assertEquals("Host operation", error.operation());
        assertEquals(handle, error.handle());
    }
    @Test void facadeRequiresValueEnvelope() {
        var error = assertThrows(EchoAgentException.class, () -> JsonSupport.responseValue(JsonSupport.MAPPER.createObjectNode()));
        assertEquals("serialization_violation", error.code());
    }
    @Test void specialKeysRemainData() throws Exception {
        var wire = JsonSupport.MAPPER.readTree("{\"kind\":\"map\",\"value\":[{\"key\":{\"kind\":\"string\",\"value\":\"__proto__\"},\"value\":{\"kind\":\"string\",\"value\":\"中文🙂\"}}]}");
        assertEquals("中文🙂", JsonSupport.fromWire(wire).path("__proto__").textValue());
    }
    private static com.fasterxml.jackson.databind.JsonNode retentionReplay() throws Exception {
        return JsonSupport.MAPPER.readTree(Path.of("../../contracts/sdk/fixtures/extension/v1/event-replay-gap-valid.json").toFile()).path("payload");
    }
    private static com.fasterxml.jackson.databind.node.ObjectNode replayEvent(String sequence) {
        var event = JsonSupport.MAPPER.createObjectNode().put("schema_version", 4).put("event_id", "event-中文😀")
                .put("content_hash", "sha256:" + "a".repeat(64)).put("sequence", sequence).put("stream_id", "stream-1").put("turn_id", "turn-中文");
        event.set("timestamp", JsonSupport.MAPPER.createObjectNode().put("unix_seconds", "-9223372036854775808").put("nanos", 999999999));
        event.set("payload", JsonSupport.MAPPER.createObjectNode().put("event_type", "typed_custom_event").set("data", WireValues.value(java.util.Map.of("__proto__", "中文😀"))));
        return event;
    }
    @Test void replayUsesCanonicalRetentionFixtureAndPreservesRawPayload() throws Exception {
        var stream = new WireHandle("stream-1", "18446744073709551615", "stream");
        var fixture = retentionReplay();
        assertEquals(fixture, ReplayResponse.validate(fixture, stream, "4"));
        assertNotSame(fixture, ReplayResponse.validate(fixture, stream, "4"));
        assertEquals("9", ReplayResponse.validate(fixture, stream, "4").path("next_cursor").path("last_processed_sequence").textValue());
        var withEvents = fixture.deepCopy();
        ((com.fasterxml.jackson.databind.node.ArrayNode) withEvents.path("events")).add(replayEvent("10")).add(replayEvent("11"));
        ((com.fasterxml.jackson.databind.node.ObjectNode) withEvents.path("next_cursor")).put("last_processed_sequence", "11");
        assertEquals(withEvents, ReplayResponse.validate(withEvents, stream, "4"));
        var noGap = JsonSupport.MAPPER.createObjectNode().put("requested_after_sequence", "9");
        noGap.set("events", JsonSupport.MAPPER.createArrayNode().add(replayEvent("10")));
        noGap.set("next_cursor", JsonSupport.MAPPER.createObjectNode().put("stream_id", "stream-1").put("last_processed_sequence", "10"));
        assertEquals(noGap, ReplayResponse.validate(noGap, stream, "9"));
    }
    @Test void replayRejectsWrongRetentionWatermarkIdentityAndMalformedEventScalars() throws Exception {
        var stream = new WireHandle("stream-1", "1", "stream");
        for (String pointer : java.util.List.of("requested_after_sequence", "next_cursor/last_processed_sequence", "gap/from_sequence", "gap/to_sequence", "gap/snapshot_watermark")) {
            var malformed = retentionReplay().deepCopy();
            var parts = pointer.split("/");
            var target = parts.length == 1 ? malformed : malformed.path(parts[0]);
            ((com.fasterxml.jackson.databind.node.ObjectNode) target).put(parts[parts.length - 1], "01");
            var error = assertThrows(EchoAgentException.class, () -> ReplayResponse.validate(malformed, stream, "4"));
            assertEquals("serialization_violation", error.code());
        }
        var fixture = retentionReplay();
        assertThrows(EchoAgentException.class, () -> ReplayResponse.validate(fixture, stream, "3"));
        assertThrows(EchoAgentException.class, () -> ReplayResponse.validate(fixture, new WireHandle("other", "1", "stream"), "4"));
        assertThrows(EchoAgentException.class, () -> ReplayResponse.validate(fixture, new WireHandle("stream-1", "1", "run"), "4"));
        for (String mutation : java.util.List.of("from", "to", "watermark", "cursor", "sequence", "stream", "event_id", "turn_id", "hash", "schema", "seconds", "nanos")) {
            var malformed = retentionReplay().deepCopy();
            var event = replayEvent("10"); ((com.fasterxml.jackson.databind.node.ArrayNode) malformed.path("events")).add(event);
            ((com.fasterxml.jackson.databind.node.ObjectNode) malformed.path("next_cursor")).put("last_processed_sequence", "10");
            var gap = (com.fasterxml.jackson.databind.node.ObjectNode) malformed.path("gap");
            switch (mutation) {
                case "from" -> gap.put("from_sequence", "6");
                case "to" -> gap.put("to_sequence", "4");
                case "watermark" -> gap.put("snapshot_watermark", "8");
                case "cursor" -> ((com.fasterxml.jackson.databind.node.ObjectNode) malformed.path("next_cursor")).put("last_processed_sequence", "11");
                case "sequence" -> event.put("sequence", "11");
                case "stream" -> event.put("stream_id", "other");
                case "event_id" -> event.put("event_id", " ");
                case "turn_id" -> event.put("turn_id", "");
                case "hash" -> event.put("content_hash", "sha256:bad");
                case "schema" -> event.put("schema_version", 0);
                case "seconds" -> ((com.fasterxml.jackson.databind.node.ObjectNode) event.path("timestamp")).put("unix_seconds", "-0");
                case "nanos" -> ((com.fasterxml.jackson.databind.node.ObjectNode) event.path("timestamp")).put("nanos", 1.5);
            }
            assertThrows(EchoAgentException.class, () -> ReplayResponse.validate(malformed, stream, "4"), mutation);
        }
    }
    @Test void replayAcceptsEmptyMaximumCursorAndRejectsWrappedOrZeroSequences() throws Exception {
        var max = "18446744073709551615";
        var stream = new WireHandle("stream-1", max, "stream");
        var empty = JsonSupport.MAPPER.createObjectNode().put("requested_after_sequence", max);
        empty.set("events", JsonSupport.MAPPER.createArrayNode());
        empty.set("next_cursor", JsonSupport.MAPPER.createObjectNode().put("stream_id", "stream-1").put("last_processed_sequence", max));
        assertEquals(empty, ReplayResponse.validate(empty, stream, max));
        ((com.fasterxml.jackson.databind.node.ArrayNode) empty.path("events")).add(replayEvent("1"));
        assertThrows(EchoAgentException.class, () -> ReplayResponse.validate(empty, stream, max));
        var malformed = retentionReplay().deepCopy();
        ((com.fasterxml.jackson.databind.node.ArrayNode) malformed.path("events")).add(replayEvent("0"));
        assertThrows(EchoAgentException.class, () -> ReplayResponse.validate(malformed, stream, "4"));
        var fixture = retentionReplay();
        assertThrows(EchoAgentException.class, () -> ReplayResponse.validate(fixture, stream, "04"));
        assertThrows(EchoAgentException.class, () -> ReplayResponse.validate(fixture, stream, "18446744073709551616"));
    }
    @Test void nativeDomainIdsRemainMapsAndExplicitHandlesRejectLostFields() throws Exception {
        for(var value: java.util.List.of(java.util.Map.of("id","case","name","case"),java.util.Map.of("generation","domain"),java.util.Map.of("id","domain","kind","session","generation","1","title","ordinary"))){
            assertEquals("map",JsonSupport.wire(value).path("kind").asText());
            assertEquals(JsonSupport.MAPPER.valueToTree(value),JsonSupport.fromWire(JsonSupport.wire(value)));
        }
        var explicit=JsonSupport.MAPPER.readTree("{\"kind\":\"handle\",\"value\":{\"id\":\"e\",\"kind\":\"extension\",\"generation\":\"1\",\"title\":\"lost\"}}");
        assertThrows(IllegalArgumentException.class,()->JsonSupport.fromWire(explicit));
    }

    @Test void explicitRecordWrapperKeepsDomainIdentityFields() throws Exception {
        var value=JsonSupport.MAPPER.readTree("{\"kind\":\"record\",\"id\":\"record-id\",\"generation\":\"1\",\"value\":{\"type_id\":\"Record\",\"fields\":[]}}");
        assertEquals(value,JsonSupport.wire(value));
        assertEquals(value,JsonSupport.fromWire(JsonSupport.wire(value)));
    }

}
