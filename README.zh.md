# echo-agent-sdk

## SDK消费者解耦

当前SDK源码已撤出A2A/LSP的执行feature、family adapter和对应语言菜单。原Plan04
精确撤销269项accepted身份、23条路由、39个操作，保留5352项身份／431条路由，新增
接受为0；该撤销证明不因Framework后继物理删除而重复执行。

标准ACP、独立Host stdio、MCP、Task/Subagent、ProjectRoot/Instruction、合法自定义工具、
签名、回放、取消与恢复继续保留。Host正式构建来源以其Cargo manifest／锁为准；消费者
去依赖和共享投影适配不等于正式新Git pin、删后Host、合同生成或运行验收通过。
详见[ADR0042](docs/adr/0042-sdk-a2a-lsp-retirement-boundary.md)，下方历史验收保原来源与范围。


`echo-agent` Rust 框架的源码优先 TypeScript、Python 和 Java SDK，通过源码构建的 ACP
Host 与 `_echo_agent/*` 扩展合同连接同一个 Rust 运行时权威。

## 迁移状态

本段观察时Host正式manifest固定Framework
`f7bac62eee04a963f92a3e4308b6ea2336e91cc0`；`27c7701e1eb116db1076da7f84bb68898544a44c`
是原SDK迁移历史来源，不再作为当前pin描述。Protocol不依赖Framework，accepted external
contract与完整Rust inventory telemetry分开维护。实际采用物理删除后的真实Framework
提交、正常生成锁及同源Host／消费者验收仍须独立完成；本仓库不发布捆绑Host或运行时。
精确来源以正式manifest/锁和原门禁回执为准，不用顶层gitlink或受控path代替。历史来源见
[MIGRATION-SOURCE.md](MIGRATION-SOURCE.md)与[ADR0001](docs/adr/0001-sdk-repository-boundary.md)。

## 目录

- `echo-sdk-protocol/`：ACP 扩展 wire value、schema、catalog 和合同工具。
- `echo-sdk-host/`：把 `echo_agent` 适配到 ACP 与 SDK 方法的源码 Rust Host。
- `contracts/sdk/`：accepted external contract、schema、fixture、Host catalog 和 Rust inventory telemetry。
- `sdks/`：TypeScript、Python、Java 和 shared SDK 源码。
- `docs/sdk/`：SDK protocol、Host、extension bridge 与 facade 文档。
- `scripts/`：合同和三语言源码验证入口。
