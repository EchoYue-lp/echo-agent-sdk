---
schema_version: 1
id: behavior.sdk-agent-session-run
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
boundary: boundary.sdk-agent-session-run
observed_at: 04eb6ad50982656b5a8b8ea649619b6a383065ff
code_refs:
- echo-sdk-host/src/core_profile/handler.rs
- echo-sdk-host/src/core_profile/handles.rs
- echo-sdk-host/src/core_profile/state.rs
- echo-sdk-host/src/core_profile/persistence.rs
- echo-sdk-host/src/factory.rs
- echo-sdk-host/src/lib.rs#manual_close_keeps_active_run_actor_until_cleanup_and_receipt_settle
- echo-sdk-host/tests/core_profile_e2e.rs#explicit_session_create_reserves_persisted_turn_identity_before_new_run
- sdks/python/tests/test_parallel_sessions_host.py
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-acp-agent-run-finite-observation
- evidence.sdk-model-read-retained-verification
finding_refs:
- finding.sdk-agent-close-cascade
- finding.sdk-cleanup-retry-owner
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

原阶段缺口（历史，当前有界范围见下）：所有资源种类的并发publication/close I/O故障与多Session state隔离尚未逐种证明；framework父future drop的pre-join/driver准入新反例正在修复，旧Task23命令绿色或局部prejoin绿色不是当前最终权威证明。

## 证据

具名候选运行范围由本组Map关联的child family Behaviors及其 `evidence.sdk-candidate-behavior-verification` 定位；该Evidence只supports具体family，不把其引用冒充本组完整通过。

所有状态保持 needs_review。具名断言的候选通过不证明能力全覆盖：SDK manifest 固定95ec66c6，八crate临时patch组合Task/ACP/LSP源码用于本地验证；最终Channel后继published revision、移除patch后的独立构建、精确main和远端CI仍待。完整Rust inventory仅遥测，最终accepted字段/退役必须由唯一生成器明确裁决。

所有资源种类的并发publication/close I/O故障与多Session state隔离尚未逐种证明；framework父future drop的pre-join/driver准入新反例正在修复，旧Task23命令绿色或局部prejoin绿色不是当前最终权威证明。

## 裁决记录

用户明确要求完成八项治理，三语言范围只以明确接受合同为准；已授权子智能体执行，主线程负责协调及验收。

## 当前观察、归档和后继边界

原fc63证据已保存为仓库内不可变历史收据，原观察epoch不可恢复为Git时不补造提交。原6711证据归档支持具名局部断言；原27个历史输入中tools.rs已变，具名测试body保持可对照；55个6711运行输入中tools.rs和Workflow已变更。新Workflow语义或其它后继场景必须绑定自己的来源/日志；不把旧整套418/TS240/Python300/Java264冒充最终当前通过。支持见历史审计Evidence及evidence.sdk-parent-driver-candidate-verification的限制，所有needs_review/开放Finding保留。

## 当前模型重观察范围

review_scope：本对象在真实当前候选Git `56929154c1851473e9e4837901b610cecf001b2d`（源码摘要`17cf243e8d6341db0ce23c5b9ee39977ce4ab100d6ed1b17ec5a7bb26c19e138`）上重新观察。上列code_refs/各scenario source_refs已逐Gitblob读取并与当前文件SHA精确相同，原锚点均存在。模型继续描述原accepted入口、框架业务/终态权威、Host owner/generation与transport消费职责、生命周期及具名失败义务；源码定位核对不证明所有行为正确、运行证据fresh或合同生成闭合。

原Behavior/status、Map场景mapped/needs_review、Rule/Asset处置、Finding/未决义务与Evidence原epoch支持范围全部保留。mapped仅入口模型，旧3ea不是本snapshot本体；历史receipt不得因本模型观察更新冒current运行。12source迁移、5秒物理结算与依赖50warning规则三选择仍未接受，完整生成/CI/main另验，任何新Rustwrapper不自动扩三语言合同。

## 当前ACP/AgentSessionRun与A有界收口

本对象恢复真实8763源码/资料，frameworkFinal本session独立8lens源审与实际native反例/修复来源通过后准确回收，Hosthandlerac5/handles8b89/state8e3/modc766/factoryda354保一authority。old193e790、Steer原eb兼容前证明与newdaA三native13input/五组及2officialRPC取消、pub9等各保原cut；不是模型writer新审或allgoalmain通过。A同key完整publication响应/owner+gen+payloadfingerprint、失败前effects释放retry、成功后原job保持/closed不复活具局部三链，A具体Finding可resolved，framework仍唯一execution/terminal，不新idempo scheduler。

菜单校准：CoreProfile扩展Session只有create/load/close；officialsession_list是协议inventory而非已advertise运行入口，save为自动state/journal/receiptprojection不新Session.save。Runget是snapshot，start返回streamhandle/eventfeed并replay/ACK；RunHandle.close用canonical cancel/wait后关闭本地订阅（TS仅非终态，PY/Java尝试取消并等）；没有Hostrunclose且不直接退役原HostRun。原waitshortsettledfalse不cancel、ACK仅initiation不是physicalcompleted。

本组Behavior保持needs_review，scene仅mapped/8Auditexamined。新Javaversion溢出、malformedarray/limits shape的协商缺陷是另具体openFinding，不被本Hostsource审PASS吞并，完整旧whole在其client变更后为历史；12sourcekind/5sB/平台/fullgeneration/main独立。B宽真实physicalfalse仍open，合作close/EOF不反证它；当前有限场景不是全部未定义组合全测。


## 2026-10-20 具名有限结算

本项verified只限原声明的具名合同单元及下列直接证据，非全SDK/平台或任意未列组合。原模型header observed_at为`8763c67bcfd6d98fe57449c88bb93f28f32bf78f`，原正文及Evidence各自runtime cut保留；当前模型观察绑定公开`04eb6ad50982656b5a8b8ea649619b6a383065ff`，不把旧测试重标为新Host执行。

直接证据：`evidence.sdk-acp-agent-run-finite-observation`、`evidence.sdk-model-read-retained-verification`。

具体复审入口：`audit.sdk-acp-agent-run-finite-state-authority`、`audit.sdk-model-read-state-rereview`；只读者已按这一个named单元核权威、生命周期、失败与证据边界，不能由共享对象推整八风险组。

源码后继分开核：原Root17be/0bbc具名review与04eb薄增量分别C0。04eb相对17be只在media lookup保两accepted名字到原Tool，三测试wrapper新增可选媒体故事，原fixture五函数/默认HTTP/旧oracles不变；本项所依非媒体生产路径不因共有文件名改变而假报重审或全套新运行。新媒体after另由a17/三语focused证明，未借旧208a14为新源码完整资格。

支持范围仍限原named consumer/字段/副作用及其明确失败；未接受identity/任意provider、codec、选项或竞态组合不自动加入分母。
