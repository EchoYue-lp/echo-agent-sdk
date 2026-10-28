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
    private static JsonNode nativeCallbackInput() throws Exception {
        return json("""
                {"agent":"中文🧪","messages":[{
                  "role":"tool","name":"actual-tool","tool_call_id":"call-original",
                  "content":{"kind":"string","value":"完整消息"},
                  "tool_calls":[{"id":"call-original","call_type":"function","function_name":"tool","arguments":"{}",
                    "provider_source":{"protocol":"responses","request_id":"request-original","item_id":"item-tool",
                      "output_index":4294967295,"content_index":0,"content_kind":"function_call","phase":"future-provider-phase"}}],
                  "reasoning_blocks":[{"kind":"opaque","provider":"responses","id":"opaque-original","data":"encrypted-original","summary":["原摘要"]}],
                  "assistant_blocks":[{"block_id":"block-original","request_id":"request-original","sequence":"18446744073709551615",
                    "phase":"unknown","delta":"安全候选","complete":true,"provider":"responses",
                    "provider_source":{"protocol":"responses","request_id":"request-original","item_id":"item-reasoning",
                      "output_index":4294967295,"content_index":4294967295,"content_kind":"reasoning","phase":"future-provider-phase"},
                    "reasoning":{"kind":"signed","thinking":"原推理","signature":"signature-original"}}],
                  "approved_assistant_blocks":[],
                  "execution_status":{"execution_id":"execution-original","request_id":"request-original",
                    "sequence":"18446744073709551615","phase":"Thinking","status":"candidate","reason":"still-running"}
                }]}
                """);
    }
    @Test void nativeCallbackMessagePreservesCurrentFieldsWithoutInferringCompletion() throws Exception {
        var input = nativeCallbackInput();
        var callback = CallbackCall.from(envelope("callback_on_think_start", input));
        // 按真实JSON编码核对；Jackson节点内部的Int/Long存储宽度不属于wire语义。
        assertEquals(input, json(callback.input().toString()));
        var message = assertInstanceOf(CallbackCall.ThinkStart.class, callback.request()).messages().get(0);
        assertEquals(new BigInteger("18446744073709551615"), message.assistantBlocks().get(0).sequence());
        assertEquals(4294967295L, message.toolCalls().get(0).providerSource().outputIndex());
        assertEquals("future-provider-phase", message.assistantBlocks().get(0).providerSource().phase());
        assertTrue(callback.input().path("messages").path(0).path("approved_assistant_blocks").isEmpty());
        assertEquals("candidate", callback.input().path("messages").path(0).path("execution_status").path("status").textValue());
        // 回调成功只确认原通知返回unit，不把候选正文或消息内status升级为Run完成。
        assertTrue(callback.completed().toJson().path("result").path("value").isNull());
        var absent = (com.fasterxml.jackson.databind.node.ObjectNode) input.path("messages").path(0);
        absent.remove("approved_assistant_blocks");
        assertEquals(input, json(CallbackCall.from(envelope("callback_on_think_start", input)).input().toString()));
        assertFalse(CallbackCall.from(envelope("callback_on_think_start", input)).input()
                .path("messages").path(0).has("approved_assistant_blocks"));
    }
    @Test void callbackNativeOptionalFieldsRetainTheirCurrentSemantics() throws Exception {
        var input = nativeCallbackInput();
        var message = (com.fasterxml.jackson.databind.node.ObjectNode) input.path("messages").path(0);
        var block = (com.fasterxml.jackson.databind.node.ObjectNode) message.path("assistant_blocks").path(0);
        block.remove("phase");
        var projected = CallbackCall.from(envelope("callback_on_think_start", input)).input();
        assertEquals("unknown", projected.path("messages").path(0).path("assistant_blocks").path(0).path("phase").textValue());
        block.put("phase", "final_answer");
        message.set("approved_assistant_blocks", message.path("assistant_blocks").deepCopy());
        message.putArray("reasoning_blocks").addObject().put("kind", "redacted").put("data", "原密文");
        assertEquals(input, json(CallbackCall.from(envelope("callback_on_think_start", input)).input().toString()));
    }
    private static void rejectNative(java.util.function.Consumer<com.fasterxml.jackson.databind.node.ObjectNode> mutate) throws Exception {
        var input = nativeCallbackInput();
        mutate.accept((com.fasterxml.jackson.databind.node.ObjectNode) input.path("messages").path(0));
        assertThrows(IllegalArgumentException.class, () -> CallbackCall.from(envelope("callback_on_think_start", input)));
    }
    @Test void nativeCallbackRejectsUnknownFieldsAndMalformedCoordinates() throws Exception {
        rejectNative(message -> message.put("unknown_native", true));
        rejectNative(message -> ((com.fasterxml.jackson.databind.node.ObjectNode) message.path("assistant_blocks").path(0)).put("unknown_native", true));
        rejectNative(message -> ((com.fasterxml.jackson.databind.node.ObjectNode) message.path("execution_status")).put("adapter_generated", true));
        rejectNative(message -> ((com.fasterxml.jackson.databind.node.ObjectNode) message.path("assistant_blocks").path(0)).put("sequence", "01"));
        rejectNative(message -> ((com.fasterxml.jackson.databind.node.ObjectNode) message.path("assistant_blocks").path(0)).put("sequence", "18446744073709551616"));
        rejectNative(message -> ((com.fasterxml.jackson.databind.node.ObjectNode) message.path("execution_status")).put("execution_id", ""));
        rejectNative(message -> ((com.fasterxml.jackson.databind.node.ObjectNode) message.path("execution_status")).put("phase", "final_answer"));
        rejectNative(message -> ((com.fasterxml.jackson.databind.node.ObjectNode) message.path("execution_status")).put("status", "completed"));
        rejectNative(message -> ((com.fasterxml.jackson.databind.node.ObjectNode) message.path("assistant_blocks").path(0)).putNull("phase"));
        rejectNative(message -> ((com.fasterxml.jackson.databind.node.ObjectNode) message.path("assistant_blocks").path(0)).remove("complete"));
        rejectNative(message -> ((com.fasterxml.jackson.databind.node.ObjectNode) message.path("assistant_blocks").path(0)).put("phase", "future-provider-phase"));
        rejectNative(message -> ((com.fasterxml.jackson.databind.node.ObjectNode) message.path("tool_calls").path(0).path("provider_source")).put("output_index", -1));
        rejectNative(message -> ((com.fasterxml.jackson.databind.node.ObjectNode) message.path("tool_calls").path(0).path("provider_source")).put("output_index", 4294967296L));
        rejectNative(message -> ((com.fasterxml.jackson.databind.node.ObjectNode) message.path("tool_calls").path(0).path("provider_source")).put("output_index", 1.5));
        rejectNative(message -> ((com.fasterxml.jackson.databind.node.ObjectNode) message.path("tool_calls").path(0).path("provider_source")).put("output_index", "1"));
        rejectNative(message -> ((com.fasterxml.jackson.databind.node.ObjectNode) message.path("tool_calls").path(0).path("provider_source")).remove("output_index"));
        rejectNative(message -> ((com.fasterxml.jackson.databind.node.ObjectNode) message.path("tool_calls").path(0).path("provider_source")).put("protocol", "unknown"));
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
