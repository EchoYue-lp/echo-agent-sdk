# 测试 Host SHA helper 的 Python 启动器兼容修复

这是 [原分块 SHA 证据](../host-binary-streaming-sha/README.md)之后的新单点修复，原 40 文件包和校验和保持不变。本切片只给 `sdks/shared/testing/file_sha256.py` 添加 `from __future__ import annotations`，函数体和其它 AST 完全相同；没有改 SDK 生产代码、Python SDK 支持下限、schema、fixture 业务或 PATH。

原官方完整语言脚本在 SDK `65c2c304b9a232f7a0bddc44c12cd8e9057db05f` 退出 1。800 输入前后未变，TS 336 项中 335 通过、唯一 Computation 失败；Python、Java 和 quickstart 阶段尚未执行。历史完整日志 93,853 字节 SHA256 `bceaedfdb43d224a3cda60846a0706c51fad87715bffc132245257924617a1ee` 保留在 `history/`。原收据还包含旧 target 的 Java 354 计数，但 `allJavaXmlFresh=false`；这些数字不证明本次失败脚本执行过 Java，不用于本切片通过声明。

TS Computation 明确调用 `execFileSync('python3', ...)`，实际解析为系统 `/usr/bin/python3`（解释器自身路径 `/Library/Developer/CommandLineTools/usr/bin/python3`），版本 3.9.6。UV_PYTHON 选择的 3.10.19 不会重写这一显式子进程命令。新 helper 没有延迟注解时，`Path | str` 在导入时被系统解释器求值，原最小导入真实退出 1 并抛出 TypeError。相邻 `computation_fixture.py` 已使用延迟注解；本修复直接复用此现有模式，而不是改变 launcher 或掩盖错误。原 helper `e0902802…` 与修复后 `0aa3acef4d3bb87f94ce521ac48c0b92b9102256b359ca00d9459ec27e31dc0b` 的完整字节、差异及最小导入前后日志均保留。

同一系统 `python3`、同一 cwd 和输入在修复后导入退出 0，实际摘要匹配文件字节。随后仅执行已授权的两个 focused 命令：真实 Node20.20.2 Computation 1 个测试方法通过，61 次 public SDK/Host 调用、25 个预期 typed error、11 组 oracle、8 个受保护文件均按原场景完成；Python3.10 原 6 项 SHA helper 单元测试通过。`native/results.json` 和 `scenario.json` 保存原 61 行响应与输入。Host 保持真实 immutable `0f1ecf23a2875621caf5128327046cbf6592ce61665c6c6872aed33fdf1931df`。这不是对 typed error 放宽或把方法数冒作调用数。

focused 命令使用实际 Node20 路径、TZ=UTC、UV_PYTHON=3.10、原系统 python3 和 PYTHONDONTWRITEBYTECODE=1；没有 PATH override。原官方环境精确值 UV_PYTHON=3.10.19 保留在原收据，focused 的 3.10 选择仍解析同已安装 3.10.19。不是逐字完全相同的 shell 环境：TMPDIR 改为本切片独立 Admin 根，避免覆盖原失败运行。`source-and-result-receipt.json` 明列两者，run-command 原收据记录实际 argv/时点/退出码/截断状态。Ruff check 与 format check 均实际退出 0（工具会话输出观察，不伪造命令收据）。

归档只复制 16 份必要原件，不复制 Runtime 服务目录、target 或 `.pyc`。`archive-manifest.json` 绑定原路径、字节数和 SHA，`SHA256SUMS` 覆盖除自身之外的每个成员。原 before/after 观察、历史失败与新成功保持各自真实时点，不将后继提交说成原执行 HEAD。

这仅是测试解释器注解求值兼容修复。Python SDK 的既定最低版本仍为 3.10，未新增 3.9 产品支持承诺。未重跑整个三语言脚本、Maven、Cargo 或 Rust 矩阵；完整官方 gate 与 Finding 结算由主任务在新源码切点另验。示例和 website 无受影响产品行为，无需同步修改；独立源码增量与材料审查在本次封包后执行。
