---
schema_version: 1
id: behavior.sdk-family-files
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
- evidence.sdk-ordinary-tools-b909-verification
- evidence.sdk-tool-value-natural-eof-after-candidate
- evidence.sdk-tools-bridge-finite-observation
- evidence.sdk-integrations-finite-observation
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

原阶段缺口（历史，后继finite范围见下）：全部 accepted filesystem verbs、non-UTF8跨平台失败及三语言副作用一致性仍缺。 published framework pin、最后变更后的独立复审和精确main验收
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

## 当前模型重观察范围

review_scope：本对象在真实当前候选Git `56929154c1851473e9e4837901b610cecf001b2d`（源码摘要`17cf243e8d6341db0ce23c5b9ee39977ce4ab100d6ed1b17ec5a7bb26c19e138`）上重新观察。上列code_refs/各scenario source_refs已逐Gitblob读取并与当前文件SHA精确相同，原锚点均存在。模型继续描述原accepted入口、框架业务/终态权威、Host owner/generation与transport消费职责、生命周期及具名失败义务；源码定位核对不证明所有行为正确、运行证据fresh或合同生成闭合。

原Behavior/status、Map场景mapped/needs_review、Rule/Asset处置、Finding/未决义务与Evidence原epoch支持范围全部保留。mapped仅入口模型，旧3ea不是本snapshot本体；历史receipt不得因本模型观察更新冒current运行。12source迁移、5秒物理结算与依赖50warning规则三选择仍未接受，完整生成/CI/main另验，任何新Rustwrapper不自动扩三语言合同。

## 当前内置工具与反向桥有限审查

本对象归入builtin10families或原bridge角色，原currentTool实际source8lens与远端3合法注册consumer原因修复独立PASS记录于sharedEvidence及8Audits。runtime69b与真实5bcb所选source相同，原tool/typedvalue/FieldEffect/PID/error具名scope沿原family表保存，author只回收原业务审查，不把catalogdone/原whole计数当证据。新A独切片不在本scope，old原阶段待办保历史不会冒新runtime缺陷。

入口工具经唯一frameworkschema/customvalidator/ToolManager；FileAuthority/Polars/Pivot/provider各原authority不复制。Bridge descriptor/kind/gen注册后实际注入Session，ToolContext/artifact、Llm messages/usage/finish、Store namespace/commit、Hook/callback/intervention保持原owner/失败策略。remoteTool Cancelled/extension_timeout经ToolFailure::from_error按risk保分类与副作用，LLM Cancelled经AgentCancelled，合法旧3fail→samebody3pass callerTokenfalse/lease0，初坏descriptor是fixture错误不生产反例。

本对象needs_review不提升verified；八lens只具名examined。network原时点/内存Rag非crossprocess/未acceptcodec或constructor具体不承诺、5sphysicalB/12sourcekind/最终新源wholemain保独立。原ToolValue/Rag取证source保4feb taggedGit；FileAuthority补包若当前尚未commit其实际原件不假5bcbsourceRef，finite审scope不变，正式交付另逐包绑定。宽feature-failure-cause仅Tool/LLM子scope已修复，原Workflow/Memory/Eval其它义务未自动关。


## 2026-10-20 具名有限结算

本项verified只限原声明的具名合同单元及下列直接证据，非全SDK/平台或任意未列组合。原模型header observed_at为`5bcbad2437e5c19f61101883044c4f67944cb97c`，原正文及Evidence各自runtime cut保留；当前模型观察绑定公开`04eb6ad50982656b5a8b8ea649619b6a383065ff`，不把旧测试重标为新Host执行。

直接证据：`evidence.sdk-ordinary-tools-b909-verification`、`evidence.sdk-tool-value-natural-eof-after-candidate`、`evidence.sdk-tools-bridge-finite-observation`、`evidence.sdk-integrations-finite-observation`。

具体复审入口：`audit.sdk-tools-bridge-finite-contract-evidence`、`audit.sdk-tools-bridge-finite-result-side-effect`；只读者已按这一个named单元核权威、生命周期、失败与证据边界，不能由共享对象推整八风险组。

源码后继分开核：原Root17be/0bbc具名review与04eb薄增量分别C0。04eb相对17be只在media lookup保两accepted名字到原Tool，三测试wrapper新增可选媒体故事，原fixture五函数/默认HTTP/旧oracles不变；本项所依非媒体生产路径不因共有文件名改变而假报重审或全套新运行。新媒体after另由a17/三语focused证明，未借旧208a14为新源码完整资格。

支持范围仍限原named consumer/字段/副作用及其明确失败；未接受identity/任意provider、codec、选项或竞态组合不自动加入分母。
