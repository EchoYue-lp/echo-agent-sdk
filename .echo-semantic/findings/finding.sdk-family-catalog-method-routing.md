---
{
  "schema_version": 1,
  "id": "finding.sdk-family-catalog-method-routing",
  "kind": "finding",
  "type": "implementation_bug",
  "status": "resolved",
  "severity": "high",
  "primary_focus": "trigger_input",
  "focus": [
    "contract_evidence",
    "failure_concurrency"
  ],
  "boundary_ref": "boundary.sdk-language-clients",
  "behavior_refs": [
    "behavior.sdk-language-clients",
    "behavior.sdk-family-content-guard"
  ],
  "rule_refs": [
    "rule.framework-runtime-authority"
  ],
  "evidence_refs": [
    "evidence.sdk-family-catalog-method-routing-candidate",
    "evidence.sdk-family-catalog-method-routing-repair",
    "evidence.sdk-family-catalog-method-routing-verification"
  ],
  "audit_refs": [
    "audit.sdk-family-catalog-method-routing-rereview"
  ],
  "decision_refs": [],
  "repair_evidence_refs": [
    "evidence.sdk-family-catalog-method-routing-repair"
  ],
  "verification_evidence_refs": [
    "evidence.sdk-family-catalog-method-routing-verification"
  ],
  "rereview_audit_refs": [
    "audit.sdk-family-catalog-method-routing-rereview"
  ],
  "discovered_at": "30d7e219cc7d3aef0f75a19217c82898c363f50d"
}
---

# family 字符串拼接绕开已接受 catalog method

## 问题

旧三语言 family helper将family直接拼成wire路径，content_guard实际方法使用content-guard，导致普通公开调用无法寻址已接受能力。定义存在/catalog done不等于消费者可用。

## 触发条件与影响

普通call或family执行content_guard.redact_exact时，错误拼写在catalog签名解析阶段失败；不是Host缺少业务能力，也不能通过新加未接受wire路径绕过。

## 证据

候选Evidence记录真实旧失败、三语言各一项c52 Host正确调用/Unicode结果及wrong-family拒绝，源码改用现有catalog.resolve唯一method/signature authority。Python共享state root初失败明确区分为fixture问题。

## 处理记录

本地缺陷resolved：精确修复、三语言真实反例验证及独立limitedpass0均有对应Evidence/Audit。最新全部语言门禁、正式pin和main/CI是独立交付残余，不用它单独阻止这个精确bug闭合。accepted范围不变，不关闭外部Issue或其它Finding。
