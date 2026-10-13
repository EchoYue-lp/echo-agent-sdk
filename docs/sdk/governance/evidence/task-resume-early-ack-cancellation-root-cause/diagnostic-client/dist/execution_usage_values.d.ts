import type { ExecutionUsage } from "./types.js";
/** Return duration in milliseconds, matching Rust's absent-to-zero helper. */
export declare function executionUsageDurationMillis(usage: ExecutionUsage): bigint;
