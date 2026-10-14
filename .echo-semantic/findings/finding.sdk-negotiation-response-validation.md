---
schema_version: 1
id: finding.sdk-negotiation-response-validation
kind: finding
type: implementation_bug
status: resolved
severity: high
primary_focus: failure_concurrency
focus:
- time_lifecycle
- contract_evidence
- data_durability
boundary_ref: boundary.sdk-language-clients
behavior_refs:
- behavior.sdk-language-clients
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-protocol-finite-observation
audit_refs:
- audit.sdk-protocol-finite-trigger-input
- audit.sdk-protocol-finite-contract-evidence
- audit.sdk-protocol-finite-time-lifecycle
decision_refs: []
repair_evidence_refs:
- evidence.sdk-protocol-finite-observation
verification_evidence_refs:
- evidence.sdk-protocol-finite-observation
rereview_audit_refs:
- audit.sdk-protocol-finite-trigger-input
- audit.sdk-protocol-finite-contract-evidence
- audit.sdk-protocol-finite-time-lifecycle
discovered_at: 30d7e219cc7d3aef0f75a19217c82898c363f50d
---

# 默认digest/extension version广告未完整验证

## 触发条件与影响

当前源码路径：`sdks/typescript/src/client.ts`。具体触发序列与审查记录见 `docs/sdk/governance/finding-ledger.md`。

## 证据

三语言协商受控Hostproxy测试已新增；目标Host完整结果待对账。

本 Finding 保持 open；静态取证、代码修补或旧 pin focused 通过不能替代最终快照验证与独立复审。
## 问题

默认digest/extension version广告未完整验证。

## 处理记录

修复与最新直接验证进行中；尚未完成最终精确 snapshot 独立复审，保持 open。

## 原广告验证及初始化失败owner局部闭合

真实622d三validator源码+107取证已提交，可恢复原Java4294967297误接受/arrayssilentdrop及TSPYlimits{}caps[]误接受，originalbefore6与七afterOnly严格分开；修复只existingSDK广告shape，BigIntegerexactversion、每membervalidated/canonicalpositiveLimits/fullcaps/Unicodecodepointsorting及Python officialinitSchemaError typed。原合法Hostproxybody与afterJ21/TSPY25 truecounts保；newTScatch leak twoactualchildren held/ignoreTERM真red→复用existingstdiohelper EOFcallback+actualexit/原5s/primaryinitcausecleanupcause，finalTS37/PYintegrity31 unchangedJ21/PY25各epoch，原body和genuinefailure/fixturecorrected/forcedcleanup区别不抹。

SDKcap actualshape+initowner source/材料独审PASS0加Protocol9materialPASS0补直接refs，specificBug三链足够localresolved不等SDKmain；newcurrentwhole787尚由Root执行，旧782whole历史不冒本after最新绿。没有release/registry/新languageAPI或schema字段，Windowsnative/12sourceidentity/B5physical/gen/platform/main另验。
