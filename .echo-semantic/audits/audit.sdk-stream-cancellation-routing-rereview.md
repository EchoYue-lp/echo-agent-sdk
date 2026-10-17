---
schema_version: 1
id: audit.sdk-stream-cancellation-routing-rereview
kind: audit
boundary_ref: boundary.sdk-language-clients
lens: failure_concurrency
freshness: examined
revision: fa62f3b3d0df3ed5b14c6d9786dfffce5db57da6
finding_refs:
- finding.sdk-task-resume-extension-invocation-conflict
challenges:
  original-matching-task-source-and-exact-cas:
    source_refs:
    - sdks/shared/testing/task_subagent_fixture.py
    - sdks/typescript/test/task-subagent-host.test.js
    - sdks/python/tests/test_task_subagent_host.py
    - sdks/java/src/test/java/com/echoagent/sdk/TaskSubagentHostTest.java
    evidence_refs:
    - evidence.sdk-task-subagent-candidate
  actual-source-and-consumer-boundary:
    revision: 56929154c1851473e9e4837901b610cecf001b2d
    source_refs:
    - docs/sdk/governance/evidence/sdk-stream-cancellation-routing/README.md
    - docs/sdk/governance/evidence/sdk-stream-cancellation-routing/final-source-manifest.json
    - docs/sdk/governance/evidence/sdk-stream-cancellation-routing/evidence-sha256.json
    - docs/sdk/governance/evidence/sdk-stream-cancellation-routing/client-routing-source-slice.patch
    - docs/sdk/governance/evidence/sdk-stream-cancellation-routing/before-source-manifest.json
    - docs/sdk/governance/evidence/sdk-successor-model-reviews/stream-cancellation-routing.json
    - sdks/typescript/src/client.ts
    - sdks/python/src/echo_agent_sdk/client.py
    - sdks/java/src/main/java/com/echoagent/sdk/EchoAgentClient.java
    - docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/final-receipt.json
    - docs/sdk/governance/evidence/sdk-language-factory-final/receipt.json
    - docs/sdk/governance/evidence/sdk-language-factory-final/source-before.json
    - docs/sdk/governance/evidence/sdk-language-factory-final/source-after.json
    evidence_refs:
    - evidence.sdk-stream-cancellation-routing-repair
    - evidence.sdk-stream-cancellation-routing-verification
---

# stream-cancellation-routing-rereview

## 审查范围

当前header绑定formal-only Task后继候选；原569等routing审查仍按其具名revision，不将旧source_refs或raw升级为当前执行。

sdk_capability_acceptance_audit actual9source/27archiveSHAs/3nativebeforeafter andTS19PY39Java27 finitePASS0C0I0M noNewstateauthority; frameworkbeyondgrace physicalclaimexcluded.

## 已检查故障假设

旧wholecounts覆盖后继源码、typedwrapper丢字段、默认成功掩失败、callback mock替真实Host、跨epoch错误借用。

## 实际实现路径与证据

具名源码、原receipt与rawhash按引用读取；scope限定结论不扩为全部SDK审查。

## 问题记录

见独立原结论及关联Finding；不同缺陷不继承此pass。

## 残余风险

当前最终合同生成、无patchpin与whole门禁/main单独待验。

## 未检查项

scope外操作、任意组合与后继Channelforward新实现。

后继matching Task三语源码增量及依赖分离已由原reader审查：真实原callback/claim派生execution/stream/invocation/PID、同source真实join与唯一exactCAS、C/D原canonical依赖prompt、B execution不重写；共享verifier不生成状态或调度。正式ADR与API证明wave边界，原超约oracle不冒产品red。候选去foreign测试尾部，保原四经典例及新matching例；Model仅窄拒绝意外调用，无新driver。原S单例及独立候选整个Task suite的运行由原Evidence分别绑定，不借S子集称候选全绿；宽Task/cleanup、超grace、restart、外部平台及main仍不在本有限通过范围。

## 本次 current_source_rereview

当前SDK真实Git`56929154c1851473e9e4837901b610cecf001b2d`/source17cf，独立reader`framework_final_review`确实重新读取原业务故障假设、当前路径/变化owner/cfg/aliases和现14/newbb4f强场景，消息回收UTC`2026-10-14T00:39:15.355735+00:00`不是原运行时间。结果：**PASS / 0C0I0M（本次有界源码业务复审）**。原CodeAudit仍业务角色，不降为receiptAudit；原正文/历史sourcemarker`source:pending`/旧rawepoch保。

本次已检查：SetupACK retains originalcancel address complete streamid/gen/kind/extension;earlycancel/terminalbeforeACK mustnot resurrect route；Only officialterminalsuccessful submission orsuccessfulunregister/actualclose releasesexactowner;wronggenfailedsend/failedclose keepsit。当前函数定位：invocationControllers；extensionStreamRoutes；stream；invocation_cancel；invocationCancellations；extensionStreamRoutes。实际afterproof范围：Current3nativebeforeafter +routewindows/task4 each/ACK12/23/18;exclude generic>5secphysical。

上方追加source_refs逐569Gitblob=currentfileSHA精确，现14/newbb4f632+54 sourcecut实际范围匹配。原FaultHypothesis文本保留并在本次业务代码复核，源码变化闭包按原scope检查；statusexamined不等全无缺陷，supports/limits/41coverage/closure不提升。Framework业务源属于独立publishedGitb909，对native工具用既有pin/buildmanifest定位，不将framework文件伪置SDKtree。原历史整仓3ea/9e缺失仍未知，不把原run重新标为569。full28gen/三兼容人选/LinuxWindows/main仍分开。


## 来源身份的无损持久化

原 header 的 source:31483b056427dc3f763899b798d84992cc0301f6bfe1816ccc8acf2952bee11d 与公开 Git fa62f3b3d0df3ed5b14c6d9786dfffce5db57da6 的 canonical source digest 精确相同；只把该 header 改为可冷取回 Git 身份。该提交的原 source_refs 逐 blob 取回核验，不改变正文、原 F988/Host14a 五例结果或审查范围；它不证明后继 F6563/ac433，也不纳入工作树四个 foreign Task 文件。必要 exact-SHA annotated evidence tag 的远端与冷仓回执仅保 ignored local。
