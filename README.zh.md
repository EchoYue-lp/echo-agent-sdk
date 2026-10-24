# echo-agent-sdk

## SDK 消费者解耦

Plan04 精确退役 A2A/LSP 的269项 accepted 身份、23条路由和39个操作，保留5352项
身份/431条路由，新增接受为0。Host 独立 pin 仍为 f7bac62e；框架物理删除另属后续结果。
ACP/MCP/Task/Subagent、Host stdio 与 R2 生命周期保留。详见
[ADR0042](docs/adr/0042-sdk-a2a-lsp-retirement-boundary.md)。下方历史验收保持原快照范围。

`echo-agent` Rust 框架的源码优先 TypeScript、Python 和 Java SDK，通过源码构建的 ACP
Host 与 `_echo_agent/*` 扩展合同连接同一个 Rust 运行时权威。

## 迁移状态

当前仓库包含 SDK-owned 历史，并将 Host 精确固定到已推送的 framework extraction
revision `27c7701e1eb116db1076da7f84bb68898544a44c`。protocol 已不依赖 framework，
accepted external contract 已与完整 Rust inventory telemetry 分离。发布状态仍以三语言
和跨仓集成门禁为准；本仓库不发布捆绑的 Host 或运行时。精确源 revision、历史、迁移路径、
provenance 和 ownership 决策见 [MIGRATION-SOURCE.md](MIGRATION-SOURCE.md) 与
[ADR 0001](docs/adr/0001-sdk-repository-boundary.md)。

## 目录

- `echo-sdk-protocol/`：ACP 扩展 wire value、schema、catalog 和合同工具。
- `echo-sdk-host/`：把 `echo_agent` 适配到 ACP 与 SDK 方法的源码 Rust Host。
- `contracts/sdk/`：accepted external contract、schema、fixture、Host catalog 和 Rust inventory telemetry。
- `sdks/`：TypeScript、Python、Java 和 shared SDK 源码。
- `docs/sdk/`：SDK protocol、Host、extension bridge 与 facade 文档。
- `scripts/`：合同和三语言源码验证入口。
