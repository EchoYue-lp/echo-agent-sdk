# Java source verification 的 test-only NOP provider

SDK library compile/runtime依赖不强制应用日志provider。本轮仅pom.xml新增test-scoped org.slf4j:slf4j-nop:2.0.18，与实际slf4j-api2.0.18版本匹配，显式保留原未配置时NOP行为。源码验证suite及standalone source quickstart使用测试classpath，不以runtime/compileprovider强加SDK下游。

旧官方whole日志有6条SLF4J(W)未配置provider提示；不是允许豁免的编译警告，本轮消除其配置根因，没有loggerlevel/suppress flags。dependency-tree精确api2.0.18 compile / nop2.0.18 test；actual-compile.cp无NOP、actual-test.cp与官方default-example.cp有NOP。最初探针误用mdep.includeScope（插件真实userproperty是includeScope）导致默认allclassPath被误读，原compile.cp/test.cp保留；classpath-isolation.json明确这一测试命令错误，不把它当原SDKscope泄漏。

真实fullMaven67suites320tests0failure/error/skip＋classpath＋Javaquickstart exit0，完整日志无SLF4J(W)。所有67XML原字节归档。随后同官方完整scripts/check-language-sdks.sh以新POM执行全部语言，其终态另归sdk-language-test-nop-complete，不拿这段Maven替whole。

原负例Host RPC error response WARN继续保留，不被该Javaprovider吞掉或变成假成功。旧完整language门禁log恒保，当前fullgeneratedaccepted-source/main验收范围仍单独未完成。
