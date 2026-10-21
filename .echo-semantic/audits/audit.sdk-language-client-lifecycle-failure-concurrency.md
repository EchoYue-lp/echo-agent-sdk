---
schema_version: 1
id: audit.sdk-language-client-lifecycle-failure-concurrency
kind: audit
boundary_ref: boundary.sdk-language-clients
lens: failure_concurrency
freshness: examined
revision: 76601b242d7c354352f3b5c7d4bf099873746b14
finding_refs:
- finding.sdk-default-stderr-backpressure
- finding.sdk-language-external-ack-projection-admission-ordering
- finding.sdk-factory-instance-routing
- finding.sdk-language-close-retry
- finding.sdk-language-consumption-ack
challenges:
  specific-default-stderr-backpressure:
    revision: source:a92b96b149d204822aa6ffdb42b512e7257293e2eb1a9511d46a169c87da578b
    source_refs:
    - sdks/python/src/echo_agent_sdk/client.py#spawn
    - sdks/python/tests/test_client_close_callbacks.py#test_default_spawn_inherits_diagnostics_without_a_private_pipe
    - sdks/typescript/src/client.ts#StdioChild
    - sdks/typescript/test/client-stdio-close.test.js#native diagnostic ownership
    - sdks/python/tests/test_telemetry_host.py#record_proxy_exit
    - sdks/typescript/test/telemetry-host.test.js#observeProxy
    - sdks/java/src/test/java/com/echoagent/sdk/TelemetryHostTest.java#observeProxy
    - sdks/shared/testing/telemetry_host_fixture.py#proxy-stderr-file
    evidence_refs:
    - evidence.sdk-source-continuity
  named-current-client-obligation:
    revision: 76601b242d7c354352f3b5c7d4bf099873746b14
    source_refs:
    - sdks/java/src/test/java/com/echoagent/sdk/AckSubmissionTest.java
    - sdks/java/src/test/java/com/echoagent/sdk/FactoryCleanupLifecycleTest.java
    - sdks/typescript/test/ack-submission.test.js
    - sdks/typescript/src/client.ts
    - sdks/python/src/echo_agent_sdk/client.py
    - sdks/python/src/echo_agent_sdk/execution_usage_values.py
    - sdks/java/src/main/java/com/echoagent/sdk/EchoAgentClient.java
    - docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/inputs-before.json
    - docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/inputs-after.json
    - docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/receipt.json
    - docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/manifest.json
    evidence_refs:
    - evidence.sdk-language-client-lifecycle-current
  specific-factory-instance-routing:
    revision: ad35887c5ceb66865a714f3f2d1da86a9c35c903
    source_refs:
    - sdks/typescript/test/factory-host.test.js
    - sdks/python/tests/test_factory_host.py
    - sdks/java/src/test/java/com/echoagent/sdk/FactoryHostTest.java
    - sdks/java/src/test/java/com/echoagent/sdk/FactoryCleanupLifecycleTest.java
    - docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/java-surefire/TEST-com.echoagent.sdk.FactoryHostTest.xml
    - docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/java-surefire/TEST-com.echoagent.sdk.FactoryCleanupLifecycleTest.xml
    evidence_refs:
    - evidence.sdk-language-client-lifecycle-current
  specific-language-close-retry:
    revision: ad35887c5ceb66865a714f3f2d1da86a9c35c903
    source_refs:
    - sdks/java/src/main/java/com/echoagent/sdk/SessionHandle.java
    - sdks/java/src/main/java/com/echoagent/sdk/ExtensionRegistration.java
    - sdks/typescript/test/protocol-host.test.js
    - sdks/python/tests/test_protocol_host.py
    - sdks/python/tests/test_client_close_callbacks.py
    - sdks/java/src/test/java/com/echoagent/sdk/ProtocolHostTest.java
    - docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/java-surefire/TEST-com.echoagent.sdk.ProtocolHostTest.xml
    evidence_refs:
    - evidence.sdk-language-client-lifecycle-current
  specific-language-consumption-ack:
    revision: ad35887c5ceb66865a714f3f2d1da86a9c35c903
    source_refs:
    - sdks/typescript/test/ack-submission.test.js
    - sdks/python/tests/test_ack_submission.py
    - sdks/java/src/test/java/com/echoagent/sdk/AckSubmissionTest.java
    - docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/java-surefire/TEST-com.echoagent.sdk.AckSubmissionTest.xml
    - docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/java-surefire/TEST-com.echoagent.sdk.ExternalAckHostTest.xml
    - docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/java-surefire/TEST-com.echoagent.sdk.ReplayTerminalHostTest.xml
    evidence_refs:
    - evidence.sdk-language-client-lifecycle-current
  specific-language-external-ack-projection-admission-ordering-before:
    revision: 4256bb8eb17637bf70b7bf1ef7408a98b6ce1264
    source_refs:
    - sdks/java/src/main/java/com/echoagent/sdk/EchoAgentClient.java
    - sdks/java/src/main/java/com/echoagent/sdk/BoundedPublisher.java
    - sdks/java/src/test/java/com/echoagent/sdk/ExternalAckHostTest.java
    - sdks/typescript/src/client.ts
    - sdks/typescript/test/ack-submission.test.js
    - sdks/python/src/echo_agent_sdk/client.py
    - sdks/python/tests/test_ack_submission.py
    evidence_refs:
    - evidence.sdk-language-client-lifecycle-current
  specific-language-external-ack-projection-admission-ordering-after:
    revision: 69cbf73831775cb53c13efbcfe20541b5c6f20ef
    source_refs:
    - sdks/java/src/main/java/com/echoagent/sdk/EchoAgentClient.java
    - sdks/java/src/main/java/com/echoagent/sdk/BoundedPublisher.java
    - sdks/java/src/test/java/com/echoagent/sdk/ExternalAckHostTest.java
    - sdks/java/src/test/java/com/echoagent/sdk/AckSubmissionTest.java
    - sdks/java/README.md
    - sdks/typescript/src/client.ts
    - sdks/typescript/test/ack-submission.test.js
    - sdks/python/src/echo_agent_sdk/client.py
    - sdks/python/tests/test_ack_submission.py
    evidence_refs:
    - evidence.sdk-language-client-lifecycle-current
