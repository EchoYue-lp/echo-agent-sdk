package com.echoagent.sdk;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.node.ObjectNode;
import java.util.concurrent.CompletionStage;

/** 仅由真实测试producer在原开始/物理结束处调用，不替SDK推断运行终态。 */
final class NativeProducerFixture {
    private final ExtensionStreamWriter writer;
    private final String execution;
    private final String request;

    NativeProducerFixture(EchoAgentClient sdk, JsonNode call) {
        writer = sdk.streamWriter(WireHandle.fromJson(call.path("stream")));
        execution = call.path("context").path("execution_id").asText(call.path("invocation_id").asText());
        request = call.path("context").path("message_id").asText(call.path("invocation_id").asText());
    }

    private ObjectNode status(String kind, int sequence, String phase) {
        var data = JsonSupport.MAPPER.createObjectNode()
            .put("execution_id", execution).put("request_id", request)
            .put("sequence", Integer.toString(sequence)).put("phase", phase).put("status", kind);
        return JsonSupport.MAPPER.createObjectNode().put("kind", "agent").set("value",
            JsonSupport.MAPPER.createObjectNode().put("event", "execution_status").set("data", data));
    }

    CompletionStage<Void> start() { return writer.chunk(status("started", 0, "Thinking")); }

    CompletionStage<Void> failed(JsonNode error) { return writer.failed(error); }

    CompletionStage<Void> finish(String text) { return finish(text, false); }

    CompletionStage<Void> finish(String text, boolean cancelled) {
        if (cancelled) return writer.complete(status("cancelled", 1, "Terminal"));
        var notice = JsonSupport.MAPPER.createObjectNode().put("kind", "agent").set("value",
            JsonSupport.MAPPER.createObjectNode().put("event", "final_answer").set("data",
                JsonSupport.MAPPER.createObjectNode().put("text", text)));
        return writer.chunk(notice).thenCompose(ignored -> writer.complete(status("settled", 1, "Terminal")));
    }
}
