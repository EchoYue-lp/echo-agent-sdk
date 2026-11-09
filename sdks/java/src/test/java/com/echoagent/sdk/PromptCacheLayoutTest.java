package com.echoagent.sdk;

import com.fasterxml.jackson.databind.JsonNode;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.DynamicTest;
import org.junit.jupiter.api.TestFactory;

import java.util.stream.Stream;

import java.util.List;
import java.util.Map;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;

class PromptCacheLayoutTest {
    private static JsonNode message(String role, String text) {
        return JsonSupport.MAPPER.createObjectNode()
                .put("role", role)
                .set("content", WireValues.value(text));
    }

    private static String tail(String body) {
        return "<echo-agent-context-projection-v2>\nmarker=\"runtime\"\nplacement=tail\nversion=\"v1\"\n\n" + body;
    }

    @Test
    void projectsRustCacheSegmentsAndRanges() {
        var layout = PromptCacheLayout.fromMessages(
                List.of(
                        message("system", "You are Echo Agent"),
                        message("system", "[Canonical context - restored]"),
                        message("user", "hello"),
                        message("assistant", "hi"),
                        message("user", tail("turn 1")),
                        message("user", tail("hook"))),
                List.of(JsonSupport.MAPPER.createObjectNode().put("name", "lookup")));

        assertEquals(1, layout.system().size());
        assertEquals(1, layout.canonical().size());
        assertEquals(2, layout.history().size());
        assertEquals(2, layout.runtimeContext().size());
        assertEquals(1, layout.tools().size());
        var ranges = layout.segmentRanges();
        assertEquals(new SegmentRange(0L, 1L), ranges.system());
        assertEquals(new SegmentRange(1L, 2L), ranges.canonical());
        assertEquals(new SegmentRange(2L, 4L), ranges.history());
        assertEquals(new SegmentRange(4L, 6L), ranges.runtimeContext());
    }

    @Test
    void leavesCanonicalEmptyWithoutMarker() {
        var layout = PromptCacheLayout.fromMessages(
                List.of(message("system", "S"), message("user", "hello")), List.of());
        assertEquals(0, layout.canonical().size());
        assertEquals(1, layout.history().size());
        assertTrue(layout.segmentRanges().canonical().isEmpty());
    }

    private record HeaderCase(String name, String text, boolean dynamic) {}

