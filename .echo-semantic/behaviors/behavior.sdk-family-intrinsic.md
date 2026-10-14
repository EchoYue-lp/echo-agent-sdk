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
observed_at: bd8e42e732588181b7a2589d91e9ffdce02d23e1
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
- evidence.sdk-integrations-finite-observation
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

原阶段缺口（历史，本次finite范围见下）：全部 accepted intrinsic helpers 的逐项消费和三语言精确结果对比未完成，不冻结完整 intrinsic membership。 published framework pin、最后变更后的独立复审和精确main验收
仍待；本证据不证明未列出的生命周期、字段或错误组合，也不自动扩大accepted合同。

## 裁决记录

用户要求SDK全能力行为治理，以明确接受跨语言合同为兼容边界；完整Rust inventory仅遥测。

## 当前观察与历史收据边界

Candidate assertion evidence is now preserved as original receipt/source fingerprints and checked against inspected current paths. It supports only named partial cases; current observation binding neither reruns old commands nor upgrades family status.

## 本次有限审查单元

行为/生命周期单元：明确接受的客户端本地helper构造/getter/不可变modifier。

剩余处置：逐接受helper覆盖矩阵，完整Rustintrinsic membership不冻结、不作分母。

本项建模阈值是已接受入口、唯一状态权威、正常生命周期与具名失败/恢复路径可定位，并有直接证据或明确开放原因；不要求所有未定义组合全测，也不由绿色测试自动verified。

## 当前模型重观察范围

review_scope：本对象在真实当前候选Git `56929154c1851473e9e4837901b610cecf001b2d`（源码摘要`17cf243e8d6341db0ce23c5b9ee39977ce4ab100d6ed1b17ec5a7bb26c19e138`）上重新观察。上列code_refs/各scenario source_refs已逐Gitblob读取并与当前文件SHA精确相同，原锚点均存在。模型继续描述原accepted入口、框架业务/终态权威、Host owner/generation与transport消费职责、生命周期及具名失败义务；源码定位核对不证明所有行为正确、运行证据fresh或合同生成闭合。

原Behavior/status、Map场景mapped/needs_review、Rule/Asset处置、Finding/未决义务与Evidence原epoch支持范围全部保留。mapped仅入口模型，旧3ea不是本snapshot本体；历史receipt不得因本模型观察更新冒current运行。12source迁移、5秒物理结算与依赖50warning规则三选择仍未接受，完整生成/CI/main另验，任何新Rustwrapper不自动扩三语言合同。

## 当前集成能力与supporting角色有界收口

原frameworkFinal独立source审查基于790/a634真实runtime，后A2A第一次missing具名consumer另由bd8三native24step/9ops/8HTTP补齐并审PASS。这是两个真实epoch，不把原A2Apendingreview文稿当今天未审，也不伪当时已审。九所选runtime69b在bd8逐Gitblob相同；currenthandles新A变动排除，旧source审不借成latestA或全部Host已验。

具体本family和registry/skill支持角色按sharedEvidence表/8Audit定位行为/唯一authority/lifecycle/失败：MCP/LSP owner/PID/全字段/derived不复活；Channel六forward原2facts/RWclose/peer隔离；EvalImprove cleanup/deadline/workspace；topologynodeedge；Permissionlive/TTL另scope；Statecheckpoint7、TraceRun17/nanotime/bigDuration/JSONLcold；Deliverymemory8ops同实例exactclaim；Testing14纯值controltransport非Mock业务。源码/API/模型不成为第二权威。

Behavior保持needs_review；入口具名mapped/examined不自动41verified、fullaccepted生成/main。Delivery无额外crossprocessdurability，configuredredaction不承诺未知所有secret；registry/skill归既有source_operation/intrinsic，无新family或任意skill执行。newAfinal、5秒physicalB/12identity及平台/main另验，支持范围未审的sourceoperationmethod保具体菜单理由，不以无限所有组合扩大义务。
