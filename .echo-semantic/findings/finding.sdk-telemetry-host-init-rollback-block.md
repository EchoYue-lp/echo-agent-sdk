---
schema_version: 1
id: finding.sdk-telemetry-host-init-rollback-block
kind: finding
type: implementation_bug
status: resolved
severity: high
primary_focus: failure_concurrency
focus:
- failure_concurrency
- time_lifecycle
- state_authority
boundary_ref: boundary.sdk-integrations
behavior_refs:
- behavior.sdk-family-telemetry
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-integrations-finite-observation
audit_refs:
- audit.sdk-integrations-finite-time-lifecycle
- audit.sdk-integrations-finite-failure-concurrency
decision_refs: []
repair_evidence_refs:
- evidence.sdk-integrations-finite-observation
verification_evidence_refs:
- evidence.sdk-integrations-finite-observation
rereview_audit_refs:
- audit.sdk-integrations-finite-time-lifecycle
- audit.sdk-integrations-finite-failure-concurrency
discovered_at: 30d7e219cc7d3aef0f75a19217c82898c363f50d
---

# Telemetry 全局 subscriber 冲突后的回滚阻塞 Host

## 问题

Host 原有全局 stderr subscriber 与 framework init 再安装全局 subscriber 冲突；初始化失败路径释放 metrics provider 时同步 shutdown 等待，阻塞 ACP dispatcher。

## 触发条件与影响

三个原生客户端正常公开 telemetry.init 在两秒观测窗口内未返回，正向成功测试各一项失败；不能伪称返回了明确 FrameworkError，也不能以永久 expected-error 替代已接受初始化能力。

## 证据

[正式原始收据](https://github.com/EchoYue-lp/echo-agent-sdk/blob/0ebe15f66d0aa57c7b7a92ae68cd1be106114ae9/docs/sdk/governance/evidence/telemetry-host/receipt.json)、[证据说明](https://github.com/EchoYue-lp/echo-agent-sdk/blob/0ebe15f66d0aa57c7b7a92ae68cd1be106114ae9/docs/sdk/governance/evidence/telemetry-host/README.md) 和 [OS 线程栈](https://github.com/EchoYue-lp/echo-agent-sdk/blob/0ebe15f66d0aa57c7b7a92ae68cd1be106114ae9/docs/sdk/governance/evidence/telemetry-host/host-init.sample.txt) 保存三语言各 0 pass/1 fail/0 skip 与实际 shutdown 栈。原 Host 为 bcb0389ae8d8a946fab9bb123945100722711029bafa8d9d6cadf95a76c9c345；冻结 Host main 源 SHA 5ece00bd831bb954e644af13739287cbc538bbf2914b760a0f6771df17cd404d，framework be5 telemetry 源 SHA b6734aae2fcfa9cb1fdd6bafe77551213b7ed206c42cdaf5d5aefaa5dbc799aa，均可在本证据目录读取原文件。OS sample 证明回滚 shutdown 阻塞，不自行声称观测到了尚未返回的原始 try_init 错误负载。

## 处理记录

[ADR 0036](../../docs/adr/0036-telemetry-host-composition.md) 的技术修复设计已独立审查通过，framework/Host 正在实现：保留框架唯一 runtime/exporter/metrics 权威，Host 组合既有 subscriber，并由原结算链持有 blocking 工作。尚无交付或最终修复、验证、复审三链，本项保持 open。collector/export/flush 及取消/EOF/关闭后的真实结算按设计待验证，不能借原其他语言门禁通过。正式 Evidence/Audit 待最终冻结绑定，当前仅直接链接真实源与原始收据，不造 source:pending。

## 具体原缺陷本地闭合

根修复frameworka634 src/telemetry.rs稳定OTEL/filtermarker/原runtimeoneowner；Hostmain/facade/state blockingcollector先settlement、RPCcancel/EOF不释放原job；原3hangOSsample→realcollectorflush/publiccloseheldreleasePIDs完整source审，noarbitrarynetwork。 原修复/准确postfixstrongcase及本session原source+closure复核三链齐，具体Bug局部resolved；旧正文待测/待main文字保作历史。当前共享Evidence包含本rootrepair与verification，Audit按具体视角/role不是泛用PASS。无需等待SDKmain才承认原Bug已解决，但平台/fullgenerator/12/5秒B/widecleanup仍独立，不外部Issueclose；未承诺网络/全物理工作保证不扩。
