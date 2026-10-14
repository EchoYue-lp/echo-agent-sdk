---
schema_version: 1
id: finding.sdk-run-steer-canonical-turn-context
kind: finding
type: implementation_bug
status: resolved
severity: high
primary_focus: state_authority
focus:
- state_authority
- time_lifecycle
- contract_evidence
boundary_ref: boundary.sdk-agent-session-run
behavior_refs:
- behavior.sdk-family-run
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-integrations-finite-observation
audit_refs:
- audit.sdk-integrations-finite-contract-evidence
- audit.sdk-integrations-finite-state-authority
decision_refs: []
repair_evidence_refs:
- evidence.sdk-integrations-finite-observation
verification_evidence_refs:
- evidence.sdk-integrations-finite-observation
rereview_audit_refs:
- audit.sdk-integrations-finite-contract-evidence
- audit.sdk-integrations-finite-state-authority
discovered_at: 30d7e219cc7d3aef0f75a19217c82898c363f50d
---

# Run.steer 未取得运行中的 canonical turn 身份

## 问题

正常公开 Run.steer 在同一个实际 running Run 的模型请求屏障处被拒或返回 accepted=false。

## 触发条件与影响

Host 按 canonical sess:turn:1 校验，实际 ReactAgent stream 却因未传 invocation context 使用随机 UUID；控制请求不能进入该运行。Host 的身份校验不应为规避此错误而放宽。

## 证据

[初始反例清单](../../docs/sdk/governance/evidence/run-steer-native/initial-red-manifest.json) 保存调用链、原消费源 SHA 和诊断边界；其当时 Java 尚未执行的记录保留。后继 [Java 原 Host 收据](../../docs/sdk/governance/evidence/run-steer-native/java-original/receipt.json) 为真实一项失败，[同源诊断 Host 收据](../../docs/sdk/governance/evidence/run-steer-native/java-diagnostic-fixed/receipt.json) 为真实一项通过；TS/Python 原 Host 与诊断 Host 原日志也在同目录。

原 Host SHA 为 bcb0389ae8d8a946fab9bb123945100722711029bafa8d9d6cadf95a76c9c345，诊断 Host 为 eb00038a0be7977f936514c7df8e06e329928f6735d466f292f10201a38abbf5。原 framework be5 的 src/acp/runtime.rs SHA d8446e7728f6883e903aa2b243b19b9cb722da79c729e1c27f73f35d5a0ffefe、echo-orchestration/src/runtime/turn_driver.rs SHA 8625c2bf7236e3cc00ef32d4b24a9c53ba03357f425eb081d66d481fd36364f9，属于框架 pin 下的源，不冒充 SDK Git 文件。

## 处理记录

通用 turn context 由框架传递、SDK 保留薄调用与唯一身份校验。框架候选修复及三语言诊断红绿已取得；legacy 兼容补丁仍待完整前后验证，尚未正式交付，故本项保持 open。当前正文链接为直接可复核来源，正式 Evidence/Audit 版本待受控源码冻结后绑定；空 evidence_refs 不代表没有原始反例。没有创建 source:pending 或假历史 Git，也不把诊断 Host 当成当前正式 pin 的通过。

## 具体原缺陷本地闭合

根修复frameworka634 echo-core/src/agent/mod.rs + src/acp/runtime.rs reservedcanonicalturn/fallback；Hostconsumer严格identity不放宽，3nativeHTTPsecondrequestUnicode/accepted/lateempty/2wait真实证明；oldbefore/diagcompat限制保。 原修复/准确postfixstrongcase及本session原source+closure复核三链齐，具体Bug局部resolved；旧正文待测/待main文字保作历史。当前共享Evidence包含本rootrepair与verification，Audit按具体视角/role不是泛用PASS。无需等待SDKmain才承认原Bug已解决，但平台/fullgenerator/12/5秒B/widecleanup仍独立，不外部Issueclose；未承诺网络/全物理工作保证不扩。
