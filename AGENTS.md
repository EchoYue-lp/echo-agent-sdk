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

仓库迁移从 `echo-agent@c5f7688` 补齐 SDK-owned 源码与过滤历史，原 extraction pin 为
`27c7701e1eb116db1076da7f84bb68898544a44c`；该历史归属保留在 `MIGRATION-SOURCE.md`。
当前 Host 精确依赖已发布 framework `9886502c83d6b1f6fa9212eb221db0e5874deaee`，
来自[framework PR #190](https://github.com/EchoYue-lp/echo-agent/pull/190)，上游精确main CI38862267005七项通过由协调者核验。
本SDK真实Cargo.lock八包同Git revision，无path patch；唯一全28profile生成和完整合同check已实际0，
accepted 5621项不增删，仅已批准A单cap/protect Unit与B两fallible签名由规范入口更新。
当前Host生命周期修复、最终profile、完整Rust/三语、独审和SDK精确main CI尚待，不借历史pin通过替代。
来源与历史分界见`MIGRATION-SOURCE.md`和`docs/MASTER-PLAN.md`。

历史00782 Host治理候选精确依赖 `00782b8aeddefe2ea8c566922cbdee6aedc80d70`，
来自 [framework PR #186](https://github.com/EchoYue-lp/echo-agent/pull/186)，承接 Git 分支切换无损修复。
SDK 锁文件另固定 rustls 0.23.46 安全补丁。framework 签名、精确源码树与合并后 CI 已核对；
[SDK 新 pin/锁](https://github.com/EchoYue-lp/echo-agent-sdk/blob/0ebe15f66d0aa57c7b7a92ae68cd1be106114ae9/docs/sdk/governance/evidence/sdk-published-00782-pin/README.md)、
[14 条 Rust 门禁](https://github.com/EchoYue-lp/echo-agent-sdk/blob/0ebe15f66d0aa57c7b7a92ae68cd1be106114ae9/docs/sdk/governance/evidence/sdk-00782-final-rust-gates/README.md)及
[第四次完整语言入口](https://github.com/EchoYue-lp/echo-agent-sdk/blob/0ebe15f66d0aa57c7b7a92ae68cd1be106114ae9/docs/sdk/governance/evidence/sdk-language-ci-repairs-final/README.md)具实际本地通过证据。
800项输入已逐Git恢复；本地Oracle21不代Linux CI Temurin17，新action警告及当前源码语义/main验收仍待实际结果。此前
`be5f0a8405b593c4ef964cd6fd2df4beabf4956c` 的 [published-pin provenance](https://github.com/EchoYue-lp/echo-agent-sdk/blob/0ebe15f66d0aa57c7b7a92ae68cd1be106114ae9/docs/sdk/governance/evidence/sdk-chart-published-pin/published-pin-provenance.json)
保留为历史构建证据，不证明当前升级后的 Host 已通过最终门禁。
[历史 a634 Rust 门禁](https://github.com/EchoYue-lp/echo-agent-sdk/blob/0ebe15f66d0aa57c7b7a92ae68cd1be106114ae9/docs/sdk/governance/evidence/sdk-a634-rust-gates/README.md)及
[完整语言门禁](https://github.com/EchoYue-lp/echo-agent-sdk/blob/0ebe15f66d0aa57c7b7a92ae68cd1be106114ae9/docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/README.md)
固定各自实际源码切点；后续运行时修复与幂等验收不得借用此前通过结果。
完整合同、新适用 CI、当前源码语义和 SDK main 最终验收仍须分别结算。
[十二项 accepted-source 连续性方案](docs/sdk/governance/accepted-source-continuity-proposal.md)
已于2026-10-15由用户明确接受，精确授权位于ADR0034；完整合同生成与消费者验收由该独立切片实际执行，SDK main 验收仍独立，不能由 pin 已发布推断 release。
精确源快照、provenance 和历史归属记录在 `MIGRATION-SOURCE.md`：

- protocol 是纯 wire/schema crate；framework 类型转换只能位于 Host adapter；
- `source-contract.json` 只覆盖 accepted external contract 和 accepted catalog；完整
  Rust public inventory、Host-only catalog、Cargo.lock 与 framework revision 只做遥测或构建 provenance；
- framework 与 SDK 不并行维护同一修复；framework 语义治理完成后，SDK 通过显式 revision
  升级集中闭合 accepted contract 变化；
- 独立构建、Host E2E 和三语言 gate 是当前交付的剩余验收面，在全部通过前不得宣称
  release 或 Parity complete。

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
- GitHub CI 仅在 push 到 `main` 后运行；任务分支和 PR 不自动触发 CI。任务合并前完成本地适用门禁，合并后由 CI 核验精确 main。
- 每次提交显式关闭 GPG 签名：
  `git -c commit.gpgsign=false commit -m "..."`。
- 开发提交执行 focused 验证；合并前执行全部适用 Rust、contract、Host E2E、三语言、
  cross-platform、dependency 和文档门禁，所有命令必须真实退出0、零失败、零格式差异。
  编译、audit、advisory、license、source、配置及其它工具警告必须为0；唯一例外是
  cargo-deny 原有 `multiple-versions = "warn"` 产生的 `warning[duplicate]`，仅在完整
  原始输出保留、版本组按已接受的精确 Cargo.lock 摘要基线复核且无未经复审的增量时，
  作为非阻断提示。锁图、framework revision、检查范围或策略变化必须重新验证与复审，
  不得自动扩大基线或推广为其它警告豁免。已接受的50组精确版本及裁决见
  [依赖警告政策](docs/sdk/governance/dependency-warning-policy-proposal.md)。
- 不使用 `--no-verify`，不把预先存在或看似无关的失败作为合并豁免。
- 推送后核对远端 SHA；superproject gitlink 只能引用已经推送的 commit。
