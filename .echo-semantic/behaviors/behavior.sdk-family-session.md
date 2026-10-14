---
schema_version: 1
id: behavior.sdk-family-session
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
boundary: boundary.sdk-agent-session-run
observed_at: 8763c67bcfd6d98fe57449c88bb93f28f32bf78f
code_refs:
- echo-sdk-host/src/core_profile/handler.rs
- echo-sdk-host/src/core_profile/handles.rs
- echo-sdk-host/src/core_profile/state.rs
- echo-sdk-host/src/core_profile/persistence.rs
- echo-sdk-host/src/factory.rs
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-candidate-behavior-verification
- evidence.sdk-parent-driver-candidate-verification
- evidence.sdk-acp-agent-run-finite-observation
finding_refs: []
---

# SDK family session

## 重要承诺

对已接受 `session` 能力明确入口、唯一权威、状态/生命周期、失败处置和直接行为证据；未接受 Rust identity 不进入承诺。

## 当前行为

- **入口/可达**：typed `session/create|load|close` → `handler.rs`、`factory.rs`；标准 ACP
  Session 与扩展 handle 通过同一 connection services 桥接，见 Core standard-prompt 测试。
- **权威/状态**：framework live Session Agent、conversation/checkpoint store；Host Session
  handle/recovery mapping。create/load → live → closing → cleanup settled → release；重启新 generation。
- **正常/失败**：cwd、history、memory/task/services 绑定准确 Session；未知/跨 owner Session、
  load 损坏/缺失持久化、create publication 失败、close 超时/重试/并发不能返回假成功。
- **已有断言**：Core `extended_standard_prompt_bridges_shared_core_handles` 与 restart
  tests；三语言 protocol-host 两 Session ID 不同，关闭 one 后 two 的 Agent name 调用成功。
- **最低充分证据**：相同 Agent 下两个 Session 写 memory/执行 Task、关闭其中一个、重启
  load history；比对隔离状态、旧 generation rejection、close debt retry 和资源配额复用。
  Agent name 成功只证明 sibling Session 仍可访问，不证明所有核心状态隔离。

## 期望行为

上述正常/失败场景须保持接受范围、完整转换和唯一权威，并在最终源码/pin证据及独立复审后闭合。

## 触发、结果与副作用

按上方具体入口和正常/失败记录检查真实主路径。可达性或catalog状态不能证明副作用、错误或结算。

## 失败、重试与恢复

按上方生命周期和最低充分证据检查取消、超时、错误、释放和恢复；不默认成功，不以其它能力族测试替代。

## 证据

阶段：`candidate_runtime_verified`；语义状态仍为 `needs_review`。

标准 prompt 与扩展 Session 共享对象；历史 native turn7 后 create/load 都精确分配 turn8；畸形身份报 create/load 实际 operation，重复失败无句柄配额泄漏。禁用 reservation 的真实反例两路径均产生 turn1。

实际来源：`echo-sdk-host/tests/core_profile_e2e.rs#extended_standard_prompt_bridges_shared_core_handles`；`echo-sdk-host/tests/core_profile_e2e.rs#explicit_session_create_reserves_persisted_turn_identity_before_new_run`；`echo-sdk-host/tests/core_profile_e2e.rs#session_load_reserves_persisted_turn_identity_before_new_run`；`echo-sdk-host/tests/core_profile_e2e.rs#malformed_recovered_identity_rejects_actual_method_without_handle_publication`。

原阶段缺口（历史，当前有界范围见下）：两 Session 的全部 memory/Task 状态隔离、持久化 I/O fault 和 close debt 的三语言共同场景仍未穷尽。 published framework pin、最后变更后的独立复审和精确main验收
仍待；本证据不证明未列出的生命周期、字段或错误组合，也不自动扩大accepted合同。

## 裁决记录

用户要求SDK全能力行为治理，以明确接受跨语言合同为兼容边界；完整Rust inventory仅遥测。

## 当前观察与历史收据边界

Candidate assertion evidence is now preserved as original receipt/source fingerprints and checked against inspected current paths. It supports only named partial cases; current observation binding neither reruns old commands nor upgrades family status.

## 本次有限审查单元

行为/生命周期单元：create/load/标准prompt共用framework Session、原生turn恢复identity与同owner清理。

剩余处置：持久化错误和Sessionclosing回调重入的新明确窗口；不外推任意resource组合。

本项建模阈值是已接受入口、唯一状态权威、正常生命周期与具名失败/恢复路径可定位，并有直接证据或明确开放原因；不要求所有未定义组合全测，也不由绿色测试自动verified。

## 当前模型重观察范围

review_scope：本对象在真实当前候选Git `56929154c1851473e9e4837901b610cecf001b2d`（源码摘要`17cf243e8d6341db0ce23c5b9ee39977ce4ab100d6ed1b17ec5a7bb26c19e138`）上重新观察。上列code_refs/各scenario source_refs已逐Gitblob读取并与当前文件SHA精确相同，原锚点均存在。模型继续描述原accepted入口、框架业务/终态权威、Host owner/generation与transport消费职责、生命周期及具名失败义务；源码定位核对不证明所有行为正确、运行证据fresh或合同生成闭合。

原Behavior/status、Map场景mapped/needs_review、Rule/Asset处置、Finding/未决义务与Evidence原epoch支持范围全部保留。mapped仅入口模型，旧3ea不是本snapshot本体；历史receipt不得因本模型观察更新冒current运行。12source迁移、5秒物理结算与依赖50warning规则三选择仍未接受，完整生成/CI/main另验，任何新Rustwrapper不自动扩三语言合同。

## 当前ACP/AgentSessionRun与A有界收口

本对象恢复真实8763源码/资料，frameworkFinal本session独立8lens源审与实际native反例/修复来源通过后准确回收，Hosthandlerac5/handles8b89/state8e3/modc766/factoryda354保一authority。old193e790、Steer原eb兼容前证明与newdaA三native13input/五组及2officialRPC取消、pub9等各保原cut；不是模型writer新审或allgoalmain通过。A同key完整publication响应/owner+gen+payloadfingerprint、失败前effects释放retry、成功后原job保持/closed不复活具局部三链，A具体Finding可resolved，framework仍唯一execution/terminal，不新idempo scheduler。

菜单校准：CoreProfile扩展Session只有create/load/close；officialsession_list是协议inventory而非已advertise运行入口，save为自动state/journal/receiptprojection不新Session.save。Runget是snapshot，start返回streamhandle/eventfeed并replay/ACK；RunHandle.close用canonical cancel/wait后关闭本地订阅（TS仅非终态，PY/Java尝试取消并等）；没有Hostrunclose且不直接退役原HostRun。原waitshortsettledfalse不cancel、ACK仅initiation不是physicalcompleted。

本组Behavior保持needs_review，scene仅mapped/8Auditexamined。新Javaversion溢出、malformedarray/limits shape的协商缺陷是另具体openFinding，不被本Hostsource审PASS吞并，完整旧whole在其client变更后为历史；12sourcekind/5sB/平台/fullgeneration/main独立。B宽真实physicalfalse仍open，合作close/EOF不反证它；当前有限场景不是全部未定义组合全测。
