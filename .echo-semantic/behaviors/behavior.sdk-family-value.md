---
schema_version: 1
id: behavior.sdk-family-value
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
boundary: boundary.sdk-language-clients
observed_at: source:3ea544c47e331b5d29a6624f978f3595619bef39fa5985230351098d38930404
code_refs:
- echo-sdk-protocol/src/scalar.rs
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-candidate-behavior-verification
- evidence.sdk-parent-driver-candidate-verification
finding_refs:
- finding.sdk-accepted-source-continuity
---

# SDK family value

## 重要承诺

对已接受 `value` 能力明确入口、唯一权威、状态/生命周期、失败处置和直接行为证据；未接受 Rust identity 不进入承诺。

## 当前行为

- **入口/可达**：`echo-sdk-protocol/src/scalar.rs` 及 accepted schema/value routes；Host
  `wire.rs`，TS/Python wire、Java `WireValues.java`/`JsonSupport.java` 的 decode/encode。
- **权威/状态**：accepted DTO 拥有值合同；framework 原字段保持业务事实。无独立 Run/
  store；encoded → validated → projected → round-trip，不能把不支持的值变成 null/default。
- **正常/失败**：u64/i64、纳秒时间/duration、paths/OS encoding、bytes、Unicode、null vs
  absent、variant/record/map duplicates、float limits、opaque handles；每条转换错误可定位。
- **已有断言**：Wire `managed_checkpoint_and_receipts_preserve_large_revisions_and_subseconds`、
  tool effects/model content tests；三语言 protocol-integrity 消费 shared malformed corpus，
  special keys/中文🙂、opaque error handle、missing response envelope。
- **最低充分证据**：同一 accepted field corpus 作三语言 → Host → framework → Host round-trip，
  对超 JS safe integer、subsecond time、非 UTF-8/Windows path 的允许/拒绝边界、invalid
  optional 与 unknown discriminant 作精确断言；platform 不可运行时保留未验证项。

## 期望行为

上述正常/失败场景须保持接受范围、完整转换和唯一权威，并在最终源码/pin证据及独立复审后闭合。

## 触发、结果与副作用

按上方具体入口和正常/失败记录检查真实主路径。可达性或catalog状态不能证明副作用、错误或结算。

## 失败、重试与恢复

按上方生命周期和最低充分证据检查取消、超时、错误、释放和恢复；不默认成功，不以其它能力族测试替代。

## 证据

阶段：`candidate_runtime_verified`；语义状态仍为 `needs_review`。

指定 wire tests 保留大 revision、subseconds、result absent/null、non-UTF8 Unix path；三语言 wire/malformed corpus 和 typed error fields 实际执行。

实际来源：`echo-sdk-host/src/core_profile/wire.rs#managed_checkpoint_and_receipts_preserve_large_revisions_and_subseconds`；`echo-sdk-host/src/core_profile/wire.rs#unix_non_utf8_paths_round_trip_without_loss`；`sdks/typescript/test/wire.test.js`；`sdks/python/tests/test_protocol_integrity.py`；`sdks/java/src/test/java/com/echoagent/sdk/ProtocolIntegrityTest.java`。

仍缺：Windows native path 与所有 accepted field 经 Host/framework 的统一 corpus 尚未完成；语言-local 单测不充当 Host round-trip。 published framework pin、最后变更后的独立复审和精确main验收
仍待；本证据不证明未列出的生命周期、字段或错误组合，也不自动扩大accepted合同。

## 裁决记录

用户要求SDK全能力行为治理，以明确接受跨语言合同为兼容边界；完整Rust inventory仅遥测。

## 当前观察与历史收据边界

Candidate assertion evidence is now preserved as original receipt/source fingerprints and checked against inspected current paths. It supports only named partial cases; current observation binding neither reruns old commands nor upgrades family status.

## 本次有限审查单元

行为/生命周期单元：闭合WireValue标量/容器、u64、路径、时间、Unicode、optional/未知wrapper无损。

剩余处置：Windows原生路径平台门禁与全部明确接受字段source生成；不把process-local Rust类型接受到JSON。

本项建模阈值是已接受入口、唯一状态权威、正常生命周期与具名失败/恢复路径可定位，并有直接证据或明确开放原因；不要求所有未定义组合全测，也不由绿色测试自动verified。
