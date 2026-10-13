---
schema_version: 1
id: evidence.sdk-java-test-logging-classpath
kind: evidence
observed_at: source:pending
source_refs:
- docs/sdk/governance/evidence/sdk-java-test-logging-provider/README.md
- docs/sdk/governance/evidence/sdk-java-test-logging-provider/classpath-isolation.json
- docs/sdk/governance/evidence/sdk-java-test-logging-provider/receipt.json
- docs/sdk/governance/evidence/sdk-java-test-logging-provider/ARCHIVE-SHA256SUMS.json
- sdks/java/pom.xml
supports:
- behavior.sdk-language-clients
limitations:
- Testclasspath only, not newruntime log gate or discardedexpectedRPCwarnings
- Fullcontract/main separate
---

# java-test-logging-classpath

## 支持的结论

POM仅testscope slf4j-nop2.0.18，在真正compileCP缺席、test/exampleCP存在；Root回收独立readerPASS0。原misnamed mdep.includeScope probe保失败，不作scope泄漏证据。正式wholeJava32067suite和同三语完整入口0providerwarn，library运行消费者继续自己选日志provider。

## 来源与范围

原归档receipt/输入hash/二进制保持不变。当前观察只绑定模型与可读取归档，不把原epoch冒称最终current全套通过。

## 已知缺口

Testclasspath only, not newruntime log gate or discardedexpectedRPCwarnings；Fullcontract/main separate。
