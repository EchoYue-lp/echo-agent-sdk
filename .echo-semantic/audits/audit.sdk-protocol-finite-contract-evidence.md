---
schema_version: 1
id: audit.sdk-protocol-finite-contract-evidence
kind: audit
boundary_ref: boundary.sdk-acp
lens: contract_evidence
freshness: examined
revision: 622d7931e01556161441d22de0697f6f35f24f17
finding_refs:
- finding.sdk-negotiation-response-validation
- finding.sdk-inventory-acceptance-authority
challenges:
  accepted-protocol-and-advertisement-shape:
    revision: 622d7931e01556161441d22de0697f6f35f24f17
    source_refs:
    - echo-sdk-protocol/src/capability.rs
    - echo-sdk-protocol/src/scalar.rs
    - echo-sdk-protocol/src/error.rs
    - echo-sdk-host/src/core_profile/wire.rs
    - echo-sdk-host/src/features.rs
    - sdks/typescript/src/wire.ts
    - sdks/python/src/echo_agent_sdk/wire.py
    - sdks/java/src/main/java/com/echoagent/sdk/WireValues.java
    - sdks/java/src/main/java/com/echoagent/sdk/EchoAgentClient.java
    - sdks/typescript/src/client.ts
    - sdks/python/src/echo_agent_sdk/client.py
    - docs/sdk/governance/evidence/negotiation-response-validation/README.md
    - docs/sdk/governance/evidence/negotiation-response-validation/manifest.json
    - docs/sdk/governance/evidence/negotiation-response-validation/independent-review.json
    - docs/sdk/governance/evidence/negotiation-response-validation/verification-summary.json
    - docs/sdk/governance/evidence/negotiation-response-validation/after/source-freeze.json
    - docs/sdk/governance/evidence/negotiation-response-validation/fixture-and-init-owner/source-freeze.json
    - docs/sdk/governance/evidence/negotiation-response-validation/fixture-and-init-owner/summary.json
    - docs/sdk/governance/evidence/negotiation-response-validation/before/java-six-faults-receipt.json
    - docs/sdk/governance/evidence/negotiation-response-validation/after/java-protocol-receipt.json
    - docs/sdk/governance/evidence/negotiation-response-validation/fixture-and-init-owner/typescript-receipts.json
    - docs/sdk/governance/evidence/negotiation-response-validation/fixture-and-init-owner/python-receipts.json
    - docs/sdk/governance/evidence/sdk-a634-rust-gates/receipt.json
    - docs/sdk/governance/evidence/sdk-a634-rust-gates/05-workspace-tests.log
    - echo-sdk-protocol/tests/core_rpc_contract.rs
    - echo-sdk-host/tests/stdio_e2e.rs
    - echo-sdk-host/tests/core_profile_e2e.rs#plain_client_and_mismatched_hello_fail_closed
    - echo-sdk-host/src/core_profile/mod.rs#negotiate_hello
    - echo-sdk-host/src/core_profile/handler.rs#require_extended
    - docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/java-surefire/TEST-com.echoagent.sdk.ProtocolIntegrityTest.xml
    evidence_refs:
    - evidence.sdk-protocol-finite-observation
  specific-accepted-source-authority:
    revision: 622d7931e01556161441d22de0697f6f35f24f17
    source_refs:
    - echo-sdk-protocol/src/inventory.rs
    - echo-sdk-protocol/src/bin/export_schema.rs
    - echo-sdk-protocol/tests/facade_inventory.rs
    - echo-sdk-protocol/src/inventory.rs#new_public_invoke_is_inventory_not_an_accepted_language_promise
    - echo-sdk-protocol/src/bin/export_schema.rs#new_source_inventory_route_does_not_change_accepted_catalog_or_digest
    - echo-sdk-protocol/src/bin/export_schema.rs#acceptance_dispositions_reject_draft_wrong_action_and_identity_substrings
    - echo-sdk-protocol/src/bin/export_schema.rs#accepted_catalog_ignores_reexport_alias_inventory_growth
    - docs/sdk/governance/evidence/sdk-final-idempotency-rust-gates/original/05-workspace-tests.log
    - docs/sdk/governance/evidence/sdk-final-idempotency-rust-gates/original/08-ci-default-protocol.log
    - docs/sdk/governance/evidence/sdk-final-idempotency-rust-gates/combined-receipt.json
    evidence_refs:
    - evidence.sdk-protocol-finite-observation
