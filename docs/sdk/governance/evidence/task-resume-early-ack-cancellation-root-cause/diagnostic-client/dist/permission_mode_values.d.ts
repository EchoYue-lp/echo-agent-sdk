/** Permission mode identifiers and policy helpers projected without evaluation. */
export declare const PermissionMode: Readonly<{
    readonly Default: "default";
    readonly Plan: "plan";
    readonly AcceptEdits: "auto-edit";
    readonly BypassPermissions: "full-auto";
    readonly Auto: "auto";
    readonly Bubble: "bubble";
    readonly DontAsk: "dont-ask";
    readonly StrictConfirm: "strict";
}>;
export type PermissionMode = (typeof PermissionMode)[keyof typeof PermissionMode];
export declare function permissionModeParse(value: string): PermissionMode;
export declare function permissionModeAllowsWrite(mode: PermissionMode): boolean;
export declare function permissionModeRequiresInteraction(mode: PermissionMode): boolean;
export declare function permissionModeUsesClassifier(mode: PermissionMode): boolean;
