---
schema_version: 1
id: audit.sdk-language-client-lifecycle-state-authority
kind: audit
boundary_ref: boundary.sdk-language-clients
lens: state_authority
freshness: examined
revision: 76601b242d7c354352f3b5c7d4bf099873746b14
finding_refs:
- finding.sdk-factory-instance-routing
challenges:
  named-current-client-obligation:
    revision: 76601b242d7c354352f3b5c7d4bf099873746b14
    source_refs:
    - sdks/typescript/src/client.ts
    - sdks/python/src/echo_agent_sdk/client.py
    - sdks/java/src/main/java/com/echoagent/sdk/EchoAgentClient.java
    - sdks/python/src/echo_agent_sdk/execution_usage_values.py
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
  named-settlement-language-clients:
    revision: 04eb6ad50982656b5a8b8ea649619b6a383065ff
    source_refs:
    - sdks/typescript/src/client.ts
    - sdks/typescript/src/wire.ts
    - sdks/python/src/echo_agent_sdk/client.py
    - sdks/python/src/echo_agent_sdk/wire.py
    - sdks/java/src/main/java/com/echoagent/sdk/EchoAgentClient.java
    - sdks/java/src/main/java/com/echoagent/sdk/WireValues.java
    - echo-sdk-protocol/src/scalar.rs
    - echo-sdk-protocol/src/error.rs
    - contracts/sdk/accepted-external-contract.json
    - scripts/check-language-sdks.sh
    - sdks/java/src/test/java/com/echoagent/sdk/FactoryCleanupLifecycleTest.java
    evidence_refs:
    - evidence.sdk-language-client-lifecycle-current
    - evidence.sdk-typescript-stdio-close-specific-closure
    - evidence.sdk-java-stdio-close-specific-closure
    - evidence.sdk-event-identity-nested-parent-closure
---

# 客户端 唯一业务与资源权威 有限复审

## 审查范围

独立 reviewer `/root/sdk_capability_acceptance_audit` 实际读取 TSdd822、PY30ca/cd076、Java86efd及原790输入/71XML。此文件回收本次已完成审查，不制造旧运行时刻或新的native执行。源码与正式证据均绑定本次真实76601提交；原执行时间/切点保持。

## 已检查故障假设

客户端没有复制 Run/Task/Store/terminal 权威；只持 cursor/发送 receipt/取消地址/原 transport 与 cleanup owner。

## 实际实现路径与证据

ACK/Factory阶段完成/取消投影和 retained original owner 源已实际独审。

## 问题记录

此具名客户端义务审查 PASS C0/I0/M0，freshness=examined不等于全SDK verified。当前TS/Java stdio Finding的candidate证据链各自保留，不由此Audit批量关闭；原Task HTTP失败保历史；受控连接fixture修复/同入口重跑0独立，不从该单例推通用Store权威。

## 残余风险

完整合同生成、12身份选择、5秒物理结算、跨平台及SDK main独立。原完整命令exit1不改成0；新同入口whole0有独立收据，后继focused不替原whole XML。

## 未检查项

上述限定scope之外的 framework Task物理结算/Store HTTP根因和所有未接受Rust identity；不新增审查分母。

## factory-instance-routing具名复审范围

原scope独立reviewer本session逐源码/实际native断言核对：三native5 distinct child/same descriptor、parallel closeA/B live/parent再构造、PY invalid factoryorigin stalegeneration零handler effect。Hostfactory_origin exactrouting与真实callback匹配，不用managedstore/LlmGraph代。 本条是原业务审查真实性回收，不另造engine执行。对应故障项可局部闭；主revision与原challenge仍保原76601观察，新challenge以ad实际提交源码/原运行收据定位，不把旧时间重标。


## 2026-10-20 具名后继的有界结算

主header revision/freshness及原挑战仍保原审查cut；这里只记录下列named单元，原17be/0bbc读审与04eb受控media增量分开，旧Evidence运行不重标。任何一个row不能推整八风险组或整composite资格。

- `behavior.sdk-language-clients`：仅named子链/实际after，整体仍hold；证据`evidence.sdk-language-client-lifecycle-current`、`evidence.sdk-typescript-stdio-close-specific-closure`、`evidence.sdk-java-stdio-close-specific-closure`、`evidence.sdk-event-identity-nested-parent-closure`。
