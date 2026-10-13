---
schema_version: 1
id: behavior.sdk-family-files
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
- evidence.sdk-ordinary-tools-b909-verification
- evidence.sdk-tool-value-natural-eof-after-candidate
finding_refs:
- finding.sdk-tool-artifact-context-propagation
---

# SDK family files

## 重要承诺

对已接受 `files` 能力明确入口、唯一权威、状态/生命周期、失败处置和直接行为证据；未接受 Rust identity 不进入承诺。

## 当前行为

- **入口/可达**：`facade/tools.rs` framework files tools；source FileAuthority guard/
  lease 在 `source_operations.rs`，Session cwd 进入真实 ToolContext。
- **权威/状态**：文件系统和 framework tool/file authority；Host lease handle/address。
  read/write/patch/move/delete 与 open guard → consume → close；文件身份必须保留。
- **正常/失败**：返回内容/diff/effect/artifact 与磁盘一致；nonexistent path、invalid patch、
  stale lease/concurrent write、partial side effect、quota 和 close/reacquire。
- **已有断言**：Core relative write/read 内容；source file tests 读取磁盘 bytes、guard 长度、
  lease 冲突、重复 close false、reacquire；tool DTO unit 保留 effects/failure/model_content。
- **最低充分证据**：accepted read/search/list/write/append/update/create/delete/move/patch
  按操作结果与实际 filesystem 验证，Unicode/non-UTF8 path、stale identity、失败无虚假
  complete effect，三语言 typed effect/partial failure；格式/contract 不替代实际文件证据。

## 期望行为

上述正常/失败场景须保持接受范围、完整转换和唯一权威，并在最终源码/pin证据及独立复审后闭合。

## 触发、结果与副作用

按上方具体入口和正常/失败记录检查真实主路径。可达性或catalog状态不能证明副作用、错误或结算。

## 失败、重试与恢复

按上方生命周期和最低充分证据检查取消、超时、错误、释放和恢复；不默认成功，不以其它能力族测试替代。

## 证据

阶段：`candidate_runtime_verified`；语义状态仍为 `needs_review`。

真实相对 write/read 内容；source disk bytes、file guard/lease冲突、重复close/reacquire；ToolResult projection 保留 partial effects/artifact/model_content。

实际来源：`echo-sdk-host/tests/core_profile_e2e.rs#tool_families_execute_framework_tools_with_the_session_cwd`；`echo-sdk-host/tests/core_profile_e2e.rs#source_files_and_skill_registry_preserve_rust_authority`；`echo-sdk-host/src/core_profile/facade/tools.rs#tool_family_projection_preserves_partial_effects_and_artifact`。

仍缺：全部 accepted filesystem verbs、non-UTF8跨平台失败及三语言副作用一致性仍缺。 published framework pin、最后变更后的独立复审和精确main验收
仍待；本证据不证明未列出的生命周期、字段或错误组合，也不自动扩大accepted合同。

## 裁决记录

用户要求SDK全能力行为治理，以明确接受跨语言合同为兼容边界；完整Rust inventory仅遥测。

## 当前观察与历史收据边界

Candidate assertion evidence is now preserved as original receipt/source fingerprints and checked against inspected current paths. It supports only named partial cases; current observation binding neither reruns old commands nor upgrades family status.

## Tool validator变化限制

当前SDK direct Tool路径在旧6711以后改变，改用同一framework validator并拒绝超额arity。旧候选运行仅保留在原来源历史审计，不证明新生产路径的完整行为；需结合新增validator具体反例与最终当前门禁，状态不提升。

## 本次有限审查单元

行为/生命周期单元：frameworkcwd/FileAuthority读写原字节与owner/lease。

剩余处置：指定读写/cwd/lease支持；格式化权限/文件保护由现framework，同用户本地交互不新权限闸。

本项建模阈值是已接受入口、唯一状态权威、正常生命周期与具名失败/恢复路径可定位，并有直接证据或明确开放原因；不要求所有未定义组合全测，也不由绿色测试自动verified。
