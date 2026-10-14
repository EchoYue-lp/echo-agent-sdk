/** Current Unix timestamp in seconds, returning zero only on invalid Date state. */
export declare function nowSecs(): number;
/** Current Unix timestamp in milliseconds. */
export declare function nowMillis(): number;
/** Current system-local timestamp with an explicit RFC3339 offset. */
export declare function nowLocal(): string;
/** Convert an instant to the system-local RFC3339 representation. */
export declare function toLocal(value: Date | string): string;
/** Serialize a UTC/offset instant using the system-local RFC3339 offset. */
export declare function localRfc3339Serialize(value: Date | string): string;
/** Parse a local or UTC RFC3339 value and return the same instant as Date. */
export declare function localRfc3339Deserialize(value: string): Date;
export declare function optionLocalRfc3339Serialize(value: Date | string | null | undefined): string | null;
export declare function optionLocalRfc3339Deserialize(value: string | null | undefined): Date | null;
