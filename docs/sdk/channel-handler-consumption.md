# Channel handlers and cooperative streams

The six operations below are the explicitly approved Channel consumption scope
([ADR 0035](../adr/0035-channel-handler-forward-consumption.md)). They reuse
existing accepted MessageHandler obligations and do not promote new framework
wrapper identities. Current Host pin a634 and each recorded consumer's actual
source cut remain separate from complete contract generation and SDK main
acceptance.

Channel plugins and handlers use the existing `_echo_agent/channels/op` family.
These calls target the original registered framework entry and its successfully
started handler. Register the plugin with its full handler extension handle;
batch start must use that same handle, including generation, for every plugin.
A name alone cannot replace the binding. Unstarted, starting, failed-start and
stopping entries reject input before invoking a callback.

| Canonical operation | Arguments | Result |
|---|---|---|
| `channels.handler.handle` | manager handle, channel ID, inbound message map | outbound message map |
| `channels.handler.handle_stream` | manager handle, channel ID, inbound message map | channels Stream handle |
| `channels.handler.reply` | manager handle, channel ID, outbound message map | null after reply finishes |
| `channels.stream.next` | Stream handle | `FacadeStreamEvent`: item, complete, failed or cancelled |
| `channels.stream.cancel` | Stream handle | bool indicating whether cancellation changed |
| `channels.stream.close` | Stream handle | bool after original producer settlement and address release |

Use the owning Session as the request receiver. Stream events retain their
stream handle and sequence; an item contains an outbound message map. Streaming
does not implicitly invoke reply. `channels.plugin.send` remains the separate
plugin transport operation. Wrong owners, generations and closed addresses
fail before callbacks; inactive handler input is `invalid_value`.

Inbound fields are `channel_id`, `sender_id`, `chat_id`, `chat_type`, `text`,
`message_id`, `timestamp` and `attachments`. Timestamp is a canonical unsigned
64-bit decimal string, including `18446744073709551615`; a JSON number, missing
value, null, leading zero or overflow is rejected. Outbound fields are
`channel_id`, `to`, `chat_type`, `text`, `reply_to` and `attachments`. The message
channel ID must match the selected entry. Chat type is direct/group. Attachments
use image/file/audio/video, standard base64 bytes and optional filename.
Unknown fields and malformed base64 are rejected rather than replaced by a
default. Missing attachments mean an empty list; explicit null is invalid.
Missing/null filename or reply_to means None, while an empty string is retained.
Encode ordinary business maps explicitly when an attachment's `kind` could be
mistaken for a WireValue discriminator; the executable consumers below show
that encoding in each language.

A registered MessageHandler callback owns its work through its awaited return.
For a streaming callback, publish chunks and one terminal through the supplied
writer, await producer cleanup/finally, then return the matching Stream outcome.
A terminal notification alone does not prove that the callback has returned.
Do not detach a producer and treat the callback's early return as its cleanup
receipt. The Host retains the original RPC response owner and framework driver
until that actual response arrives; cancellation of one call does not cancel a
sibling call or the handler generation.

CustomAgent streams also support their existing early setup ACK: the callback
may first return the Stream outcome, while its admitted producer continues.
The RPC ACK and the original producer's validated, contiguous terminal are two
separate facts. Both are required for that original owner to finish. A late ACK
after cancellation must not release the producer merely because the projected
caller result is cancelled. Missing-terminal errors and transport errors retain
their actual cause; local failure does not become an external Run receipt.

Channel, CustomAgent and registered Sandbox producers use cooperative cleanup.
A deadline while the original source is still live returns a retryable
`framework_error` with `retryable=after_delay`, retaining the same resource/stream
address and quota owner. Release or finish the original producer, then retry
close using the same handle. Successful close tombstones it; a later next fails
with `closed_handle`. A bad stream item retains its original error while cleanup
is pending, and returns a failed terminal after actual source settlement. Local
producer cleanup signals do not rewrite that failure as semantic cancellation.

Callbacks may await their manager's `channels.manager.ids` and `.list` while
active: metadata reads do not take the long operation permit. Resource or Session
closing returns `closed_handle`. A callback synchronously awaiting its own stop,
close or recursive mutation is outside this metadata reentry guarantee. Teardown
closes new admission while still accepting bounded, ordered notifications of an
already issued stream; original owner/generation and sequence checks remain.

Registered Sandbox execution uses canonical `sandbox.extension.run_stream` and
`sandbox.stream.next/cancel/close` in the shell family. Cancellation reaches the
original callback from setup onward. Close waits for the actual producer and
its cleanup, including process exit when the implementation owns a process;
fixture timeout cleanup is not proof that Host cancellation succeeded.

## Executable examples

The existing native consumer tests are maintained, executable examples of
registration, real forward calls, typed callbacks, stream writers and cleanup:

- [TypeScript](../../sdks/typescript/test/channel-handler-forward-host.test.js)
- [Python](../../sdks/python/tests/test_channel_handler_forward_host.py)
- [Java](../../sdks/java/src/test/java/com/echoagent/sdk/ChannelHandlerForwardHostTest.java)

They share [message and scenario fixtures](../../sdks/shared/testing/channel_forward_scenarios.json)
and invoke a real Host rather than fabricate its inbound consumer. With a
source-built, contract-matching Host, run the existing native entrypoints:

```sh
export ECHO_AGENT_SDK_HOST=/absolute/path/to/echo-agent-sdk-host
export ECHO_AGENT_SDK_REQUIRE_HOST_E2E=1
# Run each command from the corresponding language directory.
node --test test/channel-handler-forward-host.test.js
PYTHONPATH=src uv run --no-sync --group dev python -m pytest tests/test_channel_handler_forward_host.py
mvn -Dtest=ChannelHandlerForwardHostTest test
```

The repository's `scripts/check-language-sdks.sh` builds TypeScript examples,
lints Python examples, compiles the Java Example, runs all native tests and
executes all three existing quickstarts. Those quickstarts remain small general
Agent/Session examples; the Channel consumers above carry the real callback and
lifecycle examples so the same behavior is not duplicated into another runner.
These examples do not replace final published-framework pin and exact-snapshot
acceptance evidence.
