package com.echoagent.sdk;

import com.fasterxml.jackson.databind.JsonNode;
import org.junit.jupiter.api.Test;
import java.math.BigInteger;
import java.util.List;
import static org.junit.jupiter.api.Assertions.*;

class CallbackAndEffectsTest {
    private static JsonNode json(String value) throws Exception { return JsonSupport.MAPPER.readTree(value); }
    private static JsonNode envelope(String operation, JsonNode input) {
        var invocation = JsonSupport.MAPPER.createObjectNode().put("operation", operation); invocation.set("input", input);
        var raw = JsonSupport.MAPPER.createObjectNode().put("invocation_id", "transport-id"); raw.set("invocation", invocation); return raw;
    }
    @Test void allEightCallbacksAreTypedAndProduceMatchingUnitOutcome() throws Exception {
        var cases = List.of(
                envelope("callback_on_think_start", json("{\"agent\":\"中文😀\",\"messages\":[{\"role\":\"assistant\",\"content\":{\"kind\":\"string\",\"value\":\"think\"},\"tool_calls\":[{\"id\":\"tool-id\",\"call_type\":\"function\",\"function_name\":\"tool\",\"arguments\":\"{}\"}],\"reasoning_blocks\":[{\"kind\":\"signed\",\"thinking\":\"thought\",\"signature\":\"signature\"}]}]}")),
                envelope("callback_on_think_end", json("{\"agent\":\"中文😀\",\"steps\":[{\"kind\":\"thought\",\"text\":\"reason\"},{\"kind\":\"call\",\"tool_call_id\":\"canonical\",\"function_name\":\"tool\",\"arguments\":{\"kind\":\"null\"}}],\"prompt_tokens\":\"18446744073709551615\",\"completion_tokens\":\"18446744073709551615\"}")),
                envelope("callback_on_tool_start", json("{\"agent\":\"中文😀\",\"call_id\":\"canonical\",\"tool\":\"tool\",\"args\":{\"kind\":\"null\"}}")),
                envelope("callback_on_tool_end", json("{\"agent\":\"中文😀\",\"call_id\":\"canonical\",\"tool\":\"tool\",\"result\":\"ok\"}")),
                envelope("callback_on_tool_error", json("{\"agent\":\"中文😀\",\"call_id\":\"canonical\",\"tool\":\"tool\",\"error\":\"failed\"}")),
                envelope("callback_on_tool_interrupted", json("{\"agent\":\"中文😀\",\"call_id\":\"canonical\",\"tool\":\"tool\",\"input\":{\"kind\":\"map\",\"value\":[{\"key\":{\"kind\":\"string\",\"value\":\"__proto__\"},\"value\":{\"kind\":\"string\",\"value\":\"😀\"}}]},\"error\":\"cancelled\"}")),
                envelope("callback_on_final_answer", json("{\"agent\":\"中文😀\",\"answer\":\"answer\"}")),
                envelope("callback_on_iteration", json("{\"agent\":\"中文😀\",\"iteration\":\"18446744073709551615\"}")));
        for (var raw : cases) {
            var callback = CallbackCall.from(raw);
            assertEquals(raw.path("invocation").path("input"), callback.input());
            assertEquals("transport-id", callback.invocationId());
            assertEquals(callback.operation(), callback.completed().toJson().path("result").path("operation").textValue());
            assertTrue(callback.completed().toJson().path("result").path("value").isNull());
        }
        for (var raw : cases.subList(2, 6)) assertEquals("canonical", CallbackCall.from(raw).callId());
        var thinkEnd = assertInstanceOf(CallbackCall.ThinkEnd.class, CallbackCall.from(cases.get(1)).request());
        assertEquals(new BigInteger("18446744073709551615"), thinkEnd.promptTokens());
        assertInstanceOf(CallbackCall.Step.Call.class, thinkEnd.steps().get(1));
    }
    @Test void callbackRejectsUnknownOperationFieldAndMissingInterruptedIdentity() throws Exception {
        var raw = envelope("callback_on_tool_interrupted", json("{\"agent\":\"a\",\"tool\":\"t\",\"input\":{\"kind\":\"null\"},\"error\":\"e\"}"));
        assertThrows(IllegalArgumentException.class, () -> CallbackCall.from(raw));
        var input = (com.fasterxml.jackson.databind.node.ObjectNode) raw.path("invocation").path("input"); input.put("call_id", "id");
        input.put("unknown", "bad"); assertThrows(IllegalArgumentException.class, () -> CallbackCall.from(raw));
        input.remove("unknown"); ((com.fasterxml.jackson.databind.node.ObjectNode) raw.path("invocation")).put("operation", "callback_on_unknown");
        assertThrows(IllegalArgumentException.class, () -> CallbackCall.from(raw));
        var invalid = envelope("callback_on_iteration", json("{\"agent\":\"a\",\"iteration\":\"01\"}"));
        assertThrows(IllegalArgumentException.class, () -> CallbackCall.from(invalid));
    }
    @Test void effectsAreLosslessImmutableAndUnknownFactsFail() throws Exception {
        var source = ToolResult.success("display text");
        var effect = new ToolEffect.TestRun("test", false, new BigInteger("18446744073709551615"));
        var result = source.withEffects(List.of(new ToolEffect.FileEdit("中文😀"))).withEffects(List.of(effect));
        assertEquals(List.of(new ToolEffect.FileEdit("中文😀"), effect), result.effects());
        assertTrue(source.effects().isEmpty());
        assertFalse(new ToolEffect.TestRun("test", true, null).toJson().has("failure_count"));
        for (String invalid : List.of("{\"effect\":\"unknown\"}", "{\"effect\":\"file_edit\",\"path\":\"x\",\"unknown\":1}",
                "{\"effect\":\"test_run\",\"command\":\"c\",\"passed\":false,\"failure_count\":\"01\"}",
                "{\"effect\":\"test_run\",\"command\":\"c\",\"passed\":\"false\"}")) {
            var value = json(invalid); assertThrows(IllegalArgumentException.class, () -> ToolEffect.from(value));
        }
    }
}
