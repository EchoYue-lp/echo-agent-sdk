import type { LlmMessage, LlmToolDefinition } from "./types.js";
import { SegmentRange } from "./segment_range_values.js";
import { fromWireValue } from "./wire.js";

/** Message-cache segment indexes without owning cache state. */
export class SegmentRanges {
  public constructor(
    public readonly system: SegmentRange = new SegmentRange(),
    public readonly canonical: SegmentRange = new SegmentRange(),
    public readonly history: SegmentRange = new SegmentRange(),
    public readonly runtimeContext: SegmentRange = new SegmentRange(),
  ) {
    Object.freeze(this);
  }
}

/** Read-only prompt layout projection; cache storage and provider placement stay in Rust. */
export class PromptCacheLayout {
  public readonly system: readonly LlmMessage[];
  public readonly canonical: readonly LlmMessage[];
  public readonly history: readonly LlmMessage[];
  public readonly runtimeContext: readonly LlmMessage[];
  public readonly tools: readonly LlmToolDefinition[];

  private constructor(
    system: readonly LlmMessage[],
    canonical: readonly LlmMessage[],
    history: readonly LlmMessage[],
    runtimeContext: readonly LlmMessage[],
    tools: readonly LlmToolDefinition[],
  ) {
    this.system = Object.freeze([...system]);
    this.canonical = Object.freeze([...canonical]);
    this.history = Object.freeze([...history]);
    this.runtimeContext = Object.freeze([...runtimeContext]);
    this.tools = Object.freeze([...tools]);
    Object.freeze(this);
  }

  public static fromMessages(
    messages: readonly LlmMessage[],
    tools: readonly LlmToolDefinition[],
  ): PromptCacheLayout {
    if (!Array.isArray(messages) || !Array.isArray(tools)) {
      throw new TypeError("messages and tools must be arrays");
    }
    const systemEnd = messages.findIndex((message) => message.role !== "system");
    const sysEnd = systemEnd < 0 ? messages.length : systemEnd;
    const canonicalIndex = messages
      .slice(0, sysEnd)
      .findIndex((message) => messageText(message)?.includes("Canonical context") ?? false);
    const canonicalStart = canonicalIndex < 0 ? sysEnd : canonicalIndex;

    let runtimeStart = messages.length;
    // 保留公开 runtimeContext 名称，只投影框架严格识别的连续 dynamic 尾段。
    while (runtimeStart > sysEnd) {
      const candidate = messages[runtimeStart - 1];
      if (!candidate || candidate.role === "system" || !isDynamicTailMessage(candidate)) break;
      runtimeStart -= 1;
    }

    return new PromptCacheLayout(
      messages.slice(0, Math.min(canonicalStart, sysEnd)),
      canonicalStart < sysEnd ? messages.slice(canonicalStart, sysEnd) : [],
      messages.slice(sysEnd, runtimeStart),
      messages.slice(runtimeStart),
      tools,
    );
  }

  public segmentRanges(): SegmentRanges {
    const system = BigInt(this.system.length);
    const canonical = BigInt(this.canonical.length);
    const history = BigInt(this.history.length);
    const runtimeContext = BigInt(this.runtimeContext.length);
    return new SegmentRanges(
      new SegmentRange(0n, system),
      new SegmentRange(system, system + canonical),
      new SegmentRange(system + canonical, system + canonical + history),
      new SegmentRange(system + canonical + history, system + canonical + history + runtimeContext),
    );
  }
}

function messageText(message: LlmMessage): string | null {
  const content = fromWireValue(message.content);
  if (typeof content === "string") return content || null;
  if (!Array.isArray(content)) return null;
  const texts: string[] = [];
  for (const part of content) {
    if (!isObject(part)) continue;
    if (part.type === "text" && typeof part.text === "string") texts.push(part.text);
    else if (part.type === "resource_link" && isObject(part.resource)) {
      const resource = part.resource;
      // 只识别框架 LinkedResource 的八个字段；缺失 Option 为 null，未知字段不进入视图。
      const fields = {
        annotations: resource.annotations ?? null,
        description: resource.description ?? null,
        mime_type: resource.mime_type ?? null,
        name: resource.name,
        size: resource.size ?? null,
        title: resource.title ?? null,
        uri: resource.uri,
        meta: resource.meta ?? null,
      };
      texts.push(`[Linked resource]\n${JSON.stringify(fields)}`);
    }
  }
  // canonical 使用完整 as_text 视图；dynamic 仍只读首个 Text part 的独立信封。
  return texts.length > 0 ? texts.join("") : null;
}

function isObject(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

function isDynamicTailMessage(message: LlmMessage): boolean {
  const content = fromWireValue(message.content);
  // 与框架一致：多模态信封只能来自首个 Text part，不能拼接其它正文补头。
  const first: unknown = Array.isArray(content) ? content[0] : undefined;
  const text = typeof content === "string" ? content
    : typeof first === "object" && first !== null && "type" in first && first.type === "text"
      && "text" in first && typeof first.text === "string" ? first.text : null;
  const prefix = "<echo-agent-context-projection-v2>\n";
  if (text === null || !text.startsWith(prefix)) return false;
  const end = text.indexOf("\n\n", prefix.length);
  if (end < 0) return false;
  const lines = text.slice(prefix.length, end).split(/\r?\n/u);
  const [marker, placement, version] = lines;
  return lines.length === 3 && marker !== undefined && marker.startsWith("marker=")
    && placement === "placement=tail" && version !== undefined && version.startsWith("version=")
    && isProjectionString(marker.slice("marker=".length))
    && isProjectionString(version.slice("version=".length));
}

function isProjectionString(raw: string): boolean {
  try {
    const value: unknown = JSON.parse(raw);
    // serde_json::String 不接受未配对 surrogate；JS 的 JSON.parse 需要补这一检查。
    return typeof value === "string" && value.length > 0 && !/[\uD800-\uDFFF]/u.test(value);
  } catch {
    return false;
  }
}