    @TestFactory
    Stream<DynamicTest> strictV2HeaderCases() {
        return List.of(
                new HeaderCase("valid_tail", "<echo-agent-context-projection-v2>\nmarker=\"runtime\"\nplacement=tail\nversion=\"v1\"\n\n事实🙂", true),
                new HeaderCase("valid_empty_body", "<echo-agent-context-projection-v2>\nmarker=\"runtime\"\nplacement=tail\nversion=\"v1\"\n\n", true),
                new HeaderCase("valid_json_whitespace", "<echo-agent-context-projection-v2>\nmarker= \" \" \nplacement=tail\nversion= \"v\" \n\nbody", true),
                new HeaderCase("valid_json_unicode", "<echo-agent-context-projection-v2>\nmarker=\"\\u4e2d\\ud83d\\ude42\"\nplacement=tail\nversion=\"\\u7248\"\n\nbody", true),
                new HeaderCase("valid_crlf_header_lines", "<echo-agent-context-projection-v2>\nmarker=\"runtime\"\r\nplacement=tail\r\nversion=\"v1\"\n\nbody", true),
                new HeaderCase("bare_prefix", "[runtime_context: user text]", false),
                new HeaderCase("bare_prefix_after_spaces", "  [runtime_context: user text]", false),
                new HeaderCase("legacy_v1", "<echo-agent-context-projection-v1>\nmarker=\"runtime\"\nversion=\"body\"", false),
                new HeaderCase("boundary", "<echo-agent-context-projection-v2>\nmarker=\"runtime\"\nplacement=boundary\n\nbody", false),
                new HeaderCase("boundary_body_version", "<echo-agent-context-projection-v2>\nmarker=\"runtime\"\nplacement=boundary\n\nversion=\"body\"", false),
                new HeaderCase("unterminated_header", "<echo-agent-context-projection-v2>\nmarker=\"runtime\"\nplacement=tail\nversion=\"v1\"", false),
                new HeaderCase("missing_version", "<echo-agent-context-projection-v2>\nmarker=\"runtime\"\nplacement=tail\n\nversion=\"body\"", false),
                new HeaderCase("extra_header", "<echo-agent-context-projection-v2>\nmarker=\"runtime\"\nplacement=tail\nversion=\"v1\"\nextra=\"x\"\n\nbody", false),
                new HeaderCase("empty_marker", "<echo-agent-context-projection-v2>\nmarker=\"\"\nplacement=tail\nversion=\"v1\"\n\nbody", false),
                new HeaderCase("empty_version", "<echo-agent-context-projection-v2>\nmarker=\"runtime\"\nplacement=tail\nversion=\"\"\n\nbody", false),
                new HeaderCase("numeric_marker", "<echo-agent-context-projection-v2>\nmarker=1\nplacement=tail\nversion=\"v1\"\n\nbody", false),
                new HeaderCase("numeric_version", "<echo-agent-context-projection-v2>\nmarker=\"runtime\"\nplacement=tail\nversion=1\n\nbody", false),
                new HeaderCase("bad_json_escape", "<echo-agent-context-projection-v2>\nmarker=\"\\q\"\nplacement=tail\nversion=\"v1\"\n\nbody", false),
                new HeaderCase("trailing_marker_json", "<echo-agent-context-projection-v2>\nmarker=\"runtime\" true\nplacement=tail\nversion=\"v1\"\n\nbody", false),
                new HeaderCase("trailing_version_json", "<echo-agent-context-projection-v2>\nmarker=\"runtime\"\nplacement=tail\nversion=\"v1\" \"v2\"\n\nbody", false),
                new HeaderCase("unpaired_marker_surrogate", "<echo-agent-context-projection-v2>\nmarker=\"\\ud800\"\nplacement=tail\nversion=\"v1\"\n\nbody", false),
                new HeaderCase("unpaired_version_surrogate", "<echo-agent-context-projection-v2>\nmarker=\"runtime\"\nplacement=tail\nversion=\"\\udc00\"\n\nbody", false),
                new HeaderCase("leading_whitespace", " <echo-agent-context-projection-v2>\nmarker=\"runtime\"\nplacement=tail\nversion=\"v1\"\n\nbody", false),
                new HeaderCase("wrong_header_order", "<echo-agent-context-projection-v2>\nplacement=tail\nmarker=\"runtime\"\nversion=\"v1\"\n\nbody", false),
                new HeaderCase("extra_boundary_version", "<echo-agent-context-projection-v2>\nmarker=\"runtime\"\nplacement=boundary\nversion=\"v1\"\n\nbody", false),
                new HeaderCase("non_tail_placement", "<echo-agent-context-projection-v2>\nmarker=\"runtime\"\nplacement=Tail\nversion=\"v1\"\n\nbody", false)
        ).stream().map(example -> DynamicTest.dynamicTest(example.name(), () -> {
            var messages = List.of(message("system", "S"), message("user", "history"), message("user", example.text()));
            var tools = List.of(JsonSupport.MAPPER.createObjectNode().put("name", "second"), JsonSupport.MAPPER.createObjectNode().put("name", "first"));
            var beforeMessages = messages.stream().map(JsonNode::deepCopy).toList();
            var beforeTools = tools.stream().map(JsonNode::deepCopy).toList();
            var layout = PromptCacheLayout.fromMessages(messages, tools);
            assertEquals(example.dynamic() ? messages.subList(1, 2) : messages.subList(1, 3), layout.history());
            assertEquals(example.dynamic() ? messages.subList(2, 3) : List.of(), layout.runtimeContext());
            assertEquals(tools, layout.tools());
            assertEquals(new SegmentRange(example.dynamic() ? 2L : 3L, 3L), layout.segmentRanges().runtimeContext());
            assertEquals(beforeMessages, messages);
            assertEquals(beforeTools, tools);
        }));
    }

    @Test
    void stopsAtOrdinaryHistoryAndAnySystemRole() {
        var system = message("system", "S");
        var history = message("user", "history");
        var fact = message("user", tail("fact"));
        var lateSystem = message("system", tail("system fact"));
        for (var messages : List.of(List.of(system, fact, history), List.of(system, history, fact, lateSystem))) {
            var layout = PromptCacheLayout.fromMessages(messages, List.of());
            assertEquals(messages.subList(1, messages.size()), layout.history());
            assertEquals(List.of(), layout.runtimeContext());
        }
        var messages = List.of(system, history, fact, lateSystem, fact);
        var layout = PromptCacheLayout.fromMessages(messages, List.of());
        assertEquals(messages.subList(1, 4), layout.history());
        assertEquals(List.of(fact), layout.runtimeContext());
        assertEquals(new SegmentRange(4L, 5L), layout.segmentRanges().runtimeContext());
    }

