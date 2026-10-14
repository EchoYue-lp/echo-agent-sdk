---
schema_version: 1
id: evidence.sdk-family-catalog-method-routing-repair
kind: evidence
observed_at: 56929154c1851473e9e4837901b610cecf001b2d
source_refs:
- sdks/typescript/src/client.ts
- sdks/python/src/echo_agent_sdk/client.py
- sdks/java/src/main/java/com/echoagent/sdk/EchoAgentClient.java
- sdks/typescript/test/family-route-host.test.js
- sdks/python/tests/test_family_route_host.py
- sdks/java/src/test/java/com/echoagent/sdk/FamilyRouteHostTest.java
- docs/sdk/governance/evidence/sdk-family-route-20261013/manifest.json
- docs/sdk/governance/evidence/sdk-family-route-20261013/independent-review-receipt.json
- sdks/shared/facade-operation-catalog.json
- docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/final-receipt.json
- docs/sdk/governance/evidence/sdk-language-factory-final/receipt.json
- docs/sdk/governance/evidence/sdk-language-factory-final/source-before.json
- docs/sdk/governance/evidence/sdk-language-factory-final/source-after.json
supports:
- behavior.sdk-family-content-guard
- behavior.sdk-language-clients
limitations:
- Local exact bug closure; final package/fullsource/pubpin/mainCI delivery separate.
- Original tested sourcehash remains in manifest; current observation does not assert
  later-source equivalence.
---

# Family catalog route修复

## 支持的结论

三语言family helper直接读取既有catalog.resolve的method，并校验family归属；不再从family字符串构造协议method。无新API、accepted范围或第二路由权威。

## 来源与范围

归档manifest及实际工具/独立review会话记录限定到TS8a33/Python3aba/Javae559与c52Host。本对象分别记录修复和反例验证，未杜撰新的测试运行；原始候选Evidence继续保留。

## 已知缺口

全部family的未列组合、最终公开frameworkpin/三语言完整门禁及精确main/CI交付另待，不阻止这个精确本地路由bug已修复结论。

## 本次已复审scope的当前证据重观察

当前观察Git`56929154c1851473e9e4837901b610cecf001b2d`仅据下面明确源码业务复审的已接受路径，回收UTC`2026-10-14T00:39:15.355735+00:00`；原marker`source:pending`、旧runtimeepoch/time/sourcehash/raw全文保留，原support/limits不变。不是过去执行就发生569，不假历史整仓Git恢复。

- `audit.sdk-family-catalog-method-routing-rereview` reader`sdk_capability_acceptance_audit`：**PASS / 0C0I0M（本次有界源码业务复审）**；本次原故障检查familycontent_guard catalogwirecontent-guard exactmethod/signature, wrongfamilyreject beforehostcall；Keepdeclaredscope no implicitnewwire;comparechangedclientcancel/Javaexit code without borrowing oldwholeSHA。after仅Newbb4f FamilyRouteHostTest +TS/PYfamily-route publicUTF8/redact andsameadmission;sourcekind12fullgen remainspending。

所有新SDKsource/proof refs按真实569byte/anchor读取，原业务与原sourcefreeze记录保持；这不是一切currentfamily/runtimepassed。ordinaryTools已有原五族收据只新增本次明确Database/Artifact检查，不暗称其余方言/provider/工具所有field均新审；原candidate supports=[]不升。任何缺原完整time/tree仍在原limits，12/B/depWarn人未答/fullgeneration/CImain待。
