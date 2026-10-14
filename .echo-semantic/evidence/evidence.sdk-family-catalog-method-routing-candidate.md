---
{
  "schema_version": 1,
  "id": "evidence.sdk-family-catalog-method-routing-candidate",
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
    "docs/sdk/governance/evidence/sdk-family-route-20261013/content-guard-public-call-route-failure.log",
    "docs/sdk/governance/evidence/sdk-family-route-20261013/sdk-family-route-typescript-green.log",
    "docs/sdk/governance/evidence/sdk-family-route-20261013/sdk-family-route-python-serial-green.log",
    "docs/sdk/governance/evidence/sdk-family-route-20261013/sdk-family-route-java-green.log",
    "docs/sdk/governance/evidence/sdk-family-route-20261013/TEST-com.echoagent.sdk.FamilyRouteHostTest.xml"
  ],
  "supports": [
    "behavior.sdk-family-content-guard",
    "behavior.sdk-language-clients"
  ],
  "limitations": [
    "Three languages each one actual real-Host test for catalog family method spelling; not every family or final full language gate.",
    "Original frozen source hashes and later client changes require explicit source comparison, not observed_at promotion.",
    "Python concurrent state-root collision failure retained; serial corrected fixture scope succeeded.",
    "Independent rereview, published pin and precise main pending; accepted scope unchanged."
  ]
}
---

# SDK family catalog method 路由候选证据

## 支持的结论

客户端 family 名称与 wire method 拼写不是同一字段。已接受 content_guard family 的方法为 `_echo_agent/content-guard/op`；旧三语言通过family字符串拼接下划线路径，普通public call/family在catalog签名解析阶段失败。修复后既有catalog.resolve决定method，核对operation属于请求family再使用同catalog signature；不增加API或第二路由authority。

## 来源与范围

归档保留旧Python普通调用失败与实际三语言各一项真实Host成功：Unicode redact_exact分别经call/family消费，并拒绝wrong family。Host SHA c52da2d346d8848074b646e5442ecc83890cd68991f08287780280f871b59f89。manifest逐源码/测试和日志/XML SHA可复核；本次仅审计这些原始材料，不重跑或宣称整体语言完成。

Python首次并发测试共享state root造成连接关闭的fixture失败原日志保留；改为串行同配置公开spawn后通过，不把该失败冒充family路由生产根因。最终输入仍须按各次运行来源冻结，不能仅改observed_at使旧结果成为新运行。

## 已知缺口

所有family/wire spelling矩阵、最终完整语言/示例门禁及独立复审、已发布framework pin和精确main仍待。accepted contract未扩展；catalog数量/done不是行为证据，Finding继续open。
