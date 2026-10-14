import { MANAGED_OPERATIONS, validateManagedFields, validateManaged } from "./managed.js";
import { spawn } from "node:child_process";
import { Readable, Writable } from "node:stream";
import { client as createAcpClient, methods, ndJsonStream, } from "@agentclientprotocol/sdk";
import { FacadeCatalog } from "./catalog.js";
import { EchoAgentError } from "./errors.js";
import { fromWireValue, isWireHandle, isWireValue, toWireValue } from "./wire.js";
const DEFAULT_QUEUE_CAPACITY = 256;
const MAX_QUEUE_CAPACITY = 4096;
const DEFAULT_CLOSE_WAIT_MS = 5000;
class AsyncQueue {
    capacity;
    items = [];
    waiters = [];
    closed = false;
    constructor(capacity = DEFAULT_QUEUE_CAPACITY) {
        this.capacity = Math.max(1, Math.min(MAX_QUEUE_CAPACITY, Math.trunc(capacity)));
    }
    push(value, control = false) {
        if (this.closed)
            return;
        const controlIndex = this.items.findIndex(item => "control" in item && item.control);
        if (control && controlIndex >= 0) {
            const existing = this.items[controlIndex];
            const previous = existing && "value" in existing ? existing.value : undefined;
            if (isRecord(previous) && isRecord(previous.gap) && isRecord(value) && isRecord(value.gap)) {
                const oldFrom = previous.gap.from_sequence;
                const watermark = BigInt(String(value.gap.snapshot_watermark));
                const merged = { ...value, gap: { ...value.gap, ...(oldFrom === undefined ? {} : { from_sequence: oldFrom }) } };
                const retained = this.items.slice(controlIndex + 1).filter(item => {
                    if (!("value" in item) || !isRecord(item.value) || !isRecord(item.value.envelope))
                        return true;
                    const sequence = item.value.envelope.sequence;
                    return typeof sequence !== "string" || BigInt(sequence) > watermark;
                });
                this.items.splice(controlIndex, this.items.length - controlIndex, { value: merged, control: true }, ...retained);
            }
            else
                this.items.splice(controlIndex, 1, { value, control: true });
            return;
        }
        const dataCount = this.items.filter(item => "value" in item && !item.control).length;
        if (this.waiters.length === 0 && !control && dataCount >= this.capacity) {
            this.fail(new EchoAgentError("backpressure", `SDK stream queue exceeded its bound of ${this.capacity} data items`, "after_delay"));
            return;
        }
        this.shift({ value, ...(control ? { control: true } : {}) });
    }
    discardAcknowledgedEvents(sequence) {
        for (let index = this.items.length - 1; index >= 0; index--) {
            const item = this.items[index];
            if (item && "value" in item && !item.control && isRecord(item.value) && isRecord(item.value.envelope)
                && typeof item.value.envelope.sequence === "string" && BigInt(item.value.envelope.sequence) <= sequence)
                this.items.splice(index, 1);
        }
    }
    fail(error) {
        if (this.closed)
            return;
        this.closed = true;
        this.shift({ error });
        while (this.waiters.length > 0)
            this.shift({ done: true });
    }
    end() {
        if (this.closed)
            return;
        this.closed = true;
        while (this.waiters.length > 0)
            this.shift({ done: true });
    }
    shift(item) {
        const waiter = this.waiters.shift();
        if (waiter)
            waiter(item);
        else
            this.items.push(item);
    }
    async nextItem() {
        const item = this.items.shift();
        if (item)
            return item;
        if (this.closed)
            return { done: true };
        return new Promise((resolve) => this.waiters.push(resolve));
    }
    async *[Symbol.asyncIterator]() {
        while (true) {
            const item = await this.nextItem();
            if ("done" in item)
                return;
            if ("error" in item)
                throw item.error;
            yield item.value;
        }
    }
}
function boundedQueueCapacity(capability) {
    const raw = capability.limits.max_outstanding_live_events
        ?? capability.limits.max_stream_buffer_events;
    if (typeof raw === "number" && Number.isSafeInteger(raw) && raw > 0) {
        return Math.min(MAX_QUEUE_CAPACITY, raw);
    }
    if (typeof raw === "string" && /^\d+$/u.test(raw)) {
        try {
            const parsed = BigInt(raw);
            if (parsed > 0n)
                return Number(parsed > BigInt(MAX_QUEUE_CAPACITY) ? MAX_QUEUE_CAPACITY : parsed);
        }
        catch {
            // A malformed advertisement is rejected by the Host capability check;
            // retain a bounded local default until that check completes.
        }
    }
    return DEFAULT_QUEUE_CAPACITY;
}
// These intervals prove only received transport facts, not journal consumption.
function recordLiveSequence(feed, sequence) {
    const ranges = feed.liveRanges ??= [];
    const previous = ranges.at(-1);
    if (previous && previous.end + 1n === sequence)
        previous.end = sequence;
    else
        ranges.push({ base: sequence, end: sequence });
    if (ranges.length > 512)
        ranges.shift();
}
function replayCoversCursor(feed, sequence) {
    const ranges = feed.replayRanges ?? [];
    let frontier = feed.lastEventSequence;
    // Resolve the reachable closure independently of response arrival order.
    // Only returned events or an exact retention snapshot can authorize a cursor.
    for (let pass = 0; pass <= ranges.length; pass++) {
        const before = frontier;
        for (const range of ranges) {
            const connected = range.base <= frontier || (range.recoveryBase !== undefined && range.recoveryBase <= frontier);
            if (!connected)
                continue;
            if (sequence <= range.end && (sequence > range.base || (range.recoveryBase !== undefined && sequence === range.base)))
                return true;
            if (range.end > frontier)
                frontier = range.end;
        }
        if (frontier === before)
            break;
    }
    return false;
}
export class EchoAgentClient {
    catalog;
    capability;
    app;
    connection;
    process;
    extensionHandlers;
    events;
    updates;
    queueCapacity;
    closed = false;
    constructor(app, connection, child, catalog, capability, extensionHandlers, queueCapacity, events, updates) {
        this.app = app;
        this.connection = connection;
        this.process = child;
        this.catalog = catalog;
        this.capability = capability;
        this.extensionHandlers = extensionHandlers;
        this.queueCapacity = queueCapacity;
        this.events = events;
        this.updates = updates;
    }
    static async spawn(options) {
        const child = spawn(options.hostCommand, [...(options.args ?? [])], {
            cwd: options.cwd,
            env: options.env ? { ...process.env, ...options.env } : process.env,
            stdio: ["pipe", "pipe", "pipe"],
        });
        if (!child.stdin || !child.stdout || !child.stderr) {
            child.kill();
            throw new EchoAgentError("transport_error", "Host did not expose all stdio pipes");
        }
        if (options.onStderr) {
            child.stderr.on("data", (chunk) => options.onStderr?.(chunk.toString("utf8")));
        }
        await new Promise((resolve, reject) => {
            const onSpawn = () => {
                child.off("error", onError);
                resolve();
            };
            const onError = (error) => {
                child.off("spawn", onSpawn);
                reject(new EchoAgentError("transport_error", `Host failed to start: ${error.message}`));
            };
            child.once("spawn", onSpawn);
            child.once("error", onError);
        }).catch((error) => {
            child.kill();
            throw error;
        });
        const catalog = new FacadeCatalog(options.catalogPath);
        let queueCapacity = DEFAULT_QUEUE_CAPACITY;
        const eventFeeds = new Map();
        const updateQueues = new Map();
        const extensionHandlers = new Map();
        const invocationControllers = new Map();
        const app = createAcpClient({ name: "echo-agent-sdk-typescript" });
        const customApp = app;
        customApp.onNotification(methods.client.session.update, (params) => params, async ({ params }) => {
            const sessionId = readNonEmptyString(params, "sessionId");
            if (!sessionId) {
                failQueues(updateQueues, new EchoAgentError("serialization_violation", "session/update notification is missing sessionId"));
                return;
            }
            let queue = updateQueues.get(sessionId);
            if (!queue) {
                queue = new AsyncQueue(queueCapacity);
                updateQueues.set(sessionId, queue);
            }
            queue.push(params);
        });
        customApp.onNotification("_echo_agent/event", (params) => params, async ({ params }) => {
            let event;
            try {
                event = decodeEvent(params);
            }
            catch (error) {
                failFeeds(eventFeeds, EchoAgentError.fromUnknown(error, "_echo_agent/event"));
                return;
            }
            const streamId = event.stream.id;
            let feed = eventFeeds.get(streamId);
            if (!feed) {
                feed = { queue: new AsyncQueue(queueCapacity), lastEventSequence: 0n };
                eventFeeds.set(streamId, feed);
            }
            if (feed.handle && !sameHandle(feed.handle, event.stream)) {
                feed.queue.fail(new EchoAgentError("handle_mismatch", "event stream handle changed generation"));
                return;
            }
            feed.handle ??= { ...event.stream };
            const sequence = parsePositiveSequence(event.envelope.sequence);
            if (sequence === feed.lastEventSequence) {
                return;
            }
            if (sequence < feed.lastEventSequence) {
                feed.queue.fail(new EchoAgentError("serialization_violation", `event sequence ${event.envelope.sequence} is not strictly increasing`, "never", "_echo_agent/event"));
                return;
            }
            feed.lastEventSequence = sequence;
            feed.lastLiveSequence = sequence;
            recordLiveSequence(feed, sequence);
            feed.queue.push(event);
            if (["final_answer", "cancelled", "error"].includes(event.envelope.payload.event_type)) {
                // The terminal event remains available to the consumer before the
                // iterator observes completion. Run snapshots remain authoritative.
                feed.queue.end();
            }
        });
        customApp.onNotification("_echo_agent/gap", (params) => params, async ({ params }) => {
            let gap;
            try {
                gap = decodeGap(params);
            }
            catch (error) {
                failFeeds(eventFeeds, EchoAgentError.fromUnknown(error, "_echo_agent/gap"));
                return;
            }
            const streamId = gap.stream.id;
            let feed = eventFeeds.get(streamId);
            if (!feed) {
                feed = { queue: new AsyncQueue(queueCapacity), lastEventSequence: 0n };
                eventFeeds.set(streamId, feed);
            }
            if (feed.handle && !sameHandle(feed.handle, gap.stream)) {
                feed.queue.fail(new EchoAgentError("handle_mismatch", "gap stream handle changed generation"));
                return;
            }
            feed.handle ??= { ...gap.stream };
            feed.lastEventSequence = parsePositiveSequence(gap.gap.snapshot_watermark);
            feed.lastGapWatermark = feed.lastEventSequence;
            // One bounded control slot is reserved for the Host's pause notice.
            // The live-event window can already fill every data slot before attach.
            feed.queue.push(gap, true);
        });
        customApp.onNotification("_echo_agent/extension/cancel", (params) => params, async ({ params }) => {
            const invocationId = typeof params === "object" && params !== null && "invocation_id" in params
                ? String(params.invocation_id)
                : "";
            console.log("CANCEL_ROUTE "+JSON.stringify({at:Date.now(),invocationId,present:invocationControllers.has(invocationId)})); invocationControllers.get(invocationId)?.abort();
        });
        const factoryChildren = new Map();
        const closedFactoryChildren = new Map();
        const pendingFactoryConstruction = new Map();
        customApp.onRequest("_echo_agent/extension/invoke", (params) => params, async ({ params, signal }) => {
            const call = params;
            const extensionId = call.extension?.id;
            const operation = call.invocation?.operation;
            let handler = extensionId ? extensionHandlers.get(extensionId) : undefined;
            if (operation === "factory_create_agent") {
                const parent = handler;
                const instance = call.invocation?.input?.instance;
                const origin = call.extension;
                if (!parent?.registration || parent.registration.kind !== "agent_factory" || !isWireHandle(origin)
                    || !sameHandle(parent.registration.wire, origin) || !isWireHandle(instance) || instance.kind !== "extension"
                    || sameHandle(instance, origin) || instance.generation !== origin.generation) {
                    return { outcome: "error", error: { code: "serialization_violation", message: "factory construction requires its reserved instance handle", retryable: "never" } };
                }
                const key = JSON.stringify([instance.id, instance.generation, instance.kind]);
                if (factoryChildren.has(key) || closedFactoryChildren.has(key))
                    return { outcome: "error", error: { code: "extension_conflict", message: "factory instance identity was already constructed", retryable: "never" } };
                factoryChildren.set(key, origin);
            }
            if (call.factory_origin != null) {
                const origin = call.factory_origin;
                const child = call.extension;
                if (!isWireHandle(origin) || !isWireHandle(child))
                    return { outcome: "error", error: { code: "serialization_violation", message: "factory origin and instance must be valid extension handles", retryable: "never" } };
                const registered = extensionHandlers.get(origin.id);
                const metadata = registered?.registration;
                const childKey = JSON.stringify([child.id, child.generation, child.kind]);
                const previous = factoryChildren.get(childKey);
                const valid = isWireHandle(origin) && isWireHandle(child) && origin.kind === "extension" && child.kind === "extension"
                    && !sameHandle(origin, child) && origin.generation === child.generation
                    && metadata?.kind === "agent_factory" && sameHandle(metadata.wire, origin)
                    && ["agent_execute", "agent_chat", "agent_execute_stream", "agent_chat_stream", "agent_close"].includes(operation ?? "")
                    && (previous ? sameHandle(previous, origin) : operation === "agent_close" && sameHandle(closedFactoryChildren.get(childKey) ?? child, origin));
                if (!valid)
                    return { outcome: "error", error: { code: "invalid_value", message: "factory origin does not own this custom Agent invocation", retryable: "never" } };
                if (!previous && operation === "agent_close")
                    return { outcome: "result", result: { operation: "agent_close", value: null } };
                handler = registered;
            }
            if (!handler) {
                return {
                    outcome: "error",
                    error: {
                        code: "invalid_value",
                        message: "extension registration is not available in this SDK client",
                        retryable: "never",
                    },
                };
            }
            const controller = new AbortController();
            signal.addEventListener("abort", () => controller.abort(), { once: true });
            if (typeof call.invocation_id === "string")
                invocationControllers.set(call.invocation_id, controller);
            try {
                if (operation === "agent_close" && call.extension) {
                    const key = JSON.stringify([call.extension.id, call.extension.generation, call.extension.kind]);
                    const pending = pendingFactoryConstruction.get(key);
                    if (pending) {
                        const deadline = params.deadline;
                        const seconds = BigInt(deadline?.seconds ?? "0");
                        const milliseconds = Number(seconds > 5n ? 5n : seconds) * 1000 + Math.floor((deadline?.nanos ?? 0) / 1e6);
                        let timer;
                        try {
                            await Promise.race([pending, new Promise((_resolve, reject) => { timer = setTimeout(() => reject(new EchoAgentError("extension_timeout", "factory construction has not settled before cleanup deadline", "after_delay")), Math.max(1, milliseconds)); })]);
                        }
                        finally {
                            if (timer)
                                clearTimeout(timer);
                        }
                    }
                }
                let result;
                if (operation === "factory_create_agent" && call.invocation?.input?.instance) {
                    const instance = call.invocation.input.instance;
                    const key = JSON.stringify([instance.id, instance.generation, instance.kind]);
                    const construction = Promise.resolve().then(() => handler(params, controller.signal));
                    const settled = construction.then(() => undefined, () => undefined);
                    pendingFactoryConstruction.set(key, settled);
                    try {
                        result = await construction;
                    }
                    finally {
                        pendingFactoryConstruction.delete(key);
                    }
                }
                else
                    result = await handler(params, controller.signal);
                const outcome = controller.signal.aborted ? cancelledOutcome() : normalizeOutcome(result);
                if (operation === "agent_close" && outcome.outcome === "result" && outcome.result.operation === "agent_close") {
                    const key = JSON.stringify([call.extension?.id, call.extension?.generation, call.extension?.kind]);
                    const origin = factoryChildren.get(key);
                    factoryChildren.delete(key);
                    if (origin)
                        closedFactoryChildren.set(key, origin);
                    if (closedFactoryChildren.size > 512) {
                        const oldest = closedFactoryChildren.keys().next().value;
                        if (oldest !== undefined)
                            closedFactoryChildren.delete(oldest);
                    }
                }
                return outcome;
            }
            catch (error) {
                if (controller.signal.aborted)
                    return cancelledOutcome();
                const failure = extensionFailure(error);
                return {
                    outcome: "error",
                    error: {
                        code: failure.code,
                        message: failure.message,
                        retryable: failure.retryable,
                        operation: failure.operation,
                        details: failure.details,
                        handle: failure.handle,
                    },
                };
            }
            finally {
                if (typeof call.invocation_id === "string")
                    { console.log("DELETE_ROUTE "+JSON.stringify({at:Date.now(),invocationId:call.invocation_id,operation:call.invocation?.operation,stream:call.stream})); invocationControllers.delete(call.invocation_id); }
            }
        });
        const stream = ndJsonStream(Writable.toWeb(child.stdin), Readable.toWeb(child.stdout));
        const connection = app.connect(stream);
        const hello = {
            extension_protocol_version: 1,
            contract_digest: options.contractDigest ?? catalog.contractDigest,
            source_contract_digest: options.sourceContractDigest ?? catalog.sourceContractDigest,
            required_features: [...(options.requiredFeatures ?? [])].sort(),
            required_capabilities: [...(options.requiredCapabilities ?? [])].sort(),
        };
        try {
            const initialized = await connection.agent.request("initialize", {
                protocolVersion: 1,
                clientCapabilities: {
                    fs: { readTextFile: false, writeTextFile: false },
                    terminal: false,
                    _meta: { echo_agent: hello },
                },
            });
            if (initialized.protocolVersion !== 1)
                throw new EchoAgentError("extension_capability_mismatch", "unsupported ACP protocol version");
            const capability = initialized
                .agentCapabilities?._meta?.echo_agent;
            if (!capability)
                throw new EchoAgentError("extension_capability_mismatch", "Host did not advertise echo-agent extension");
            validateCapability(capability, { ...options, contractDigest: hello.contract_digest, sourceContractDigest: hello.source_contract_digest });
            queueCapacity = boundedQueueCapacity(capability);
            const result = new EchoAgentClient(app, connection, child, catalog, capability, extensionHandlers, queueCapacity, eventFeeds, updateQueues);
            const onHostExit = () => {
                const error = new EchoAgentError("host_exited", "echo-agent SDK Host exited before the client closed");
                for (const feed of result.events.values()) {
                    if (result.closed)
                        feed.queue.end();
                    else
                        feed.queue.fail(error);
                }
                for (const queue of result.updates.values()) {
                    if (result.closed)
                        queue.end();
                    else
                        queue.fail(error);
                }
            };
            child.once("exit", onHostExit);
            child.once("error", (error) => {
                if (result.closed)
                    return;
                const failure = new EchoAgentError("transport_error", `Host process failed: ${error.message}`);
                failFeeds(result.events, failure);
                failQueues(result.updates, failure);
            });
            return result;
        }
        catch (error) {
            connection.close(error);
            child.kill();
            throw EchoAgentError.fromUnknown(error);
        }
    }
    async request(method, params) {
        try {
            return await this.connection.agent
                .request(method, params);
        }
        catch (error) {
            throw EchoAgentError.fromUnknown(error, method);
        }
    }
    async notify(method, params) {
        try {
            await this.connection.agent
                .notify(method, params);
        }
        catch (error) {
            throw EchoAgentError.fromUnknown(error, method);
        }
    }
    async registerExtension(kind, implementationId, descriptor, handler, timeout) {
        const response = await this.request("_echo_agent/extension/register", {
            kind,
            implementation_id: implementationId,
            descriptor,
            timeout,
        });
        // Metadata is tied to this registration wrapper, never to the reusable callback.
        const registered = Object.assign((call, signal) => handler(call, signal), { registration: { wire: response.extension, kind } });
        this.extensionHandlers.set(response.extension.id, registered);
        return new ExtensionRegistration(this, response.extension);
    }
    async registerTypedExtension(implementationId, descriptor, handler, timeout) {
        if (descriptor.descriptor_version !== 1) {
            throw new EchoAgentError("invalid_value", "only extension descriptor_version 1 is supported");
        }
        return this.registerExtension(descriptor.kind, implementationId, descriptor, handler, timeout);
    }
    async registerExtensionKind(expected, implementationId, descriptor, handler) {
        if (descriptor.kind !== expected) {
            throw new EchoAgentError("invalid_value", `expected ${expected} extension descriptor, received ${descriptor.kind}`);
        }
        return this.registerTypedExtension(implementationId, descriptor, handler);
    }
    async registerTool(implementationId, descriptor, handler, timeout) {
        return this.registerExtension("tool", implementationId, descriptor, typedExtensionHandler("tool", handler), timeout);
    }
    async registerLlmClient(implementationId, descriptor, handler, timeout) {
        return this.registerExtension("llm_client", implementationId, descriptor, typedExtensionHandler("llm_client", handler), timeout);
    }
    async registerStore(implementationId, descriptor, handler, timeout) {
        return this.registerExtension("store", implementationId, descriptor, typedExtensionHandler("store", handler), timeout);
    }
    async registerCritic(implementationId, descriptor, handler, timeout) {
        return this.registerExtension("critic", implementationId, descriptor, typedExtensionHandler("critic", handler), timeout);
    }
    async registerContextCompressor(implementationId, descriptor, handler, timeout) {
        return this.registerExtension("context_compressor", implementationId, descriptor, typedExtensionHandler("context_compressor", handler), timeout);
    }
    async registerAgentComponent(implementationId, descriptor, handler, timeout) {
        validateAgentComponentDescriptor(descriptor);
        return this.registerExtension("agent_component", implementationId, descriptor, typedExtensionHandler("agent_component", handler), timeout);
    }
    async registerHumanLoopProvider(implementationId, descriptor, handler) {
        return this.registerExtensionKind("human_loop_provider", implementationId, descriptor, handler);
    }
    async registerHook(implementationId, descriptor, handler) {
        return this.registerExtensionKind("hook", implementationId, descriptor, handler);
    }
    async registerAgentCallback(implementationId, descriptor, handler) {
        return this.registerExtensionKind("agent_callback", implementationId, descriptor, async (raw, signal) => {
            const call = decodeAgentCallback(raw);
            const result = await handler(call, signal);
            const outcome = normalizeTypedExtensionResult(call.invocation.operation, result);
            if (outcome.outcome === "result" && outcome.result.value !== null)
                throw new EchoAgentError("serialization_violation", "Agent callback must return null unit value");
            return outcome;
        });
    }
    async registerInterventionCallback(implementationId, descriptor, handler) {
        return this.registerExtensionKind("intervention_callback", implementationId, descriptor, handler);
    }
    async registerAgentFactory(implementationId, descriptor, handler) {
        return this.registerExtensionKind("agent_factory", implementationId, descriptor, handler);
    }
    async registerCustomAgent(implementationId, descriptor, handler) {
        return this.registerExtensionKind("custom_agent", implementationId, descriptor, handler);
    }
    async registerChannelPlugin(implementationId, descriptor, handler) {
        return this.registerExtensionKind("channel_plugin", implementationId, descriptor, handler);
    }
    async registerChannelMessageHandler(implementationId, descriptor, handler) {
        return this.registerExtensionKind("channel_message_handler", implementationId, descriptor, handler);
    }
    async unregisterExtension(extension) {
        const result = await this.request("_echo_agent/extension/unregister", { extension });
        this.extensionHandlers.delete(extension.id);
        return result;
    }
    streamWriter(stream) {
        return new ExtensionStreamWriter(this, stream);
    }
    async invoke(operation, handle, args = []) {
        const value = await this.request("_echo_agent/facade/invoke", {
            operation,
            signature_digest: this.catalog.signature(operation),
            handle,
            arguments: args.map(toWireValue),
        });
        if (!value || !("value" in value) || !isWireValue(value.value))
            throw new EchoAgentError("serialization_violation", "facade response requires a value envelope");
        return fromWireValue(value.value);
    }
    async countTokens(tokenizer, text) {
        const count = await this.invoke("echo_core::tokenizer::Tokenizer::count_tokens", tokenizer.resource, [tokenizer.owner_session_id, text]);
        return BigInt(count);
    }
    async classifyTurnOutcome(eventWire) {
        const operation = "echo_orchestration::runtime::turn_driver::TurnOutcome::classify";
        return this.invoke(operation, undefined, [eventWire]);
    }
    async call(operation, handle, args = []) {
        const resolved = this.catalog.resolve(operation);
        if (resolved.method === "_echo_agent/facade/invoke")
            return this.invoke(operation, handle, args);
        return this.family(resolved.family, operation, handle, args);
    }
    async family(family, operation, session, args = []) {
        // Family names are catalog identities; their wire method can use a
        // different spelling (for example content_guard -> content-guard).
        const resolved = this.catalog.resolve(operation);
        if (resolved.family !== family || resolved.method === "_echo_agent/facade/invoke") {
            throw new EchoAgentError("invalid_value", "operation does not belong to the requested facade family");
        }
        const method = resolved.method;
        const value = await this.request(method, {
            operation,
            signature_digest: this.catalog.familySignature(method, operation),
            handle: session,
            arguments: args.map(toWireValue),
        });
        if (!value || !("value" in value) || !isWireValue(value.value))
            throw new EchoAgentError("serialization_violation", "facade response requires a value envelope");
        return fromWireValue(value.value);
    }
    async createAgent(config = { variant: "host_default" }, idempotencyId) {
        const response = await this.request("_echo_agent/agent/create", {
            config,
            idempotency_id: idempotencyId,
        });
        return new AgentHandle(this, response.agent);
    }
    updatesFor(sessionId) {
        let queue = this.updates.get(sessionId);
        if (!queue) {
            queue = new AsyncQueue(this.queueCapacity);
            this.updates.set(sessionId, queue);
        }
        return queue;
    }
    eventsFor(streamId, handle) {
        let feed = this.events.get(streamId);
        if (!feed) {
            feed = { queue: new AsyncQueue(this.queueCapacity), handle: handle ? { ...handle } : undefined, lastEventSequence: 0n };
            this.events.set(streamId, feed);
        }
        else if (handle && feed.handle && !sameHandle(feed.handle, handle)) {
            feed.queue.fail(new EchoAgentError("handle_mismatch", "event stream handle changed generation"));
        }
        else {
            if (handle)
                feed.handle ??= { ...handle };
        }
        return this.iterateEvents(feed);
    }
    publishFirstEvent(stream, envelope) {
        const event = decodeEvent({ stream, envelope });
        let feed = this.events.get(stream.id);
        if (!feed) {
            feed = { queue: new AsyncQueue(this.queueCapacity), handle: { ...stream }, lastEventSequence: 0n };
            this.events.set(stream.id, feed);
        }
        if (feed.handle && !sameHandle(feed.handle, stream)) {
            feed.queue.fail(new EchoAgentError("handle_mismatch", "event stream handle changed generation"));
            return;
        }
        const sequence = parsePositiveSequence(event.envelope.sequence);
        if (sequence <= feed.lastEventSequence)
            return;
        feed.handle = { ...stream };
        feed.lastEventSequence = sequence;
        feed.lastLiveSequence = sequence;
        recordLiveSequence(feed, sequence);
        feed.queue.push(event);
    }
    endEvents(streamId) {
        this.events.get(streamId)?.queue.end();
    }
    endUpdates(sessionId) {
        this.updates.get(sessionId)?.end();
    }
    async acknowledge(stream, sequence) {
        // The official writer queues notification objects; capture the address
        // before awaiting so caller mutations cannot change the submitted identity.
        stream = { ...stream };
        const parsed = parsePositiveSequence(sequence);
        const feed = this.events.get(stream.id);
        const owned = feed?.handle && sameHandle(feed.handle, stream);
        const covered = feed && replayCoversCursor(feed, parsed);
        const observed = owned && (covered || parsed === feed.lastGapWatermark
            || parsed <= (feed.acknowledgedSequence ?? 0n)
            || feed.liveRanges?.some(range => range.base <= parsed && parsed <= range.end));
        try {
            await this.notify("_echo_agent/event/ack", {
                ack: { stream, last_processed_sequence: parsed.toString() },
            });
        }
        catch (error) {
            const failure = EchoAgentError.fromUnknown(error, "_echo_agent/event/ack");
            throw new EchoAgentError(failure.code, failure.message, failure.retryable, failure.operation, { ack_submission: "outcome_unknown", stream, sequence: parsed.toString(), cause_details: failure.details }, failure.handle);
        }
        if (feed && observed) {
            // Each completion commits only its own submitted, observed cursor.
            // Another in-flight ACK cannot supply successful consumption evidence.
            if (parsed > (feed.acknowledgedSequence ?? 0n))
                feed.acknowledgedSequence = parsed;
            if (parsed > feed.lastEventSequence)
                feed.lastEventSequence = parsed;
            const committed = feed.acknowledgedSequence ?? 0n;
            feed.queue.discardAcknowledgedEvents(committed);
            if (feed.replayTerminalSequence !== undefined && committed >= feed.replayTerminalSequence)
                feed.queue.end();
        }
    }
    observeReplay(stream, result) {
        const feed = this.events.get(stream.id);
        if (!feed?.handle || !sameHandle(feed.handle, stream))
            return;
        const base = BigInt(result.gap?.gap.snapshot_watermark ?? result.requested_after_sequence);
        const end = BigInt(result.next_cursor.last_processed_sequence);
        for (const event of result.events) {
            if (["final_answer", "cancelled", "error"].includes(event.envelope.payload.event_type))
                feed.replayTerminalSequence = BigInt(event.envelope.sequence);
        }
        const ranges = feed.replayRanges ??= [];
        // Empty pages have no event coverage and must not intercept a later chain.
        if (!result.gap && base === end)
            return;
        if (result.gap) {
            ranges.push({ base, end, recoveryBase: BigInt(result.requested_after_sequence) });
        }
        else {
            // Merge all overlapping ordinary pages, preserving every gap's own bridge.
            let mergedBase = base;
            let mergedEnd = end;
            let changed = true;
            while (changed) {
                changed = false;
                for (let index = ranges.length - 1; index >= 0; index--) {
                    const range = ranges[index];
                    if (!range || range.recoveryBase !== undefined || range.base > mergedEnd || range.end < mergedBase)
                        continue;
                    mergedBase = range.base < mergedBase ? range.base : mergedBase;
                    mergedEnd = range.end > mergedEnd ? range.end : mergedEnd;
                    ranges.splice(index, 1);
                    changed = true;
                }
            }
            ranges.push({ base: mergedBase, end: mergedEnd });
        }
        if (ranges.length > 512)
            ranges.shift();
    }
    async *iterateEvents(feed) {
        let pendingSequence;
        try {
            for await (const queued of feed.queue) {
                const acknowledged = feed.acknowledgedSequence ?? 0n;
                let item = queued;
                if ("gap" in item) {
                    if (BigInt(item.gap.snapshot_watermark) <= acknowledged)
                        continue;
                    if (BigInt(item.gap.from_sequence) <= acknowledged)
                        item = { ...item, gap: { ...item.gap, from_sequence: (acknowledged + 1n).toString() } };
                }
                else if (BigInt(item.envelope.sequence) <= acknowledged)
                    continue;
                // A gap requires explicit replay/snapshot recovery and an explicit ACK.
                pendingSequence = "gap" in item ? undefined : item.envelope.sequence;
                yield item;
                if (pendingSequence && feed.handle) {
                    await this.acknowledge(feed.handle, pendingSequence);
                    pendingSequence = undefined;
                }
            }
        }
        finally {
            if (pendingSequence && feed.handle && !this.closed) {
                try {
                    await this.acknowledge(feed.handle, pendingSequence);
                }
                catch {
                    // The Host may already have exited while the consumer was closing.
                }
            }
        }
    }
    async close() {
        if (this.closed)
            return;
        this.closed = true;
        this.extensionHandlers.clear();
        for (const feed of this.events.values())
            feed.queue.end();
        for (const queue of this.updates.values())
            queue.end();
        this.connection.close();
        const exited = new Promise((resolve) => {
            if (this.process.exitCode !== null)
                resolve();
            else
                this.process.once("exit", () => resolve());
        });
        const settled = await Promise.race([
            exited.then(() => true),
            new Promise((resolve) => setTimeout(() => resolve(false), DEFAULT_CLOSE_WAIT_MS)),
        ]);
        if (!settled && !this.process.killed) {
            this.process.kill();
            await exited;
        }
    }
    async [Symbol.asyncDispose]() {
        await this.close();
    }
}
export class AgentHandle {
    client;
    wire;
    closed = false;
    constructor(client, wire) {
        this.client = client;
        this.wire = wire;
    }
    async describe() {
        if (this.closed)
            throw new EchoAgentError("closed_handle", "agent handle is closed");
        return this.client.request("_echo_agent/agent/describe", { agent: this.wire });
    }
    async createSession(cwd) {
        if (this.closed)
            throw new EchoAgentError("closed_handle", "agent handle is closed");
        const response = await this.client.request("_echo_agent/session/create", {
            agent: this.wire,
            working_dir: cwd ? { encoding: "utf8", path: cwd } : undefined,
        });
        return new SessionHandle(this.client, this.wire, response.session, response.acp_session_id, response.task_run);
    }
    async close() {
        if (this.closed)
            return;
        await this.client.request("_echo_agent/agent/close", { agent: this.wire });
        this.closed = true;
    }
}
export class ExtensionRegistration {
    client;
    wire;
    closed = false;
    constructor(client, wire) {
        this.client = client;
        this.wire = wire;
    }
    async close() {
        if (this.closed)
            return;
        await this.client.unregisterExtension(this.wire);
        this.closed = true;
    }
}
export class ExtensionStreamWriter {
    client;
    stream;
    sequence = 0n;
    closed = false;
    constructor(client, stream) {
        this.client = client;
        this.stream = stream;
    }
    next() {
        if (this.closed)
            throw new Error("extension stream is already closed");
        if (this.sequence === (1n << 64n) - 1n)
            throw new RangeError("extension stream sequence exceeds u64");
        this.sequence += 1n;
        return this.sequence.toString();
    }
    async chunk(value) {
        await this.client.notify("_echo_agent/extension/stream", {
            event: "chunk", stream: this.stream, sequence: this.next(), value,
        });
    }
    async agentComponentChunk(value) {
        await this.chunk({ kind: "agent_component", value });
    }
    async complete(value) {
        await this.client.notify("_echo_agent/extension/stream", {
            event: "complete", stream: this.stream, sequence: this.next(), value,
        });
        this.closed = true;
    }
    async agentComponentComplete(value) {
        await this.complete({ kind: "agent_component", value });
    }
    async failed(error) {
        await this.client.notify("_echo_agent/extension/stream", {
            event: "failed", stream: this.stream, sequence: this.next(), error,
        });
        this.closed = true;
    }
    async cancelled() {
        await this.client.notify("_echo_agent/extension/stream", {
            event: "cancelled", stream: this.stream, sequence: this.next(),
        });
        this.closed = true;
    }
}
export class SessionHandle {
    client;
    agent;
    wire;
    acpSessionId;
    taskRun;
    closed = false;
    constructor(client, agent, wire, acpSessionId, taskRun) {
        this.client = client;
        this.agent = agent;
        this.wire = wire;
        this.acpSessionId = acpSessionId;
        this.taskRun = taskRun;
    }
    async prompt(text, options) {
        this.ensureOpen();
        const request = this.client.request("session/prompt", {
            sessionId: this.acpSessionId,
            prompt: [{ type: "text", text }],
        });
        return this.abortable(request, options?.signal, () => this.cancel());
    }
    updates() {
        this.ensureOpen();
        return this.client.updatesFor(this.acpSessionId);
    }
    async startRun(input) {
        this.ensureOpen();
        const response = await this.client.request("_echo_agent/run/start", {
            session: this.wire,
            input: input.mode === "execute" ? { kind: "execute", task: input.text } : { kind: "chat", text: input.text },
        });
        if (response.first_event)
            this.client.publishFirstEvent(response.stream, response.first_event);
        return new RunHandle(this.client, response.run, response.stream);
    }
    async invoke(operation, args = []) {
        this.ensureOpen();
        const resolved = this.client.catalog.resolve(operation);
        return resolved.method === "_echo_agent/facade/invoke"
            ? this.client.call(operation, this.agent, [this.wire, ...args])
            : this.client.call(operation, this.wire, args);
    }
    async cancel() {
        this.ensureOpen();
        await this.client.notify("session/cancel", { sessionId: this.acpSessionId });
    }
    async close() {
        if (this.closed)
            return;
        await this.client.request("_echo_agent/session/close", { session: this.wire });
        this.closed = true;
        this.client.endUpdates(this.acpSessionId);
    }
    ensureOpen() {
        if (this.closed)
            throw new EchoAgentError("closed_handle", "session handle is closed");
    }
    async abortable(request, signal, cancel) {
        if (!signal)
            return request;
        if (signal.aborted) {
            await cancel().catch(() => undefined);
            throw new EchoAgentError("cancelled", "session operation was cancelled");
        }
        return new Promise((resolve, reject) => {
            let settled = false;
            const onAbort = () => {
                if (settled)
                    return;
                settled = true;
                signal.removeEventListener("abort", onAbort);
                void cancel().catch(() => undefined);
                reject(new EchoAgentError("cancelled", "session operation was cancelled"));
            };
            signal.addEventListener("abort", onAbort, { once: true });
            request.then((value) => {
                if (settled)
                    return;
                settled = true;
                signal.removeEventListener("abort", onAbort);
                resolve(value);
            }, (error) => {
                if (settled)
                    return;
                settled = true;
                signal.removeEventListener("abort", onAbort);
                reject(error);
            });
        });
    }
}
export class RunHandle {
    client;
    wire;
    stream;
    closed = false;
    constructor(client, wire, stream) {
        this.client = client;
        this.wire = wire;
        this.stream = stream;
    }
    get events() { return this.client.eventsFor(this.stream.id, this.stream); }
    get() {
        this.ensureOpen();
        return this.client.request("_echo_agent/run/get", { run: this.wire });
    }
    status() {
        this.ensureOpen();
        return this.client.call("echo_orchestration::runtime::turn_driver::TurnReceipt::status", this.wire, []);
    }
    outcomeStatus() {
        this.ensureOpen();
        return this.client.call("echo_orchestration::runtime::turn_driver::TurnOutcome::status", this.wire, []);
    }
    usage() {
        this.ensureOpen();
        return this.client.call("echo_orchestration::runtime::turn_driver::TurnReceipt::usage", this.wire, []);
    }
    wait(options) {
        this.ensureOpen();
        const timeout = options?.timeoutMs === undefined ? undefined : durationForMilliseconds(options.timeoutMs);
        const request = this.client.request("_echo_agent/run/wait", { run: this.wire, timeout });
        return this.abortable(request, options?.signal, () => this.cancel().then(() => undefined));
    }
    cancel() {
        this.ensureOpen();
        return this.client.request("_echo_agent/run/cancel", { run: this.wire });
    }
    steer(text) {
        this.ensureOpen();
        return this.client.request("_echo_agent/run/steer", { run: this.wire, text });
    }
    async replay(afterSequence = "0", limit = 512) {
        this.ensureOpen();
        const raw = await this.client.request("_echo_agent/run/replay", {
            stream: this.stream, after_sequence: afterSequence, max_events: String(limit),
        });
        const result = decodeReplayResponse(raw, this.stream, afterSequence);
        this.client.observeReplay(this.stream, result);
        return result;
    }
    async close() {
        if (this.closed)
            return;
        const state = await this.get().catch((error) => { throw error; });
        if (!isTerminalStatus(state.status)) {
            await this.cancel();
            const settled = await this.wait({ timeoutMs: DEFAULT_CLOSE_WAIT_MS });
            if (!settled.settled)
                throw new EchoAgentError("timeout", "run did not settle before close timeout", "after_delay");
        }
        this.closed = true;
        this.client.endEvents(this.stream.id);
    }
    ensureOpen() {
        if (this.closed)
            throw new EchoAgentError("closed_handle", "run handle is closed");
    }
    async abortable(request, signal, cancel) {
        if (!signal)
            return request;
        if (signal.aborted) {
            await cancel().catch(() => undefined);
            throw new EchoAgentError("cancelled", "run operation was cancelled");
        }
        return new Promise((resolve, reject) => {
            let settled = false;
            const onAbort = () => {
                if (settled)
                    return;
                settled = true;
                signal.removeEventListener("abort", onAbort);
                void cancel().catch(() => undefined);
                reject(new EchoAgentError("cancelled", "run operation was cancelled"));
            };
            signal.addEventListener("abort", onAbort, { once: true });
            request.then((value) => {
                if (settled)
                    return;
                settled = true;
                signal.removeEventListener("abort", onAbort);
                resolve(value);
            }, (error) => {
                if (settled)
                    return;
                settled = true;
                signal.removeEventListener("abort", onAbort);
                reject(error);
            });
        });
    }
}
function readNonEmptyString(value, key) {
    if (typeof value !== "object" || value === null || !(key in value))
        return undefined;
    const candidate = value[key];
    return typeof candidate === "string" && candidate.trim().length > 0 ? candidate : undefined;
}
function parsePositiveSequence(value) {
    if (typeof value !== "string" || !/^\d+$/u.test(value)) {
        throw new EchoAgentError("serialization_violation", "event sequence must be decimal text");
    }
    try {
        const sequence = BigInt(value);
        if (sequence < 1n || sequence > ((1n << 64n) - 1n) || sequence.toString() !== value) {
            throw new EchoAgentError("serialization_violation", "event sequence is outside the u64 range");
        }
        return sequence;
    }
    catch (error) {
        if (error instanceof EchoAgentError)
            throw error;
        throw new EchoAgentError("serialization_violation", "event sequence is not a valid u64");
    }
}
function isCanonicalU64Text(value) {
    if (typeof value !== "string" || !/^(?:0|[1-9]\d*)$/u.test(value))
        return false;
    try {
        const parsed = BigInt(value);
        return parsed <= ((1n << 64n) - 1n) && parsed.toString() === value;
    }
    catch {
        return false;
    }
}
function sameHandle(left, right) {
    return left.id === right.id && left.generation === right.generation && left.kind === right.kind;
}
function decodeEvent(value) {
    if (typeof value !== "object" || value === null) {
        throw new EchoAgentError("serialization_violation", "event notification must be an object");
    }
    const candidate = value;
    const stream = candidate.stream;
    if (!isWireHandle(stream) || stream.kind !== "stream") {
        throw new EchoAgentError("serialization_violation", "event notification has an invalid stream handle");
    }
    const envelope = candidate.envelope;
    if (typeof envelope !== "object" || envelope === null) {
        throw new EchoAgentError("serialization_violation", "event notification is missing its envelope");
    }
    const entry = envelope;
    const eventId = readNonEmptyString(entry, "event_id");
    const contentHash = readNonEmptyString(entry, "content_hash");
    const streamId = readNonEmptyString(entry, "stream_id");
    const turnId = readNonEmptyString(entry, "turn_id");
    if (!eventId || !contentHash || !streamId || !turnId || !/^sha256:[0-9a-f]{64}$/iu.test(contentHash)) {
        throw new EchoAgentError("serialization_violation", "event envelope identity is malformed");
    }
    if (streamId !== stream.id) {
        throw new EchoAgentError("serialization_violation", "event envelope stream_id does not match its handle");
    }
    const sequence = readNonEmptyString(entry, "sequence");
    if (!sequence)
        throw new EchoAgentError("serialization_violation", "event envelope is missing sequence");
    parsePositiveSequence(sequence);
    const payload = entry.payload;
    if (typeof payload !== "object" || payload === null) {
        throw new EchoAgentError("serialization_violation", "event envelope payload is missing");
    }
    const payloadRecord = payload;
    const eventType = readNonEmptyString(payloadRecord, "event_type");
    if (!eventType)
        throw new EchoAgentError("serialization_violation", "event envelope payload is malformed");
    const schemaVersion = entry.schema_version;
    if (typeof schemaVersion !== "number" || !Number.isInteger(schemaVersion) || schemaVersion < 1) {
        throw new EchoAgentError("serialization_violation", "event envelope schema_version is malformed");
    }
    const timestamp = entry.timestamp;
    if (typeof timestamp !== "object" || timestamp === null) {
        throw new EchoAgentError("serialization_violation", "event envelope timestamp is malformed");
    }
    const timestampRecord = timestamp;
    const unixSeconds = readNonEmptyString(timestampRecord, "unix_seconds");
    const nanos = timestampRecord.nanos;
    if (!unixSeconds || typeof nanos !== "number" || !Number.isInteger(nanos) || nanos < 0 || nanos >= 1_000_000_000) {
        throw new EchoAgentError("serialization_violation", "event envelope timestamp is malformed");
    }
    try {
        const parsed = BigInt(unixSeconds);
        if (parsed < -(1n << 63n) || parsed > (1n << 63n) - 1n || parsed.toString() !== unixSeconds) {
            throw new Error("timestamp range");
        }
    }
    catch {
        throw new EchoAgentError("serialization_violation", "event envelope timestamp seconds are malformed");
    }
    return {
        stream,
        envelope: {
            schema_version: schemaVersion,
            event_id: eventId,
            content_hash: contentHash,
            sequence,
            stream_id: streamId,
            conversation_id: readNonEmptyString(entry, "conversation_id"),
            run_id: readNonEmptyString(entry, "run_id"),
            turn_id: turnId,
            message_id: readNonEmptyString(entry, "message_id"),
            execution_id: readNonEmptyString(entry, "execution_id"),
            parent_event_id: readNonEmptyString(entry, "parent_event_id"),
            timestamp: {
                unix_seconds: unixSeconds,
                nanos,
                ...(readNonEmptyString(timestampRecord, "rfc3339") === undefined
                    ? {}
                    : { rfc3339: readNonEmptyString(timestampRecord, "rfc3339") }),
            },
            payload: {
                event_type: eventType,
                ...(payloadRecord.data === undefined ? {} : { data: payloadRecord.data }),
            },
        },
    };
}
function decodeGap(value) {
    if (typeof value !== "object" || value === null) {
        throw new EchoAgentError("serialization_violation", "gap notification must be an object");
    }
    const candidate = value;
    const stream = candidate.stream;
    if (!isWireHandle(stream) || stream.kind !== "stream") {
        throw new EchoAgentError("serialization_violation", "gap notification has an invalid stream handle");
    }
    const gap = candidate.gap;
    if (typeof gap !== "object" || gap === null) {
        throw new EchoAgentError("serialization_violation", "gap notification is missing its gap");
    }
    const entry = gap;
    const from = readNonEmptyString(entry, "from_sequence");
    const to = readNonEmptyString(entry, "to_sequence");
    const watermark = readNonEmptyString(entry, "snapshot_watermark");
    const reason = readNonEmptyString(entry, "reason");
    if (!from || !to || !watermark || !reason) {
        throw new EchoAgentError("serialization_violation", "gap fields are malformed");
    }
    const fromValue = parsePositiveSequence(from);
    const toValue = parsePositiveSequence(to);
    const watermarkValue = parsePositiveSequence(watermark);
    if (toValue < fromValue || watermarkValue < toValue) {
        throw new EchoAgentError("serialization_violation", "gap sequence range is malformed");
    }
    return { stream, gap: { from_sequence: from, to_sequence: to, reason, snapshot_watermark: watermark } };
}
function failFeeds(feeds, error) {
    for (const feed of feeds.values())
        feed.queue.fail(error);
}
function failQueues(queues, error) {
    for (const queue of queues.values())
        queue.fail(error);
}
function durationForMilliseconds(value) {
    if (!Number.isSafeInteger(value) || value < 0) {
        throw new RangeError("timeoutMs must be a non-negative safe integer");
    }
    const seconds = Math.floor(value / 1000);
    const nanos = (value % 1000) * 1_000_000;
    return { seconds: String(seconds), nanos };
}
function isTerminalStatus(status) {
    return status === "completed" || status === "cancelled" || status === "failed" || status === "interrupted" || status === "closed";
}
function validateCapability(capability, options) {
    if (capability.extension_protocol_version !== 1)
        throw new EchoAgentError("extension_capability_mismatch", "unsupported extension protocol version");
    if (!Array.isArray(capability.features) || !capability.features.every(value => typeof value === "string")
        || !Array.isArray(capability.capabilities) || !capability.capabilities.every(value => value && typeof value.capability === "string")
        || typeof capability.limits !== "object" || capability.limits === null || Array.isArray(capability.limits)) {
        throw new EchoAgentError("extension_capability_mismatch", "malformed Host capability advertisement");
    }
    for (const feature of options.requiredFeatures ?? []) {
        if (!capability.features.includes(feature))
            throw new EchoAgentError("feature_unavailable", `Host lacks feature ${feature}`);
    }
    for (const required of options.requiredCapabilities ?? []) {
        if (!capability.capabilities.some((entry) => entry.capability === required)) {
            throw new EchoAgentError("extension_capability_mismatch", `Host lacks capability ${required}`);
        }
    }
    if (options.contractDigest && capability.contract_digest !== options.contractDigest) {
        throw new EchoAgentError("extension_capability_mismatch", "contract digest mismatch");
    }
    if (options.sourceContractDigest && capability.source_contract_digest !== options.sourceContractDigest) {
        throw new EchoAgentError("extension_capability_mismatch", "source contract digest mismatch");
    }
}
function cancelledOutcome() {
    return {
        outcome: "error",
        error: { code: "cancelled", message: "extension invocation was cancelled", retryable: "never" },
    };
}
function normalizeOutcome(value) {
    if (typeof value !== "object" || value === null || !("outcome" in value)) {
        return {
            outcome: "error",
            error: { code: "invalid_value", message: "extension handler must return an outcome", retryable: "never" },
        };
    }
    const candidate = value;
    if (candidate.outcome === "result" && "result" in candidate) {
        return candidate;
    }
    if (candidate.outcome === "stream" && isWireHandle(candidate.stream)) {
        return candidate;
    }
    if (candidate.outcome === "error" && typeof candidate.error === "object" && candidate.error !== null) {
        return candidate;
    }
    return {
        outcome: "error",
        error: { code: "invalid_value", message: "extension handler returned a malformed outcome", retryable: "never" },
    };
}
function componentForOperation(operation) {
    if (operation.startsWith("conversation_"))
        return "conversation_store";
    if (operation.startsWith("run_"))
        return "run_store";
    if (operation.startsWith("runtime_"))
        return "runtime_state_store";
    if (operation.startsWith("audit_"))
        return "audit_logger";
    if (operation === "context_project")
        return "context_projector";
    if (operation === "memory_trigger")
        return "memory_trigger_sink";
    if (operation === "guard_check")
        return "guard";
    if (operation === "search_provider_search")
        return "search_provider";
    if (operation.startsWith("workflow_checkpoint_"))
        return "workflow_checkpoint_store";
    if (operation.startsWith("revisioned_task_"))
        return "revisioned_task_store";
    if (operation.startsWith("sandbox_"))
        return "sandbox_executor";
    if (operation.startsWith("mcp_transport_"))
        return "mcp_transport";
    if (operation === "embedder_embed")
        return "embedder";
    if (operation === "memory_promoter_promote")
        return "memory_promoter";
    if (operation === "workflow_run")
        return "workflow";
    if (operation === "workflow_run_stream")
        return "workflow";
    if (operation === "intent_classify")
        return "intent_classifier";
    if (operation === "skill_load_allows")
        return "skill_load_policy";
    return undefined;
}
const AGENT_COMPONENT_UNIT_INPUTS = new Set([
    "workflow_checkpoint_list",
    "workflow_checkpoint_clear",
    "sandbox_is_available",
    "sandbox_cleanup",
    "mcp_transport_close",
    "mcp_transport_try_notification",
]);
const AGENT_COMPONENT_INPUT_FIELDS = {
    conversation_create: ["conversation"],
    conversation_get: ["conversation_id"],
    conversation_list: ["user_id", "agent_type", "limit", "offset"],
    conversation_update: ["conversation_id", "title", "summary", "compressed_before_id"],
    conversation_delete: ["conversation_id"],
    conversation_save_messages: ["conversation_id", "messages"],
    conversation_get_messages: ["conversation_id"],
    conversation_count_messages: ["conversation_id"],
    conversation_ensure: ["conversation"],
    conversation_search: ["query", "limit"],
    run_save: ["run"],
    run_load: ["run_id"],
    run_list_by_session: ["session_id"],
    run_list_all: ["limit"],
    run_append_event: ["run_id", "event"],
    run_list_by_parent: ["parent_run_id"],
    runtime_get_checkpoint: ["conversation_id"],
    runtime_save_checkpoint: ["checkpoint"],
    runtime_save_checkpoint_for_scope: ["scope_id", "checkpoint"],
    runtime_state_ids: ["scope_id"],
    runtime_clear_state: ["scope_id", "runtime_state_id"],
    runtime_clear_scope: ["scope_id"],
    runtime_clear_conversation: ["conversation_id"],
    audit_log: ["event"],
    audit_query: ["session_id", "agent_name", "from", "to", "limit"],
    context_project: ["iteration", "agent_name", "session_id", "conversation_id", "run_id", "turn_id"],
    memory_trigger: ["trigger"],
    guard_check: ["content", "direction"],
    search_provider_search: ["query", "max_results"],
    workflow_checkpoint_save: ["checkpoint"],
    workflow_checkpoint_save_if_generation: ["checkpoint", "expected_generation"],
    workflow_checkpoint_load: ["checkpoint_id"],
    workflow_checkpoint_claim: ["checkpoint_id"],
    workflow_checkpoint_ack_claim: ["checkpoint_id", "attempt_id"],
    workflow_checkpoint_requeue_claim: ["checkpoint_id", "attempt_id"],
    workflow_checkpoint_renew_claim: ["checkpoint_id", "attempt_id"],
    workflow_checkpoint_list: [],
    workflow_checkpoint_list_by_graph: ["graph_name"],
    workflow_checkpoint_list_filtered: ["filter"],
    workflow_checkpoint_delete: ["checkpoint_id"],
    workflow_checkpoint_clear: [],
    revisioned_task_load: ["scope_id"],
    revisioned_task_compare_and_commit: ["scope_id", "commit"],
    revisioned_task_compare_and_commit_execution: ["scope_id", "commit"],
    sandbox_is_available: [],
    sandbox_execute: ["command"],
    sandbox_execute_stream: ["command"],
    sandbox_execute_with_limits: ["command", "limits"],
    sandbox_execute_with_limits_and_cancel: ["command", "limits"],
    sandbox_cleanup: [],
    mcp_transport_send: ["request"],
    mcp_transport_notify: ["notification"],
    mcp_transport_close: [],
    mcp_transport_try_notification: [],
    embedder_embed: ["text"],
    memory_promoter_promote: ["evicted"],
    workflow_run: ["input"],
    workflow_run_stream: ["input"],
    intent_classify: ["user_input", "context"],
    skill_load_allows: ["descriptor"],
};
function hasNoUnknownFields(value, allowed) {
    const accepted = new Set(allowed);
    return Object.keys(value).every((field) => accepted.has(field));
}
function optionalText(value) {
    return value === undefined || value === null || typeof value === "string";
}
function requiredText(input, field) {
    return typeof input[field] === "string";
}
function optionalU64(value) {
    return value === undefined || value === null || isCanonicalU64Text(value);
}
function validateAgentComponentDescriptor(descriptor) {
    const capabilities = descriptor.capabilities;
    if (capabilities?.conversation_projection && (capabilities.conversation_projection !== "unsupported" && descriptor.component !== "conversation_store" || !["unsupported", "atomic_v1"].includes(capabilities.conversation_projection)))
        throw new EchoAgentError("invalid_value", "conversation projection capability is invalid for component");
    if (capabilities?.runtime_state && (capabilities.runtime_state !== "unsupported" && descriptor.component !== "runtime_state_store" || !["unsupported", "revisioned_v1"].includes(capabilities.runtime_state)))
        throw new EchoAgentError("invalid_value", "runtime state capability is invalid for component");
    if (capabilities?.persistence_call && (capabilities.persistence_call !== "unsupported" && !["conversation_store", "runtime_state_store"].includes(descriptor.component) || !["unsupported", "absolute_deadline_v1"].includes(capabilities.persistence_call)))
        throw new EchoAgentError("invalid_value", "persistence capability is invalid for component");
    if (capabilities && "execution_cas_v1" in capabilities && typeof capabilities.execution_cas_v1 !== "boolean")
        throw new EchoAgentError("invalid_value", "execution CAS capability must be boolean");
    if (capabilities?.execution_cas_v1 && descriptor.component !== "revisioned_task_store")
        throw new EchoAgentError("invalid_value", "execution CAS capability only belongs to revisioned task store");
    const heartbeat = capabilities?.claim_heartbeat_interval_ms;
    if (descriptor.component === "workflow_checkpoint_store") {
        if (!isCanonicalU64Text(heartbeat)) {
            throw new EchoAgentError("invalid_value", "workflow checkpoint stores require a canonical claim heartbeat");
        }
        const value = BigInt(heartbeat);
        if (value < 1n || value > 300000n) {
            throw new EchoAgentError("invalid_value", "claim heartbeat must be from 1 to 300000 ms");
        }
    }
    else if (heartbeat !== undefined && heartbeat !== null) {
        throw new EchoAgentError("invalid_value", "claim heartbeat is only valid for workflow checkpoint stores");
    }
}
function validateAgentComponentCall(component, call) {
    const operation = typeof call.operation === "string" ? call.operation : "";
    const input = call.input === undefined || call.input === null
        ? AGENT_COMPONENT_UNIT_INPUTS.has(operation) ? {} : call.input
        : call.input;
    if (componentForOperation(operation) !== component || !isRecord(input)) {
        throw new EchoAgentError("serialization_violation", "Agent component call kind/operation is invalid");
    }
    if (MANAGED_OPERATIONS[operation]) {
        try {
            validateManagedFields(MANAGED_OPERATIONS[operation][0], input);
            return;
        }
        catch (error) {
            throw new EchoAgentError("serialization_violation", String(error));
        }
    }
    const inputFields = AGENT_COMPONENT_INPUT_FIELDS[operation];
    if (inputFields === undefined || !hasNoUnknownFields(input, inputFields)) {
        throw new EchoAgentError("serialization_violation", `Agent component ${operation} has unknown input fields`);
    }
    const wire = (field) => isWireValue(input[field]);
    const text = (field) => requiredText(input, field);
    const valid = (() => {
        switch (operation) {
            case "conversation_create": return wire("conversation");
            case "conversation_get":
            case "conversation_delete":
            case "conversation_get_messages":
            case "conversation_count_messages":
            case "runtime_get_checkpoint":
            case "runtime_clear_conversation":
                return text("conversation_id");
            case "conversation_list":
                return optionalText(input.user_id) && optionalText(input.agent_type)
                    && optionalU64(input.limit) && optionalU64(input.offset);
            case "conversation_update":
                return text("conversation_id") && optionalText(input.title) && optionalText(input.summary)
                    && (input.compressed_before_id === undefined || input.compressed_before_id === null
                        || (typeof input.compressed_before_id === "string" && /^-?(0|[1-9]\d*)$/u.test(input.compressed_before_id)));
            case "conversation_save_messages":
                return text("conversation_id") && Array.isArray(input.messages) && input.messages.every(isWireValue);
            case "conversation_ensure": return wire("conversation");
            case "conversation_search": return text("query") && isCanonicalU64Text(input.limit);
            case "run_save": return wire("run");
            case "run_load": return text("run_id");
            case "run_list_by_session": return text("session_id");
            case "run_list_all": return isCanonicalU64Text(input.limit);
            case "run_append_event": return text("run_id") && wire("event");
            case "run_list_by_parent": return text("parent_run_id");
            case "runtime_save_checkpoint": return wire("checkpoint");
            case "runtime_save_checkpoint_for_scope": return text("scope_id") && wire("checkpoint");
            case "runtime_state_ids":
            case "runtime_clear_scope": return text("scope_id");
            case "runtime_clear_state": return text("scope_id") && text("runtime_state_id");
            case "audit_log": return wire("event");
            case "audit_query":
                return optionalText(input.session_id) && optionalText(input.agent_name)
                    && optionalText(input.from) && optionalText(input.to) && optionalU64(input.limit);
            case "context_project":
                return isCanonicalU64Text(input.iteration) && text("agent_name")
                    && optionalText(input.session_id) && optionalText(input.conversation_id)
                    && optionalText(input.run_id) && optionalText(input.turn_id);
            case "memory_trigger": return wire("trigger");
            case "guard_check":
                return text("content") && ["input", "output", "tool_input", "tool_output"].includes(String(input.direction));
            case "search_provider_search": return text("query") && isCanonicalU64Text(input.max_results);
            case "workflow_checkpoint_save": return wire("checkpoint");
            case "workflow_checkpoint_save_if_generation":
                return wire("checkpoint") && isCanonicalU64Text(input.expected_generation);
            case "workflow_checkpoint_load":
            case "workflow_checkpoint_claim":
            case "workflow_checkpoint_delete":
                return text("checkpoint_id");
            case "workflow_checkpoint_ack_claim":
            case "workflow_checkpoint_requeue_claim":
            case "workflow_checkpoint_renew_claim":
                return text("checkpoint_id") && text("attempt_id");
            case "workflow_checkpoint_list":
            case "workflow_checkpoint_clear":
            case "sandbox_is_available":
            case "sandbox_cleanup":
            case "mcp_transport_close":
            case "mcp_transport_try_notification": return Object.keys(input).length === 0;
            case "workflow_checkpoint_list_by_graph": return text("graph_name");
            case "workflow_checkpoint_list_filtered": return wire("filter");
            case "revisioned_task_load": return text("scope_id");
            case "revisioned_task_compare_and_commit":
            case "revisioned_task_compare_and_commit_execution":
                if (!text("scope_id"))
                    return false;
                validateManaged(operation.endsWith("_execution") ? "TaskExecutionCommit" : "TaskGraphCommit", input.commit);
                return true;
            case "sandbox_execute":
            case "sandbox_execute_stream": return wire("command");
            case "sandbox_execute_with_limits":
            case "sandbox_execute_with_limits_and_cancel":
                return wire("command") && wire("limits");
            case "mcp_transport_send": return wire("request");
            case "mcp_transport_notify": return wire("notification");
            case "embedder_embed": return text("text");
            case "memory_promoter_promote":
                return Array.isArray(input.evicted) && input.evicted.every(isRecord);
            case "workflow_run": return text("input");
            case "workflow_run_stream": return text("input");
            case "intent_classify":
                return text("user_input") && Array.isArray(input.context) && input.context.every(isRecord);
            case "skill_load_allows": return isRecord(input.descriptor);
            default: return false;
        }
    })();
    if (!valid) {
        throw new EchoAgentError("serialization_violation", `Agent component ${operation} input does not match its typed contract`);
    }
}
function validateAgentComponentResult(component, expected, result) {
    if (!isRecord(result) || result.operation !== expected || componentForOperation(expected) !== component) {
        throw new EchoAgentError("invalid_value", "Agent component result operation does not match the callback");
    }
    const value = result.value;
    const managed = MANAGED_OPERATIONS[expected];
    if (managed) {
        try {
            validateManagedFields({ [managed[1]]: managed[2] }, value);
            return;
        }
        catch (error) {
            throw new EchoAgentError("serialization_violation", String(error));
        }
    }
    const unit = new Set([
        "conversation_update", "conversation_delete", "conversation_save_messages", "run_save",
        "run_append_event",
        "runtime_save_checkpoint", "runtime_save_checkpoint_for_scope", "runtime_clear_conversation", "audit_log",
        "workflow_checkpoint_save", "workflow_checkpoint_delete", "workflow_checkpoint_clear",
        "workflow_checkpoint_ack_claim", "workflow_checkpoint_requeue_claim", "workflow_checkpoint_renew_claim",
        "sandbox_cleanup", "mcp_transport_notify", "mcp_transport_close",
    ]);
    if (unit.has(expected)) {
        if (value !== undefined)
            throw new EchoAgentError("invalid_value", `${expected} must return a unit result`);
        return;
    }
    if (!isRecord(value))
        throw new EchoAgentError("invalid_value", `${expected} must return a typed value object`);
    const resultFields = {
        conversation_create: ["conversation"], conversation_get: ["conversation"],
        conversation_list: ["conversations"], conversation_get_messages: ["messages"],
        conversation_count_messages: ["count"], run_load: ["run"],
        conversation_ensure: ["conversation"], conversation_search: ["conversations"],
        run_list_by_session: ["runs"], run_list_all: ["runs"],
        run_list_by_parent: ["runs"],
        runtime_get_checkpoint: ["checkpoint"], runtime_state_ids: ["state_ids"],
        runtime_clear_state: ["receipt"], runtime_clear_scope: ["receipt"],
        audit_query: ["events"], context_project: ["projections"], memory_trigger: ["disposition"],
        guard_check: ["result"], search_provider_search: ["results"],
        workflow_checkpoint_save_if_generation: ["committed"],
        workflow_checkpoint_load: ["checkpoint"], workflow_checkpoint_claim: ["checkpoint"],
        workflow_checkpoint_list: ["checkpoints"], workflow_checkpoint_list_by_graph: ["checkpoints"],
        workflow_checkpoint_list_filtered: ["checkpoints"], revisioned_task_load: ["graph"],
        revisioned_task_compare_and_commit: ["graph"], revisioned_task_compare_and_commit_execution: ["graph"], sandbox_is_available: ["available"],
        sandbox_execute: ["result"], sandbox_execute_with_limits: ["result"],
        sandbox_execute_with_limits_and_cancel: ["result"],
        mcp_transport_send: ["response"], mcp_transport_try_notification: ["notification"],
        embedder_embed: ["vector"],
        memory_promoter_promote: ["submitted", "promoted", "deduplicated"],
        workflow_run: ["output"],
        intent_classify: ["intent"],
        skill_load_allows: ["allowed"],
    };
    const expectedFields = resultFields[expected];
    if (expectedFields === undefined || !hasNoUnknownFields(value, expectedFields)) {
        throw new EchoAgentError("invalid_value", `${expected} result has unknown fields`);
    }
    const values = (field) => Array.isArray(value[field]) && value[field].every(isWireValue);
    const valid = (() => {
        switch (expected) {
            case "conversation_create": return isWireValue(value.conversation);
            case "conversation_get": return value.conversation === null || value.conversation === undefined || isWireValue(value.conversation);
            case "conversation_list": return values("conversations");
            case "conversation_get_messages": return values("messages");
            case "conversation_count_messages": return isCanonicalU64Text(value.count);
            case "conversation_ensure": return isWireValue(value.conversation);
            case "conversation_search": return values("conversations");
            case "run_load": return value.run === null || value.run === undefined || isWireValue(value.run);
            case "run_list_by_session":
            case "run_list_all":
            case "run_list_by_parent": return values("runs");
            case "runtime_get_checkpoint": return value.checkpoint === null || value.checkpoint === undefined || isWireValue(value.checkpoint);
            case "runtime_state_ids": return Array.isArray(value.state_ids) && value.state_ids.every((item) => typeof item === "string");
            case "runtime_clear_state":
            case "runtime_clear_scope": return isWireValue(value.receipt);
            case "audit_query": return values("events");
            case "context_project": return values("projections");
            case "memory_trigger": return value.disposition === "persist" || value.disposition === "captured";
            case "guard_check": return isWireValue(value.result);
            case "search_provider_search": return values("results");
            case "workflow_checkpoint_save_if_generation": return typeof value.committed === "boolean";
            case "workflow_checkpoint_load":
            case "workflow_checkpoint_claim":
                return value.checkpoint === null || value.checkpoint === undefined || isWireValue(value.checkpoint);
            case "workflow_checkpoint_list":
            case "workflow_checkpoint_list_by_graph":
            case "workflow_checkpoint_list_filtered":
                return values("checkpoints");
            case "revisioned_task_load":
                if (value.graph == null)
                    return true;
                validateManaged("RevisionedTaskGraph", value.graph);
                return true;
            case "revisioned_task_compare_and_commit":
            case "revisioned_task_compare_and_commit_execution":
                validateManaged("RevisionedTaskGraph", value.graph);
                return true;
            case "sandbox_is_available": return typeof value.available === "boolean";
            case "sandbox_execute":
            case "sandbox_execute_with_limits":
            case "sandbox_execute_with_limits_and_cancel": return isWireValue(value.result);
            case "mcp_transport_send": return isWireValue(value.response);
            case "mcp_transport_try_notification":
                return value.notification === null || value.notification === undefined || isWireValue(value.notification);
            case "embedder_embed":
                return Array.isArray(value.vector)
                    && value.vector.every((item) => typeof item === "number" && Number.isFinite(item));
            case "memory_promoter_promote":
                return isCanonicalU64Text(value.submitted)
                    && isCanonicalU64Text(value.promoted)
                    && isCanonicalU64Text(value.deduplicated);
            case "workflow_run": return isWireValue(value.output);
            case "intent_classify": return isWireValue(value.intent);
            case "skill_load_allows": return typeof value.allowed === "boolean";
            default: return false;
        }
    })();
    if (!valid)
        throw new EchoAgentError("invalid_value", `${expected} result does not match its typed contract`);
}
/**
 * Decode the Host-owned reverse invocation envelope for a typed extension.
 * This is deliberately only a wire/shape adapter; execution and settlement
 * remain in the Rust Host and the official ACP connection.
 */
