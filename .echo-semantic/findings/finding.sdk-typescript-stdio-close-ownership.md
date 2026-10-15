---
schema_version: 1
id: finding.sdk-typescript-stdio-close-ownership
kind: finding
type: implementation_bug
status: resolved
severity: high
primary_focus: time_lifecycle
focus:
- failure_concurrency
- state_authority
- contract_evidence
boundary_ref: boundary.sdk-language-clients
behavior_refs:
- behavior.sdk-language-clients
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-typescript-stdio-close-specific-closure
audit_refs:
- audit.sdk-typescript-stdio-close-specific-rereview
decision_refs: []
repair_evidence_refs:
- evidence.sdk-typescript-stdio-close-specific-closure
verification_evidence_refs:
- evidence.sdk-typescript-stdio-close-specific-closure
rereview_audit_refs:
- audit.sdk-typescript-stdio-close-specific-rereview
discovered_at: 30d7e219cc7d3aef0f75a19217c82898c363f50d
---

# TypeScript 原 stdio 关闭准入、EOF 与实际进程结算

## 问题

原 public close 没有充分持有同一 stdin EOF/原子进程退出事实。

## 触发条件与影响

候选修复继续发现同步 abort 回调重入可能启动两次 close，以及 child 已退出但 stdin 回调的意外 EIO 晚到时错误提前成功；expected EPIPE/DESTROYED、已发送 signal 都不能充当实际 exit。该范围是原生 transport 资源生命周期，不新增框架终态或 Task 五秒政策。

## 证据

[精确切片清单](https://github.com/EchoYue-lp/echo-agent-sdk/blob/0ebe15f66d0aa57c7b7a92ae68cd1be106114ae9/docs/sdk/governance/evidence/typescript-stdio-close/deferred-eof-final/manifest.json) 与[说明](https://github.com/EchoYue-lp/echo-agent-sdk/blob/0ebe15f66d0aa57c7b7a92ae68cd1be106114ae9/docs/sdk/governance/evidence/typescript-stdio-close/deferred-eof-final/README.md)保留实际旧编译客户端的两项 Important 反例（2 fail）及同两例修复后2 pass；没有手抄旧实现或回退生产 dist。当前候选 client.ts SHA dd822f73e5cfeb84a5d082923b255d18357b3480a03602ec6a79fcb45e9aeb3f，实际 dist SHA 5a9cee6ae1d7bfdbcf16761e3f4b9ab67014e66a5f1e461f873fc194f7fb1b7c，10个真实 child 场景全部通过，零跳过，并已独立复审 PASS。deferred promise 先发布，再同步关闭原 Connection；EOF callback/已完成事实和原 child exit 共同结算，原五秒 native grace/fallback 不重置，晚到意外错误与同 owner 重试保留。

[当前实体 public-close 收据](https://github.com/EchoYue-lp/echo-agent-sdk/blob/0ebe15f66d0aa57c7b7a92ae68cd1be106114ae9/docs/sdk/governance/evidence/telemetry-host/publicclose-current-source/receipt.json)保存实际 fixture 源与客户端/dist：Host 4967dd43、fixture d4516646、测试 e152c4b1；held export 时 close 未完成、原 Host 活着，release 后原父进程观察同一 Host exit0。此18项清单包与15项 stdio 清单均已独立复审通过。旧 publicclose-approved 的12项包缺少当期 fixture 实体，其历史通过保留，不能替代当前源精确证明。

## 处理记录

这是已具有限修复、验证与独审的本地 candidate；本 Finding 当前保留 open，待主任务将明确源码和正式材料绑定真实可恢复切点并完成当前 SDK 组合验收。没有把30d发现基准或4feb历史观察冒充上述执行版本；正式 Evidence/Audit 引用待最终冻结，不造 source:pending。该局部通过不等 SDK main、最终框架 pin 或完整三语言关闭义务全部闭合。

## 当前有限本地闭合（不等SDK main）

本具体stdio缺陷已具修复、精确同源验证与独立具体source rereview，正式code/packet可从Git76601恢复，故局部resolved。原before/中间candidate及open时期原文保为历史，不伪称当时已闭合。单一综合Evidence同时记录repair和verification，未造近重复对象。官方a634后继完整语言0的790inputs与最终client source一致，Java原TelemetryXML两个真实case包含公开slow-flush close，TS对应exact fixture/真实child也有独立来源；kill/EOF与实际process exit的区别不删除。完整生成、12项身份、5秒Task物理政策、平台/SDK main仍独立，不能只因未主交付继续假报这个局部Bug尚未修复。
