---
schema_version: 1
id: evidence.sdk-workflow-observation-source-gaps
kind: evidence
observed_at: 404d447f1a69fb4af2843272005b894063888ed9
source_refs:
- echo-sdk-host/src/core_profile/facade/mod.rs
- echo-sdk-host/src/core_profile/facade/workflow.rs
- echo-sdk-host/src/core_profile/extension_bridge.rs
- sdks/typescript/test/llm-workflow-stream-host.test.js
- sdks/shared/testing/workflow_checkpoint_fixture.py
- sdks/typescript/test/workflow-checkpoint-host.test.js
- sdks/python/tests/test_workflow_checkpoint_host.py
- sdks/java/src/test/java/com/echoagent/sdk/WorkflowCheckpointHostTest.java
supports:
- behavior.sdk-family-workflow
- rule.framework-runtime-authority
- finding.sdk-workflow-resume-observation-claim
limitations:
- Current header binds the bounded A candidate; historical 019e static-source and later native runtime epochs remain separately identified below.
- Public framework checkpoint is external provenance in the body, not an SDK observed_at or local code_ref.
- Historical source used 00782; later native uses published 988 and immutable Host14a, with only the approved A capability and reverse operation.
- Ordinary error policy and deferred StepFn boundary remain; bounded A native verification does not close the wider Graph lifecycle Finding.
---

# 已接受 Workflow 的 B 消费边界源码取证

## 支持的结论

本节记录019e历史源码调查；当前header绑定后继A候选，实际运行结论单列于“后继14a”，不反向改写历史。

SDK019e 的 facade/workflow.rs：build 分支读取 Session Agent live LlmClient，优先 build_graph_with_client；resume、resume_exact、resume_with_state 只调原 Graph 并用 framework(error) 无损投影。extension_bridge.rs 的 ExtensionLlmClientProxy 和 ExtensionAgentComponentProxy 是原 reverse 执行入口；facade/mod.rs 的 workflow.extension.run 则独立 stringify。TS llm-workflow-stream-host.test.js 第72行是实际 Session.invoke 前向 consumer，而非目录/menu 推测。

## 来源与范围

