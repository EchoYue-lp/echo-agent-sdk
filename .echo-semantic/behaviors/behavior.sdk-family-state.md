---
schema_version: 1
id: behavior.sdk-family-state
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
boundary: boundary.sdk-integrations
observed_at: 04eb6ad50982656b5a8b8ea649619b6a383065ff
code_refs:
- echo-sdk-host/src/core_profile/facade/mod.rs
- echo-sdk-host/src/core_profile/facade/registry.rs
- echo-sdk-host/src/core_profile/facade/source_operations.rs
- echo-sdk-host/src/core_profile/facade/integrations.rs
- echo-sdk-host/src/core_profile/facade/memory.rs
- echo-sdk-host/src/core_profile/facade/workflow.rs
- echo-sdk-host/src/core_profile/facade/state_delivery.rs
- echo-sdk-host/src/core_profile/facade/tools.rs
- echo-sdk-host/src/core_profile/facade/observability.rs
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-candidate-behavior-verification
- evidence.sdk-parent-driver-candidate-verification
- evidence.sdk-state-trace-delivery-candidate
- evidence.sdk-managed-retirement
- evidence.sdk-managed-supporting-operation-boundary
- evidence.sdk-integrations-finite-observation
- evidence.sdk-model-read-retained-verification
finding_refs: []
---

# SDK family state

## 重要承诺

对已接受 `state` 能力明确入口、唯一权威、状态/生命周期、失败处置和直接行为证据；未接受 Rust identity 不进入承诺。

## 当前行为

- **入口/可达**：`facade/state_delivery.rs::dispatch_state` 直接使用 Host shared
  framework `FileRuntimeStateStore`；Agent/Session runtime 保存应访问同一 service。
- **权威/状态**：framework runtime/checkpoint store；Host 不维护第二 checkpoint JSON。
  save → get/list → runtime/scope clear，commit/identity/generation 保留。
- **正常/失败**：完整 checkpoint save/get、一致 scope clear；invalid scope/revision、
  partial write/corrupt file、并发 save/clear、重启/outcome unknown 不能吞错。
- **已有断言**：Core save ok/list 一个 runtime/get/clear 与 unknown checkpoint operation；
  Bridge managed persistence tests 和 Wire field tests 是其它指定边界，不等同文件路径证明。
- **最低充分证据**：真实 Run checkpoint 与 facade read 同一字段及 journal identity，
  clear scope/重启隔离、file fault 与 atomicity，三语言 accepted checkpoints/commit receipts
  大 revision/subsecond/optional round-trip；不能只断言 save 的 `ok=true`。

## 期望行为

上述正常/失败场景须保持接受范围、完整转换和唯一权威，并在最终源码/pin证据及独立复审后闭合。

## 触发、结果与副作用

按上方具体入口和正常/失败记录检查真实主路径。可达性或catalog状态不能证明副作用、错误或结算。

## 失败、重试与恢复

按上方生命周期和最低充分证据检查取消、超时、错误、释放和恢复；不默认成功，不以其它能力族测试替代。

## 证据

阶段：`candidate_runtime_verified`；语义状态仍为 `needs_review`。

Host checkpoint facade 的 save/list/get/clear 指定断言；真实 managed 事务在 commit 后断连/重启保持 pending projection、AlreadyApplied 复用同 receipt，三语言比较副作用 ordinal 不重复。

实际来源：`echo-sdk-host/tests/core_profile_e2e.rs#state_delivery_and_trace_families_use_framework_services`；`echo-sdk-host/tests/extension_bridge_e2e.rs#managed_commit_disconnect_restart_replays_same_receipt`；`sdks/typescript/test/managed-host.test.js`。

原阶段缺口（历史，本次finite范围见下）：文件后端所有字段、scope clear/I/O fault/atomicity 未由 managed backend 证据替代；11 managed typed calls 不等于全部实际操作。 published framework pin、最后变更后的独立复审和精确main验收
仍待；本证据不证明未列出的生命周期、字段或错误组合，也不自动扩大accepted合同。

## 裁决记录

用户要求SDK全能力行为治理，以明确接受跨语言合同为兼容边界；完整Rust inventory仅遥测。

## 当前观察与历史收据边界

Candidate assertion evidence is now preserved as original receipt/source fingerprints and checked against inspected current paths. It supports only named partial cases; current observation binding neither reruns old commands nor upgrades family status.

## State/Trace/Delivery后继候选

