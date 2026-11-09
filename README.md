# echo-agent-sdk

## SDK消费者解耦

当前SDK源码已撤出A2A/LSP的执行feature、family adapter和对应语言菜单。原Plan04
精确撤销269项accepted身份、23条路由、39个操作，保留5352项身份／431条路由，新增
接受为0；该撤销证明不因Framework后继物理删除而重复执行。

标准ACP、独立Host stdio、MCP、Task/Subagent、ProjectRoot/Instruction、合法自定义工具、
签名、回放、取消与恢复继续保留。Host正式构建来源以其Cargo manifest／锁为准；消费者
去依赖和共享投影适配不等于正式新Git pin、删后Host、合同生成或运行验收通过。
详见[ADR0042](docs/adr/0042-sdk-a2a-lsp-retirement-boundary.md)，下方历史验收保原来源与范围。


Source-first TypeScript, Python, and Java SDKs for the Rust `echo-agent`
framework, connected through a source-built ACP Host and the
`_echo_agent/*` extension contract.

## 当前来源与验证（2026-11-09）

当前 Host 精确采用 Framework main `3413d64dae23353c6d02c546ac374deded855827`，
SDK 锁 SHA-256 为 `95e3caabaf5e72fc71ad1d6b79964eae1883b297f0006667e39c4c3a6eb8c5e5`。
锁内共 654 包，13 个 Git 包均来自同一精确 HTTPS 来源。accepted 范围为 5344 项身份／430 条路由；
[ADR0046](docs/adr/0046-cache-layout-source-identity-projection.md) 的四条缓存字段来源处置
保留三语言旧公开字段，仅映射真实 Rust `dynamic`，不新增语言 API。

本机完整生成、合同检查、Rust／Host 矩阵和三个语言完整入口已按实际快照通过。
TypeScript 527 项通过，Python 601 项及 18 个子测试通过；Java 400 项测试零失败、零错误。
各语言保留 1 个原有 Windows 原生跳过项，三个 quickstart 通过，完整语言入口零警告。
这不是 Windows／Linux 原生或精确 SDK main CI 通过；本轮固定 c0bc 的 strict Source／Final
尚未执行，SDK main 合并、精确 CI 与发布资格另行结算。来源、风险裁决及原始回执见
[当前验证状态](docs/sdk/governance/framework-development-pin-status.md)。

## 历史迁移记录