[framework f0a6cd1c13706c20955209b698c1f891c09d78b3](https://github.com/EchoYue-lp/echo-agent/tree/f0a6cd1c13706c20955209b698c1f891c09d78b3)，tree 9ee1e4340ab09e75cb2d5b5a95c22acc0611362e。本次实际用 git show 读取该 committed tree，未把当前 dirty F 新设计/测试当来源：

- [loader.rs AgentNode226–238](https://github.com/EchoYue-lp/echo-agent/blob/f0a6cd1c13706c20955209b698c1f891c09d78b3/src/workflow/loader.rs#L226)：把给定 LlmClient 交 ReactAgentBuilder，再 add_agent_node。
- [node.rs191/197](https://github.com/EchoYue-lp/echo-agent/blob/f0a6cd1c13706c20955209b698c1f891c09d78b3/echo-orchestration/src/workflow/node.rs#L191)：原 Agent 流执行及 event? 接收错误。
- [graph.rs1286–1296](https://github.com/EchoYue-lp/echo-agent/blob/f0a6cd1c13706c20955209b698c1f891c09d78b3/echo-orchestration/src/workflow/graph.rs#L1286)：任何 resume Err 普通 requeue；requeue failure 再 stringify 为 Other。
- [graph.rs1301 heartbeat](https://github.com/EchoYue-lp/echo-agent/blob/f0a6cd1c13706c20955209b698c1f891c09d78b3/echo-orchestration/src/workflow/graph.rs#L1301)：同 attempt renew 与原 resume future 的生命周期需要框架强反例，不能由 SDK 拥有或用旧通过补证。

## 已知缺口

两项 open 义务分别见 finding.sdk-workflow-resume-observation-claim（F graph/claim authority）与 finding.sdk-workflow-direct-run-observation-projection（SDK wire 投影）。仅新增源码取证和 cfg(test) 真实 ACP projection 回归源码；未执行测试、未修 direct-run 生产分支，也未核新的 heartbeat/expiry/restart after。SDK原e5ed27冻结、client纯验证及旧业务证据保原 epoch；不冒新pin/whole/main通过。

预备测试原件与编排记录仅在 repo-local .echo-semantic/local/sdk-cancellation-observation/，不是正式 code_refs/source_refs。后续真实 pin 后需先该单根因 red，验证 callback确已准入/工作仍held/observer错误完整/原地址不释放，再修复/after/独审；不把缺 variant 的编译失败当 red。

## 后继14a：registered Llm 三语A有限验收

同一公开framework988和不可变Host `14a0cb58c44bdb84990262f1324e1a2513513a8b40fc0640a9d36f91f95e27e9` 上，TS、Python、Java原WorkflowCheckpointHostTest各实际1例、0fail/error/skip；共享30步oracle验证原Store绑定、四个legacy入口零Node、protect正回应门、明确保护Error无Node，以及原LLM callback准入的同Popen持有、释放和实际join。unknown后原protected checkpoint/attempt不重领、不requeue、不重复LLM副作用。历史45fc Java整例close失败和14a分阶段真实失败均保留，没有重新标绿。

首后继实测源码SHA256：shared workflow_checkpoint_fixture.py `34b3bc90b7dd723f389eeeee4076ae90c51814e599a44dbfd6860484569ac1c4`；TS workflow-checkpoint-host.test.js `6beb67627854c4b10952d9c7b53c9e0bf4eac0c92187bd99b64ade2ddf7bb51d`；Python test_workflow_checkpoint_host.py `3437c6ce813a185d0fc073ab0e03e2c7ef7d7176462d97f34bdaefa92d8cb5ac`；Java WorkflowCheckpointHostTest.java `18b2676f93141be2fba931d1ea8d6328162cd5f5fca1fea3bd7e15b40e650ac2`。这些是本后继切点，不冒充019e历史源码内容。

完整定向日志分别为TS983B/SHA256 `bf3bbd2fb6bdeceb2e9bd3c10056525726fdc2e15dc2fcc43679f4225ce6e6dc`、Python98B/`55c1c7f57e634a2b3613f65f80bdfa4b31f2b56968feee4e2cfd3dc45d5af019`、Java574B/`ccc4a56b78afd6815563bd29d51923a0d85b4f25432ff62d49de1ac8733482a7`；均exit0/nullsignal/notrunc，JavafreshXML无失败/跳过。TS/Python各611输入零漂移，TS从当前源码build，Python实际加载受控S/src；Java保存独立实际切点，不累加为全套计数。

Java原分阶段失败日志5500B/SHA256 `339b49f9f9df05cbda4a1a06cdb21cfddff2672c697949835595271ba0be68f0` 证明写入已排空、原Host仍alive/exitedpending至五秒，随后原SDK watchdog处置同PID。修正的是测试对合法unknown关闭的预期：必须精确timeout/after_delay/client/close与原五秒错误、实际阶段、原invocation地址和同retained transport owner重试。最终49采样中48个夹前后仍live的采样地址均为同一对象，0提前清；原Host实际exit137，重试仅完成本地I/O清理。诊断SHA256 `1f6e4fcc11cb02ecb0d3d8361a39b9cbff49af5ef912ed04466f5c7322b4fae2`，没有吞generic timeout、延长期限或改生产。

TS原Host实际SIGTERM、Python实际returncode=-15；各自只记录本地transport清理返回。三语均在关闭后fresh inspect，对stores、llm_calls、calls与关闭前最终快照逐项精确相等；没有解保护、重领、重复副作用或合成matching receipt。TS/Python只证明关闭前地址保留，不借Java采样宣称其关闭期间持续保留。原reader核最终源码、完整runner/log/XML及backend原件后有限pass、行动项0；raw仅保ignored任务local。

独立Git候选以已公开3351937为基底，仅收A三个native文件与共享workflow fixture；未带B三个Task测试、Task fixture或Linux诊断。候选重新build TS、显式加载candidate/src的Python、Java clean后重编译，三语A各实际1例通过。完整日志分别TS995B/SHA256 `eece5160348665b652c003acb81f1646cb1cef69d7366e63539da73400a62481`、Python98B/`ff31e902bd9e23e785e90c40101f9cc9887d0f01cc74c2a1e8ad3fadf9cfe284`、Java1003B/`9b46af5b5c052faf37c6d713a35912191cfa5ae18051c1069a2cba44f11f4193`，均exit0/nullsignal/notrunc、无失败/跳过，JavafreshXML另存原件。候选A四文件与上述已审source逐字相等；611语言输入与S的差异恰为排除的B四文件，其5个排除路径均精等公开父提交；50项Host生产源码/Cargo/config输入与14a原构建冻结零漂移，无Rust生产/协议/生成物变更，复用原Host不重建。

首次候选canonical真实报告7项旧source63bd不可恢复，未作豁免。原固定c0bc source_digest_at_revision证明公开3351937精等63bd；只把旧四Evidence、一个Audit及原Workflow challenge的历史metadata改绑该真实Git，保原body/status/epoch，不冒新业务结果。当前A对象与baseline仍绑定当前候选source，S中的B/Linux受控dirty不成为候选来源。

这不是自然Host退出、framework物理执行/Graph claim结算、任意Graph晚到成功、File/跨provider持久恢复或真实保护ACK丢包证据；宽Finding与完整SDK/main门禁继续open。新增的是原接受入口的测试和有限事实，不增加示例调用或官网功能承诺；原examples的调用形状未变，最终整体验收仍需执行原examples链路。

当前Java oracle另已按正式Host/本机transport边界修正，源码SHA256 `641c66db13333ade6b7ed440ead8ba802395935cd4237104b00d059a41bd8d7c`。local close返回成功不等于业务graceful，不再无条件要求抛错；该成功分支仍须同PID已实际非零退出、transport清理完成、原地址在live采样不早清及原backend不变。无matching unary结果却Host exit0会被拒绝：原F close在未结算invocation/profile债务时返回Err，Host main将Err映射exit2。错误分支仍精确timeout/retry/op/等待原Host退出阶段与同owner retry，原先49采样与诊断原件均保留。这个约束不改变任何生产关闭逻辑或期限；不把非零退出统称自然退出。

修正后的同一独立候选Java clean重编译，A整例及既有HostEarlyExitTest的实际child退出后合法local close控制各1例、0fail/error/skip。完整1001B日志SHA256 `c7fdad9be98ca0a5706eebae6485cfab7487d3802cdf93eaa54139806b1badd0`；A freshXML SHA256 `69f0deca2d3f1b05443760796e28bfcf35bdbc529aea5b2e3fe6b986e667e112`，独立close控制XML `e96478dc242ddaead6e1e2e88b8a76919cef75ca578d1220735e9d9aa96388e9`。本次A实际走严格typed timeout分支、49采样/0地址早清、原Host137及同owner retry，backend不变；未声称A实际走过成功分支，控制例只验证既有client本机close契约。TS/Python/共享fixture与前述已运行候选逐字不变，按影响闭包复用其结果。

## 历史：后继实施阶段的源码准备

以下保留当时的阶段记录；其中observed_at指原019e，未执行描述不代表后继14a状态。

上面的observed_at/固定f0a源码和“尚未执行”描述原019e调查epoch。根任务此后已取得F Native FunctionNode Memory/File与后继原owner的分别before；原件与F源验证由F持有，不是SDK真实Host结果。SDK A单cap/op的Protocol与三客户端各3项pure after有实际独审通过，保护算法仍只归原Graph/Store，不借pure decoder测试证明Node生命周期。当前已准备原checkpoint fixture的独立optional真实Host路径，原router-only语料/业务oracle保：legacy绑定后新注册不换cap、protect提交后hold正回应Node0、明确Error响应不执行Node、实际registeredLlm入站才准入同Popen/foreign身份。没有运行新Host/Native三语，不宣称File/跨provider重启或真实保护ACK丢包；direct-run投影仍等新pin实际red后修复。上述状态说明不把原observed_at重标成新业务验证或三链闭合。
