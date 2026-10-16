---
schema_version: 1
id: evidence.sdk-workflow-stream-late-completed-payload
kind: evidence
observed_at: 335193791c70a6cb077033c3f8f0cb1223e5266e
source_refs:
- echo-sdk-host/src/core_profile/facade/mod.rs
- sdks/typescript/test/llm-workflow-stream-host.test.js
- sdks/python/tests/test_llm_workflow_stream_host.py
- sdks/java/src/test/java/com/echoagent/sdk/LlmWorkflowStreamHostTest.java
- sdks/shared/testing/llm_workflow_stream_fixture.py
supports:
- behavior.sdk-family-workflow
- rule.framework-runtime-authority
- finding.sdk-workflow-direct-run-observation-projection
- finding.sdk-workflow-stream-late-completed-payload
limitations:
- Original stream before/after covers TypeScript; later exact C1 has one actual late-payload native pass per language, separately bound below.
- Original direct/stream red-green binaries use opt0 and emit a Darwin linker warning; these historical behavioral results do not qualify the final zero-warning gate.
- C1 native results do not qualify the subsequent full-queue cancellation repair or final configuration representation; the affected common cut is being refreshed.
- No Graph late-success source, durable checkpoint or restart guarantee is established by this extension-stream case.
---

# 原 Workflow producer 的真实晚到结果

## 支持的结论

原extension stream在observation后继续消费和转发其真正Completed payload，取消/发送失败的清理路径保持各自原边界。

## 来源与范围

### 非流direct-run的实际红绿

同一 `facade::tests::workflow_direct_run_preserves_observation_while_original_work_is_held` 正式ACP/profile前向→原reverse Workflow callback实际准入held工作，ordinary对照先满足，observation完整身份断言失败；before日志2199B/SHA256 `eac69512cce80465f5e62c6765b799f02812d8070b8a654e016762bfcfc5bb70`，exit101/1fail。仅ExecutionNotSettled改用原wire projector后，同例日志940B/SHA256 `5217496be1019f8afcac490c8bf446644eced68bc642640d6c620b26b27f72b5`，exit0/1pass；两者均无signal/截断，均属原opt0带链接warning切点，不是最终构建资格。原工作只从原callback准入，最终release/join同一guard/任务；ordinary错误前缀/无观察details对照保留。

### 流式原Completed payload的实际红绿

2026-10-16，同一原 public SDK 用例 `Workflow observation retains the real late Completed payload` 在不可变旧 Host `c88f3c0f4130be540a9ecd038e248c7695acad3d99e642a0bc3c5629b93a481b` 实际失败，generic complete 取代 Completed item；只改观察分支后，新 Host `19c6edca8307190c52d0377c5c66f26d97fc829f51df4da3d760c6f236a5e499` 同例通过。callback 内创建实际 async producer 和独立 run/attempt 身份，unknown 发出时副作用计数为0；释放 gate 后等待原 producer 发送真实 complete，再读取同一 facade stream，精核 result、steps、duration、handle 与顺序。没有把 RPC id、局部 EOF 或 observer 完成当原执行完成。

第一次 after 已收到正确 item，但测试错误假设 TS decoder 会自动解包 duration，故断言失败；原失败原样保留。移除该重复错误断言，由共享 verifier 使用既有 canonical unbox 校验 duration 全值；修正后的同一测试重新在旧 Host 失败、新 Host 通过，未修改 producer 输入或顺序。

原件仅保留在 ignored `.echo-semantic/local/sdk-parallel-delivery-01a14385/`。修正后 before 日志1532B，SHA256 `5c32039d8235dbed14697cad4266f772e51d04e5f3c6aea1874def1ea8131832`，exit1；after日志957B，SHA256 `5e9985b8137c948715fb83728a2081af3a2c3ab0565eabe1220ab7553235cea3`，exit0；均无signal、无截断。首个真实 before 日志SHA256 `615c680a8ff9da81fadedb7fbc70ac7c91d10666708383fae44d1c26d35dc26f`，错误测试 after日志SHA256 `ead406faadbbe6e1205fca06676256e6b64f52b8c4194bc1a8932174d6e5c012`。不可变 Host、配置、实际 observed.json、source freeze 和 runner/resource 收据分别保存，不提交原件或源码复制。

