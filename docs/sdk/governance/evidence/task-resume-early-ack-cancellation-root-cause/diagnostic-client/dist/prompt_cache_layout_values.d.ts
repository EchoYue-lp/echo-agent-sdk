import type { LlmMessage, LlmToolDefinition } from "./types.js";
import { SegmentRange } from "./segment_range_values.js";
/** Message-cache segment indexes without owning cache state. */
export declare class SegmentRanges {
    readonly system: SegmentRange;
    readonly canonical: SegmentRange;
    readonly history: SegmentRange;
    readonly runtimeContext: SegmentRange;
    constructor(system?: SegmentRange, canonical?: SegmentRange, history?: SegmentRange, runtimeContext?: SegmentRange);
}
/** Read-only prompt layout projection; cache storage and provider placement stay in Rust. */
export declare class PromptCacheLayout {
    readonly system: readonly LlmMessage[];
    readonly canonical: readonly LlmMessage[];
    readonly history: readonly LlmMessage[];
    readonly runtimeContext: readonly LlmMessage[];
    readonly tools: readonly LlmToolDefinition[];
    private constructor();
    static fromMessages(messages: readonly LlmMessage[], tools: readonly LlmToolDefinition[]): PromptCacheLayout;
    segmentRanges(): SegmentRanges;
}
