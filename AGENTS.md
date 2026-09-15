# echo-agent-sdk AGENTS.md

本文件是 AI agent 在本仓库工作时的最高优先级约束。本仓库可独立检出使用，不依赖
superproject 中的规则文件才能冷启动。

## 仓库定位

`echo-agent-sdk` 是 `echo-agent` 的独立、多语言 SDK 产品仓库，承载：

- Rust `echo-sdk-protocol` wire/schema crate；
- Rust `echo-sdk-host` ACP Host 和 framework adapter；
- TypeScript、Python、Java SDK；
- SDK contracts、生成器、文档、示例和 CI。

`echo-agent` 是独立通用框架，也是本仓库的上游依赖；`echo-agent-cli` 是 EKO 应用，
不是 SDK 行为权威。依赖方向只能是 SDK 指向 framework，framework 不得依赖本仓库。

## 当前迁移门禁

当前仓库处于 source import 阶段。源快照、已知未闭合项和后续入口记录在
`MIGRATION-SOURCE.md`。在上游 `echo-agent` 当前语义治理完成并冻结可解析 revision 前：

- 允许原样承接 SDK-owned 源码、合同、三语言实现、文档、脚本和 CI；
- 不以当前 contract drift、scope count 或独立构建失败为由在两边追逐修改；
- 不宣称 SDK 已独立构建通过、合同闭合、Runnable 或 Parity complete；
- 后续集中收敛必须以已确认的跨仓设计和计划为执行来源。

## 分层与唯一权威

- `echo_agent` 唯一拥有 Agent、Run、Task、PlanTask、SubagentRun、事件、重试、取消、
  恢复、持久化和 terminal 语义。
- `echo-sdk-host` 只做 ACP/runtime 组合、无损类型转换、handle/resource 生命周期和本地
  transport，不建立第二套执行或状态权威。
- `echo-sdk-protocol` 的目标边界是纯 wire/schema，不依赖 `echo_agent`、`echo_core` 或
  其它 echo framework crate。source import 阶段的旧依赖是待收敛缺口，不是长期定位。
- 只有明确接受的跨语言 external contract 属于阻断兼容面。完整 Rust public inventory
  只作为非阻断漂移遥测，不得自动扩大 SDK 承诺。
- 统一使用 `Subagent` / `subagent` 术语；仅第三方固定 wire name 可在最小适配边界保留。

## Rust 硬性约束

- 任何可能包含中文或 emoji 的字符串禁止字节切片和字节长度截断；使用
  `.chars().take(N).collect::<String>()`。
- 禁止 `.unwrap()`、`.expect()`、`panic!`、`unreachable!`、`todo!` 和可能越界的直接
  索引。异常路径返回 `Result` 或显式处理。
- 整数运算可能溢出时使用 `checked_*`、`saturating_*` 或明确的 wrapping 语义。
- Adapter 转换必须字段级无损；不得用 schema-free JSON、字符串拼接或默认值吞掉
  identity、generation、sequence、deadline、error 和 terminal cause。

## 文档、合同与示例

- SDK 正式文档、ADR、示例和合同只在本仓库维护；framework 仅保留边界说明和链接。
- 任何公开 wire/语言 API 变化必须同步 protocol、Host、三语言 SDK、fixtures、文档和
  contract tests。
- 生成物只能由仓库唯一生成入口更新，禁止手工修改生成 JSON 来绕过漂移检查。
- 示例必须进入可执行编译或测试链路。
- 架构变化必须记录 ADR，包含背景、候选、决策、取舍和影响。

## 分支、提交与交付

- 新任务使用非 `main` 分支，命名 `<type>/Echoyue/<任务名>`。
- 每次提交显式关闭 GPG 签名：
  `git -c commit.gpgsign=false commit -m "..."`。
- 开发提交执行 focused 验证；合并前执行全部适用 Rust、contract、Host E2E、三语言、
  cross-platform、dependency 和文档门禁，所有命令必须零失败、零警告、零格式差异。
- 不使用 `--no-verify`，不把预先存在或看似无关的失败作为合并豁免。
- 推送后核对远端 SHA；superproject gitlink 只能引用已经推送的 commit。
