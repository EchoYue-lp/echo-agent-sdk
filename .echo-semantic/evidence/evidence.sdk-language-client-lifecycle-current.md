---
schema_version: 1
id: evidence.sdk-language-client-lifecycle-current
kind: evidence
observed_at: ad35887c5ceb66865a714f3f2d1da86a9c35c903
source_refs:
- sdks/typescript/src/client.ts
- sdks/python/src/echo_agent_sdk/client.py
- sdks/python/src/echo_agent_sdk/execution_usage_values.py
- sdks/java/src/main/java/com/echoagent/sdk/EchoAgentClient.java
- sdks/shared/toolchain.json
- sdks/typescript/test/factory-host.test.js
- sdks/python/tests/test_factory_host.py
- sdks/java/src/test/java/com/echoagent/sdk/FactoryHostTest.java
- sdks/java/src/test/java/com/echoagent/sdk/FactoryCleanupLifecycleTest.java
- sdks/java/src/main/java/com/echoagent/sdk/SessionHandle.java
- sdks/java/src/main/java/com/echoagent/sdk/ExtensionRegistration.java
- sdks/typescript/test/protocol-host.test.js
- sdks/python/tests/test_protocol_host.py
- sdks/python/tests/test_client_close_callbacks.py
- sdks/java/src/test/java/com/echoagent/sdk/ProtocolHostTest.java
- sdks/typescript/test/ack-submission.test.js
- sdks/python/tests/test_ack_submission.py
- sdks/java/src/test/java/com/echoagent/sdk/AckSubmissionTest.java
- sdks/typescript/src/errors.ts
- sdks/java/src/main/java/com/echoagent/sdk/EchoAgentException.java
- sdks/typescript/src/wire.ts
- sdks/python/src/echo_agent_sdk/wire.py
- sdks/java/src/main/java/com/echoagent/sdk/WireValues.java
- sdks/typescript/test/wire.test.js
- sdks/python/tests/test_wire.py
- sdks/java/src/main/java/com/echoagent/sdk/JsonSupport.java
- sdks/shared/fixtures/native-projection-invalid.json
- sdks/typescript/test/protocol-integrity.test.js
- sdks/python/tests/test_protocol_integrity.py
- sdks/java/src/test/java/com/echoagent/sdk/ProtocolIntegrityTest.java
supports:
- behavior.sdk-language-clients
limitations:
- Client wire error fidelity does not prove Host reverse Tool/LlmClient preserves Rust cancellation/timeout variants or Session/Run idempotency-key semantics; those separate adapter defects remain outside this closure.
- Eight named client review lenses examined do not mean a successful whole-language gate or full SDK completion.
- The original a634 whole command exited1 with a Task pause reverse Store HTTP error; its original71 XML and790 identical inputs are preserved.
- Prior b89/3aa and earlier569/be5 proofs retain their epochs; no old execution is relabelled as this revision.
- Accepted12 identity migration, five-second physical settlement, full generator, platform CI and SDK main remain separate.
asset_refs:
- asset.sdk-historical-proof-location
---

# 三语言客户端具名生命周期审查

## 支持的结论

本次独立 reader `/root/sdk_capability_acceptance_audit` 对三个实际客户端的八个风险视角逐项读取源与原运行材料，结论为有界 examined、Critical0/Important0/Minor0。TS dd822f73、Python client30ca4fb5/最低版本import cd07640f、Java86efd395 的精确SHA见本 scope 原清单。唯一业务权威继续为framework；客户端只拥有消费水位、原发送receipt、取消地址与原transport/cleanup owner，不生成第二终态或权限策略。

这不是由测试数直接推断无缺陷。每个故障假设、源码入口和实际断言由8份窄Audit记录；原TS10真实child与same-source public-close、Java19 lifecycle+72相关及原whole XML中的2个Telemetry case、Python官方process/context/最低版本均各保自己的证据范围。已审场景足以记录当前客户端这一切片，不以未定义的全部组合无限扩大目标。

## 来源与范围

