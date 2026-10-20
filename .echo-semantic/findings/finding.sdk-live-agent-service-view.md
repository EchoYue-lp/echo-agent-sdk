---
schema_version: 1
id: finding.sdk-live-agent-service-view
kind: finding
type: authority_conflict
status: resolved
severity: high
primary_focus: state_authority
focus:
- time_lifecycle
- contract_evidence
- data_durability
boundary_ref: boundary.sdk-integrations
behavior_refs:
- behavior.sdk-integrations
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-task-memory-workflow-finite-observation
audit_refs:
- audit.sdk-task-memory-workflow-finite-state-authority
decision_refs: []
repair_evidence_refs:
- evidence.sdk-task-memory-workflow-finite-observation
verification_evidence_refs:
- evidence.sdk-task-memory-workflow-finite-observation
rereview_audit_refs:
- audit.sdk-task-memory-workflow-finite-state-authority
discovered_at: 30d7e219cc7d3aef0f75a19217c82898c363f50d
---

# set_store/set_llm_client后family继续使用旧captured authority

## 触发条件与影响

当前源码路径：`echo-sdk-host/src/core_profile/facade/memory.rs`。具体触发序列与审查记录见 `docs/sdk/governance/finding-ledger.md`。

## 证据

改live Agent读取；Store A→B/Workflow LLM回归待执行。

本 Finding 保持 open；静态取证、代码修补或旧 pin focused 通过不能替代最终快照验证与独立复审。
## 问题

set_store/set_llm_client后family继续使用旧captured authority。

## 处理记录

修复与最新直接验证进行中；尚未完成最终精确 snapshot 独立复审，保持 open。

## 已完成的具体本地闭合

本次补齐原 Finding 的两个具体 live authority 义务。Memory 原具名测试在同 Agent 的 Store A→B 后实际读写 B，错误 namespace 零写，旧值与计数强断言已执行。Workflow 新具名测试先由原 SessionAuthorityServices 捕获同一个 AgentHandle，再通过既有公开 set_llm_client 将 A 替换成 B，随后经真实 Host graph.build 与框架 AgentNode 执行。A 的真实 stream 请求数为 0，B 为 1；完整 ChatRequest 保留原 task 与 Unicode system/user，completed graph state 输出 B 的 Unicode 回答与原任务、唯一路径。该证据不是 pure router 的 callback0，也不是 getter 观察。

修复生产路径是原 live Agent 服务读取；workflow 新 d516 文件只增加 cfg(test)，生产 prefix 9ae6 与原修复源一致。b7e6 是该代码与正式包的真实可恢复 Git 观察，不表示早期 Memory 命令重新执行。原 30d captured client/config 源已冻结，作为旧缺陷源码证据，未造旧产品执行红。两次新 fixture 错误（缺合法 owner、读错 state JSON 路径）明确保留。focused 1、Host lib 187/0 ignored、严格 Clippy 和 fmt/check 实际通过；sdk_capability_acceptance_audit 已独立读源码、真实断言和正式包，PASS C0/I0/M0。

此处仅证明替换后新建 graph 使用当前 provider，不承诺已建 graph 中途热换，不新增 SDK setter、RPC 或跨进程持久化。Memory 与 Workflow 原具体 captured authority 缺陷三链足够本地 resolved；B 物理结算、12 身份处置、平台与 SDK main 均独立。
原正文“回归待执行/保持 open”是旧取证阶段的原记录，保原字节；本段为后继三链闭合状态，不指外部 Issue 或 SDK main 已关闭。
