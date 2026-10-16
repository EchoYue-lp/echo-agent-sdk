---
schema_version: 1
id: finding.sdk-workflow-resume-observation-claim
kind: finding
type: implementation_bug
status: open
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
audit_refs:
- audit.sdk-workflow-stream-late-completed-payload
decision_refs:
- evidence.sdk-workflow-protected-claim-human-decision
repair_evidence_refs:
- evidence.sdk-workflow-observation-source-gaps
verification_evidence_refs:
- evidence.sdk-workflow-observation-source-gaps
rereview_audit_refs:
- audit.sdk-workflow-stream-late-completed-payload
discovered_at: 019e700279677a2eb77e265b5038d9d581bc22da
---

# 已接受 Graph.resume 的未结算观察不能走普通 claim requeue

## 问题

SDK _echo_agent/workflow/op 的 workflow.graph.resume、resume_exact、resume_with_state 委托同一 framework Graph。workflow.graph.build 从 Session 当前 LlmClient 构建 declarative AgentNode；registered Llm proxy → ReactAgent → Node → Graph.resume 是已接受主路径，不是 deferred 任意 StepFn/闭包 API。

## 触发条件与影响

固定上游公开检查点 [f0a6cd1](https://github.com/EchoYue-lp/echo-agent/blob/f0a6cd1c13706c20955209b698c1f891c09d78b3/echo-orchestration/src/workflow/graph.rs#L1286) 的 resume_with_updates 对任何 Err 调 requeue_claim。ExecutionNotSettled 表示原工作物理未知，成功 requeue 会让另一个 resume 再领同 checkpoint；若 requeue 自己失败，组合错误变为 Other，原 run/execution 身份丢失并可被分类为失败。公开 source/tree 是 f0a6cd1c13706c20955209b698c1f891c09d78b3 / 9ee1e4340ab09e75cb2d5b5a95c22acc0611362e，不代表 SDK 已升级构建 pin。

## 处理记录

修复 owner 为 framework Graph及其CheckpointStore，SDK 只委托和无损投影，禁止在 Host 加 requeue、claim、heartbeat、恢复 scheduler 或第二 terminal authority。既有一般失败/明确终态的 requeue 政策不能因本 Finding 被无差别删除。

同一原 attempt 的 renew/heartbeat、expiry 与 restart 必须有具名反例：观察不能解锁重领；续租失败不能靠丢原 future、Other 或本地 EOF证明 physical closed；恢复不能把旧未知 attempt当作可重复副作用的凭证。它们是同权威待验证矩阵，本对象不宣称这些分支都已执行或另立产品 API。

## 证据

本段记录019e源码调查阶段，当时只有固定源码与 SDK 实际接线证据；未执行此根因的 before/after。SDK 仍固定00782，新 B variant 必须在公开框架 revision/明确租约后消费，不能用混合 path override、旧 Host 或缺 variant 的编译失败冒充该根因的 red。F owner 已承接，修复、真实同源验证和独立复审三链齐全前保持 open。其它 Workflow48/后继场景的历史通过不关闭此具体故障假设。

## 已批准 A，修复与验证尚待

用户2026-10-16明确选择“保留能力并实施保护合同（推荐）”，绑定原提案23c5a59/7288B。原Store effect前保护、单cap/reverse增量、Graph绑定Store查询及protected CRUD拒绝的确切范围见人的裁决Evidence/ADR0039。原unknown owner可能长期阻断的成本获明确接受；这里不是接受当前重复执行风险，也不把Finding改成risk_accepted/resolved。原Graph保护实现、SDK无损消费、新pin真实验证和独立复审链仍待；SDK direct-run投影独立Finding不被本裁决替代。

## 历史状态：Native 与 SDK 场景分别待验收

根任务随后已取得框架 Native FunctionNode 的 Memory/File 两项真实 before，以及后继 original-owner 两项 before；这些是框架原生路径，由F writer维护原件与验证，不是本SDK registeredLlm/AgentNode或三语Host运行证据，不能由上节“当时未执行”否认，也不能借其关闭本消费者链。

SDK A的Protocol/TS/Python/Java各3项具名pure after已实际0且独立复审；只有Java decoder有旧源码before，其新增cap/Unit作为after控制，详细原件仅在本仓任务local。Host薄proxy与三语Workflow optional场景现为源码准备：绑定原Store、protect正回应门、commit后Error无Node、实际LLM callback内原Popen未知及同PID释放。尚未编译新Host或运行这些Native场景，精Fmain/pin/唯一生成器与完整验证/复审仍待；既有Graph缺cap canonical投影与独立SDK direct-run生产均先保留，等待新pin真实反例。状态继续open，不按总数机械resolved。

## 后继14a有限消费者验收，宽义务仍开放

公开988精pin与唯一生成器后，原共享30步A场景已在不可变14a Host的TS/Python/Java各实际1例通过、0fail/error/skip，并获原reader有限pass。具体来源、各日志SHA、真实Java关闭失败及严格expected-error整例、关闭后三组backend精确不变由原Evidence的“后继14a”段承载。原protected attempt、原LLM副作用与foreign identity保持；同外部PID退出及SDK处置Host不构造framework receipt，不解除claim。Java同owner retry只结算其本机transport清理。

本对象保持open：上述消费例不代framework Memory/File heartbeat、expiry、restart及其它原owner故障矩阵，也不证明provider跨进程持久性或真实ACK丢包。Framework Graph/Store仍唯一拥有保护和结算；SDK不增加恢复调度器、第二receipt或状态权威。
