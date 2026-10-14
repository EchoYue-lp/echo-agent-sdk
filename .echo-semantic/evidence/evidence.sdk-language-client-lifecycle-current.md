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
- docs/sdk/governance/evidence/typescript-stdio-close/deferred-eof-final/manifest.json
- docs/sdk/governance/evidence/typescript-stdio-close/deferred-eof-final/SHA256SUMS.json
- docs/sdk/governance/evidence/telemetry-host/publicclose-current-source/receipt.json
- docs/sdk/governance/evidence/telemetry-host/publicclose-current-source/SHA256SUMS.json
- docs/sdk/governance/evidence/sdk-java-host-graceful-close/receipt.json
- docs/sdk/governance/evidence/sdk-java-host-graceful-close/manifest.json
- docs/sdk/governance/evidence/sdk-java-host-graceful-close/independent-review.json
- docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/receipt.json
- docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/manifest.json
- docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/inputs-before.json
- docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/inputs-after.json
- docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/java-surefire/TEST-com.echoagent.sdk.TelemetryHostTest.xml
- sdks/typescript/test/factory-host.test.js
- sdks/python/tests/test_factory_host.py
- sdks/java/src/test/java/com/echoagent/sdk/FactoryHostTest.java
- sdks/java/src/test/java/com/echoagent/sdk/FactoryCleanupLifecycleTest.java
- docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/java-surefire/TEST-com.echoagent.sdk.FactoryHostTest.xml
- docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/java-surefire/TEST-com.echoagent.sdk.FactoryCleanupLifecycleTest.xml
- sdks/java/src/main/java/com/echoagent/sdk/SessionHandle.java
- sdks/java/src/main/java/com/echoagent/sdk/ExtensionRegistration.java
- sdks/typescript/test/protocol-host.test.js
- sdks/python/tests/test_protocol_host.py
- sdks/python/tests/test_client_close_callbacks.py
- sdks/java/src/test/java/com/echoagent/sdk/ProtocolHostTest.java
- docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/java-surefire/TEST-com.echoagent.sdk.ProtocolHostTest.xml
- sdks/typescript/test/ack-submission.test.js
- sdks/python/tests/test_ack_submission.py
- sdks/java/src/test/java/com/echoagent/sdk/AckSubmissionTest.java
- docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/java-surefire/TEST-com.echoagent.sdk.AckSubmissionTest.xml
- docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/java-surefire/TEST-com.echoagent.sdk.ExternalAckHostTest.xml
- docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/java-surefire/TEST-com.echoagent.sdk.ReplayTerminalHostTest.xml
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
- docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/java-surefire/TEST-com.echoagent.sdk.ProtocolIntegrityTest.xml
supports:
- behavior.sdk-language-clients
limitations:
- Client wire error fidelity does not prove Host reverse Tool/LlmClient preserves
  Rust cancellation/timeout variants or Session/Run idempotency-key semantics; those
  separate adapter defects remain outside this closure.
- Eight named client review lenses examined do not mean a successful whole-language
  gate or full SDK completion.
- The original a634 whole command exited1 with a Task pause reverse Store HTTP error;
  its original71 XML and790 identical inputs are preserved.
- Prior b89/3aa and earlier569/be5 proofs retain their epochs; no old execution is
  relabelled as this revision.
- Accepted12 identity migration, five-second physical settlement, full generator,
  platform CI and SDK main remain separate.
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
