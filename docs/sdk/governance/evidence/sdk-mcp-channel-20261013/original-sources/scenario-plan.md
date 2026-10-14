# MCP/channel candidate behavior and current failure

All source changes for this lane remain under /tmp. No SDK/Framework production,
accepted catalog, permission rule or CI contract has been modified.

Observed real native TypeScript calls against immutable Hostcb9b:

1. Actual stdio MCP Python process fixture supports initialize, tools/resources/
   resource templates/prompts, binary and Unicode resource data, tool call and
   ping. Exact metadata and raw responses are checked; two same-name servers
   belong to separate Session managers. Disconnect A makes its actual PID dead
   and its derived alias closed while B remains alive/callable. Closing B kills
   B's PID. This is real MCP transport, not fabricated Host responses.
2. An accepted registered McpTransport component initializes through real Host
   reverse callbacks. First actual close returns explicit failure; alias and
   owner stay addressable, next close succeeds exactly once more and closes old
   alias. Four resource slots are then reusable. This proves bounded same-owner
   close retry, not every callback cancellation/shutdown combination.
3. Two registered ChannelPlugin callbacks start their own real subprocesses.
   Outbound image/file/audio/video attachment bytes, Unicode filenames and
   reply_to reach callback parameters exactly. Explicit business WireValue maps
   disambiguate attachment.kind from unknown protocol wrappers. stop_exact A
   closes A's process and leaves B running, Session teardown closes B.
4. The same channel case with first A stop failure proves A/B PIDs remain alive
   after failure; second stop closes only A, B continues health and then closes
   with the Session. No inbound ChannelMessageHandler call is fabricated.

These four complete native cases passed. MessageHandler reverse callbacks
remain unreachable from current accepted plugin start: the received handler id
is an opaque name with no client-to-Host inbound entry point. That independent
open capability Finding is not closed by plugin start/send/stop evidence.

Important additional counterexample:

ChannelHealth callback starts public channels.manager.ids for the same manager
and awaits it while the health callback is still in progress. Native request
cannot finish within the controlled callback gate; once callback returns,
manager.ids immediately completes. Source dispatch_channels holds the manager
Tokio mutex over channel.health_check().await, causing circular wait for a
callback that genuinely awaits the read. The test uses a bounded gate only to
observe the dependency and allow cleanup; it does not claim arbitrary timing
is the business requirement. Raw events show callback-enter, read-pending,
callback-return and read-after-outer-response in order. No inbound message,
new API, fake Store or terminal fact is introduced.

Reuse/layer finding:

Current framework ChannelManager stores Box<dyn ChannelPlugin> and exposes only
borrowed get/get_mut. No canonical cloneable channel reference or async owned
lease exists. Health/send methods are readonly on ChannelPlugin; start/stop are
mutable lifecycle operations. A SDK second plugin map, name-based re-creation,
or a shadow manager would create parallel identity/lifecycle authority and is
not an acceptable fix. A framework-owned exact instance/lease primitive may be
needed to release outer caller/manager locks before readonly reverse callback;
this is a product-independent mechanism. Start/stop cannot simply drop locks
without preserving existing lifecycle/handler bookkeeping and retry ownership.
The coordinator must choose and dispatch the framework boundary fix; this lane
has not silently changed framework public API or copied the manager.

Not yet proved: full three-language matching scenarios, transport close caller
cancellation/hold cleanup, inbound MessageHandler real consumer, callback read
reentry resolution, all MCP optional notifications/resources lifecycle, and
published framework pin/main/full gates.
