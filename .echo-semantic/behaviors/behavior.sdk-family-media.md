---
schema_version: 1
id: behavior.sdk-family-media
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
observed_at: 5bcbad2437e5c19f61101883044c4f67944cb97c
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
- evidence.sdk-computation-50
- evidence.sdk-tools-bridge-finite-observation
- evidence.sdk-media-accepted-route-repair-verification
finding_refs:
- finding.sdk-media-accepted-tool-name-routing
---

# SDK family media

## 重要承诺

对已接受 `media` 能力明确入口、唯一权威、状态/生命周期、失败处置和直接行为证据；未接受 Rust identity 不进入承诺。

## 当前行为

- **入口/可达**：`facade/tools.rs` 按 feature 构造 image/pdf/excel/word/text framework
  tools；accepted 操作不是都能由 text_stats 代表。
- **权威/状态**：framework parser/tool、输入文件与输出 artifact；资源创建/写文件由
  tool 管理，Host 保留 ToolResultContent/image URL、data/effect/failure。
- **正常/失败**：PDF pages、Excel cells/formulas、Word structure、text search/process/
  export、image fetch/view；坏格式/missing file、Unicode/encoding、size、partial output。
- **已有断言**：UTF-8 text_stats精确计数，unique结果/preview与export bytes；输入不变，missing input失败时目标sentinel不变。
- **最低充分证据**：为 accepted parser/export 操作准备小 fixture，逐项 expected semantic
  value/artifact 与 file bytes 对账，malformed/large input、export fault、不支持格式明确
  失败；三语言 paths/bytes/optional/content type一致，外部 viewer 非必要时不声称渲染完成。

## 期望行为

上述正常/失败场景须保持接受范围、完整转换和唯一权威，并在最终源码/pin证据及独立复审后闭合。

## 触发、结果与副作用

按上方具体入口和正常/失败记录检查真实主路径。可达性或catalog状态不能证明副作用、错误或结算。

## 失败、重试与恢复

按上方生命周期和最低充分证据检查取消、超时、错误、释放和恢复；不默认成功，不以其它能力族测试替代。

## 证据

阶段：`candidate_runtime_verified`；语义状态仍为 `needs_review`。

UTF-8 text_stats精确计数，unique结果/preview与export bytes；输入不变，missing input失败时目标sentinel不变。

实际来源：`evidence.sdk-tool-behavior-candidate`，绑定当前仍相同的具名测试文件与实测Host4d436；
28次工具调用+1Session close、10typed RPC error及database success=false拒写分别核对。
reported effects empty只表示已报告事实，实际文件/ref变更另行验证。GET-only记录不证明
无POST，invalid branch没有repo A全部refs前后断言。

原阶段缺口（历史，后继finite范围见下）：PDF/Excel/Word/image parser/export及其它text操作、GBK/大输入、实际图像model_content和partial output/cancel仍缺；不以text场景代表所有media。 正式pin、最后变更后的独立复审与精确main仍待。
归档观察当前源摘要不重绑定旧whole运行；不自动扩大accepted合同。

## 裁决记录

用户要求SDK全能力行为治理，以明确接受跨语言合同为兼容边界；完整Rust inventory仅遥测。

## 当前观察与历史收据边界

Candidate assertion evidence is now preserved as original receipt/source fingerprints and checked against inspected current paths. It supports only named partial cases; current observation binding neither reruns old commands nor upgrades family status.

## Tool validator变化限制

当前SDK direct Tool路径在旧6711以后改变，改用同一framework validator并拒绝超额arity。旧候选运行仅保留在原来源历史审计，不证明新生产路径的完整行为；需结合新增validator具体反例与最终当前门禁，状态不提升。

## 本次有限审查单元

行为/生命周期单元：已接受文件/bytes/UTF8统计preview/export原内容。

剩余处置：50共同语料与原28tool分别限定格式、UTF8及原导出bytes；未接受任意codec不进入证明义务。

本项建模阈值是已接受入口、唯一权威、生命周期及具名失败可定位，直接证据或明确开放原因可复核；mapped不代表verified，不要求未定义的任意组合。

## 当前模型重观察范围

review_scope：本对象在真实当前候选Git `56929154c1851473e9e4837901b610cecf001b2d`（源码摘要`17cf243e8d6341db0ce23c5b9ee39977ce4ab100d6ed1b17ec5a7bb26c19e138`）上重新观察。上列code_refs/各scenario source_refs已逐Gitblob读取并与当前文件SHA精确相同，原锚点均存在。模型继续描述原accepted入口、框架业务/终态权威、Host owner/generation与transport消费职责、生命周期及具名失败义务；源码定位核对不证明所有行为正确、运行证据fresh或合同生成闭合。

