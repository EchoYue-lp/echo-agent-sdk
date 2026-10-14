---
schema_version: 1
id: finding.sdk-telemetry-fixture-pretry-owner-leak
kind: finding
type: implementation_bug
status: resolved
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
evidence_refs:
- evidence.sdk-telemetry-fixture-exception-owner-closure
audit_refs:
- audit.sdk-telemetry-fixture-exception-owner-closure
decision_refs: []
repair_evidence_refs:
- evidence.sdk-telemetry-fixture-exception-owner-closure
verification_evidence_refs:
- evidence.sdk-telemetry-fixture-exception-owner-closure
rereview_audit_refs:
- audit.sdk-telemetry-fixture-exception-owner-closure
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


## 后继具名局部闭合候选

旧Telemetry5场景spawn服务后pretry哈希异常遗漏owner，原受控setup sparse确留Python30023与collector30024且无stop，诊断后来reap不冒after。v1两Important原源码/日志保，v2四source+3Hashhelper freeze实际after：TS6/PY3.10四/Java3methods0，15scene35原fixture/collector/proxyPID死；setup/hash/close/evidence错误保primary+cleanup errors、逐所有release、真正timeouterror/Thread.start失败均测。forcedcleanup143是预期异常路径非gracefulproductclose；proxy/prepare/verify/cleanup_host byteexact没另替EOF。FrameworkFinal全source/native/105材料独审PASS0，B物理产品承诺不由fixture修复关闭。

第四次原官方完整入口已真实exit0：TS336/PY415+18/Java355与76份freshXML/3quickstarts；800输入前后相同，原Host0f1。执行HEAD9dd只有client399961当时未提交，之后b0保存同字节，不假9dd已含after。此前三次完整失败receipt均保：前两次分别因launcher和nonAtomicGap止于TS，PY/Java未达；第三次TS336通过，Python414通过/1失败（另18子测试通过），Java未达。Gap只是独立测试oracle：GET旧水位与稍后REPLAY不是原子equal，9dd具名before红/连续真实序号+terminal独立upperbound after已审，不新生产Finding、不默认ACK缺陷。第四whole整体证据与正式95件归档均已独立复审PASS C0/I0/M0，随后原字节提交为b315f03；本次当期观察只增该真实Git正式引用，候选仍待元材料最后refs轻核及Root授权安装。
原before正文、discovered_at和所有旧raw时间保；resolved仅Admin候选，Root整体/正式新包独审PASS和最终版本绑定后才准安装，B/12残余不关。


## 正式完整入口出处的当期复核

本次当期观察Git b315f03保存新whole正式包及各原修复源码；source refs旧focused修复链与其运行时间保持，b315不是原执行日期。原第四whole执行HEAD9dd含一未提交client399961，b0继承保存同字节；800before/after/sourceGitBindings逐byte/mode匹配，不假9dd含修后生产源。
