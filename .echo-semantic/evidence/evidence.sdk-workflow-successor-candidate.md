---
schema_version: 1
id: evidence.sdk-workflow-successor-candidate
kind: evidence
observed_at: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
source_refs:
- echo-sdk-host/src/core_profile/facade/workflow.rs
- sdks/shared/testing/workflow_checkpoint_fixture.py
- sdks/typescript/test/workflow-checkpoint-host.test.js
- sdks/python/tests/test_workflow_checkpoint_host.py
- sdks/java/src/test/java/com/echoagent/sdk/WorkflowCheckpointHostTest.java
- docs/sdk/governance/evidence/sdk-workflow-checkpoint-20261013/sdk-workflow-formal-proof.json
- docs/sdk/governance/evidence/sdk-workflow-checkpoint-20261013/execution-summary.json
- docs/sdk/governance/evidence/sdk-workflow-checkpoint-20261013/execution-evidence.md
- docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/final-receipt.json
- docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/source-build-manifest.json
- docs/sdk/governance/evidence/sdk-language-factory-final/receipt.json
- docs/sdk/governance/evidence/sdk-language-factory-final/source-before.json
- docs/sdk/governance/evidence/sdk-language-factory-final/source-after.json
supports:
- behavior.sdk-family-workflow
limitations:
- 48 rows=47workflow calls+1Sessionclose; 17of24acceptedoperations, not completefamily.
- Successor Host cb9/source f5c72 does not inherit old6711 whole418/three-language
  completegreen.
- UUID/time stable relation mapping and unordered keys normalization only; rawordereddata
  kept.
- 7acceptedoperations, stream/cancel/restart/durableexternalstore and finalpin/main
  remain pending.
---

# Workflow successor behavior candidate

## 支持的结论

三语言正式共享测试执行 48 条记录：47 次 Workflow 调用与一次 Session close，覆盖 24 个已接受操作中的 17 个，包含 11 个预期 typed error。归一化只稳定标记 UUID/时间的身份关系，并排序 SharedState.keys 无序集合；路径、标签、上下文、状态及数值字段保留原顺序和内容。三语言结果一致。

具名断言覆盖完整 17 字段 checkpoint、restore/resume/branch、父子身份与 lineage、标签/generation、state/owner/quota，以及 Session 关闭后的 stale handle。其它七个已接受操作仍开放。

## 来源与范围

旧 Host6711 对 tuple checkpoint 输入错误地继续执行或消费 canonical state；后继 cb9 在副作用前拒绝，并保留原字段。当前 Workflow 源码 SHA f5c72f5e132d0de1e2fd6b9a17fb892201026838cff72db4b5a802bed02e7701 与归档 formal proof 一致。Host SHA cb9b7fcad3946d9428cdef5a7b7258a334ee387ed34ca0612b4f40b262ef20a2。

实际输入、日志、原始 transcript 和归一化结果都在 Workflow 归档目录；不是将旧 Rust418/TS240/Python300/Java264 全套结果重新命名为新验证。该证据仅支持这份后继来源和列出的断言，仍使用临时 framework override 与既有接受范围。

## 已知缺口

完整 stream next/cancel/close、graph cancel、extension run/run_stream、provider/durable restart 与并发 claim 矩阵未证明。所有模型保持 needs_review。最终接受合同生成、已发布 pin、匹配的完整门禁和精确 main 验收仍待。

正式consumer归档路径对应WorkflowCheckpoint前缀，原formalreceipt内旧命名作为原输入epoch保留；当前源码路径已显式重定向，不假旧文件仍在。

## 本次 current_source_business_rereview

原已审SDKruntime代码来源Git569/source17cf和治理文稿可恢复Git`9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d`（source0a13）明确分开。独立reader`sdk_capability_acceptance_audit`本次实际重读原业务fault/完整字段/副作用/owner/调用方变化与14Rust/newbb4f632原after证据，消息回收UTC`2026-10-14T01:56:14.724478+00:00`，不是原执行时间。结果：**PASS0 有限原业务义务**。

WF48原17/24operations及后继7verb分开有native消费，完整WorkflowCheckpoint17fields是不同于AgentCheckpoint7的类型；lineage/stateowner/resume/branch/tuple错误无consume已核；MemoryCheckpointStore无未accepted跨进程保证。

原观测marker`source:3ea544c47e331b5d29a6624f978f3595619bef39fa5985230351098d38930404`、旧run-time/host/sourcehash/raw和正文保，所有source_refs从真实9dbGitblob读取且锚点存在；现在SDKbe5pin与新测试写入会改变当前文件，不能把历史b909当前源码审升级成未来be5gate。真实9db框架pin为b909，工作树be5after另Evidence处理，不将两个仓tree混写。新增门禁sourceRef只具名Scope，没有以整count判族verified；supports/limits原保、不声明旧3ea整仓可恢复、不改unknown成无缺陷。

原义务和全文历史主张保留以审查，若旧unknown因素（TTL/三getter消费）已有新原证据，只在本段给来源区别，不抹原epoch。ChartI1/5sB历史BLOCK明确不因observed版本合法消失，12source/CI/main/globalacceptedproducer仍pending，41family/lens状态不提升。
