import { FacadeCatalog } from "./catalog.js";
import type { EchoAgentCapability, ExtensionDescriptor, ExtensionInvokeCall, ExtensionResult, ExtensionOutcome, FacadeStreamItem, LlmClientDescriptor, LlmClientExtensionHandler, LlmInvocation, CriticDescriptor, CriticExtensionHandler, CriticInvocation, ContextCompressorExtensionHandler, ContextCompressorInvocation, ContextCompressorDescriptor, AgentComponentDescriptor, AgentCallbackInvocation, AgentCallbackResult, AgentComponentExtensionHandler, AgentComponentInvocation, AgentComponentStreamChunk, AgentComponentStreamComplete, ExecutionUsage, RunGetResult, RunWaitResult, RunCancelResult, ReplayResult, StoreDescriptor, StoreExtensionHandler, StoreInvocation, ToolDescriptor, ToolExtensionHandler, ToolInvocation, WireEventEnvelope, WireHandle, WireValue, TurnStatus, TokenizerReference } from "./types.js";
export interface SpawnOptions {
    readonly hostCommand: string;
    readonly args?: readonly string[];
    readonly cwd?: string;
    readonly env?: NodeJS.ProcessEnv;
    readonly catalogPath?: string;
    readonly requiredFeatures?: readonly string[];
    readonly requiredCapabilities?: readonly string[];
    readonly contractDigest?: string;
    readonly sourceContractDigest?: string;
    readonly onStderr?: (chunk: string) => void;
}
export type ExtensionHandler = (call: unknown, signal: AbortSignal) => Promise<unknown> | unknown;
export type ExtensionInvokeOutcome = ExtensionOutcome<ExtensionResult>;
export declare class EchoAgentClient {
    readonly catalog: FacadeCatalog;
    readonly capability: EchoAgentCapability;
    private readonly app;
    private readonly connection;
    private readonly process;
    private readonly extensionHandlers;
    private readonly events;
    private readonly updates;
    private readonly queueCapacity;
    private closed;
    private constructor();
    static spawn(options: SpawnOptions): Promise<EchoAgentClient>;
    request<T = unknown>(method: string, params?: unknown): Promise<T>;
    notify(method: string, params?: unknown): Promise<void>;
    registerExtension(kind: string, implementationId: string, descriptor: unknown, handler: ExtensionHandler, timeout?: {
        readonly seconds: string;
        readonly nanos: number;
    }): Promise<ExtensionRegistration>;
    registerTypedExtension(implementationId: string, descriptor: ExtensionDescriptor, handler: ExtensionHandler, timeout?: {
        readonly seconds: string;
        readonly nanos: number;
    }): Promise<ExtensionRegistration>;
    private registerExtensionKind;
    registerTool(implementationId: string, descriptor: ToolDescriptor, handler: ToolExtensionHandler, timeout?: {
        readonly seconds: string;
        readonly nanos: number;
    }): Promise<ExtensionRegistration>;
    registerLlmClient(implementationId: string, descriptor: LlmClientDescriptor, handler: LlmClientExtensionHandler, timeout?: {
        readonly seconds: string;
        readonly nanos: number;
    }): Promise<ExtensionRegistration>;
    registerStore(implementationId: string, descriptor: StoreDescriptor, handler: StoreExtensionHandler, timeout?: {
        readonly seconds: string;
        readonly nanos: number;
    }): Promise<ExtensionRegistration>;
    registerCritic(implementationId: string, descriptor: CriticDescriptor, handler: CriticExtensionHandler, timeout?: {
        readonly seconds: string;
        readonly nanos: number;
    }): Promise<ExtensionRegistration>;
    registerContextCompressor(implementationId: string, descriptor: ContextCompressorDescriptor, handler: ContextCompressorExtensionHandler, timeout?: {
        readonly seconds: string;
        readonly nanos: number;
    }): Promise<ExtensionRegistration>;
    registerAgentComponent(implementationId: string, descriptor: AgentComponentDescriptor, handler: AgentComponentExtensionHandler, timeout?: {
        readonly seconds: string;
        readonly nanos: number;
    }): Promise<ExtensionRegistration>;
    registerHumanLoopProvider(implementationId: string, descriptor: ExtensionDescriptor & {
        readonly kind: "human_loop_provider";
    }, handler: ExtensionHandler): Promise<ExtensionRegistration>;
    registerHook(implementationId: string, descriptor: ExtensionDescriptor & {
        readonly kind: "hook";
    }, handler: ExtensionHandler): Promise<ExtensionRegistration>;
    registerAgentCallback(implementationId: string, descriptor: ExtensionDescriptor & {
        readonly kind: "agent_callback";
    }, handler: (call: ExtensionInvokeCall<AgentCallbackInvocation>, signal: AbortSignal) => Promise<AgentCallbackResult | ExtensionOutcome<AgentCallbackResult>> | AgentCallbackResult | ExtensionOutcome<AgentCallbackResult>): Promise<ExtensionRegistration>;
    registerInterventionCallback(implementationId: string, descriptor: ExtensionDescriptor & {
        readonly kind: "intervention_callback";
    }, handler: ExtensionHandler): Promise<ExtensionRegistration>;
    registerAgentFactory(implementationId: string, descriptor: ExtensionDescriptor & {
        readonly kind: "agent_factory";
    }, handler: ExtensionHandler): Promise<ExtensionRegistration>;
    registerCustomAgent(implementationId: string, descriptor: ExtensionDescriptor & {
        readonly kind: "custom_agent";
    }, handler: ExtensionHandler): Promise<ExtensionRegistration>;
    registerChannelPlugin(implementationId: string, descriptor: ExtensionDescriptor & {
        readonly kind: "channel_plugin";
    }, handler: ExtensionHandler): Promise<ExtensionRegistration>;
    registerChannelMessageHandler(implementationId: string, descriptor: ExtensionDescriptor & {
        readonly kind: "channel_message_handler";
    }, handler: ExtensionHandler): Promise<ExtensionRegistration>;
    unregisterExtension(extension: WireHandle): Promise<unknown>;
    streamWriter(stream: WireHandle): ExtensionStreamWriter;
    invoke<T = unknown>(operation: string, handle: WireHandle | undefined, args?: readonly unknown[]): Promise<T>;
    countTokens(tokenizer: TokenizerReference, text: string): Promise<bigint>;
    classifyTurnOutcome(eventWire: WireValue): Promise<WireValue | null>;
    call<T = unknown>(operation: string, handle: WireHandle | undefined, args?: readonly unknown[]): Promise<T>;
    family<T = unknown>(family: string, operation: string, session: WireHandle | undefined, args?: readonly unknown[]): Promise<T>;
    createAgent(config?: unknown, idempotencyId?: string): Promise<AgentHandle>;
    updatesFor(sessionId: string): AsyncIterable<unknown>;
    eventsFor(streamId: string, handle?: WireHandle): AsyncIterable<FacadeStreamItem>;
    publishFirstEvent(stream: WireHandle, envelope: WireEventEnvelope): void;
    endEvents(streamId: string): void;
    endUpdates(sessionId: string): void;
    acknowledge(stream: WireHandle, sequence: string): Promise<void>;
    observeReplay(stream: WireHandle, result: ReplayResult): void;
    private iterateEvents;
    close(): Promise<void>;
    [Symbol.asyncDispose](): Promise<void>;
}
export declare class AgentHandle {
    readonly client: EchoAgentClient;
    readonly wire: WireHandle;
    private closed;
    constructor(client: EchoAgentClient, wire: WireHandle);
    describe(): Promise<unknown>;
    createSession(cwd?: string): Promise<SessionHandle>;
    close(): Promise<void>;
}
export declare class ExtensionRegistration {
    readonly client: EchoAgentClient;
    readonly wire: WireHandle;
    private closed;
    constructor(client: EchoAgentClient, wire: WireHandle);
    close(): Promise<void>;
}
export declare class ExtensionStreamWriter {
    readonly client: EchoAgentClient;
    readonly stream: WireHandle;
    private sequence;
    private closed;
    constructor(client: EchoAgentClient, stream: WireHandle);
    private next;
    chunk(value: unknown): Promise<void>;
    agentComponentChunk(value: AgentComponentStreamChunk): Promise<void>;
    complete(value: unknown): Promise<void>;
    agentComponentComplete(value: AgentComponentStreamComplete): Promise<void>;
    failed(error: unknown): Promise<void>;
    cancelled(): Promise<void>;
}
export declare class SessionHandle {
    readonly client: EchoAgentClient;
    readonly agent: WireHandle;
    readonly wire: WireHandle;
    readonly acpSessionId: string;
    readonly taskRun?: WireHandle | undefined;
    private closed;
    constructor(client: EchoAgentClient, agent: WireHandle, wire: WireHandle, acpSessionId: string, taskRun?: WireHandle | undefined);
    prompt(text: string, options?: {
        readonly signal?: AbortSignal;
    }): Promise<unknown>;
    updates(): AsyncIterable<unknown>;
    startRun(input: {
        readonly mode?: "chat" | "execute";
        readonly text: string;
    }): Promise<RunHandle>;
    invoke<T = unknown>(operation: string, args?: readonly unknown[]): Promise<T>;
    cancel(): Promise<void>;
    close(): Promise<void>;
    private ensureOpen;
    private abortable;
}
export declare class RunHandle {
    readonly client: EchoAgentClient;
    readonly wire: WireHandle;
    readonly stream: WireHandle;
    private closed;
    constructor(client: EchoAgentClient, wire: WireHandle, stream: WireHandle);
    get events(): AsyncIterable<FacadeStreamItem>;
    get(): Promise<RunGetResult>;
    status(): Promise<TurnStatus>;
    outcomeStatus(): Promise<TurnStatus>;
    usage(): Promise<ExecutionUsage>;
    wait(options?: {
        readonly timeoutMs?: number;
        readonly signal?: AbortSignal;
    }): Promise<RunWaitResult>;
    cancel(): Promise<RunCancelResult>;
    steer(text: string): Promise<unknown>;
    replay(afterSequence?: string, limit?: number): Promise<ReplayResult>;
    close(): Promise<void>;
    private ensureOpen;
    private abortable;
}
/**
 * Decode the Host-owned reverse invocation envelope for a typed extension.
 * This is deliberately only a wire/shape adapter; execution and settlement
 * remain in the Rust Host and the official ACP connection.
 */
export declare function decodeExtensionInvokeCall<K extends "tool" | "llm_client" | "store" | "critic" | "context_compressor" | "agent_component">(value: unknown, expectedKind: K): ExtensionInvokeCall<K extends "tool" ? ToolInvocation : K extends "llm_client" ? LlmInvocation : K extends "store" ? StoreInvocation : K extends "critic" ? CriticInvocation : K extends "context_compressor" ? ContextCompressorInvocation : AgentComponentInvocation>;
export declare function decodeReplayResponse(value: unknown, stream: WireHandle, requestedAfter?: string): ReplayResult;
export declare function decodeAgentCallback(value: unknown): ExtensionInvokeCall<AgentCallbackInvocation>;
