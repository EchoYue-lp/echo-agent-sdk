# echo-agent-sdk

`echo-agent` Rust 框架的源码优先 TypeScript、Python 和 Java SDK，通过源码构建的 ACP
Host 与 `_echo_agent/*` 扩展合同连接同一个 Rust 运行时权威。

## 迁移状态

当前仓库是从 `echo-agent` 提取的源码迁移检查点。上游 framework 仍在完成本轮语义
治理，因此独立构建和兼容性收敛暂不在本检查点完成。精确源 revision、迁移路径、已知
缺口和后续入口见 [MIGRATION-SOURCE.md](MIGRATION-SOURCE.md)。

在 framework pin、protocol 边界、合同生成物、Host 集成和三语言门禁共同闭合前，
不得把当前仓库描述为可独立运行或 Parity complete。

## 目录

- `echo-sdk-protocol/`：ACP 扩展 wire value、schema、catalog 和合同工具。
- `echo-sdk-host/`：把 `echo_agent` 适配到 ACP 与 SDK 方法的源码 Rust Host。
- `contracts/sdk/`：SDK schema、fixture、catalog 和导入的 inventory 资产。
- `sdks/`：TypeScript、Python、Java 和 shared SDK 源码。
- `docs/sdk/`：SDK protocol、Host、extension bridge 与 facade 文档。
- `scripts/`：合同和三语言源码验证入口。
