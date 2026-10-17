---
schema_version: 1
id: finding.sdk-workflow-direct-run-observation-projection
kind: finding
type: implementation_bug
status: resolved
severity: high
primary_focus: failure_concurrency
focus:
- state_authority
- time_lifecycle
- data_durability
- contract_evidence
boundary_ref: boundary.sdk-integrations
behavior_refs:
- behavior.sdk-family-workflow
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-workflow-observation-source-gaps
- evidence.sdk-workflow-stream-late-completed-payload
audit_refs:
- audit.sdk-workflow-stream-late-completed-payload
decision_refs: []
repair_evidence_refs:
- evidence.sdk-workflow-stream-late-completed-payload
verification_evidence_refs:
- evidence.sdk-workflow-stream-late-completed-payload
rereview_audit_refs:
- audit.sdk-workflow-stream-late-completed-payload
discovered_at: 019e700279677a2eb77e265b5038d9d581bc22da
---

# Workflow extension 非流 run 的观察错误被字符串化

## 问题

Session.invoke(workflow.extension.run, [registration, input]) 经 _echo_agent/workflow/op → facade_admission → registered Workflow component proxy.run → 原 reverse agent_component_call/workflow_run。现有 TypeScript llm-workflow-stream-host.test.js 的真实前向调用使用该入口。

## 触发条件与影响

SDK019e 的 facade/mod.rs 在 direct 非流分支把任意 ReactError 经 error.to_string → sdk_error(FrameworkError, Never)。上游B/当前 Host adapter 恢复 ExecutionNotSettled 后，这处仍会丢 observation/run_id/可选 execution_id；调用方收到普通 FrameworkError，无法区分物理未知。相邻 Graph adapter 的 framework(error) 已委托唯一 wire::framework_error，不能据此假定本独立分支已修。

## 处理记录

修复 owner 是 SDK 错误投影；原 CallbackInvocationOwner、Workflow 原执行与 framework graph/claim 权威不变。显式 NotSettled 必须经唯一 wire projector，原完整身份不裁剪/不缺省；普通 direct Workflow error 的 code、Never、operation、原消息前缀及普通 details 政策保持。不能在 SDK补 requeue/terminal/默认结果或新 driver。

## 证据

已准备 facade::tests::workflow_direct_run_preserves_observation_while_original_work_is_held：真实 in-process 官方 ACP/profile 前向请求与反向 Workflow callback；原工作只在该 callback 实际准入，持有同一 guard/Join；RPC 必须在工作 held 时返回完整原 error，同时保原 invocation admission。另一 ordinary callback 严格对照原错误消息/无 observation fields，最后明确 release/join 同一个原工作。测试传输 teardown 不当作未知 owner 的正常物理结算。

原准备阶段仅有测试源码、生产stringify保持，须先精确新pin/租约再真实red；这是原019e调查时点，保留其顺序要求，不表示下述实际执行尚未发生。旧00782 Host、缺variant编译错误、单纯helper或其它family通过均不算反证。

## 后继实际修复与有限验证

公开988 pin后，同一正式ACP/profile强例实际1fail：observation被stringify而丢details；ordinary对照已先通过。最窄修复仅ExecutionNotSettled走唯一wire projector，ordinary原前缀/字段政策保持；同selector实际1pass，两个原opt0日志各有Darwin链接warning，保原epoch而不冒最终零警告。源码与C1整体独审已覆盖该投影，修复/验证/复审沿原Workflow Evidence/Audit追加，不另造终态权威。

满队列修复阶段曾等待最终配置/共同cut下direct selector及受影响native刷新，彼时本Finding保持open，要求具体最新验证与三链复核齐备后再结算。该阶段没有把foreign work退出或测试teardown当未知Host执行的物理回执；后继终态见下节。

## 最新具名闭合

最终共同cut的direct完整selector在不可变f658测试binary实际1pass，三语late-payload在新14a Host各1pass；原真实before与warning epoch保持，具体日志/精确输入见原Workflow Evidence。C2整体增量reader亲核源码、配置、新运行原件及输入，pass、行动项0，并明确本Finding具名条件齐备。此处只关闭direct-run丢typed观察的投影缺陷；宽cleanup/Graph claim/Task/Sandbox/managed、完整SDK和精确main不由它关闭，前段open描述保留原等待阶段。