以下为历史 f7 接受切点，保留其原 SHA、CI 和验证范围。该切点的 Host 采用 framework `f7bac62eee04a963f92a3e4308b6ea2336e91cc0`
(tree `0acc8d94603b647e4d795c0b2eea20ecb0c3da7d`), from
[PR #197](https://github.com/EchoYue-lp/echo-agent/pull/197). Its original
[main CI run 39605663648](https://github.com/EchoYue-lp/echo-agent/actions/runs/39605663648)
completed all seven jobs, followed by an independent remote-only cold source
check. This upstream acceptance is not a passing SDK Host or consumer result.
The historical pin-switch SDK lock SHA-256 was `1c865a57b7e17587fba2f668d956832cd7b0f6c6167e198304c965227f3debaa`: only the eight framework and three
upstream vendored Git source identities change from the existing webpki-patched
SDK lock; all 656 package versions, checksums and dependency edges are unchanged.
A03 official Cargo update and locked offline metadata each exited 0 using frozen
existing user-mirror cache inputs, and produced this exact lock. The metadata used
`--no-deps` (`resolve=null`); this is source-only proof, not fresh registry, cold
cache, yanked-status or full graph validation. Subsequent SDK acceptance at
[eb223986](https://github.com/EchoYue-lp/echo-agent-sdk/commit/eb22398676310fcbd67e9f4babe4afcefd56ed06) passed [all nine original exact-main CI jobs](https://github.com/EchoYue-lp/echo-agent-sdk/actions/runs/39720432985)
and a GitHub-only cold source check using 36 necessary historical refs.
It records 50 verified behaviors, 75 resolved Findings and one human-accepted
historical risk; the four non-blocking Rust inventory drift artifacts remain
telemetry and were not regenerated.
At that pre-withdrawal qualification cut, accepted membership remained5,621 identity/kind keys and454 routes. Pin/lock
provenance does not authorize a 117-artifact update, new public language APIs,
or automatic closure of the historical managed-exit cause.

### Previous 356 acceptance cut

The SDK-owned import and extraction history includes the original clean pin
`27c7701e1eb116db1076da7f84bb68898544a44c`. That historical Host pinned published
framework `3560812caede84998a1f6f26d7bf3fa81ea0486c` from
[framework PR #193](https://github.com/EchoYue-lp/echo-agent/pull/193), with its
exact squash tree and all seven exact-main CI jobs verified. The canonical
producer completed all 28 profiles and wrote 117 artifacts; the complete
contract check passed without blocking or telemetry drift. The 5,621 accepted
identity/kind keys and 454 accepted routes are unchanged. Only the accepted
`set_model` Rust signature follows its fallible result; new named framework
constructors and context-only fields are not automatically language APIs.
Current source, Host and gate qualifications remain tied to their actual cuts;
see [current acceptance](docs/sdk/governance/capability-review-remaining.md).
SDK main [1e1c4f03](https://github.com/EchoYue-lp/echo-agent-sdk/commit/1e1c4f03f670561d6ca804a56d35967eeea191ce) passed all nine original exact-head CI jobs and a remote-only cold semantic check. The four composite behavior holds, historical managed-exit cause, and native Hook/Stop acceptance remain separate.

The previous `6563d26570cc4a8d93c86b7e37443b0559476807` checkpoint came from
[framework PR #191](https://github.com/EchoYue-lp/echo-agent/pull/191). Its
`RunEntry::wait_result` inventory entry remains Host/Rust-only telemetry; those
historical runtime results do not qualify the current successor source.

An earlier governance candidate resolved
`00782b8aeddefe2ea8c566922cbdee6aedc80d70` from
[framework PR #186](https://github.com/EchoYue-lp/echo-agent/pull/186).
It included the framework Git branch-switch repair; this SDK lock also selects
rustls 0.23.46. Signed source/tree, framework post-main CI and the SDK
[published-pin dependency/build checks](https://github.com/EchoYue-lp/echo-agent-sdk/blob/0ebe15f66d0aa57c7b7a92ae68cd1be106114ae9/docs/sdk/governance/evidence/sdk-published-00782-pin/README.md)
have actual reviewed receipts. [Rust14](https://github.com/EchoYue-lp/echo-agent-sdk/blob/0ebe15f66d0aa57c7b7a92ae68cd1be106114ae9/docs/sdk/governance/evidence/sdk-00782-final-rust-gates/README.md)
and the [fourth complete language run](https://github.com/EchoYue-lp/echo-agent-sdk/blob/0ebe15f66d0aa57c7b7a92ae68cd1be106114ae9/docs/sdk/governance/evidence/sdk-language-ci-repairs-final/README.md)
passed locally with all 800 execution inputs recoverable in Git. Its Oracle
JDK21 result does not replace Linux CI Temurin17; new fixture/action revisions
still require actual remote CI and current-source semantic/main acceptance.
The earlier be5 [published-pin provenance](https://github.com/EchoYue-lp/echo-agent-sdk/blob/0ebe15f66d0aa57c7b7a92ae68cd1be106114ae9/docs/sdk/governance/evidence/sdk-chart-published-pin/published-pin-provenance.json)
remains historical. The protocol remains framework-free, and full Rust
inventory remains telemetry. The [historical a634 Rust gates](https://github.com/EchoYue-lp/echo-agent-sdk/blob/0ebe15f66d0aa57c7b7a92ae68cd1be106114ae9/docs/sdk/governance/evidence/sdk-a634-rust-gates/README.md)
and [complete language gate](https://github.com/EchoYue-lp/echo-agent-sdk/blob/0ebe15f66d0aa57c7b7a92ae68cd1be106114ae9/docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/README.md)
bind their exact execution inputs; later runtime repairs require their own
verification and do not inherit those earlier passes.

The [twelve accepted-source continuity choices](docs/sdk/governance/accepted-source-continuity-proposal.md)
have been accepted and implemented through the canonical producer; their
authority is recorded in [ADR0034](docs/adr/0034-sdk-governance-authority-and-lossless-adaptation.md).
Accepted SDK main governance is recorded at [eb223986](https://github.com/EchoYue-lp/echo-agent-sdk/commit/eb22398676310fcbd67e9f4babe4afcefd56ed06).
Its accepted scope and retained risk boundaries are documented in
[current acceptance](docs/sdk/governance/capability-review-remaining.md).
This source acceptance does not authorize registry publication. This source repository
does not publish a bundled Host or runtime.
See [MIGRATION-SOURCE.md](MIGRATION-SOURCE.md) and
[ADR 0001](docs/adr/0001-sdk-repository-boundary.md) for the source boundary,
history, provenance, and ownership decision.

## Contents

- `echo-sdk-protocol/`: ACP extension wire values, schema, catalog, and contract tooling.
- `echo-sdk-host/`: source-built Rust Host adapting `echo_agent` to ACP and SDK methods.
- `contracts/sdk/`: accepted external contracts, schemas, fixtures, Host catalog, and Rust inventory telemetry.
- `sdks/`: TypeScript, Python, Java, and shared source SDK assets.
- `docs/sdk/`: SDK protocol, Host, extension bridge, and facade documentation.
- `scripts/`: contract and source-language verification entry points.
