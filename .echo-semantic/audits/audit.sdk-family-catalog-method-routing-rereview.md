---
{
  "schema_version": 1,
  "id": "audit.sdk-family-catalog-method-routing-rereview",
  "kind": "audit",
  "boundary_ref": "boundary.sdk-language-clients",
  "lens": "contract_evidence",
  "freshness": "examined",
  "revision": "source:pending",
  "finding_refs": [
    "finding.sdk-family-catalog-method-routing"
  ],
  "challenges": {
    "catalog-exact-method-and-family-membership": {
      "revision": "source:pending",
      "source_refs": [
        "sdks/typescript/src/client.ts",
        "sdks/python/src/echo_agent_sdk/client.py",
        "sdks/java/src/main/java/com/echoagent/sdk/EchoAgentClient.java",
        "sdks/typescript/test/family-route-host.test.js",
        "sdks/python/tests/test_family_route_host.py",
        "sdks/java/src/test/java/com/echoagent/sdk/FamilyRouteHostTest.java"
      ],
      "evidence_refs": [
        "evidence.sdk-family-catalog-method-routing-repair",
        "evidence.sdk-family-catalog-method-routing-verification"
      ]
    }
  }
}
---

# Family catalog method routing independent rereview

## 审查范围

实际独立sdk_capability_acceptance_audit与协调者回收的limited pass0结论，绑定TS8a33/PY3aba/Javae559及三语言c52真实各一场景。审查来源为同goal中的独立复审消息，不伪称额外机器report；归档receipt标明这一来源。

## 已检查故障假设

family标识与wire拼写相同的错误假设；wrong-family绕过catalog；修复暗增wire/API；原始失败与Python共享state rootfixture错误混淆；旧全套green冒充新source通过。

## 实际实现路径与证据

catalog.resolve决定method/signature，三客户端只转换参数。content_guard实际content-guard路径、UTF8redact_exact与call/family可消费，wrong family在调用前拒绝，实际日志/XML保原输入。review结论只覆盖该局部缺陷。

## 问题记录

此范围独立复审未发现remainingfinding。repair/verification与rereview齐备，因此可本地resolved；能力模型仍needs_review。

## 残余风险

完整scope独立build、所有family异常、最终pin和mainCI仍独立待交付；不关闭外部Issue、不registrypublish。

## 未检查项

新后继codec源码、未列family/操作、全平台与最终线上仓库快照。
