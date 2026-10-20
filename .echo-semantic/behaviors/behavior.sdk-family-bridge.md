---
schema_version: 1
id: behavior.sdk-family-bridge
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
boundary: boundary.sdk-extension-bridge
observed_at: 5bcbad2437e5c19f61101883044c4f67944cb97c
code_refs:
- echo-sdk-host/src/core_profile/extension_bridge.rs
- echo-sdk-host/src/core_profile/handler.rs
- echo-sdk-host/src/factory.rs
- echo-sdk-protocol/src/methods.rs
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-candidate-behavior-verification
- evidence.sdk-parent-driver-candidate-verification
- evidence.sdk-managed-retirement
- evidence.sdk-llm-workflow-stream
- evidence.sdk-python-typed-component-stream-verification
- evidence.sdk-managed-supporting-operation-boundary
- evidence.sdk-eval-improve-topology
- evidence.sdk-search-sandbox-06bb
- evidence.sdk-channel-cooperative-two
- evidence.sdk-extension-teardown-prefix-verification
- evidence.sdk-stream-cancellation-routing-verification
- evidence.sdk-factory-child-settlement-verification
- evidence.sdk-tools-bridge-finite-observation
finding_refs:
- finding.sdk-python-agent-component-stream-projection
- finding.sdk-sandbox-canonical-stream-routing
- finding.sdk-custom-agent-invocation-cancel-projection
- finding.sdk-channel-external-callback-cooperative-close
- finding.sdk-extension-teardown-stream-prefix
- finding.sdk-task-resume-extension-invocation-conflict
- finding.sdk-factory-terminal-close-settlement
---

# SDK family bridge

## 重要承诺

对已接受 `bridge` 能力明确入口、唯一权威、状态/生命周期、失败处置和直接行为证据；未接受 Rust identity 不进入承诺。

## 当前行为

- **入口/可达**：typed extension register/unregister → `extension_bridge.rs`，factory 把
  proxy 注入 live Session Agent；invoke/cancel/stream 反向走同一 official connection。
- **权威/状态**：Host lease/owner、deadline、pending mailbox/stream；framework Tool、
  LlmClient、Store/hook/callback 消费与 commit/terminal。descriptor validate → registered
  → captured by Session → pending invocation/stream → result/error/cancel → unregister。
- **正常/失败**：kind/operation/signature mismatch、late/duplicate result、callback 重入、
  unregister failure、initial stream ACK timeout、disconnect、no stream terminal、managed
  persistence outcome unknown；取消不能撤销已提交事实或提前释放 cleanup owner。
- **已有断言**：Bridge tool/callback/hook、Store/human-loop、LLM streaming/nonstreaming、
  malformed-kind/out-of-order/oversized/flood、deadline/cancel/disconnect；新增 managed
  commit/deadline/restart 及 pending stream owner tests 已在所列候选运行；未覆盖每个 accepted component 的全部合法消费与失败结算，最终 pin 仍需验收。
- **最低充分证据**：对每个 accepted bridge kind/operation 至少合法消费和失败结算，验证
  Session 捕获的 proxy、factory instance locality、ToolEffect/full receipt 字段，store
  deadline-before/after commit 与 lost ACK/restart 同一 receipt；三语言 callback reentry、
  exception、cancel、disconnect，测 callback owner 释放和未完成调用数归零。

## 期望行为

上述正常/失败场景须保持接受范围、完整转换和唯一权威，并在最终源码/pin证据及独立复审后闭合。

## 触发、结果与副作用

按上方具体入口和正常/失败记录检查真实主路径。可达性或catalog状态不能证明副作用、错误或结算。

## 失败、重试与恢复

按上方生命周期和最低充分证据检查取消、超时、错误、释放和恢复；不默认成功，不以其它能力族测试替代。

## 证据

阶段：`candidate_runtime_verified`；语义状态仍为 `needs_review`。

34 项 Bridge 测试实际执行（含准入拒绝场景）；Tool/hook/callback、LLM stream、factory 和 managed store 的指定路径消费及错误结算；deadline-before-work 无 mutation、lost ACK/重启同 receipt、初始 ACK 被取消后释放 callback owner。三语言 managed/callback/late factory 场景亦实际执行。

实际来源：`echo-sdk-host/tests/extension_bridge_e2e.rs#tool_bridge_round_trip_with_callbacks_intervention_and_hook`；`echo-sdk-host/tests/extension_bridge_e2e.rs#managed_persistence_deadline_and_lost_ack_preserve_commit_authority`；`echo-sdk-host/tests/extension_bridge_e2e.rs#managed_commit_disconnect_restart_replays_same_receipt`；`echo-sdk-host/tests/extension_bridge_e2e.rs#cancelled_initial_stream_response_releases_session_callback_owner`。

原阶段缺口（历史，后继finite范围见下）：接受的每个 component operation 的合法消费/异常结算尚未逐项覆盖；11 DTO corpus 不等于11实际持久化动作。 published framework pin、最后变更后的独立复审和精确main验收
仍待；本证据不证明未列出的生命周期、字段或错误组合，也不自动扩大accepted合同。

## 裁决记录

用户要求SDK全能力行为治理，以明确接受跨语言合同为兼容边界；完整Rust inventory仅遥测。

## 当前观察与历史收据边界

Candidate assertion evidence is now preserved as original receipt/source fingerprints and checked against inspected current paths. It supports only named partial cases; current observation binding neither reruns old commands nor upgrades family status.

