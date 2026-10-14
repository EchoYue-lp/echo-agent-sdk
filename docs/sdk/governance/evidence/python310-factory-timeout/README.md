# Python3.10 factory cleanup 超时错误映射修复

本包记录一个已接受 extension deadline 行为的最低支持版本修复。生产差异仅为 `sdks/python/src/echo_agent_sdk/client.py` 的一行：`except TimeoutError as error` 改为 `except asyncio.TimeoutError as error`。`reviewable.patch` SHA256 `6e0d20a42152f47ecd3dd66e5aaeb23285fe48c2bf7f00227cb0de83e3af8c1f`；原 client `fc843caf…`，修复后 `399961d089a9dcc59af871c377d700d6a8f4c0b5d4fd209f50a3c7a0ac6a1f8b`。原完整测试文件 SHA `6de7f5e8f9fdbad783b652c362de97e261c687bca21c5eb2fda9a87a5b160e60` 未改，断言、等待时间和 skip 均未变。

原第三次官方完整脚本退出 1：TS 336/336、TS quickstart 及 Ruff 195 文件已完成；Python 为 1 failed、414 passed、18 subtests passed。原 95,086 字节日志 SHA `5954cd5dac9e461ed5de095cb8f2b214b22d39b03762bf9fd5fdf552f1a2c309` 和收据逐字节保留在 `history/`。800 输入前后相同。Python quickstart 和 Java 尚未到达；原收据的旧 Java354/allJavaXmlFresh=false 不是本次执行证据，不能纳入通过计数。

根因不是 test 预期错误：Python3.10 的 `asyncio.wait_for` 抛出 `asyncio.exceptions.TimeoutError`，它不属于内置 `TimeoutError`。该异常漏过原局部捕获，落入通用 callback Exception 分支而错误返回 `extension_failed` / retryable `never`。Python3.12 中 asyncio.TimeoutError 是内置类的别名，原测试因此通过。真实原单用例在3.10失败、3.12通过，两次原 run-command 收据和日志保留；真实 wait_for+shield 小型 probe 也显示两版本异常身份不同，而且被 shield 的同一 future 未取消、仍可后续完成。probe 的 Admin 直接 `uv --no-project` 输出含一条无 project 提示，原样记在 observation，不伪装成零警告门禁。正式 pytest 和版本读取在 SDK Python cwd 中运行。

修复复用同文件 `_settle_process_close` 已有 `except asyncio.TimeoutError` 模式，保持原 `EchoAgentError(extension_timeout, ..., after_delay)` 映射。deadline/1ms floor、shield、pending future、reserved child owner、handler、finally 和成功 close 回收条件均不变。等待超时不取消 construction 或移除 child；实际 construction 结算后仍由同 owner 重试，重复 close 不再执行。原测试的这些断言继续有效。[扩展桥合同](../../../sdk-extension-bridge.md)明确 construction deadline 不证明工作停止，必须保留 cleanup/retry owner；不新增 API、状态权威或错误码。

局部方案在原源码未修改时已经独审 PASS C0/I0/M0；获得同 goal 新鲜预检后才实施一行。修复后运行整个原 `tests/test_protocol_integrity.py`，Python3.10 和3.12 各 **31 passed、0 failed**，包含原失败用例及其它协议完整性用例。两个版本覆盖同一文件，不累计为唯一62用例。真实依赖读取证实两环境 ACP 都为0.12.1。Ruff client check 和 format check 实际退出0。生产文件与原测试字节在上述运行后冻结匹配，源码增量和正式归档材料在封包后另作独立复审。

`history/proposal-README.md` 与 `proposal-manifest.json` 保留只读提案阶段的原状态，不回写为事后执行结果。`after-source.json` 与后续原命令收据记录实施后的事实；不将后继 HEAD 写成原失败执行点。24 个必要原件含完整 client 前后字节、原测试、实际异常 probe、版本、原红绿与新整文件通过日志，不复制运行目录、target 或 `.pyc`。

本切片没有 Host/native、Cargo、Maven 或完整语言重跑；它只证明这处最低支持版本 timeout 映射和原文件回归。新源码的完整官方语言验收由主任务另行执行。Hash helper、旧 bridge 和哈希补证包不回改，Python SDK最低支持仍为3.10，Rust61输入不因这一 Python 客户端差异变化。示例、website 和公共 wire 行为无新增变化，无需同步新 API 文案；不得以本包关闭其它 Finding、合同十三项阻断或 main 交付。