内部转发修复不新增示例调用或官网产品说明；必要长期说明归本 SDK 的原 extension-bridge 文档与 ADR0039。整项 A/B 文档、最终三语言、零警告 profile 和 main CI 仍分别待验。

## 已知缺口

header绑定精确候选的源码/consumer引用观察；上文TS旧Host红绿按原binary及日志固定，不借header更新冒候选三语言已运行。

上述before/after仅TypeScript及原opt0 Host切点；Python/Java准备不代实际运行。后继保存profile的零警告Host另有独立构建证据，不重标本历史语言结果，也不扩大为任意Graph late source、持久恢复或SDK最终验收。

### C1后继已执行范围

精确私有候选efc46f9b35fbcc8565e0a01eeb1ea2f40a01fcff/tree0cb01a5450de32e7501c2ce12638200297d6e442使用同一不可变8bf4a1d6deb647e608153c807ca07785dd31f7d3223a88cc3daf9b835b8bff5c Host，三语言late Completed各实际1pass、无失败/跳过；候选611语言源码输入前后零漂移，TypeScript候选重新build、Python明确导入候选src、Java freshXML。TS日志836B/SHA256 `2d0f71a32504d6da50e855a1ff5d497047e03f226cc177f2537016be7e19f3ce`，Python98B/SHA256 `8a5d3f8c1b452931de963c4bdff9775f7404361fdf3f79b4d740fb5a4f7e6e92`，Java946B/SHA256 `1a7ed15ea356a6efc1080e2a71cc49eccf2762028804ef5e3314fe6fee44ba90`，JavaXML1test/0failure/0error/0skip，SHA256 `9fb83718bf794b7b87af53c2be34dd5adf9d58dc4f54a0e7486a37721e7446c1`。完整原件只在同local；原TS red不重造，原阶段“Python/Java未运行”只保历史。

后继C1独审又发现同转发任务的满队列取消缺陷，原两chunk真实red后cancel/send select最小修复，同两case实际通过且增量复审pass。它改变共同生产cut，因此C1三语结果不能自动升级为后继cut；最终配置Host与direct/三native影响闭包仍在刷新，两个Finding先保持open。这里不以晚到foreign terminal创造Graph/Host Run回执。

### 最新共同cut的实际验证

在实际配置构建、无行为或warning flags覆盖的新Host14a0cb58c44bdb84990262f1324e1a2513513a8b40fc0640a9d36f91f95e27e9上，精确C2的三语言late Completed各重新实际1pass、0fail、0skip；没有用C1结果替代。TS日志844B/SHA256 `f9d448d7cfdfdb5a4a5df7d0a5acbeb8dc45079cea0f98ce512e7c87c2c9a63c`，Python98B/SHA256 `8a5d3f8c1b452931de963c4bdff9775f7404361fdf3f79b4d740fb5a4f7e6e92`，Java956B/SHA256 `f73994296150842ea9eab6d1aedf6395c30f419b39a55fd229ba83cd029d5bd9`；Java freshXML1test/0failure/0error/0skip，SHA256 `81104358250c6fd593d068d3d7ea7ff7b9f4093fbf875deb2b02a8a168b2d728`。所有runner exit0、无signal、无截断，611项候选语言输入前后零漂移，原peer的held/真实Completed/完整payload/seq1→seq2断言和共享oracle保持。

同实际共同Rust源码的不可变测试binary f6584098564c0d86e3068cb33741bbdeb3c6c2f892d7eca1b72cea4f24baf80c上，direct-run完整selector再次实际1pass，日志225B/SHA256 `b6af155b91bc48293540451f525cdf0f1e4b2b13a30e9da7bf58671b9d6ad77f`。4项Rust定向（direct1/unwind1/drain2）与三语各1分别统计，完整原件/输入/复制的freshXML仅在local的 `config-c2-runtime-results.json`及对应runner/log/provenance定位。该结果不覆盖独立Task/protected场景、#120重验、完整SDK平台门禁或main。
