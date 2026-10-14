---
title: Telemetry native test fixture cleanup after setup failures
artifact: design
carrier: markdown
---

# Telemetry native test fixture cleanup

Status: proposed for independent review. Plan: none; this is one bounded test-support repair, not a runtime or public-contract change. No SDK source is changed before design approval and fresh preflight. Root controls all native/Cargo/Maven execution leases.

## Actual failure and boundary

SDK CI eebd9f952649cca20d20eabab7f88ecdbaea5813 ran Node 20.20.2 and built a 2,959,264,016-byte Host. Its archived language-cancelled.log contains actual ERR_FS_FILE_TOO_LARGE failures in other tests. All five TypeScript Telemetry cases start Python --serve, then call readFileSync(host) for SHA before entering try/finally. This exact file is unchanged at 9a59943. A rejected test leaves its ChildProcess handles and the fixture's collector alive. CI cancellation lists five Python and five telemetry_composition children; this supports the setup-leak chain, not an assertion that OTLP shutdown or the last displayed team test hung. Telemetry failure TAP itself was not present in that cancelled log.

Python telemetry tests similarly hash the entire Host after starting --serve but before try; Java business telemetry does the same. A file-read/metadata exception in any language must not abandon the fixture owner, even when bounded SHA avoids the original large-file trigger.

## Owned source and reuse

Only these four existing files belong to this slice:

- sdks/typescript/test/telemetry-host.test.js
- sdks/python/tests/test_telemetry_host.py
- sdks/java/src/test/java/com/echoagent/sdk/TelemetryHostTest.java
- sdks/shared/testing/telemetry_host_fixture.py

Reuse ToolValue's sole helpers: synchronous fileSha256(path) from sdks/shared/testing/file-sha256.mjs, file_sha256(path) from sdks/shared/testing/file_sha256.py, and package-private FileSha256.sha256(Path). All retain exact SHA-256 bytes, bounded reads, closed descriptors and original errors. This slice adds no second hash helper, dependency, wire method, SDK product shutdown rule, Task state, or B-policy decision.

Existing stop and release-export files, actual original process handles, subprocess wait, and existing failed-test-only --cleanup-host mechanism remain the only fixture cleanup controls. No new stop RPC or substitute process is introduced. Helper factoring, if useful, stays private within these existing tests.

## Required lifecycle

Each successfully started --serve process immediately belongs to an encompassing try/finally or equivalent scoped owner, before metadata/hash/readiness/evidence operations. Ownership acquisition has no post-start exception window. Compute Host SHA before starting background resources where convenient, but do not rely on that ordering as the cleanup mechanism.

All cleanup obligations are attempted independently: release an actual held export, close the original SDK client if obtained, execute existing controlled Host cleanup when necessary on a failed test, publish stop, wait for the same fixture process, and save available evidence. Failed evidence writes or SDK close cannot skip stop/wait. Normal success retains SDK shutdown/close before fixture stop, so the collector remains available for genuine flushing. Cleanup never supplies EOF or a cancellation frame in place of public_close.

Preserve the primary test/setup error. If cleanup also fails, attach the cleanup errors as cause/suppressed errors (or an aggregate retaining the primary first), rather than replacing or silently discarding either. With no primary error, cleanup failure fails the test. Do not report physical settlement merely because a signal was sent: wait for the original fixture and collector PIDs to exit. Keep existing five-second cleanup waits/fallback; no larger test timeout or unconditional success is introduced.

## Python serve-only termination

If the existing fixture SIGTERM fallback is retained, install its cleanup-aware handler exclusively while executing --serve. SIGTERM must unwind this serve owner's original collector finally, terminate/wait the same collector if normal stop did not finish, and release its model HTTP server/thread. Save the prior signal handler and restore it when serve ends if the function returns. The handler does not write host stdin, alter proxy forwarding, fabricate host-exit.json, or make a successful business settlement claim.

Do not install this handler at module import or in --proxy-host. The proxy/public_close path keeps its original signal and EOF behavior byte-for-byte. Controlled cleanup is labelled failed-test cleanup, never accepted telemetry flush or Host graceful-close proof.

## Finite counterexamples and acceptance

Before evidence can reuse the old exact TypeScript setup sequence with a self-owned sparse 2,959,264,016-byte artifact: start the genuine existing --serve/collector, hit the actual readFileSync size error before try, observe both original PIDs alive, then explicitly clean up those same test-owned processes. This is a hash/setup-leak counterexample, not an executable Host or OTLP business E2E. It runs only after Root grants the native lease; no Cargo build is needed if collector provenance is qualified.

The persistent regression exercises the repaired actual scoped fixture owner with a controlled post-spawn metadata/hash read failure and a readiness failure. Assert the primary error remains visible, the original service/collector PIDs were identified, and both are dead after bounded cleanup. A failed evidence write or close callback must also leave the owner cleanup reachable; combine these into the same bounded fault case where feasible instead of a new test matrix. Verify serve SIGTERM fallback against the genuine collector, without applying any signal interception to proxy mode. Fault injection is confined to test-support callbacks or Admin probes, never production SDK APIs.

Then execute the existing real Telemetry cases without weakening assertions: TypeScript five cases, Python two console cases plus public_close, Java business console loop plus public_close. Preserve actual model/OTLP payloads, held ACK, original Host PID, exit code and public-close assertions. All test sources must be frozen before each producer runs. Finally Root runs the unfiltered official whole-language script/CI; focused tests do not substitute for it.

## Available artifacts and limits

Read-only candidate collector exists at /Users/ls/MyWork/code/ylp_agent_learn/lp-agent/.worktrees/echo-agent-sdk-worktrees/sdk-final-framework-pin/target/debug/deps/telemetry_composition-cbc07894d36a66c0, size 25,187,920 bytes, SHA-256 b8d5ef88b0451709e68dfd8eb657f3e4daa0a502ad2750d2c5ee70a9b0b7258a. Its depfile uses a relative tests/telemetry_composition.rs path, so the binary is not yet qualified as a published-00782 build merely from that pathname. Root must bind an actual build provenance or grant a fresh collector producer before native acceptance. Older immutable collector binaries exist and remain historical until matched to their source epoch.

The original eebd cancelled log SHA-256 is d4a85d86d1af20f0025d157e66dceed3b4893059c2ae67da2370f254f4dad2ca. Current/old TS telemetry test SHA e152c4b1950db19319977ba02bb58e0c7ecbff2f32a3b98517cc28b14a0f62c8; shared fixture SHA d4516646cb4ad760f99632ceb769f9182eb374a40f14267316f763fa95bf8bb9. No current CI is cancelled, filtered, or restarted by this slice.
