---
schema_version: 1
id: behavior.sdk-family-permission
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
boundary: boundary.sdk-integrations
observed_at: source:3ea544c47e331b5d29a6624f978f3595619bef39fa5985230351098d38930404
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
- evidence.sdk-memory-permission-candidate
finding_refs: []
---

# SDK family permission

## 重要承诺

对已接受 `permission` 能力明确入口、唯一权威、状态/生命周期、失败处置和直接行为证据；未接受 Rust identity 不进入承诺。

## 当前行为

- **入口/可达**：`facade/permission.rs::dispatch`/`service_of` 获取 live Session Agent
  `PermissionService`，agent run 工具权限路径应消费同一 service。
- **权威/状态**：framework PermissionService/rules/approval cache；Host 不新增 policy。
  mode/rules update → exact invocation check → approval/cache expiry/revoke；按 Session 隔离。
- **正常/失败**：allow/deny/ask/require_approval 与 context、effect binding；非法 mode/
  permission/rule、ttl/time、invalid cache target、callback failure；不添加本地交互权限闸。
- **已有断言**：Core source Session permission_mode setter/getter；Bridge human-loop 消费。
  这些不覆盖 family 的全套 rule/cache/context 更新行为。
- **最低充分证据**：family add/remove rules 后 live tool 相同 effect decision、mode check
  variants 与 exact context、expiry/revoke/cleanup；两 Session rule/cache 隔离，失败更新无
  半写；三语言保留 ask suggestions 与 matcher/source/TTL，不用仅 enum 解析做权威证明。

## 期望行为

上述正常/失败场景须保持接受范围、完整转换和唯一权威，并在最终源码/pin证据及独立复审后闭合。

## 触发、结果与副作用

按上方具体入口和正常/失败记录检查真实主路径。可达性或catalog状态不能证明副作用、错误或结算。

## 失败、重试与恢复

按上方生命周期和最低充分证据检查取消、超时、错误、释放和恢复；不默认成功，不以其它能力族测试替代。

## 证据

阶段：`candidate_runtime_verified`；语义状态仍为 `needs_review`。

source permission_mode setter/getter 和真实 HumanLoop 回调消费；部分 permission DTO unit 的未知 rule/source 拒绝实际执行。

实际来源：`echo-sdk-host/tests/core_profile_e2e.rs#negotiated_facade_admission_fails_closed`；`echo-sdk-host/tests/extension_bridge_e2e.rs#store_and_human_loop_extensions_round_trip`。

仍缺：完整 matcher/cache/context/TTL、两 Session live tool 决策隔离与撤销失败未覆盖。 published framework pin、最后变更后的独立复审和精确main验收
仍待；本证据不证明未列出的生命周期、字段或错误组合，也不自动扩大accepted合同。

## 裁决记录

用户要求SDK全能力行为治理，以明确接受跨语言合同为兼容边界；完整Rust inventory仅遥测。

## 当前观察与历史收据边界

Candidate assertion evidence is now preserved as original receipt/source fingerprints and checked against inspected current paths. It supports only named partial cases; current observation binding neither reruns old commands nor upgrades family status.

## Memory/Permission后继消费者范围

三语言临时原生consumer各59证据记录（52operation结果/2typed失败/11Store回调及model/journal/真正Run副作用）已有共同对照；不是59独立test。正式TS/Python单场景执行与临时Java原来源分开归档。30分钟TTL过期和未接受approval_receipt仍明确缺口，不造短TTL API，状态不提升。

## 本次有限审查单元

行为/生命周期单元：framework规则/cache/context、HumanLoop修改args与真实tool effect。

剩余处置：59rows支持拒写/approvedonce/Session隔离；30minTTL没短setter，保具体未观察原因且不新增配置。

本项建模阈值是已接受入口、唯一状态权威、正常生命周期与具名失败/恢复路径可定位，并有直接证据或明确开放原因；不要求所有未定义组合全测，也不由绿色测试自动verified。
