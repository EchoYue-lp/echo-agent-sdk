/** Memory lifetime scopes with Rust-compatible wire names and priority. */
export declare const MemoryScope: Readonly<{
    readonly User: "user";
    readonly Project: "project";
    readonly Repo: "repo";
    readonly Task: "task";
    readonly Session: "session";
    readonly Run: "run";
}>;
export type MemoryScope = (typeof MemoryScope)[keyof typeof MemoryScope];
export declare function memoryScopeAll(): readonly MemoryScope[];
export declare function memoryScopeName(scope: MemoryScope): string;
export declare function memoryScopePriority(scope: MemoryScope): number;
export declare function memoryScopeIsPersistent(scope: MemoryScope): boolean;
export declare function parseMemoryScope(value: string): MemoryScope | null;