---

# 客户端 并发提交、取消与独立会话 有限复审

## 审查范围

独立 reviewer `/root/sdk_capability_acceptance_audit` 实际读取 TSdd822、PY30ca/cd076、Java86efd及原790输入/71XML。此文件回收本次已完成审查，不制造旧运行时刻或新的native执行。源码与正式证据均绑定本次真实76601提交；原执行时间/切点保持。

## 已检查故障假设

ACK乱序/取消pending写不充洞；earlyACK/terminal不复活取消地址，跨Session feeds隔离，Factorycleanup同owner coalescing。

## 实际实现路径与证据

原失败归档中ACK18、Routes4、Factory25、ParallelSession/Lifecycle具名断言已通过，未用后继focused覆原whole。

## 问题记录

此具名客户端义务审查 PASS C0/I0/M0，freshness=examined不等于全SDK verified。当前TS/Java stdio Finding的candidate证据链各自保留，不由此Audit批量关闭；原Task HTTP失败保历史；受控连接fixture修复/同入口重跑0独立，不从该单例推通用Store权威。

## 残余风险

完整合同生成、12身份选择、5秒物理结算、跨平台及SDK main独立。原完整命令exit1不改成0；新同入口whole0有独立收据，后继focused不替原whole XML。

## 未检查项

上述限定scope之外的 framework Task物理结算/Store HTTP根因和所有未接受Rust identity；不新增审查分母。

## 原测试来源定位限制

以下辅助测试源码未在76601提交，故不列为该Git的source_refs；其原运行/断言名与精确hash由790执行输入和原XML保留，未删除故障义务：sdks/python/tests/test_parallel_sessions_host.py。原测试并未因缺blob而被宣称在该Git完整恢复。

