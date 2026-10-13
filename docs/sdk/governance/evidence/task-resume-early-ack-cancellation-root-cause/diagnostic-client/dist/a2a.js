/** Closed A2A task-state values exposed by the Rust facade. */
export const TaskState = Object.freeze({
    Submitted: "submitted",
    Working: "working",
    InputRequired: "input-required",
    Completed: "completed",
    Failed: "failed",
    Canceled: "canceled",
});
const TASK_STATES = new Set(Object.values(TaskState));
const MAX_USIZE_64 = (1n << 64n) - 1n;
export function taskStateIsTerminal(state) {
    validateTaskState(state);
    return state === TaskState.Completed || state === TaskState.Failed || state === TaskState.Canceled;
}
export function taskStateCanTransitionTo(current, next) {
    validateTaskState(current);
    validateTaskState(next);
    if (taskStateIsTerminal(current))
        return false;
    return (current === TaskState.Submitted
        && (next === TaskState.Working || next === TaskState.Canceled))
        || (current === TaskState.Working
            && (next === TaskState.Completed
                || next === TaskState.Failed
                || next === TaskState.InputRequired
                || next === TaskState.Canceled))
        || (current === TaskState.InputRequired
            && (next === TaskState.Working || next === TaskState.Canceled));
}
export class A2AMessage {
    role;
    parts;
    constructor(role, parts) {
        this.role = role;
        this.parts = Object.freeze(parts.map((part) => Object.freeze({ ...part })));
        Object.freeze(this);
    }
    static userText(text) {
        if (typeof text !== "string")
            throw new TypeError("message text must be text");
        return new A2AMessage("user", [{ type: "text", text }]);
    }
    static agentText(text) {
        if (typeof text !== "string")
            throw new TypeError("message text must be text");
        return new A2AMessage("agent", [{ type: "text", text }]);
    }
    textContent() {
        return this.parts
            .filter((part) => part.type === "text")
            .map((part) => part.text)
            .join("\n");
    }
}
export class A2AArtifact {
    name;
    index;
    parts;
    append;
    constructor(parts, name, index, append = false) {
        if (!Array.isArray(parts))
            throw new TypeError("artifact parts must be an array");
        if (name !== undefined)
            validateText(name, "artifact name");
        if (index !== undefined && (index < 0n || index > MAX_USIZE_64)) {
            throw new TypeError("artifact index must fit Rust usize");
        }
        if (typeof append !== "boolean")
            throw new TypeError("artifact append must be boolean");
        this.name = name;
        this.index = index;
        this.parts = Object.freeze(parts.map((part) => freezePart(part)));
        this.append = append;
        Object.freeze(this);
    }
    static new(parts, options = {}) {
        return new A2AArtifact(parts, options.name, normalizeArtifactIndex(options.index), options.append ?? false);
    }
}
export class A2AError {
    code;
    message;
    constructor(code, message) {
        if (!Number.isInteger(code) || code < -2147483648 || code > 2147483647) {
            throw new TypeError("A2A error code must be an i32");
        }
        validateText(message, "A2A error message");
        this.code = code;
        this.message = message;
        Object.freeze(this);
    }
    static new(code, message) {
        return new A2AError(code, message);
    }
}
export class TaskStatusUpdateEvent {
    taskId;
    status;
    isFinal;
    constructor(taskId, status, isFinal) {
        validateText(taskId, "task id");
        if (!(status instanceof A2ATaskStatus))
            throw new TypeError("status must be an A2ATaskStatus");
        if (typeof isFinal !== "boolean")
            throw new TypeError("final must be boolean");
        this.taskId = taskId;
        this.status = status;
        this.isFinal = isFinal;
        Object.freeze(this);
    }
    static new(taskId, status, isFinal = false) {
        return new TaskStatusUpdateEvent(taskId, status, isFinal);
    }
}
export class TaskArtifactUpdateEvent {
    taskId;
    artifact;
    isFinal;
    constructor(taskId, artifact, isFinal) {
        validateText(taskId, "task id");
        if (!(artifact instanceof A2AArtifact))
            throw new TypeError("artifact must be an A2AArtifact");
        if (typeof isFinal !== "boolean")
            throw new TypeError("final must be boolean");
        this.taskId = taskId;
        this.artifact = artifact;
        this.isFinal = isFinal;
        Object.freeze(this);
    }
    static new(taskId, artifact, isFinal = false) {
        return new TaskArtifactUpdateEvent(taskId, artifact, isFinal);
    }
}
export class A2AStreamResponse {
    jsonrpc;
    id;
    result;
    error;
    constructor(id, result, error) {
        validateText(id, "stream response id");
        let frozenResult;
        if (result) {
            if (result.type === "status" && !(result.event instanceof TaskStatusUpdateEvent)) {
                throw new TypeError("invalid A2A status event");
            }
            if (result.type === "artifact" && !(result.event instanceof TaskArtifactUpdateEvent)) {
                throw new TypeError("invalid A2A artifact event");
            }
            if (result.type !== "status" && result.type !== "artifact") {
                throw new TypeError("invalid A2A stream event");
            }
            frozenResult = result.type === "status"
                ? Object.freeze({ type: "status", event: result.event })
                : Object.freeze({ type: "artifact", event: result.event });
        }
        if (error !== undefined && !(error instanceof A2AError))
            throw new TypeError("error must be an A2AError");
        this.jsonrpc = "2.0";
        this.id = id;
        this.result = frozenResult;
        this.error = error;
        Object.freeze(this);
    }
    static new(id, result, error) {
        return new A2AStreamResponse(id, result, error);
    }
}
export class A2ATaskParams {
    id;
    sessionId;
    message;
    constructor(message, id, sessionId) {
        if (!(message instanceof A2AMessage))
            throw new TypeError("message must be an A2AMessage");
        if (id !== undefined)
            validateText(id, "task id");
        if (sessionId !== undefined)
            validateText(sessionId, "session id");
        this.id = id;
        this.sessionId = sessionId;
        this.message = message;
        Object.freeze(this);
    }
    static new(message, id, sessionId) {
        return new A2ATaskParams(message, id, sessionId);
    }
}
export class A2ATaskRequest {
    jsonrpc = "2.0";
    id;
    method;
    params;
    constructor(id, method, params) {
        validateText(id, "request id");
        validateText(method, "request method");
        if (!(params instanceof A2ATaskParams))
            throw new TypeError("params must be A2ATaskParams");
        this.id = id;
        this.method = method;
        this.params = params;
        Object.freeze(this);
    }
    static new(id, method, params) {
        return new A2ATaskRequest(id, method, params);
    }
}
export class A2ATask {
    id;
    sessionId;
    status;
    history;
    artifacts;
    constructor(id, status, sessionId, history = [], artifacts = []) {
        validateText(id, "task id");
        if (sessionId !== undefined)
            validateText(sessionId, "session id");
        if (!(status instanceof A2ATaskStatus))
            throw new TypeError("status must be an A2ATaskStatus");
        if (!Array.isArray(history) || history.some((value) => !(value instanceof A2AMessage))) {
            throw new TypeError("history must contain A2AMessage values");
        }
        if (!Array.isArray(artifacts) || artifacts.some((value) => !(value instanceof A2AArtifact))) {
            throw new TypeError("artifacts must contain A2AArtifact values");
        }
        this.id = id;
        this.sessionId = sessionId;
        this.status = status;
        this.history = Object.freeze([...history]);
        this.artifacts = Object.freeze([...artifacts]);
        Object.freeze(this);
    }
    static new(id, status, sessionId, history = [], artifacts = []) {
        return new A2ATask(id, status, sessionId, history, artifacts);
    }
}
export class A2ATaskResponse {
    jsonrpc = "2.0";
    id;
    result;
    error;
    constructor(id, result, error) {
        if (id !== undefined)
            validateText(id, "response id");
        if (result !== undefined && !(result instanceof A2ATask))
            throw new TypeError("result must be an A2ATask");
        if (error !== undefined && !(error instanceof A2AError))
            throw new TypeError("error must be an A2AError");
        this.id = id;
        this.result = result;
        this.error = error;
        Object.freeze(this);
    }
    static new(id, result, error) {
        return new A2ATaskResponse(id, result, error);
    }
}
export class A2ATaskStatus {
    state;
    message;
    timestamp;
    constructor(state, message) {
        validateTaskState(state);
        this.state = state;
        this.message = message;
        this.timestamp = new Date().toISOString();
        Object.freeze(this);
    }
    static new(state) {
        return new A2ATaskStatus(state);
    }
    static withMessage(state, message) {
        if (!(message instanceof A2AMessage))
            throw new TypeError("A2A task status message must be an A2AMessage");
        return new A2ATaskStatus(state, message);
    }
}
export class AgentProvider {
    organization;
    url;
    constructor(organization, url) {
        this.organization = organization;
        this.url = url;
        Object.freeze(this);
    }
    static new(organization) {
        if (typeof organization !== "string")
            throw new TypeError("organization must be text");
        return new AgentProvider(organization);
    }
    withUrl(url) {
        if (typeof url !== "string")
            throw new TypeError("provider url must be text");
        return new AgentProvider(this.organization, url);
    }
}
export class AgentSkill {
    id;
    name;
    description;
    examples;
    inputModes;
    outputModes;
    tags;
    constructor(name, description, examples = [], tags = []) {
        this.id = name;
        this.name = name;
        this.description = description;
        this.examples = Object.freeze([...examples]);
        this.inputModes = Object.freeze([]);
        this.outputModes = Object.freeze([]);
        this.tags = Object.freeze([...tags]);
        Object.freeze(this);
    }
    static new(name, description) {
        if (typeof name !== "string" || typeof description !== "string") {
            throw new TypeError("skill name and description must be text");
        }
        return new AgentSkill(name, description);
    }
    withExamples(examples) {
        return this.copy({ examples: textList(examples, "examples") });
    }
    withTags(tags) {
        return this.copy({ tags: textList(tags, "tags") });
    }
    copy(changes) {
        return new AgentSkill(this.name, this.description ?? "", changes.examples ?? this.examples, changes.tags ?? this.tags);
    }
}
export class AgentCard {
    name;
    description;
    url;
    version;
    provider;
    skills;
    defaultInputModes;
    defaultOutputModes;
    authentication;
    capabilities;
    constructor(input) {
        this.name = input.name;
        this.description = input.description;
        this.url = input.url;
        this.version = input.version;
        this.provider = input.provider;
        this.skills = Object.freeze([...input.skills]);
        this.defaultInputModes = Object.freeze([...input.defaultInputModes]);
        this.defaultOutputModes = Object.freeze([...input.defaultOutputModes]);
        this.authentication = input.authentication
            ? Object.freeze({
                schemes: Object.freeze(input.authentication.schemes.map((scheme) => Object.freeze({ scheme: scheme.scheme, config: Object.freeze({ ...scheme.config }) }))),
            })
            : undefined;
        this.capabilities = Object.freeze({ ...input.capabilities });
        Object.freeze(this);
    }
    static builder(name, url) {
        validateText(name, "agent name");
        validateText(url, "agent url");
        return new AgentCardBuilder(name, url);
    }
    /** @internal */
    static fromInput(input) {
        return new AgentCard(input);
    }
}
export class AgentCardBuilder {
    descriptionValue;
    versionValue;
    providerValue;
    skillsValue = [];
    inputModesValue = ["text/plain"];
    outputModesValue = ["text/plain"];
    authenticationValue;
    streamingValue = false;
    pushNotificationsValue = false;
    nameValue;
    urlValue;
    constructor(nameValue, urlValue) {
        validateText(nameValue, "agent name");
        validateText(urlValue, "agent url");
        this.nameValue = nameValue;
        this.urlValue = urlValue;
    }
    description(value) {
        validateText(value, "agent description");
        this.descriptionValue = value;
        return this;
    }
    version(value) {
        validateText(value, "agent version");
        this.versionValue = value;
        return this;
    }
    provider(value) {
        if (!(value instanceof AgentProvider))
            throw new TypeError("provider must be an AgentProvider");
        this.providerValue = value;
        return this;
    }
    skill(value) {
        if (!(value instanceof AgentSkill))
            throw new TypeError("skill must be an AgentSkill");
        this.skillsValue.push(value);
        return this;
    }
    skills(values) {
        if (!Array.isArray(values) || values.some((value) => !(value instanceof AgentSkill))) {
            throw new TypeError("skills must be an AgentSkill array");
        }
        this.skillsValue.push(...values);
        return this;
    }
    inputModes(values) {
        this.inputModesValue = [...textList(values, "input modes")];
        return this;
    }
    outputModes(values) {
        this.outputModesValue = [...textList(values, "output modes")];
        return this;
    }
    authentication(value) {
        if (!value || !Array.isArray(value.schemes))
            throw new TypeError("invalid agent authentication");
        this.authenticationValue = value;
        return this;
    }
    streaming() {
        this.streamingValue = true;
        return this;
    }
    pushNotifications() {
        this.pushNotificationsValue = true;
        return this;
    }
    build() {
        return AgentCard.fromInput({
            name: this.nameValue,
            url: this.urlValue,
            description: this.descriptionValue,
            version: this.versionValue,
            provider: this.providerValue,
            skills: this.skillsValue,
            defaultInputModes: this.inputModesValue,
            defaultOutputModes: this.outputModesValue,
            authentication: this.authenticationValue,
            capabilities: {
                streaming: this.streamingValue,
                pushNotifications: this.pushNotificationsValue,
                stateTransitionHistory: false,
            },
        });
    }
}
function validateTaskState(state) {
    if (!TASK_STATES.has(state))
        throw new TypeError(`unknown A2A task state: ${state}`);
}
function textList(values, field) {
    if (!Array.isArray(values) || values.some((value) => typeof value !== "string")) {
        throw new TypeError(`${field} must be a string array`);
    }
    return [...values];
}
function validateText(value, field) {
    if (typeof value !== "string")
        throw new TypeError(`${field} must be text`);
}
function freezePart(part) {
    if (!part || (part.type !== "text" && part.type !== "file")) {
        throw new TypeError("artifact parts must be valid A2A parts");
    }
    if (part.type === "text")
        validateText(part.text, "artifact text");
    if (part.type === "file") {
        validateText(part.mimeType, "artifact mime type");
        validateText(part.data, "artifact data");
    }
    return Object.freeze({ ...part });
}
function normalizeArtifactIndex(value) {
    if (value === undefined)
        return undefined;
    if (typeof value === "bigint") {
        if (value < 0n || value > MAX_USIZE_64)
            throw new TypeError("artifact index must fit Rust usize");
        return value;
    }
    if (!Number.isSafeInteger(value) || value < 0)
        throw new TypeError("artifact index must be a non-negative safe integer");
    return BigInt(value);
}
//# sourceMappingURL=a2a.js.map