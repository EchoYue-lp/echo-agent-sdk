---
title: Paused consumer replay oracle under legitimate event progress
artifact: design
carrier: markdown
---

# Paused consumer replay oracle

Plan: none. Scope is exactly two existing tests, no SDK/Host/protocol/fixture/runtime change. Root already accepts this as a local test-contract correction, subject to independent source review and fresh preflight before writes.

## Exact allowed paths and before source

- `sdks/typescript/test/gap-ack-host.test.js` SHA256 `264683fd5c4a8f46617ad94e0ec1f6f65a4850d104ce477db4cdd4d0bad3bc47`
- `sdks/python/tests/test_gap_ack_host.py` SHA256 `0f95fa1c26cf0749a231a1e310d58b386fa55cb0e1810664b120a5ae3f2d1d3c`

Only the paused-consumer coalesced-gap window 1/2 blocks change. Java has no corresponding assertion and is excluded. Other tests/loops and the shared model fixture remain byte-for-byte unchanged.

## Actual evidence and cause

The complete official whole run failed TypeScript line202 with old Run.get last_sequence5 but a later replay next_cursor6. Source and 800-input freeze are recorded by Root. An unchanged source focused attempt passed once; that is not evidence against the race. phase1-3-summary.json and terminal-barrier-full-observations.json preserve a deterministic diagnostic: after original get returned running5, insert only the public same-Run wait-for-terminal call before replay; receipt and snapshot then report8, replay returns unique actual events3..8, and the untouched equality fails expected5/actual8. Original byte tee retained. No production observer, fake sequence or sleep was added.

The shared model's release file is published before the suspect get/replay pair. It has only one parked barrier; that release remains present, so the producer continues. Run.get samples the ledger at one RPC; replay later reads committed journal events at another RPC. Existing contracts do not freeze the journal at the older get. Equality between those independent observations is therefore unsupported. The exact monotonic bound is that this fixture's complete replay page includes at least the durable earlier snapshot prefix.

## Corrected oracle

Capture the page-start cursor before replay. For every returned event, assert its actual sequence equals the preceding cursor plus one, then advance that expected sequence. Preserve the existing assertion that the sequence was not already in covered, and preserve the exact event_id in covered. This checks every actual event; no filtering or deduplication is introduced. Require a nonempty page at this point and require next_cursor.last_processed_sequence exactly equals the last actual returned sequence.

Require that returned cursor is greater than or equal to the earlier get.last_sequence, not a hardcoded expected5/6/8. This lower bound is appropriate only because this finite fixture was observed to produce eight total events, requests64 events, has no retention action in this scenario, and no byte-limit truncation. The observed whole and controlled responses are below this cap. Keep existing final replay(0,64) exact equality of the entire covered event_id sequence and exactly one final_answer. After the existing public wait reports settled, read the same Run terminal snapshot and require the recovery cursor not exceed its actual last_sequence. The final page must start at sequence1 and be contiguous to that terminal tail, preserve the exact stream identity, contain unique event IDs, and have next_cursor and last actual envelope both equal the terminal tail. Keep the finite page within64 events, without hardcoding the observed tail8. Thus the lower-bound correction cannot accept an arbitrary future cursor or omit later terminal facts. Do not change ACK contents or require a new production snapshot/version API.

Keep the original coalesced queued gap from_sequence=consumed+1 and watermark advancement assertions, iterator return, public ACK, release, wait and owner cleanup. Keep every duplicate/full-multiset/final-answer assertion. No timeout increase, new sleep, fixed extra event, changed source state authority or widened replay coverage.

## Validation and source preservation

Before: original whole expected5/actual6 and controlled terminal-barrier expected5/actual8 remain immutable raw failure evidence. The controlled copy modifies only scheduling through an existing public call, absolute imports, and diagnostic retention/trace; it is not the product test source.

After: apply the same corrected strict oracle to the frozen diagnostic copy, preserving its public terminal barrier and full trace. It must pass while recording the same legitimate older-get/newer-replay relation. Then execute both native paused-consumer windows in TypeScript and Python (four actual cases) with the same Node20, Python3.10, UTC, immutable Host0f1 and original fixture/tee, saving source/hash/raw before and after. This is a bounded contract-oracle test, not a claim of whole-language or all concurrency validation. The official complete language script remains a separate Root gate.

Archive exact before/current two-source hashes, minimal patch, controlled raw before/after and original-node focused result, final four-case results, Host hash, full typed responses and raw tee, and independent scoped review. Preserve fixture/runtime unchanged hashes and all historical failures. Do not write S before this design PASS and current HEAD preflight check0.
