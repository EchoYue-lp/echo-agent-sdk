import { randomUUID } from "node:crypto";
function nonEmpty(value, label) {
    if (typeof value !== "string" || value.trim().length === 0) {
        throw new TypeError(`${label} must not be empty`);
    }
    return value;
}
/** Validated event stream identity values. */
export class StreamId {
    value;
    constructor(value) {
        this.value = nonEmpty(value, "event stream_id");
        Object.freeze(this);
    }
    static new(value) {
        return new StreamId(value);
    }
    asStr() {
        return this.value;
    }
    toString() {
        return this.value;
    }
}
/** Validated event identity values. */
export class EventId {
    value;
    constructor(value) {
        this.value = nonEmpty(value, "event_id");
        Object.freeze(this);
    }
    static new(value) {
        return new EventId(value);
    }
    asStr() {
        return this.value;
    }
    toString() {
        return this.value;
    }
}
/** Immutable transport identity projection without owning a live stream. */
export class EventIdentity {
    streamId;
    conversationId;
    runId;
    turnId;
    messageId;
    executionId;
    parentEventId;
    constructor(fields) {
        this.streamId = fields.streamId;
        this.conversationId = fields.conversationId;
        this.runId = fields.runId;
        this.turnId = nonEmpty(fields.turnId, "turn_id");
        this.messageId = fields.messageId;
        this.executionId = fields.executionId;
        this.parentEventId = fields.parentEventId;
        this.validate();
        Object.freeze(this);
    }
    static new(streamId, turnId) {
        return new EventIdentity({ streamId: StreamId.new(streamId), turnId });
    }
    static forRun(runId) {
        const id = nonEmpty(runId, "run_id");
        return new EventIdentity({
            streamId: StreamId.new(randomUUID()),
            runId: id,
            turnId: id,
            executionId: id,
        });
    }
    static forChat(conversationId, turnId, messageId, runId) {
        return new EventIdentity({
            streamId: StreamId.new(randomUUID()),
            conversationId: conversationId === undefined ? undefined : nonEmpty(conversationId, "conversation_id"),
            runId: runId === undefined ? undefined : nonEmpty(runId, "run_id"),
            turnId,
            messageId: nonEmpty(messageId, "message_id"),
        });
    }
    static fromInvocation(invocation) {
        return EventIdentity.fromRuntimeContext(invocation?.runtime);
    }
    static fromRuntimeContext(runtime) {
        const runId = runtime?.run_id ?? undefined;
        const executionId = runtime?.execution_id ?? undefined;
        const turnId = runtime?.turn_id ?? executionId ?? runId ?? randomUUID();
        const parentEventId = runtime?.parent_event_id ?? runtime?.subagent_lineage?.parent_event_id ?? undefined;
        return new EventIdentity({
            streamId: StreamId.new(randomUUID()),
            conversationId: runtime?.conversation_id ?? undefined,
            runId,
            turnId,
            messageId: runtime?.message_id ?? undefined,
            executionId,
            parentEventId,
        });
    }
    validate() {
        StreamId.new(this.streamId.asStr());
        nonEmpty(this.turnId, "turn_id");
        for (const [value, label] of [
            [this.conversationId, "conversation_id"],
            [this.runId, "run_id"],
            [this.messageId, "message_id"],
            [this.executionId, "execution_id"],
            [this.parentEventId, "event_id"],
        ]) {
            if (value !== undefined)
                nonEmpty(value, label);
        }
    }
    withConversationId(value) {
        return this.copy({ conversationId: nonEmpty(value, "conversation_id") });
    }
    withRunId(value) {
        return this.copy({ runId: nonEmpty(value, "run_id") });
    }
    withMessageId(value) {
        return this.copy({ messageId: nonEmpty(value, "message_id") });
    }
    withExecutionId(value) {
        return this.copy({ executionId: nonEmpty(value, "execution_id") });
    }
    withParentEventId(value) {
        return this.copy({ parentEventId: nonEmpty(value, "event_id") });
    }
    copy(overrides) {
        return new EventIdentity({
            streamId: overrides.streamId ?? this.streamId,
            conversationId: overrides.conversationId ?? this.conversationId,
            runId: overrides.runId ?? this.runId,
            turnId: overrides.turnId ?? this.turnId,
            messageId: overrides.messageId ?? this.messageId,
            executionId: overrides.executionId ?? this.executionId,
            parentEventId: overrides.parentEventId ?? this.parentEventId,
        });
    }
}
//# sourceMappingURL=event_identity_values.js.map