本次观察绑定Root真实提交76601；执行时刻仍以原收据为准，不宣称原命令是在此后提交时运行。四个执行源均已与790个官方运行输入before/after逐项核对；其原命令日志106585bytes，exit1。原original-surefire目录有71XML；TelemetryHostTest原XML真实2tests/0failure/0error/0skip，分别为 publicCloseWaitsForRealSlowTelemetryFlush 与 acceptedTelemetryExportsRealBusinessRecordsAndFlushes（后者内部console两配置，不虚记3个JUnit case）。没有拿后继focused替换原XML；完整同入口后继成功另列自己的收据。

## 已知缺口

原完整语言入口a634曾exit1，Task pause reverse Store HTTP错误保持原失败收据。后继只声明受控fixture响应为nonpersistent，再同入口实际exit0；该前后实验支持Connectionclose机制，不冒称原随机socket具有直接因果日志。旧569或be5所写泛仍缺保历史，新的current模型只收窄到已审8lens；源版本现绑定真实76601，不冒3cd partial seed已经完整生成。Finding本身要独立repair/verification/rereview链，不能因本表批量resolved。12源/kind与5秒B决定、完整生成、跨平台CI/main继续独立。

## 本次完整语言后继切点

正式同入口重跑exit0，100567bytes SHA af652b11，790输入前后同；TS308/PY375+18/Java332及71XML/3quickstarts均通过。实际Host193e49b8/frameworka634。原失败日志98d714/167项archive保留，TaskHTTP fixture仅显式声明nonpersistent Connection close；受控旧sameconnection错误→new不同连接通过，不冒称原socket直接因果链。源版本76601与4client原SHA逐Gitblob精确一致，完整生成/main/12与5秒选择仍独立。

## 原取消路由证据出处 (historical)

docs/sdk/governance/evidence/sdk-stream-cancellation-routing/final-source-manifest.json : 原早ACK消费源和取消地址断言保留其原证据SHA及切点，此文件尚未入本次76601，不冒充该Git的blob。

## 五项旧客户端逻辑缺陷的精确闭合范围

本次SDK capability独立reviewer逐原defect/refscope核实际业务修复、原namedconsumer断言和已完成source审，不只靠whole0或单一lifecycleAudit。现正文三链可同一个共享Evidence含repair/verification，但每项引用对应精确Audit视角；原failedrun及新negotiation/TS initcleanup独slice保持。

### factory-instance-routing

三native5 distinct child/same descriptor、parallel closeA/B live/parent再构造、PY invalid factoryorigin stalegeneration零handler effect。Hostfactory_origin exactrouting与真实callback匹配，不用managedstore/LlmGraph代。

对应原运行/fixture引用：sdks/typescript/test/factory-host.test.js, sdks/python/tests/test_factory_host.py, sdks/java/src/test/java/com/echoagent/sdk/FactoryHostTest.java, sdks/java/src/test/java/com/echoagent/sdk/FactoryCleanupLifecycleTest.java, docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/java-surefire/TEST-com.echoagent.sdk.FactoryHostTest.xml, docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/java-surefire/TEST-com.echoagent.sdk.FactoryCleanupLifecycleTest.xml。相关review视角：state-authority, failure-concurrency。原/sourcepatch执行切点和原缺陷cause保持，不声称语义观察ad提交时重新运行命令。

### language-close-retry

真实session/agent close以及registration/run失败重试后原callback继续live，不是仅nativeprocess closeFuture。保原owner直到确认、同ownerretry/coalescedwait；新TS initFailureCatch漏child另scope不被此局部闭合覆盖。

对应原运行/fixture引用：sdks/java/src/main/java/com/echoagent/sdk/SessionHandle.java, sdks/java/src/main/java/com/echoagent/sdk/ExtensionRegistration.java, sdks/typescript/test/protocol-host.test.js, sdks/python/tests/test_protocol_host.py, sdks/python/tests/test_client_close_callbacks.py, sdks/java/src/test/java/com/echoagent/sdk/ProtocolHostTest.java, docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/java-surefire/TEST-com.echoagent.sdk.ProtocolHostTest.xml。相关review视角：time-lifecycle, failure-concurrency。原/sourcepatch执行切点和原缺陷cause保持，不声称语义观察ad提交时重新运行命令。

### language-consumption-ack

