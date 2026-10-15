---
schema_version: 1
id: audit.sdk-language-client-lifecycle-contract-evidence
kind: audit
boundary_ref: boundary.sdk-language-clients
lens: contract_evidence
freshness: examined
revision: 76601b242d7c354352f3b5c7d4bf099873746b14
finding_refs:
- finding.sdk-wire-data-fidelity
challenges:
  named-current-client-obligation:
    revision: 76601b242d7c354352f3b5c7d4bf099873746b14
    source_refs:
    - sdks/shared/toolchain.json
    - sdks/python/src/echo_agent_sdk/execution_usage_values.py
    - sdks/typescript/test/wire.test.js
    - sdks/python/tests/test_wire.py
    - sdks/typescript/src/client.ts
    - sdks/python/src/echo_agent_sdk/client.py
    - sdks/java/src/main/java/com/echoagent/sdk/EchoAgentClient.java
    - docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/inputs-before.json
    - docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/inputs-after.json
    - docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/receipt.json
    - docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/manifest.json
    - docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/java-surefire/TEST-com.echoagent.sdk.TestingContractTest.xml
    evidence_refs:
    - evidence.sdk-language-client-lifecycle-current
  specific-wire-data-fidelity:
    revision: ad35887c5ceb66865a714f3f2d1da86a9c35c903
    source_refs:
    - sdks/typescript/src/wire.ts
    - sdks/python/src/echo_agent_sdk/wire.py
    - sdks/java/src/main/java/com/echoagent/sdk/WireValues.java
    - sdks/typescript/test/wire.test.js
    - sdks/python/tests/test_wire.py
    - sdks/java/src/test/java/com/echoagent/sdk/ProtocolHostTest.java
    - sdks/java/src/main/java/com/echoagent/sdk/JsonSupport.java
    - sdks/shared/fixtures/native-projection-invalid.json
    - sdks/typescript/test/protocol-integrity.test.js
    - sdks/python/tests/test_protocol_integrity.py
    - sdks/java/src/test/java/com/echoagent/sdk/ProtocolIntegrityTest.java
    - docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/java-surefire/TEST-com.echoagent.sdk.ProtocolIntegrityTest.xml
    evidence_refs:
    - evidence.sdk-language-client-lifecycle-current
  java-fixture-portable-stream-route:
    revision: 8156b64a3aee785b98e93adab65744709c0c7496
    source_refs:
    - sdks/java/src/test/java/com/echoagent/sdk/StreamCancellationRouteHostTest.java
    - sdks/shared/testing/mcp_channel_config_fixture.py
    - sdks/java/src/main/java/com/echoagent/sdk/EchoAgentClient.java
    evidence_refs:
    - evidence.sdk-official-current-language-gate
  java-fixture-pending-response-exit:
    revision: b9bc86ac0c6866283af912ca3da382c495d55240
    source_refs:
    - sdks/java/src/test/java/com/echoagent/sdk/HostEarlyExitTest.java
    - sdks/java/src/main/java/com/echoagent/sdk/EchoAgentClient.java
    evidence_refs:
    - evidence.sdk-official-current-language-gate
  java-fixture-blocking-task-fixture:
    revision: 131bb03f1e87dc8411ebc1f81e87dd927962dc54
    source_refs:
    - sdks/java/src/test/java/com/echoagent/sdk/TaskSubagentHostTest.java
    - sdks/shared/testing/task_subagent_fixture.py
    - echo-sdk-host/src/core_profile/facade/task_runtime.rs
    evidence_refs:
    - evidence.sdk-official-current-language-gate

---

# 客户端 精确来源与惯用语言入口 有限复审

## 审查范围

独立 reviewer `/root/sdk_capability_acceptance_audit` 实际读取 TSdd822、PY30ca/cd076、Java86efd及原790输入/71XML。此文件回收本次已完成审查，不制造旧运行时刻或新的native执行。源码与正式证据均绑定本次真实76601提交；原执行时间/切点保持。

## 已检查故障假设

明确accepted contract、lossless scalar/container、Python>=3.10及native消费不被catalog计数或localTestingValue假Host替代。

## 实际实现路径与证据

八lens审查与790输入前后完全一致；完整生成/跨平台/main独立待验。

## 问题记录

此具名客户端义务审查 PASS C0/I0/M0，freshness=examined不等于全SDK verified。当前TS/Java stdio Finding的candidate证据链各自保留，不由此Audit批量关闭；原Task HTTP失败保历史；受控连接fixture修复/同入口重跑0独立，不从该单例推通用Store权威。

## 残余风险

完整合同生成、12身份选择、5秒物理结算、跨平台及SDK main独立。原完整命令exit1不改成0；新同入口whole0有独立收据，后继focused不替原whole XML。

## 未检查项

上述限定scope之外的 framework Task物理结算/Store HTTP根因和所有未接受Rust identity；不新增审查分母。

## 原测试来源定位限制

以下辅助测试源码未在76601提交，故不列为该Git的source_refs；其原运行/断言名与精确hash由790执行输入和原XML保留，未删除故障义务：sdks/java/src/test/java/com/echoagent/sdk/TestingContractTest.java。原测试并未因缺blob而被宣称在该Git完整恢复。

## wire-data-fidelity具名复审范围

原scope独立reviewer本session逐源码/实际native断言核对：原13sharedinvalidcorpus三语严格拒nonstring/duplicate/missing/unknownmalformed；actualHost memory特殊__proto__/constructor+中文emoji原值exact不defaultempty。wire封闭代数与prototype安全，不把Java新advertarray静默drop当该closedwire已修。 本条是原业务审查真实性回收，不另造engine执行。对应故障项可局部闭；主revision与原challenge仍保原76601观察，新challenge以ad实际提交源码/原运行收据定位，不把旧时间重标。

## 直接decoder/官方错误consumer来源补齐

JsonSupport.java与native-projection-invalid13corpus、三ProtocolIntegrity测试/原JavaXML9是本specific wire/error义务直接消费，不仅790hash间接定位；本次adchallenge及已保存收据观察不重标旧执行time，原主76601challenge保持。


## 三个Java fixture后继challenge的范围

本次只回收ACK原portable route1case、Tool/SDKcap原pending-response fixture16case、FrameFinal原Task constrained-pool两负例与after完整class两profile各4case的有限复审；模型作者不新冒源码审者。主revision76601、既有challenge与历史正文精确保留，三个新challenge分别绑定8156/b9bc/131真实after源，不改旧运行时点。测试路径/物理frame完成前置/测试阻塞executor是三个独立support原因，不归框架生产调度/取消/持久性bug；新Linux工程验收仍待，原7open和B/12不由这些局部结果关闭。
