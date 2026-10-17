---
schema_version: 1
id: behavior.sdk-family-standard
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
boundary: boundary.sdk-acp
observed_at: 622d7931e01556161441d22de0697f6f35f24f17
code_refs:
- echo-sdk-host/src/lib.rs
- echo-sdk-host/src/factory.rs
- echo-sdk-host/src/features.rs
- echo-sdk-host/src/core_profile/mod.rs
- echo-sdk-host/src/core_profile/state.rs
- echo-sdk-protocol/src/capability.rs
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-candidate-behavior-verification
- evidence.sdk-parent-driver-candidate-verification
- evidence.sdk-acp-agent-run-finite-observation
- evidence.sdk-protocol-finite-observation
finding_refs: []
---

# SDK family standard

## 重要承诺

对已接受 `standard` 能力明确入口、唯一权威、状态/生命周期、失败处置和直接行为证据；未接受 Rust identity 不进入承诺。

## 当前行为

- **入口/可达**：official ACP v1 methods → `run_stdio`/framework `AcpAgentAdapter`，
  可无 Echo profile 使用；不靠 accepted facade catalog 将标准方法重新封装。
- **权威/状态**：framework standard Session/prompt/cancel，official schema 定义协商及
  wire shape；连接 initialize → Session → prompt/update → stop/cancel → EOF。
- **正常/失败**：标准 prompt/update 与扩展 Run 共享事实；版本/非法 params/无 profile、
  EOF、配置准备失败、stdout 污染不能被扩展成功用例遮盖。
- **已有断言**：Stdio standard prompt、parked-model cancel、EOF no noise、invalid config
  no secret；Protocol ACP baseline 检查 v1 constants、lock/checksum、unstable exclusions。
- **最低充分证据**：最终 binary 上普通 ACP client 无 profile 的正常 prompt/cancel/EOF，
  与 extended standard prompt 共享 Session/Run 比对；初始化错误、unsupported version
  和 startup cleanup 的真实行为。baseline constants 不是协议执行证据。

## 期望行为

上述正常/失败场景须保持接受范围、完整转换和唯一权威，并在最终源码/pin证据及独立复审后闭合。

## 触发、结果与副作用

按上方具体入口和正常/失败记录检查真实主路径。可达性或catalog状态不能证明副作用、错误或结算。

## 失败、重试与恢复

按上方生命周期和最低充分证据检查取消、超时、错误、释放和恢复；不默认成功，不以其它能力族测试替代。

## 证据

阶段：`candidate_runtime_verified`；语义状态仍为 `needs_review`。

extended standard session/prompt 的 end_turn、session/update 与 Session identity 同步；三语言真实 Host smoke/examples 亦执行标准 prompt。

实际来源：`echo-sdk-host/tests/core_profile_e2e.rs#extended_standard_prompt_bridges_shared_core_handles`；`sdks/typescript/test/host.test.js`。

原阶段缺口（历史，当前有界范围见下）：无 profile 的独立 stdio_e2e suite 不在此 Host136/31/34/12 收据内；unsupported version/startup cleanup 与 plain ACP cancel/EOF 最终运行仍缺。 published framework pin、最后变更后的独立复审和精确main验收
仍待；本证据不证明未列出的生命周期、字段或错误组合，也不自动扩大accepted合同。

## 裁决记录

用户要求SDK全能力行为治理，以明确接受跨语言合同为兼容边界；完整Rust inventory仅遥测。

## 当前观察与历史收据边界

Candidate assertion evidence is now preserved as original receipt/source fingerprints and checked against inspected current paths. It supports only named partial cases; current observation binding neither reruns old commands nor upgrades family status.

## 本次有限审查单元

行为/生命周期单元：标准ACP initialize/session/prompt/cancel/EOF与可选扩展保持角色分离。

剩余处置：标准stdio4有实际证据；最终当前pin完整stdio/Profile测试与平台signal待交付。

本项建模阈值是已接受入口、唯一状态权威、正常生命周期与具名失败/恢复路径可定位，并有直接证据或明确开放原因；不要求所有未定义组合全测，也不由绿色测试自动verified。

