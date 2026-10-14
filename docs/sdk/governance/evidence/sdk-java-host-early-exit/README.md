# Java original-child exit and pending handshake settlement

Current scoped candidate: `closing-final-source-class-manifest.json`, 59 tests passed, zero failures/errors/skips. The late-final 57-case and initial 55-case manifests remain distinct historical epochs. The previous `epoch55-source-class-manifest.json` is a historical passing snapshot that does not cover late subscribers. No framework/Protocol/wire change, producer acceptance decision or broad language-suite completion is claimed.

## Genuine pre-fix facts

The independent LP-owned `EarlyExitProbe.java` uses the actual pinned official ACP Java transport/session. A real Python child exits with status 7 either before reading initialize or after receiving it; the probe records real PID, exit, stderr and an initialize future still pending two seconds after that exit. The public SDK exhibits the same pending initialization with a real exited child. `probe.log`, `observed.json`, source/class/jar hashes in `before-source-class-manifest.json` preserve this diagnosis. Reflection in that probe only reads diagnostic Process identity; production uses no reflection.

A separate real immutable Host rejects `sdk_profile.state_root="relative-state"`, exits 2, emits the explicit absolute-path error and writes no stdout (`actual-host-invalid-config.log`, `host-config-receipt.json`). That is a correct fixture configuration rejection. The client leaving initialize pending after the actual process exits is a separate lifecycle bug; the repair does not relax the Host path rule.

`real-child-before.log` and `before-TEST-HostEarlyExitTest.xml` show both public SDK early-exit cases actually failing with timeout, despite the child exiting. The official `acp-core-0.17.0.jar` was inspected directly: its stdio reader completes its private inbound sink on EOF; session pending requests are dismissed on session close, not on that sink completion. SDK spawn only awaited initialize with the official 30-second request timeout. There is no project `StdIoAcpSession.java`. Official GitHub source retrieval was unavailable in this environment; conclusions are based on the exact local official artifact's bytecode and the real process evidence, not a different API version.

## Existing-owner repair

A private nested `OwnedStdioTransport` extends the official transport. It starts exactly one off-thread wait on the existing public `awaitForExit()` only after `super.connect` actually launches the child. Official transport remains the sole Process/stdio owner; there is no replacement ProcessBuilder, polling loop, custom JSON-RPC transport or public API. An original failure promise keeps the first observed cause. Initialize and ordinary RPC waits race their official responses with that same exit failure, returning the existing explicit `transport_error` rather than waiting for an unrelated request deadline.

The spawn handshake observer owns the same client cleanup independently of a caller cancelling its returned wait. Caller cancellation closes that original client and cancels the original initialize submission; a late completed client is not leaked. Completed successful results are never overwritten after exit.

Expected exit is marked only when the actual session/transport close phase begins, after original factory cleanup prerequisites succeed. A cleanup failure while the child remains alive cannot suppress a later unexpected exit. If native transport close itself fails, expected-close state resets and an already observed original exit is delivered from the same retained first cause. This does not claim failed factory cleanup physically completed.

The constructor's original exit observer cancels and clears local transport cancellation addresses, fails existing event/update publishers using their existing failure path, then dismisses official pending RPC slots. After map insertion, new event/update publishers check that same completed exit promise and fail immediately. Consequently either the observer traversal sees a newly inserted publisher or its post-insertion check sees the terminal fact; no separate Run terminal or scheduler state is introduced. Normal successful client close retains the existing completed/EOF behavior.

## Current verification and preserved attempts

`java-lifecycle-focused-closing-final.log` is the final command, with seven copied `closing-final-TEST-*.xml` reports:

- HostEarlyExitTest: 9 cases. Two public real children exit before/during initialize; cancelling an unresponsive initialize wait ends that same PID; real original Process exit after an injected existing cleanup failure ends its pending request and clears official slots; real Process exit fails an idle subscriber and clears local cancellation addresses; post-exit newly created event/update subscribers immediately receive error; and publishers created while the same existing cleanup is held receive natural EOF when close completes.
- JavaClientCloseLifecycleTest: 3 existing cases for shared original transport close, failed close/retry and cancelled waiter ownership.
- LifecycleTest: 17 existing publisher/handle cases.
- AckSubmissionTest: 18 existing ACK recovery and submission cases.
- StreamCancellationRoutesTest: 4 existing routing-window/failed-close cases.
- StreamCancellationRouteHostTest: 1 genuine public Host setup-ACK cancellation case.
- ProtocolHostTest: 7 real Host negotiation, owner, retry and callback cases.

The first five early-exit tests produced a passing 55-case snapshot. Review then identified late publishers created after the exit observer had already traversed its maps. `late-subscribe-before.log` and its XML actually reproduce two post-exit subscriber timeouts on that snapshot; the final insertion/exit-fact checks resolve both, and all 57 cases pass in that epoch. An earlier implementation attempt failed idle-subscriber/pending-slot checks because a source edit had not inserted the intended constructor observer; `java-lifecycle-focused-reviewed.log` preserves its 53 passes/one failure/one error, without relabeling it as successful proof. Its corrected 55-case command and the final 57-case command have distinct source epochs.

The two composition exit tests use reflection to instantiate the private client with a real official child and inject an existing factory cleanup failure or inspect pending slots. They are local transport composition tests, not physical framework Task/Run settlement evidence. The public real-child and immutable real-Host suites provide separate native transport/normal-use proof. The selected immutable Host is `echo-sdk-b909-natural-eof-host`, SHA `8d81900442aaf97de4a4287a1cc9d11aae63b15f8f3b5a89b6811347806fdc10`, matching the current candidate catalog.

## Limits and delivery

Pure stdout EOF while the child remains alive has no public lifecycle callback in the pinned official transport; this scoped repair observes actual Process exit and launch failure. It does not claim to solve that separate EOF-with-live-child case, every outbound I/O error, remote physical cancellation or post-terminal user-detached cleanup. It records no raw production stderr that could contain credentials. The three-client stream cancellation and ACK behavior remain covered by their original targeted tests; TypeScript/Python production source was untouched here.

No Cargo, framework source modification, full contract generation, accepted-source migration, Git commit/push/PR or registry publication was performed. The twelve accepted source differences remain pending user decision. Examples and website APIs are unchanged; local lifecycle comments and this evidence document explain the bugfix. Full Java/SDK gates and exact-main delivery remain parent-owned and must use this final source snapshot.

## Final closing-admission window

The original close starts by traversing existing publishers, then waits for owned cleanup. Review identified that a publisher created during that wait was absent from the first traversal and only checked closed, not closing, so it could survive successful close without EOF. Two real-child controlled-cleanup tests reproduce these event/update timeouts in `closing-subscribe-before.log` and its XML. The final creation paths use the existing `closed || closing` admission condition; this creates no new lifecycle authority and does not clear cleanup debt or stream addresses. `java-lifecycle-focused-closing-final.log` and seven closing-final XML reports pass all 59 cases, zero errors/skips. The two new tests retain the original held cleanup, release it normally, await the actual same transport close and observe natural subscriber completion. The final manifest binds the exact production/test and compiled class hashes; previous 55/57 epochs are not treated as proof of this last window.
