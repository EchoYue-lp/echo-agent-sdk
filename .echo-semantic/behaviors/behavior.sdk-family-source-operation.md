---
schema_version: 1
id: behavior.sdk-family-source-operation
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
observed_at: bd8e42e732588181b7a2589d91e9ffdce02d23e1
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
- evidence.sdk-integrations-finite-observation
finding_refs:
- finding.sdk-live-model-source-authority
- finding.sdk-retained-transcript-projection-identity
- finding.sdk-session-base-prompt-view
- finding.sdk-busy-session-read-authority
---

# SDK family source_operation

## 重要承诺

对已接受 `source_operation` 能力明确入口、唯一权威、状态/生命周期、失败处置和直接行为证据；未接受 Rust identity 不进入承诺。

## 当前行为

- **入口/可达**：`facade/registry.rs` exact accepted identity/digest →
  `_echo_agent/facade/invoke` → `facade/source_operations.rs::dispatch` 或同一 family adapter。
  ReactAgent/Agent service、SkillRegistry、file/tokenizer/guard/store 等由 receiver 调用。
- **权威/状态**：所调用 framework 对象及上述各 family 权威；Host resource只保存被
  寻址实例。source setter/getter 必须作用于当前 live Agent，而非过时 definition clone。
- **正常/失败**：exact receiver/arguments、live prompt/skills/services、constructor resource
  字段、optional、same-owner wrong-kind、bad signature/identity/feature；不能字符串猜 route。
- **已有断言**：Core admission live prompt/max_iterations/skills/permission/services、
  stateful resource/file authority/turn receipt；canonical sentinel 仅 reachability。
- **最低充分证据**：accepted route 按产品能力归组对齐本矩阵，逐 stateful setter→live
  consumer→getter/side effect、constructor→合法 method→close；所有转换字段 round-trip，
  非法 receiver/signature 无 mutation。source route 总数不成为项目完成分母。

## 期望行为

上述正常/失败场景须保持接受范围、完整转换和唯一权威，并在最终源码/pin证据及独立复审后闭合。

## 触发、结果与副作用

按上方具体入口和正常/失败记录检查真实主路径。可达性或catalog状态不能证明副作用、错误或结算。

## 失败、重试与恢复

按上方生命周期和最低充分证据检查取消、超时、错误、释放和恢复；不默认成功，不以其它能力族测试替代。

## 证据

阶段：`candidate_runtime_verified`；语义状态仍为 `needs_review`。

指定liveAgent setter/getter、resource方法/close、文件权威和receipt投影实际执行；全canonical null-input哨兵只支持adapter reachability。

实际来源：`echo-sdk-host/tests/core_profile_e2e.rs#negotiated_facade_admission_fails_closed`；`echo-sdk-host/tests/core_profile_e2e.rs#stateful_source_resources_preserve_owner_state_and_close`；`echo-sdk-host/tests/core_profile_e2e.rs#source_files_and_skill_registry_preserve_rust_authority`；`echo-sdk-host/tests/core_profile_e2e.rs#turn_receipt_source_operations_project_live_and_recovered`。

原阶段缺口（历史，本次finite范围见下）：accepted setter→真实consumer、所有sourceconstructor→合法method→close及各字段无损仍需逐能力归组，不以identity数量完成。 published framework pin、最后变更后的独立复审和精确main验收
仍待；本证据不证明未列出的生命周期、字段或错误组合，也不自动扩大accepted合同。

## 裁决记录

用户要求SDK全能力行为治理，以明确接受跨语言合同为兼容边界；完整Rust inventory仅遥测。

## 当前观察与历史收据边界

Candidate assertion evidence is now preserved as original receipt/source fingerprints and checked against inspected current paths. It supports only named partial cases; current observation binding neither reruns old commands nor upgrades family status.

## 本次有限审查单元

行为/生命周期单元：exactaccepted definingidentity/signature→现framework服务/资源。

剩余处置：不是每Rustpubidentity三语言；逐能力sourceconstruct/method/close的明确义务由菜单/Findings跟踪。

本项建模阈值是已接受入口、唯一状态权威、正常生命周期与具名失败/恢复路径可定位，并有直接证据或明确开放原因；不要求所有未定义组合全测，也不由绿色测试自动verified。

## 当前模型重观察范围

