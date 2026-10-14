---
schema_version: 1
id: finding.sdk-subagent-await-result
kind: finding
type: implementation_bug
status: resolved
severity: high
primary_focus: failure_concurrency
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
- audit.sdk-task-memory-workflow-finite-time-lifecycle
- audit.sdk-task-memory-workflow-finite-failure-concurrency
decision_refs: []
repair_evidence_refs:
- evidence.sdk-task-memory-workflow-finite-observation
verification_evidence_refs:
- evidence.sdk-task-memory-workflow-finite-observation
rereview_audit_refs:
- audit.sdk-task-memory-workflow-finite-time-lifecycle
- audit.sdk-task-memory-workflow-finite-failure-concurrency
discovered_at: 30d7e219cc7d3aef0f75a19217c82898c363f50d
---

# await timeout 消费单次 JoinHandle 后永久丢失结果

## 触发条件与影响

当前源码路径：`echo-sdk-host/src/core_profile/facade/subagent.rs`。具体触发序列与审查记录见 `docs/sdk/governance/finding-ledger.md`。

## 证据

旧 pin 和 target 的 join均take单次slot；短timeout后重复await真实Host待执行。

本 Finding 保持 open；静态取证、代码修补或旧 pin focused 通过不能替代最终快照验证与独立复审。
## 问题

await timeout 消费单次 JoinHandle 后永久丢失结果。

## 处理记录

修复与最新直接验证进行中；尚未完成最终精确 snapshot 独立复审，保持 open。

## 原逻辑缺陷具体本地闭合

单Joincollector memo原resultreceipt，短timeout→重复fullawait同claim/attempt1/result/constraints不耗结果；非restart与全physical保证。 原postfixstrongassert及真实当前切片审/反例来源符合三链，具体root局部resolved，不把旧正文待回归当今天未修。physicalB、12、liveLLM替换/平台/source新版全gate/main分别不由该Bug改变。sharedEvidence包含原fix与verification双role，每Audit引用实际对应义务，不镜像新对象或强造API。
