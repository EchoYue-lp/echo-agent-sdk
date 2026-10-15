# ADR 0035: Channel Handler Forward Consumption through the Original Entry

- Date: 2026-10-13
- Scope: SDK Channel forward input, typed consumption and framework ownership boundary.

## Status

approved

The user approved retaining the three accepted MessageHandler obligations and the six-operation forward consumption scope. Implementation and independently reviewed native three-language consumers are recorded in the [Channel behavior evidence](https://github.com/EchoYue-lp/echo-agent-sdk/blob/0ebe15f66d0aa57c7b7a92ae68cd1be106114ae9/docs/sdk/governance/evidence/channel-handler-forward-native/README.md). That evidence retains its actual earlier Host/source cut; it is not a claim that the current candidate passed all final gates. Complete generated contracts and exact-main delivery remain pending. This ADR does not accept every new Rust wrapper identity or close unrelated Findings.

## Context

The accepted ChannelMessageHandler reverse bridge has no production forward consumer. Language ChannelPlugin.start receives a handler name, while the framework retains the actual handler. Registration validates names but batch start can bind another extension generation. A textual selector cannot establish ownership. The accepted handle/handle_stream/reply behaviors require a real input path and stream consumption lifecycle.

## Alternatives

1. Retire the three obligations. The user did not select this option.
2. Clone handlers into another SDK registry and route by handler_id. This loses the original entry's lifecycle and allows stale generation calls.
3. Extend the existing framework owned ChannelHandle with invocation scope, then use the existing SDK family request, typed DTOs, HandleRegistry and FacadeStreamRuntime.

## Decision

Choose option 3. Use `_echo_agent/channels/op` and precisely six closed channels-family operations: handler.handle, handler.handle_stream, handler.reply and stream.next/cancel/close. The three controls are required to consume the approved stream operation; they are not another handler API. All operations and typed input/result shapes are emitted by the single production generator table and contribute to the accepted signature digest.

Session and complete manager/stream handles identify owner/generation. Forward calls select the original entry by channel_id and use only its successfully started current handler generation. The message channel_id must match. Batch start validates every original plugin-to-handler full identity before any start side effect. No arbitrary handler selector or default handler is accepted.

Reuse the framework ChannelEntry lifecycle, ChannelSession/SessionStreamReceipt ownership patterns and cancellation. SDK manages only type conversion, resource admission and transport. Stream close, Channel stop and resource/Session close signal cancellation before waiting on the read/write permits held by admitted work. Real producer Join/receipt and callback lease settlement permit resource release; waiter cancellation never fabricates external Run terminal. Failed cleanup retains the same retry owner.

## Exact accepted source choices

The following sections retain the existing accepted canonical method identities. They are not acceptance deltas and do not require `--accept-source` when the existing seed already contains them. The defining source paths in rustdoc must continue to resolve to the same canonical identity and kind. New ChannelHandle wrapper methods and internal scope/receipt identities remain Rust/Host-only unless a separate exact approved disposition is recorded after fresh source inspection.

### Retain handle

- Action: accept
- Kind: method
- Source: echo_agent::channels::MessageHandler::handle
- Reason: retain the accepted handler obligation and provide its real Channel forward consumer
- Compatibility: channels.handler.handle invokes the original started entry and returns the existing typed outbound message
- Rollback: restore the framework pin and complete SDK accepted contract together without silently retiring this obligation

### Retain handle stream

- Action: accept
- Kind: method
- Source: echo_agent::channels::MessageHandler::handle_stream
- Reason: retain the accepted native async stream obligation and its actual consumption lifecycle
- Compatibility: channels.handler.handle_stream returns an owner-fenced channels Stream consumed by channels.stream.next/cancel/close
- Rollback: restore the framework pin and complete SDK accepted contract together while retaining explicit open consumer debt

### Retain reply

- Action: accept
- Kind: method
- Source: echo_agent::channels::MessageHandler::reply
- Reason: retain the accepted reply obligation through the original bound handler
- Compatibility: channels.handler.reply awaits typed outbound delivery and returns null success or the original typed failure
- Rollback: restore the framework pin and complete SDK accepted contract together without substituting plugin.send

## Industry evidence and tradeoffs

[Tokio shared-state](https://tokio.rs/tokio/tutorial/shared-state) supports short registry locks and retained object ownership for asynchronous operations. [Tokio RwLock](https://docs.rs/tokio/latest/tokio/sync/struct.RwLock.html) documents deadlock when a held read, queued write and subsequent read depend on one another. Metadata callbacks therefore use short lookup paths, while lifecycle close first cancels the original stream/invocation.

[MCP cancellation](https://modelcontextprotocol.io/specification/2025-06-18/basic/utilities/cancellation) and [Tokio graceful shutdown](https://tokio.rs/tokio/topics/shutdown) distinguish requesting cancellation from completing shutdown. The SDK reuses its actual producer collector and framework receipts instead of inventing another business lifecycle. These choices are reusable framework mechanisms with a thin SDK protocol adapter, without EKO product policies.

## Consequences and verification

The design is [Channel handler forward](../supreme/specs/2026-10-13T1800-channel-handler-forward/design.md). Typed maps reject unknown/malformed fields, preserve Unicode/base64 and canonical u64 without defaults, and declare optional missing/null semantics. No new transport method or manual generated JSON is permitted.

Implementation must update the six operation schemas/digests, protocol/Host/client code, fixtures, examples and formal SDK documentation together. Three native language consumers must demonstrate single result, stream items/terminal and reply. Internal tests cover exact binding, Session isolation, stale generation, failed start, quota/setup/publication rollback, slow next, cancellation, close retry, actual producer settlement and metadata reentry. Applicable repository gates, independent review, published framework exact pin and SDK exact-main evidence remain required.

The framework milestones were delivered independently; the current SDK pin is published framework a634. The generic invocation slice retains framework ADR 0085 as its design history. Earlier owned-Channel evidence proves only its recorded source cut and does not substitute for current-generation or later runtime repair validation. Website synchronization is checked; an unchanged branding/install site may be documented as inapplicable, while API examples must remain executable.

## Callback completion and shared handler admission

ChannelMessageHandler uses the existing shared callback lease and concurrency quota, matching the framework's `MessageHandler: Send + Sync` methods on `&self`. It does not acquire a registration-wide exclusive key: cancelling invocation A must permit independent invocation B through the same original handler while A finishes its asynchronous cleanup. ChannelPlugin and the other existing exclusive kinds retain their existing policy; a handler synchronizes its own implementation as required by the original trait contract.

The Channel-specific transport adapter retains the original lease in the official ACP response-consumption task. Cancellation and deadline notices signal the original callback; they do not finish its invocation or release the framework Channel entry receipt. A streaming callback's terminal notification carries content, while its original RPC return records callback completion. The adapter consumes both concurrently and ends only after both facts, preventing callback-finally cleanup from being bypassed and avoiding waiting for the response before consuming a bounded stream. This proves the registered callback's return only; user-detached external jobs are outside that receipt.
