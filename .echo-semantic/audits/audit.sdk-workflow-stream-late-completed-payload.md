---
schema_version: 1
id: audit.sdk-workflow-stream-late-completed-payload
kind: audit
boundary_ref: boundary.sdk-integrations
lens: failure_concurrency
freshness: examined
revision: source:63bd9a4fce3c7177293460c887500eb369d93fc26d2e51820d7b2b88c58d5c43
finding_refs:
- finding.sdk-workflow-stream-late-completed-payload
- finding.sdk-workflow-direct-run-observation-projection
challenges:
  direct-run-observation-versus-ordinary-error:
    source_refs:
    - echo-sdk-host/src/core_profile/facade/mod.rs
    - echo-sdk-host/src/core_profile/wire.rs
    evidence_refs:
    - evidence.sdk-workflow-stream-late-completed-payload
  original-source-and-actual-terminal:
    revision: source:63bd9a4fce3c7177293460c887500eb369d93fc26d2e51820d7b2b88c58d5c43
    source_refs:
    - echo-sdk-host/src/core_profile/facade/mod.rs
    - sdks/typescript/test/llm-workflow-stream-host.test.js
    - sdks/python/tests/test_llm_workflow_stream_host.py
    - sdks/java/src/test/java/com/echoagent/sdk/LlmWorkflowStreamHostTest.java
    - sdks/shared/testing/llm_workflow_stream_fixture.py
    evidence_refs:
    - evidence.sdk-workflow-stream-late-completed-payload
---

# Workflow 晚到 Completed 局部独审

## 审查范围

仅原extension Workflow观察后的Completed payload转发和三语测试源码，不是Graph晚到回执或完整SDK验收。

后继C1整体独审还覆盖同原Workflow direct-run投影：仅ExecutionNotSettled无损转换，ordinary原错误政策保持；对应实际原red/after由同Evidence分别定位。C1纯三语言与三native有限结果被reader核对，不是C1全部门禁或后继生产cut已完成。

## 已检查故障假设

将观察错误当终止进入drain丢弃原结果；错误解包duration使测试与真实decoder冲突；局部源码审被误当三语已运行。

## 实际实现路径与证据

最新C2整体增量审复用原C1全源码审，并核队列select修复、配置唯一性、新14a/f658运行来源；亲读4项Rust定向、三语各1的完整日志/runner/Java freshXML及611语言/63编译输入零漂移，pass、行动项0。reader明确direct-run与late Completed两个具名Finding已有原真实before、修复、当前after与独立复审，可以在准确引用下有限resolved；不扩大为完整SDK或9条排除路径已验。

独立 reviewer 对上述源码返回 pass，无新增阻断或行动项；其实际读取原 payload 丢失 before 和 duration 错误断言 after，未运行 Cargo/Host。审查确认 continue 只恢复原 source 转发，取消/sendfail 不变；三语各一次原 callback 创建 producer，shared verifier 精核完整结果、duration、原 handle 与 seq1→seq2。删除 TS 错误解包断言未降低字段要求。

同连接通知处理顺序有依赖源码与 Host inline await 的依据，测试不靠 sleep 或重试掩盖事件竞态。

## 问题记录

已修复局部转发与错误duration断言；原失败与修正后同case红绿分别保留，没有降低逐字段要求。

## 残余风险

header仅绑定精确候选源码引用；原独审和TypeScript执行结果仍保上文各自固定输入，不是重新执行候选三语言。

本审查只覆盖局部实现与测试源码，不证明Python/Java已运行，也不证明当时的opt1候选消除链接警告或整体SDK完成。后继保存profile构建有独立证据，不回写本历史审查范围。

## 未检查项

任意Graph晚到结果、持久恢复、独立Task/protected场景、完整工程门禁与精确main CI。具名late-payload的最终共同cut三语言各1已实际核对；不把它扩为整个语言套件。
