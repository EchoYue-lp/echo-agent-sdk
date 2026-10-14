/** Closed A2A task-state values exposed by the Rust facade. */
export declare const TaskState: Readonly<{
    readonly Submitted: "submitted";
    readonly Working: "working";
    readonly InputRequired: "input-required";
    readonly Completed: "completed";
    readonly Failed: "failed";
    readonly Canceled: "canceled";
}>;
export type TaskState = (typeof TaskState)[keyof typeof TaskState];
export declare function taskStateIsTerminal(state: TaskState): boolean;
export declare function taskStateCanTransitionTo(current: TaskState, next: TaskState): boolean;
export type A2APart = {
    readonly type: "text";
    readonly text: string;
} | {
    readonly type: "file";
    readonly mimeType: string;
    readonly data: string;
};
export declare class A2AMessage {
    readonly role: string;
    readonly parts: readonly A2APart[];
    private constructor();
    static userText(text: string): A2AMessage;
    static agentText(text: string): A2AMessage;
    textContent(): string;
}
export declare class A2AArtifact {
    readonly name?: string;
    readonly index?: bigint;
    readonly parts: readonly A2APart[];
    readonly append: boolean;
    private constructor();
    static new(parts: readonly A2APart[], options?: {
        readonly name?: string;
        readonly index?: number | bigint;
        readonly append?: boolean;
    }): A2AArtifact;
}
export declare class A2AError {
    readonly code: number;
    readonly message: string;
    private constructor();
    static new(code: number, message: string): A2AError;
}
export declare class TaskStatusUpdateEvent {
    readonly taskId: string;
    readonly status: A2ATaskStatus;
    readonly isFinal: boolean;
    private constructor();
    static new(taskId: string, status: A2ATaskStatus, isFinal?: boolean): TaskStatusUpdateEvent;
}
export declare class TaskArtifactUpdateEvent {
    readonly taskId: string;
    readonly artifact: A2AArtifact;
    readonly isFinal: boolean;
    private constructor();
    static new(taskId: string, artifact: A2AArtifact, isFinal?: boolean): TaskArtifactUpdateEvent;
}
export type A2AStreamEvent = {
    readonly type: "status";
    readonly event: TaskStatusUpdateEvent;
} | {
    readonly type: "artifact";
    readonly event: TaskArtifactUpdateEvent;
};
export declare class A2AStreamResponse {
    readonly jsonrpc: string;
    readonly id: string;
    readonly result?: A2AStreamEvent;
    readonly error?: A2AError;
    private constructor();
    static new(id: string, result?: A2AStreamEvent, error?: A2AError): A2AStreamResponse;
}
export declare class A2ATaskParams {
    readonly id?: string;
    readonly sessionId?: string;
    readonly message: A2AMessage;
    private constructor();
    static new(message: A2AMessage, id?: string, sessionId?: string): A2ATaskParams;
}
export declare class A2ATaskRequest {
    readonly jsonrpc = "2.0";
    readonly id: string;
    readonly method: string;
    readonly params: A2ATaskParams;
    private constructor();
    static new(id: string, method: string, params: A2ATaskParams): A2ATaskRequest;
}
export declare class A2ATask {
    readonly id: string;
    readonly sessionId?: string;
    readonly status: A2ATaskStatus;
    readonly history: readonly A2AMessage[];
    readonly artifacts: readonly A2AArtifact[];
    private constructor();
    static new(id: string, status: A2ATaskStatus, sessionId?: string, history?: readonly A2AMessage[], artifacts?: readonly A2AArtifact[]): A2ATask;
}
export declare class A2ATaskResponse {
    readonly jsonrpc = "2.0";
    readonly id?: string;
    readonly result?: A2ATask;
    readonly error?: A2AError;
    private constructor();
    static new(id?: string, result?: A2ATask, error?: A2AError): A2ATaskResponse;
}
export declare class A2ATaskStatus {
    readonly state: TaskState;
    readonly message?: A2AMessage;
    readonly timestamp: string;
    private constructor();
    static new(state: TaskState): A2ATaskStatus;
    static withMessage(state: TaskState, message: A2AMessage): A2ATaskStatus;
}
export declare class AgentProvider {
    readonly organization: string;
    readonly url?: string;
    private constructor();
    static new(organization: string): AgentProvider;
    withUrl(url: string): AgentProvider;
}
export declare class AgentSkill {
    readonly id: string;
    readonly name: string;
    readonly description?: string;
    readonly examples: readonly string[];
    readonly inputModes: readonly string[];
    readonly outputModes: readonly string[];
    readonly tags: readonly string[];
    private constructor();
    static new(name: string, description: string): AgentSkill;
    withExamples(examples: readonly string[]): AgentSkill;
    withTags(tags: readonly string[]): AgentSkill;
    private copy;
}
export type AuthenticationScheme = Readonly<{
    scheme: string;
    config: Readonly<Record<string, unknown>>;
}>;
export type AgentAuthentication = Readonly<{
    schemes: readonly AuthenticationScheme[];
}>;
export type AgentCapabilities = Readonly<{
    streaming: boolean;
    pushNotifications: boolean;
    stateTransitionHistory: boolean;
}>;
type AgentCardInput = {
    name: string;
    url: string;
    description?: string;
    version?: string;
    provider?: AgentProvider;
    skills: readonly AgentSkill[];
    defaultInputModes: readonly string[];
    defaultOutputModes: readonly string[];
    authentication?: AgentAuthentication;
    capabilities: AgentCapabilities;
};
export declare class AgentCard {
    readonly name: string;
    readonly description?: string;
    readonly url: string;
    readonly version?: string;
    readonly provider?: AgentProvider;
    readonly skills: readonly AgentSkill[];
    readonly defaultInputModes: readonly string[];
    readonly defaultOutputModes: readonly string[];
    readonly authentication?: AgentAuthentication;
    readonly capabilities: AgentCapabilities;
    private constructor();
    static builder(name: string, url: string): AgentCardBuilder;
    /** @internal */
    static fromInput(input: AgentCardInput): AgentCard;
}
export declare class AgentCardBuilder {
    private descriptionValue?;
    private versionValue?;
    private providerValue?;
    private readonly skillsValue;
    private inputModesValue;
    private outputModesValue;
    private authenticationValue?;
    private streamingValue;
    private pushNotificationsValue;
    private readonly nameValue;
    private readonly urlValue;
    constructor(nameValue: string, urlValue: string);
    description(value: string): AgentCardBuilder;
    version(value: string): AgentCardBuilder;
    provider(value: AgentProvider): AgentCardBuilder;
    skill(value: AgentSkill): AgentCardBuilder;
    skills(values: readonly AgentSkill[]): AgentCardBuilder;
    inputModes(values: readonly string[]): AgentCardBuilder;
    outputModes(values: readonly string[]): AgentCardBuilder;
    authentication(value: AgentAuthentication): AgentCardBuilder;
    streaming(): AgentCardBuilder;
    pushNotifications(): AgentCardBuilder;
    build(): AgentCard;
}
export {};
