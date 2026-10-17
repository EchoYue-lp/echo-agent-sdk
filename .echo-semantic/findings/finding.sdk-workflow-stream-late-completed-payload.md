---
schema_version: 1
id: finding.sdk-workflow-stream-late-completed-payload
kind: finding
type: implementation_bug
status: resolved
severity: high
primary_focus: failure_concurrency
focus:
- state_authority
- time_lifecycle
- contract_evidence
boundary_ref: boundary.sdk-integrations
behavior_refs:
- behavior.sdk-family-workflow
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
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
discovered_at: 637a308e0d1c7f13339d279eb1f91aedfb403685
---

# Workflow 观察后的原 Completed payload 被清理 drain 丢弃

## 问题

原extension stream的观察后清理会丢弃真正晚到的Completed payload。

## 触发条件与影响

真实 `workflow.extension.run_stream` 原 callback 创建 producer，先经同一 writer 报告完整 not_settled，原工作仍由 gate 持有；释放并实际发送 complete 后，旧 Host 只返回 generic complete。`facade/mod.rs` 的观察分支 break 进入忽略所有 Ok 的清理 drain，丢失原 result、total_steps、elapsed，违反既有 Completed item 后接 generic complete 的合同。

## 处理记录

最小修复只把该观察分支改为 continue，继续转发原 source；取消和发送失败仍走原 drain。不增加执行、状态、receipt、操作或 wire variant，也不从这个反例推断另一 Graph source 存在相同的晚到结果路径。

## 证据

原公开 TypeScript SDK 同例已真实 before1→after0，含 gate held、单次 callback、原 stream/extension、完整错误身份、真实 complete、完整 Completed payload 和 seq1→seq2。Python/Java 同场景已加入原 native suites，复用原 fixture payload 与共享逐字段 verifier，尚未执行。独立源码审查通过，不代三语言运行或最后 profile 门禁；本对象继续 open，等待最终来源的验证和独立复审链。

后继精确C1候选已实际三语各1pass/0fail/0skip，611语言输入前后一致，原件与精Host/source身份已补原Evidence；前段“尚未执行”保原阶段，不作为当前状态。之后同转发任务满队列取消被整体审查发现，已有真实反例与最窄修复/两case after及增量复审，最终共同cut仍须刷新direct与三native。故本Finding保持open，不把C1旧结果转写成后继源码或main通过。

## 最新具名闭合

最终共同cut的新14a Host已重新在精C2三语各实际1pass、无失败/跳过，611语言输入零漂移；同时新f658测试binary direct1/unwind1/drain2通过。原TS before/after和C1结果均保独立，最新结果/完整payload及seq断言由同Evidence定位。C2整体增量reader亲核输入/源码/实际raw/runner/Java freshXML返回pass、行动项0，并确认该缺陷可有限resolved。此处不推断另一Graph晚到回执或关闭宽生命周期义务；前段open仅原等待阶段，整体SDK/主目标/main保持未完成。
