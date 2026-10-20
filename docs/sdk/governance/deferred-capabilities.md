# 未接受能力的产品处置

本页维护 D01–D14 的产品范围和接受条件。当前治理验收、开放问题及 CI 状态仅以[当前剩余验收](capability-review-remaining.md)为准；下面的前一阶段记录保留其原观察切点，不代表当前状态。

## 当前集合观察（2026-10-20，framework3560812）

原唯一生成器已在精确已发布F3560812下实际完成28profiles/117产物update及fullcheck，canonical库存10973，scope为external5621、Host/Rust-only1908、internal91、language_intrinsic819、deferred2534。accepted identity/kind与454routes不增退；计数只界定集合，不是完成率。下面原6563/2528的2026-10-17段保其历史切点，不改写为今天。

新增canonical10项中，4个Context transform方法是Host/Rust-only；6个仍deferred，按原D01–D14互斥规则归属：

| 原组 | 本次新增，非accepted named API | 原6563数→356数 |
| --- | --- | ---: |
| D01 | ReactAgent::new_with_configured_llm_client | 430→431 |
| D04 | PreparedContextTransform | 84→85 |
| D06 | TranscriptContextOnlyMessage及digest/position字段，TranscriptProjectionCheckpoint::context_only | 269→273 |
| 其它D组 | 无新增 | 原数不变 |
| 合计 | 6项，无自动接受或退役 | 2528→2534 |

原accepted聚合/既有字段不因新named identity仍deferred而丢掉合法完整值：RuntimeStore messages_json仍原样String，当前真实schema3/sidecar由框架解释；SDK不新增DTO/field selector/并行cursor。新配置构造器由Host Rust调用，但deferred是原生成分类，不能手工改称已接受或Host-only。parity entries另有11alias新增不独立计入canonical；只按本表集合归属，不扩大三语合同。

本次deferred surface遥测为bridge47, family9, intrinsic1530, invoke202, standard2, value744，不把route可见性当accepted承诺。三语已接受构造/consumer及错误边界沿原具名Evidence；新SDK整套与main另验，不追加每Rust identity测试。

## 当前集合观察（2026-10-17）

本SDK固定公开framework6563d26570cc4a8d93c86b7e37443b0559476807，其唯一generator生成的当前候选清单中，canonical deferred为2528项，明确accepted canonical为5621项；这不是framework仓库发布的SDK产物。原D01–D14选择规则仍完整、互斥：无重复、未分组或accepted交叉；不因route投影存在就自动接受该Rust identity。当前deferred的route surface分别为intrinsic1530、bridge47、standard2、value740、invoke201、family8。

|原组|当前canonical deferred项数|
|---|---:|
|D01|430|
|D02|227|
|D03|466|
|D04|84|
|D05|118|
|D06|269|
|D07|223|
|D08|111|
|D09|104|
|D10|88|
|D11|159|
|D12|117|
|D13|99|
|D14|33|
|合计|2528|

原1,448项表、实例和旧处置论证保留其27c库存epoch。下方集合脚本按当前生成物重算归属及surface遥测，移除过时“所有deferred必须intrinsic”的假设；不改唯一generator、scope、accepted集合或产品边界。数量仅描述库存，不是完成率或新的语言API承诺。

## 前一阶段有限验收（历史观察，2026-10-15）