---

# Protocol contract_evidence 有限审查

## 审查范围

SDKcapability原实际8lens source审与后shape/initowner具体原independentreview范围回收，real622d source/107packet可恢复；原eightcore/wire unchanged与3newvalidator不同epoch不混。作者不新businessreview/所有types承诺。

## 已检查故障假设

specificnative7/9/localwirecorpus和真实Hostproxy6oldfaultJava与TSPY2shape/PYoverflowtyped、7afterOnly与Unicodeordercontrol严格分开；whole782preSource不是postRepairwhole，Windowsnative/fullproducer/main仍独立。

## 实际实现路径与证据

原hellodimension/Rustnamedplain/malformed/features/stdstdio4、nativeProtocol7/Integrity9/typedwirecorpus各守原scope；新合法Hostproxy actualbeforefault+aftershape/control/newinitnativeprocesscause同sharedEvidence/refs正式sourcefreeze/receipt。schema不变，不改变RunTaskauthority。

## 问题记录

原Important具体广告shape现repair/verify/actualindepchaincomplete，可Finding局部resolved，不借allfamilyPASS或旧whole782。TSinitcatch追加Bug具体stage同正式repair，不反推所有cleanup物理已完成；新callback/Hosttypedcause不由Protocol字段证明。

## 残余风险

Windowsnative路径、fullacceptedgenerator12source、5sphysicalB/platform/main待；pendingordinarynoSDKadvertisement标准ACP例外不变。老unknown全组合不作为新义务。

## 未检查项

未接受publicRustidentity/任意codec/remotephysicalproducer，不用总数或Catalogdone纳菜单；源版本新finalwhole须Root新freeze实际验。

## 原具名协议入口/消费断言直接定位

core_rpc_contract与stdio4、plain/mismatchedhello实测试、negotiate_hello/require_extended生产入口，以及原JavaProtocolIntegrityXML9均追加真实622d直接refs。原oldRust/三native命令时间为历史，622d源/receipt观察不把后继A新增coretests当原命令已运行；这只是该Protocol故障义务的可复核来源补齐，不要求新工程门禁或全fields矩阵。

## 自动接受库存身份原缺陷的真实源码复审

本次 sdk_capability_acceptance_audit 确实读取 inventory、export_schema、facade_inventory 的原接受权威故障与修复路径，有限源码复审 C0/I0/M0。三个修复源在真实85400与622d及本次当前逐字节相同。manifest_entries 的新 routable 身份默认 Deferred / sdk_scope=false；apply_accepted_source_scope 以既有 seed 的 defining source + ItemKind 精确匹配，不由 signature、alias、路由或数量推接受。非canonical/duplicate seed、missing/wrongkind、unknown add/invalid retire 都在 mutation 前明确拒绝。producer 在文件写入前 apply，catalog_documents 仅 canonical accepted，acceptance delta 要已批准的精确 ADR 与 full-update。

facade_inventory 八个具名真实 fixture 覆盖新可路由项不接受、alias/signature 漂移不改变既有 source/kind、missing atomic/精确退役、新 Task result 不接受兄弟字段、RustHost recovery authority 不暗升语言 API、seed 精确重现、invalid delta 原子拒绝、收集全部 missing 且 partial retire 不掩余项；inventory/new public invoke 与 export 三个反例断言也实际在正式05/08通过。没有原旧产品执行红日志就不写 executed RED；meaningful postfix 场景与本次真实源码独审支持原自动扩大身份的具体逻辑缺陷本地 resolved。

seed 未在此改动，完整28 profiles 的12 missing source 按原明确合同正确阻写，旧身份是否迁移/退役仍待用户。该正确阻断不是旧自动接受 bug 仍未修，也不是 full generator 已通过。库存量不成为能力完成分母，不自动接受 API、关闭全部模型或宣称 SDK main 完成。
本 challenge 是 SDKcap 本次新增实际 source review；不是从协商 PASS 或 generic catalog 完成标志借来的通过。原其它 Protocol challenge 保持。
