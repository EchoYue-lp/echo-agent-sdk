---
schema_version: 1
id: finding.sdk-task-control-locality
kind: finding
type: authority_conflict
status: resolved
severity: high
primary_focus: state_authority
focus:
- time_lifecycle
- contract_evidence
- data_durability
boundary_ref: boundary.sdk-task-subagent
behavior_refs:
- behavior.sdk-task-subagent
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-task-memory-workflow-finite-observation
audit_refs:
- audit.sdk-task-memory-workflow-finite-state-authority
- audit.sdk-task-memory-workflow-finite-failure-concurrency
decision_refs: []
repair_evidence_refs:
- evidence.sdk-task-memory-workflow-finite-observation
verification_evidence_refs:
- evidence.sdk-task-memory-workflow-finite-observation
rereview_audit_refs:
- audit.sdk-task-memory-workflow-finite-state-authority
- audit.sdk-task-memory-workflow-finite-failure-concurrency
discovered_at: 30d7e219cc7d3aef0f75a19217c82898c363f50d
---

# 控制单 Task 实际取消/暂停整张 graph

## 触发条件与影响

当前源码路径：`echo-sdk-host/src/core_profile/facade/task_runtime.rs`。具体触发序列与审查记录见 `docs/sdk/governance/finding-ledger.md`。

## 证据

SDK根token与框架Cancelled/Paused全图推断两层反例；需要独立兄弟/后续wave验证。

本 Finding 保持 open；静态取证、代码修补或旧 pin focused 通过不能替代最终快照验证与独立复审。
## 问题

控制单 Task 实际取消/暂停整张 graph。

## 处理记录

修复与最新直接验证进行中；尚未完成最终精确 snapshot 独立复审，保持 open。

## 原逻辑缺陷具体本地闭合

原rootcancel全图误推改为exactattempt/controlclaim；pauseA/BRunning/Cpending→B/C完成canonicalB后resumeA2/D1，cancelA兄弟继续，3native4完整输出/次数；Bphysical独立。 原postfixstrongassert及真实当前切片审/反例来源符合三链，具体root局部resolved，不把旧正文待回归当今天未修。physicalB、12、liveLLM替换/平台/source新版全gate/main分别不由该Bug改变。sharedEvidence包含原fix与verification双role，每Audit引用实际对应义务，不镜像新对象或强造API。
