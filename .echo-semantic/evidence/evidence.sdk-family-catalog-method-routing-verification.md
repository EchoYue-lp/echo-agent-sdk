---
{
  "schema_version": 1,
  "id": "evidence.sdk-family-catalog-method-routing-verification",
  "kind": "evidence",
  "observed_at": "source:pending",
  "source_refs": [
    "sdks/typescript/src/client.ts",
    "sdks/python/src/echo_agent_sdk/client.py",
    "sdks/java/src/main/java/com/echoagent/sdk/EchoAgentClient.java",
    "sdks/typescript/test/family-route-host.test.js",
    "sdks/python/tests/test_family_route_host.py",
    "sdks/java/src/test/java/com/echoagent/sdk/FamilyRouteHostTest.java",
    "docs/sdk/governance/evidence/sdk-family-route-20261013/manifest.json",
    "docs/sdk/governance/evidence/sdk-family-route-20261013/independent-review-receipt.json"
  ],
  "supports": [
    "behavior.sdk-family-content-guard",
    "behavior.sdk-language-clients"
  ],
  "limitations": [
    "Local exact bug closure; final package/fullsource/pubpin/mainCI delivery separate.",
    "Original tested sourcehash remains in manifest; current observation does not assert later-source equivalence."
  ]
}
---

# Family catalog route实际验证

## 支持的结论

原content_guard普通调用因下划线wire路径失败；修复后三语言各一项实际c52 Host场景通过，call/family Unicode redact_exact与wrong-family本地拒绝均覆盖。归档rawlogs/XML及源码hash精确限定，不是全family矩阵。

## 来源与范围

归档manifest及实际工具/独立review会话记录限定到TS8a33/Python3aba/Javae559与c52Host。本对象分别记录修复和反例验证，未杜撰新的测试运行；原始候选Evidence继续保留。

## 已知缺口

全部family的未列组合、最终公开frameworkpin/三语言完整门禁及精确main/CI交付另待，不阻止这个精确本地路由bug已修复结论。
