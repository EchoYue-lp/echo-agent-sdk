---
schema_version: 1
id: behavior.sdk-family-database
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
- evidence.sdk-tool-behavior-candidate
- evidence.sdk-ordinary-tools-b909-verification
- evidence.sdk-tool-value-natural-eof-after-candidate
finding_refs:
- finding.sdk-database-describe-dynamic-value
- finding.sdk-tool-artifact-context-propagation
---

# SDK family database

## 重要承诺

对已接受 `database` 能力明确入口、唯一权威、状态/生命周期、失败处置和直接行为证据；未接受 Rust identity 不进入承诺。

## 当前行为

- **入口/可达**：`facade/tools.rs` framework SqlQuery/ListTables/DescribeTable；schema
  参数映射后工具连接 DB，无 Host 自建 SQL 执行权限或数据库迁移。
- **权威/状态**：framework tool 和目标数据库事实；连接/query 生命周期由工具管理。
- **正常/失败**：tables/schema/rows/types/null、大整数与查询限制；read-only rejection、
  bad URL/SQL/schema、timeout/connection close，失败不得误投成 RPC success 数据。
- **已有断言**：真实SQLite columns/rows/null精确值；DELETE返回structured_error/tool_error与permanent/stop/side_effect=none；reread相同且DB文件SHA不变。
- **最低充分证据**：查询精确 row/schema/null/large int，rejected write 后数据库 unchanged，
  list/describe、invalid SQL/timeout/cleanup、三语言 typed result/error；SDK framework DB
  能力不意味着 EKO CLI 引入 SQLite。

## 期望行为

上述正常/失败场景须保持接受范围、完整转换和唯一权威，并在最终源码/pin证据及独立复审后闭合。

## 触发、结果与副作用

按上方具体入口和正常/失败记录检查真实主路径。可达性或catalog状态不能证明副作用、错误或结算。

## 失败、重试与恢复

按上方生命周期和最低充分证据检查取消、超时、错误、释放和恢复；不默认成功，不以其它能力族测试替代。

## 证据

阶段：`candidate_runtime_verified`；语义状态仍为 `needs_review`。

真实SQLite columns/rows/null精确值；DELETE返回structured_error/tool_error与permanent/stop/side_effect=none；reread相同且DB文件SHA不变。

实际来源：`evidence.sdk-tool-behavior-candidate`，绑定当前仍相同的具名测试文件与实测Host4d436；
28次工具调用+1Session close、10typed RPC error及database success=false拒写分别核对。
reported effects empty只表示已报告事实，实际文件/ref变更另行验证。GET-only记录不证明
无POST，invalid branch没有repo A全部refs前后断言。

仍缺：list_tables/describe_table、large integer、invalid SQL/URL、分页/timeout/connection failure及其它数据库驱动仍缺。 正式pin、最后变更后的独立复审与精确main仍待。
归档观察当前源摘要不重绑定旧whole运行；不自动扩大accepted合同。

## 裁决记录

用户要求SDK全能力行为治理，以明确接受跨语言合同为兼容边界；完整Rust inventory仅遥测。

## 当前观察与历史收据边界

Candidate assertion evidence is now preserved as original receipt/source fingerprints and checked against inspected current paths. It supports only named partial cases; current observation binding neither reruns old commands nor upgrades family status.

## Tool validator变化限制

当前SDK direct Tool路径在旧6711以后改变，改用同一framework validator并拒绝超额arity。旧候选运行仅保留在原来源历史审计，不证明新生产路径的完整行为；需结合新增validator具体反例与最终当前门禁，状态不提升。

## 本次有限审查单元

行为/生命周期单元：已接受查询/DTO/structured_error与真实文件副作用。

剩余处置：SQLite仅framework工具复用，不CLI依赖；拒写ToolResult和typedRPCerror分开；list/describe按接受项补证。

本项建模阈值是已接受入口、唯一状态权威、正常生命周期与具名失败/恢复路径可定位，并有直接证据或明确开放原因；不要求所有未定义组合全测，也不由绿色测试自动verified。
