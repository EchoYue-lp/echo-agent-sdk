---
schema_version: 1
id: behavior.sdk-family-task
kind: behavior
status: needs_review
expectation: human_confirmed
risk: high
primary_focus: state_authority
focus:
- time_lifecycle
- failure_concurrency
- contract_evidence
- data_durability
boundary: boundary.sdk-task-subagent
observed_at: source:3ea544c47e331b5d29a6624f978f3595619bef39fa5985230351098d38930404
code_refs:
- echo-sdk-host/src/core_profile/facade/task.rs
- echo-sdk-host/src/core_profile/facade/task_runtime.rs
- echo-sdk-host/src/core_profile/facade/subagent.rs
- echo-sdk-host/src/factory.rs
- echo-sdk-protocol/src/methods.rs
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-candidate-behavior-verification
- evidence.sdk-parent-driver-candidate-verification
- evidence.sdk-task-subagent-candidate
- evidence.sdk-stream-cancellation-routing-verification
- evidence.sdk-task-context-direct-getter-consumption
finding_refs:
- finding.sdk-accepted-source-continuity
- finding.sdk-task-resume-extension-invocation-conflict
---

# SDK family task

## 重要承诺

对已接受 `task` 能力明确入口、唯一权威、状态/生命周期、失败处置和直接行为证据；未接受 Rust identity 不进入承诺。

## 当前行为

- **入口/可达**：typed task CRUD/execute/control → `facade/task.rs`、`task_runtime.rs`；
  Session live `RevisionedTaskStore` 与 runtime service 被真正 execute/control 消费。
- **权威/状态**：framework `TaskRevisionService`、`RuntimeTaskService`、revisioned store
  及 claim；`FacadeTaskController` 只 dispatch/转换/settle。graph revision → exact claim →
  attempt → settle；pause/cancel 必须限制在对应 Task/attempt。
- **正常/失败**：依赖结果进入 successor；重复 execute、stale revision、active-claim patch、
  local pause/resume/cancel、close interruption 不改变 sibling claim 或丢失 result。
- **已有断言**：Core task authority/execute-control tests、
  `task_pause_is_exact_and_resume_reuses_canonical_dependency_result`、
  `task_cancel_is_exact_and_pending_sibling_continues`；Facade duplicate execute typed conflict。
  候选组合源码与Host已编译执行；正式pin/main仍待。
- **最低充分证据**：必要 framework successor 完成后跑上述场景及三语言并发 DAG；对
  snapshot revision、claim identity、sibling 状态、resume dependency result、full context
  DTO 作字段级断言，证明 adapter 无 frontier/重试/取消第二权威。

## 期望行为

上述正常/失败场景须保持接受范围、完整转换和唯一权威，并在最终源码/pin证据及独立复审后闭合。

## 触发、结果与副作用

按上方具体入口和正常/失败记录检查真实主路径。可达性或catalog状态不能证明副作用、错误或结算。

## 失败、重试与恢复

按上方生命周期和最低充分证据检查取消、超时、错误、释放和恢复；不默认成功，不以其它能力族测试替代。

## 证据

阶段：`candidate_runtime_verified`；语义状态仍为 `needs_review`。

CRUD 与 execute 真实共享 Session framework store；exact pause/resume 保存依赖结果，exact cancel 让 pending sibling 继续；仅 CRUD 后端 execute 在副作用前明确拒绝；CAS backend error 可查询 cause。

实际来源：`echo-sdk-host/tests/core_profile_e2e.rs#task_rpc_shares_the_session_task_authority`；`echo-sdk-host/tests/core_profile_e2e.rs#task_pause_is_exact_and_resume_reuses_canonical_dependency_result`；`echo-sdk-host/tests/core_profile_e2e.rs#task_cancel_is_exact_and_pending_sibling_continues`；`echo-sdk-host/tests/extension_bridge_e2e.rs#crud_only_task_store_rejects_execute_before_runtime_side_effects`；`echo-sdk-host/tests/extension_bridge_e2e.rs#revisioned_task_extension_is_shared_by_crud_and_execution`。

仍缺：三语言 revisioned DAG/claim control/context 字段同场景比较与 published framework successor 验收仍缺。 published framework pin、最后变更后的独立复审和精确main验收
仍待；本证据不证明未列出的生命周期、字段或错误组合，也不自动扩大accepted合同。

## 裁决记录

用户要求SDK全能力行为治理，以明确接受跨语言合同为兼容边界；完整Rust inventory仅遥测。

## 当前观察与历史收据边界

Four temporary shared true-Host scenarios per language are prepared/executed against6711: exact pause/cancel sibling isolation, persisted dependency result reuse, stale revision/duplicate execute, idempotent short/full await, CRUD-only preflight. They remain temporary inputs and do not prove complete context/failure/restart coverage.

正式承接增量：cb9Host上的公开正式三语言各4test已通过，来源/日志/XML按evidence.sdk-task-subagent-candidate正式proof限定；原临时输入不再被误说为未采用，但完整family仍needs_review。

## 本次有限审查单元

行为/生命周期单元：一个revisioned graph/service执行CAS、exact控制及canonical依赖结果。

剩余处置：正式三语言四场景有界支持；原claim/context剩余字段对账与复审receipt归档，不能自动扩Task API。

本项建模阈值是已接受入口、唯一状态权威、正常生命周期与具名失败/恢复路径可定位，并有直接证据或明确开放原因；不要求所有未定义组合全测，也不由绿色测试自动verified。
