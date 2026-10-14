/** ACP runtime values projected without owning connection or ledger state. */
export declare const ConnectionMode: Readonly<{
    readonly Standard: "standard";
    readonly Extended: "extended";
}>;
export type ConnectionMode = (typeof ConnectionMode)[keyof typeof ConnectionMode];
export declare const ExtensionSettlement: Readonly<{
    readonly Answered: "answered";
    readonly TimedOut: "timed_out";
    readonly Cancelled: "cancelled";
    readonly Disconnected: "disconnected";
}>;
export type ExtensionSettlement = (typeof ExtensionSettlement)[keyof typeof ExtensionSettlement];
export declare function extensionSettlementIsAnswered(settlement: ExtensionSettlement): boolean;
export interface AcpLedgerLimits {
    readonly maxEvents: bigint;
    readonly maxBytes: bigint;
}
export declare function acpLedgerLimits(maxEvents: bigint, maxBytes: bigint): AcpLedgerLimits;
export declare function acpLedgerLimitsDefault(): AcpLedgerLimits;
export interface AcpAdapterConfig {
    readonly name: string;
    readonly title: string;
    readonly version: string;
    readonly maxSessions: bigint;
    readonly maxPromptChars: bigint;
    readonly maxUpdateChars: bigint;
    readonly maxUpdatesPerTurn: bigint;
    readonly maxTotalUpdateChars: bigint;
    readonly maxExtensionConcurrency: bigint;
    readonly shutdownTimeout: WireDuration;
}
export declare function acpAdapterConfigDefault(version?: string): AcpAdapterConfig;
export declare function acpAdapterConfigValidate(config: AcpAdapterConfig): void;
export declare const ExtensionLeaseError: Readonly<{
    readonly AdmissionClosed: "extension admission is closed";
    readonly ConcurrencyLimit: "extension concurrency limit reached";
    readonly ExclusiveConflict: "extension is already executing an exclusive invocation";
}>;
export type ExtensionLeaseError = (typeof ExtensionLeaseError)[keyof typeof ExtensionLeaseError];
export declare function extensionLeaseErrorAsStr(error: ExtensionLeaseError): string;
import type { WireDuration } from "./types.js";
