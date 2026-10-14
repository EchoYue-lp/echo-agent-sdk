---
schema_version: 1
id: evidence.sdk-language-client-lifecycle-current
kind: evidence
observed_at: 76601b242d7c354352f3b5c7d4bf099873746b14
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
supports:
- behavior.sdk-language-clients
limitations:
- Client wire error fidelity does not prove Host reverse Tool/LlmClient preserves Rust cancellation/timeout variants or Session/Run idempotency-key semantics; those separate adapter defects remain outside this closure.
- Eight named client review lenses examined do not mean a successful whole-language gate or full SDK completion.
- The original a634 whole command exited1 with a Task pause reverse Store HTTP error; its original71 XML and790 identical inputs are preserved.
- Prior b89/3aa and earlier569/be5 proofs retain their epochs; no old execution is relabelled as this revision.
- Accepted12 identity migration, five-second physical settlement, full generator, platform CI and SDK main remain separate.
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