## factory-instance-routing具名复审范围

原scope独立reviewer本session逐源码/实际native断言核对：三native5 distinct child/same descriptor、parallel closeA/B live/parent再构造、PY invalid factoryorigin stalegeneration零handler effect。Hostfactory_origin exactrouting与真实callback匹配，不用managedstore/LlmGraph代。 本条是原业务审查真实性回收，不另造engine执行。对应故障项可局部闭；主revision与原challenge仍保原76601观察，新challenge以ad实际提交源码/原运行收据定位，不把旧时间重标。

## language-close-retry具名复审范围

原scope独立reviewer本session逐源码/实际native断言核对：真实session/agent close以及registration/run失败重试后原callback继续live，不是仅nativeprocess closeFuture。保原owner直到确认、同ownerretry/coalescedwait；新TS initFailureCatch漏child另scope不被此局部闭合覆盖。 本条是原业务审查真实性回收，不另造engine执行。对应故障项可局部闭；主revision与原challenge仍保原76601观察，新challenge以ad实际提交源码/原运行收据定位，不把旧时间重标。

## language-consumption-ack具名复审范围

原scope独立reviewer本session逐源码/实际native断言核对：原officialwriter completion提交消费而非fetch/nextpending就消费；gap不autoACK、显式snapshot/replaycoverage，failed/cancelledpending不commit、terminalcoverage自然EOF。ACKv4原独review+18/4/1强assert不是counts单证。 本条是原业务审查真实性回收，不另造engine执行。对应故障项可局部闭；主revision与原challenge仍保原76601观察，新challenge以ad实际提交源码/原运行收据定位，不把旧时间重标。


## 本次ACK竞态的有限复审

原26c Linux Java窗口及三语各自受控before→after按复用Evidence本次段保存。Java2e0由FrameFinal实际源码/after审PASS；TS/PY69cb由SDKcap实际source/postMinor/after审PASS。69cb是执行时2e0dirty精确四源的后保存观察，不冒新执行时间；成功投影与容量进展、失败/pending不消费、取消/关闭/身份边界及版本/总transport内存限制均保。只此竞态localresolved，原top76601/旧challenge/旧Finding义务不改，不推全语言/新Linux/七open/continuity绿。


## 2026-10-22 默认stderr背压的有限复审

边界为language-clients，视角为failure_concurrency。可反证假设是默认启动在initialize前
创建无owner的stderr pipe，诊断写满后阻止握手；Python19和TS21B各独立实际反例、
同原future/child在sole drain后恢复，非timeout/重启/强杀归因，21a01晚EOF UNKNOWN保留。
当前具体修复复用原ACP/Node spawn的stderr继承，stdout/stdin和原close body保持，
显式TS callback仍有原读者。对PY20默认三组+原七、TS22默认三组/callback一组+原十二，
逐原件核自然exit/完整stderr/stdout隔离；对24实际五场另核Rust child0、真正proxy0、
terminal后完整stderr、held-export/ACK/collector llm.calls，Root及独立读者C0/I0/M0。

TS shutdown waiter在这些关闭场景实际得到transport_error或closed_handle并settled，
不是成功shutdown响应；物理flush结论来自Host自然0、实际collector/ACK和原屏障。
input_eof在hold期间没有活动publicclose waiter，其close_settled=false是尚未调用close的
初值，不当作一个未完成close操作。public_close与rpc_cancel按各自原调用/取消身份断言结算。

仅本Finding的repair/verification/rereview闭合。主revision76601、其它challenge/97Audit、
Map/Behavior的整体状态与managed历史UNKNOWN均保；原完整语言1和完整whole/平台/main
未验不由focused替代。父应用stderr被其自身重定向时的sink消费与任意callback Unicode
分块语义未被本项泛推保证；本次callback精字节验证使用原2MiB ASCII输入。