SDK Host 精确依赖已发布框架 `00782b8a`，八个框架包同一 Git 来源、没有 path patch；框架合并后七项 CI 已成功。SDK [14 条 Rust 门禁](https://github.com/EchoYue-lp/echo-agent-sdk/blob/0ebe15f66d0aa57c7b7a92ae68cd1be106114ae9/docs/sdk/governance/evidence/sdk-00782-final-rust-gates/README.md)实际全0，workspace 主计数474，不累加重叠 feature profiles。[完整语言入口](https://github.com/EchoYue-lp/echo-agent-sdk/blob/0ebe15f66d0aa57c7b7a92ae68cd1be106114ae9/docs/sdk/governance/evidence/sdk-language-00782-reviewed-final/README.md)实际退出0：TypeScript329、Python394加18子测试、Java351/75份XML及三个quickstart；793项执行输入前后相同，随后逐项保存为可恢复Git。预期负例的运行时WARN按原字节保留，编译告警为0。

当前按实际语义对象读取：52项Finding本地resolved、4项open。Git分支切换同名dirty覆盖与SDK worktree入口栈溢出已各自完成特定修复、消费者验证和独立复审链；本地闭合不宣SDK main或外部Issue已关闭。剩余为12项accepted source/kind迁移，以及cleanup/Sandbox/Task resume三个宽Finding共同关联的5秒物理结算B；只涉及两个待人的裁决单位，不自动退役合同或接受风险。

[完整合同只读检查](https://github.com/EchoYue-lp/echo-agent-sdk/blob/0ebe15f66d0aa57c7b7a92ae68cd1be106114ae9/docs/sdk/governance/evidence/sdk-contract-00782-readonly/README.md)真实退出1，保留13项连续性错误：12项提案未批准，另1项已有独立批准的Curator保存项仍按原证据归因；本轮没有update/accept/retire或seed修改，未到达的后续命令不算通过。40格审查是五个in_scope区域的有界风险映射（38 covered、2根manifest not_applicable），不是41族全部verified或完整生成通过。库存建模已闭合（inventory_closure=closed），行为模型保持open，保留上述B与12项连续性边界。SDK `eebd9f952649cca20d20eabab7f88ecdbaea5813` 已实际通过本地官方全项目strict，包含严格快照及变更依据；该切点工作树与Git源码摘要同为 `473fb206…`，308份正式语义文件的路径、模式、内容与Git一致。这个结果只属于eebd切点；后继文稿或CI修改须按新树重新绑定摘要与验证，不能继承为未来结果。现有远端工程CI没有执行该语义工具，不作为远端语义通过证据；后续固定工具版本的独立语义job仍待实现和实跑。Windows job只检查Rust库，不证明Windows原生SDK消费者；精确SDK main仍须发布后手工官方strict复核。registry发布及CLI/官网外部写入不由本表授权。

具名审查范围见[能力处置](capability-review-dispositions.md)，三链及开放对象见[Finding账本](finding-ledger.md)。历史段中的“当前”“未运行”“open”只描述该段原观察，不作为当前待办；原日志、输入、sourcecut与限制不改写。

## 原延期范围说明（历史来源及D01–D14保留）

本页是 SDK 能力治理的范围解释和后续接受条件，不是新的合同或生成器分类权威。
本页下方1,448项集合仍绑定 framework `27c7701e` 库存epoch，只作遥测。当前 Host pin 为已发布 main `a6346643`，无临时 patch；旧 b909/be5 布局、method identity 与计数保原来源，不能用后继 pin 默认为新的 inventory。客户端局部关闭问题与 events/resources 已按具体证据提交有限审查里程碑，未扩大 deferred 接受范围。Session/Run 意图A已获批准且正在实现，后继 Host 原因转换、live journal、A2A 和新源码切点的最终整套门禁仍待；12项 source/kind 迁移与5秒B选择、完整生成/SDK main仍独立。D01–D14的产品分组、保留Rust/Host与延期理由不变，原accepted操作/字段不因本页被退役。

## 权威与处置含义

- [accepted external contract](../../../contracts/sdk/accepted-external-contract.json) 和
  [accepted catalog](../../../contracts/sdk/accepted-facade-operation-catalog.json) 定义当前
  TypeScript/Python/Java 兼容范围。不能从三语言 `done`、Cargo feature 已编译或完整 Host
  catalog 中有同模块入口推导新增承诺。
- [parity manifest](../../../contracts/sdk/parity-manifest.json) 是完整 Rust identity 库存。
  本次只选择 `canonical == true && sdk_scope == deferred`，alias 不独立计数。
  该生成epoch的1,448项全部为intrinsic路由。数字用于验证分类覆盖，不能用作产品完成率。
- **保留 Rust/Host**：已有能力由 Rust 框架或已接受的 Host 入口提供；当前不接受它的底层
  构造器、控制对象或语言局部 helper 为新增跨语言 API，不删框架 API。
- **继续延期**：产品需求可能成立，但新增控制/配置/部署合同尚未接受。下面明确其行为、
  权威与将来接受条件；延期不取消同能力族已经接受的操作、字段或扩展桥。
- 以上是能力层的处置，不手改 `sdk_scope`，不宣称把 deferred 生成物重新分类成
  `host_or_rust_only`。如将来改变 accepted scope，由唯一生成入口同步所有合同和消费者。

Framework 继续拥有执行、DAG、重试、取消、恢复、commit 与 terminal。SDK adapter 只转换、
组合及管理 owner/generation/resource 地址。下面“已有 Host 路径”只证明源码中的组合边界，
不宣称本轮行为测试已经通过；行为证据与 Finding 见
[能力图](./capability-map.md)和[问题账本](./finding-ledger.md)。

## 无重叠覆盖表

下表的模块是 `echo_agent::` 后第一段；D02 的 subagent 和 D12 的工具子模块采用更具体
规则，优先从 D01、D11 移出。规则只适用于上述 deferred canonical 集合。

| 组 | 有产品意义的能力 | 精确选择规则 | 当前项数（遥测） | 当前处置 |
|---|---|---|---:|---|
| D01 | Agent 组装、模型请求、会话执行驱动 | `agent`（排除 `agent::subagent`）、`llm`、`config`、`profiles`、`headless`、`intent`、`runtime`、`prelude` | 291 | 保留 Rust/Host；可移植配置/helper 继续延期 |
| D02 | 任务、Subagent 协作与定时触发 | `tasks`、`scheduler`、`agent::subagent` | 126 | 保留 Rust/Host；新增调度/触发合同继续延期 |
| D03 | 记忆检索、provenance 与演化 | `memory`、`evolution` | 89 | 保留既有 Store 路径；演化控制继续延期 |
| D04 | 上下文组装、压缩与 token 预算 | `compression`、`context`、`tokenizer` | 79 | 保留 Rust/Host；压缩策略控制继续延期 |
| D05 | Workflow 构建、执行与断点 | `workflow` | 115 | 保留已接受 graph 路径；新 DSL/构建控制继续延期 |
| D06 | checkpoint、journal 与投递结算 | `state`、`delivery` | 90 | 保留 Rust/Host 权威 |
| D07 | Skill、Plugin、Hook 部署与项目规则 | `skills`、`plugin`、`hooks`、`skill_telemetry`、`project_rules` | 131 | 保留现有桥和规则读取；部署管理继续延期 |
| D08 | 人工决策、工具权限、Guard 与审计 | `human_loop`、`guard`、`audit` | 84 | 保留 Rust/Host；新的决策-provider 管理继续延期 |
| D09 | MCP、LSP、A2A 与拓扑集成 | `mcp`、`lsp`、`a2a`、`topology` | 57 | 保留现有 manager/client 路径；transport/server 配置继续延期 |
| D10 | IM channel 会话、消息与协议适配 | `channels` | 58 | 保留现有 channel 桥；会话轮转/附件策略继续延期 |
| D11 | 工具调用上下文、效果、权限与资源控制 | `tools`（排除 D12 子模块） | 132 | 保留 Rust/Host；语言局部值 helper 继续延期 |
| D12 | 内置文件、数据、媒体与外部工具实例 | `tools::{builtin,data,data_quality,files,git,lsp,media,rag,web}` | 117 | 保留 Rust/Host 工具；新增 backend/provider 构造合同继续延期 |
| D13 | Trace、评估、改进、遥测与错误诊断 | `eval`、`improve`、`trace`、`telemetry`、`retry`、`error` | 48 | 保留已接受诊断路径；新诊断/helper 合同继续延期 |
| D14 | 本地路径、sandbox 与文件身份 helper | `sandbox`、`paths`、`utils` | 31 | 保留 Rust/Host；可移植路径/helper 继续延期 |
| 合计 | 14 个能力组 | 41 个顶层模块，所有 deferred canonical 恰好归入一组 | 1,448 | 不新增 accepted 合同 |

## 每组的行为、边界与接受条件

以下 identity 是 manifest 可直接检索的实例；framework 源路径均相对原该inventory观察版本的仓库根，
不是当前 framework main 的推测。`value:*` 代表接受的值合同，不能当成可调用 RPC。

### D01：Agent 组装、模型请求与执行驱动

实例：`echo_agent::agent::ReactAgent`、`AgentBuilder`、`AgentConfig`、
`echo_agent::llm::ChatRequest::with_timeouts`、`echo_agent::runtime::AgentTurnDriver`，以及
`echo_agent::prelude::PromptTemplateManager::render`。源码分别见 framework
`src/agent/react/mod.rs`、`src/agent/react/builder.rs`、`echo-orchestration/src/runtime/`、`src/llm.rs`；
manifest 的 `source_paths` 区分 root/prelude facade 与真正定义。

已有 Host 的 `agent/create`、Session、Run 入口在
[`handler.rs`](../../../echo-sdk-host/src/core_profile/handler.rs)，factory 在
[`factory.rs`](../../../echo-sdk-host/src/factory.rs)；LlmClient/Agent/component 反向调用在
[`extension_bridge.rs`](../../../echo-sdk-host/src/core_profile/extension_bridge.rs)。
`ChatRequest` 的 `messages/timeouts/tools` 已有 accepted value 字段，而类型/helper
自身可以仍是 deferred。Rust builder 注入 trait object、pipeline stage 和 cancellation token，
没有一份逐 builder 方法公开的 accepted 创建合同。流水线 stage 不应成为语言端第二执行器。

保留这些底层实例和驱动在 Rust/Host。若接受新的 Agent 配置、prompt-template 或 profile
helper，需定义参数字段、未知字段/能力错误、默认行为和与 Session factory 的作用域；
三语言验证 Unicode 模板、缺值、非法 model/config、创建失败回滚和 close。模型、终态与
取消仍调用 framework，不能把 Rust builder API 逐项翻译成 SDK 公共状态机。

### D02：任务、Subagent 与定时触发

实例：`echo_agent::tasks::TaskSpec::stable_hash`、`TaskSubagentContext`、`TaskClaim`、
`echo_agent::agent::subagent::SubagentBuilder`、`SubagentMessageReceipt`、
`echo_agent::scheduler::CronTask::next_run_after`。定义见 framework
`echo-orchestration/src/tasks/`、`echo-orchestration/src/scheduler/cron_task.rs` 和
`src/agent/subagent/builder.rs`。

[`task.rs`](../../../echo-sdk-host/src/core_profile/facade/task.rs)、
[`task_runtime.rs`](../../../echo-sdk-host/src/core_profile/facade/task_runtime.rs) 和
[`subagent.rs`](../../../echo-sdk-host/src/core_profile/facade/subagent.rs) 已组合 framework
Session services、revisioned graph、claim 与 Subagent executor。accepted `value:task`
包含 `TaskSpec` 的 `depends_on/max_retries/extension`，不自动接受 `stable_hash` 算法或
任意本地 builder/context 构造。Cron helper 未形成 SDK 定时任务注册合同。

现有任务控制继续沿唯一 TaskRun graph；底层 claim、attempt 和上下文保留 Rust/Host。
新增远程协作策略或定时触发需接受 owner、revision、timezone、错过触发、重试及关机/重启
处置的合同；三语言真实 Host 验证 stale revision、重复触发、Session close、取消与 claim
settlement。不得新建平行任务 CRUD、DAG validator 或语言调度器。

### D03：记忆与演化

实例：`echo_agent::memory::MemoryMeta::is_hot_eligible`、`MemoryFilter`、`TypedMemoryEntry`、
`echo_agent::evolution::MemoryRuntimeIntegrationBuilder`、`SecretScanner`、`SkillPatch`。
源码见 framework `echo-core/src/memory/` 与 `src/evolution/runtime_integration.rs`。

已有 [`memory.rs`](../../../echo-sdk-host/src/core_profile/facade/memory.rs) 复用 Session
Store；[`source_operations.rs`](../../../echo-sdk-host/src/core_profile/facade/source_operations.rs)
保存具体 InMemory/File/Sqlite backend authority。Store 扩展桥也已接受。`MemoryMeta` 的
provenance/风险/status 等字段属于 accepted `value:memory`，其 eligibility/helper 方法
以及演化 runtime 组装不因此成为语言端策略承诺。

保留 Store commit、Draft/Active 资格和演化副作用权威在 framework。新增演化控制需定义
proposal/activation、revision、扫描器输入输出及写入失败/撤销合同；三语言验证 stale proposal、
回调错误、Store outcome unknown、时间和 Unicode 无损。不得以 helper 的默认值替代服务判定。

### D04：上下文、压缩与 token 预算

实例：`echo_agent::compression::compressor::HybridCompressor`、`CompressionCheckpoint`、
`StructuredSummary`、`echo_agent::context::ContextAssembler::assemble`、
`echo_agent::tokenizer::HeuristicTokenizer`。源码见 framework
`echo-state/src/compression/compressor/hybrid.rs`、`src/context/`、`src/tokenizer.rs`。

memory family 的 source prefixes 包含 `echo_core::compression`，但这不接受所有 compressor
构造方法；`ContextBudget/ContextSources` 部分字段已有 value 合同。
Host source-operation adapter 和 component bridge 提供组合入口，没有接受整套压缩策略
控制面。阈值、模型 tokenizer 和 checkpoint 的语义仍由 framework 决定。

保留压缩器实现。新增可移植预算/helper 或压缩配置需明确计数单位、模型依赖、checkpoint
身份、保留窗口、失败后的原文/状态；三语言验证中文/emoji、预算边界、摘要失败、取消、
checkpoint 恢复和不同模型的可观测结果。不得复制压缩/恢复状态权威。

### D05：Workflow

实例：`echo_agent::workflow::WorkflowDefinition::build_graph_with_client`、`GraphBuilder`、
`ConcurrentWorkflowBuilder`、`dsl::StateGraph`、`SharedState`。定义见 framework
`src/workflow/loader.rs` 和 `echo-orchestration/src/workflow/`。

[`workflow.rs`](../../../echo-sdk-host/src/core_profile/facade/workflow.rs) 持有真正 framework
Graph/SharedState，accepted catalog 已包含 build/run/stream、interrupt、resume_exact、branch
和 checkpoint 操作。`WorkflowDefinition` 的 nodes/edges 等字段已经 accepted，Rust DSL、
trait-bound builder 以及 JSON/YAML 编译 helper 仍无独立 accepted 方法合同。

保留 graph 执行与恢复权威。新增 DSL 或 pipeline 配置需接受节点/边错误、未知配置字段、
文件来源、checkpoint provenance 和中断控制语义；三语言验证非法图、循环、并发节点失败、
取消、跨 graph restore 错配和流终态。不得在语言端或 adapter 再建 graph 引擎。

### D06：checkpoint、journal、投递结算

实例：`echo_agent::state::AgentCheckpoint::restore_runtime_payload`、
`state::journal::PreparedJournalBatch`、`JournalBatchPrepareError`、
`echo_agent::delivery::DeliveryLedgerProjection`、`DeliverySettlement`。定义见 framework
`echo-state/src/journal/mod.rs`、`echo-state/src/delivery.rs` 与 `src/state/`。

[`state_delivery.rs`](../../../echo-sdk-host/src/core_profile/facade/state_delivery.rs) 已有
checkpoint 和 ledger 操作；Host 的 [`events.rs`](../../../echo-sdk-host/src/core_profile/events.rs)
与 [`persistence.rs`](../../../echo-sdk-host/src/core_profile/persistence.rs) 组合事件交付和恢复。
accepted `value:state` 已包含 `JournalIdentity` 和 checkpoint 字段；本地 prepared batch、
restore helper 和结算构造器并不是额外客户端 commit/replay API。

保留 journal append、commit、ledger transition 和 terminal 在 framework。若增加诊断性
snapshot/恢复操作，需接受 journal identity、watermark、revision、部分写入及 outcome unknown
合同；三语言验证重启、换代、stale handle、gap ACK、错误恢复状态与 terminal 顺序。
客户端不能用本地 checkpoint helper 推断恢复成功或补造已覆盖事件。

### D07：Skill、Plugin、Hook 与项目规则

实例：`echo_agent::skills::SkillRegistry`、`skills::external::ActivateSkillTool`、
`skills::external::validate::validate_skill_dir`、`echo_agent::plugin::PluginWiringResult`、
`echo_agent::hooks::HookResult`、`echo_agent::project_rules::inject_rules_with_root`。
定义见 framework `echo-execution/src/skills/registry.rs`、`src/plugin.rs`、`echo-core/src/hooks/`、
`echo-core/src/project_rules.rs`。

已有 Hook/Agent component 桥与 tool 执行路径；project_rules family 接受 resolve/load。
完整 Skill/Plugin 的安装发现、目录校验、激活、权限合并、script 环境和撤销管理没有作为
一个跨语言部署产品合同接受。HookResult 已有部分 value 字段，不意味着客户端拥有注入顺序。

保留 framework 注册/激活与 prompt 注入权威。若新增部署控制需接受目录身份、scope、manifest
诊断、激活/撤销、脚本环境及失败回滚；三语言验证重复注册、损坏 manifest、中文路径、Hook
取消/重入、撤销失败和 Session 所捕获旧 proxy 的处置。插件/技能管理不扩散为第二 registry。

### D08：人工决策、权限、Guard 与审计

实例：`echo_agent::human_loop::DefaultPermissionRequestHandler`、`HumanLoopRequest`、
`PermissionAuditEntry`、`echo_agent::guard::core::rule::RuleGuardBuilder`、
`echo_agent::audit::AuditCallback`。定义见 framework
`echo-orchestration/src/human_loop/permission.rs`、`echo-core/src/guard/`、`src/audit.rs`。

已有 [`permission.rs`](../../../echo-sdk-host/src/core_profile/facade/permission.rs)、
content_guard family 和 HumanLoop/Guard/Callback 桥。服务对象本地构造和 provider 事件派发
helper 没有成为额外语言 API；不能据此撤销已接受权限操作或人为给用户交互加门控。

保留 effect 判断、审批缓存与审计顺序在 framework。新增 provider 管理需接受 request identity、
deadline、late decision、withdraw、重入和关联 effect；三语言验证超时、取消、重复决策、
断连和回调失败。保留 EKO 本地个人工具的产品边界，不引入线上多租户权限模型。

### D09：外部集成

实例：`echo_agent::mcp::integration::server::McpServerBuilder`、
`mcp::integration::transport::stdio::StdioTransport`、`echo_agent::lsp::StdioLspClient`、
`echo_agent::a2a::A2AClient`、`echo_agent::topology::TopologyNode`。
定义见 framework `echo-integration/src/mcp/server.rs`、`echo-integration/src/mcp/transport/`、
`src/lsp.rs`、`src/a2a/`、`src/topology.rs`。

[`integrations.rs`](../../../echo-sdk-host/src/core_profile/facade/integrations.rs) 已组合
MCP/LSP manager、A2A client 和 topology tracker。accepted catalog 已覆盖 connect/read/tool、
LSP start/stop/restart、A2A task/stream 与 topology snapshot。底层 transport/server 的 Rust
构造器、JwtClaims helper 和配置解析方法没有逐项 accepted 方法合同。

保留现有 client/manager 能力，不误称整个集成延期。新增 transport/provider/server 控制需接受
配置、凭证处理、owner/generation、发布回滚、清理和远端错误；三语言验证连接中断、超时、
资源限额、wrong owner、stream terminal 和重复 close。密钥不进入日志。

### D10：Channel 会话与消息

实例：`echo_agent::channels::AgentChannelHandler`、`ChannelSessionRotation`、
`InboundMessage`、`MessageAttachment`、`integration::channels::feishu::proto::ProtoFrame`。
定义见 framework `src/channels.rs` 和 `echo-integration/src/channels/`。

accepted channels family 已提供 manager/start/stop/send，桥接受 channel plugin/message
handler。底层会话轮转、附件解析、Feishu websocket/proto 配置没有变成跨语言 IM 协议合同；
第三方 protocol 字段不应逐项变成语言客户端能力。

保留 transport 和 AgentChannelHandler 在 Rust/Host。新增消息/附件或轮转策略需定义稳定
语言中立 message DTO、Session 归属、轮转原因与 delivery receipt；三语言验证重复消息、
附件失败、多 Session、发送结果不确定、断线重连和停止/回调竞态。

### D11：工具调用与效果控制

实例：`echo_agent::tools::InvocationResourceGuard`、`ToolContext`、`ExternalRunContext`、
`ToolCapabilities`、`ToolFailure`、`cell::CommandCellObservationLease`。
定义见 framework `echo-core/src/tools/mod.rs` 与其 `tools/` 子模块。

已有 Tool/Agent 桥和 [`tools.rs`](../../../echo-sdk-host/src/core_profile/facade/tools.rs)；
accepted value、handle 和桥合同覆盖特定工具数据。Invocation resource guard、CommandCell
lease、uplink 和 capability 组装仍是本地执行机制，语言客户端不能直接成为 lease/效果权威。

保留这些机制在 framework/Host。新增工具 context/helper 需先区分可序列化事实与进程局部
authority，并接受 effect、身份、权限、artifact 和 failure 字段；三语言验证取消、资源释放、
重复结果、stale invocation、回调异常和 effect 承诺无损。不得把 outcome unknown 填成成功。

### D12：具体工具与 provider 实例

实例：`echo_agent::tools::files::files::ReadFileTool`、`files::apply_patch::ApplyPatchTool`、
`media::excel::ExcelWriteTool`、`rag::RagStore`、`web::providers::brave::BraveSearchProvider`。
定义见 framework `echo-tools/src/files/files.rs`、`echo-tools/src/media/`、`echo-tools/src/rag.rs`、
`echo-tools/src/web/`。

Host tools family 已有 read/write/patch、data/media/RAG/web/git 等工具操作。具体 Rust
实例的构造、新 backend 和 provider helper 没有 accepted SDK 创建方法，工具已经可执行
并不承诺其内部实例可以在三语言逐方法控制。D12 不包含 D11 的调用上下文/lease。

保留现有工具实现。新增 backend/provider 配置需明确创建参数、secret handling、文件路径
语义、副作用和资源生命周期；三语言真实 Host 验证 Unicode/非 UTF-8 路径、非法输入、
部分写入、超大结果、取消与失败副作用。工具构造器不能成为绕过统一 Tool 执行的第二入口。

### D13：诊断、质量与遥测

实例：`echo_agent::trace::Run`、`TokenUsage`、`echo_agent::eval::EvalResult`、
`echo_agent::improve::RunCritique`、`echo_agent::telemetry::Metrics`、
`echo_agent::retry::core::with_retry_if`、`echo_agent::error::AgentFailure::message#a25634796038`。
定义见 framework `src/trace/mod.rs`、`src/eval/`、`src/improve/`、`src/telemetry.rs` 与
`echo-core/src/error.rs`。

Host 的 observability、telemetry 和 trace store 已有 accepted 操作与 value；本地诊断
builder、格式化 helper 及 retry closure 未接受。manifest 签名后缀用于消歧，不能删掉后缀
将不同重载当同一个客户端方法。

保留通用重试和计数器权威。新增导出/分析/helper 需接受输入输出、单位、错误/trace 保真和
factory cleanup；三语言验证溢出、deadline、Unicode、缺字段、失败样本与资源关闭。
遥测 snapshot 不反推 Run terminal 或覆盖 journal。

### D14：本地路径、sandbox 与文件身份

实例：`echo_agent::sandbox::ExecutionResult`、`default_language_image`、
`echo_agent::paths::DataRoot`、`echo_agent::utils::fs::encode_path_segment_identity`、
`echo_agent::utils::canonical_json::canonical_json_bytes`。定义见 framework
`echo-core/src/sandbox.rs`、`src/paths.rs`、`echo-core/src/utils/`。

Host source adapter 保存真正文件 lease/descriptor；sandbox 扩展流与 accepted scalar path/bytes
合同已存在。原生路径身份、默认 image 和 canonical JSON helper 没有逐函数 accepted 合同，
不能把客户端自行编码结果当作 Host 文件身份。

保留本地文件/sandbox 权威。新增可移植 helper 或 execution 配置需接受 platform、native
bytes、编码、identity 与 exec failure/cancel 语义；三语言验证中文/emoji、非 UTF-8 路径、
Windows native path、无效编码、重复文件地址、取消和退出码。不能静默规范化身份或丢 byte。

## ADR 0031/0032 与当前接受范围对账

[ADR 0031](../../adr/0031-sdk-identity-governance-scope.md) 关于“按能力与 Finding 治理、库存仅
遥测”的决策仍有效。其 `9,682`、`5,606`、`4,076` 是历史快照，不能用来判断当前范围；
“剩余 intrinsic mapping backlog”也不能覆盖后来已接受的 native value/helper 群。

[ADR 0032](../../adr/0032-sdk-contract-scope-classification.md) 明确五种 scope、alias 继承和
接受判定先于语言状态。旧生成epoch canonical实测为 external 5,622、Host/Rust 1,774、language
intrinsic 790、internal helper 90、deferred 1,448，总计 9,724。这里只把它们作为观察数据，
不恢复 ADR 中“精确库存计数作为门禁”的旧表述：当前 source-contract 的阻断输入限于 accepted
external contract/catalog，完整 inventory、framework revision 和 Host-only catalog 是遥测或
构建 provenance。正式范围以这些当前生成物及 ADR 0001 为准。

具体避免三类误读：

1. `MemoryMeta`/`TaskSpec`/`AgentCheckpoint`/`WorkflowDefinition` 的某些字段已 accepted，
   类型或 helper deferred 不得被解释成这些值不属于合同。
2. memory/workflow/MCP/LSP/Task/Channel 的现有 Host 操作已 accepted，底层构造器 deferred
   不得被解释成该能力族全部延期。
3. accepted intrinsic 能力群以稳定具名合同接受，不从 `done` 推导。新的 helper、配置或
   provider 即使源码能直接调用，也必须先接受行为合同再生成语言 API。

本页不改变 ADR 决策、不改变 accepted scope，也没有兼容性迁移或发版副作用。

## 将来真正需要产品选择的分支

以下是将来改变范围时需要选择的新增产品方向，不是本轮既有治理/延期的前置确认门禁：

- 接受 portable helper/config，或继续让语言客户端组合现有值合同，例如模板、预算、时间与
  profile helper。需选择精确算法/默认值是否构成跨语言稳定承诺。
- 接受 Host 资源控制面，或继续通过现有 family/component 桥，例如 compressor、Skill/Plugin
  部署、provider/transport 配置。需选择新配置/handle/lifecycle 合同范围。
- 接受新运行时能力，例如定时触发、演化控制、Channel 会话策略。需选择用户可见行为和持久化
  边界；framework 仍是唯一执行/状态权威，不复制 runtime。

没有这些新增选择，默认延续本页处置；不逐 identity 要求确认。已经接受能力中的字段丢失、
取消/恢复错误、清理缺口与语言差异是当前合同修复，不能以 deferred 为理由延期。

一项能力被接受前需要同一切片同时具备：具名行为与失败合同、唯一 framework 权威、薄且无损
adapter、owner/generation/close 或纯值算法约束、协议 fixture、三语言相同场景真实 Host
证据（纯值 helper 为同一输入/输出反例）、文档/示例/CI 和独立复审。只出现 catalog `done`
或 handler 可达不满足此条件。

## 可复核的完整集合检查

在 SDK 仓库根运行以下只读脚本。它不新增 schema，不改生成物；强制每项恰好一个归属，
完整集合与 accepted 身份互斥。输出数量随库存更新，数量变化只提示更新处置模型。

```python
import collections
import json
from pathlib import Path

manifest = json.loads(Path("contracts/sdk/parity-manifest.json").read_text())
accepted = json.loads(Path("contracts/sdk/accepted-external-contract.json").read_text())
modules = {
    "D01": {"agent", "llm", "config", "profiles", "headless", "intent", "runtime", "prelude"},
    "D02": {"tasks", "scheduler"},
    "D03": {"memory", "evolution"},
    "D04": {"compression", "context", "tokenizer"},
    "D05": {"workflow"},
    "D06": {"state", "delivery"},
    "D07": {"skills", "plugin", "hooks", "skill_telemetry", "project_rules"},
    "D08": {"human_loop", "guard", "audit"},
    "D09": {"mcp", "lsp", "a2a", "topology"},
    "D10": {"channels"},
    "D11": {"tools"},
    "D12": set(),
    "D13": {"eval", "improve", "trace", "telemetry", "retry", "error"},
    "D14": {"sandbox", "paths", "utils"},
}
tool_leaves = {"builtin", "data", "data_quality", "files", "git", "lsp", "media", "rag", "web"}
deferred = [e for e in manifest["entries"] if e["canonical"] and e["sdk_scope"] == "deferred"]
accepted_paths = {e["path"] for e in accepted["entries"]}
seen = set()
counts = collections.Counter()
surfaces = collections.Counter()
for entry in deferred:
    path = entry["path"]
    parts = path.split("::")
    hits = [group for group, names in modules.items() if parts[1] in names]
    if parts[1] == "agent" and parts[2:3] == ["subagent"]:
        hits = ["D02"]
    if parts[1] == "tools" and len(parts) > 2 and parts[2] in tool_leaves:
        hits = ["D12"]
    if len(hits) != 1 or path in seen or path in accepted_paths:
        raise ValueError((path, hits, "missing, overlapping or conflicting disposition"))
    # A projected route does not accept this deferred identity into a language API.
    surfaces.update([entry["route"]["surface"]])
    seen.add(path)
    counts.update(hits)
if len(seen) != len(deferred):
    raise ValueError("incomplete canonical deferred coverage")
print(dict(sorted(counts.items())))
print("route surfaces", dict(sorted(surfaces.items())))
print("covered", len(seen), "canonical deferred identities; no overlap")
```

历史静态集合重核结果（见原evidence/sdk-deferred-product-disposition/receipt.json及逐identity group-membership.json）：D01–D14 的数量与旧覆盖表一致，1,448 项完整且互斥，未修改合同、生成器、
SDK API 或源码。本次文档分类不需要 examples/website 行为同步；没有新增对外能力。
这份集合证据只证明处置覆盖，不证明各能力的生命周期或异常路径已经验证。

## 当前后继与产品处置结论

14组不只列计数：各组都有行为、framework唯一权威、已有accepted主能力、Rust/Host保留对象、继续延期的新增产品方向和明确接受条件。逐identity实例/source_paths及互斥选择现在保存于[evidence/sdk-deferred-product-disposition/group-membership.json](https://github.com/EchoYue-lp/echo-agent-sdk/blob/0ebe15f66d0aa57c7b7a92ae68cd1be106114ae9/docs/sdk/governance/evidence/sdk-deferred-product-disposition/group-membership.json)，源inputhash与旧frameworkepoch在receipt，能从每一条回到唯一生成物；不是第二分类权威。没有未分组或accepted交叉项。最终sourcegen更新时只能重算这套集合，不能手改scope。

D10不延期原MessageHandler义务：用户已批准ADR0035 handle/handle_stream/reply与必要next/cancel/close，implementation与三语言后继验证进行中，Finding至独立复审/最终生成闭合前保open。新wrapper/internalreceipt不自动接受所有Rustpublicidentity。现有1,448条旧库存中可能与新明确接受发生变动，必须由最终sourcegen正式承接。

三项managed逆向DTO边界见[managed-supporting-operation-boundary.md](managed-supporting-operation-boundary.md)。保留现reverse DTO/adapter和Rust稳定请求删除机制，当前未接受新的forward stabledelete；无需为补测试强加SDK API，也不退役原DTO/既有delete/clear合同。

当前无需新的产品选择即可执行已有处置。portablehelper/provider部署/cron/evolution等明确未接受的新方向继续延期，原因是尚无用户承诺的算法、配置或持久化合同，而非“框架没有第二消费者”。Channel原承诺已经取得人的选择，不再重复询问。最终pin/生成、适用门禁/CI和精确main是工程交付条件，不是产品裁决。

## 数量的artifact名称与epoch

|数量|对应实际材料|用途|
|---|---|---|
|5,622|旧accepted-external-contract.json canonical阻断seed entries|明确接受范围的旧快照，不是完整Rust inventory|
|9,724|旧inventory-telemetry.json canonical_items|完整Rust canonical遥测，不是accepted seed|
|23,672|旧parity-manifest.json entries|含aliases/intrinsic等条目，与canonical计数不可混比|
|default23,552 / full36,519|当前每profile原始publicitem extraction|尚未mergecanonical的原始量，不能与5,622比较完成度|

旧三份生成物仍绑定27c库存epoch；current新profile提取数字来自唯一generator writer实读，最终merge/acceptance待其成功生成。没有新增counts门禁、完成率或遥测接受权威。实际材料hash见[evidence/sdk-count-authority-calibration/artifact-facts.json](https://github.com/EchoYue-lp/echo-agent-sdk/blob/0ebe15f66d0aa57c7b7a92ae68cd1be106114ae9/docs/sdk/governance/evidence/sdk-count-authority-calibration/artifact-facts.json)。本次接受增量只为TaskExecution两个字段、DiagnosticCode enum与两个variant共5项，Curator save一项retire；supportingvalue3在第二wholeupdate待正式承接。没有由所有Rustwrapper自动接受API，也不手改JSON。
