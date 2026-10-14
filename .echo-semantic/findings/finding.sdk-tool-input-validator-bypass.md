---
schema_version: 1
id: finding.sdk-tool-input-validator-bypass
kind: finding
type: implementation_bug
status: resolved
severity: high
primary_focus: trigger_input
focus:
- result_side_effect
- contract_evidence
boundary_ref: boundary.sdk-integrations
behavior_refs:
- behavior.sdk-family-chart
- behavior.sdk-integrations
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-tool-input-validation-candidate
- evidence.sdk-tools-bridge-finite-observation
audit_refs:
- audit.sdk-tools-bridge-finite-trigger-input
decision_refs: []
repair_evidence_refs:
- evidence.sdk-tools-bridge-finite-observation
- evidence.sdk-tool-input-validation-candidate
verification_evidence_refs:
- evidence.sdk-tools-bridge-finite-observation
- evidence.sdk-tool-input-validation-candidate
rereview_audit_refs:
- audit.sdk-tools-bridge-finite-trigger-input
discovered_at: 30d7e219cc7d3aef0f75a19217c82898c363f50d
---

# Direct Tool adapter 绕过规范输入校验

## 问题

direct facade Tool 调用只绑定位置参数，不调用framework唯一schema/custom输入validator；非法enum可由leaf Tool默认处理并返回伪成功。额外null参数也可能被旧绑定逻辑静默忽略。

## 触发条件与影响

真实chart调用not_a_chart_type得到line success，违反已接受输入与失败合同。不能用catalog reachability或原正向工具证据证明非法输入明确失败。

## 证据

候选Evidence保留实际prototype旧成功与修复后canonicalInvalidParameter；新418 Rust输入来源精确归档，初编译路径错误不当行为红证据。

## 处理记录

状态open。薄adapter改调现framework validate_tool_input且绑定拒超额arity，不新增第二validator或接受publichelper三语言API。正式非法参数回归、完整门禁、独立复审与最终main仍待。

## 具体本地修复、验证与复审已完成

原 adapter 跳过唯一 framework schema/custom validator 的具体 bug 已改为同一 validate_tool_input，参数绑定拒 excess arity（含 null），没有第二 validator。真实 prototype not_a_chart_type→假 line 的原成功和修后 typed InvalidParameter 保在旧专属 Evidence；正式 spy 强断言 missing/type/enum/excessNull/customBad 全 executes0、valid 精确 executes1，最新原05真实通过。后继 Chart 整数义务独立已有61原生/独审闭合，不以旧非法 enum 修复替整数证明，不把 helper 自动接受为三语言 API。
真实 current_tool_boundary_review 原 source review 与本次具体 closure 复核支持三链局部 resolved。原正文等待语句是原阶段历史，保原字节；不以 SDK main 未交付长期假 open。B/12/宽资源/平台/main 仍各自独立。
