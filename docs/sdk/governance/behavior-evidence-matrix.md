# SDK 能力行为与证据矩阵

当前验收与阻断状态统一见[当前有限验收](capability-review-remaining.md#当前有限验收2026-10-15)。12项源身份迁移已批准并由独立SDK切片执行；物理结算B及工具A/B已批准，由独立上游实施待交付。本页下列原阶段计数、未批准文字和失败事实仅保留历史切点，不表示当前仍等待同一裁决。

## 前一验收切点（历史原文，2026-10-15）

SDK Host 精确依赖已发布框架 `00782b8a`，八个框架包同一 Git 来源、没有 path patch；框架合并后七项 CI 已成功。SDK [14 条 Rust 门禁](evidence/sdk-00782-final-rust-gates/README.md)实际全0，workspace 主计数474，不累加重叠 feature profiles。[完整语言入口](evidence/sdk-language-00782-reviewed-final/README.md)实际退出0：TypeScript329、Python394加18子测试、Java351/75份XML及三个quickstart；793项执行输入前后相同，随后逐项保存为可恢复Git。预期负例的运行时WARN按原字节保留，编译告警为0。

当前按实际语义对象读取：52项Finding本地resolved、4项open。Git分支切换同名dirty覆盖与SDK worktree入口栈溢出已各自完成特定修复、消费者验证和独立复审链；本地闭合不宣SDK main或外部Issue已关闭。剩余为12项accepted source/kind迁移，以及cleanup/Sandbox/Task resume三个宽Finding共同关联的5秒物理结算B；只涉及两个待人的裁决单位，不自动退役合同或接受风险。

[完整合同只读检查](evidence/sdk-contract-00782-readonly/README.md)真实退出1，保留13项连续性错误：12项提案未批准，另1项已有独立批准的Curator保存项仍按原证据归因；本轮没有update/accept/retire或seed修改，未到达的后续命令不算通过。40格审查是五个in_scope区域的有界风险映射（38 covered、2根manifest not_applicable），不是41族全部verified或完整生成通过。库存建模已闭合（inventory_closure=closed），行为模型保持open，保留上述B与12项连续性边界。SDK `eebd9f952649cca20d20eabab7f88ecdbaea5813` 已实际通过本地官方全项目strict，包含严格快照及变更依据；该切点工作树与Git源码摘要同为 `473fb206…`，308份正式语义文件的路径、模式、内容与Git一致。这个结果只属于eebd切点；后继文稿或CI修改须按新树重新绑定摘要与验证，不能继承为未来结果。现有远端工程CI没有执行该语义工具，不作为远端语义通过证据；后续固定工具版本的独立语义job仍待实现和实跑。Windows job只检查Rust库，不证明Windows原生SDK消费者；精确SDK main仍须发布后手工官方strict复核。registry发布及CLI/官网外部写入不由本表授权。

具名审查范围见[能力处置](capability-review-dispositions.md)，三链及开放对象见[Finding账本](finding-ledger.md)。历史段中的“当前”“未运行”“open”只描述该段原观察，不作为当前待办；原日志、输入、sourcecut与限制不改写。

## 前一阶段文字与证据（历史，原正文保留）

以下“当前/待回归/open”均是原阶段文字；现在以前述状态和Finding对象为准，原日志/sourcecut/限制不改写。

本矩阵是逐能力审查及验收的工作账本，范围来自
`contracts/sdk/accepted-facade-operation-catalog.json` 的 **41 个 family**，与
[Capability Map](capability-map.md)、[Finding 账本](finding-ledger.md)配合使用。
它不把完整 Rust public inventory 转成跨语言承诺；未接受 identity 的处置见
[deferred 能力](deferred-capabilities.md)。family 是盘点索引，行为、权威与失败路径才是验收单位。

## 当前参照与历史证据等级

本轮客户端小里程碑已形成真实 Git：Java `59fde9d3`、TypeScript `cf6a042e`、Python `1ca513a1`，正式证据随后提交为 `76601b24`。三语言客户端八个具名风险视角均经实际独立源码审查记为有界 `examined`，C/I/M0；TS/Java 两项具体 stdio 关闭缺陷具完整修复、同源验证与具体复审链，已本地 resolved。当前 a634 的14条Rust门禁实际全0，依赖 audit/deny 和精确50组基线独审通过；官方完整语言同入口重跑退出0（TS308、PY375+18、Java332/71XML、3quickstarts），790项输入前后同。原退出1/98d714/TaskHTTP连接夹具证据保留，后继只证明受控Connection close机制，未声称原socket直接因果。完整生成、12项source/kind与5秒B选择、当前最终结构/平台/SDK main仍独立，不把客户端局部闭合扩为41族或八个全局视角verified。

此前已冻结治理参照为可恢复私有 Git `4febaba8b00851be63b256d1dce013fb566b3bb6`，
其当前框架 pin 为已发布 `be5f0a84`，没有本地 path patch。该切点的14条Rust门禁、完整官方
三语言入口（TS290、Python368+18子测试、Java320/67XML与3个quickstart）和全项目strict已实际通过。
这不是SDK main交付或全部业务verified：12项accepted source/kind迁移与5秒物理结算B仍待人选择。
本文件之后的任何新增验证必须有自己的来源，不能盲绑到4feb运行时刻。

SDK 已精确依赖框架 main `a6346643bda47679a96760f56f45e41fd73b61b0`，八个框架包同一 Git 来源、无本地 path patch，锁文件 SHA `a7a331cf`。框架 PR185 与合并后 main 的七项 CI 均实际成功，合并树与已审 `6d9be119` 树 `093a48f0` 完全相同，精确 main strict 已通过；验收收据为工作区 admin/framework-main-a634-acceptance.json（SHA `0d9b7097`）。上述原接线时14条仅7步通过、依赖独审待完成的记录是历史；现在14条及依赖复审已取得终态，当前以本节页首客户端里程碑为准。此前 be5/旧 Host 的14Rust、Chart61、TTL和完整语言结果按原证据切点保留，不冒充本次 a634 执行。\n\n历史起点SDK30d7、框架95ec及fc63候选的记录保留如下。其临时组合、二进制和旧计数只说明
当时具名断言，不能当当前pin或未修复状态；下列更新行按各正式后继证据的具体case对账。

**以下 41 项均仍为 `needs_review`。** 本次已核对冻结候选 Host 与三语言运行收据，
并将实际测试体中的断言逐 family 对账。下列等级只描述证据阶段，不提升语义对象完成状态：

- `source_checked`：入口、注册和调用链可读；不能证明运行结果。
- `assertion_present`：存在具体断言，但此能力仍无足够当前运行证据。
- `candidate_runtime_verified`：指定断言在记录的候选二进制/源码上实际成功；只证明所列
  输入、结果、错误或副作用。原早期证据使用临时组合；后继正式be5证据以自己的构建/运行清单为准，仍不等于SDK main验收。
- `runtime_verified`：最后相关修改后的正式 pin、完整场景证据、独立复审及精确 main
  验收；本矩阵没有此级的新声明。
- `missing`：没有足够行为断言或新运行证据，不用 catalog `done` 替代。

历史冻结 Host `fc63c0815b319424ea2c64a94d3fc6a365a3dff6b70882a23b9edd1266d2f6eb`
在 full-facade profile 下实际通过 library 136、Core 31、Bridge 34、Facade 12，全部零失败、
零 ignored；三语言同一 Host 的脚本等价命令和示例通过（TS 219、Python 279 加18 subtests、
Java 233，零 skip）。这些计数是运行来源，不是41项能力完成的分母。
逐项证据与hash见 `.echo-semantic/evidence/evidence.sdk-candidate-behavior-verification.md`。

新增局部证据分别归档为 `evidence.sdk-tool-behavior-candidate` 与
`evidence.sdk-testing-value-candidate`：工具场景29条记录实为28次tool调用+1次Session close，
10个typed RPC error之外另有database success=false拒写；testing为14已接受字段的纯值合同。
两组只提升具名证据支持，不提升41个Behavior的needs_review，不能沿用旧whole-source摘要
宣称新整仓运行全绿。工具reported effects empty与实际文件/ref副作用分别检查；GET-only
fixture记录不证明无POST，非法分支未比较repo A全部refs。


每项“最低充分证据”是可执行场景的验收要求，不是增加 accepted API 的授权。测试可以
组合运行，但各 family 的观察结果必须可分辨。不能用成功请求总数代替逐项断言。

## 共同入口与共同不变量

标准 ACP：`echo-sdk-host/src/lib.rs::run_stdio` → `factory.rs` → framework
`AcpAgentAdapter`，在 official ACP connection 上处理标准方法。扩展：
`echo-sdk-host/src/core_profile/mod.rs::attach` → `handler.rs` 或
`facade/mod.rs` → `facade/registry.rs::CompiledOperationCatalog` → concrete adapter。
动态注册再消费的链路包括 extension descriptor、factory instance、callback、source
exact operation、derived resource 和 stream pull；定义存在或注册成功不能证明真实消费。

除另有明确说明，family/op 经 `facade/mod.rs` 的 accepted operation/signature/feature
admission、Session receiver 和 owner/generation 检查后进入下面的具体分支。source route
经 `_echo_agent/facade/invoke` 的 exact route 转换进入同一服务，不另起任务调度器。
`echo-sdk-host/src/features.rs` 的编译能力与 hello advertisement 必须一致。

SDK Host 只拥有寻址、handle generation/owner、发送窗口、callback mailbox 和资源结算；
framework 业务实例拥有运行、DAG、取消、checkpoint/commit 和 terminal。并非所有对象
都是持久状态：tool 调用的文件/进程/远端副作用有自己的事实源，telemetry 是进程级，
测试值和 helper 不存在远端执行器。矩阵按这些实际差异限定“唯一权威”。

每个可执行 family 的共同反例至少检查：未知操作、错误 signature、未编译 feature、
wrong kind/owner/generation、参数类型/数量不符、Session/Agent close 竞态；不允许静默
默认、跨 Session 变更或 publication 失败遗留无 owner 资源。协议错误应保留 code、
retryability、operation、handle 和 detail；框架工具的 `success=false` 结果与 RPC error
是不同结果，三语言必须同样区分。预期具体错误码以 accepted DTO 和实际路径为准，
不以“返回了某个错误”作为通过条件。

## 证据入口索引

以下短名定位实际测试函数体；是否执行及支持什么结论以下方候选行与冻结收据为准。

| 短名 | 文件与主要范围 |
|---|---|
| Core | `echo-sdk-host/tests/core_profile_e2e.rs`：core lifecycle、family/source adapters、恢复 |
| Facade | `echo-sdk-host/tests/facade_feature_adapters_e2e.rs`：quota、stream、A2A、Task、cascade |
| Bridge | `echo-sdk-host/tests/extension_bridge_e2e.rs`：真实 Host 反向调用与 managed persistence |
| Stdio | `echo-sdk-host/tests/stdio_e2e.rs`：标准 ACP prompt/cancel、EOF、配置失败 |
| Protocol | `echo-sdk-protocol/tests/core_rpc_contract.rs`、`extension_contract.rs`、`facade_inventory.rs`、`acp_baseline.rs`：schema/值/声明；需检查每个 test 的具体断言 |
| Wire | `echo-sdk-host/src/core_profile/wire.rs`：DTO 字段转换 unit assertions |
| Languages | `scripts/check-language-sdks.sh`：三语言编译、示例和测试入口；脚本 exit 不能替代行为 scope |

Core `every_canonical_source_operation_reaches_a_host_adapter` 对每个 accepted canonical
operation 发送 `arguments=[null]`，仅把特定消息
`source operation is canonical but has no Host authority adapter` 加入缺失列表。这是
adapter 覆盖哨兵；其它非法参数错误也能通过，不能证明合法输入结果或副作用。
Protocol inventory/catalog 测试同样不能证明 Host 生命周期或语言消费语义。

## 核心运行、值和桥

### family: agent_lifecycle

- **入口/可达**：typed `agent/create|describe|close` → `handler.rs` → `factory.rs`、
  `handles.rs`；prepared definition 被真实 Session factory 消费，不能只看 create。
- **权威/状态**：framework Session Agent/connection services 决定运行；Host Agent handle
  只定位 definition 并拥有子 Session/resource owner。create → published → closing → released；
  关闭失败须保留可重试 owner，不能提前删掉活运行。
- **正常/失败**：配置与 idempotency 一致时复用规定对象；冲突、quota、Session 创建中途失败、
  Agent close 与 start 并发、重复 close/EOF 必须显式结算。无关 Agent 不被 cascade。
- **已有断言**：Core `valid_hello_completes_full_core_lifecycle`；Facade
  `agent_cascade_preserves_same_named_other_manager_and_releases_resources` 有 same-name
  隔离及 quota reuse 断言；Languages protocol-host 有 Agent close transport 错误后重试。
- **最低充分证据**：真实 Host 两 Agent/多 Session，阻塞 Run 与 MCP/stream 下 close，
  注入底层 close 失败后重试；断言旧 owner 资源不可用、其它 Agent 继续可用、进程/slot
  结算及 idempotency 冲突无额外创建。代理合成 close 错误不证明 Host 自身 cleanup。

- **候选运行证据（candidate_runtime_verified）**：真实 create/describe/close 和重复关闭、Agent cascade 后资源配额复用及两个同名独立 MCP manager 隔离；活 Run 的同一连接 actor 保持到清理与规范 receipt 结算。
  来源：`echo-sdk-host/tests/core_profile_e2e.rs#valid_hello_completes_full_core_lifecycle`；`echo-sdk-host/tests/facade_feature_adapters_e2e.rs#agent_cascade_preserves_same_named_other_manager_and_releases_resources`；`echo-sdk-host/src/lib.rs#manual_close_keeps_active_run_actor_until_cleanup_and_receipt_settle`。
- **仍缺或证据边界**：多 Agent 创建/关闭并发及全部资源种类 publication fault 尚无逐种语言场景。 未覆盖的已接受行为须具名登记输入、结果/失败与缺证原因；不增加未定义的任意组合验收门槛。

### family: session

- **入口/可达**：typed `session/create|load|close` → `handler.rs`、`factory.rs`；标准 ACP
  Session 与扩展 handle 通过同一 connection services 桥接，见 Core standard-prompt 测试。
- **权威/状态**：framework live Session Agent、conversation/checkpoint store；Host Session
  handle/recovery mapping。create/load → live → closing → cleanup settled → release；重启新 generation。
- **正常/失败**：cwd、history、memory/task/services 绑定准确 Session；未知/跨 owner Session、
  load 损坏/缺失持久化、create publication 失败、close 超时/重试/并发不能返回假成功。
- **已有断言**：Core `extended_standard_prompt_bridges_shared_core_handles` 与 restart
  tests；三语言 protocol-host 两 Session ID 不同，关闭 one 后 two 的 Agent name 调用成功。
- **最低充分证据**：相同 Agent 下两个 Session 写 memory/执行 Task、关闭其中一个、重启
  load history；比对隔离状态、旧 generation rejection、close debt retry 和资源配额复用。
  Agent name 成功只证明 sibling Session 仍可访问，不证明所有核心状态隔离。

- **候选运行证据（candidate_runtime_verified）**：标准 prompt 与扩展 Session 共享对象；历史 native turn7 后 create/load 都精确分配 turn8；畸形身份报 create/load 实际 operation，重复失败无句柄配额泄漏。禁用 reservation 的真实反例两路径均产生 turn1。
  来源：`echo-sdk-host/tests/core_profile_e2e.rs#extended_standard_prompt_bridges_shared_core_handles`；`echo-sdk-host/tests/core_profile_e2e.rs#explicit_session_create_reserves_persisted_turn_identity_before_new_run`；`echo-sdk-host/tests/core_profile_e2e.rs#session_load_reserves_persisted_turn_identity_before_new_run`；`echo-sdk-host/tests/core_profile_e2e.rs#malformed_recovered_identity_rejects_actual_method_without_handle_publication`。
- **仍缺或证据边界**：现有多Session/Task与Store场景支持具名隔离事实；持久化I/O fault或close debt剩余义务按具体已接受路径列账，不能以“穷尽所有组合”作为完成条件。 未覆盖的已接受行为须具名登记输入、结果/失败与缺证原因；不增加未定义的任意组合验收门槛。

### family: run

- **入口/可达**：typed `run/start|get|wait|steer|cancel` → `handler.rs` → framework turn
  services/`RunEntry`；source chat 路由必须复用该 Run，不直接构造第二执行循环。
- **权威/状态**：framework Run/turn receipt 决定 terminal；Host 映射与 wait projection。
  published → active → cancellation requested/steered → settled；wait timeout 不等于 Run 终态。
- **正常/失败**：start、steer、wait 与 cancel 保留 exact Run identity/receipt；重复 start、
  idempotency mismatch、cancel-vs-completion、disconnect/kill、持久化失败不能伪造 completed。
- **已有断言**：Core `killed_active_run_is_interrupted_never_completed`、
  `turn_receipt_source_operations_project_live_and_recovered` 检查 live/recovered receipt status；
  Bridge sandbox cancellation test 检查 cancelled terminal；语言 callback exception 检查 failed。
- **最低充分证据**：三语言同一 parked-model scenario 分别 cancel/timeout/steer、并发 wait、
  EOF 与进程 kill/restart；比对 terminal cause、receipt、事件顺序和可观察模型/工具副作用。

- **候选运行证据（candidate_runtime_verified）**：kill 后 recovered Run 是 Interrupted 而非 Completed；live/recovered receipt 与 delivery 字段保留；取消需等待 sandbox cleanup，并检查 Cancelled terminal 与 cleanup 次数。另一个三语言并行 Agent feed 场景核对唯一事件终态及 journal union，证明取消 A 不改变 B 自然终态；两场景分别支持这些事实。
  来源：`echo-sdk-host/tests/core_profile_e2e.rs#killed_active_run_is_interrupted_never_completed`；`echo-sdk-host/tests/core_profile_e2e.rs#turn_receipt_source_operations_project_live_and_recovered`；`echo-sdk-host/tests/extension_bridge_e2e.rs#sandbox_cancel_bridge_waits_for_cleanup_and_preserves_cancelled_terminal`。
- **仍缺或证据边界**：steer、并发 wait、取消与自然完成竞争的统一三语言矩阵仍缺。 未覆盖的已接受行为须具名登记输入、结果/失败与缺证原因；不增加未定义的任意组合验收门槛。

### family: event_replay

- **入口/可达**：framework committed observer → `core_profile/events.rs::StreamDelivery`；
  `handler.rs` 的 run replay/event ACK 和语言 event feed 消费该 stream。
- **权威/状态**：framework `EventJournal` 拥有 identity/generation/committed sequence；
  Host cursor/window 仅管理 delivery。commit → live/gap → replay 或 snapshot coverage → ACK
  → continuation；snapshot covered watermark 与 last-sent 不是同一个量。
- **正常/失败**：#120 gap ACK 后只发送 coverage 后的事件；page cursor ACK、重复/未来/旧
  ACK、retention gap、oversized event、send failure、journal replacement、terminal ordering。
- **已有断言**：Core `ack_window_snapshot_ack_resumes_strictly_after_watermark`、
  `consumed_single_replay_page_ack_resumes_live_after_page_cursor`、multiple-page sibling；
  `sdks/shared/testing/gap_ack_fixture.py` 为真实 Host count/oversized gap 提供 parked/release。
  page ACK、external ACK 与 terminal replay ACK 的候选绿色见下方精确场景；最终 pin 验收仍待。
- **最低充分证据**：三语言 snapshot ACK、single/multiple replay-page ACK 同一场景，释放
  model 后断言严格 sequence > consumed watermark、无重复/漏失且 terminal 来自 journal；
  加 journal identity 换代、future ACK rejection、restart replay 和 send failure 后恢复。

- **候选运行证据（candidate_runtime_verified）**：snapshot ACK 与 page cursor ACK 后只续发已覆盖水位之后事件；三语言 count/oversized、两窗口 external ACK、terminal replay ACK 自然 EOF 都实际执行，journal union/event_id 与唯一 terminal 对账；未来 ACK 不推进。另有真实 segmented journal 原生轮转/prune：追加1..6后retained_floor=6，replay返回gap水位5及cursor6；只允许ACK6，拒绝ACK7。replay响应enqueue失败和空future page均不扩大ACK coverage。
  来源：`echo-sdk-host/src/core_profile/events.rs#real_journal_retention_page_authorizes_only_its_returned_watermark`；`echo-sdk-host/src/core_profile/events.rs#replay_publication_failure_and_empty_future_page_grant_no_ack_coverage`；`echo-sdk-host/tests/core_profile_e2e.rs#ack_window_snapshot_ack_resumes_strictly_after_watermark`；`echo-sdk-host/tests/core_profile_e2e.rs#consumed_single_replay_page_ack_resumes_live_after_page_cursor`；`echo-sdk-host/tests/core_profile_e2e.rs#consumed_multiple_replay_page_ack_resumes_live_after_page_cursor`；`sdks/typescript/test/gap-ack-host.test.js`；`sdks/python/tests/test_gap_ack_host.py`；`sdks/java/src/test/java/com/echoagent/sdk/GapAckHostTest.java`。
- **仍缺或证据边界**：journal identity替换/错generation的明确故障场景，以及正式pin重启回放和已识别风险的独立复审仍待；原生prune及指定enqueue失败边界已有候选断言，不列为未验证。 未覆盖的已接受行为须具名登记输入、结果/失败与缺证原因；不增加未定义的任意组合验收门槛。

### family: task

- **入口/可达**：typed task CRUD/execute/control → `facade/task.rs`、`task_runtime.rs`；
  Session live `RevisionedTaskStore` 与 runtime service 被真正 execute/control 消费。
- **权威/状态**：framework `TaskRevisionService`、`RuntimeTaskService`、revisioned store
  及 claim；`FacadeTaskController` 只 dispatch/转换/settle。graph revision → exact claim →
  attempt → settle；pause/cancel 必须限制在对应 Task/attempt。
- **正常/失败**：依赖结果进入 successor；重复 execute、stale revision、active-claim patch、
  local pause/resume/cancel、close interruption 不改变 sibling claim 或丢失 result。
- **已有断言**：Core task authority/execute-control tests、
  `task_pause_is_exact_and_resume_reuses_canonical_dependency_result`、
  `task_cancel_is_exact_and_pending_sibling_continues`；Facade duplicate execute typed conflict。
  当前组合 framework/Host 已实际执行；仍不能提升为正式 pin/main 验收。
- **最低充分证据**：必要 framework successor 完成后跑上述场景及三语言并发 DAG；对
  snapshot revision、claim identity、sibling 状态、resume dependency result，以及已接受的
  DispatchRequest/AgentTaskInput、结果/receipt 字段和四项 getter 消费作字段级断言，证明 adapter 无
  frontier/重试/取消第二权威。旧三项 StructField 身份仍属12项提案，待批准；本修订不自动退约。

- **已证实（限具体case）**：三语言各四项正式共享DAG/control场景已执行并由最新官方完整入口承接：exact pause/cancel A、B/C继续、resume复用canonical B result/执行事实、stale CAS/duplicate execute、CRUD-only拒绝执行；TaskContext四项真实Host组合核原run_id/waiver/policy getter消费。
  证据：[Task/Subagent正式证据](task-subagent-evidence.md)、evidence.sdk-task-subagent-candidate、evidence.sdk-task-context-direct-getter-consumption及[最新语言门禁](evidence/sdk-language-be5-reviewed-final/README.md)。
- **仍缺/不承诺的边界**：TaskContext没有完整三字段专用reverseDTO，不声称直接把整个context交给CustomAgent；5秒强中止后的物理结算B仍开放。原四场景绿色不关闭该义务，12项身份迁移仍待选择。

### family: subagent

- **入口/可达**：typed dispatch/await/control → `facade/subagent.rs`，由 framework
  `SubagentExecutor` 建立真实 attempt；extension custom agent/factory 也是其 adapter。
- **权威/状态**：framework Subagent run/result/attempt receipt；Host handle owner/generation。
  admit → spawn → publish → await/control → settled → close。短 await 不应丢结果。
- **正常/失败**：unknown attempt、重复 cancel、spawn publication quota failure、reused ID/
  generation、Session close 竞态；result/dispatch receipt 不能仅保留文本。
- **已有断言**：Core `subagent_short_await_preserves_result_and_dispatch_receipt`、
  `subagent_family_dispatches_and_controls_live_attempts`；Bridge custom/factory 消费 tests。
- **最低充分证据**：真实 Host short await 后第二 await 得完整 result，control 绑定 exact
  attempt，multi-Session isolation、quota rollback、callback failure/cancel/close；三语言
  相同 context/result/receipt round-trip，不用 framework-only 测试替代 SDK 转换。

- **已证实（限具体case）**：原四个三语言场景核短await后完整结果/同receipt、重复await不新增attempt、约束进入原prompt、控制绑定原claim。客户端setup ACK后取消地址保留与Factory两事实/同owner重试也有真实反例和后继证明。
  证据：evidence.sdk-task-subagent-candidate、evidence.sdk-stream-cancellation-routing-verification、evidence.sdk-factory-child-settlement-verification。
- **仍缺/不承诺的边界**：只支持原已列context/result与合作取消窗口；不声称完整TaskSubagentContext DTO。5秒后原生产者是否实际停止仍由开放B义务承接，不能用本地future结束代替。

### family: bridge

- **入口/可达**：typed extension register/unregister → `extension_bridge.rs`，factory 把
  proxy 注入 live Session Agent；invoke/cancel/stream 反向走同一 official connection。
- **权威/状态**：Host lease/owner、deadline、pending mailbox/stream；framework Tool、
  LlmClient、Store/hook/callback 消费与 commit/terminal。descriptor validate → registered
  → captured by Session → pending invocation/stream → result/error/cancel → unregister。
- **正常/失败**：kind/operation/signature mismatch、late/duplicate result、callback 重入、
  unregister failure、initial stream ACK timeout、disconnect、no stream terminal、managed
  persistence outcome unknown；取消不能撤销已提交事实或提前释放 cleanup owner。
- **已有断言**：Bridge tool/callback/hook、Store/human-loop、LLM streaming/nonstreaming、
  malformed-kind/out-of-order/oversized/flood、deadline/cancel/disconnect；managed
  commit/deadline/restart 及 pending stream owner tests 已在指定候选运行，不扩张为每个component全覆盖。
- **最低充分证据**：对每个 accepted bridge kind/operation 至少合法消费和失败结算，验证
  Session 捕获的 proxy、factory instance locality、ToolEffect/full receipt 字段，store
  deadline-before/after commit 与 lost ACK/restart 同一 receipt；三语言 callback reentry、
  exception、cancel、disconnect，测 callback owner 释放和未完成调用数归零。

- **候选运行证据（candidate_runtime_verified）**：34 项 Bridge 测试实际执行（含准入拒绝场景）；Tool/hook/callback、LLM stream、factory 和 managed store 的指定路径消费及错误结算；deadline-before-work 无 mutation、lost ACK/重启同 receipt、初始 ACK 被取消后释放 callback owner。三语言 managed/callback/late factory 场景亦实际执行。
  来源：`echo-sdk-host/tests/extension_bridge_e2e.rs#tool_bridge_round_trip_with_callbacks_intervention_and_hook`；`echo-sdk-host/tests/extension_bridge_e2e.rs#managed_persistence_deadline_and_lost_ack_preserve_commit_authority`；`echo-sdk-host/tests/extension_bridge_e2e.rs#managed_commit_disconnect_restart_replays_same_receipt`；`echo-sdk-host/tests/extension_bridge_e2e.rs#cancelled_initial_stream_response_releases_session_callback_owner`。
- **仍缺或证据边界**：接受的每个 component operation 的合法消费/异常结算尚未逐项覆盖；11 DTO corpus 不等于11实际持久化动作。 未覆盖的已接受行为须具名登记输入、结果/失败与缺证原因；不增加未定义的任意组合验收门槛。

### family: standard

- **入口/可达**：official ACP v1 methods → `run_stdio`/framework `AcpAgentAdapter`，
  可无 Echo profile 使用；不靠 accepted facade catalog 将标准方法重新封装。
- **权威/状态**：framework standard Session/prompt/cancel，official schema 定义协商及
  wire shape；连接 initialize → Session → prompt/update → stop/cancel → EOF。
- **正常/失败**：标准 prompt/update 与扩展 Run 共享事实；版本/非法 params/无 profile、
  EOF、配置准备失败、stdout 污染不能被扩展成功用例遮盖。
- **已有断言**：Stdio standard prompt、parked-model cancel、EOF no noise、invalid config
  no secret；Protocol ACP baseline 检查 v1 constants、lock/checksum、unstable exclusions。
- **最低充分证据**：最终 binary 上普通 ACP client 无 profile 的正常 prompt/cancel/EOF，
  与 extended standard prompt 共享 Session/Run 比对；初始化错误、unsupported version
  和 startup cleanup 的真实行为。baseline constants 不是协议执行证据。

- **已证实（限具体case）**：独立、无Echo profile的stdio suite实际通过四项：标准prompt、parked model取消、stdin EOF干净退出、非法配置不污染stdout/不泄露secret；扩展standard prompt共用Session/Run的测试另外保留。
  证据：[be5最终Rust门禁](evidence/sdk-chart-published-pin/rust-gates-reviewed-final/README.md)中workspace-tests.log/ci-default-host.log的4项stdio结果；不是仅Host136/31/34/12旧收据。
- **仍缺/不承诺的边界**：未列出的协议版本/启动失败处置须按实际接受的ACP版本与错误合同定位；四项通过不自动证明所有平台signal或外部ACP实现互操作。

### family: value

- **入口/可达**：`echo-sdk-protocol/src/scalar.rs` 及 accepted schema/value routes；Host
  `wire.rs`，TS/Python wire、Java `WireValues.java`/`JsonSupport.java` 的 decode/encode。
- **权威/状态**：accepted DTO 拥有值合同；framework 原字段保持业务事实。无独立 Run/
  store；encoded → validated → projected → round-trip，不能把不支持的值变成 null/default。
- **正常/失败**：u64/i64、纳秒时间/duration、paths/OS encoding、bytes、Unicode、null vs
  absent、variant/record/map duplicates、float limits、opaque handles；每条转换错误可定位。
- **已有断言**：Wire `managed_checkpoint_and_receipts_preserve_large_revisions_and_subseconds`、
  tool effects/model content tests；三语言 protocol-integrity 消费 shared malformed corpus，
  special keys/中文🙂、opaque error handle、missing response envelope。
- **最低充分证据**：同一 accepted field corpus 作三语言 → Host → framework → Host round-trip，
  对超 JS safe integer、subsecond time、非 UTF-8/Windows path 的允许/拒绝边界、invalid
  optional 与 unknown discriminant 作精确断言；platform 不可运行时保留未验证项。

- **候选运行证据（candidate_runtime_verified）**：指定 wire tests 保留大 revision、subseconds、result absent/null、non-UTF8 Unix path；三语言 wire/malformed corpus 和 typed error fields 实际执行。
  来源：`echo-sdk-host/src/core_profile/wire.rs#managed_checkpoint_and_receipts_preserve_large_revisions_and_subseconds`；`echo-sdk-host/src/core_profile/wire.rs#unix_non_utf8_paths_round_trip_without_loss`；`sdks/typescript/test/wire.test.js`；`sdks/python/tests/test_protocol_integrity.py`；`sdks/java/src/test/java/com/echoagent/sdk/ProtocolIntegrityTest.java`。
- **仍缺或证据边界**：Windows native path 与所有 accepted field 经 Host/framework 的统一 corpus 尚未完成；语言-local 单测不充当 Host round-trip。 未覆盖的已接受行为须具名登记输入、结果/失败与缺证原因；不增加未定义的任意组合验收门槛。

### family: intrinsic

- **入口/可达**：accepted intrinsic/helper identities 经语言 catalog/helper 或 schema
  消费；family 无远程 method/operation，不应推断 `_echo_agent/intrinsic/op` 可执行。
- **权威/状态**：accepted external contract 决定明确承诺的 helper shape；无 Host runtime
  状态机。完整 inventory intrinsic membership/digest 仅 telemetry，不进入阻断协商。
- **正常/失败**：已接受 helper 的 constructor/value/optional 表达在语言可用；未接受
  Rust intrinsic 不新增 API，不用填默认值伪造 helper 支持。
- **已有断言**：Protocol facade inventory 与三语言 catalog tests 主要核对声明及映射。
- **最低充分证据**：列出 accepted helper 的实际消费位置并执行语言 value round-trip；
  contract gate 证明未接受 intrinsic 漂移不阻断 accepted handshake。catalog 对齐本身不
  是全部 helper 行为完成；不恢复冻结完整 intrinsic membership 摘要。

- **已证实（限具体case）**：已有具名三语言helper/value tests实际执行构造、getter、不可变modifier、typedfailure和纯值投影；原testing14字段12positive7invalid另有真实公共codec消费。inventory非external漂移不改变accepted contract门禁的测试支持范围边界。
  证据：sdks/typescript/test/intrinsic-tool-values.test.js、helpers.test.js、Java IntrinsicToolValueTest及evidence.sdk-testing-value-candidate。
- **仍缺/不承诺的边界**：具名helper仅按明确接受合同对账，不冻结完整Rust intrinsic membership或恢复每identity parity分母。catalog done不作行为证明，未接受helper不新增远程方法。

## 集成 family 逐项矩阵

### family: memory

- **入口/可达**：`facade/memory.rs::dispatch`，memory.store 路径取 live Session Agent
  Store；resource 路径从 `source_operations.rs::MemoryStoreAuthority` 及统一 registry 取实例。
- **权威/状态**：framework `memory::Store` 及其 namespace/provenance/expiry；Host 只
  保存 resource owner。put/get/search/list/delete；resource open → consume → close。
- **正常/失败**：put 后真实 agent/tool recall 同一数据；namespace、search mode、page/
  truncated、expiry/dedup；file I/O 或 extension Store failure 应传播，不能假 put 成功。
- **已有断言**：Core admission test put/get/delete/unknown；stateful-source resource get
  key/value、cross-owner/close；adapter unit 覆盖部分 search/resource 操作。未证明真实
  Session agent recall 与外部 resource store 相同，更未覆盖全部 provenance/expiry。
- **最低充分证据**：真实 Host memory.store 写后由 live Agent memory tool 读取；多 Session
  namespace isolation，resource search_with/prune/dedup 与显式 expiry/provenance；file/managed
  Store fault、publication/close 重试，三语言完整 item 字段和 large timestamps round-trip。

- **已证实（限具体case）**：三语言59行共享场景已核live Store替换、namespace/CRUD/search/prune/dedup指定操作、Active recall与provenance，以及反向Store真实消费和失败副作用；原来源与新业务复审分开记录。
  证据：evidence.sdk-memory-permission-candidate及[Memory/Permission证据](memory-permission-evidence.md)。
- **仍缺/不承诺的边界**：只支持该场景列出的provenance/expiry/namespace与回调字段，不外推任意Store实现或所有召回策略。未接受的approval_receipt字段不自动进入合同。

### family: workflow

- **入口/可达**：`facade/workflow.rs::dispatch` → framework declarative graph/engine；
  extension workflow 与 facade stream 用同一 graph/stream adapter。
- **权威/状态**：framework `workflow::Graph`、`SharedState`、`CheckpointStore`、cancel
  token；Host graph/state/stream owner。build → run → interrupt/checkpoint → exact resume/
  restore/branch → terminal/close。
- **正常/失败**：condition path、pending node、state revision/identity、branch/tag/load；
  invalid graph/checkpoint、cancel-vs-next、same-owner wrong-kind、Session close、stream error。
- **已有断言**：Core declarative graph 5 nodes/4 edges、conditional yes/no、checkpoint、state
  get/set/keys、cancel then framework error、cross owner；Facade unified stream/backpressure/
  connection teardown；Bridge workflow-agent component stream。
- **最低充分证据**：resume_exact/resume_with_state/restore/branch/tag 的完整 state/checkpoint
  字段比对，cancel/fault 后 graph terminal 唯一；stream next 并发/slow consumer/close retry
  不漏 NodeError/Completed，三语言同场景且重启后 checkpoint 身份不被默认替换。

- **候选运行证据（candidate_runtime_verified）**：真实五节点四边条件图、checkpoint/state 操作；cancel 返回规范 Cancelled/cause 并在 stream failed terminal 保留；统一资源 owner/stream/backpressure/close 和反向 workflow-agent 消费实际执行。
  来源：`echo-sdk-host/tests/core_profile_e2e.rs#workflow_family_runs_declarative_graphs_over_the_framework_engine`；`echo-sdk-host/tests/facade_feature_adapters_e2e.rs#workflow_cancel_preserves_canonical_failure_in_rpc_and_stream_terminal`；`echo-sdk-host/tests/facade_feature_adapters_e2e.rs#workflow_stream_uses_unified_handle_backpressure_and_close`；`echo-sdk-host/tests/extension_bridge_e2e.rs#workflow_agent_component_stream_flows_through_facade_pull_stream`。
- **仍缺或证据边界**：resume_exact/restore/branch/tag 所有 state/checkpoint 字段及三语言重启共同场景未完成。 未覆盖的已接受行为须具名登记输入、结果/失败与缺证原因；不增加未定义的任意组合验收门槛。

### family: delivery

- **入口/可达**：`facade/state_delivery.rs::dispatch_delivery`，ledger.open 后 resource
  存储真实 framework `DeliveryLedger`；enqueue/claim/transition/defer/settle/recover 操作它。
- **权威/状态**：每个ledger的framework内存journal/checkpoint与exact delivery claim；
  Host 只标 owner/address。pending → claimed → dispatched/deferred/settled，revision/CAS
  必须携带，不以 delivery ID 单独代替 claim。
- **正常/失败**：同 claim 转移、recovery snapshot；stale/wrong claim、duplicate settle、
  concurrent claim、journal/commit failure/outcome unknown、close 不能返回虚构成功。
- **已有断言**：Core state-delivery test open/enqueue/claim/transition/settle/snapshot/recover；
  是 memory ledger 路径，不自动证明 durable crash recovery 或 managed CAS。
- **最低充分证据**：单 active claim 可 effect、effect 前 supersede、effect 后 claim_next=None；
  旧 claim transition rejection 且 snapshot unchanged，
  defer/recover/settle 字段及副作用 exact，真实Host三语言CAS/receipt round-trip及同实例recover。当前open只构造memory ledger，不加入未接受的跨进程durability门槛；Rust另可选journal不据此删除。

- **已证实（限具体case）**：当前接受的ledger.open/enqueue/claim_next/transition/defer/settle/recover/snapshot走原MemoryEventJournal+MemoryCheckpointStore。三语言已核完整exact claim、旧attempt/四个stale command无变更、effect前supersession、仅active可effect、effect后严格claim_next=null及同实例recover。
  证据：accepted-facade-operation-catalog.json的delivery family；state_delivery.rs::delivery.ledger.open实际构造，以及evidence.sdk-state-trace-delivery-candidate/exact_claim_roundtrip unit。
- **仍缺/不承诺的边界**：当前SDK接受的open未提供持久journal/store选择，故不承诺跨进程/crash durable Delivery。保留已有同实例恢复、CAS/receipt/错误语义；框架Rust可选持久journal能力不删除，未来接受新的durable Host路由才新增对应故障/重启证明。

### family: state

- **入口/可达**：`facade/state_delivery.rs::dispatch_state` 直接使用 Host shared
  framework `FileRuntimeStateStore`；Agent/Session runtime 保存应访问同一 service。
- **权威/状态**：framework runtime/checkpoint store；Host 不维护第二 checkpoint JSON。
  save → get/list → runtime/scope clear，commit/identity/generation 保留。
- **正常/失败**：完整 checkpoint save/get、一致 scope clear；invalid scope/revision、
  partial write/corrupt file、并发 save/clear、重启/outcome unknown 不能吞错。
- **已有断言**：Core save ok/list 一个 runtime/get/clear 与 unknown checkpoint operation；
  Bridge managed persistence tests 和 Wire field tests 是其它指定边界，不等同文件路径证明。
- **最低充分证据**：真实 Run checkpoint 与 facade read 同一字段及 journal identity，
  clear scope/重启隔离、file fault 与 atomicity，三语言已接受AgentCheckpoint七字段与subsecond/optional round-trip；大revision/commit receipt另按managed合同；不能只断言 save 的 `ok=true`。

- **已证实（限具体case）**：三语言FileRuntimeStateStore场景已比较AgentCheckpoint公开7字段、可选值和纳秒时间，scope A清理保留B，关闭并冷恢复后generation/runtime owner索引与原数据保持；managed lost ACK恢复证据属于另一个具名路径。
  证据：[State/Trace/Delivery证据](state-trace-delivery-evidence.md)、evidence.sdk-state-trace-delivery-candidate及formal executed-test-source。
- **仍缺/不承诺的边界**：scope索引是投影，不宣clear全局all-or-none；未由指定冷恢复场景证明所有文件损坏或并发写入失败。AgentCheckpoint7字段不与WorkflowCheckpoint17或Trace Run17选定字段混淆。

### family: trace

- **入口/可达**：`facade/observability.rs::dispatch_trace` 创建 framework RunStore；
  `source_operations.rs` 的 trace resource verbs 复用实例。
- **权威/状态**：framework `trace::RunStore`，memory 或 `JsonlRunStore`；Host store handle。
  open → save/load/list → close；list page/truncation 和 absent run 是合同结果。
- **正常/失败**：完整 Run/RunSummary round-trip，unknown mode、missing/invalid path、quota、
  JSONL write/corruption、Session close；未发布 store 不应耗用 slot。
- **已有断言**：Core trace memory save/load/list_recent；adapter
  `failed_trace_open_does_not_consume_the_only_resource_slot` 断言非法 open 后可复用配额。
- **最低充分证据**：JSONL path、persist/reopen、full Run metadata/events/usage 字段，
  malformed run、I/O fault、retention/page boundaries、quota failure 后资源计数、close retry；
  live framework collector 与 SDK store 是否同实例须按 route 明确，不假定所有 trace 自动写入。

- **已证实（限具体case）**：三语言实际写JSONL、保原字节SHA、关闭后重开/冷读；选定Run17字段、events/options、sanitizer、纳秒timestamp及timings.total_duration_ms=9007199254740993逐项比较。非法构造不占唯一slot也有原unit。
  证据：[State/Trace/Delivery证据](state-trace-delivery-evidence.md)、evidence.sdk-state-trace-delivery-candidate与原JSONL执行产物。
- **仍缺/不承诺的边界**：不假定facade trace自动绑定live collector，不声称所有enum/retention/I/O fault组合已测；实际只承诺已选路由/字段与错误合同，未列义务需具名而非无限全矩阵。

### family: eval

- **入口/可达**：`facade/observability.rs::dispatch_eval` 的 constraints/report；accepted
  source runner/grader route 在 `source_operations.rs`，通过 extension factory 真正消费 agent。
- **权威/状态**：framework eval constraint evaluator/report/runner/grader；Host runner
  resource 和 factory address。report leaf 无长期状态；runner 才有 invocation 生命周期。
- **正常/失败**：criteria results/counts、factory lazy create、per-case outcomes；malformed
  run/result、grader error、deadline/cancel、factory release/quota/feature disabled。
- **已有断言**：Core constraints 与 report total=2/passed=1/failed=1、unknown op；Bridge
  eval/improve source factory lazy invocation。报表计数不证明 runner 取消与清理。
- **最低充分证据**：所有 accepted runner/grader verbs 的真实输入及 result fields，factory
  instance 隔离，one failing grader/timeout/disconnect 后 cleanup 和 case outcome，不丢
  metric/error；三语言 DTO/feature error 一致，禁止把仅统计 report 当作 runner 覆盖。

- **候选运行证据（candidate_runtime_verified）**：report total2/pass1/fail1 与 constraints；真实 EvalRunner lazy factory 子实例消费已执行，factory failure 在 case 结果保留 canonical cause；重型 Factory 两场景不再 stack overflow。
  来源：`echo-sdk-host/tests/core_profile_e2e.rs#eval_and_improve_families_use_the_framework_analyzers`；`echo-sdk-host/tests/extension_bridge_e2e.rs#eval_and_improve_source_adapters_invoke_agent_factory_lazily`；`echo-sdk-host/src/core_profile/facade/source_operations.rs#factory_failure_retains_canonical_cause_in_real_eval_result`。
- **仍缺或证据边界**：全部 grader/metric/error 字段、timeout/disconnect/cancel 的 runner cleanup 尚未穷尽。 未覆盖的已接受行为须具名登记输入、结果/失败与缺证原因；不增加未定义的任意组合验收门槛。

### family: improve

- **入口/可达**：`facade/observability.rs::dispatch_improve` 和 source improve loop/
  trajectory saver route；agent factory 反向调用来自 bridge，不在 Host 重建优化执行器。
- **权威/状态**：framework ShareGPT exporter、RunAnalyzer、source loop；Host resource/
  callback ownership。leaf export/analyze 与 stateful loop/saver 的生命周期分别验收。
- **正常/失败**：trajectory messages、critique 与 run identity；improve 编译但 eval 不在时
  analyze 应明确 FeatureUnavailable；factory failure/cancel/timeout/save I/O 不能默认成功。
- **已有断言**：Core ShareGPT messages 为数组的形状断言、critique run_id/success；Bridge factory lazy invocation。
- **最低充分证据**：真实 source loop/result/stop/cancel 和 trajectory save/reopen，保留
  critique 指标/失败原因、factory instance owner cleanup；独立 feature build 下 analyze
  缺 eval 的错误 detail；三语言同一 run 分析结果，不能靠 loop registration 即认定已执行。

- **候选运行证据（candidate_runtime_verified）**：ShareGPT messages 为数组的形状、critique run_id/success、factory lazy invocation 指定断言实际执行；未验证导出内容逐项正确。
  来源：`echo-sdk-host/tests/core_profile_e2e.rs#eval_and_improve_families_use_the_framework_analyzers`；`echo-sdk-host/tests/extension_bridge_e2e.rs#eval_and_improve_source_adapters_invoke_agent_factory_lazily`。
- **仍缺或证据边界**：source loop stop/cancel、trajectory save/reopen、缺 eval feature 的 standalone 行为矩阵未验证。 未覆盖的已接受行为须具名登记输入、结果/失败与缺证原因；不增加未定义的任意组合验收门槛。

### family: permission

- **入口/可达**：`facade/permission.rs::dispatch`/`service_of` 获取 live Session Agent
  `PermissionService`，agent run 工具权限路径应消费同一 service。
- **权威/状态**：framework PermissionService/rules/approval cache；Host 不新增 policy。
  mode/rules update → exact invocation check → approval/cache expiry/revoke；按 Session 隔离。
- **正常/失败**：allow/deny/ask/require_approval 与 context、effect binding；非法 mode/
  permission/rule、ttl/time、invalid cache target、callback failure；不添加本地交互权限闸。
- **已有断言**：Core source Session permission_mode setter/getter；Bridge human-loop 消费。
  这些不覆盖 family 的全套 rule/cache/context 更新行为。
- **最低充分证据**：family add/remove rules 后 live tool 相同 effect decision、mode check
  variants 与 exact context、expiry/revoke/cleanup；两 Session rule/cache 隔离，失败更新无
  半写；三语言保留 ask suggestions 与 matcher/source/TTL，不用仅 enum 解析做权威证明。

- **已证实（限具体case）**：三语言59行场景有真实HumanLoop上下文、拒写零effect与approved once。默认TTL另外以三原生客户端各26次调用、真实等待≥1801500ms验证：四条审批到期false，读取不续期/不删除，stats仍2+2，显式cleanup4→0、fresh审批true；六原PID退出。
  证据：[真实默认30分钟TTL](evidence/sdk-real-permission-ttl/README.md)、evidence.sdk-real-permission-default-ttl、evidence.sdk-memory-permission-candidate。
- **仍缺/不承诺的边界**：默认30分钟到期未观察的旧缺口已补；未新增短TTL setter或改clock。只证明易失Session缓存，不承诺跨进程审批持久化；其它明确matcher/revoke故障义务须单独列case，不能因当前证明自动全族verified。

### family: mcp

- **入口/可达**：`facade/integrations.rs::dispatch` → `McpManager`/`McpClient`；source
  exact constructors/tool adapters 同服务，`IntegrationResources` 保存 preparation/cleanup debt。
- **权威/状态**：framework manager/client/transport；Host manager-to-derived-client/tool
  关联及 handles。prepare/connect → publish → tools/resources/prompts calls → disconnect/
  close_all → awaited cleanup。同名 server 在不同 manager 中不是同一 owner。
- **正常/失败**：config/reconcile/get_clients/tool call、server metadata；connect/close failure、
  cancelled preparation、quota publication rollback、manager close 后 derived handles 失效。
- **已有断言**：Core fresh manager/list、connect fault；Bridge MCP quota publication 失败
  等待 transport close；integrations unit 用 CleanupTransport 覆盖部分 debt retry；Facade
  Agent cascade same-name isolation。mock transport unit 不证明 native subprocess 生命周期。
- **最低充分证据**：真实 stdio/HTTP fixture server connect/call/resources/prompts/close，
  两 manager 同名隔离、derived client/tool 失效；close fault 后可重试 debt、取消 publication
  不遗留进程/slot；三语言 typed metadata/error/callback，feature disabled 单独验收。

- **候选运行证据（candidate_runtime_verified）**：config/file 两真实 stdio batch 在 quota1 拒绝整批 SDK alias，不发布无返回 prefix；已成功连接仍由 Agent 管理，best-effort accepted 行为保留，既有 alias 与同名其他 manager 不被误关；close debt unit 与 transport publication quota 场景实际执行。
  来源：`echo-sdk-host/tests/facade_feature_adapters_e2e.rs#mcp_config_batch_quota_failure_does_not_publish_unreturned_prefix`；`echo-sdk-host/tests/facade_feature_adapters_e2e.rs#mcp_file_batch_quota_failure_does_not_publish_unreturned_prefix`；`echo-sdk-host/tests/facade_feature_adapters_e2e.rs#mcp_config_batch_preserves_best_effort_and_existing_alias`；`echo-sdk-host/tests/facade_feature_adapters_e2e.rs#agent_cascade_preserves_same_named_other_manager_and_releases_resources`。
- **仍缺或证据边界**：HTTP/SSE全请求资源/prompts、取消 publication 与 native close fault 的三语言统一矩阵仍缺。 未覆盖的已接受行为须具名登记输入、结果/失败与缺证原因；不增加未定义的任意组合验收门槛。

### family: a2a

- **入口/可达**：`facade/integrations.rs::dispatch` → framework `a2a::A2AClient`；task
  SSE stream 经 `facade/stream.rs`，统一 owner/window 而非第二 A2A task authority。
- **权威/状态**：远端 A2A task、framework client/protocol；Host client/stream handles。
  open/discover → send/get/cancel 或 stream → terminal/close；远端 cancel acknowledgement
  与本地 stream close 必须区分。
- **正常/失败**：task/update/artifact metadata、stream order；closed-port discovery、HTTP/
  malformed SSE、timeout/cancel/disconnect、missing terminal、quota publish rollback。
- **已有断言**：Core client handle 与 closed-port FrameworkError；Facade
  `a2a_sse_stream_delivers_items_then_terminal` 检查 item/terminal；此为 loopback fixture，
  不证明全部 accepted task cancel/recovery 或外部互操作。
- **最低充分证据**：fixture 覆盖 discover/send/get/cancel、SSE fail/no-terminal/reconnect，
  并发 next、Session close 回收；三语言逐字段保留 task/artifact/status/remote error，
  明确不支持的恢复边界，不能把本地 close 等同远端已取消。

- **候选运行证据（candidate_runtime_verified）**：closed-port discovery 返回 FrameworkError；loopback SSE 产生 items 后唯一 terminal，并使用统一 stream。
  来源：`echo-sdk-host/tests/core_profile_e2e.rs#integration_families_use_framework_managers_and_clients`；`echo-sdk-host/tests/facade_feature_adapters_e2e.rs#a2a_sse_stream_delivers_items_then_terminal`。
- **仍缺或证据边界**：discover/send/get/cancel 的完整远端 DTO、断线恢复和三语言同场景未覆盖，不推断远端取消。 未覆盖的已接受行为须具名登记输入、结果/失败与缺证原因；不增加未定义的任意组合验收门槛。

### family: lsp

- **入口/可达**：`facade/integrations.rs::dispatch` → framework `LspManager`/
  `StdioLspClient`；derived client 经统一 registry，manager/子 client 关联需检查。
- **权威/状态**：framework server process/initialize/document version/diagnostic state；
  Host manager/client owner。config/root → start/initialize → document/RPC → shutdown/restart。
- **正常/失败**：load_config/start/restart/get_client、did_open/change/save/close、诊断及
  definition/reference/hover/completion；bad config/server exit/stale doc/close failure。
- **已有断言**：Core fresh manager status/no running servers，仅此不能证明 LSP process
  或 request response。当前已接受 client methods 必须逐行为补场景。
- **最低充分证据**：可控 stdio LSP fixture 的 initialize → UTF-16/Unicode document →
  did_change/version → response/diagnostic → restart/shutdown，错误 RPC 与 server crash，
  derived handle close/quota/owner/generation；三语言 positions/ranges/null/arrays 无损。

- **候选运行证据（candidate_runtime_verified）**：同一真实 stdio fixture 的29 case+1 lifecycle 共30观察/44 text-document calls，三语言返回值/typed errors/request trace 哈希一致；malformed response 明确失败后后续请求可用，shutdown 进程已退出，旧 client 不能复活。
  来源：`sdks/typescript/test/lsp-host.test.js`；`sdks/python/tests/test_lsp_host.py`；`sdks/java/src/test/java/com/echoagent/sdk/LspHostTest.java`。
- **仍缺或证据边界**：diagnostics push 的4项新增用例与新 framework 修复正在独立工作，未包括本冻结三语言收据；UTF-16版本/change/restart/owner竞态完整矩阵仍缺。 未覆盖的已接受行为须具名登记输入、结果/失败与缺证原因；不增加未定义的任意组合验收门槛。

### family: channels

- **入口/可达**：`facade/integrations.rs::dispatch_channels`，channels + bridge 编译后
  manager 注册 extension plugin，framework `ChannelManager` 消费 message handler。
- **权威/状态**：framework channel/plugin connection 与 message delivery；Host proxy
  lease/resource/handler address。register → start → receive/send/health → stop → close。
- **正常/失败**：plugin message/attachments/result，stop_exact/list/ids；start/stop/send
  callback error、disconnect、feature/bridge absent、publication failure 和 unregister race。
- **已有断言**：Bridge `channel_plugin_facade_round_trip` 和 channel feature requirement；
  Core admission create resource/MethodNotFound。仅 manager.open 不证明 receive/send。
- **最低充分证据**：真实 Host plugin start/send/receive/hook/attachments，stop 与 pending
  callback 结算、stop_exact 不影响 sibling manager、重入/error/disconnect 后 resource
  可重试/无 leak；三语言 message identity 与 delivery outcome 一致。

- **候选运行证据（candidate_runtime_verified）**：真实 plugin 经 facade manager 注册、start/send/stop 的指定输入/输出与 feature requirement 断言通过；未以 manager.open 代替插件消费。
  来源：`echo-sdk-host/tests/extension_bridge_e2e.rs#channel_plugin_facade_round_trip`。
- **仍缺或证据边界**：所有 receive/attachment/重入、stop_exact sibling隔离及三语言 bridge异常资源结算仍缺。 未覆盖的已接受行为须具名登记输入、结果/失败与缺证原因；不增加未定义的任意组合验收门槛。

### family: telemetry

- **入口/可达**：`facade/telemetry.rs::dispatch` → framework init/status/shutdown，
  `facade/mod.rs` feature branch 注册；不是 Session-owned metrics store。
- **权威/状态**：framework process-global telemetry subscriber/exporter/Metrics。init →
  status → shutdown；同进程多 Session 初始化冲突是全局事实，不应伪称 Session 隔离。
- **正常/失败**：OTLP config、repeated init/shutdown、exporter error；当前 optional
  enable_console 由严格 enable_console_of 拒绝错误类型；该反例已在指定候选执行，exporter/并发初始化与最终 pin 证据仍待。
- **已有断言**：Core feature-disabled telemetry.status 的 admission；没有充分 exporter
  flush/shutdown 或错误参数行为证明。
- **最低充分证据**：独立 Host process 中 fixture collector 收到记录、shutdown flush，
  invalid endpoint/service/enable_console 类型明确错误、repeated lifecycle，配置秘密不入日志；
  三语言全局语义和错误一致，不把 status.initialized 单一布尔当作 exporter 完成。

- **候选运行证据（candidate_runtime_verified）**：invalid console string/number/list/map 在全局初始化前明确 InvalidValue，missing/null/true 的布尔规则精确；三语言 smoke status.initialized 为 boolean。
  来源：`echo-sdk-host/src/core_profile/facade/telemetry.rs#invalid_console_flag_is_rejected_before_process_initialization`；`sdks/typescript/test/host.test.js`。
- **仍缺或证据边界**：真实 exporter collector/flush/shutdown、secret logging、进程级重复生命周期缺行为证明。 未覆盖的已接受行为须具名登记输入、结果/失败与缺证原因；不增加未定义的任意组合验收门槛。

### family: topology

- **入口/可达**：`facade/integrations.rs::dispatch` → framework `TopologyTracker`；
  exact constructor/node adapters 与 family tracker 共享实例。
- **权威/状态**：framework tracker nodes/edges/stats；Host resource owner。open → node/
  call record → snapshot/export/clear → close，不用 Host 重算拓扑业务模型。
- **正常/失败**：node variants、duration、stats 与 graph exports；unknown node type、bad
  duration、owner/generation/quota/close、concurrent record/snapshot。
- **已有断言**：Core add node/record call、snapshot、nodes 数/类型、edges 数、stats 不返回
  全 snapshot、cross-owner InvalidValue。
- **最低充分证据**：exact node metadata/duration、clear 后 empty、Mermaid/JSON/DOT 与
  snapshot 一致，非法 inputs 无 mutation、并发 record 与 page limit；三语言 fields/large
  counters/Unicode names，无 resource close 后继续记账。

- **候选运行证据（candidate_runtime_verified）**：add node/record edge，snapshot2nodes1edge，再加 exact label/metadata 的节点得3nodes；stats 不返回 full snapshot，cross-session access 明确 InvalidValue。
  来源：`echo-sdk-host/tests/core_profile_e2e.rs#integration_families_use_framework_managers_and_clients`。
- **仍缺或证据边界**：duration/large counters、clear/导出一致性、并发 mutation 与三语言字段对比仍缺。 未覆盖的已接受行为须具名登记输入、结果/失败与缺证原因；不增加未定义的任意组合验收门槛。

### family: web

- **入口/可达**：`facade/tools.rs::dispatch` 构造 framework WebFetch/WebExtract/WebSearch，
  按 schema positional arguments 与 Session ToolContext 调用 `Tool::execute`。
- **权威/状态**：framework tool/HTTP response；Host 投影 typed ToolResult，不建立缓存或
  网络策略。admit → HTTP/extract → result/artifact，取消后副作用事实保持真实。
- **正常/失败**：fetch/extract/search 内容、metadata/truncation；HTTP status、redirect、
  invalid URL、encoding/large body、timeout/cancel 应保留 failure/result，不能只输出文本。
- **已有断言**：Core offline web_extract 仅断言 success=true，未断言 heading/body，也未
  证明 web_fetch/search 网络路径。
- **最低充分证据**：loopback fetch/redirect/search fixture 比较内容与 structured fields，
  失败/timeout/超大 body/中文 UTF-8、cancel/EOF、tool effect/full error 三语言一致；保留
  本地产品现有轻量校验，不另加公网服务威胁门控。

- **候选运行证据（candidate_runtime_verified）**：离线 web_extract 实际通过 success=true；这是一次真实 framework tool 调用，不包含 heading/body 精确值断言。
  来源：`echo-sdk-host/tests/core_profile_e2e.rs#tool_families_execute_framework_tools_with_the_session_cwd`。
- **仍缺或证据边界**：fetch/search/redirect/timeout/超大 body 的loopback与三语言内容/effect对账仍缺。 未覆盖的已接受行为须具名登记输入、结果/失败与缺证原因；不增加未定义的任意组合验收门槛。

### family: files

- **入口/可达**：`facade/tools.rs` framework files tools；source FileAuthority guard/
  lease 在 `source_operations.rs`，Session cwd 进入真实 ToolContext。
- **权威/状态**：文件系统和 framework tool/file authority；Host lease handle/address。
  read/write/patch/move/delete 与 open guard → consume → close；文件身份必须保留。
- **正常/失败**：返回内容/diff/effect/artifact 与磁盘一致；nonexistent path、invalid patch、
  stale lease/concurrent write、partial side effect、quota 和 close/reacquire。
- **已有断言**：Core relative write/read 内容；source file tests 读取磁盘 bytes、guard 长度、
  lease 冲突、重复 close false、reacquire；tool DTO unit 保留 effects/failure/model_content。
- **最低充分证据**：accepted read/search/list/write/append/update/create/delete/move/patch
  按操作结果与实际 filesystem 验证，Unicode/non-UTF8 path、stale identity、失败无虚假
  complete effect，三语言 typed effect/partial failure；格式/contract 不替代实际文件证据。

- **已证实（限具体case）**：三语言普通工具后继语料覆盖14个文件工具操作的指定输入：真实create/append/update/move/delete/transactional patch、失败更新/重复创建/stale hash、多文件回滚、read/diff/list/grep/glob/repo_map/code_search；原字节、artifact连续分页/SHA/cursor拒绝及副作用对账。
  证据：[Tool/Value证据](evidence/tool-value-effects/README.md)、其b909-natural-eof-after与最新完整语言记录。
- **仍缺/不承诺的边界**：不把工具写入证明扩成FileAuthority取得guard后同路径replacement一定拒写；该具体合同与现路径仍调查中。未采集全文件树时不声称任意外部路径都无副作用。

### family: shell

- **入口/可达**：`facade/tools.rs` framework ShellTool；sandbox.extension.run_stream
  经 `facade/mod.rs` bridge adapter/统一 stream registry。
- **权威/状态**：framework shell/sandbox executor 与 OS process；Host stream/callback
  owner。start → stdout/stderr pull → exit/cancel → cleanup settled → release。
- **正常/失败**：cwd/exit code/output、sandbox typed effects；nonzero exit、spawn failure、
  timeout/cancel/disconnect、pending initial ACK、进程子树清理与重复 close。
- **已有断言**：Core printf output/success；Bridge sandbox cancel 等待 cleanup 并保留
  cancelled terminal；pending stream owner 候选运行已通过；native process完整矩阵仍待。
- **最低充分证据**：真实 child process 与 streaming executor，nonzero exit、slow/pending
  stream、kill/cancel/EOF 后进程/slot 无 leak且 result 真实；三语言 signal/timeout/stream
  结束同场景，不用 printf 正常路径推断取消正确。

- **候选运行证据（candidate_runtime_verified）**：printf真实输出与 success；sandbox Cancelled terminal 在真实 cleanup副作用后结算，pending callback owner 相关桥测试实际通过。
  来源：`echo-sdk-host/tests/core_profile_e2e.rs#tool_families_execute_framework_tools_with_the_session_cwd`；`echo-sdk-host/tests/extension_bridge_e2e.rs#sandbox_cancel_bridge_waits_for_cleanup_and_preserves_cancelled_terminal`。
- **仍缺或证据边界**：native child signal/nonzeroexit/慢stream/kill后进程与slot完整三语言矩阵仍缺。 未覆盖的已接受行为须具名登记输入、结果/失败与缺证原因；不增加未定义的任意组合验收门槛。

### family: git

- **入口/可达**：`facade/tools.rs` framework git tools，在 Session cwd；worktree
  enter/exit/list 也必须走已有 framework 工具，Host 不自建 worktree manager。
- **权威/状态**：Git repository/index/worktree 事实与 framework ToolResult/effects。
  status/diff/log/blame/branch/commit、worktree create/enter/exit，失败不能消失用户 dirty state。
- **正常/失败**：非 repo、invalid ref、dirty/conflict、commit failure、worktree close/
  cwd 变化；actual commit SHA/index/files 与返回对应。
- **已有断言**：原Core弱断言保留；新增共享真实Host精确场景：两Session cwd隔离、exact status/diff；sdk-proof创建指向原HEAD；invalid branch后repo A HEAD/sdk-proof及repo B refs指定值不变；Session A close后B仍可用。
- **最低充分证据**：isolated fixture repo 全 accepted git 操作、dirty preservation、
  invalid ref/commit failure、enter/exit/list 的 filesystem/Git 对账；三语言 effect/error
  对等，不对真实工作树执行破坏性验收。

- **候选运行证据（candidate_runtime_verified）**：两Session cwd隔离、exact status/diff；sdk-proof创建指向原HEAD；invalid branch后repo A HEAD/sdk-proof及repo B refs指定值不变；Session A close后B仍可用。
  来源：`echo-sdk-host/tests/tool_behavior_governance_e2e.rs`、三语言ToolBehaviors测试与共享fixture；
  `.echo-semantic/evidence/evidence.sdk-tool-behavior-candidate.md`绑定实测Host/源hash/原始日志。
- **仍缺或证据边界**：未比较repo A全部refs前后；git log/blame/commit和worktree enter/exit/list完整生命周期、冲突/失败清理及platform variants仍缺。 最后相关变更后的独立复审、正式pin和精确main仍待；
  不从整组绿色推断未列出的字段/生命周期/失败组合。

### family: database

- **入口/可达**：`facade/tools.rs` framework SqlQuery/ListTables/DescribeTable；schema
  参数映射后工具连接 DB，无 Host 自建 SQL 执行权限或数据库迁移。
- **权威/状态**：framework tool 和目标数据库事实；连接/query 生命周期由工具管理。
- **正常/失败**：tables/schema/rows/types/null、大整数与查询限制；read-only rejection、
  bad URL/SQL/schema、timeout/connection close，失败不得误投成 RPC success 数据。
- **已有断言**：原Core弱断言保留；新增共享真实Host精确场景：真实SQLite columns/rows/null精确值；DELETE返回structured_error/tool_error与permanent/stop/side_effect=none；reread相同且DB文件SHA不变。
- **最低充分证据**：查询精确 row/schema/null/large int，rejected write 后数据库 unchanged，
  list/describe、invalid SQL/timeout/cleanup、三语言 typed result/error；SDK framework DB
  能力不意味着 EKO CLI 引入 SQLite。

- **候选运行证据（candidate_runtime_verified）**：真实SQLite columns/rows/null精确值；DELETE返回structured_error/tool_error与permanent/stop/side_effect=none；reread相同且DB文件SHA不变。
  来源：`echo-sdk-host/tests/tool_behavior_governance_e2e.rs`、三语言ToolBehaviors测试与共享fixture；
  `.echo-semantic/evidence/evidence.sdk-tool-behavior-candidate.md`绑定实测Host/源hash/原始日志。
- **仍缺或证据边界**：list_tables/describe_table、large integer、invalid SQL/URL、分页/timeout/connection failure及其它数据库驱动仍缺。 最后相关变更后的独立复审、正式pin和精确main仍待；
  不从整组绿色推断未列出的字段/生命周期/失败组合。

### family: rag

- **入口/可达**：`facade/tools.rs::RagToolSet` 为每 Session 创建 framework RagStore/
  HttpEmbedder；index/search 共用 set，chunk tool 是独立 leaf。
- **权威/状态**：framework vector store/embedder；Host 只拥有 Session instance reference。
  configure → chunk/index → search → Session close drop，不能每次调用新建空 store。
- **正常/失败**：index 后 search 命中、Session isolation；缺 key/invalid config、embedding
  provider fault、cancel/large document/close，明确 FeatureUnavailable/InvalidConfig。
- **已有断言**：Core chunk success；adapter unit 使用 TestEmbedder index/search 命中，
  第二 RagToolSet 不命中。TestEmbedder 不证明真实 Host HttpEmbedder 配置与网络路径。
- **最低充分证据**：loopback embedding provider 真实 Host index→search、两 Session
  isolation、missing env/bad endpoint/provider error/cancel，无半提交索引；三语言命中
  score/source/chunk metadata 与字符边界、清理和 typed error，不能以 chunk 成功覆盖 index。

- **候选运行证据（candidate_runtime_verified）**：真实 Host chunk工具success；TestEmbedder unit中同一Session工具集index/search命中、另一工具集不命中。
  来源：`echo-sdk-host/tests/core_profile_e2e.rs#remaining_tool_families_execute_framework_tools_locally`；`echo-sdk-host/src/core_profile/facade/tools.rs#rag_index_and_search_share_one_session_store`。
- **仍缺或证据边界**：真实 Host HttpEmbedder网络路径/索引半提交/错误取消与三语言score/metadata缺口；TestEmbedder不是网络证据。 未覆盖的已接受行为须具名登记输入、结果/失败与缺证原因；不增加未定义的任意组合验收门槛。

### family: chart

- **入口/可达**：`facade/tools.rs` → framework GenerateChartTool，输入按 tool schema 映射。
- **权威/状态**：framework chart spec/result/artifact 与必要输出文件；leaf 调用不拥有
  长期 Run。validate data/options → generate → effect/artifact result。
- **正常/失败**：accepted chart types、optional color、Unicode/title、data values；invalid
  type/field/schema、oversized data/artifact write fault，不能返回默认空 spec。
- **已有断言**：原Core弱断言保留；新增共享真实Host精确场景：bar完整normalized Vega-Lite schema/mark/title/data/encoding/width/height，中文emoji和color=null omission；missing data返回framework_error及完整canonical detail。
- **最低充分证据**：精确 spec schema/type/title/fields/data，合法 optional omission 与
  null，失败 code/full ToolResult，artifact 引用若存在则核对 bytes/path；三语言结构一致，
  明确 spec generation 与 raster rendering 的证据边界。

- **候选运行证据（candidate_runtime_verified）**：bar完整normalized Vega-Lite schema/mark/title/data/encoding/width/height，中文emoji和color=null omission；missing data返回framework_error及完整canonical detail。
  来源：`echo-sdk-host/tests/tool_behavior_governance_e2e.rs`、三语言ToolBehaviors测试与共享fixture；
  `.echo-semantic/evidence/evidence.sdk-tool-behavior-candidate.md`绑定实测Host/源hash/原始日志。
- **仍缺或证据边界**：其它accepted chart type、invalid type/field/large数据、HTML/raster与artifact write fault仍缺；本组仅spec生成。 最后相关变更后的独立复审、正式pin和精确main仍待；
  不从整组绿色推断未列出的字段/生命周期/失败组合。

### family: media

- **入口/可达**：`facade/tools.rs` 按 feature 构造 image/pdf/excel/word/text framework
  tools；accepted 操作不是都能由 text_stats 代表。
- **权威/状态**：framework parser/tool、输入文件与输出 artifact；资源创建/写文件由
  tool 管理，Host 保留 ToolResultContent/image URL、data/effect/failure。
- **正常/失败**：PDF pages、Excel cells/formulas、Word structure、text search/process/
  export、image fetch/view；坏格式/missing file、Unicode/encoding、size、partial output。
- **已有断言**：原Core弱断言保留；新增共享真实Host精确场景：UTF-8 text_stats精确计数，unique结果/preview与export bytes；输入不变，missing input失败时目标sentinel不变。
- **最低充分证据**：为 accepted parser/export 操作准备小 fixture，逐项 expected semantic
  value/artifact 与 file bytes 对账，malformed/large input、export fault、不支持格式明确
  失败；三语言 paths/bytes/optional/content type一致，外部 viewer 非必要时不声称渲染完成。

- **候选运行证据（candidate_runtime_verified）**：UTF-8 text_stats精确计数，unique结果/preview与export bytes；输入不变，missing input失败时目标sentinel不变。
  来源：`echo-sdk-host/tests/tool_behavior_governance_e2e.rs`、三语言ToolBehaviors测试与共享fixture；
  `.echo-semantic/evidence/evidence.sdk-tool-behavior-candidate.md`绑定实测Host/源hash/原始日志。
- **仍缺或证据边界**：PDF/Excel/Word/image parser/export及其它text操作、GBK/大输入、实际图像model_content和partial output/cancel仍缺；不以text场景代表所有media。 最后相关变更后的独立复审、正式pin和精确main仍待；
  不从整组绿色推断未列出的字段/生命周期/失败组合。

### family: data

- **入口/可达**：`facade/tools.rs` → framework data/data_quality tools，CSV/file 输入
  和参数按 tool schema；Host 不重算 aggregate/join/quality 结果。
- **权威/状态**：framework dataset computation 与 file/export effects；leaf 调用，
  read/transform → result 或 export，副作用及 truncation 必须保留。
- **正常/失败**：read/filter/aggregate/profile/topn/bin/ratio/join/pivot/correlate/quality；
  missing columns、invalid types/zero denominator/missing data、bad path/size/export partial failure。
- **已有断言**：原Core弱断言保留；新增共享真实Host精确场景：CSV sort/select精确列、行序、schema/value与typed data/output一致；JSON export文件内容；缺列和无效格式typed错误后原输入/sentinel不变。
- **最低充分证据**：共享 tiny dataset 为各 accepted operation 给精确值/null/column
  期望，joint schema/Unicode/large ints/NaN 边界和无损拒绝；export 磁盘结果与 effects，
  三语言同 fixture 比对，不能仅检查 result.success。

- **候选运行证据（candidate_runtime_verified）**：CSV sort/select精确列、行序、schema/value与typed data/output一致；JSON export文件内容；缺列和无效格式typed错误后原输入/sentinel不变。
  来源：`echo-sdk-host/tests/tool_behavior_governance_e2e.rs`、三语言ToolBehaviors测试与共享fixture；
  `.echo-semantic/evidence/evidence.sdk-tool-behavior-candidate.md`绑定实测Host/源hash/原始日志。
- **仍缺或证据边界**：aggregate/join/pivot/ratio/quality等其它accepted操作、缺失/非有限值与大整数、限制/partial export/cancel及platform variants仍缺。 最后相关变更后的独立复审、正式pin和精确main仍待；
  不从整组绿色推断未列出的字段/生命周期/失败组合。

### family: statistics

- **入口/可达**：`facade/tools.rs` → framework ExploratoryStatisticsTool，columns 与
  data_path 按 tool schema（不能假定 positional 顺序与 Rust 声明一致）。
- **权威/状态**：framework descriptive statistical computation；leaf result，不能由
  SDK 追加推断性结论或改写 accepted method semantics。
- **正常/失败**：统计样本、缺失/常量/空列、columns optional、missing file/invalid data。
- **已有断言**：Core success 且 output 含 `"inference":false`；未断言均值/计数等数值。
- **最低充分证据**：固定样本精确 descriptive values、empty/missing/constant/invalid column
  结果与错误，optional omission/null 和 float/nonfinite 边界；三语言结构与 inference=false，
  禁止把全文 substring 当作全部 statistical result 证明。

- **候选运行证据（candidate_runtime_verified）**：exploratory statistics真实success且output含inference=false；未断言均值/count精确数。
  来源：`echo-sdk-host/tests/core_profile_e2e.rs#remaining_tool_families_execute_framework_tools_locally`。
- **仍缺或证据边界**：固定样本所有数值/empty/constant/nonfinite边界和三语言结果比较仍缺。 未覆盖的已接受行为须具名登记输入、结果/失败与缺证原因；不增加未定义的任意组合验收门槛。

### family: research

- **入口/可达**：`facade/tools.rs` → framework arXiv/Semantic Scholar/PubMed/clinical
  trials/PDF fetch/BibTeX tools；线上 service 请求由真实 tool client 发出。
- **权威/状态**：远端文献记录、framework parser/output，PDF 文件 artifact 为副作用事实。
- **正常/失败**：query/page/metadata/Unicode authors、BibTeX 内容；HTTP rate-limit/status/
  malformed response、timeout/cancel、PDF invalid/size/file write fault。
- **已有断言**：原Core弱断言保留；新增共享真实Host精确场景：真实loopbackHTTP PDF页数/text/metadata；503/nonPDF/malformedPDF typed错误；截断preview/fullartifact实际path、SHA、bytes、完整JSON；BibTeX字段/逃逸/文件bytes与空输入不覆盖。
- **最低充分证据**：可控 HTTP fixture 每种 accepted search 解析/result metadata/分页、
  failed status/malformed/cancel，BibTeX exact fields 和 PDF bytes/effects；如 tool 不可注入
  endpoint，记录可重复 consumer evidence 与未验证限制，不用网络不可控理由默认通过。

- **候选运行证据（candidate_runtime_verified）**：真实loopbackHTTP PDF页数/text/metadata；503/nonPDF/malformedPDF typed错误；截断preview/fullartifact实际path、SHA、bytes、完整JSON；BibTeX字段/逃逸/文件bytes与空输入不覆盖。
  来源：`echo-sdk-host/tests/tool_behavior_governance_e2e.rs`、三语言ToolBehaviors测试与共享fixture；
  `.echo-semantic/evidence/evidence.sdk-tool-behavior-candidate.md`绑定实测Host/源hash/原始日志。
- **仍缺或证据边界**：固定官方endpoint的四search provider parser/分页/取消未证明，accepted catalog无with_base_url route；provider rate-limit、write/partial/cancel及其它输入仍缺。 最后相关变更后的独立复审、正式pin和精确main仍待；
  不从整组绿色推断未列出的字段/生命周期/失败组合。

### family: content_guard

- **入口/可达**：`facade/tools.rs::dispatch_content_guard`，exact resource constructor/
  mode/check 在 `source_operations.rs`，调用 framework ContentGuard。
- **权威/状态**：framework guard mode/detection/redaction decision；Host guard resource
  owner。construct detect/redact/reject → check → close，不能把 rejected 转成 clean。
- **正常/失败**：match types/spans、clean/detected/rejected/redacted、direction/exact mode；
  unknown mode/type、Unicode spans/optional、owner/close/quota。
- **已有断言**：Core detect email nonempty matches、clean true、check variant rejected；
  source resources test no secret false。未证明 redact 文本和 Unicode span 对齐。
- **最低充分证据**：每个 accepted guard mode 的精确 text/spans/variant，中文 emoji 前后
  PII、same owner wrong resource、invalid mode 不默认 detect、close 后不可用；三语言
  redaction/direction/result fields，不把本地 guard 扩成新的线上权限门控。

- **候选运行证据（candidate_runtime_verified）**：email detect nonempty matches、plain文本is_clean=true、PII返回rejected variant；source owner/close检查实际执行。
  来源：`echo-sdk-host/tests/core_profile_e2e.rs#tool_families_execute_framework_tools_with_the_session_cwd`；`echo-sdk-host/tests/core_profile_e2e.rs#stateful_source_resources_preserve_owner_state_and_close`。
- **仍缺或证据边界**：redact精确文本与中文emoji span、非法mode不默认及三语言完整fields仍缺。 未覆盖的已接受行为须具名登记输入、结果/失败与缺证原因；不增加未定义的任意组合验收门槛。

### family: project_rules

- **入口/可达**：`facade/tools.rs::dispatch_project_rules` → framework InstructionResolver/
  load_project_rules；exact resolver resource 在 `source_operations.rs`。
- **权威/状态**：framework resolver sources/order/content 与磁盘 instruction files；Host
  resolver handle。root/path/config → load/resolve → result → close。
- **正常/失败**：hierarchy/agents_files_only、source path/order、excluded files；missing/
  unreadable/invalid config、Unicode path/content、root changes、owner/close。
- **已有断言**：Core empty workspace is_empty=true；source resolver content 包含 root/nested
  AGENTS，排除 CLAUDE 文本。未断言顺序/source metadata/错误配置和 live Agent 消费。
- **最低充分证据**：精确 precedence/order/path/source result，与 live Session prompt
  消费对账；invalid root/partial I/O failure、rule update 后行为、source handle owner/
  close；三语言 paths/content/optional 无损，不静默覆盖用户自定义规则。

- **已证实（限具体case）**：RAG/Rules三语言共享后继场景已核InstructionResolver来源优先级/path、文件动态修改、坏UTF8明确错误、目录跳过、owner/close和原文件不变；这部分由真实resolver消费，不是catalog注册成功。
  证据：evidence.sdk-rag-project-rules、evidence.sdk-rag-scalar-verification与[evidence/rag-project-rules/README.md](evidence/rag-project-rules/README.md)。
- **仍缺/不承诺的边界**：当前证明是resolver读值；rules是否自动进入Session prompt须先核是否属于已接受合同，不能从当前缺入口推不需要，也不能预设自动注入保证。

### family: testing

- **入口/可达**：accepted external contract14个field identity，均为surface=value、无method/operation；
  family operations=[]。TOOL_FAMILIES排除testing，不创建远程Mock执行器。
- **权威/状态**：三语言公共WireValue codec拥有接受payload的投影/拒绝；framework保留Rust
  Mock对象执行。纯值生命周期，不拥有Store、Run、terminal或SDK测试调度器。
- **正常/失败**：12个positive payload与7个generic scalar/container invalid case，Unicode/
  integer/duration/optional/opaque wrapper不丢字段；unknown wrapper保留不扩大接受范围。
- **已有断言**：TS/Python公共codec各19 tests真实通过；Java公共WireValues.value和
  EchoAgentClient.invoke经controlled official ACP transport按相同expected JSON执行2 JUnit
  test内12+7断言；该transport只回显值，不是Host/Mock runtime。
- **最低充分证据**：accepted scope与14个field payload对应、三语言精确投影/错误及最后source
  对账。不存在testing op的Host拒绝行为须另有实际场景；catalog done不是运行证据。

- **候选证据（candidate_value_verified）**：TS/PY19+19与Java2tests内部12+7断言零失败/skip；
  shared corpus、tests和wirecodec源hash与实测receipt仍相同。TS/PY raw dump相等，Java无raw
  transcript，只记录动态same-expected断言。归档 `evidence.sdk-testing-value-candidate`。
- **仍缺或证据边界**：最终published framework inventory/signature核对、最新独立复审和精确main；
  本组没有真实Host testing op拒绝新证据，也不证明Rust mock reconstruction/runtime或
  Mock专属validator；无serde源类型的payload包装不能当成Rust对象重建。

### family: structured_output

- **入口/可达**：`facade/structured_output.rs::validate` 由 dedicated method/accepted
  route 调用；此路径只检验 schema/sample，不执行 Agent。
- **权威/状态**：Host使用jsonschema validator判定schema/sample；当前AgentConfigWire/AgentCreate
  合同没有output schema/repair配置。accepted仅validate，不承诺Run structured输出或repair policy。
- **正常/失败**：schema compile、sample valid/errors（有界）、absent sample、too many
  args、schema byte bound、invalid wire；valid schema 不等于 Agent 最终输出合格。
- **已有断言**：Core admission 中 valid sample 的 valid=true、invalid request typed error；
  未完整证明schema over-limit/error paths。
- **最低充分证据**：false sample 的 exact errors/path、schema compile failure、payload
  bound、optional sample区别；三语言原样保留validate结果与错误。Run最终输出/repair不属于
  此已接受合同，不能作为新增验收目标。

- **候选运行证据（candidate_runtime_verified）**：合法sample valid=true和非法请求typederror在真实Host执行。
  来源：`echo-sdk-host/tests/core_profile_e2e.rs#negotiated_facade_admission_fails_closed`。
- **仍缺或证据边界**：false样本errors/path、schema超限/compilefailure及三语言validate结果缺口；无Run repair承诺。 未覆盖的已接受行为须具名登记输入、结果/失败与缺证原因；不增加未定义的任意组合验收门槛。

### family: source_operation

- **入口/可达**：`facade/registry.rs` exact accepted identity/digest →
  `_echo_agent/facade/invoke` → `facade/source_operations.rs::dispatch` 或同一 family adapter。
  ReactAgent/Agent service、SkillRegistry、file/tokenizer/guard/store 等由 receiver 调用。
- **权威/状态**：所调用 framework 对象及上述各 family 权威；Host resource只保存被
  寻址实例。source setter/getter 必须作用于当前 live Agent，而非过时 definition clone。
- **正常/失败**：exact receiver/arguments、live prompt/skills/services、constructor resource
  字段、optional、same-owner wrong-kind、bad signature/identity/feature；不能字符串猜 route。
- **已有断言**：Core admission live prompt/max_iterations/skills/permission/services、
  stateful resource/file authority/turn receipt；canonical sentinel 仅 reachability。
- **最低充分证据**：accepted route 按产品能力归组对齐本矩阵，逐 stateful setter→live
  consumer→getter/side effect、constructor→合法 method→close；所有转换字段 round-trip，
  非法 receiver/signature 无 mutation。source route 总数不成为项目完成分母。

- **候选运行证据（candidate_runtime_verified）**：指定liveAgent setter/getter、resource方法/close、文件权威和receipt投影实际执行；全canonical null-input哨兵只支持adapter reachability。
  来源：`echo-sdk-host/tests/core_profile_e2e.rs#negotiated_facade_admission_fails_closed`；`echo-sdk-host/tests/core_profile_e2e.rs#stateful_source_resources_preserve_owner_state_and_close`；`echo-sdk-host/tests/core_profile_e2e.rs#source_files_and_skill_registry_preserve_rust_authority`；`echo-sdk-host/tests/core_profile_e2e.rs#turn_receipt_source_operations_project_live_and_recovered`。
- **仍缺或证据边界**：accepted setter→真实consumer、所有sourceconstructor→合法method→close及各字段无损仍需逐能力归组，不以identity数量完成。 未覆盖的已接受行为须具名登记输入、结果/失败与缺证原因；不增加未定义的任意组合验收门槛。

### family: invoke

- **入口/可达**：`_echo_agent/facade/invoke` → `facade/mod.rs` exact registry；
  `facade.resource.close` → `FacadeState::close_resource`，所有 resource/stream 共用 handles。
- **权威/状态**：catalog 仅 addressing/signature/feature authority；HandleRegistry 管
  owner/generation，底层 framework instance 决定业务 close/cancel 是否成功。
  admit → dispatch → typed value/error；closing → awaited cleanup → unregister。
- **正常/失败**：canonical/family route 一致、resource close idempotency；wrong signature/
  unknown method/feature、wrong owner/kind、fallible cleanup、重复 close、cancel/EOF。
- **已有断言**：Core admission/unknown/signature/feature、resource close/repeated false；
  Facade quota/backpressure/cascade，integrations mock close debt retry。
- **最低充分证据**：每个 resource kind publication quota rollback、失败 close保留可重试
  owner、成功 close 后 exact handle invalid 且 slot 复用；wrong same-owner kind 不误关对象。
  三语言显式 error envelope/handle、no-value rejection、Client close/cancel 不推断终态。

- **候选运行证据（candidate_runtime_verified）**：真实unknown/signature/feature admission、resource配额/backpressure/cascade指定断言；family参数保留原opaque handle而非投影为enum；resourceclose失败保留owner的指定unit已执行。
  来源：`echo-sdk-host/tests/core_profile_e2e.rs#negotiated_facade_admission_fails_closed`；`echo-sdk-host/tests/facade_feature_adapters_e2e.rs#facade_resource_limits_match_the_advertised_bound`；`echo-sdk-host/src/core_profile/facade/mod.rs#family_handle_argument_retains_opaque_resource_address`。
- **仍缺或证据边界**：每个resource kind publication/closefault/竞态与三语言invalidowner/kind组合仍需反例与正式pin复审。 未覆盖的已接受行为须具名登记输入、结果/失败与缺证原因；不增加未定义的任意组合验收门槛。

## 三语言共同场景与不足

客户端不是第 42 个 catalog family，但它贯穿每个 accepted family。实际入口是
`sdks/typescript/src/client.ts`/`wire.ts`，`sdks/python/src/echo_agent_sdk/client.py`/
`wire.py`，`sdks/java/src/main/java/com/echoagent/sdk/EchoAgentClient.java`/
`JsonSupport.java`/`WireValues.java`。transport/feed/lease 只投影协议，不拥有 Run、Task、
Store commit 或恢复终态。各语言异步与时间单位不同必须转换，不以 smoke/API inventory 证明一致。

| 共同场景 | 当前可见断言及证明边界 | 最低充分的新证据 |
|---|---|---|
| 协商版本/digest/capability | 三语言 protocol-host 的 bad-version/bad-digest/bad-acp-version 拒绝；`sdks/python/tests/protocol_host_proxy.py` 注入真实 Host response | 最终 Host advertisement、required/optional capability、未编译 feature、unknown operation/signature；比较 typed code/detail，不只拒绝类型 |
| 无损值和错误 | `sdks/shared/fixtures/native-projection-invalid.json` 由三语言 protocol-integrity 消费；opaque handle、Unicode/special map keys、missing envelope | accepted integers/time/path/bytes/optional/variant shared corpus 实际经过 Host；native projection unsupported value 必须显式报错 |
| 多 Session/close retry | protocol-host 两 Session，one close 后 two invoke；Agent/Session/run/unregister 的代理错误后重试 | Host 自身失败 close/cleanup debt 的真实错误及 retry，挂起 Run/stream/MCP 下多 Session 隔离、资源结算；比较 Run close retry 的语言 timeout code 差异是否合同规定 |
| gap/ACK/replay/live | TS `sdks/typescript/test/gap-ack-host.test.js`、Python `sdks/python/tests/test_gap_ack_host.py`、Java `sdks/java/src/test/java/com/echoagent/sdk/GapAckHostTest.java` 共用 fixture，snapshot/page ACK 后只消费更新事件 | 正式 pin/binary 各场景 zero skip、record sequence/watermark/journal identity，live consumer ACK 而非收到即 ACK、重启/terminal/error 顺序 |
| callback 异常/取消 | protocol-host 非 streaming LLM callback 抛异常后 terminal.status/receipt.outcome 都 failed；protocol-integrity 部分 callback/factory unit tests | Tool/LlmClient/Store/hook/accepted callback types，async reentry、timeout/cancel/disconnect/late result、stream pending ACK；副作用与 lease释放对账 |
| Task/Subagent 并发 | Rust场景及三语言各4个同DAG/control场景已执行，4项getter消费另有真实组合 | 三语言同一 revisioned DAG、exact claim control、result context、stale update/duplicate execute、多 Session，framework authority 与 SDK receipt round-trip |
| 持久化/重启 | Core Run/history recovery 与 Bridge managed Store lost ACK/restart 有源码场景；指定候选已运行；正式 pin/main 证据仍缺 | 三语言同一 commit outcome unknown→retry/restart receipt、process kill→interrupted、journal换代与 handle generation；确认真实持久化副作用与无重复 commit |
| feature family | Rust family 正常/部分失败 assertions，本矩阵逐项区分 | 接受的 language API 调用同一 Host 场景，result/error/events/effects canonical 对比；选值或 remote边界后仍保留每项 family 的行为证据 |

每种语言的 Host tests 在缺 `ECHO_AGENT_SDK_HOST` 时会跳过；final gate 必须确认该变量
指向本次最后相关改动后的 full-facade binary、binary hash、完整测试摘要和零 skip。
proxy 注入只证明客户端对损坏/失败 transport 的反应，不证明 Host 真实 timeout 或 cleanup。

## 后续验收交接要求

1. 稳定正式 framework successor/pin 和 SDK 源码，先记录所有 family 当前状态及 open
   Finding 对应场景；不以编译通过改写行为 `verified`。
2. 将上述最低充分场景对应到真实 test assertion；明确输入、结果/error、唯一权威、
   事件/副作用、资源清理、durability、time/concurrency 与 feature 八个视角。
3. 在统一 Cargo 窗口和当前二进制上运行 Host 及三语言场景，记录命令/退出码/source SHA/
   binary hash/feature profile/log；旧 pin red/green 与 target green 分开保留。
4. 每个 family 缺口得到修复、证据或明确合法范围处置；接受合同内未验证行为不能静默
   deferred。完成独立反例复审及最后变更后的验证，再进行精确 main 验收。

本矩阵的 coverage/path 校验只证明 41 family 没有在文档中遗漏、所引证据入口存在。
它不能证明 Host 实现正确、测试通过、语义 Snapshot 闭合或 main 已交付。

## 本轮有限审查处置

本表后继事实校准以4feb冻结参照及各自原运行证据为准，之后新增验证单独记录，不将本次文稿变更冒旧快照已包含。41family的可交付审查单元与明确剩余原因见[能力处置](capability-review-dispositions.md)。原fc63计数/指纹已转历史收据，其归档Evidence不再直接supports当前全部family。正式Workflow48/Task4/StateTrace9/Memory59/MCP4和临时compute45分别绑定自己的来源；新ACK/Channel和189ff后继组合由专属反例/复审归档。建模mapped不自动等同运行verified或所有lenscovered。
