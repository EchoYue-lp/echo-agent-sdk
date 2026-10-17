---
schema_version: 1
id: finding.sdk-event-identity-nested-parent-projection
kind: finding
type: implementation_bug
status: resolved
severity: medium
primary_focus: contract_evidence
focus:
- result_side_effect
boundary_ref: boundary.sdk-language-clients
behavior_refs:
- behavior.sdk-language-clients
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-event-identity-nested-parent-closure
audit_refs:
- audit.sdk-event-identity-nested-parent-closure
decision_refs: []
repair_evidence_refs:
- evidence.sdk-event-identity-nested-parent-closure
verification_evidence_refs:
- evidence.sdk-event-identity-nested-parent-closure
rereview_audit_refs:
- audit.sdk-event-identity-nested-parent-closure
discovered_at: 0ebe15f66d0aa57c7b7a92ae68cd1be106114ae9
---

# EventIdentity嵌套parent投影局部修复

## 问题

Python与Java原纯值构造入口丢失 accepted runtime.subagent_lineage.parent_event_id：Python先将lineage容器转字符串再查字段，Java只查flat字段。原0ebe同一Python/Java probe各exit1；TS原nested peer exit0。185559仅修原投影：保容器到叶值转换、原flat优先与语言各自空值校验、不变UUID/其余identity字段/immutable更新；不新增API或Host状态权威。实际after同probe各exit0；Python3.10原模块3项与JUnit类3项均0失败/0skip，Java实际Oracle21运行且--release17编译通过，不冒已执行JDK17。SDKcap原源码、具名pure-value测试及材料审PASS C0/I0/M0；本元作者只回收该有限结论。

## 触发条件与影响

标准runtime带nested lineage时丢parent关联，但不据此声称事件序号/ACK消费或资源ownership失效。

## 证据

原观察0ebe，修复真实Git `185559cec4d0d82365321d131fd1d2a9575c5eb4`，原四源码/test SHA与修后sourceRefs由专属Evidence保存。Admin `sdk-event-identity-lineage/receipt.json`（1563B，SHA256 `b526c7f447a41d809fe6ed03b41a6fb511bcdd5e3ae6c1f3f7f812a6df834937`）；Admin `sdk-event-identity-lineage/independent-review.json`（1089B，SHA256 `3bb5bdcda419d234ef20c0d2a7eeeef3823d3c82ae62eda4cd97312213a3cb77`）。

## 处理记录

三链已齐，本文件是尚未安装的Admin候选，须独审及Root授权后落盘。仅这一个accepted pure-value parent投影局部闭合；不是全部lineage/跨进程事件/Host集成或Linux/JDK17完整语言验证。Map/Behavior仍保持原needs_review。B3、12accepted-source、合同13、PID/Linux和Host退出未知原因独立。历史raw不新增S包，下面Admin定位可保原byte恢复，不伪source_refs已在Git。
