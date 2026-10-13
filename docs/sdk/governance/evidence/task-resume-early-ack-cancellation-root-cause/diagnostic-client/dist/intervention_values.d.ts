import type { ParamValue } from "./helpers.js";
/** Immutable intervention decision values; execution remains Host-owned. */
export type InterventionResult = Readonly<{
    block: boolean;
    blockReason?: string;
    injectedContext?: string;
    redirectTo?: string;
    cancel: boolean;
    modifiedArgs?: ParamValue;
}>;
export declare const InterventionResult: Readonly<{
    allow(): InterventionResult;
    block(reason: string): InterventionResult;
    inject(context: string): InterventionResult;
    cancel(): InterventionResult;
    modifyArgs(args: ParamValue): InterventionResult;
}>;
