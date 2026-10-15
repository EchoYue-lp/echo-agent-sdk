---
title: Wait for the existing successor paused gap before duplicate ACK assertions
artifact: design
carrier: markdown
---

# Successor paused-gap receive barrier

Status: proposed independent design review. Plan: none. This is one test-only synchronization repair in the accepted event/ACK scenario. No protocol, Host, client runtime, shared fixture or shared tee is changed. Existing product B-policy and closed historical production fixes are outside this slice.

## Exact three caller paths and before hashes

- `sdks/typescript/test/gap-ack-host.test.js` SHA256 `b695a7ef9077e69f863e7fcac8f2e06ec95ce41df519be77f011fffe9499e0e0`
- `sdks/python/tests/test_gap_ack_host.py` SHA256 `79135f92f27f211393f1919b14b6273f3977e0c1ab208a4fad1b06a96910fe10`
- `sdks/java/src/test/java/com/echoagent/sdk/GapAckHostTest.java` SHA256 `432aff92b8f78a4608fe3ee75a9f7e69a6a7548ad6abe352bf2d1071ccb5f146`

Each caller actually has the same race: after ACKing the first gap, it waits only for any additional notification, optionally consumes one resumed item, captures notification count, sends the old ACK again, and requires the count not to change. The TypeScript and Java blocks are source-equivalent here, even though this Linux run failed only Python. All three are included for the same four parameter combinations (window/oversized × raw/consumer); Java has one parameterized method, not four new methods. Other tests and production source remain unchanged.

## Actual diagnosis

CI722 used Python3.12.3 and failed Python [True-window] with4!=3. True means consuming the live iterator, not a restart. The unchanged source passes one local focused attempt; that is not evidence against a race. The controlled original-copy probe preserves frame bytes/order and the original failed assertion. It temporarily holds the real successor gap at the transparent Admin tee until the duplicate old ACK is observed on stdin. Timeline proves that real Host gap4 was already received before that duplicateACK2. Baseline3 contains event1/gap2/event3; delivery of the preexisting gap4 increases it to4. This recreates the exact assertion failure without new Host behavior, synthetic events, sequence changes or a second authority.

Host DeliveryState::acknowledge runs resume_after under its send_serial owner. Its bounded journal lookahead publishes event3 and then pauses at gap4 for window1. The existing wait for notifications.length>before.length can finish upon event3 alone; consuming one resumed item also finishes upon event3. The second model request remains parked until the test later writes release. A prior ACK's in-flight successor gap is therefore a legitimate arrival, not a duplicate ACK re-send.

## Required minimal test change

Replace the weak any-new-notification barrier in these three caller blocks with a bounded wait on the actual raw transcript for a successor `_echo_agent/gap` on the original full stream handle (id, generation, kind). Its from_sequence and snapshot_watermark must both be strictly greater than the original acknowledged watermark. Use the existing waitFor/wait_for helpers and their existing deadline, not a new timer, sleep or fixed count4. The same-stream gap is the existing Host pause announcement; no model/Host test control is added.

Assert the original model parked marker is still present and release absent at this boundary. Keep the existing after-watermark check on every new raw event/gap, then capture the count only after observing that successor pause fact. Send the old duplicate ACK exactly as before, retain the run.get round trip, and retain the exact count equality. Keep the complete original successor gap object and compare its full stream/gap identity before and after the duplicate ACK, so an old ACK cannot replace/retract the new pause. Do not ACK the successor gap as part of synchronization.

All existing future-ACK rejection, replay values, iterator steps, snapshot-covered no-resend assertions, later release, wait, final outcome, continuation replay and cleanup remain. The barrier does not declare the new gap consumed; only the original explicit cursor is ACKed. No expectation or timeout budget is increased. The raw predicate must not discard unexpected extra frames: the retained count equality and all-new-frames watermark checks continue to expose duplicate/replayed delivery.

## Controlled before/after and validation

The original controlled failure and exact source/tee copies remain immutable. For the corrected controlled copy, the Admin scheduling gate releases the same original held gap when the test enters the new receive barrier (a diagnostic-only marker); it does not write or modify protocol frames. This avoids deadlocking a deliberate diagnostic hold against the stronger test. The new test must then see the genuine successor pause, record its full identity/count, send the old ACK, and pass the unchanged count equality with no extra notification. Timeline must identify the original gap, receipt before baseline, and duplicate ACK afterward. This is a scheduling counterexample, not a product transport implementation or a claim that the original CI used that gate.

After source approval and fresh preflight, run the existing four scenarios per language against immutable Host0f1 under Node20/UTC and Python3.12 (matching CI major/minor) or separately the supported Python3.10 as scheduled. TypeScript name-selection skips unrelated tests must be reported separately. Python exact node IDs avoid incidental collection. Java selects the original parameterized gapAckSnapshotContinuation method and executes its four rows; no new case or method is added. No Cargo/Host build is necessary. Root assigns the sole producer and Maven lease; this design preparation runs nothing.

Preserve three source before/after hashes, unchanged shared fixture/proxy/client/runtime hashes, all controlled old/new raw receipts, exact CI failure log, full timeline and stream frames, actual per-language selected counts, first failed/green epochs and independent narrow review. Official unfiltered whole-language/remote CI remains Root's separate gate. An implementation/evidence PASS does not certify all concurrency combinations or reopen/close product B-policy.
