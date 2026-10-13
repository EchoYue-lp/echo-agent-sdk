---
{
  "schema_version": 1,
  "id": "behavior.sdk-agent-session-run",
  "kind": "behavior",
  "status": "needs_review",
  "expectation": "human_confirmed",
  "risk": "high",
  "primary_focus": "state_authority",
  "focus": [
    "time_lifecycle",
    "failure_concurrency",
    "contract_evidence",
    "data_durability"
  ],
  "boundary": "boundary.sdk-agent-session-run",
  "observed_at": "source:3ea544c47e331b5d29a6624f978f3595619bef39fa5985230351098d38930404",
  "code_refs": [
    "echo-sdk-host/src/core_profile/handler.rs",
    "echo-sdk-host/src/core_profile/handles.rs",
    "echo-sdk-host/src/core_profile/state.rs",
    "echo-sdk-host/src/core_profile/persistence.rs",
    "echo-sdk-host/src/factory.rs",
    "echo-sdk-host/src/lib.rs#manual_close_keeps_active_run_actor_until_cleanup_and_receipt_settle",
    "echo-sdk-host/tests/core_profile_e2e.rs#explicit_session_create_reserves_persisted_turn_identity_before_new_run",
    "sdks/python/tests/test_parallel_sessions_host.py"
  ],
  "rule_refs": [
    "rule.framework-runtime-authority"
  ],
  "evidence_refs": [],
  "finding_refs": [
    "finding.sdk-agent-close-cascade",
    "finding.sdk-cleanup-retry-owner"
  ]
}
---

# Agent、Session、Run

## 重要承诺

本能力的行为、唯一权威、生命周期、失败路径与无损合同必须有可复核证据。

## 当前行为

Agent/Session/Run handler复用framework规范执行、receipt与取消。HandleRegistry唯一管理owner/generation/idempotency/配额；创建及关闭admission、durable receipt slot、失败写入重试和Agent cascade已有候选反例。Session create/load读取持久原生身份并调用framework reserve_recovered_turn_ids，turn7后分配turn8；禁用同一调用得到turn1的两路径红证据已取得。活Run关闭保持同一个官方actor，不以watcher替代join/receipt。真实kill恢复Interrupted；并行Session取消A不改变B。

## 期望行为

保持框架业务权威、Host寻址与资源ownership、三语言无损投影；所有已确认缺陷修复并取得最新反例验证。

## 触发、结果与副作用

源码入口与实际注册/分派路径由本 Map 的 source_refs 定位。输出为 typed response/error、framework committed event 或 generation-fenced handle；所有动态入口仍需逐场景验证。

## 失败、重试与恢复

Agent/Session/Run handler复用framework规范执行、receipt与取消。HandleRegistry唯一管理owner/generation/idempotency/配额；创建及关闭admission、durable receipt slot、失败写入重试和Agent cascade已有候选反例。Session create/load读取持久原生身份并调用framework reserve_recovered_turn_ids，turn7后分配turn8；禁用同一调用得到turn1的两路径红证据已取得。活Run关闭保持同一个官方actor，不以watcher替代join/receipt。真实kill恢复Interrupted；并行Session取消A不改变B。

仍缺：所有资源种类的并发publication/close I/O故障与多Session state隔离尚未逐种证明；framework父future drop的pre-join/driver准入新反例正在修复，旧Task23命令绿色或局部prejoin绿色不是当前最终权威证明。

## 证据

具名候选运行范围由本组Map关联的child family Behaviors及其 `evidence.sdk-candidate-behavior-verification` 定位；该Evidence只supports具体family，不把其引用冒充本组完整通过。

所有状态保持 needs_review。具名断言的候选通过不证明能力全覆盖：SDK manifest 固定95ec66c6，八crate临时patch组合Task/ACP/LSP源码用于本地验证；最终Channel后继published revision、移除patch后的独立构建、精确main和远端CI仍待。完整Rust inventory仅遥测，最终accepted字段/退役必须由唯一生成器明确裁决。

所有资源种类的并发publication/close I/O故障与多Session state隔离尚未逐种证明；framework父future drop的pre-join/driver准入新反例正在修复，旧Task23命令绿色或局部prejoin绿色不是当前最终权威证明。

## 裁决记录

用户明确要求完成八项治理，三语言范围只以明确接受合同为准；已授权子智能体执行，主线程负责协调及验收。

## 当前观察、归档和后继边界

原fc63证据已保存为仓库内不可变历史收据，原观察epoch不可恢复为Git时不补造提交。原6711证据归档支持具名局部断言；原27个历史输入中tools.rs已变，具名测试body保持可对照；55个6711运行输入中tools.rs和Workflow已变更。新Workflow语义或其它后继场景必须绑定自己的来源/日志；不把旧整套418/TS240/Python300/Java264冒充最终当前通过。支持见历史审计Evidence及evidence.sdk-parent-driver-candidate-verification的限制，所有needs_review/开放Finding保留。
