/** Permission mode identifiers and policy helpers projected without evaluation. */
export const PermissionMode = Object.freeze({
  Default: "default",
  Plan: "plan",
  AcceptEdits: "auto-edit",
  BypassPermissions: "full-auto",
  Auto: "auto",
  Bubble: "bubble",
  DontAsk: "dont-ask",
  StrictConfirm: "strict",
} as const);

export type PermissionMode = (typeof PermissionMode)[keyof typeof PermissionMode];

export function permissionModeParse(value: string): PermissionMode {
  if (typeof value !== "string") throw new TypeError("permission mode must be text");
  // 与Framework当前id()一致：输入不归一，避免恢复已经退出的旧模式拼写。
  const mode = Object.values(PermissionMode).find(mode => mode === value);
  if (mode === undefined) throw new Error(`invalid permission mode '${value}'`);
  return mode;
}

export function permissionModeAllowsWrite(mode: PermissionMode): boolean {
  return mode === PermissionMode.BypassPermissions || mode === PermissionMode.AcceptEdits;
}

export function permissionModeRequiresInteraction(mode: PermissionMode): boolean {
  return mode !== PermissionMode.BypassPermissions
    && mode !== PermissionMode.Auto
    && mode !== PermissionMode.DontAsk
    && mode !== PermissionMode.AcceptEdits;
}

export function permissionModeUsesClassifier(mode: PermissionMode): boolean {
  return mode === PermissionMode.Auto;
}
