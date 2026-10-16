---
schema_version: 1
id: evidence.sdk-workflow-observation-source-gaps
kind: evidence
observed_at: 019e700279677a2eb77e265b5038d9d581bc22da
source_refs:
- echo-sdk-host/src/core_profile/facade/mod.rs
- echo-sdk-host/src/core_profile/facade/workflow.rs
- echo-sdk-host/src/core_profile/extension_bridge.rs
- sdks/typescript/test/llm-workflow-stream-host.test.js
supports:
- behavior.sdk-family-workflow
- rule.framework-runtime-authority
limitations:
- Static source and accepted caller trace only; no new Rust, Host or language runtime execution.
- Public framework checkpoint is external provenance in the body, not an SDK observed_at or local code_ref.
- SDK build pin remains 00782; no path override, generated contract or public API expansion.
- Ordinary error policy and deferred StepFn boundary remain; two Findings have independent owners and no closure chains yet.
---

# 已接受 Workflow 的 B 消费边界源码取证

## 支持的结论

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

## 后继实施阶段的范围说明

上面的observed_at/固定f0a源码和“尚未执行”描述原019e调查epoch。根任务此后已取得F Native FunctionNode Memory/File与后继原owner的分别before；原件与F源验证由F持有，不是SDK真实Host结果。SDK A单cap/op的Protocol与三客户端各3项pure after有实际独审通过，保护算法仍只归原Graph/Store，不借pure decoder测试证明Node生命周期。当前已准备原checkpoint fixture的独立optional真实Host路径，原router-only语料/业务oracle保：legacy绑定后新注册不换cap、protect提交后hold正回应Node0、明确Error响应不执行Node、实际registeredLlm入站才准入同Popen/foreign身份。没有运行新Host/Native三语，不宣称File/跨provider重启或真实保护ACK丢包；direct-run投影仍等新pin实际red后修复。上述状态说明不把原observed_at重标成新业务验证或三链闭合。
