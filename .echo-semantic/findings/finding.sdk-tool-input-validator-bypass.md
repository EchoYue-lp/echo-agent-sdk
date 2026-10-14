---
{
  "schema_version": 1,
  "id": "finding.sdk-tool-input-validator-bypass",
  "kind": "finding",
  "type": "implementation_bug",
  "status": "open",
  "severity": "high",
  "primary_focus": "trigger_input",
  "focus": [
    "result_side_effect",
    "contract_evidence"
  ],
  "boundary_ref": "boundary.sdk-integrations",
  "behavior_refs": [
    "behavior.sdk-family-chart",
    "behavior.sdk-integrations"
  ],
  "rule_refs": [
    "rule.framework-runtime-authority"
  ],
  "evidence_refs": [
    "evidence.sdk-tool-input-validation-candidate"
  ],
  "audit_refs": [],
  "decision_refs": [],
  "repair_evidence_refs": [],
  "verification_evidence_refs": [],
  "rereview_audit_refs": [],
  "discovered_at": "30d7e219cc7d3aef0f75a19217c82898c363f50d"
}
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
