import type { JsonValue, ToolFailure, ToolFailureCategory, ToolResultData, ToolEffect, ToolResultContent, ToolResultKind, WireValue } from "./types.js";
/** JSON value exposed by the Rust `ParamValue` projection. */
export type ParamValue = JsonValue;
/**
 * Type-safe view over a tool call's JSON object.
 *
 * Rust stores the raw JSON and a recursively typed `ParamValue` map. The
 * TypeScript view keeps the same distinction while using the language's
 * native JSON values and immutable snapshots.
 */
export declare class ToolCallParams {
    readonly raw: JsonValue;
    private readonly parsed;
    private constructor();
    static fromValue(value: JsonValue): ToolCallParams;
    static fromParams(params: Readonly<Record<string, JsonValue>>): ToolCallParams;
    getStr(key: string): string | undefined;
    getNumber(key: string): number | undefined;
    getBool(key: string): boolean | undefined;
    get(key: string): ParamValue | undefined;
    validateRequired(key: string, expectedType: string): void;
    has(key: string): boolean;
    len(): number;
    isEmpty(): boolean;
}
/** Immutable language-native equivalent of Rust's `ToolResult` constructors. */
export declare class ToolResultValue implements ToolResultData {
    readonly kind: ToolResultKind;
    readonly output: string;
    readonly success: boolean;
    readonly truncated: boolean;
    readonly artifact?: ToolResultData["artifact"];
    readonly data?: WireValue | null;
    readonly error?: string | null;
    readonly failure?: ToolFailure | null;
    readonly metadata?: Readonly<Record<string, string>>;
    readonly mime_type?: string | null;
    readonly model_content?: readonly ToolResultContent[];
    readonly effects?: readonly ToolEffect[];
    private constructor();
    static success(output: string): ToolResultValue;
    static successJson(data: JsonValue): ToolResultValue;
    static successWithKind(kind: ToolResultKind, output: string): ToolResultValue;
    static error(error: string): ToolResultValue;
    static invalidArguments(error: string): ToolResultValue;
    static failure(category: ToolFailureCategory, error: string): ToolResultValue;
    withOutput(output: string): ToolResultValue;
    withError(error: string): ToolResultValue;
    withFailure(failure: ToolFailure): ToolResultValue;
    withData(data: JsonValue): ToolResultValue;
    withTruncated(truncated: boolean): ToolResultValue;
    withMimeType(mimeType: string): ToolResultValue;
    withArtifact(artifact: NonNullable<ToolResultData["artifact"]>): ToolResultValue;
    withMeta(key: string, value: string): ToolResultValue;
    withMetadata(metadata: Readonly<Record<string, string>>): ToolResultValue;
    withModelContent(content: ToolResultContent): ToolResultValue;
    withEffects(effects: readonly ToolEffect[]): ToolResultValue;
    toJSON(): ToolResultData;
    private static create;
    private copy;
}
export declare const ToolResult: typeof ToolResultValue;
/**
 * Split UTF-8 text into chunks capped by encoded byte length.
 *
 * A chunk is never split in the middle of a Unicode scalar value. The limit
 * is clamped to one byte, matching the Rust helper's `max_chunk_bytes.max(1)`.
 */
export declare function splitUtf8Chunks(text: string, maxChunkBytes: number): string[];
/**
 * Stateful UTF-8 decoder for byte streams whose read boundaries may split a
 * multi-byte scalar value.
 *
 * `TextDecoder` with streaming enabled preserves incomplete suffixes between
 * pushes and replaces malformed sequences with U+FFFD. `finish` flushes an
 * incomplete suffix and resets the decoder for a subsequent stream.
 */
export declare class IncrementalUtf8Decoder {
    private readonly maxChunkBytes;
    private decoder;
    constructor(maxChunkBytes?: number);
    push(bytes: Uint8Array): string[];
    finish(): string | undefined;
}
/** Extract JSON from a fenced markdown block or bare text. */
export declare function extractJsonFromMarkdown(content: string): string;
/** Remove trailing commas before `}` or `]`, outside quoted strings. */
export declare function cleanJson(value: string): string;
export declare function validateToolEffect(value: ToolEffect): void;
