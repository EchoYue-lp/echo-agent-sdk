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
observed_at: 02059dd3014633a23c188e30e73b0634a8c5621d
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
- evidence.sdk-task-memory-workflow-finite-observation
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

原569阶段缺口（历史，当前有限范围见后继段）：三语言 revisioned DAG/claim control/context 字段同场景比较与 published framework successor 验收仍缺。 published framework pin、最后变更后的独立复审和精确main验收
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

## 当前模型重观察范围

review_scope：本对象在真实当前候选Git `56929154c1851473e9e4837901b610cecf001b2d`（源码摘要`17cf243e8d6341db0ce23c5b9ee39977ce4ab100d6ed1b17ec5a7bb26c19e138`）上重新观察。上列code_refs/各scenario source_refs已逐Gitblob读取并与当前文件SHA精确相同，原锚点均存在。模型继续描述原accepted入口、框架业务/终态权威、Host owner/generation与transport消费职责、生命周期及具名失败义务；源码定位核对不证明所有行为正确、运行证据fresh或合同生成闭合。

原Behavior/status、Map场景mapped/needs_review、Rule/Asset处置、Finding/未决义务与Evidence原epoch支持范围全部保留。mapped仅入口模型，旧3ea不是本snapshot本体；历史receipt不得因本模型观察更新冒current运行。12source迁移、5秒物理结算与依赖50warning规则三选择仍未接受，完整生成/CI/main另验，任何新Rustwrapper不自动扩三语言合同。

## 当前Task/Memory/Workflow具名审查

当前观测恢复真实02059，所选TaskRuntime/Subagent/memory/workflow/selector/sourceoperations等10文件与原审69b逐blob一致；currentfactory/coretests因意图A变化，原源定位不冒新A被审。CLI原实际8lens作者sourceaudit有限examined、无新增已证实缺陷，后经本材料独立质控；作者身份与独立review角色不混同。新sharedEvidence和8窄Audit按四acceptedfamily与原唯一authority明确scope，没有near-synonym contextfamily。

原正式a634/790whole TaskJ4/WFcheckpointJ1/MemoryJ1/LLMWFJ3及三语强断言支持exactCAS、canonicaldeps、短await不耗结果、liveStore、checkpoint/claimack与streamcancel具名路径。Taskcontext四Host getter消费为runid/waiver/nonwaivedmissingresult/nondefaultpolicy；三nativeCallback仅runid来自SessionACP，不宣完整3fieldreverseDTO。ContextInheritance纯值不是livehistory/memory继承E2E；compressor真实注册消费不推全部三nativehook重入取消，这一具体支持性缺口保留。

Behavior保持needs_review/场景mapped只表有限审查闭包。内存Store/checkpoint无额外crossprocessdurability保证，5秒physicalB/wideTaskFinding仍open、12sourceidentity仍待人，当前A已有批准但其实现后原生/全gate另切点。scope外未接受sourceoperation/构造器不纳新增义务，也不以‘所有组合未穷尽’否定已审能力路径。
