/** Memory lifetime scopes with Rust-compatible wire names and priority. */
export const MemoryScope = Object.freeze({
    User: "user",
    Project: "project",
    Repo: "repo",
    Task: "task",
    Session: "session",
    Run: "run",
});
const MEMORY_SCOPE_ALL = Object.freeze([
    MemoryScope.User,
    MemoryScope.Project,
    MemoryScope.Repo,
    MemoryScope.Task,
    MemoryScope.Session,
    MemoryScope.Run,
]);
const MEMORY_SCOPES = new Set(MEMORY_SCOPE_ALL);
export function memoryScopeAll() {
    return MEMORY_SCOPE_ALL;
}
export function memoryScopeName(scope) {
    validateMemoryScope(scope);
    return scope;
}
export function memoryScopePriority(scope) {
    validateMemoryScope(scope);
    return MEMORY_SCOPE_ALL.indexOf(scope);
}
export function memoryScopeIsPersistent(scope) {
    validateMemoryScope(scope);
    return scope === MemoryScope.User || scope === MemoryScope.Project || scope === MemoryScope.Repo;
}
export function parseMemoryScope(value) {
    if (typeof value !== "string")
        return null;
    switch (value.trim().toLowerCase()) {
        case "user": return MemoryScope.User;
        case "project":
        case "proj": return MemoryScope.Project;
        case "repo": return MemoryScope.Repo;
        case "task": return MemoryScope.Task;
        case "session":
        case "sess": return MemoryScope.Session;
        case "run": return MemoryScope.Run;
        default: return null;
    }
}
function validateMemoryScope(scope) {
    if (typeof scope !== "string" || !MEMORY_SCOPES.has(scope)) {
        throw new TypeError(`unknown memory scope: ${String(scope)}`);
    }
}
//# sourceMappingURL=memory_scope_values.js.map