原Behavior/status、Map场景mapped/needs_review、Rule/Asset处置、Finding/未决义务与Evidence原epoch支持范围全部保留。mapped仅入口模型，旧3ea不是本snapshot本体；历史receipt不得因本模型观察更新冒current运行。12source迁移、5秒物理结算与依赖50warning规则三选择仍未接受，完整生成/CI/main另验，任何新Rustwrapper不自动扩三语言合同。

## 当前内置工具与反向桥有限审查

本对象归入builtin10families或原bridge角色，原currentTool实际source8lens与远端3合法注册consumer原因修复独立PASS记录于sharedEvidence及8Audits。runtime69b与真实5bcb所选source相同，原tool/typedvalue/FieldEffect/PID/error具名scope沿原family表保存，author只回收原业务审查，不把catalogdone/原whole计数当证据。新A独切片不在本scope，old原阶段待办保历史不会冒新runtime缺陷。

入口工具经唯一frameworkschema/customvalidator/ToolManager；FileAuthority/Polars/Pivot/provider各原authority不复制。Bridge descriptor/kind/gen注册后实际注入Session，ToolContext/artifact、Llm messages/usage/finish、Store namespace/commit、Hook/callback/intervention保持原owner/失败策略。remoteTool Cancelled/extension_timeout经ToolFailure::from_error按risk保分类与副作用，LLM Cancelled经AgentCancelled，合法旧3fail→samebody3pass callerTokenfalse/lease0，初坏descriptor是fixture错误不生产反例。

本对象needs_review不提升verified；八lens只具名examined。network原时点/内存Rag非crossprocess/未acceptcodec或constructor具体不承诺、5sphysicalB/12sourcekind/最终新源wholemain保独立。原ToolValue/Rag取证source保4feb taggedGit；FileAuthority补包若当前尚未commit其实际原件不假5bcbsourceRef，finite审scope不变，正式交付另逐包绑定。宽feature-failure-cause仅Tool/LLM子scope已修复，原Workflow/Memory/Eval其它义务未自动关。


## 2026-10-20：十二个non-text accepted消费者与同源路由修复（04eb首检查点历史）

原只text的不足由已有ToolsBridge Evidence文末后继承接：两accepted拼写不等原Tool名的真实old0bbc反例、新a17同probeafter及三语各一原媒体短链已实际执行。12unique操作/19call明确字段、文件、PNG model_content和错误/sentinel，不以telemetry或“未接受codec”排除既有承诺；不声称任意格式/所有选项/外部viewer或模型Run。原text default case与verifier完整保留并同文件回归通过。新finding.sdk-media-accepted-tool-name-routing跟踪同一两alias rootcause，当前open/空三链以待公开修复源码和最终当前门禁后精确闭合；本Behavior继续needs_review，不从focusedgreen直接升级。


## 2026-10-20 具名有限结算

本项整体继续needs_review。原具名子链已有后继并不授整个composite或媒体最终资格；保原header观察版本和历史runtime cut，当前04eb源码定位及受控增量另记。

直接证据：`evidence.sdk-candidate-behavior-verification`、`evidence.sdk-tool-behavior-candidate`、`evidence.sdk-computation-50`、`evidence.sdk-tools-bridge-finite-observation`、`evidence.sdk-media-accepted-route-repair-verification`。

具体复审入口：`audit.sdk-tools-bridge-finite-contract-evidence`、`audit.sdk-tools-bridge-finite-data-durability`、`audit.sdk-tools-bridge-finite-failure-concurrency`、`audit.sdk-tools-bridge-finite-permission-external`、`audit.sdk-tools-bridge-finite-result-side-effect`、`audit.sdk-tools-bridge-finite-state-authority`、`audit.sdk-tools-bridge-finite-time-lifecycle`、`audit.sdk-tools-bridge-finite-trigger-input`；只读者已按这一个named单元核权威、生命周期、失败与证据边界，不能由共享对象推整八风险组。

源码后继分开核：原Root17be/0bbc读审在本项指出十二个non-text补证缺口，不能称当时媒体PASS。04eb薄增量与a17/三语focused另经C0：仅media两accepted名字到原Tool及原测试新增可选故事，原fixture五函数/默认HTTP/text oracles不变。这里确有媒体生产路径变化，不能把共用的非媒体路径连续性当媒体验证；原两个真实before与新直接after分cut保存，未借旧208a14为新源码完整资格。

新12non-text/19call与原text已实际focused通过；04eb公开repair Evidence已绑定同源两alias Finding，该项仍OPEN，verification/rereview与最终新冻结完整门禁/精审待，不授全媒体最终资格。
