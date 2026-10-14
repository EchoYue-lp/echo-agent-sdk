---
schema_version: 1
id: evidence.sdk-java-test-logging-classpath
kind: evidence
observed_at: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
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

## 新规则文稿切点的有限观察

本次观测Git`9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d`对应实际新治理文稿/正式TTL归档切点，源码摘要`0a13f994adc90d29746d23c7e071dfb55d8c12eb92c5000b1785092bdc10392f`。原marker`source:pending`及原正文/inputhash/runtimeepoch保；全部source_refs在本Gitblob=currentbyte/anchors核实，旧Git569仍可恢复，不把新的governance文稿Git当过去运行日期或重新执行。

本对象只观察原receipt/人选择/当前政策或接受门禁文稿。依赖用户只明确接受原4c940的50groupduplicate边界，不批准12source/B5或外部CLI-W；28profile12missing correctfail/fullgen未通过，oldcounts/deferred14仍旧27c库存，不autoacceptRustAPI。

原supports/limitations保留，观察不提升41family/8lens、Finding闭合或accepted成员。运行型证据仍原569/17cf/bb4f/b909，不借newcut冒freshSDK代码执行；仅当时原强cases及已完成新scope可用。
