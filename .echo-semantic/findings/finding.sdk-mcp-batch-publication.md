---
schema_version: 1
id: finding.sdk-mcp-batch-publication
kind: finding
type: implementation_bug
status: resolved
severity: high
primary_focus: time_lifecycle
focus:
- state_authority
- failure_concurrency
- result_side_effect
boundary_ref: boundary.sdk-integrations
behavior_refs:
- behavior.sdk-integrations
- behavior.sdk-family-mcp
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-integrations-finite-observation
audit_refs:
- audit.sdk-integrations-finite-failure-concurrency
- audit.sdk-integrations-finite-state-authority
decision_refs: []
repair_evidence_refs:
- evidence.sdk-integrations-finite-observation
verification_evidence_refs:
- evidence.sdk-integrations-finite-observation
rereview_audit_refs:
- audit.sdk-integrations-finite-failure-concurrency
- audit.sdk-integrations-finite-state-authority
discovered_at: 30d7e219cc7d3aef0f75a19217c82898c363f50d
---

# MCP 批量连接后的资源发布失败

## 问题

framework Agent 批量加载明确允许返回成功子集，不能改为新 strict API。SDK source adapter 在框架连接完成后逐项注册 facade alias；quota 中途失败时，已注册前缀未返给调用方且未回收。

## 触发条件与影响

两个真实服务器连接成功但 resource quota 仅允许一个；`echo-sdk-host/src/core_profile/facade/source_operations.rs` 的 load_mcp_config/load_mcp_from_file 注册循环后项失败，前缀别名占配额，连接仍由原 Agent/Manager 拥有。

## 证据

当前为实际可达源码风险，尚需真实 Host quota1/two-server 反例、发布失败处理及 same-owner Session close/retry 验证。batch warning skip 属已有框架策略，不是本问题的豁免。

## 处理记录

保持 open；复用原 resource reservation/alias owner 和框架 Manager 清理，不复制 parser/runtime，不按名字误关闭其他 client。

## 具体原缺陷本地闭合

两真实server/quota1的config/file批发布，sharedreservation无未返前缀、原bestEffort不改、aliasrollback/PIDdeath/slotreuse；sameowner不误关peer。 原修复/准确postfixstrongcase及本session原source+closure复核三链齐，具体Bug局部resolved；旧正文待测/待main文字保作历史。当前共享Evidence包含本rootrepair与verification，Audit按具体视角/role不是泛用PASS。无需等待SDKmain才承认原Bug已解决，但平台/fullgenerator/12/5秒B/widecleanup仍独立，不外部Issueclose；未承诺网络/全物理工作保证不扩。
