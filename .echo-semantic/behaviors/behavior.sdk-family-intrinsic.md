---
schema_version: 1
id: behavior.sdk-family-intrinsic
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
- sdks/typescript/src/client.ts
- sdks/typescript/src/wire.ts
- sdks/python/src/echo_agent_sdk/client.py
- sdks/python/src/echo_agent_sdk/wire.py
- sdks/java/src/main/java/com/echoagent/sdk/EchoAgentClient.java
- sdks/java/src/main/java/com/echoagent/sdk/WireValues.java
- echo-sdk-protocol/src/scalar.rs
- echo-sdk-protocol/src/error.rs
- contracts/sdk/accepted-external-contract.json
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-candidate-behavior-verification
- evidence.sdk-parent-driver-candidate-verification
finding_refs: []
---

# SDK family intrinsic

## 重要承诺

对已接受 `intrinsic` 能力明确入口、唯一权威、状态/生命周期、失败处置和直接行为证据；未接受 Rust identity 不进入承诺。

## 当前行为

- **入口/可达**：accepted intrinsic/helper identities 经语言 catalog/helper 或 schema
  消费；family 无远程 method/operation，不应推断 `_echo_agent/intrinsic/op` 可执行。
- **权威/状态**：accepted external contract 决定明确承诺的 helper shape；无 Host runtime
  状态机。完整 inventory intrinsic membership/digest 仅 telemetry，不进入阻断协商。
- **正常/失败**：已接受 helper 的 constructor/value/optional 表达在语言可用；未接受
  Rust intrinsic 不新增 API，不用填默认值伪造 helper 支持。
- **已有断言**：Protocol facade inventory 与三语言 catalog tests 主要核对声明及映射。
- **最低充分证据**：列出 accepted helper 的实际消费位置并执行语言 value round-trip；
  contract gate 证明未接受 intrinsic 漂移不阻断 accepted handshake。catalog 对齐本身不
  是全部 helper 行为完成；不恢复冻结完整 intrinsic membership 摘要。

## 期望行为

上述正常/失败场景须保持接受范围、完整转换和唯一权威，并在最终源码/pin证据及独立复审后闭合。

## 触发、结果与副作用

按上方具体入口和正常/失败记录检查真实主路径。可达性或catalog状态不能证明副作用、错误或结算。

## 失败、重试与恢复

按上方生命周期和最低充分证据检查取消、超时、错误、释放和恢复；不默认成功，不以其它能力族测试替代。

## 证据

阶段：`candidate_runtime_verified`；语义状态仍为 `needs_review`。

实际语言-local ToolCallParams/ToolResult 构造、typed getters、failure recovery、不可变 modifier 与部分 helper 执行；done/catalog assertions 仅作声明检查，不支持行为完成。

实际来源：`sdks/typescript/test/intrinsic-tool-values.test.js`；`sdks/typescript/test/helpers.test.js`；`sdks/java/src/test/java/com/echoagent/sdk/IntrinsicToolValueTest.java`。

仍缺：全部 accepted intrinsic helpers 的逐项消费和三语言精确结果对比未完成，不冻结完整 intrinsic membership。 published framework pin、最后变更后的独立复审和精确main验收
仍待；本证据不证明未列出的生命周期、字段或错误组合，也不自动扩大accepted合同。

## 裁决记录

用户要求SDK全能力行为治理，以明确接受跨语言合同为兼容边界；完整Rust inventory仅遥测。

## 当前观察与历史收据边界

Candidate assertion evidence is now preserved as original receipt/source fingerprints and checked against inspected current paths. It supports only named partial cases; current observation binding neither reruns old commands nor upgrades family status.

## 本次有限审查单元

行为/生命周期单元：明确接受的客户端本地helper构造/getter/不可变modifier。

剩余处置：逐接受helper覆盖矩阵，完整Rustintrinsic membership不冻结、不作分母。

本项建模阈值是已接受入口、唯一状态权威、正常生命周期与具名失败/恢复路径可定位，并有直接证据或明确开放原因；不要求所有未定义组合全测，也不由绿色测试自动verified。