    @Test
    void onlyFirstTextPartCanSupplyEnvelope() {
        // WireValues 接受原生 Map/List；JsonNode 是 Iterable，不能据此假定编码为字段对象。
        var valid = Map.<String, Object>of("type", "text", "text", tail("fact"));
        var image = Map.<String, Object>of("type", "image_url", "image_url", Map.of("url", "data:image/png;base64,AA=="));
        var splitHead = Map.<String, Object>of("type", "text", "text", "<echo-agent-context-projection-v2>\n");
        var splitRest = Map.<String, Object>of("type", "text", "text", "marker=\"runtime\"\nplacement=tail\nversion=\"v1\"\n\nbody");
        for (var parts : List.of(List.of(valid), List.of(image, valid), List.of(splitHead, splitRest))) {
            var partMessage = JsonSupport.MAPPER.createObjectNode().put("role", "user");
            JsonNode content = WireValues.value(parts);
            assertEquals(JsonSupport.MAPPER.valueToTree(parts), JsonSupport.fromWire(content));
            partMessage.set("content", content);
            var layout = PromptCacheLayout.fromMessages(List.of(message("system", "S"), partMessage), List.of());
            assertEquals(parts.get(0) == valid ? 1 : 0, layout.runtimeContext().size());
        }
    }

    @Test
    void handlesEmptyAndEntirelySystemMessages() {
        for (List<JsonNode> messages : List.of(List.<JsonNode>of(), List.of(message("system", tail("fact")), message("system", "S")))) {
            var layout = PromptCacheLayout.fromMessages(messages, List.of());
            assertEquals(messages, layout.system());
            assertEquals(List.of(), layout.history());
            assertEquals(List.of(), layout.runtimeContext());
        }
    }

    private record CanonicalCase(String name, String partsJson, boolean canonical) {}

