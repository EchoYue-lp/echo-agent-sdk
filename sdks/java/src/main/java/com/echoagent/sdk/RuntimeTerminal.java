package com.echoagent.sdk;

import com.fasterxml.jackson.databind.JsonNode;
import java.math.BigInteger;
import java.util.Set;

/** Framework V6末尾观察的读取适配，不拥有执行器或任务完成状态。 */
final class RuntimeTerminal {
    private RuntimeTerminal() {}

    static boolean isTerminal(JsonNode envelope) {
        if (!"execution_status".equals(envelope.path("payload").path("event_type").asText())) return false;
        final JsonNode value;
        try {
            value = JsonSupport.fromWire(envelope.path("payload").path("data"));
        } catch (RuntimeException error) {
            var failure = invalid(); failure.initCause(error); throw failure;
        }
        return statusTerminal(value);
    }

    private static boolean statusTerminal(JsonNode value) {
        if (!value.isObject() || !text(value.path("execution_id"))
                || (value.hasNonNull("request_id") && !text(value.path("request_id")))
                || !Set.of("Thinking", "Acting", "Terminal").contains(value.path("phase").asText())
                || !Set.of("started", "waiting", "tool_pending", "candidate", "settled", "cancelled", "failed").contains(value.path("status").asText())
                || (value.hasNonNull("reason") && !value.path("reason").isTextual())) throw invalid();
        JsonNode sequence = value.path("sequence");
        if (!sequence.isIntegralNumber() && !sequence.isTextual()) throw invalid();
        String encoded = sequence.asText();
        if (!encoded.matches("0|[1-9][0-9]{0,19}")) throw invalid();
        if (new BigInteger(encoded).compareTo(JsonSupport.MAX_U64) > 0) throw invalid();
        return "Terminal".equals(value.path("phase").asText())
                && Set.of("settled", "cancelled", "failed").contains(value.path("status").asText());
    }

    /** Reverse JSON 保留原 sequence 文本，禁止默认 mapper 把 BigInteger 编成数值。 */
    static void validateReverseAgentValue(JsonNode payload, boolean complete) {
        if (payload == null || !"agent".equals(payload.path("kind").asText())) return;
        JsonNode event = payload.path("value");
        if (!"execution_status".equals(event.path("event").asText())) return;
        JsonNode data = event.path("data");
        if (!data.isObject() || !data.path("sequence").isTextual()) throw invalid();
        var allowed = Set.of("execution_id", "request_id", "sequence", "phase", "status", "reason");
        data.fieldNames().forEachRemaining(name -> { if (!allowed.contains(name)) throw invalid(); });
        boolean terminal = statusTerminal(data);
        boolean terminalKind = Set.of("settled", "failed", "cancelled").contains(data.path("status").asText());
        if (terminalKind != complete || (terminalKind && !terminal)) throw invalid();
    }

    private static boolean text(JsonNode value) { return value.isTextual() && !value.textValue().isBlank(); }
    private static EchoAgentException invalid() { return new EchoAgentException("serialization_violation", "malformed runtime execution status", "never", "_echo_agent/event", null); }
}
