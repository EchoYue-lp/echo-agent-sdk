package com.echoagent.sdk;

import com.agentclientprotocol.sdk.json.AcpJsonMapper;
import com.agentclientprotocol.sdk.json.TypeRef;
import com.agentclientprotocol.sdk.spec.AcpClientSession;
import com.agentclientprotocol.sdk.spec.AcpClientTransport;
import com.agentclientprotocol.sdk.spec.AcpSchema;
import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;
import org.junit.jupiter.api.Test;
import reactor.core.publisher.Mono;

import java.nio.file.Path;
import java.time.Duration;
import java.util.ArrayList;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import java.util.concurrent.ExecutionException;
import java.util.concurrent.TimeUnit;
import java.util.function.Function;

import static org.junit.jupiter.api.Assertions.*;

/** Uses existing public value encoding and invocation response projection.
 * The controlled transport echoes values only; it implements no testing API,
 * Mock executor, persistence or Host state authority. */
class TestingContractTest {
    private static final ObjectMapper JSON = new ObjectMapper();
    private static JsonNode corpus() throws Exception {
        return JSON.readTree(Path.of("../shared/fixtures/testing-contract-values.json").toFile());
    }
    private static Object nativeInput(JsonNode node) {
        if (node.isNull()) return null;
        if (node.isBoolean()) return node.booleanValue();
        if (node.isTextual()) return node.textValue();
        if (node.isIntegralNumber()) return node.longValue();
        if (node.isArray()) {
            var values = new ArrayList<Object>();
            node.forEach(item -> values.add(nativeInput(item)));
            return values;
        }
        if (node.isObject() && node.has("kind")) return node.deepCopy();
        var values = new LinkedHashMap<String, Object>();
        node.fields().forEachRemaining(entry -> values.put(entry.getKey(), nativeInput(entry.getValue())));
        return values;
    }
    private static final class EchoTransport implements AcpClientTransport {
        private final AcpJsonMapper mapper = AcpJsonMapper.createDefault();
        private Function<Mono<AcpSchema.JSONRPCMessage>, Mono<AcpSchema.JSONRPCMessage>> handler;
        JsonNode captured;
        JsonNode override;
        @Override public Mono<Void> connect(Function<Mono<AcpSchema.JSONRPCMessage>, Mono<AcpSchema.JSONRPCMessage>> handler) {
            this.handler = handler;
            return Mono.empty();
        }
        @Override public Mono<Void> sendMessage(AcpSchema.JSONRPCMessage message) {
            return Mono.defer(() -> {
                if (message instanceof AcpSchema.JSONRPCRequest request) {
                    var params = JSON.valueToTree(request.params());
                    captured = params.path("arguments").get(0).deepCopy();
                    var result = JSON.createObjectNode();
                    result.set("value", override == null ? captured.deepCopy() : override.deepCopy());
                    return handler.apply(Mono.just(new AcpSchema.JSONRPCResponse("2.0", request.id(), result, null))).then();
                }
                return Mono.empty();
            });
        }
        @Override public <T> T unmarshalFrom(Object value, TypeRef<T> type) { return mapper.convertValue(value, type); }
        @Override public Mono<Void> closeGracefully() { return Mono.empty(); }
    }
    private static EchoAgentClient client(EchoTransport transport) throws Exception {
        // Reuse the same constructor injection as existing transport tests;
        // conversion itself goes through public invoke, not private decoder.
        var session = new AcpClientSession(Duration.ofSeconds(2), transport, Map.of(), Map.of(), Function.identity());
        var catalog = new FacadeCatalog(Path.of("../shared/facade-operation-catalog.json"), Path.of("../shared/contract-digests.json"));
        var constructor = EchoAgentClient.class.getDeclaredConstructor(AcpClientSession.class, FacadeCatalog.class, AcpClientTransport.class);
        constructor.setAccessible(true);
        return constructor.newInstance(session, catalog, transport);
    }
    @Test void acceptedPayloadsUsePublicEncodingAndResponseProjection() throws Exception {
        var transport = new EchoTransport();
        try (var sdk = client(transport)) {
            for (var item : corpus().path("cases")) {
                var nativeValue = nativeInput(item.path("native_input"));
                var encoded = WireValues.value(nativeValue);
                assertEquals(item.path("expected_wire"), encoded, item.path("name").asText());
                var projected = sdk.invoke("echo_core::agent::Agent::name", null, List.of(nativeValue))
                        .toCompletableFuture().get(3, TimeUnit.SECONDS);
                assertEquals(encoded, transport.captured);
                assertEquals(item.path("expected_projection"), projected, item.path("name").asText());
                if (item.path("name").asText().equals("context-absence-distinct")) assertFalse(projected.has("call_id"));
                if (item.path("name").asText().equals("context-null-identities")) assertTrue(projected.path("call_id").isNull());
                if (item.path("name").asText().equals("delay-large-subsecond")) {
                    assertEquals("18446744073709551615", projected.path("value").path("seconds").textValue());
                    assertEquals(999999999, projected.path("value").path("nanos").intValue());
                }
            }
        }
    }
    @Test void malformedResponseProjectionFailsExplicitly() throws Exception {
        var transport = new EchoTransport();
        try (var sdk = client(transport)) {
            for (var item : corpus().path("invalid_cases")) {
                transport.override = item.path("wire");
                var failure = assertThrows(ExecutionException.class,
                        () -> sdk.invoke("echo_core::agent::Agent::name", null, List.of("fixture-input"))
                                .toCompletableFuture().get(3, TimeUnit.SECONDS), item.path("name").asText());
                assertNotNull(failure.getCause());
                assertFalse(failure.getCause().getMessage().isBlank());
            }
        }
    }
}
