---
schema_version: 1
id: audit.sdk-language-client-lifecycle-data-durability
kind: audit
boundary_ref: boundary.sdk-language-clients
lens: data_durability
freshness: examined
revision: 76601b242d7c354352f3b5c7d4bf099873746b14
finding_refs: []
challenges:
  named-current-client-obligation:
    revision: 76601b242d7c354352f3b5c7d4bf099873746b14
    source_refs:
    - sdks/typescript/test/ack-submission.test.js
    - sdks/python/tests/test_ack_submission.py
    - sdks/java/src/test/java/com/echoagent/sdk/AckSubmissionTest.java
    - sdks/typescript/src/client.ts
    - sdks/python/src/echo_agent_sdk/client.py
    - sdks/python/src/echo_agent_sdk/execution_usage_values.py
    - sdks/java/src/main/java/com/echoagent/sdk/EchoAgentClient.java
    - docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/inputs-before.json
    - docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/inputs-after.json
    - docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/receipt.json
    - docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/manifest.json
    - docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/java-surefire/TEST-com.echoagent.sdk.GapAckHostTest.xml
    evidence_refs:
    - evidence.sdk-language-client-lifecycle-current
---

# 客户端 消费水位与连通覆盖 有限复审

## 审查范围

独立 reviewer `/root/sdk_capability_acceptance_audit` 实际读取 TSdd822、PY30ca/cd076、Java86efd及原790输入/71XML。此文件回收本次已完成审查，不制造旧运行时刻或新的native执行。源码与正式证据均绑定本次真实76601提交；原执行时间/切点保持。

## 已检查故障假设

pending/failed ACK 不推进已消费cursor，snapshot/replay只用实际返回且连通的覆盖；客户端未承诺另一个durable Store。

## 实际实现路径与证据

original-surefire AckSubmissionTest18、GapAckHostTest8，terminal replay/managed restart具名证据。

## 问题记录

此具名客户端义务审查 PASS C0/I0/M0，freshness=examined不等于全SDK verified。当前TS/Java stdio Finding的candidate证据链各自保留，不由此Audit批量关闭；原Task HTTP失败保历史；受控连接fixture修复/同入口重跑0独立，不从该单例推通用Store权威。

## 残余风险

完整合同生成、12身份选择、5秒物理结算、跨平台及SDK main独立。原完整命令exit1不改成0；新同入口whole0有独立收据，后继focused不替原whole XML。

## 未检查项

上述限定scope之外的 framework Task物理结算/Store HTTP根因和所有未接受Rust identity；不新增审查分母。

## 原测试来源定位限制

以下辅助测试源码未在76601提交，故不列为该Git的source_refs；其原运行/断言名与精确hash由790执行输入和原XML保留，未删除故障义务：sdks/java/src/test/java/com/echoagent/sdk/GapAckHostTest.java。原测试并未因缺blob而被宣称在该Git完整恢复。
