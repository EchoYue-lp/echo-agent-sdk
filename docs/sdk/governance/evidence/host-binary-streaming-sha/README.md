# Host 二进制分块 SHA256 补证

本包保存测试证据层的修复与有限验证。CI 的真实 Node 20.20.2 遇到 2,959,264,016 字节 Host，原整文件读取抛出 `ERR_FS_FILE_TOO_LARGE`。`history/language-cancelled.log` 保留原取消运行的完整字节；该运行同时含其它测试故障，不把全部 not-ok 都归因于哈希，也不把取消称为完整通过。原本地 Node 22.19 与约 707 MB 的 Host 没有覆盖 CI 的大文件条件。

修复切片为 22 个已有调用/独立 oracle 文件与 6 个新 helper/单元测试，共 28 个源码路径。`source-freeze.json` 绑定每个 SHA；`existing-callers.patch` 以 SDK `9a59943e70da06b1af231b1fb22bddc879885f28` 为原始来源，六个新文件的实际字节位于 `executed-source/`。这不是宣称修复已在那个旧提交中运行。三种语言各自复用唯一测试 helper；Java 提取原 Channel 测试已存在的正确循环，固定最多 1 MiB 分块至 EOF，并保证关闭文件及原 IO 错误可见。

仅 Host/executable 的摘要表达式被替换。逐行采样位置、完整 lower-hex SHA 字段、错误和结果数据均保留；不使用跨场景缓存，不改变小 fixture/source 摘要。Python 七个 consumer 和共享 computation oracle 的非摘要业务 AST 保持；源码独立复审已 PASS C0/I0/M0。Telemetry 的三语言文件、其服务清理和时间语义修复由其它切片负责，不在本 28 路径内；本包不替它们结算。

| 实际运行 | 结果 |
| --- | --- |
| Node 20.20.2 新 helper 单元测试 | 6 通过、0 失败 |
| 本地 Node 22.19 新 helper 单元测试 | 6 通过、0 失败 |
| Python 3.10 最终源码单元测试 | 6 通过、0 失败 |
| Java `FileSha256Test` | 3 个 JUnit 方法通过，0 failure/error/skip |

各版本重用相同逻辑场景，不累加为唯一测试数。覆盖空文件、Unicode、跨分块与非整块尾部、真正超过 2 GiB 的稀疏文件、缺失路径和目录 IO 错误。Java 三个方法将多个输入分组，不能称为六个 JUnit 测试。Java 原收据的实际执行环境是本地 Oracle JDK 21.0.5 / Maven 3.9.0，不是远端 Temurin17；源码由统一 producer 编译，原 XML SHA 为 `18baa745801c532506ec68d28ee775552fc70ffca4905d43a0b171ace20630fa`。附带的原 Java 运行前映射与总结包括 37 路径的合并窗口和时间切片结果，作为来源保持原样，仅本 28 路径与哈希方法结论属于此包。

原 Node 20 机制反例使用 2,147,483,649 字节的稀疏文件，实际仅占 36,864 字节。旧整文件读取先抛出原错误；固定 1 MiB 分块进行 2,049 次读取，完整摘要为 `05aa6dd2c62008228e2cb1ab20553b85be8565f3ddf82f0527661905c4015750`。原 probe 的峰值 RSS 为 41,600 KiB；这是该受控 probe 的进程观察，不是共享 helper 每个单测或真实 Host 的 RSS 测量。正式单元测试使用实际 helper 并核同一大文件摘要。稀疏文件、npm Node 安装缓存、运行目录、target 和 `.pyc` 均未复制。

初次 Python Ruff 有四处 I001，原失败日志保留；仅四处 import 排序修正后，最终十个 Python 文件 Ruff check/format check 均 0，最终 Python3.10 单元测试另一次通过。初次通过的 Python 单测保留其先前 epoch，不冒最终源字节。九个 JavaScript 文件 Node20 语法检查全部 0。原始 run-command 收据保留实际 argv、路径、时间、退出码和截断状态。归档阶段只复制字节，不重新运行测试。

源码正确性审查已通过，材料归档审查在本次封包后独立进行。此包仅证明摘要机制、调用替换和对应单元验证，不是完整三语言 Host/quickstart 门禁，不关闭尚待全量验收的 Finding，也不承接合同十三项阻断、inventory parity、SDK main 或发布。Rust 61 输入未因这次测试支持修复变化；没有机械重跑 Rust 矩阵。SDK 生产 API、schema、取消策略、seed 和 framework 权威均未改。示例与 website 没有相应行为变化，故此归档切片无需修改它们。
