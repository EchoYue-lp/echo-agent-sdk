---
schema_version: 1
id: audit.sdk-main747-retirement-remainder-rereview
kind: audit
lens: contract_evidence
freshness: examined
revision: source:f3bbd9c3ae4a6e0c2132f00d1f39be88a0855eb1d8b00265936ddfb0295dfde7
source_refs:
- docs/adr/0042-sdk-a2a-lsp-retirement-boundary.md
- contracts/sdk/accepted-external-contract.json
- contracts/sdk/accepted-facade-operation-catalog.json
- echo-sdk-host/src/core_profile/handler.rs
- sdks/java/src/main/java/com/echoagent/sdk/RunHandle.java
- sdks/typescript/src/index.ts
- sdks/python/src/echo_agent_sdk/__init__.py
- sdks/typescript/test/jwt-values.test.js
- sdks/python/tests/test_jwt_values.py
- sdks/java/src/test/java/com/echoagent/sdk/A2AValueTest.java
- scripts/check-language-sdks.sh
finding_refs:
- finding.sdk-main747-retirement-remainder
boundary_ref: boundary.sdk-integrations
challenges:
  retired-public-api-not-dead-code:
    revision: source:f3bbd9c3ae4a6e0c2132f00d1f39be88a0855eb1d8b00265936ddfb0295dfde7
    source_refs:
    - sdks/typescript/src/index.ts
    - sdks/python/src/echo_agent_sdk/__init__.py
    - sdks/typescript/test/jwt-values.test.js
    - sdks/python/tests/test_jwt_values.py
    - sdks/java/src/test/java/com/echoagent/sdk/A2AValueTest.java
    evidence_refs:
    - evidence.sdk-main747-retained-retirement-boundary
    result: 确认main747四JWT公开构造和三fixture standalone真实存在且被有意移除，保留CHANGED；没有误称死代码或全API等价。
  finite-static-equivalence-only:
    revision: source:f3bbd9c3ae4a6e0c2132f00d1f39be88a0855eb1d8b00265936ddfb0295dfde7
    source_refs:
    - contracts/sdk/accepted-external-contract.json
    - contracts/sdk/accepted-facade-operation-catalog.json
    - echo-sdk-host/src/core_profile/handler.rs
    - sdks/java/src/main/java/com/echoagent/sdk/RunHandle.java
    evidence_refs:
    - evidence.sdk-main747-retained-retirement-boundary
    result: 原轻量checker独立自然0且stdout精同81714ea9；accepted source/alias/family另核0，两个原owner字节同，fixture调用按来源路径独立归类外部0。仅三有限静态matched。
  before-runtime-and-task-unknowns-preserved:
    revision: source:f3bbd9c3ae4a6e0c2132f00d1f39be88a0855eb1d8b00265936ddfb0295dfde7
    source_refs:
    - docs/adr/0042-sdk-a2a-lsp-retirement-boundary.md
    - contracts/sdk/accepted-external-contract.json
    - contracts/sdk/accepted-facade-operation-catalog.json
    - echo-sdk-host/src/core_profile/handler.rs
    - sdks/java/src/main/java/com/echoagent/sdk/RunHandle.java
    - sdks/typescript/src/index.ts
    - sdks/python/src/echo_agent_sdk/__init__.py
    - sdks/typescript/test/jwt-values.test.js
    - sdks/python/tests/test_jwt_values.py
    - sdks/java/src/test/java/com/echoagent/sdk/A2AValueTest.java
    - scripts/check-language-sdks.sh
    evidence_refs:
    - evidence.sdk-main747-retained-retirement-boundary
    result: before747动态、旧task:f5整体输入、外部动态消费者未知均保留；after完整矩阵不能填before。ADR0042具名unsupported退役与原c0bc有限场景合同支持此范围，不豁免真正保留动态义务。
---

# 当前七路径具名退役的独立复审记录

## 审查范围

Root登记实际独立review结果PASS C0/I0/M0，限本Finding和关联Evidence、实际窄静态checker、精确来源与具名授权。实现writer为Root，复审lane未写正式材料；其先前事实报告不作为自我背书的动态资格。

## 已检查故障假设

核查是否把仍公开的JWT构造误称死代码、是否以accepted为0偷换公开API无变化、是否用after绿色推before、是否丢失unknown、字面引用是否误按文本而非来源路径分类。实际检查未发现阻断。

## 实际实现路径与证据

原件位于主根sdk-semantic-maintenance-r01/current-seven-semantic-source-review.json（SHA25688d106acb08e37b5b4697151a8533a3538ee680a43e71139267c3ba99e859d57）及md（SHA2561ae48cfae5eb817bfb96c031ebc652f708a9c1772b55ac60356b900e061e76e2）。原ADR0042:193-195明确允许旧A2A/LSP unsupported；原c0bc artifact-contract:110-117的等价只覆盖检查场景。复审另执行原checker自然0，并额外核accepted source/alias/family及调用归类。

## 问题记录

当前正式两材料无Critical/Important/Minor。最初保持open是审查前状态；本次登记完成后，仅将本窄Finding置resolved，不更改旧Finding、Map或baseline closure。

## 残余风险

被退役公开构造和standalone能力明确CHANGED；before动态、f5整体输入和外部动态消费者未知继续保留。真正保留动态注册/反射/配置义务仍受原阻断合同约束。

## 未检查项

本审查不运行或替代canonical/preflight、本地重型矩阵、main CI、GUI/IM/真实provider或发布。只有后续原入口实际通过才可登记其结果。
