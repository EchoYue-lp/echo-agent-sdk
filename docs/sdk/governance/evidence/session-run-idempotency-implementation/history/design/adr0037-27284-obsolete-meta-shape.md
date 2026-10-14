# ADR 0037: Session and Run idempotent publication

## Status

**Accepted**. The user explicitly chose option A of [the bounded proposal](../sdk/governance/session-run-idempotency-proposal.md). The approved proposed-text SHA was `1c91a8c94bbadfc1cab2d5b33edd25c3ea3887ca2e1e259975fa1697726b073b`; its original bytes remain in the diagnostic history. This approval authorizes implementation, not a completed behavior claim.

## Context and alternatives

Both accepted requests validate idempotency_id but current handlers ignore it. The real Host counter records fresh Sessions and repeated completed Runs/provider calls for identical keyed requests. Choice A retains the parameters with bounded replay/conflict/closed-owner behavior. Choice B would explicitly reject keyed requests before effects and defer the accepted parameters. The user chose A. Existing Agent create, Stripe's idempotent-request contract and Google AIP-155 provide comparison; references and their direct-fetch provenance are in the proposal/evidence.

## Decision

Within one connection and handle generation, scope an exact key to operation and original Agent owner (Session create) or Session owner (Run start). Fingerprint the validated complete canonical payload. Same key/payload returns the original complete immutable response, concurrently or after completion. Different payload is existing invalid_request/never. Owner/resource closure yields existing closed_handle/never, with a retained tombstone preventing recreation. Local client RunHandle.close alone does not retire the Host Run. Connection/restart durable deduplication is not promised.

The existing Host Registry owns only publication records and admission reservations. Framework services still exclusively create/run/cancel/settle Sessions and Turns. One original publication job remains owned by the existing connection cleanup/settlement machinery; it never depends on an RPC waiter staying alive. Before effect admission, cancelled first requests release reservations and notify waiters. After effect admission, retain the original job until its actual result/cleanup and retain the immutable result. Duplicate waiter cancellation affects only its observation. No new Host wait timer/error enum or parallel runtime state machine is introduced.

Close/EOF fences admission first, waits original settlements, then discards publication mappings. An unfinished bounded close retains the key/cleanup owner and retry address. Budget exhaustion rejects before effects; replay/tombstone records are not silently evicted. The detailed A scope, failure contract and barrier acceptance criteria in the approved proposal govern implementation.

### Exact effect admission in existing owners

For Run start, the existing synchronous Session begin_turn executes inside the publication admission lock: cancellation/fencing is checked first and only successful original admission marks an effect. Busy rejection therefore releases the key rather than becoming a cached terminal failure.

For Session create, the framework SessionRegistry retains its original creation gate, capacity, duplicate-identity and shutdown checks. A private short-lived observer in the existing CoreProfileSessionFactory marks effect admission only when its original factory actually starts constructing the Agent. An internal random token associates that observer through ACP context metadata; it never contains the request key. The factory removes the token before create_agent, and a reservation guard removes unused associations on every exit. Weak CoreProfileState capture avoids a retained owner cycle. This observer supplies one admission fact, not another registry or validator. It adds no wire field or public callback contract and must not reach persistence, Agent configuration or client callback context.

## Consequences and verification

Lost-response retries can replay the first publication without a second factory/Turn/provider effect. Verification must observe real factory/provider effects, canonical full-response replay, changed-payload conflict, concurrent retries, closure, quota and first/duplicate waiter cancellation before/after effect admission, plus EOF settlement. The original four-case counter is historical before evidence; it does not certify the fix. Public documentation/examples/website applicability and final repository gates remain delivery obligations.

This changes behavior of existing accepted parameters, with no new wire fields, public protocol errors or language API. It does not decide the separate accepted-source migration or five-second physical settlement proposals. Reversion must explicitly retire/reject the parameters under a new product decision rather than silently restore ignoring them.
