---
schema_version: 1
id: behavior.sdk-task-subagent
kind: behavior
status: verified
expectation: human_confirmed
risk: high
primary_focus: state_authority
focus:
- time_lifecycle
- failure_concurrency
- contract_evidence
- data_durability
boundary: boundary.sdk-task-subagent
observed_at: 04eb6ad50982656b5a8b8ea649619b6a383065ff
code_refs:
- echo-sdk-host/src/core_profile/facade/task.rs
- echo-sdk-host/src/core_profile/facade/task_runtime.rs
- echo-sdk-host/src/core_profile/facade/subagent.rs
- echo-sdk-host/src/factory.rs
- echo-sdk-protocol/src/methods.rs
- echo-sdk-host/src/core_profile/facade/task_runtime.rs#claim_task
- echo-sdk-host/src/core_profile/facade/subagent.rs#collect_result
- echo-sdk-host/tests/core_profile_e2e.rs#task_cancel_is_exact_and_pending_sibling_continues
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-task-memory-workflow-finite-observation
- evidence.sdk-model-read-retained-verification
finding_refs:
- finding.sdk-subagent-await-result
- finding.sdk-task-control-locality
- finding.sdk-task-runtime-store-authority
---

# Task 与 Subagent

## 重要承诺

本能力的行为、唯一权威、生命周期、失败路径与无损合同必须有可复核证据。

## 当前行为

factory中的同一个TaskRevisionService供应CRUD及FacadeTaskController；controller仅转换metadata、claim/dispatch并调用framework服务，不拥有frontier或第二DAG validator。execution CAS preflight在副作用前拒CRUD-only后端；exact pause/cancel绑定canonical attempt，独立sibling继续，Completed result含absent/null区别。Subagent单collector保留join结果，短await超时后可再次await，不提前耗尽句柄；runtime失败经task/list保留origin_operation/cause。Hostcore31/bridge34中这些具名场景已执行。

## 期望行为

保持框架业务权威、Host寻址与资源ownership、三语言无损投影；所有已确认缺陷修复并取得最新反例验证。

## 触发、结果与副作用

源码入口与实际注册/分派路径由本 Map 的 source_refs 定位。输出为 typed response/error、framework committed event 或 generation-fenced handle；所有动态入口仍需逐场景验证。

## 失败、重试与恢复

factory中的同一个TaskRevisionService供应CRUD及FacadeTaskController；controller仅转换metadata、claim/dispatch并调用framework服务，不拥有frontier或第二DAG validator。execution CAS preflight在副作用前拒CRUD-only后端；exact pause/cancel绑定canonical attempt，独立sibling继续，Completed result含absent/null区别。Subagent单collector保留join结果，短await超时后可再次await，不提前耗尽句柄；runtime失败经task/list保留origin_operation/cause。Hostcore31/bridge34中这些具名场景已执行。

原569阶段缺口（历史，当前有限范围见后继段）：三语言共享revisioned DAG/claim/control/context字段完整场景未闭合；上游parent-drop/prejoin/driver-unwind准入新增修复须在最终组合快照重新复审和全门禁，旧framework23绿色不替代它。

## 证据

具名候选运行范围由本组Map关联的child family Behaviors及其 `evidence.sdk-candidate-behavior-verification` 定位；该Evidence只supports具体family，不把其引用冒充本组完整通过。

所有状态保持 needs_review。具名断言的候选通过不证明能力全覆盖：SDK manifest 固定95ec66c6，八crate临时patch组合Task/ACP/LSP源码用于本地验证；最终Channel后继published revision、移除patch后的独立构建、精确main和远端CI仍待。完整Rust inventory仅遥测，最终accepted字段/退役必须由唯一生成器明确裁决。

三语言共享revisioned DAG/claim/control/context字段完整场景未闭合；上游parent-drop/prejoin/driver-unwind准入新增修复须在最终组合快照重新复审和全门禁，旧framework23绿色不替代它。

## 裁决记录

用户明确要求完成八项治理，三语言范围只以明确接受合同为准；已授权子智能体执行，主线程负责协调及验收。

## 当前观察、归档和后继边界

原fc63证据已保存为仓库内不可变历史收据，原观察epoch不可恢复为Git时不补造提交。原6711证据归档支持具名局部断言；原27个历史输入中tools.rs已变，具名测试body保持可对照；55个6711运行输入中tools.rs和Workflow已变更。新Workflow语义或其它后继场景必须绑定自己的来源/日志；不把旧整套418/TS240/Python300/Java264冒充最终当前通过。支持见历史审计Evidence及evidence.sdk-parent-driver-candidate-verification的限制，所有needs_review/开放Finding保留。

