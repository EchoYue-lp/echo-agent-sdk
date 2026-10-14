---
schema_version: 1
id: behavior.sdk-family-testing
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
observed_at: 56929154c1851473e9e4837901b610cecf001b2d
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
- evidence.sdk-testing-value-candidate
finding_refs: []
---

# SDK family testing

## 重要承诺

对已接受 `testing` 能力明确入口、唯一权威、状态/生命周期、失败处置和直接行为证据；未接受 Rust identity 不进入承诺。

## 当前行为

- **入口/可达**：accepted external contract14个field identity，均为surface=value、无method/operation；
  family operations=[]。TOOL_FAMILIES排除testing，不创建远程Mock执行器。
- **权威/状态**：三语言公共WireValue codec拥有接受payload的投影/拒绝；framework保留Rust
  Mock对象执行。纯值生命周期，不拥有Store、Run、terminal或SDK测试调度器。
- **正常/失败**：12个positive payload与7个generic scalar/container invalid case，Unicode/
  integer/duration/optional/opaque wrapper不丢字段；unknown wrapper保留不扩大接受范围。
- **已有断言**：TS/Python公共codec各19 tests真实通过；Java公共WireValues.value和
  EchoAgentClient.invoke经controlled official ACP transport按相同expected JSON执行2 JUnit
  test内12+7断言；该transport只回显值，不是Host/Mock runtime。
- **最低充分证据**：accepted scope与14个field payload对应、三语言精确投影/错误及最后source
  对账。不存在testing op的Host拒绝行为须另有实际场景；catalog done不是运行证据。


## 期望行为

上述正常/失败场景须保持接受范围、完整转换和唯一权威，并在最终源码/pin证据及独立复审后闭合。

## 触发、结果与副作用

按上方具体入口和正常/失败记录检查真实主路径。可达性或catalog状态不能证明副作用、错误或结算。

## 失败、重试与恢复

按上方生命周期和最低充分证据检查取消、超时、错误、释放和恢复；不默认成功，不以其它能力族测试替代。

## 证据

阶段：`candidate_value_verified`；语义状态仍为 `needs_review`。

14已接受field payload、12positive+7invalid corpus在TS/PY各19test与Java2test内部12+7断言
实际通过，零fail/skip。`evidence.sdk-testing-value-candidate`归档source/log/XML及TS/PY raw
projection/rejection，Java仅实际same-expected断言，没有伪造raw transcript。

当前归档逐文件hash与实测inputs一致，整仓来源演进不代表该输入经过新wholemain gate。
本组无真实Host Mock runtime，无testing method/operation；源类型没有serde codec，因此
payload包装不证明Rust mock对象重建。generic value rejection不证明Mock专属validator。
未增加不存在op的Host拒绝运行证据；最终正式pin/签名、独立复审、精确main仍待。

## 裁决记录

用户要求SDK全能力行为治理，以明确接受跨语言合同为兼容边界；完整Rust inventory仅遥测。

## 当前观察与历史收据边界

Testing accepted14field-payload value corpus12positive/7invalid is directly executed in three clients; no testing remote operation or Rust Mock runtime is added. Value Evidence remains bounded and independent.

## 本次有限审查单元

行为/生命周期单元：14accepted字段payload的三语言本地编码/投影。

剩余处置：12positive/7invalid直接执行，不承诺RustMockruntime或无serde对象重建，无不存在remoteoperation。

本项建模阈值是已接受入口、唯一状态权威、正常生命周期与具名失败/恢复路径可定位，并有直接证据或明确开放原因；不要求所有未定义组合全测，也不由绿色测试自动verified。

## 当前模型重观察范围

review_scope：本对象在真实当前候选Git `56929154c1851473e9e4837901b610cecf001b2d`（源码摘要`17cf243e8d6341db0ce23c5b9ee39977ce4ab100d6ed1b17ec5a7bb26c19e138`）上重新观察。上列code_refs/各scenario source_refs已逐Gitblob读取并与当前文件SHA精确相同，原锚点均存在。模型继续描述原accepted入口、框架业务/终态权威、Host owner/generation与transport消费职责、生命周期及具名失败义务；源码定位核对不证明所有行为正确、运行证据fresh或合同生成闭合。

原Behavior/status、Map场景mapped/needs_review、Rule/Asset处置、Finding/未决义务与Evidence原epoch支持范围全部保留。mapped仅入口模型，旧3ea不是本snapshot本体；历史receipt不得因本模型观察更新冒current运行。12source迁移、5秒物理结算与依赖50warning规则三选择仍未接受，完整生成/CI/main另验，任何新Rustwrapper不自动扩三语言合同。
