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
observed_at: source:3ea544c47e331b5d29a6624f978f3595619bef39fa5985230351098d38930404
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

仍缺：接受的每个 component operation 的合法消费/异常结算尚未逐项覆盖；11 DTO corpus 不等于11实际持久化动作。 published framework pin、最后变更后的独立复审和精确main验收
仍待；本证据不证明未列出的生命周期、字段或错误组合，也不自动扩大accepted合同。

## 裁决记录

用户要求SDK全能力行为治理，以明确接受跨语言合同为兼容边界；完整Rust inventory仅遥测。

## 当前观察与历史收据边界

Candidate assertion evidence is now preserved as original receipt/source fingerprints and checked against inspected current paths. It supports only named partial cases; current observation binding neither reruns old commands nor upgrades family status.

## 本次有限审查单元

行为/生命周期单元：descriptor/注册撤销、reverse invocation/stream/cancel/deadline及factory child身份。

剩余处置：managed第八个真实retirement consumer与Unsupported零突变已三语言2场景/20backend通过；三逆DTO已核actualRuststabledeletecaller与当前forwardaccepted边界，保supporting逆schema/RustHostonly，不强新增API。Python typed component stream与新approved Channelforward分别保具体缺陷/计划，不再以11DTO计数证明全部操作。

本项建模阈值是已接受入口、唯一权威、生命周期及具名失败可定位，直接证据或明确开放原因可复核；mapped不代表verified，不要求未定义的任意组合。
