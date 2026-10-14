# a634 候选完整三语言门禁

本包记录一次官方 `bash scripts/check-language-sdks.sh` 完整入口，2026-10-14 06:30:05.921Z 至 06:35:27.085Z，真实退出 0、100567 字节输出未截断。不是把分阶段检查拼成一次通过：TypeScript 308 tests 全通过、0 failed/cancelled/skipped；Python 375 passed 加 18 subtests passed；Java 当前 71 个 XML 合计 332 tests、0 failures/errors/skipped。71 个 XML 在其它 Maven 执行前立即原字节保存，原 mtime 全在此次完整命令时间窗内，manifest 保留每个 XML 的原 mtime、计数和 SHA。

TypeScript/Python/Java quickstart 均由同一入口实际执行，完整日志分别保留 Node 总结后的 `echo-agent`、pytest 总结后的 `echo-agent`、末尾 `echo-agent Java SDK connected: agent-2`。Ruff check/format 真实通过，184 files already formatted。编译警告、SLF4J provider 警告为 0；原始负例 RPC 运行时 WARN 完整保留并单独计数，没有过滤日志或改变断言。

框架精确 revision 为 `a6346643bda47679a96760f56f45e41fd73b61b0`，真实 Host SHA256 为 `193e49b8557db9b746a1cc8c6f5cf30bb6d3154a630562f7ef8551b949e878d1`。原 receipt SHA256 是 `b1c6ed389e99b2ad3d37c87e303e6580c5a38a72d3cb5a98943352cc0188d77c`；其中 `sha256` 字段 `af652b11a1774e77cb1f9486755e2598c2ab3bb6652a9096d6a85d4c326fa89b` 指完整日志摘要，不混称 receipt 摘要。原 Admin 路径、命令、时间、上次失败路径在 receipt 原样保留。

790 项输入 before/after 内容完全相同；归档时重新核对当前 790 项也全匹配。有限源码副本仅包含官方入口、三个语言 client、实际 Task/Subagent fixture、Host manifest 和 Cargo 配置，没有复制整份 790 源码树。执行身份由原输入摘要固定；SDK 在此期间由主线程提交语言开发里程碑，manifest 的当前 Git HEAD 只是归档时观察，不能冒充此次整个候选源码在运行时已经进入该 commit。原 Task fixture 明确发送 `Connection: close`，此次只证明这一已执行 HTTP fixture 生命周期；不推出任意长期持有的外部 socket 均已关闭。

之前完整失败的 receipt/log、790 项 before/after 和 71 XML 已原字节另存 `history/failed/`（332 tests、0 failures、1 error、0 skipped），与本次通过的 71 XML 分开；原目录也保留。`http-fixture-counterexample/` 保存受控两次 HTTP 机制实验的 7 原文件、原 fixture/关键 Java 源与 class、原失败 Task 状态、唯一 header patch、修复后原 Task class 的 4 tests 全绿收据/日志/XML及相关 JDK 源片段。未重复复制六个完整诊断运行树。受控实验真实观测同 socket 第二次 POST 被非持久连接关闭，显式 close 后第二次使用新 socket；唯一改动是 fixture 的 `Connection: close` header，未改变普通 JDK HTTP 客户端策略。原全套失败请求与具体端口的关联仍不可得，不能把受控实验 socket 身份冒充原失败现场。14 项 Rust 门禁另见 [Rust 证据](../sdk-a634-rust-gates/README.md)。本入口 export/catalog check 仍针对现有接受合同候选，不替代十二项 accepted-source 连续性裁决和完整生成；SDK main、远端适用 CI 和最终交付须由主线程继续验收。本次只有证据归档，没有运行新测试或改动生产源码。