原officialwriter completion提交消费而非fetch/nextpending就消费；gap不autoACK、显式snapshot/replaycoverage，failed/cancelledpending不commit、terminalcoverage自然EOF。ACKv4原独review+18/4/1强assert不是counts单证。

对应原运行/fixture引用：sdks/typescript/test/ack-submission.test.js, sdks/python/tests/test_ack_submission.py, sdks/java/src/test/java/com/echoagent/sdk/AckSubmissionTest.java, docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/java-surefire/TEST-com.echoagent.sdk.AckSubmissionTest.xml, docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/java-surefire/TEST-com.echoagent.sdk.ExternalAckHostTest.xml, docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/java-surefire/TEST-com.echoagent.sdk.ReplayTerminalHostTest.xml。相关review视角：data-durability, failure-concurrency。原/sourcepatch执行切点和原缺陷cause保持，不声称语义观察ad提交时重新运行命令。

### language-error-fidelity

三actualHost stale_handle opaque完整handle与code/retryable/op/details保；ProtocolIntegrity从官方ACPerror解完整tuple。这里只客户端errorprojection，不原Rust远端Tool/LLM原因分类或新negotiation广告缺陷。

对应原运行/fixture引用：sdks/typescript/src/errors.ts, sdks/python/src/echo_agent_sdk/client.py, sdks/java/src/main/java/com/echoagent/sdk/EchoAgentException.java, sdks/java/src/test/java/com/echoagent/sdk/ProtocolHostTest.java, docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/java-surefire/TEST-com.echoagent.sdk.ProtocolHostTest.xml。相关review视角：result-side-effect。原/sourcepatch执行切点和原缺陷cause保持，不声称语义观察ad提交时重新运行命令。

### wire-data-fidelity

原13sharedinvalidcorpus三语严格拒nonstring/duplicate/missing/unknownmalformed；actualHost memory特殊__proto__/constructor+中文emoji原值exact不defaultempty。wire封闭代数与prototype安全，不把Java新advertarray静默drop当该closedwire已修。

对应原运行/fixture引用：sdks/typescript/src/wire.ts, sdks/python/src/echo_agent_sdk/wire.py, sdks/java/src/main/java/com/echoagent/sdk/WireValues.java, sdks/typescript/test/wire.test.js, sdks/python/tests/test_wire.py, sdks/java/src/test/java/com/echoagent/sdk/ProtocolHostTest.java。相关review视角：contract-evidence, result-side-effect。原/sourcepatch执行切点和原缺陷cause保持，不声称语义观察ad提交时重新运行命令。


## 本次五缺陷观察的版本性质

observed ad358是这次明确五defect复核与已保存历史收据的定位，不声称旧整语言命令在ad新commit时重新执行。闭合wire13非法corpus的Java实际decoder是JsonSupport，构造codecWireValues不单独替解析；官方AcpError tuple的直接consumer为ProtocolIntegrityTest，三语言fixtures/测试与原XML9具体refs追加，原76601主challenge字节不变。

## 本任务全目标差异的精确路径依据补齐

此段只给本owner对应实际证据/consumer的可复核输入性质，不定义新runtime authority、不新增接受API、不用路径数量证明行为。原observed_at ad35887c5ceb66865a714f3f2d1da86a9c35c903、原命令时间/Host/数据/限制全部不变；可直接列入source_refs的历史取证副本已在该原观察Git逐blob读取存在，其余当前test输入另由真实db34a76226732d50892be5718cd48836576aea9c和当前完整官方语言包保存，不冒旧运行时刻。

|准确路径|输入角色与范围|当前保存的原字节SHA|
|---|---|---|
|docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/executed-source/sdks/python/src/echo_agent_sdk/client.py|原before/after/contract-basis源码取证副本；只解释此owner原故障/输入，不执行/注册/拥有新状态|30ca4fb5f2a008160290fe3664d16bb70b2d168d57b693f23bab6f2a9628e38e|
|sdks/python/tests/test_execution_usage_values.py|该能力真实native测试或共同fixture，当前输入证据；不把Ruff/输入哈希单独当行为PASS|f2265b12c574ee104c502de3e66f24a0898b535f003bb8dbeba118454f568b96|

