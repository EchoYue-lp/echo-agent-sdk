import { SegmentRange } from "./segment_range_values.js";
/** Message-cache segment indexes without owning cache state. */
export class SegmentRanges {
    system;
    canonical;
    history;
    runtimeContext;
    constructor(system = new SegmentRange(), canonical = new SegmentRange(), history = new SegmentRange(), runtimeContext = new SegmentRange()) {
        this.system = system;
        this.canonical = canonical;
        this.history = history;
        this.runtimeContext = runtimeContext;
        Object.freeze(this);
    }
}
/** Read-only prompt layout projection; cache storage and provider placement stay in Rust. */
export class PromptCacheLayout {
    system;
    canonical;
    history;
    runtimeContext;
    tools;
    constructor(system, canonical, history, runtimeContext, tools) {
        this.system = Object.freeze([...system]);
        this.canonical = Object.freeze([...canonical]);
        this.history = Object.freeze([...history]);
        this.runtimeContext = Object.freeze([...runtimeContext]);
        this.tools = Object.freeze([...tools]);
        Object.freeze(this);
    }
    static fromMessages(messages, tools) {
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
        for (let index = messages.length - 1; index >= 0; index -= 1) {
            const candidate = messages[index];
            if (candidate && (messageText(candidate)?.trimStart().startsWith("[runtime_context:") ?? false)) {
                runtimeStart = index;
                while (runtimeStart > sysEnd) {
                    const previous = messages[runtimeStart - 1];
                    if (!previous || !(messageText(previous)?.trimStart().startsWith("[runtime_context:") ?? false))
                        break;
                    runtimeStart -= 1;
                }
                break;
            }
        }
        return new PromptCacheLayout(messages.slice(0, Math.min(canonicalStart, sysEnd)), canonicalStart < sysEnd ? messages.slice(canonicalStart, sysEnd) : [], messages.slice(sysEnd, runtimeStart), messages.slice(runtimeStart), tools);
    }
    segmentRanges() {
        const system = BigInt(this.system.length);
        const canonical = BigInt(this.canonical.length);
        const history = BigInt(this.history.length);
        const runtimeContext = BigInt(this.runtimeContext.length);
        return new SegmentRanges(new SegmentRange(0n, system), new SegmentRange(system, system + canonical), new SegmentRange(system + canonical, system + canonical + history), new SegmentRange(system + canonical + history, system + canonical + history + runtimeContext));
    }
}
function messageText(message) {
    const content = message.content;
    if (typeof content === "string")
        return content;
    if (isWireString(content))
        return content.value;
    return null;
}
function isWireString(value) {
    return typeof value === "object"
        && value !== null
        && !Array.isArray(value)
        && value.kind === "string"
        && typeof value.value === "string";
}
//# sourceMappingURL=prompt_cache_layout_values.js.map