    @TestFactory
    Stream<DynamicTest> canonicalTextViewCases() {
        return List.of(
                new CanonicalCase("single_text", "[{\"type\": \"text\", \"text\": \"Canonical context\"}]", true),
                new CanonicalCase("split_text_no_separator", "[{\"type\": \"text\", \"text\": \"Canonical \"}, {\"type\": \"text\", \"text\": \"context\"}]", true),
                new CanonicalCase("split_text_no_inserted_space", "[{\"type\": \"text\", \"text\": \"Canonical\"}, {\"type\": \"text\", \"text\": \"context\"}]", false),
                new CanonicalCase("ignored_media_between_text", "[{\"type\": \"text\", \"text\": \"Canonical\"}, {\"type\": \"file\", \"name\": \"ignored\", \"content\": \"AA==\"}, {\"type\": \"text\", \"text\": \" context\"}]", true),
                new CanonicalCase("media_names_are_not_text", "[{\"type\": \"file\", \"name\": \"Canonical context\", \"content\": \"AA==\"}, {\"type\": \"image_url\", \"image_url\": {\"url\": \"Canonical context\"}}]", false),
                new CanonicalCase("empty_parts", "[]", false),
                new CanonicalCase("empty_text", "[{\"type\": \"text\", \"text\": \"\"}]", false),
                new CanonicalCase("resource_title_missing_optionals", "[{\"type\": \"resource_link\", \"resource\": {\"name\": \"report\", \"uri\": \"resource://report\", \"title\": \"Canonical context\"}}]", true),
                new CanonicalCase("resource_name", "[{\"type\": \"resource_link\", \"resource\": {\"name\": \"Canonical context\", \"uri\": \"resource://report\"}}]", true),
                new CanonicalCase("resource_uri", "[{\"type\": \"resource_link\", \"resource\": {\"name\": \"report\", \"uri\": \"Canonical context\"}}]", true),
                new CanonicalCase("resource_description", "[{\"type\": \"resource_link\", \"resource\": {\"name\": \"report\", \"uri\": \"resource://report\", \"description\": \"Canonical context\"}}]", true),
                new CanonicalCase("resource_mime", "[{\"type\": \"resource_link\", \"resource\": {\"name\": \"report\", \"uri\": \"resource://report\", \"mime_type\": \"Canonical context\"}}]", true),
                new CanonicalCase("resource_annotations_value", "[{\"type\": \"resource_link\", \"resource\": {\"name\": \"report\", \"uri\": \"resource://report\", \"annotations\": {\"note\": \"Canonical context\"}}}]", true),
                new CanonicalCase("resource_annotations_key", "[{\"type\": \"resource_link\", \"resource\": {\"name\": \"report\", \"uri\": \"resource://report\", \"annotations\": {\"Canonical context\": true}}}]", true),
                new CanonicalCase("resource_meta", "[{\"type\": \"resource_link\", \"resource\": {\"name\": \"report\", \"uri\": \"resource://report\", \"meta\": {\"note\": \"Canonical context\"}}}]", true),
                new CanonicalCase("resource_minimal_no_marker", "[{\"type\": \"resource_link\", \"resource\": {\"name\": \"report\", \"uri\": \"resource://report\"}}]", false),
                new CanonicalCase("resource_unknown_field_ignored", "[{\"type\": \"resource_link\", \"resource\": {\"name\": \"report\", \"uri\": \"resource://report\", \"extra\": \"Canonical context\"}}]", false),
                new CanonicalCase("resource_prevents_text_join", "[{\"type\": \"text\", \"text\": \"Canonical\"}, {\"type\": \"resource_link\", \"resource\": {\"name\": \"report\", \"uri\": \"resource://report\"}}, {\"type\": \"text\", \"text\": \" context\"}]", false),
                new CanonicalCase("resource_fields_do_not_join", "[{\"type\": \"resource_link\", \"resource\": {\"name\": \"Canonical\", \"uri\": \" context\"}}]", false),
                new CanonicalCase("resource_json_newline_not_space", "[{\"type\": \"resource_link\", \"resource\": {\"name\": \"report\", \"uri\": \"resource://report\", \"title\": \"Canonical\\ncontext\"}}]", false),
                new CanonicalCase("resource_empty_optionals", "[{\"type\": \"resource_link\", \"resource\": {\"name\": \"report\", \"uri\": \"resource://report\", \"description\": \"\", \"mime_type\": \"\", \"title\": \"\", \"annotations\": null, \"meta\": null, \"size\": null}}]", false),
                new CanonicalCase("text_after_resource", "[{\"type\": \"resource_link\", \"resource\": {\"name\": \"report\", \"uri\": \"resource://report\"}}, {\"type\": \"text\", \"text\": \"Canonical context\"}]", true)
        ).stream().map(example -> DynamicTest.dynamicTest(example.name(), () -> {
            List<?> parts = JsonSupport.MAPPER.readValue(example.partsJson(), List.class);
            var system = JsonSupport.MAPPER.createObjectNode().put("role", "system");
            JsonNode content = WireValues.value(parts);
            assertEquals(JsonSupport.MAPPER.readTree(example.partsJson()), JsonSupport.fromWire(content));
            system.set("content", content);
            JsonNode before = system.deepCopy();
            var layout = PromptCacheLayout.fromMessages(List.of(system), List.of());
            assertEquals(example.canonical() ? List.of() : List.of(system), layout.system());
            assertEquals(example.canonical() ? List.of(system) : List.of(), layout.canonical());
            assertEquals(List.of(), layout.history());
            assertEquals(List.of(), layout.runtimeContext());
            assertEquals(example.canonical() ? 0L : 1L, layout.segmentRanges().system().end());
            assertEquals(1L, layout.segmentRanges().canonical().end());
            assertEquals(before, system);
        }));
    }

    @Test
    void multipartCanonicalKeepsSystemBoundaryAndDynamicFirstPart() {
        var canonical = JsonSupport.MAPPER.createObjectNode().put("role", "system");
        canonical.set("content", WireValues.value(List.of(Map.of("type", "text", "text", "Canonical context"))));
        var system = message("system", "S");
        var history = message("user", "history");
        var fact = message("user", tail("fact"));
        var layout = PromptCacheLayout.fromMessages(List.of(system, canonical, system, history, fact), List.of());
        assertEquals(List.of(system), layout.system());
        assertEquals(List.of(canonical, system), layout.canonical());
        assertEquals(List.of(history), layout.history());
        assertEquals(List.of(fact), layout.runtimeContext());
        var late = PromptCacheLayout.fromMessages(List.of(system, history, canonical), List.of());
        assertEquals(List.of(), late.canonical());
        assertEquals(List.of(history, canonical), late.history());
        assertEquals(List.of(), late.runtimeContext());
        var resourceThenText = JsonSupport.MAPPER.createObjectNode().put("role", "user");
        resourceThenText.set("content", WireValues.value(List.of(
                Map.of("type", "resource_link", "resource", Map.of("name", "report", "uri", "resource://report", "title", tail("fact"))),
                Map.of("type", "text", "text", tail("fact")))));
        assertEquals(List.of(), PromptCacheLayout.fromMessages(List.of(resourceThenText), List.of()).runtimeContext());
    }
}
