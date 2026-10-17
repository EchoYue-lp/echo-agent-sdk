---
schema_version: 1
id: audit.sdk-family-catalog-method-routing-rereview
kind: audit
boundary_ref: boundary.sdk-language-clients
lens: contract_evidence
freshness: examined
revision: 56929154c1851473e9e4837901b610cecf001b2d
finding_refs:
- finding.sdk-family-catalog-method-routing
challenges:
  catalog-exact-method-and-family-membership:
    revision: 56929154c1851473e9e4837901b610cecf001b2d
    source_refs:
    - sdks/typescript/src/client.ts
    - sdks/python/src/echo_agent_sdk/client.py
    - sdks/java/src/main/java/com/echoagent/sdk/EchoAgentClient.java
    - sdks/typescript/test/family-route-host.test.js
    - sdks/python/tests/test_family_route_host.py
    - sdks/java/src/test/java/com/echoagent/sdk/FamilyRouteHostTest.java
    - sdks/shared/facade-operation-catalog.json
    - docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/final-receipt.json
    - docs/sdk/governance/evidence/sdk-language-factory-final/receipt.json
    - docs/sdk/governance/evidence/sdk-language-factory-final/source-before.json
    - docs/sdk/governance/evidence/sdk-language-factory-final/source-after.json
    evidence_refs:
    - evidence.sdk-family-catalog-method-routing-repair
    - evidence.sdk-family-catalog-method-routing-verification
---

# Family catalog method routing independent rereview

## 审查范围

实际独立sdk_capability_acceptance_audit与协调者回收的limited pass0结论，绑定TS8a33/PY3aba/Javae559及三语言c52真实各一场景。审查来源为同goal中的独立复审消息，不伪称额外机器report；归档receipt标明这一来源。

## 已检查故障假设

family标识与wire拼写相同的错误假设；wrong-family绕过catalog；修复暗增wire/API；原始失败与Python共享state rootfixture错误混淆；旧全套green冒充新source通过。

## 实际实现路径与证据

catalog.resolve决定method/signature，三客户端只转换参数。content_guard实际content-guard路径、UTF8redact_exact与call/family可消费，wrong family在调用前拒绝，实际日志/XML保原输入。review结论只覆盖该局部缺陷。

## 问题记录

此范围独立复审未发现remainingfinding。repair/verification与rereview齐备，因此可本地resolved；能力模型仍needs_review。

## 残余风险

完整scope独立build、所有family异常、最终pin和mainCI仍独立待交付；不关闭外部Issue、不registrypublish。

## 未检查项

新后继codec源码、未列family/操作、全平台与最终线上仓库快照。

## 本次 current_source_rereview

当前SDK真实Git`56929154c1851473e9e4837901b610cecf001b2d`/source17cf，独立reader`sdk_capability_acceptance_audit`确实重新读取原业务故障假设、当前路径/变化owner/cfg/aliases和现14/newbb4f强场景，消息回收UTC`2026-10-14T00:39:15.355735+00:00`不是原运行时间。结果：**PASS / 0C0I0M（本次有界源码业务复审）**。原CodeAudit仍业务角色，不降为receiptAudit；原正文/历史sourcemarker`source:pending`/旧rawepoch保。

本次已检查：familycontent_guard catalogwirecontent-guard exactmethod/signature, wrongfamilyreject beforehostcall；Keepdeclaredscope no implicitnewwire;comparechangedclientcancel/Javaexit code without borrowing oldwholeSHA。当前函数定位：catalog.resolve；self.catalog.resolve；catalog.resolve。实际afterproof范围：Newbb4f FamilyRouteHostTest +TS/PYfamily-route publicUTF8/redact andsameadmission;sourcekind12fullgen remainspending。

上方追加source_refs逐569Gitblob=currentfileSHA精确，现14/newbb4f632+54 sourcecut实际范围匹配。原FaultHypothesis文本保留并在本次业务代码复核，源码变化闭包按原scope检查；statusexamined不等全无缺陷，supports/limits/41coverage/closure不提升。Framework业务源属于独立publishedGitb909，对native工具用既有pin/buildmanifest定位，不将framework文件伪置SDKtree。原历史整仓3ea/9e缺失仍未知，不把原run重新标为569。full28gen/三兼容人选/LinuxWindows/main仍分开。