## 当前模型重观察范围

review_scope：本对象在真实当前候选Git `56929154c1851473e9e4837901b610cecf001b2d`（源码摘要`17cf243e8d6341db0ce23c5b9ee39977ce4ab100d6ed1b17ec5a7bb26c19e138`）上重新观察。上列code_refs/各scenario source_refs已逐Gitblob读取并与当前文件SHA精确相同，原锚点均存在。模型继续描述原accepted入口、框架业务/终态权威、Host owner/generation与transport消费职责、生命周期及具名失败义务；源码定位核对不证明所有行为正确、运行证据fresh或合同生成闭合。

原Behavior/status、Map场景mapped/needs_review、Rule/Asset处置、Finding/未决义务与Evidence原epoch支持范围全部保留。mapped仅入口模型，旧3ea不是本snapshot本体；历史receipt不得因本模型观察更新冒current运行。12source迁移、5秒物理结算与依赖50warning规则三选择仍未接受，完整生成/CI/main另验，任何新Rustwrapper不自动扩三语言合同。

## 当前Task/Memory/Workflow具名审查

当前观测恢复真实02059，所选TaskRuntime/Subagent/memory/workflow/selector/sourceoperations等10文件与原审69b逐blob一致；currentfactory/coretests因意图A变化，原源定位不冒新A被审。CLI原实际8lens作者sourceaudit有限examined、无新增已证实缺陷，后经本材料独立质控；作者身份与独立review角色不混同。新sharedEvidence和8窄Audit按四acceptedfamily与原唯一authority明确scope，没有near-synonym contextfamily。

原正式a634/790whole TaskJ4/WFcheckpointJ1/MemoryJ1/LLMWFJ3及三语强断言支持exactCAS、canonicaldeps、短await不耗结果、liveStore、checkpoint/claimack与streamcancel具名路径。Taskcontext四Host getter消费为runid/waiver/nonwaivedmissingresult/nondefaultpolicy；三nativeCallback仅runid来自SessionACP，不宣完整3fieldreverseDTO。ContextInheritance纯值不是livehistory/memory继承E2E；compressor真实注册消费不推全部三nativehook重入取消，这一具体支持性缺口保留。

Behavior保持needs_review/场景mapped只表有限审查闭包。内存Store/checkpoint无额外crossprocessdurability保证，5秒physicalB/wideTaskFinding仍open、12sourceidentity仍待人，当前A已有批准但其实现后原生/全gate另切点。scope外未接受sourceoperation/构造器不纳新增义务，也不以‘所有组合未穷尽’否定已审能力路径。

## 2026-10-19 context支持边界已分类

原Task/Subagent Evidence后继明确：ContextInheritance7 intrinsic与4值字段是本地纯值；原TS全字段preset和PY/Java代表默认断言，不是livehistory/memory父继承。未接受from_parent/新DTO不新增；parent_context nonnull保持原process-local typed拒绝。原普通Session history/live Store切换仍沿各自真实消费者，不混同Subagent继承。accepted compressor预算/retained producer after与模型适配仍待，SourceOperation两个open Finding不因本段关闭；不新family、不改此needs_review或既有Task/claim权威。


## 2026-10-20 具名有限结算

本项verified只限原声明的具名合同单元及下列直接证据，非全SDK/平台或任意未列组合。原模型header observed_at为`02059dd3014633a23c188e30e73b0634a8c5621d`，原正文及Evidence各自runtime cut保留；当前模型观察绑定公开`04eb6ad50982656b5a8b8ea649619b6a383065ff`，不把旧测试重标为新Host执行。

直接证据：`evidence.sdk-task-memory-workflow-finite-observation`、`evidence.sdk-model-read-retained-verification`。

具体复审入口：`audit.sdk-task-memory-workflow-finite-state-authority`、`audit.sdk-retained-identity-rereview`；只读者已按这一个named单元核权威、生命周期、失败与证据边界，不能由共享对象推整八风险组。

源码后继分开核：原Root17be/0bbc具名review与04eb薄增量分别C0。04eb相对17be只在media lookup保两accepted名字到原Tool，三测试wrapper新增可选媒体故事，原fixture五函数/默认HTTP/旧oracles不变；本项所依非媒体生产路径不因共有文件名改变而假报重审或全套新运行。新媒体after另由a17/三语focused证明，未借旧208a14为新源码完整资格。

支持范围仍限原named consumer/字段/副作用及其明确失败；未接受identity/任意provider、codec、选项或竞态组合不自动加入分母。
