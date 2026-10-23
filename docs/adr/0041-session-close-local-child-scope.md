# ADR 0041: Session close disposes its local child subscriptions

- Date: 2026-10-23
- Status: Accepted for the local lifecycle repair; release verification remains separate.

## Context

The existing Host `session/close` settles its owned Runs, releases their handles
and removes their live delivery routes. An ACK-paused route can be removed
without a final live notification. TypeScript and Java retained a local Run
feed waiting for that notification; `RunHandle.close()` could also fail with
`closed_handle` before disposing the local feed. Java could create another
open publisher on the first subscription after close. Python already marked
known children closed, but a late `start_run` response and cancellation of a
close waiter could escape that local cleanup.

The Host remains the sole authority for Run settlement and terminal facts.
This repair belongs to the language clients' resource scope, not the framework,
Protocol, or a second Host lifecycle authority. The current Python
`SessionHandle._runs` / `RunHandle._mark_closed` and each client's existing
mailbox close operations supply the mechanism.

## Decision

Track Runs actually returned by a Session in that Session's local scope. Use
local object identity: multiple idempotent start replies can alias one
wire Run and every returned local handle still needs the parent disposal signal.
Only a successful `session/close` reply disposes that scope and its existing child
subscriptions. A successful start response delivered after that close is
immediately disposed locally. A failed parent close keeps its children usable
for retry. Concurrent close observers share one original close operation;
cancelling a Java/Python observer does not cancel that operation.

Disposal preserves buffered facts and then ends the local iterator/publisher.
It does not synthesize an event, receipt, `Completed`, `Cancelled`, or business
success. Later TypeScript/Java child subscriptions see the same ended local
mailbox; Python's existing closed-handle guard continues to reject a new child
subscription. No client may reopen the released Host Run or replay its removed
delivery route. Existing closed-handle query codes are preserved per language.

Automatic ACK observation must not hold a disposed iterator open. Already
submitted ACK and child-close RPC operations keep their original completion
owner; only the local iterator/close observer stops waiting after disposal.
No disposal signal advances the acknowledged cursor or claims consumption.

A child close can release its local subscription after an authoritative
`closed_handle`, `stale_handle`, or `host_exited` result. A transient
`host_shutting_down` result retains the typed failure and local scope for retry.
Timeout and other failed settlement also remain retryable, without claiming
that physical work ended.

## Alternatives and consequences

Adding a new Host terminal notification, permitting post-close replay, or
introducing `run/close` would change the accepted wire contract; none is needed
to dispose these local subscriptions. Ignoring all close errors would erase
unsettled ownership. Cancelling the original close/ACK RPC with its observer
would reproduce lost cleanup ownership. A separate global child registry would
duplicate the existing Session and feed ownership.

This repair adds no accepted member, wire route, generated contract artifact,
framework pin, or dependency. Unit regressions cover pending and first
subscriptions, late start, retry, concurrent/cancelled observers, buffered facts
and ACK waits. Real-Host and full language checks remain distinct evidence;
these tests do not prove recovery of terminal facts after the Host disposed its
delivery route. Existing quickstarts remain valid and in the language compile/
test chain. Website API pages require no new entry because no API is added.