## 本次有限审查单元

行为/生命周期单元：descriptor/注册撤销、reverse invocation/stream/cancel/deadline及factory child身份。

剩余处置：managed第八个真实retirement consumer与Unsupported零突变已三语言2场景/20backend通过；三逆DTO已核actualRuststabledeletecaller与当前forwardaccepted边界，保supporting逆schema/RustHostonly，不强新增API。Python typed component stream与新approved Channelforward分别保具体缺陷/计划，不再以11DTO计数证明全部操作。

本项建模阈值是已接受入口、唯一权威、生命周期及具名失败可定位，直接证据或明确开放原因可复核；mapped不代表verified，不要求未定义的任意组合。

## 当前模型重观察范围

review_scope：本对象在真实当前候选Git `56929154c1851473e9e4837901b610cecf001b2d`（源码摘要`17cf243e8d6341db0ce23c5b9ee39977ce4ab100d6ed1b17ec5a7bb26c19e138`）上重新观察。上列code_refs/各scenario source_refs已逐Gitblob读取并与当前文件SHA精确相同，原锚点均存在。模型继续描述原accepted入口、框架业务/终态权威、Host owner/generation与transport消费职责、生命周期及具名失败义务；源码定位核对不证明所有行为正确、运行证据fresh或合同生成闭合。

原Behavior/status、Map场景mapped/needs_review、Rule/Asset处置、Finding/未决义务与Evidence原epoch支持范围全部保留。mapped仅入口模型，旧3ea不是本snapshot本体；历史receipt不得因本模型观察更新冒current运行。12source迁移、5秒物理结算与依赖50warning规则三选择仍未接受，完整生成/CI/main另验，任何新Rustwrapper不自动扩三语言合同。

## 当前内置工具与反向桥有限审查

本对象归入builtin10families或原bridge角色，原currentTool实际source8lens与远端3合法注册consumer原因修复独立PASS记录于sharedEvidence及8Audits。runtime69b与真实5bcb所选source相同，原tool/typedvalue/FieldEffect/PID/error具名scope沿原family表保存，author只回收原业务审查，不把catalogdone/原whole计数当证据。新A独切片不在本scope，old原阶段待办保历史不会冒新runtime缺陷。

入口工具经唯一frameworkschema/customvalidator/ToolManager；FileAuthority/Polars/Pivot/provider各原authority不复制。Bridge descriptor/kind/gen注册后实际注入Session，ToolContext/artifact、Llm messages/usage/finish、Store namespace/commit、Hook/callback/intervention保持原owner/失败策略。remoteTool Cancelled/extension_timeout经ToolFailure::from_error按risk保分类与副作用，LLM Cancelled经AgentCancelled，合法旧3fail→samebody3pass callerTokenfalse/lease0，初坏descriptor是fixture错误不生产反例。

本对象needs_review不提升verified；八lens只具名examined。network原时点/内存Rag非crossprocess/未acceptcodec或constructor具体不承诺、5sphysicalB/12sourcekind/最终新源wholemain保独立。原ToolValue/Rag取证source保4feb taggedGit；FileAuthority补包若当前尚未commit其实际原件不假5bcbsourceRef，finite审scope不变，正式交付另逐包绑定。宽feature-failure-cause仅Tool/LLM子scope已修复，原Workflow/Memory/Eval其它义务未自动关。


## 2026-10-20 具名有限结算

本项整体继续needs_review。原具名子链已有后继并不授整个composite或媒体最终资格；保原header观察版本和历史runtime cut，当前04eb源码定位及受控增量另记。

直接证据：`evidence.sdk-candidate-behavior-verification`、`evidence.sdk-parent-driver-candidate-verification`、`evidence.sdk-managed-retirement`、`evidence.sdk-llm-workflow-stream`、`evidence.sdk-python-typed-component-stream-verification`、`evidence.sdk-managed-supporting-operation-boundary`、`evidence.sdk-eval-improve-topology`、`evidence.sdk-search-sandbox-06bb`、`evidence.sdk-channel-cooperative-two`、`evidence.sdk-extension-teardown-prefix-verification`、`evidence.sdk-stream-cancellation-routing-verification`、`evidence.sdk-factory-child-settlement-verification`、`evidence.sdk-tools-bridge-finite-observation`。

具体复审入口：`audit.sdk-tools-bridge-finite-contract-evidence`、`audit.sdk-tools-bridge-finite-data-durability`、`audit.sdk-tools-bridge-finite-failure-concurrency`、`audit.sdk-tools-bridge-finite-permission-external`、`audit.sdk-tools-bridge-finite-result-side-effect`、`audit.sdk-tools-bridge-finite-state-authority`、`audit.sdk-tools-bridge-finite-time-lifecycle`、`audit.sdk-tools-bridge-finite-trigger-input`；只读者已按这一个named单元核权威、生命周期、失败与证据边界，不能由共享对象推整八风险组。

源码后继分开核：原Root17be/0bbc具名review与04eb薄增量分别C0。04eb相对17be只在media lookup保两accepted名字到原Tool，三测试wrapper新增可选媒体故事，原fixture五函数/默认HTTP/旧oracles不变；本项所依非媒体生产路径不因共有文件名改变而假报重审或全套新运行。新媒体after另由a17/三语focused证明，未借旧208a14为新源码完整资格。

已知reversecallback与owner链可承接；managed历史Linux/main/Hook未结，不授整个bridge最终资格。
