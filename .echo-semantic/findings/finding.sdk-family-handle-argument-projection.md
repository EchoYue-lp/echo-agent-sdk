---
{
  "schema_version": 1,
  "id": "finding.sdk-family-handle-argument-projection",
  "kind": "finding",
  "type": "implementation_bug",
  "status": "open",
  "severity": "high",
  "primary_focus": "contract_evidence",
  "focus": [
    "state_authority",
    "trigger_input"
  ],
  "boundary_ref": "boundary.sdk-integrations",
  "behavior_refs": [
    "behavior.sdk-integrations",
    "behavior.sdk-family-lsp"
  ],
  "rule_refs": [
    "rule.framework-runtime-authority"
  ],
  "evidence_refs": [
    "evidence.sdk-candidate-behavior-verification"
  ],
  "audit_refs": [],
  "decision_refs": [],
  "repair_evidence_refs": [],
  "verification_evidence_refs": [],
  "rereview_audit_refs": [],
  "discovered_at": "30d7e219cc7d3aef0f75a19217c82898c363f50d"
}
---

# 已返回 typed handle 不能作为 family 参数正常回传

## 问题

五个 family 旧 json_of 直接调用 WireValue.into_json，明确拒绝 Handle，导致正常 typed API 的 LSP open_exact 返回 handle 后 load_config 无法使用。

## 触发条件与影响

三语言真实 Host 正常 open/load 配置在 argument0 报 InvalidValue；旧资源 map workaround 不作为正常 API 证据。

## 证据

三语言旧 Host red 与 UTF8/generationMAX adapter unit red；common family_argument_json 仅将 WireHandle DTO 映射到已有资源解析及统一 Registry ladder。当前 Host136/core31/bridge34/facade12 和三语言29场景绿，引用候选 Evidence 的具体范围，不证明最终 published pin。

## 处理记录

保持 open，最终修复/验证/rereview 与精确 snapshot 归档后闭合；其它 typed value 继续按原显式转换规则，不引入 process-local 资源默认 JSON 投影。