## 当前模型重观察范围

review_scope：本对象在真实当前候选Git `56929154c1851473e9e4837901b610cecf001b2d`（源码摘要`17cf243e8d6341db0ce23c5b9ee39977ce4ab100d6ed1b17ec5a7bb26c19e138`）上重新观察。上列code_refs/各scenario source_refs已逐Gitblob读取并与当前文件SHA精确相同，原锚点均存在。模型继续描述原accepted入口、框架业务/终态权威、Host owner/generation与transport消费职责、生命周期及具名失败义务；源码定位核对不证明所有行为正确、运行证据fresh或合同生成闭合。

原Behavior/status、Map场景mapped/needs_review、Rule/Asset处置、Finding/未决义务与Evidence原epoch支持范围全部保留。mapped仅入口模型，旧3ea不是本snapshot本体；历史receipt不得因本模型观察更新冒current运行。12source迁移、5秒物理结算与依赖50warning规则三选择仍未接受，完整生成/CI/main另验，任何新Rustwrapper不自动扩三语言合同。

## 当前ACP/AgentSessionRun与A有界收口

本对象恢复真实8763源码/资料，frameworkFinal本session独立8lens源审与实际native反例/修复来源通过后准确回收，Hosthandlerac5/handles8b89/state8e3/modc766/factoryda354保一authority。old193e790、Steer原eb兼容前证明与newdaA三native13input/五组及2officialRPC取消、pub9等各保原cut；不是模型writer新审或allgoalmain通过。A同key完整publication响应/owner+gen+payloadfingerprint、失败前effects释放retry、成功后原job保持/closed不复活具局部三链，A具体Finding可resolved，framework仍唯一execution/terminal，不新idempo scheduler。

菜单校准：CoreProfile扩展Session只有create/load/close；officialsession_list是协议inventory而非已advertise运行入口，save为自动state/journal/receiptprojection不新Session.save。Runget是snapshot，start返回streamhandle/eventfeed并replay/ACK；RunHandle.close用canonical cancel/wait后关闭本地订阅（TS仅非终态，PY/Java尝试取消并等）；没有Hostrunclose且不直接退役原HostRun。原waitshortsettledfalse不cancel、ACK仅initiation不是physicalcompleted。

本组Behavior保持needs_review，scene仅mapped/8Auditexamined。新Javaversion溢出、malformedarray/limits shape的协商缺陷是另具体openFinding，不被本Hostsource审PASS吞并，完整旧whole在其client变更后为历史；12sourcekind/5sB/平台/fullgeneration/main独立。B宽真实physicalfalse仍open，合作close/EOF不反证它；当前有限场景不是全部未定义组合全测。

## Protocol与广告shape后继有界闭合

原SDKcap独立Protocol8lens source审BLOCK的specificshape现经合法Hostproxy真before修复/newinitowner增量及实际源/材料独审PASS，source622d与原gate取证同。JBigIntegerexactversion/capfeatures全member拒/17canonicalpositiveu64/positivecallbacku32/2duration/nonemptydistinctcaps、Unicodecodepointsortedfeatures；Python officialValidationError仅initialize typed，TSfailedinithelper实际EOF/childexit持有原owner，原primaryerrorcleanupcause保。六genuineJava前误接受、TS/PY两shape/PYtypedoverflow与七afterOnly负例不混；TS37/PYintegrity31/原J21/PY25按freeze各切点，不旧782whole复用为新source完整通过。

具体negotiationFinding修复/反例/verification/actualrereview三链已齐可localresolved，Behaviour仍needs_review/8lensexamined不全familyverified。core_rpc_contract/stdstdio4/plain/mismatchedhello和require_extended/negotiate_hello直接引用真实，旧运行time不因观察622d被重标；Windowsnative、fullgenerator12/B5physical平台/main独立。协议类编译catalog不是全部Rustinventorylanguageparity，无新接口/字段/错误taxonomy。
