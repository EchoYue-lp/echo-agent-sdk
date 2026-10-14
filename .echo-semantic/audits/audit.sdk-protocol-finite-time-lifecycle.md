---
schema_version: 1
id: audit.sdk-protocol-finite-time-lifecycle
kind: audit
boundary_ref: boundary.sdk-acp
lens: time_lifecycle
freshness: examined
revision: 622d7931e01556161441d22de0697f6f35f24f17
finding_refs:
- finding.sdk-negotiation-response-validation
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
---

# Protocol time_lifecycle 有限审查

## 审查范围

SDKcapability原实际8lens source审与后shape/initowner具体原independentreview范围回收，real622d source/107packet可恢复；原eightcore/wire unchanged与3newvalidator不同epoch不混。作者不新businessreview/所有types承诺。

## 已检查故障假设

标准stdio4与客户端原cancel/EOF/closeretryscope支持；newinitializationError catch复用原settlementhelper，held与ignoreTERM二actualchildfail→EOF+realexit成功，原initerror为primarycleanupcause，不physicalTaskB。

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
