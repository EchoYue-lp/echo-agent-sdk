/** Validated event stream identity values. */
export declare class StreamId {
    readonly value: string;
    private constructor();
    static new(value: string): StreamId;
    asStr(): string;
    toString(): string;
}
/** Validated event identity values. */
export declare class EventId {
    readonly value: string;
    private constructor();
    static new(value: string): EventId;
    asStr(): string;
    toString(): string;
}
export type EventRuntimeContext = {
    conversation_id?: string | null;
    run_id?: string | null;
    turn_id?: string | null;
    message_id?: string | null;
    execution_id?: string | null;
    parent_event_id?: string | null;
    subagent_lineage?: {
        parent_event_id?: string | null;
    } | null;
};
/** Immutable transport identity projection without owning a live stream. */
export declare class EventIdentity {
    readonly streamId: StreamId;
    readonly conversationId: string | undefined;
    readonly runId: string | undefined;
    readonly turnId: string;
    readonly messageId: string | undefined;
    readonly executionId: string | undefined;
    readonly parentEventId: string | undefined;
    private constructor();
    static new(streamId: string, turnId: string): EventIdentity;
    static forRun(runId: string): EventIdentity;
    static forChat(conversationId: string | undefined, turnId: string, messageId: string, runId?: string): EventIdentity;
    static fromInvocation(invocation: {
        runtime?: EventRuntimeContext | null;
    } | null | undefined): EventIdentity;
    static fromRuntimeContext(runtime?: EventRuntimeContext | null): EventIdentity;
    validate(): void;
    withConversationId(value: string): EventIdentity;
    withRunId(value: string): EventIdentity;
    withMessageId(value: string): EventIdentity;
    withExecutionId(value: string): EventIdentity;
    withParentEventId(value: string): EventIdentity;
    private copy;
}
