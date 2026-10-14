---
schema_version: 1
id: evidence.sdk-real-permission-default-ttl
kind: evidence
observed_at: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
source_refs:
- docs/sdk/governance/evidence/sdk-real-permission-ttl/run_typescript.mjs
- docs/sdk/governance/evidence/sdk-real-permission-ttl/summary.json
- docs/sdk/governance/evidence/sdk-real-permission-ttl/snapshot-provenance.json
- docs/sdk/governance/evidence/sdk-real-permission-ttl/run_python.py
- docs/sdk/governance/evidence/sdk-real-permission-ttl/SHA256SUMS.json
- docs/sdk/governance/evidence/sdk-real-permission-ttl/PermissionTtl.java
- docs/sdk/governance/evidence/sdk-real-permission-ttl/scenario.json
- docs/sdk/governance/evidence/sdk-real-permission-ttl/source-provenance.json
- docs/sdk/governance/evidence/sdk-real-permission-ttl/java.cp
- docs/sdk/governance/evidence/sdk-real-permission-ttl/host-source-build-manifest.json
- docs/sdk/governance/evidence/sdk-real-permission-ttl/proof-scope.md
- docs/sdk/governance/evidence/sdk-real-permission-ttl/read-only-client-source-hashes.json
- docs/sdk/governance/evidence/sdk-real-permission-ttl/runner-source-sha256.json
- docs/sdk/governance/evidence/sdk-real-permission-ttl/runner-launch.json
- docs/sdk/governance/evidence/sdk-real-permission-ttl/initial-live-processes.txt
- docs/sdk/governance/evidence/sdk-real-permission-ttl/final-process-observation.txt
- docs/sdk/governance/evidence/sdk-real-permission-ttl/observed-framework-source/approval_cache.rs
- docs/sdk/governance/evidence/sdk-real-permission-ttl/python/host.json
- docs/sdk/governance/evidence/sdk-real-permission-ttl/python/runner.log
- docs/sdk/governance/evidence/sdk-real-permission-ttl/python/status.json
- docs/sdk/governance/evidence/sdk-real-permission-ttl/typescript/host.json
- docs/sdk/governance/evidence/sdk-real-permission-ttl/typescript/runner.log
- docs/sdk/governance/evidence/sdk-real-permission-ttl/typescript/status.json
- docs/sdk/governance/evidence/sdk-real-permission-ttl/java/host.json
- docs/sdk/governance/evidence/sdk-real-permission-ttl/java/runner.log
- docs/sdk/governance/evidence/sdk-real-permission-ttl/java/runner-first-compile-error.log
- docs/sdk/governance/evidence/sdk-real-permission-ttl/java/status.json
- docs/sdk/governance/evidence/sdk-real-permission-ttl/python/state/.runtime_state.lease
- docs/sdk/governance/evidence/sdk-real-permission-ttl/python/state/host/session-index.json
- docs/sdk/governance/evidence/sdk-real-permission-ttl/python/state/host/generation
- docs/sdk/governance/evidence/sdk-real-permission-ttl/typescript/state/.runtime_state.lease
- docs/sdk/governance/evidence/sdk-real-permission-ttl/typescript/state/host/session-index.json
- docs/sdk/governance/evidence/sdk-real-permission-ttl/typescript/state/host/generation
- docs/sdk/governance/evidence/sdk-real-permission-ttl/java/state/.runtime_state.lease
- docs/sdk/governance/evidence/sdk-real-permission-ttl/java/state/host/session-index.json
- docs/sdk/governance/evidence/sdk-real-permission-ttl/java/state/host/generation
- docs/sdk/governance/evidence/sdk-real-permission-ttl/observed-client-source/sdks/shared/facade-operation-catalog.json
- docs/sdk/governance/evidence/sdk-real-permission-ttl/observed-client-source/sdks/shared/contract-digests.json
- docs/sdk/governance/evidence/sdk-real-permission-ttl/observed-client-source/sdks/python/src/echo_agent_sdk/catalog.py
- docs/sdk/governance/evidence/sdk-real-permission-ttl/observed-client-source/sdks/python/src/echo_agent_sdk/client.py
- docs/sdk/governance/evidence/sdk-real-permission-ttl/observed-client-source/sdks/python/src/echo_agent_sdk/wire.py
- docs/sdk/governance/evidence/sdk-real-permission-ttl/observed-client-source/sdks/typescript/dist/client.js
- docs/sdk/governance/evidence/sdk-real-permission-ttl/observed-client-source/sdks/typescript/dist/catalog.js
- docs/sdk/governance/evidence/sdk-real-permission-ttl/observed-client-source/sdks/typescript/dist/index.js
- docs/sdk/governance/evidence/sdk-real-permission-ttl/observed-client-source/sdks/typescript/dist/wire.js
- docs/sdk/governance/evidence/sdk-real-permission-ttl/observed-host-source/echo-sdk-host/src/factory.rs
- docs/sdk/governance/evidence/sdk-real-permission-ttl/observed-host-source/echo-sdk-host/src/core_profile/handles.rs
- docs/sdk/governance/evidence/sdk-real-permission-ttl/observed-host-source/echo-sdk-host/src/core_profile/facade/permission.rs
- docs/sdk/governance/evidence/sdk-real-permission-ttl/README.md
- docs/sdk/governance/evidence/sdk-real-permission-ttl/archive-manifest.json
supports:
- behavior.sdk-family-permission
limitations:
- VolatileSession/cache TTL only, no crossprocessdurability
- TSnumericexitnull preserved, finalPID absence not fabricatedexit0
- OriginalfirstJavaprivategettercompilefailure notbehaviorred
- No new acceptedTTLsetter/approvalreceipt contract
- NewgovernanceGit archivesoldrun, not futurepin proof
---

# real-permission-default-ttl

## 支持的结论

三原生公开Session.invoke各26真实calls、原Instant/default1800000ms不改clock/短setter，monotonic均>=1801500ms。四原审批到期false，读stats仍2global+2tool不会renew/delete；explicitcleanup4→0、fresh1true，6原PID实际消失。sdkCapabilityReader实际原47raw/source/clock/fields独立PASS0C0I0M；50formal仅48originalbytes+README/archive两元文，原47checksum b3ee4798不覆盖，originalruntime真实569/17cf/bb4f/Gitb909，不newdocGit重新30min。

## 来源与范围

真实治理观察Git`9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d`，runtime来源若涉及工具仍569/17cf/bb4f/Gitb909原cut；不是newGit日期执行。

## 已知缺口

VolatileSession/cache TTL only, no crossprocessdurability；TSnumericexitnull preserved, finalPID absence not fabricatedexit0；OriginalfirstJavaprivategettercompilefailure notbehaviorred；No new acceptedTTLsetter/approvalreceipt contract；NewgovernanceGit archivesoldrun, not futurepin proof。