export function decodeExtensionInvokeCall(value, expectedKind) {
    if (!isRecord(value))
        throw new EchoAgentError("serialization_violation", "extension invocation must be an object");
    const extension = value.extension;
    if (!isWireHandle(extension) || extension.kind !== "extension") {
        throw new EchoAgentError("serialization_violation", "extension invocation has an invalid extension handle");
    }
    const invocationId = value.invocation_id;
    if (typeof invocationId !== "string" || invocationId.trim().length === 0) {
        throw new EchoAgentError("serialization_violation", "extension invocation_id must be a non-empty string");
    }
    const deadline = value.deadline;
    if (!isRecord(deadline)
        || typeof deadline.seconds !== "string"
        || !/^\d+$/u.test(deadline.seconds)
        || (deadline.seconds.length > 1 && deadline.seconds.startsWith("0"))
        || typeof deadline.nanos !== "number"
        || !Number.isInteger(deadline.nanos)
        || deadline.nanos < 0
        || deadline.nanos >= 1_000_000_000) {
        throw new EchoAgentError("serialization_violation", "extension invocation deadline is malformed");
    }
    const invocation = value.invocation;
    if (!isRecord(invocation) || typeof invocation.operation !== "string" || !("input" in invocation)) {
        throw new EchoAgentError("serialization_violation", "extension invocation payload is malformed");
    }
    if (extensionKindForOperation(invocation.operation) !== expectedKind) {
        throw new EchoAgentError("invalid_value", `${invocation.operation} is not a ${expectedKind} extension operation`, "never", "_echo_agent/extension/invoke");
    }
    if (expectedKind === "critic") {
        const input = invocation.input;
        if (!isRecord(input)
            || typeof input.task !== "string"
            || typeof input.answer !== "string"
            || typeof input.context !== "string") {
            throw new EchoAgentError("serialization_violation", "critique invocation input must contain task, answer, and context strings");
        }
    }
    if (expectedKind === "context_compressor") {
        const input = invocation.input;
        if (!isRecord(input)
            || !Array.isArray(input.messages)
            || !input.messages.every(isRecord)
            || !isCanonicalU64Text(input.token_limit)
            || (input.current_query !== undefined && input.current_query !== null
                && typeof input.current_query !== "string")
            || (input.focus_instructions !== undefined && input.focus_instructions !== null
                && typeof input.focus_instructions !== "string")
            || !isRecord(input.tokenizer)
            || !isWireHandle(input.tokenizer.resource)
            || input.tokenizer.resource.kind !== "facade_resource"
            || typeof input.tokenizer.owner_session_id !== "string"
            || input.tokenizer.owner_session_id.length === 0) {
            throw new EchoAgentError("serialization_violation", "compressor invocation requires messages and canonical token_limit");
        }
    }
    if (expectedKind === "agent_component") {
        const input = invocation.input;
        const call = isRecord(input) ? input.call : undefined;
        if (!isRecord(input)
            || typeof input.component !== "string"
            || !isRecord(call)
            || typeof call.operation !== "string"
            || (!("input" in call) && !AGENT_COMPONENT_UNIT_INPUTS.has(call.operation))) {
            throw new EchoAgentError("serialization_violation", "agent component invocation requires component and an operation-discriminated call");
        }
        const normalizedCall = !("input" in call) && AGENT_COMPONENT_UNIT_INPUTS.has(call.operation)
            ? { ...call, input: {} }
            : call;
        validateAgentComponentCall(input.component, normalizedCall);
        if (normalizedCall !== call) {
            invocation.input = { ...input, call: normalizedCall };
        }
    }
    if (value.stream !== undefined && value.stream !== null
        && (!isWireHandle(value.stream) || value.stream.kind !== "stream")) {
        throw new EchoAgentError("serialization_violation", "extension stream handle is malformed");
    }
    return {
        context: value.context,
        deadline: { seconds: deadline.seconds, nanos: deadline.nanos },
        extension,
        invocation: invocation,
        invocation_id: invocationId,
        stream: value.stream,
    };
}
function typedExtensionHandler(expectedKind, handler) {
    return async (value, signal) => {
        const call = decodeExtensionInvokeCall(value, expectedKind);
        const result = await handler(call, signal);
        if (isExtensionOutcome(result) && result.outcome === "error") {
            validateExtensionError(result.error);
            return result;
        }
        if (expectedKind === "agent_component") {
            if (call.invocation.operation === "agent_component_call_stream") {
                if (!isExtensionOutcome(result)
                    || result.outcome !== "stream"
                    || !isWireHandle(result.stream)
                    || result.stream.kind !== "stream") {
                    throw new EchoAgentError("invalid_value", "streaming Agent component callback must return the Host-issued stream");
                }
                return normalizeTypedExtensionResult(call.invocation.operation, result);
            }
            const expected = call.invocation.input.call.operation;
            const payload = isExtensionOutcome(result)
                ? result.outcome === "result" ? result.result : undefined
                : result;
            if (!isRecord(payload)
                || payload.operation !== "agent_component_call"
                || !isRecord(payload.value)
                || !isRecord(payload.value.result)
                || payload.value.result.operation !== expected) {
                throw new EchoAgentError("invalid_value", "agent component result does not match the typed callback operation");
            }
            validateAgentComponentResult(call.invocation.input.component, expected, payload.value.result);
        }
        return normalizeTypedExtensionResult(call.invocation.operation, result);
    };
}
function normalizeTypedExtensionResult(operation, value) {
    if (isExtensionOutcome(value)) {
        if (value.outcome === "result") {
            if (!isExtensionResult(value.result) || value.result.operation !== operation) {
                throw new EchoAgentError("invalid_value", "typed extension result operation does not match invocation");
            }
        }
        else if (value.outcome === "stream") {
            if (!isWireHandle(value.stream) || value.stream.kind !== "stream") {
                throw new EchoAgentError("invalid_value", "typed extension stream outcome has an invalid handle");
            }
            if (!operation.endsWith("_stream")) {
                throw new EchoAgentError("invalid_value", "non-streaming extension operation returned a stream");
            }
        }
        return value;
    }
    if (!isExtensionResult(value) || value.operation !== operation || operation.endsWith("_stream")) {
        throw new EchoAgentError("invalid_value", "typed extension result operation does not match invocation");
    }
    return { outcome: "result", result: value };
}
function isExtensionOutcome(value) {
    return isRecord(value) && (value.outcome === "result" || value.outcome === "stream" || value.outcome === "error");
}
function isExtensionResult(value) {
    return isRecord(value) && typeof value.operation === "string" && "value" in value;
}
function extensionKindForOperation(operation) {
    if (operation === "tool_execute" || operation === "tool_execute_stream" || operation === "tool_validate_parameters") {
        return "tool";
    }
    if (operation === "llm_chat" || operation === "llm_chat_stream")
        return "llm_client";
    if (operation === "critic_critique")
        return "critic";
    if (operation === "compressor_compress")
        return "context_compressor";
    if (operation === "agent_component_call" || operation === "agent_component_call_stream") {
        return "agent_component";
    }
    if (operation.startsWith("store_"))
        return "store";
    return undefined;
}
function isRecord(value) {
    return typeof value === "object" && value !== null && !Array.isArray(value);
}
const EXTENSION_ERROR_CODES = new Set([
    "acp_protocol_mismatch",
    "extension_version_mismatch",
    "extension_digest_mismatch",
    "extension_capability_mismatch",
    "invalid_request",
    "invalid_config",
    "invalid_value",
    "feature_unavailable",
    "stale_handle",
    "closed_handle",
    "framework_error",
    "extension_rejected",
    "extension_failed",
    "extension_timeout",
    "extension_disconnected",
    "extension_conflict",
    "cancelled",
    "host_shutting_down",
    "host_exited",
    "event_gap",
    "replay_unavailable",
    "payload_too_large",
    "serialization_violation",
]);
function extensionFailure(error) {
    const candidate = EchoAgentError.fromUnknown(error, "_echo_agent/extension/invoke");
    if (EXTENSION_ERROR_CODES.has(candidate.code))
        return candidate;
    return new EchoAgentError("extension_failed", candidate.message, candidate.retryable, "_echo_agent/extension/invoke", candidate.details);
}
export function decodeReplayResponse(value, stream, requestedAfter) {
    if (!isRecord(value) || !Array.isArray(value.events) || !isRecord(value.next_cursor)
        || !isCanonicalU64Text(value.requested_after_sequence) || !isCanonicalU64Text(value.next_cursor.last_processed_sequence)
        || value.next_cursor.stream_id !== stream.id || (requestedAfter !== undefined && value.requested_after_sequence !== requestedAfter)) {
        throw new EchoAgentError("serialization_violation", "replay response cursor/envelope is malformed");
    }
    const gap = value.gap == null ? undefined : decodeGap({ stream, gap: value.gap });
    let previous = BigInt(gap?.gap.snapshot_watermark ?? value.requested_after_sequence);
    if (gap && BigInt(gap.gap.from_sequence) !== BigInt(value.requested_after_sequence) + 1n)
        throw new EchoAgentError("serialization_violation", "replay gap does not follow requested cursor");
    const events = value.events.map(envelope => {
        const event = decodeEvent({ stream, envelope });
        const sequence = BigInt(event.envelope.sequence);
        if (sequence !== previous + 1n)
            throw new EchoAgentError("serialization_violation", "replay event sequence is not contiguous");
        previous = sequence;
        return event;
    });
    if (previous !== BigInt(value.next_cursor.last_processed_sequence))
        throw new EchoAgentError("serialization_violation", "replay cursor differs from returned events/snapshot");
    return { requested_after_sequence: value.requested_after_sequence, events, next_cursor: { stream_id: stream.id, last_processed_sequence: value.next_cursor.last_processed_sequence }, gap };
}
export function decodeAgentCallback(value) {
    if (!isRecord(value) || !isWireHandle(value.extension) || value.extension.kind !== "extension" || typeof value.invocation_id !== "string" || !isRecord(value.invocation) || !isRecord(value.invocation.input))
        throw new EchoAgentError("serialization_violation", "callback invocation malformed");
    const { operation, input } = value.invocation;
    const fields = { callback_on_think_start: ["agent", "messages"], callback_on_think_end: ["agent", "steps", "prompt_tokens", "completion_tokens"], callback_on_tool_start: ["agent", "call_id", "tool", "args"], callback_on_tool_end: ["agent", "call_id", "tool", "result"], callback_on_tool_error: ["agent", "call_id", "tool", "error"], callback_on_tool_interrupted: ["agent", "call_id", "tool", "input", "error"], callback_on_final_answer: ["agent", "answer"], callback_on_iteration: ["agent", "iteration"] };
    const allowed = fields[String(operation)];
    if (!allowed || !hasNoUnknownFields(input, allowed) || typeof input.agent !== "string")
        throw new EchoAgentError("serialization_violation", "callback operation/input malformed");
    const valid = operation === "callback_on_think_start" ? Array.isArray(input.messages)
        : operation === "callback_on_think_end" ? Array.isArray(input.steps) && isCanonicalU64Text(input.prompt_tokens) && isCanonicalU64Text(input.completion_tokens)
            : operation === "callback_on_tool_start" ? typeof input.tool === "string" && optionalText(input.call_id) && isWireValue(input.args)
                : operation === "callback_on_tool_end" ? typeof input.tool === "string" && optionalText(input.call_id) && typeof input.result === "string"
                    : operation === "callback_on_tool_error" ? typeof input.tool === "string" && optionalText(input.call_id) && typeof input.error === "string"
                        : operation === "callback_on_tool_interrupted" ? typeof input.tool === "string" && typeof input.call_id === "string" && input.call_id.length > 0 && isWireValue(input.input) && typeof input.error === "string"
                            : operation === "callback_on_final_answer" ? typeof input.answer === "string"
                                : isCanonicalU64Text(input.iteration);
    if (!valid)
        throw new EchoAgentError("serialization_violation", "callback input violates typed contract");
    return value;
}
function validateExtensionError(value) {
    const boundedText = (candidate, max) => typeof candidate === "string" && Array.from(candidate).length <= max;
    if (!isRecord(value) || !EXTENSION_ERROR_CODES.has(String(value.code)) || typeof value.code !== "string"
        || !boundedText(value.message, 4096) || !["never", "always", "after_delay"].includes(String(value.retryable)) || typeof value.retryable !== "string"
        || value.operation != null && !boundedText(value.operation, 256) || value.handle != null && !isWireHandle(value.handle)) {
        throw new EchoAgentError("serialization_violation", "extension error does not match its complete typed contract");
    }
    if (value.details != null) {
        const details = value.details;
        if (!isRecord(details))
            throw new EchoAgentError("serialization_violation", "extension error details must retain an object");
        if (details.fields != null && (!Array.isArray(details.fields) || details.fields.length > 16 || !details.fields.every(field => isRecord(field) && boundedText(field.key, 128) && boundedText(field.value, 2048))))
            throw new EchoAgentError("serialization_violation", "extension error detail fields are malformed");
        if (details.facade != null) {
            if (!isRecord(details.facade))
                throw new EchoAgentError("serialization_violation", "extension facade failure must retain an object");
            const facade = details.facade;
            const limits = { operation: 1024, receiver_kind: 64, expected_type: 256, required_feature: 128, resource: 256 };
            for (const [key, max] of Object.entries(limits))
                if (facade[key] != null && (!boundedText(facade[key], max) || !facade[key].trim()))
                    throw new EchoAgentError("serialization_violation", "extension facade detail identity is malformed");
            if (facade.signature_digest != null && (typeof facade.signature_digest !== "string" || !/^sha256:[0-9a-f]{64}$/iu.test(facade.signature_digest)))
                throw new EchoAgentError("serialization_violation", "extension facade signature digest is malformed");
        }
    }
}
//# sourceMappingURL=client.js.map