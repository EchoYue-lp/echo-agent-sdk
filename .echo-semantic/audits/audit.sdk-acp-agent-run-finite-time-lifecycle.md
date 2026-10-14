---
schema_version: 1
id: audit.sdk-acp-agent-run-finite-time-lifecycle
kind: audit
boundary_ref: boundary.sdk-agent-session-run
lens: time_lifecycle
freshness: examined
revision: 8763c67bcfd6d98fe57449c88bb93f28f32bf78f
finding_refs:
- finding.sdk-session-run-idempotency-contract
challenges:
  original-acp-session-run-publication-obligation:
    revision: 8763c67bcfd6d98fe57449c88bb93f28f32bf78f
    source_refs:
    - echo-sdk-host/src/core_profile/handler.rs
    - echo-sdk-host/src/core_profile/handles.rs
    - echo-sdk-host/src/core_profile/state.rs
    - echo-sdk-host/src/core_profile/mod.rs
    - echo-sdk-host/src/factory.rs
    - echo-sdk-host/src/core_profile/persistence.rs
    - echo-sdk-host/src/lib.rs
    - echo-sdk-host/src/features.rs
    - echo-sdk-protocol/src/methods.rs
    - echo-sdk-protocol/src/capability.rs
    - echo-sdk-host/tests/core_profile_e2e.rs
    - echo-sdk-host/tests/stdio_e2e.rs
    - docs/adr/0037-session-run-idempotent-publication.md
    - docs/sdk/governance/session-run-idempotency-proposal.md
    - docs/sdk/governance/evidence/session-run-idempotency-implementation/independent-review.json
    - docs/sdk/governance/evidence/session-run-idempotency-implementation/manifest.json
    - docs/sdk/governance/evidence/session-run-idempotency-native/final-rpc-host/receipt.json
    - docs/sdk/governance/evidence/sdk-final-idempotency-rust-gates/combined-receipt.json
    - docs/sdk/governance/evidence/sdk-a634-rust-gates/05-workspace-tests.log
    - docs/sdk/governance/evidence/sdk-a634-rust-gates/10-ci-core-profile.log
    evidence_refs:
    - evidence.sdk-acp-agent-run-finite-observation
---

# ACP/AgentSessionRun time_lifecycle 有限审查

## 审查范围

frameworkFinal本session实际独立源审+必要caller补读，sourceac5/8b89/8e3/c766/da354固定真实8763。原193e/790、Steer诊断和新A da873/13inputs按实际epoch分开，不新全goal审。

## 已检查故障假设

Agentmutex串Sessioncreate/load与cascadeclose；Sessionclosing/cancel先fence、RWwrite等原ops、facade/frameworkclose/receiptpersist/remove全成功才tombstone，失败原ownerretry；RPCcancel前effectwithdraw后effect原job独计数保持。

## 实际实现路径与证据

preparedefinition/factory到frameworkSession.begin_turn/prepare_run/EventIdentity/RunEntry唯一authority，publicationmapping原owner保，正式A源码/真实RPCcancel/pub9/native3×5scope均具sourceRefs；原restore/badidentity/closedretention/durabledebt实际tests按原cuts保存。静态catalog不能造list/save/runclose主路径。

## 问题记录

有限scope审C0/I0/M0，新A具体intentgap具批准方案/修复/反例/verify/实际source独审，可局部闭合，不因whole12missing而假保持这个已经实现的specificgap。Java新wire缺陷独立，不能由Hostslice被审吞并。

## 残余风险

Akeyconnectiongeneration不可durable跨restart；B5physical/12/生成/平台/main尚待，cancelack非producerstop，短wait非cancel。role和methods按准确接受菜单不额外新API。

## 未检查项

未advertiseSessionlist/publicsave/Hostrunclose、未接受Rusthelper及全unlisted组合不纳本slice；任意remotephysical必须真实原authority证据。