review_scope：本对象在真实当前候选Git `56929154c1851473e9e4837901b610cecf001b2d`（源码摘要`17cf243e8d6341db0ce23c5b9ee39977ce4ab100d6ed1b17ec5a7bb26c19e138`）上重新观察。上列code_refs/各scenario source_refs已逐Gitblob读取并与当前文件SHA精确相同，原锚点均存在。模型继续描述原accepted入口、框架业务/终态权威、Host owner/generation与transport消费职责、生命周期及具名失败义务；源码定位核对不证明所有行为正确、运行证据fresh或合同生成闭合。

原Behavior/status、Map场景mapped/needs_review、Rule/Asset处置、Finding/未决义务与Evidence原epoch支持范围全部保留。mapped仅入口模型，旧3ea不是本snapshot本体；历史receipt不得因本模型观察更新冒current运行。12source迁移、5秒物理结算与依赖50warning规则三选择仍未接受，完整生成/CI/main另验，任何新Rustwrapper不自动扩三语言合同。

## 当前集成能力与supporting角色有界收口

原frameworkFinal独立source审查基于790/a634真实runtime，后A2A第一次missing具名consumer另由bd8三native24step/9ops/8HTTP补齐并审PASS。这是两个真实epoch，不把原A2Apendingreview文稿当今天未审，也不伪当时已审。九所选runtime69b在bd8逐Gitblob相同；currenthandles新A变动排除，旧source审不借成latestA或全部Host已验。

具体本family和registry/skill支持角色按sharedEvidence表/8Audit定位行为/唯一authority/lifecycle/失败：MCP/LSP owner/PID/全字段/derived不复活；Channel六forward原2facts/RWclose/peer隔离；EvalImprove cleanup/deadline/workspace；topologynodeedge；Permissionlive/TTL另scope；Statecheckpoint7、TraceRun17/nanotime/bigDuration/JSONLcold；Deliverymemory8ops同实例exactclaim；Testing14纯值controltransport非Mock业务。源码/API/模型不成为第二权威。

Behavior保持needs_review；入口具名mapped/examined不自动41verified、fullaccepted生成/main。Delivery无额外crossprocessdurability，configuredredaction不承诺未知所有secret；registry/skill归既有source_operation/intrinsic，无新family或任意skill执行。newAfinal、5秒physicalB/12identity及平台/main另验，支持范围未审的sourceoperationmethod保具体菜单理由，不以无限所有组合扩大义务。

## 2026-10-19 accepted source分组承接

272操作/37receiver仅用于十二能力组的漏项核对。原处置表逐组列live配置、services、Subagent/MCP、MemoryStore、Plugin、Skill/prompt、Eval/Improve/Topology、Evolution、Security/policy、Projectrules、FS/artifact/Git与Turn/Plan/state的原直接consumer、构造/方法/close、失败和明确限制。新e31/2cae/88a定向证据与原8481完整运行分开，不以null reachability或三语载荷往返代替实际backend消费。

模型与retained投影Finding仍open，预算producer消费者after仍待；新scalar/live04仅具名范围，Sink managerfalse、RunStore六种helper仅三种actual、Intent/Skill default与无非默认fence branch资格均保留。没有新API/Store/validator/SDKcursor，不降既有accepted承诺为deferred；本Behavior状态不改，最新pin/生成/门禁/main独立。

## 2026-10-19 Session base prompt新增明确反例

原SourceOperation分组与两Finding不变。新finding.sdk-session-base-prompt-view记录inherent base setter实际成功后Session Traitbase getter仍构造A的真实旧Host red；current getter已owned且正常，Traitbase与runtime override意义不能合并。该义务只要求同一现Handle读取base，零Session定义不动；源/typed数据/原日志和after限制在原Integration Evidence后继。原Memory/Workflow已resolved不回滚，不混F root-system位置或模型配置修复；本Behavior状态不变。

## 2026-10-19 忙时读取不等实际空状态

新增finding.sdk-busy-session-read-authority记录原MCP initialize屏障真实native：已配置cwd的普通Session并发getter在原writer持锁时返回null，释放并完整load后cwd仍同路径。原wrapper同步try_read默认不可作为RPC状态；修复候选必须读取原authority且处理Agent与Context回调父子等待，不伪缓存/新协议/改F同步Trait。Context层和snapshot实际mutation仅源码调查，不冒native已闭合；本条open/空三链，原base/model/projection/managed各自独立。
