---
schema_version: 1
id: audit.sdk-event-identity-nested-parent-closure
kind: audit
boundary_ref: boundary.sdk-language-clients
lens: contract_evidence
freshness: examined
revision: 185559cec4d0d82365321d131fd1d2a9575c5eb4
finding_refs:
- finding.sdk-event-identity-nested-parent-projection
challenges:
  nested-parent-leaf-fidelity:
    revision: 185559cec4d0d82365321d131fd1d2a9575c5eb4
    source_refs:
    - sdks/python/src/echo_agent_sdk/event_identity_values.py
    - sdks/python/tests/test_event_identity_values.py
    - sdks/java/src/main/java/com/echoagent/sdk/EventIdentity.java
    - sdks/java/src/test/java/com/echoagent/sdk/EventIdentityTest.java
    - sdks/typescript/src/event_identity_values.ts
    - sdks/typescript/test/event-identity-values.test.js
    evidence_refs:
    - evidence.sdk-event-identity-nested-parent-closure
---

# 原独立投影复审的有限回收

## 审查范围

回收独立SDKcap实际4source/test源码与具名probe审查，不新冒本模型作者业务review。

## 已检查故障假设

容器过早stringify、仅flat读取、合法flat优先或空值校验被改、otheridentity/输入对象被mutate、JDK17编译冒运行。

## 实际实现路径与证据

Python与Java原纯值构造入口丢失 accepted runtime.subagent_lineage.parent_event_id：Python先将lineage容器转字符串再查字段，Java只查flat字段。原0ebe同一Python/Java probe各exit1；TS原nested peer exit0。185559仅修原投影：保容器到叶值转换、原flat优先与语言各自空值校验、不变UUID/其余identity字段/immutable更新；不新增API或Host状态权威。实际after同probe各exit0；Python3.10原模块3项与JUnit类3项均0失败/0skip，Java实际Oracle21运行且--release17编译通过，不冒已执行JDK17。SDKcap原源码、具名pure-value测试及材料审PASS C0/I0/M0；本元作者只回收该有限结论。

## 问题记录

before反例与after同probe保；Admin `sdk-event-identity-lineage/receipt.json`（1563B，SHA256 `b526c7f447a41d809fe6ed03b41a6fb511bcdd5e3ae6c1f3f7f812a6df834937`）；Admin `sdk-event-identity-lineage/independent-review.json`（1089B，SHA256 `3bb5bdcda419d234ef20c0d2a7eeeef3823d3c82ae62eda4cd97312213a3cb77`）。

## 残余风险

仅这一个accepted pure-value parent投影局部闭合；不是全部lineage/跨进程事件/Host集成或Linux/JDK17完整语言验证。Map/Behavior仍保持原needs_review。B3、12accepted-source、合同13、PID/Linux和Host退出未知原因独立。历史raw不新增S包，下面Admin定位可保原byte恢复，不伪source_refs已在Git。

## 未检查项

Host/native/全语言whole、所有第三方runtime对象形状未由此审覆盖。
