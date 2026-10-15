---
schema_version: 1
id: finding.sdk-accepted-source-continuity
kind: finding
type: intent_gap
status: resolved
severity: high
primary_focus: contract_evidence
focus:
- state_authority
- contract_evidence
boundary_ref: boundary.sdk-product
behavior_refs:
- behavior.sdk-family-value
- behavior.sdk-family-invoke
- behavior.sdk-family-task
- behavior.sdk-repository-ownership
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-accepted-source-continuity-gate
audit_refs:
- audit.sdk-accepted-source-continuity-rereview
decision_refs: []
repair_evidence_refs:
- evidence.sdk-accepted-source-continuity-gate
verification_evidence_refs:
- evidence.sdk-accepted-source-continuity-gate
rereview_audit_refs:
- audit.sdk-accepted-source-continuity-rereview
discovered_at: 30d7e219cc7d3aef0f75a19217c82898c363f50d
---

# 明确accepted源/kind连续性阻写

## 问题

b909全28profile producer正确拒12旧canonical source/kind缺失：2obsoleteSnapshotownerfield、7Securitycanonical moved/rootalias仍保、3TaskContextpublicfield转getter。不能把根alias/无SDKconsumer/Catalogdone当自动retire或完整Rustinventoryparity目标。

## 触发条件与影响

完整update读取旧acceptedseed，membership由exactsource/kind唯一判断；同名getter与field不同key。正确exit1/noartifactswrite，不得旁路门禁默认丢旧承诺。

## 证据

原12closure保旧entry/signature/classification/route/28profiles/loghash。proposed文稿仅推荐一次有限兼容选择，尚未humanapproved；无新产品API/第二authority。

## 处理记录

等待根完整审阅+一次人的兼容裁决，协议owner唯一exactmigrationtable/ADR随后执行；本Finding保持open。既往sourcecontinuitymerge历史另对象，不借旧wave2closure关闭此次source/kind迁移。

## Task三字段证据界限

现methods.rs无完整TaskSubagentContext三字段专用reverseDTO。真实Hostrun_id()→loadsnapshot、waived_dependency_ids()→依赖skip提示、delegation_policy()→DispatchRequest；Task三语言四场景不能证明完整context值直接给CustomAgent。拟保这些原生语义现薄映射/getterRustHostonly，授权身份迁移后定向补现路径字段消费，不新增wire/API。2nonSerdeSnapshot Rustowner字段的旧wire_value身份仍需人明确退役决定，不能无consumer自动删。


## 2026-10-15 已批准的独立修复切片

用户已明确接受原提案的精确12项：7项Security defining source一对一迁至core
retention并保旧alias、operation、输出shape；3项TaskContext public field身份退役，
保现有Host三个只读getter消费且不新增语言API；2项obsolete snapshot owner身份退役，
保原Skill/Task/Plan唯一权威。正式授权复用ADR0034的精确Action/Kind/Source锚点。
旧CuratorState.save及TaskExecution/DiagnosticCode裁决保持。上述等待人选择的段落为
原发现切点事实，不再表示本次裁决未批准。

本次不删除任何源码文件或能力，不建立平行分类器；唯一generator完整28profiles处理
13 retire及12 accept（含此前批准的6项）。回滚点为a48854a的完整seed/生成物及其
匹配framework pin，按ADR整片恢复。Security路由回归已先取得真实失败再修复，
Task既有4项getter消费和三语言Security真实Host为直接验证；完成验证及独立复审前
本Finding继续open。其它cleanup/Sandbox/Task-resume/managed passive-exit义务不动。


## 独立上游批准状态补充

父任务后续已取得物理结算B及工具A/B的明确实现和交付授权，分别由framework与
工具独立writer实施，尚待实际交付。它们不改变本Finding的精确12项切片；原历史
未批准/失败段落保留原切点事实，不表示当前仍等待同一批准，亦不表示上游已闭合。


## 当前已执行验收

本切片完整28profiles update/check实际通过；最终Rust8门禁0/workspace478/0/0、
官方三语言同一入口0（TS343/PY427+18/J363、76freshXML、三个quickstart），793输入与
新实际Host哈希前后同，4getter和Security原生语义均有直接结果。原失败/中断各保。
最后全差异独立复审与公开22e CI型精候选的ordinary/原30dchange门禁是收口步骤，
不会通过删除原PF或手改JSON求绿。工具c0bc已独立交付，不在此切换SDK action。


## 本切片三链闭合

repair：ADR0034正式人类裁决、唯一28profiles完整更新、薄route/精确getter分类已实施。
verification：完整合同0、Rust8门禁0/workspace478、官方完整语言入口0/793同源新Host，
以及自然无PF的精CI候选5553全文件blobmode/content精确相同，公开22e ordinary strict与
原base30d7e219cc7d3aef0f75a19217c82898c363f50d加require-change-evidence均实际0。
rereview：audit.sdk-accepted-source-continuity-rereview独立PASS0；安装记录后只metadata
增量由主代理核对并再次执行同原base门禁。故仅本12项source/kind阻写Finding resolved。
安装0.5 task/repository-scope历史引用及8MB错误如实保留，未删原PF、未改工具求绿。
B/其它宽Finding/全SDK main及归档continuity不在闭合范围，不接受残余风险。
