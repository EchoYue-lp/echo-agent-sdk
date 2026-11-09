package com.echoagent.sdk;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.DeserializationFeature;

import java.io.IOException;
import java.util.ArrayList;
import java.util.Collection;
import java.util.List;

/** Read-only prompt layout projection; cache placement stays in Rust. */
public final class PromptCacheLayout {
    private final List<JsonNode> system;
    private final List<JsonNode> canonical;
    private final List<JsonNode> history;
    private final List<JsonNode> runtimeContext;
    private final List<JsonNode> tools;

    private PromptCacheLayout(
            List<JsonNode> system,
            List<JsonNode> canonical,
            List<JsonNode> history,
            List<JsonNode> runtimeContext,
            List<JsonNode> tools) {
        this.system = List.copyOf(system);
        this.canonical = List.copyOf(canonical);
        this.history = List.copyOf(history);
        this.runtimeContext = List.copyOf(runtimeContext);
        this.tools = List.copyOf(tools);
    }

    public static PromptCacheLayout fromMessages(
            Collection<? extends JsonNode> messages,
            Collection<? extends JsonNode> tools) {
        List<JsonNode> messageValues = snapshots(messages, "messages");
        List<JsonNode> toolValues = snapshots(tools, "tools");
        int systemEnd = messageValues.size();
        for (int index = 0; index < messageValues.size(); index++) {
            if (!"system".equals(messageValues.get(index).path("role").asText())) {
                systemEnd = index;
                break;
            }
        }

        int canonicalStart = systemEnd;
        for (int index = 0; index < systemEnd; index++) {
            String text = messageText(messageValues.get(index));
            if (text != null && text.contains("Canonical context")) {
                canonicalStart = index;
                break;
            }
        }

        int runtimeStart = messageValues.size();
        // 保留公开 runtimeContext 名称，值对应框架严格识别的连续 dynamic 尾段。
        while (runtimeStart > systemEnd) {
            JsonNode message = messageValues.get(runtimeStart - 1);
            if ("system".equals(message.path("role").asText()) || !isDynamicTailMessage(message)) break;
            runtimeStart -= 1;
        }

        return new PromptCacheLayout(
                messageValues.subList(0, Math.min(canonicalStart, systemEnd)),
                canonicalStart < systemEnd ? messageValues.subList(canonicalStart, systemEnd) : List.of(),
                messageValues.subList(systemEnd, runtimeStart),
                messageValues.subList(runtimeStart, messageValues.size()),
                toolValues);
    }

    public List<JsonNode> system() { return copies(system); }
    public List<JsonNode> canonical() { return copies(canonical); }
    public List<JsonNode> history() { return copies(history); }
    public List<JsonNode> runtimeContext() { return copies(runtimeContext); }
    public List<JsonNode> tools() { return copies(tools); }

    public SegmentRanges segmentRanges() {
        long systemLength = system.size();
        long canonicalLength = canonical.size();
        long historyLength = history.size();
        long runtimeLength = runtimeContext.size();
        long canonicalEnd = systemLength + canonicalLength;
        long historyEnd = canonicalEnd + historyLength;
        return new SegmentRanges(
                new SegmentRange(0L, systemLength),
                new SegmentRange(systemLength, canonicalEnd),
                new SegmentRange(canonicalEnd, historyEnd),
                new SegmentRange(historyEnd, historyEnd + runtimeLength));
    }

    private static List<JsonNode> snapshots(Collection<? extends JsonNode> values, String name) {
        if (values == null) throw new IllegalArgumentException(name + " must not be null");
        var snapshots = new ArrayList<JsonNode>();
        for (JsonNode value : values) {
            if (value == null || !value.isObject()) {
                throw new IllegalArgumentException(name + " entries must be JSON objects");
            }
            snapshots.add(value.deepCopy());
        }
        return List.copyOf(snapshots);
    }

    private static List<JsonNode> copies(List<JsonNode> values) {
        var copies = new ArrayList<JsonNode>();
        for (JsonNode value : values) copies.add(value.deepCopy());
        return List.copyOf(copies);
    }

    private static String messageText(JsonNode message) {
        JsonNode content = JsonSupport.fromWire(message.get("content"));
        if (content == null) return null;
        if (content.isTextual()) return content.textValue().isEmpty() ? null : content.textValue();
        if (!content.isArray()) return null;
        var texts = new ArrayList<String>();
        for (JsonNode part : content) {
            if ("text".equals(part.path("type").asText()) && part.path("text").isTextual()) {
                texts.add(part.get("text").textValue());
            } else if ("resource_link".equals(part.path("type").asText()) && part.path("resource").isObject()) {
                JsonNode resource = part.get("resource");
                var fields = JsonSupport.MAPPER.createObjectNode();
                // 缺失 Option 为 null；只投影 LinkedResource 原字段，忽略未知字段。
                for (String name : List.of("annotations", "description", "mime_type", "name", "size", "title", "uri", "meta")) {
                    fields.set(name, resource.has(name) ? resource.get(name)
                            : com.fasterxml.jackson.databind.node.NullNode.instance);
                }
                texts.add("[Linked resource]\n" + fields);
            }
        }
        // 只供 canonical 使用；dynamic 的首 Text part 信封读取保持独立。
        return texts.isEmpty() ? null : String.join("", texts);
    }

    private static boolean isDynamicTailMessage(JsonNode message) {
        JsonNode content = JsonSupport.fromWire(message.get("content"));
        if (content != null && content.isArray()) {
            JsonNode first = content.isEmpty() ? null : content.get(0);
            // 与框架一致：只能从首个 Text part 取完整信封，不能合并正文补头。
            content = first != null && "text".equals(first.path("type").asText())
                    ? first.get("text") : null;
        }
        if (content == null || !content.isTextual()) return false;
        String text = content.textValue();
        String prefix = "<echo-agent-context-projection-v2>\n";
        if (!text.startsWith(prefix)) return false;
        int end = text.indexOf("\n\n", prefix.length());
        if (end < 0) return false;
        String[] lines = text.substring(prefix.length(), end).split("\\r?\\n", -1);
        return lines.length == 3 && lines[0].startsWith("marker=")
                && lines[1].equals("placement=tail") && lines[2].startsWith("version=")
                && isProjectionString(lines[0].substring("marker=".length()))
                && isProjectionString(lines[2].substring("version=".length()));
    }

    private static boolean isProjectionString(String raw) {
        try {
            JsonNode value = JsonSupport.MAPPER.reader()
                    .with(DeserializationFeature.FAIL_ON_TRAILING_TOKENS).readTree(raw);
            // 不做数字到字符串强制转换；补齐 serde_json 对未配对 surrogate 的拒绝。
            return value != null && value.isTextual() && !value.textValue().isEmpty()
                    && value.textValue().codePoints().noneMatch(cp -> cp >= 0xD800 && cp <= 0xDFFF);
        } catch (IOException error) {
            return false;
        }
    }
}
