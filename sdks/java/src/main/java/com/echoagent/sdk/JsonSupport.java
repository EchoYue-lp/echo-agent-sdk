package com.echoagent.sdk;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.databind.node.ArrayNode;
import com.fasterxml.jackson.databind.node.ObjectNode;

import java.math.BigInteger;
import java.util.Map;

final class JsonSupport {
    static final ObjectMapper MAPPER = new ObjectMapper();
    static final BigInteger MIN_I64 = BigInteger.ONE.shiftLeft(63).negate();
    static final BigInteger MAX_U64 = BigInteger.ONE.shiftLeft(64).subtract(BigInteger.ONE);

    private JsonSupport() {}

    static ObjectNode wire(Object value) {
        var result = MAPPER.createObjectNode();
        if (value == null) return result.put("kind", "null");
        if (value instanceof WireHandle handle) {
            result.put("kind", "handle");
            result.set("value", handle.toJson());
            return result;
        }
        if (value instanceof JsonNode node && node.isObject() && node.has("kind") && (java.util.Set.of("null","bool","string","i64","u64","f64","bytes","duration","timestamp","path","handle","list","map","record","variant","unknown").contains(node.path("kind").asText()) || !node.has("id") && !node.has("generation"))) {
            return (ObjectNode) node;
        }
        if (value instanceof Boolean bool) return result.put("kind", "bool").put("value", bool);
        if (value instanceof String text) return result.put("kind", "string").put("value", text);
        if (value instanceof BigInteger integer) {
            if (integer.compareTo(MIN_I64) < 0 || integer.compareTo(MAX_U64) > 0) {
                throw new IllegalArgumentException("integer is outside the i64/u64 wire range");
            }
            return result.put("kind", integer.signum() < 0 ? "i64" : "u64").put("value", integer.toString());
        }
        if (value instanceof Integer integer) return result.put("kind", integer < 0 ? "i64" : "u64").put("value", integer.toString());
        if (value instanceof Long integer) return result.put("kind", integer < 0 ? "i64" : "u64").put("value", integer.toString());
        if (value instanceof Double number) {
            if (!Double.isFinite(number)) throw new IllegalArgumentException("wire numbers must be finite");
            return result.put("kind", "f64").put("value", number);
        }
        if (value instanceof Float number) {
            if (!Float.isFinite(number)) throw new IllegalArgumentException("wire numbers must be finite");
            return result.put("kind", "f64").put("value", number);
        }
        if (value instanceof Iterable<?> iterable) {
            result.put("kind", "list");
            ArrayNode values = result.putArray("value");
            for (Object item : iterable) values.add(wire(item));
            return result;
        }
        if (value instanceof Map<?, ?> map) {
            result.put("kind", "map");
            ArrayNode entries = result.putArray("value");
            for (var entry : map.entrySet()) {
                if (!(entry.getKey() instanceof String key)) throw new IllegalArgumentException("wire map keys must be strings");
                var pair = MAPPER.createObjectNode();
                pair.set("key", wire(key));
                pair.set("value", wire(entry.getValue()));
                entries.add(pair);
            }
            return result;
        }
        throw new IllegalArgumentException("unsupported wire value: " + value.getClass().getName());
    }

    static JsonNode fromWire(JsonNode value) {
        if (value == null || !value.isObject() || !value.has("kind") || !value.get("kind").isTextual()) return value;
        return switch (value.get("kind").textValue()) {
            case "null" -> com.fasterxml.jackson.databind.node.NullNode.instance;
            case "bool" -> {
                if (!value.path("value").isBoolean()) throw projectionError("wire bool must be boolean");
                yield value.get("value");
            }
            case "string" -> {
                if (!value.path("value").isTextual()) throw projectionError("wire string must be text");
                yield value.get("value");
            }
            case "f64" -> {
                if (!value.path("value").isNumber() || !Double.isFinite(value.path("value").doubleValue())) throw projectionError("wire f64 must be finite numeric data");
                yield value.get("value");
            }
            case "i64" -> {
                WireValues.i64(requiredText(value, "value", "i64"));
                yield value.get("value");
            }
            case "u64" -> {
                WireValues.u64(requiredText(value, "value", "u64"));
                yield value.get("value");
            }
            case "handle" -> {
                var handle = WireHandle.fromJson(value.get("value"));
                // Handle generations are WireU64 on the Rust side. Keep the
                // opaque generation text, but reject non-canonical or
                // overflowing values before exposing the handle payload.
                WireValues.u64(handle.generation());
                yield value.get("value");
            }
            case "list" -> {
                if (!value.path("value").isArray()) throw projectionError("wire list requires an array value");
                var values = MAPPER.createArrayNode();
                for (JsonNode item : value.get("value")) values.add(decodeWireChild(item));
                yield values;
            }
            case "map" -> {
                if (!value.path("value").isArray()) throw projectionError("wire map requires an entry array");
                var object = MAPPER.createObjectNode();
                for (JsonNode entry : value.get("value")) {
                    var key = entry.path("key");
                    if (!entry.isObject() || !entry.has("value") || !"string".equals(key.path("kind").asText()) || !key.path("value").isTextual()) {
                        throw projectionError("native map projection requires string keys and explicit values");
                    }
                    var name = key.get("value").textValue();
                    if (object.has(name)) throw projectionError("native map projection cannot preserve duplicate keys");
                    object.set(name, decodeWireChild(entry.get("value")));
                }
                yield object;
            }
            // Preserve typed and future additive variants with their discriminator.
            default -> value;
        };
    }

    private static JsonNode decodeWireChild(JsonNode value){
        if(value==null||!value.isObject()||!value.path("kind").isTextual())throw projectionError("wire container child requires its discriminator");
        return fromWire(value);
    }

    /** Preserve raw typed/future variants; native projection rules do not restrict raw WireValue maps. */
    static JsonNode validateWire(JsonNode value) {
        if(value == null || !value.isObject() || !value.path("kind").isTextual())throw projectionError("WireValue requires its discriminator");
        var kind=value.path("kind").textValue();
        if(java.util.Set.of("bool","string","f64","i64","u64","handle").contains(kind))fromWire(value);
        else if("list".equals(kind)){
            if(!value.path("value").isArray())throw projectionError("wire list requires array value");
            value.path("value").forEach(JsonSupport::validateWire);
        } else if("map".equals(kind)){
            if(!value.path("value").isArray())throw projectionError("wire map requires entry array");
            for(var entry:value.path("value")){
                if(!entry.isObject()||!entry.has("key")||!entry.has("value"))throw projectionError("wire map entry requires key/value");
                validateWire(entry.get("key"));validateWire(entry.get("value"));
            }
        } else if(!"null".equals(kind)&&!value.has("value"))throw projectionError("typed wire value requires payload");
        return value.deepCopy();
    }

    static JsonNode responseValue(JsonNode response) {
        if (response == null || !response.isObject() || !response.path("value").isObject() || !response.path("value").path("kind").isTextual()) throw projectionError("facade response requires a value envelope");
        return fromWire(response.get("value"));
    }

    private static EchoAgentException projectionError(String message) {
        return new EchoAgentException("serialization_violation", message, "never", null, null);
    }

    private static String requiredText(JsonNode object, String field, String kind) {
        var node = object.get(field);
        if (node == null || !node.isTextual()) {
            throw new IllegalArgumentException(kind + " wire value must be canonical decimal text");
        }
        return node.textValue();
    }
}
