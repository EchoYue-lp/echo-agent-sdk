---
schema_version: 1
id: finding.sdk-java-stdio-process-first-close
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
- evidence.sdk-java-stdio-close-specific-closure
audit_refs:
- audit.sdk-java-stdio-close-specific-rereview
decision_refs: []
repair_evidence_refs:
- evidence.sdk-java-stdio-close-specific-closure
verification_evidence_refs:
- evidence.sdk-java-stdio-close-specific-closure
rereview_audit_refs:
- audit.sdk-java-stdio-close-specific-rereview
discovered_at: 30d7e219cc7d3aef0f75a19217c82898c363f50d
---

# Java 官方 stdio transport 先 destroy 的公开关闭缺口

## 问题

已锁 com.agentclientprotocol:acp-core:0.17.0 的 StdioAcpClientTransport.closeGracefully 先 destroy 直接子进程，再等待；不能由它返回或 wrapper 退出来推断实际 Host/exporter 清理完成。

## 触发条件与影响

原公开 close 的真实反例在 export ACK held 时已经完成，但原 Host 仍 alive。此问题不同于此前已局部 resolved 的意外 early-exit/late subscriber 投影，不回滚该旧 Finding。

## 证据

[正式反例收据](../../docs/sdk/governance/evidence/telemetry-host/java-publicclose-counterexample/receipt.json)为真实1 test/1 fail/0 skip：public_close_done=true、real_host_exited=false，后续受控清理不是成功结算。原执行客户端 SHA b65dea36e455ea74a9c3926b64e456dea037771c9d1125ad158e34cbf73db14e；[官方依赖原件](../../docs/sdk/governance/evidence/telemetry-host/java-publicclose-counterexample/official-dependency.json)固定 jar SHA daa9487c594817032f218aaa2b2707731a8872caedfc11faec0137d1441e2f47，read-only javap 不是生产反射。

[已批准技术设计](../../docs/sdk/governance/evidence/sdk-java-host-graceful-close/design.md) SHA d6d68763219fdeea148d3c14df352d86f25cedf085aa73b8821c2ae3f4d7c962，独立设计审查零 findings。复用原 process owner 和官方 JSON-RPC I/O：已接受 writer tail 先 drain，同一 stdin EOF，再等原 process exit，原 native 五秒窗口到期才强停；强停请求、关闭 waiter 取消或 delegate EOF 不是假 exit/flush。没有新 wire、框架调度权威或 Task 物理结算政策。

[初步修复后原场景](../../docs/sdk/governance/evidence/telemetry-host/java-publicclose-after-initial/receipt.json)在中间源 b89f4edb54e59db35d64e7101733d837038602652b4672e45922cdf6917c5ca9 取得1 pass/0 skip：held 时 public close 未完成且 Host alive，release 后同一原 Host exit0。这个18项清单是候选中间切点，不是最终 writer/force/exit 边界、独立实现复审或当前全 SDK 通过。

## 处理记录

### 原 b89 初步候选观察 (historical)

实现与最终回归仍由指定 owner 进行，Finding 保持 open/candidate。等待已接受帧真实写出、失败/并发/强停重试、意外退出、stderr drain 与原 public-close 的最后同源验证和独审。原30d仅是可恢复发现基准，不冒充 b89 或最新执行修订；正式 Evidence/Audit 待最终真实快照绑定，当前直接引用原件，不发明版本。其余12项 source/kind 与5秒 B 提案未由本修复裁决。

## 最终候选源码与定向验证（仍未 SDK main 交付）

后继最终源 `86efd3959d478723949910ff97058f26a6fd32b3e050b54713f83288a2ca6131` 保留资源关闭异常/原失败重试，并等待已启动 stderr drainer 结算。正式 [收据](../../docs/sdk/governance/evidence/sdk-java-host-graceful-close/receipt.json) 与 [独立复审](../../docs/sdk/governance/evidence/sdk-java-host-graceful-close/independent-review.json)记录19项生命周期、72项相关回归均零失败/错误/跳过，最终源码及说明文稿独审 PASS C0/I0/M0。归档47项payload的原manifest/checksums保持；同一公开 telemetry 场景的3aa成功不能证明后继cleanup增量已在新pin实跑。SDK现已pin已发布a634，但当前完整语言/最终主交付尚待，本Finding保持open/candidate，不把原b89处理中句子当作最新源码状态。

## 当前有限本地闭合（不等SDK main）

本具体stdio缺陷已具修复、精确同源验证与独立具体source rereview，正式code/packet可从Git76601恢复，故局部resolved。原before/中间candidate及open时期原文保为历史，不伪称当时已闭合。单一综合Evidence同时记录repair和verification，未造近重复对象。官方a634后继完整语言0的790inputs与最终client source一致，Java原TelemetryXML两个真实case包含公开slow-flush close，TS对应exact fixture/真实child也有独立来源；kill/EOF与实际process exit的区别不删除。完整生成、12项身份、5秒Task物理政策、平台/SDK main仍独立，不能只因未主交付继续假报这个局部Bug尚未修复。