本owner表中runtime test/fixture共1项。它们是当前官方whole793的同源输入及各组具名独立证明的定位，不从输入哈希/Ruff本身推业务PASS；当前SourceGit在db34a76226732d50892be5718cd48836576aea9c可恢复。归档项只作为原before/after/contract witness，不执行、不产生第二实现/validator。Java/Python版本兼容与原门禁仍各用实际原收据，旧supports/limitations/运行时间不改，不声称所有族fresh。


## 本次三语言ACK竞态的before证据

原Java Linuxhead26c/PR merge ed05c8e6c91fbcde67d955933cb28fb326c7f144，官方[job116253904798](https://github.com/EchoYue-lp/echo-agent-sdk/actions/runs/38683776490/job/116253904798)：ExternalAckHostTest4/1failure/0error/0skip，window1在line109报不可恢复event_gap。Admin原log sdk-26c781e-ci-evidence/116253904798.log（204864 bytes，SHA2108cdd28bb4aff26748c522b2d6e42cea07d60379c2c8660a8f93103a26be4f）。Java原三源4256逐字节同26c；旧client32c4768b/pub1d261d81/testea174e4a。Admin sdk-java-external-ack-race/phase1-3-summary.json（1996 bytes，SHAb146207869df0102405b8cd8d8e9c049e505d681690fa9b7450e8969502d8733）区别真实Host0f1成功observer进入但未retire时event3溢出，与official AcpClientSession native outcome pending的机制反例；后者不是Host E2E。未修改自然方法两window行一次通过不证明race absent。

TS/PY官方mechanism counter（不是物理Host）实际source4256与2e0不变：ACK2 peer可见且native completion被hold→event3→capacity1永久失败，放行ACK成功后floor2也不恢复event3。SDKcap已独审counter validity PASS。TS typed backpressure，result SHA7c7dea0631df557dd3320f779feec9537c6717ffe549d4252095a551fb1e031a，原log497 bytes SHAa27e40680c045c73744de8f1ec8b1598f4fd65941d748e84ee7bdb3aabfb20d7；PY typed event_gap/details.reason=client_queue_full，result SHAecc5a4ae4ea58a1c1c1b7577c7e0444556caf13a5e055e2959374b45c98dc311，log1052 bytes SHA5a2d549e040af8663e7dd8472196fc3fd46150153f06a2e10495262ed6693071。原文件在 `/Users/ls/MyWork/code/ylp_agent_learn/lp-agent/.worktrees/admin/sdk-ts-python-ack-capacity-race/`，summary SHA780a0cac13cf8c36566139cda3fd7a84b248c41bdcdb8248c0b8f2a5dad3ba17，原counter源码/结果17项seal保留，不把Admin路径伪Git source_ref或新建S raw副本。其TS原client611b1a87/testa2d74349，PY原client399961d0/teste61b00e4，按4256实际Git恢复；完整旧输入SHA在原before-source.json/summary保存。

## 三语言ACK竞态的局部修复、验证和独立复审

本段当前源码保存观察为69cbf73831775cb53c13efbcfe20541b5c6f20ef，不修改原observed_at ad358/旧source_refs/命令/限制；原before证据不改为通过，较早未安装open候选保Admin原字节。现仅本具体竞态具局部三链，不宣全部语言/Host行为通过、永久有界内存或新Linux全套完成。

Java修复2e0保持原官方通知顺序，publisher成功消费投影先退休、后释放相关准入，复用同owner容量进展/关闭语义；无新容量、队列或Framework调度权威。原受控Host before1→after0，最终49和相邻28各0；reverse-final-receipt与adjacent-receipt分别保最终源及运行角色，FrameFinal已实际源码+after有限PASS0。Oracle21/release17编译不等Linux17后验。

TS/PY修复69cb保actual native success-only消费、发送owner不受caller cancellation中断，准入依赖原ACK投影/原队列容量变化而不是提前丢prefix；保持原typed error、failure/pending不消费、正常需求/取消/关闭与同stream身份边界。Python pre-first-step cancel原实际红后修复清理，未降低最低3.10。SDKcap已实际source/postMinor有限PASS0。原persistent两语behavior before红→同条件after0，初persistent测试字节未单独快照的限制诚实保留；原完整受控before原源及17项seal/raw仍在Admin，不虚构缺失快照。原TS/PY官方mechanism before不是物理Host before。

紧凑执行索引 `/Users/ls/MyWork/code/ylp_agent_learn/lp-agent/.worktrees/admin/sdk-ts-python-ack-capacity-race/implementation/verification-index.json`，SHA5521c0b4805150d0b646be4fe244cefe3e4487e0c904faa409c472d3ebdd66a5；其中运行时HEAD2e0及dirty实际四源由postminor/source-freeze.json保存，observedHeadAfterGates69cb仅后保存观察。TS最终61 ACK/Protocol/stdio-close、Node20 ACK19、原注册handler受控2和物理Host window1/2各0；PY3.12.4/ACP0.12.1三files72、PY3.10.19 ACK34、postMinor物理Host2及Ruff均0。较早两版本三files71属于preMinor切点，不冒最终四源；TS真实Host原test成功会清工作目录，仅实际TAP/强断言源码/receipt保存，PY原notifications保其native目录。所有raw仅Admin，未新建SDK归档包。

当前精确源码索引（各路径在69cb Gitblob/current SHA一致，Java源也精确存在2e0）：

|路径|SHA256|
|---|---|
|`sdks/java/src/main/java/com/echoagent/sdk/EchoAgentClient.java`|`0e3181817d84578c51efb60ce4fafbe8be9e599396ab679dd8a9bacc2b06ef28`|
|`sdks/java/src/main/java/com/echoagent/sdk/BoundedPublisher.java`|`7a31941c2e23fca16455cd06abbdc354ef9ef100934836ff435d23e885f64571`|
|`sdks/java/src/test/java/com/echoagent/sdk/ExternalAckHostTest.java`|`1b04e84b3af4bc231a881c221b48e0ca96f1afaedd5cc6cd640372556aebf55e`|
|`sdks/java/src/test/java/com/echoagent/sdk/AckSubmissionTest.java`|`27203e61b204edd92a9bd15b9e3792f5ace75204a5cd8402cc2a344fb0b497a6`|
|`sdks/java/README.md`|`d692aab9947c1457f85c09c55a83185d4fe3513230277e68bcf9af70986c6890`|
|`sdks/typescript/src/client.ts`|`dd94c5af88343323a599622e3baafa991fdc2a8ae712cd1b98a3923d9e7474e1`|
|`sdks/typescript/test/ack-submission.test.js`|`506902c45d96c0dc4fe841f574ed7f46ad022e32deeea2af8de302b0a320b63f`|
|`sdks/python/src/echo_agent_sdk/client.py`|`2c1174c46dcfb063510db2869fe1de8702504d85d54000afc399bfc1d1a532fe`|
|`sdks/python/tests/test_ack_submission.py`|`67fabad728818f1a2437617edfeccc8ec0d3a54f6be80d310618b321eba21efc`|

本局部Finding repair/verification引用本Evidence具体段，rereview引用对应failure-concurrency具名69cb challenge；旧language-consumption-ack闭合范围与#120原framework权威不重开。PID/Gap/Host未知及其余七open/B3/12/合同13/平台/main/payload必要性和全局continuity1173阻断均保。当前唯一状态页本次只维护既有结论与新69cb范围：`docs/sdk/governance/capability-review-remaining.md`，候选SHA `130ec38df8ea338a4d006cf3dce37c89a92efc149fac8d8c68c629d8f000d7a3`；它是文稿维护输入依据，不是新runtime通过证明。未新增公共API/accepted schema，examples及官网无需外仓接口改动；JavaREADME已随2e0同步。未来69cb新Linux完整语言/CI与当前原30d语义门禁必须各有新actual，不能借旧26c或本地focused替代。


## 历史证明存储的有界表示迁移

原id/业务正文/status/supports/limitations/命令及原观察含义不改变；本冻结父 `7f14a1b9c751b0612ceb42224f83f29d09aea46c` 中此对象完整原header/source refs可由公开证据tag恢复。只将具体archive物理依赖交当前位置资产与真实same-role consumer，全部旧义务由显式Eq/CT承接，不把存储重观察冒原业务新审或旧失败变成功。
