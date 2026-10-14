---
schema_version: 1
id: finding.sdk-telemetry-fixture-pretry-owner-leak
kind: finding
type: implementation_bug
status: open
severity: high
primary_focus: time_lifecycle
focus:
- failure_concurrency
- result_side_effect
boundary_ref: boundary.sdk-language-clients
behavior_refs:
- behavior.sdk-language-clients
rule_refs:
- rule.framework-runtime-authority
evidence_refs: []
audit_refs: []
decision_refs: []
repair_evidence_refs: []
verification_evidence_refs: []
rereview_audit_refs: []
discovered_at: 9a59943e70da06b1af231b1fb22bddc879885f28
---

# Telemetry夹具在保护区之前异常遗留原子进程

## 问题

Telemetry两个console场景和三个slow-flush模式共5组，JS先spawn Python serve，serve再启动真实Rustcollector，然后在try保护区之前以readFileSync(host)构造facts。大Host哈希抛错时finally尚未进入，原service/collector没有进入既有关闭流程。该ownership缺口与终态取消日志的TAP最后251/排序缓冲现象一致；不能把它归因为SDK/OTLPshutdown产品挂起、协作线程问题或5秒B策略。日志8条已输出not ok不包含这5条独立终态结果，故不虚增为13个已输出失败。

## 触发条件与影响

本地macOS官方793输入完整语言329/Python394+18/Java351与75XML实际0保留原平台/sourcecut，不能推Linux原生完整通过。本次Linux旧eebd语言job在新9a配置提交后权威终态Cancelled，取消不是成功或完整TAP终态。新运行38643983612仍live，不能预写其结果。

## 证据

原日志仅保存于Admin `/Users/ls/MyWork/code/ylp_agent_learn/lp-agent/.worktrees/admin/sdk-pr5-eebd-ci-evidence/language-cancelled.log`，166086字节，SHA256 `d4a85d86d1af20f0025d157e66dceed3b4893059c2ae67da2370f254f4dad2ca`；未制造futureGit或source:pending Evidence。发现观察Git `9a59943e70da06b1af231b1fb22bddc879885f28`只是当前冻结源码定位，原运行属于eebdCI，不冒9a执行。实际相关源码Git字节如下：

- `sdks/typescript/test/telemetry-host.test.js`，SHA256 `e152c4b1950db19319977ba02bb58e0c7ecbff2f32a3b98517cc28b14a0f62c8`。
- `sdks/shared/testing/telemetry_host_fixture.py`，SHA256 `d4516646cb4ad760f99632ceb769f9182eb374a40f14267316f763fa95bf8bb9`。

## 处理记录

ACK唯一拥有Telemetry JS及Python serve cleanup修复；需要pretry/hash/setup failure仍结清原Python与collector、保正常export/flush/cancel/EOF/publicclose断言、真正原PID退出。当前只有真实CI异常与原源码机制证据，不称修复已完成或执行后绿，保持open。

这些是SDK测试/证据夹具缺陷，复用既有language lifecycle与测试支撑边界，不新增runtime/API/源接受策略/权限或第42能力族。原B三个宽Finding与12项人选择仍独立；本Finding不得借新run尚进行、旧macOS绿或编译budget配置而自动resolved。正式repair/verification/rereview及raw归档真Git冻结后再补三链，当前引用数组为空明确未闭合。


## 后继CI状态观察（原候选之后）

原上文“38643983612仍live”是候选准备当时的状态。Root后续已确认该运行终态failure；本Finding尚未读取其新原始失败日志，不据结果名推断它与旧eebd反例同cause，也不声称修复已验证。原执行版本、发现观察和旧日志Hash不改变。