三语言各三场景在cb9执行，九个raw结果/指定DTO及侧作用已对账；对应部分见evidence.sdk-state-trace-delivery-candidate。原执行输入保留，新required Host空白前置条件diff明确，不将内存Delivery恢复当跨进程durable证明，也不外推全局clear或所有Trace事件。状态仍needs_review。

## 本次有限审查单元

行为/生命周期单元：FileRuntimeStateStore checkpoint/scopes/owner索引与cold恢复。

剩余处置：三语言9scene中state部分支持scope隔离/冷恢复；clear不宣全局all-or-none。

本项建模阈值是已接受入口、唯一状态权威、正常生命周期与具名失败/恢复路径可定位，并有直接证据或明确开放原因；不要求所有未定义组合全测，也不由绿色测试自动verified。

## 当前模型重观察范围

review_scope：本对象在真实当前候选Git `56929154c1851473e9e4837901b610cecf001b2d`（源码摘要`17cf243e8d6341db0ce23c5b9ee39977ce4ab100d6ed1b17ec5a7bb26c19e138`）上重新观察。上列code_refs/各scenario source_refs已逐Gitblob读取并与当前文件SHA精确相同，原锚点均存在。模型继续描述原accepted入口、框架业务/终态权威、Host owner/generation与transport消费职责、生命周期及具名失败义务；源码定位核对不证明所有行为正确、运行证据fresh或合同生成闭合。

原Behavior/status、Map场景mapped/needs_review、Rule/Asset处置、Finding/未决义务与Evidence原epoch支持范围全部保留。mapped仅入口模型，旧3ea不是本snapshot本体；历史receipt不得因本模型观察更新冒current运行。12source迁移、5秒物理结算与依赖50warning规则三选择仍未接受，完整生成/CI/main另验，任何新Rustwrapper不自动扩三语言合同。

## 当前集成能力与supporting角色有界收口

原frameworkFinal独立source审查基于790/a634真实runtime，后A2A第一次missing具名consumer另由bd8三native24step/9ops/8HTTP补齐并审PASS。这是两个真实epoch，不把原A2Apendingreview文稿当今天未审，也不伪当时已审。九所选runtime69b在bd8逐Gitblob相同；currenthandles新A变动排除，旧source审不借成latestA或全部Host已验。

具体本family和registry/skill支持角色按sharedEvidence表/8Audit定位行为/唯一authority/lifecycle/失败：MCP/LSP owner/PID/全字段/derived不复活；Channel六forward原2facts/RWclose/peer隔离；EvalImprove cleanup/deadline/workspace；topologynodeedge；Permissionlive/TTL另scope；Statecheckpoint7、TraceRun17/nanotime/bigDuration/JSONLcold；Deliverymemory8ops同实例exactclaim；Testing14纯值controltransport非Mock业务。源码/API/模型不成为第二权威。

Behavior保持needs_review；入口具名mapped/examined不自动41verified、fullaccepted生成/main。Delivery无额外crossprocessdurability，configuredredaction不承诺未知所有secret；registry/skill归既有source_operation/intrinsic，无新family或任意skill执行。newAfinal、5秒physicalB/12identity及平台/main另验，支持范围未审的sourceoperationmethod保具体菜单理由，不以无限所有组合扩大义务。


## 2026-10-20 具名有限结算

本项verified只限原声明的具名合同单元及下列直接证据，非全SDK/平台或任意未列组合。原模型header observed_at为`bd8e42e732588181b7a2589d91e9ffdce02d23e1`，原正文及Evidence各自runtime cut保留；当前模型观察绑定公开`04eb6ad50982656b5a8b8ea649619b6a383065ff`，不把旧测试重标为新Host执行。

直接证据：`evidence.sdk-state-trace-delivery-candidate`、`evidence.sdk-integrations-finite-observation`、`evidence.sdk-model-read-retained-verification`。

具体复审入口：`audit.sdk-integrations-finite-data-durability`、`audit.sdk-retained-identity-rereview`；只读者已按这一个named单元核权威、生命周期、失败与证据边界，不能由共享对象推整八风险组。

源码后继分开核：原Root17be/0bbc具名review与04eb薄增量分别C0。04eb相对17be只在media lookup保两accepted名字到原Tool，三测试wrapper新增可选媒体故事，原fixture五函数/默认HTTP/旧oracles不变；本项所依非媒体生产路径不因共有文件名改变而假报重审或全套新运行。新媒体after另由a17/三语focused证明，未借旧208a14为新源码完整资格。

支持范围仍限原named consumer/字段/副作用及其明确失败；未接受identity/任意provider、codec、选项或竞态组合不自动加入分母。
