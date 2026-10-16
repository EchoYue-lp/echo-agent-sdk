# SDK 全能力治理能力图

当前验收与阻断状态统一见[当前有限验收](capability-review-remaining.md#当前有限验收2026-10-17)。本页只维护其独有能力结构、逐项处置或历史账本；下文历史切点的计数、证据与义务保留。

## 前一阶段文字与证据（历史，原正文保留）

以下“当前/待回归/open”均是原阶段文字；现在以前述状态和Finding对象为准，原日志/sourcecut/限制不改写。

SDK 从30d7开始治理，当前已精确依赖已发布框架 `a6346643`，没有本地 path patch。此前 `76601` 正式证据切点的14条Rust及官方完整语言入口均已通过并独审：790项输入前后同，TS308、PY375+18、Java332/71XML和3quickstarts；原失败链保留。该结果不覆盖随后 Session/Run 意图A的实施、远端流原因修复、live journal 与A2A新增测试后的最终组合门禁，最新整体验收仍待。已具名审查按各组有限 `examined/mapped` 记录，41个Behavior与全局风险视角不自动verified，inventory/behavior闭合仍open；12项合同迁移和5秒B仍待决定，A已批准实施不再列成待选择。

## 当前与历史证据 (2026-10-14)

原 be5/290/368+18/320/759/bcb 属历史切点，不作当前待办或新源码通过证据。

- [a634 完整语言原切点](https://github.com/EchoYue-lp/echo-agent-sdk/blob/0ebe15f66d0aa57c7b7a92ae68cd1be106114ae9/docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/README.md)
- [a634 Rust14](https://github.com/EchoYue-lp/echo-agent-sdk/blob/0ebe15f66d0aa57c7b7a92ae68cd1be106114ae9/docs/sdk/governance/evidence/sdk-a634-rust-gates/README.md)
- [Task 3语言原行为](task-subagent-evidence.md)
- [源码可恢复性](semantic-source-provenance.md)

## 数据流与唯一权威

`language client → official ACP connection → profile/accepted catalog dispatch → thin Host adapter → framework authority → typed response / committed event / owned resource`

Framework拥有Task/DAG、取消、恢复、Store commit和terminal。Host拥有地址/owner/generation、反向调用准入、delivery窗口和资源结算；三语言拥有消费/transport投影。本文和语义图均为审查材料，不成为运行时第二权威。Catalog done只证明声明映射。

## 原模型与历史证据区别 (be5 historical)

组级主路径mapped表示入口、唯一权威、生命周期与具名失败义务已建模；41个family Behavior仍needs_review，开放Finding及最终交付分别保留。旧候选的八crate临时patch、95ec/b909 pin及各Host运行保留原epoch；当前be5的Rust和Chart共享61语料已核，完整语言同一官方入口已实际退出0，94项归档和独立复审完成，完整合同生成仍被12项兼容选择阻断。框架main已交付不替SDK精确main/CI。完整Rust inventory仅遥测；accepted字段必须由唯一生成器明确裁决。

历史fc63 TS219/Python279+18/Java233已原字节归档；原6711候选完整运行是Rust418、TS240/Python300+18/Java264，Java2bad源有精确收据。但Workflow后继生产变更后这些原结果不证明当前全套，必须使用各后继来源/日志。

历史诊断Host4d436b60…实际Hostlib136/core31/bridge34/facade12/标准stdio4均零失败；29request共同场景绑定fc63，42diagnostic共同场景绑定4d43。证据分别见 `/tmp/sdk-diagnostic-host-validation.json`、`/tmp/sdk-lsp-language-proof.json` 和 `/tmp/sdk-lsp-diagnostics-language-proof.json`；最终验收须保存精确源码/二进制/日志，不能改observed摘要重用旧运行结论。

## 历史能力展开记录（按原运行切点保留）

以下各能力主体保存编写时的源码、当时待办与原证据范围；其中“新pin待构建”“Channel未实现”“consumer仍open”等句子是历史阶段，不是当前待办。当前事实以页首验收状态和[逐能力处置表](capability-review-dispositions.md)为准，原日志及历史Evidence不改写。

## ACP 与协商

`lib.rs::run_stdio_with_close_owner` / factory 组合一个官方ACP connection；core_profile::attach只在该连接注册扩展。编译features与accepted hello/digest校验是协商来源，不通过运行时配置伪造能力。关闭owner保留原actor到framework receipt/cleanup，并实际join actor，panic/abort/config无runtime明确失败。诊断Host4d436b60…的标准stdio4、Hostlib136/core31/bridge34/facade12均通过；plain标准入口有直接证据，不再表述为全部未执行。

仍缺：标准ACP已有四项真实stdio场景。尚待最终无patch published pin独立构建，以及已接受协议版本、startup/EOF错误签名与平台关闭约束的精确对账；不要求未承诺的平台组合。

实际入口/测试锚点：

- `echo-sdk-host/src/lib.rs#run_stdio_with_close_owner`
- `echo-sdk-host/tests/stdio_e2e.rs`
- `echo-sdk-host/tests/core_profile_e2e.rs#extended_standard_prompt_bridges_shared_core_handles`

## Agent、Session、Run

Agent/Session/Run handler复用framework规范执行、receipt与取消。HandleRegistry唯一管理owner/generation/idempotency/配额；创建及关闭admission、durable receipt slot、失败写入重试和Agent cascade已有候选反例。Session create/load读取持久原生身份并调用framework reserve_recovered_turn_ids，turn7后分配turn8；禁用同一调用得到turn1的两路径红证据已取得。活Run关闭保持同一个官方actor，不以watcher替代join/receipt。真实kill恢复Interrupted；并行Session取消A不改变B。

仍缺：Agent cascade/cleanup retry仍由具名Finding承接；Session closing读取准入与owned Channel回调新窗口须各自最终反例/复审。framework parent-drop修复使用自己的后继证据，旧23项通过不替后继。

实际入口/测试锚点：

- `echo-sdk-host/src/lib.rs#manual_close_keeps_active_run_actor_until_cleanup_and_receipt_settle`
- `echo-sdk-host/tests/core_profile_e2e.rs#explicit_session_create_reserves_persisted_turn_identity_before_new_run`
- `echo-sdk-host/src/core_profile/handler.rs`
- `sdks/python/tests/test_parallel_sessions_host.py`

## Task 与 Subagent

factory中的同一个TaskRevisionService供应CRUD及FacadeTaskController；controller仅转换metadata、claim/dispatch并调用framework服务，不拥有frontier或第二DAG validator。execution CAS preflight在副作用前拒CRUD-only后端；exact pause/cancel绑定canonical attempt，独立sibling继续，Completed result含absent/null区别。Subagent单collector保留join结果，短await超时后可再次await，不提前耗尽句柄；runtime失败经task/list保留origin_operation/cause。Hostcore31/bridge34中这些具名场景已执行。

仍缺：正式三语言各四项DAG/control/短await场景已有证据；canonical result/执行次数增强与callback结算后继源独立归档。剩余已接受claim/context字段和framework parent-drop修复按各自具名义务对账，不扩大Task API。

实际入口/测试锚点：

- `echo-sdk-host/src/factory.rs`
- `echo-sdk-host/src/core_profile/facade/task_runtime.rs#claim_task`
- `echo-sdk-host/src/core_profile/facade/subagent.rs#collect_result`
- `echo-sdk-host/tests/core_profile_e2e.rs#task_cancel_is_exact_and_pending_sibling_continues`

## 事件、ACK、replay、持久化

framework EventJournal唯一拥有committed envelope、journal identity和terminal；StreamDelivery只管理发送窗口与cursor。#120候选修复把snapshot/page已消费ACK的coverage推进到所覆盖水位，再续发严格更大的sequence；成功enqueue后才授权ACK，重复/未来ACK不退回水位。12个event unit包含真实segmented journal轮转/prune、oversized gap、single/multipage coverage、空future page及publish失败不扩大授权。真实三语言count/oversized、延迟external replay ACK两窗口、terminal replay ACK自然EOF均执行，并比较journal union/event_id/唯一terminal；不是旧tiny-window ACK后即退出。

仍缺：ACKv4 retention连接/实际liveinterval/pending失败回滚已真实反例与独立exactslice通过；提交消费局部Finding resolved，Hostwatermark/events完整scope及新红绿已独立PASS，本地bugresolved，最终交付单列，物理prune不是三语言E2E。

实际入口/测试锚点：

- `echo-sdk-host/src/core_profile/events.rs#real_journal_retention_page_authorizes_only_its_returned_watermark`
- `echo-sdk-host/src/core_profile/events.rs#replay_publication_failure_and_empty_future_page_grant_no_ack_coverage`
- `echo-sdk-host/tests/core_profile_e2e.rs#consumed_multiple_replay_page_ack_resumes_live_after_page_cursor`
- `sdks/shared/testing/gap_ack_fixture.py`
- `sdks/typescript/test/gap-ack-host.test.js`
- `sdks/python/tests/test_gap_ack_host.py`
- `sdks/java/src/test/java/com/echoagent/sdk/ExternalAckHostTest.java`

## 资源、流、bounded transport

统一Registry验证资源kind/owner/generation并保留可重试cleanup owner；typed Handle参数仅投影完整DTO后回到原resource parser/Registry，不转换任意process-local值。stream唯一collector记录真实producer成功/错误，panic/abort/EOF不伪Complete；初次recv感知cancel，未交付terminal不提前消费sequence。accepted cancel与terminal commit在Registry锁下原子决定先后；failed cleanup/caller abort保留owner。quota/pending reservation、MCP同名exact Arc、manager关闭派生LSPclient不复活已有候选units/真实Host证据。

仍缺：producer/next/cancel/terminal原子scope已局部resolved且独立复审；cleanup retry与typed handle projection具名Finding保留。Agent cascade和资源发布回滚按现有明确合同分别审查，不借stream局部通过宣称全资源无缺陷。

实际入口/测试锚点：

- `echo-sdk-host/src/core_profile/facade/stream.rs#accepted_cancel_between_snapshot_and_terminal_commit_returns_cancelled`
- `echo-sdk-host/src/core_profile/facade/stream.rs#cancelled_pull_during_failed_cleanup_preserves_error_and_terminal_sequence`
- `echo-sdk-host/src/core_profile/facade/mod.rs#family_handle_argument_retains_opaque_resource_address`
- `sdks/typescript/test/lsp-host.test.js`

## 扩展桥

extension register/unregister经descriptor/capability准入，反向call复用官方connection lease/deadline；Tool/LLM/Store/hook/callback由framework消费并决定terminal/commit。initial response前RAII stream reservation避免超时泄漏；cleanup/interrupted/terminal callback使用connection作用域，不因Run取消丢清理。Factory先reserve官方child handle及factory_origin，再publish/rollback，同Factory可创建独立Session实例。managed11 typed DTO/TaskStore execution-CAS显式opt-in，真实deadline-before-work、lost ACK/断连重启同receipt保留commit权威。Bridge34候选通过，只证明具名调用；Java factory cleanup并发/失败future/transport owner的原2bad源/264测试与独立pass保留具体epoch；当前Channel新增ownedentry不继承该scope结论。

仍缺：managed Store的11个DTO与deadline/结果恢复已有有界证据，剩余真实caller/operation审计仍明确open；Channel MessageHandler原缺口已用户批准保承诺与forward范围；当前设计/实现未完。reverse mailbox/child factory/取消重入按具名Finding审查，不用任意组合扩大门槛。

实际入口/测试锚点：

- `echo-sdk-host/src/core_profile/extension_bridge.rs`
- `echo-sdk-host/tests/extension_bridge_e2e.rs#managed_commit_disconnect_restart_replays_same_receipt`
- `sdks/shared/testing/managed_persistence_backend.py`
- `sdks/java/src/test/java/com/echoagent/sdk/FactoryCleanupLifecycleTest.java`
- `sdks/java/src/test/java/com/echoagent/sdk/LateFactoryHostTest.java`

## 集成能力与精确 source invoke

CompiledOperationCatalog按确切accepted identity/signature/feature寻址，薄adapter调用live framework service。memory/workflow读取当前Agent服务，mixednamespace/配置错误在副作用前拒绝；Delivery携完整claim而不以id重建当前attempt；trace构造验证后发布，quota失败可复用。MCP config/file批量SDKalias原子发布保原best-effort native连接权威及exact Arc+Session；工具投影完整ToolResult/effects/artifact/canonical cause。LSP真实29request场景+manager生命周期有30观测/44原始请求；新诊断另42观测/42原始通知跨三语言全字段匹配，severity缺失与integer/text code无损、每URI错误/恢复/清空隔离、无URI走原runtime fatal/PID退出。两份证据各绑定fc63/4d43不同不可变Host，不互冒同一final输入。

仍缺：Workflow48行覆盖17/24已接受操作，原剩7项已有0744后继真实消费者证据；Memory/Permission59行、State/Trace/Delivery9项、MCP4项、LSP29请求/42诊断分别有来源。testing14字段本地值已有19行语料，不是RustMock runtime；Guard/Pivot189ff及Tool新validator各用后继运行/复审，不沿用旧whole通过。

实际入口/测试锚点：

- `echo-sdk-host/src/core_profile/facade/registry.rs`
- `echo-sdk-host/src/core_profile/facade/integrations.rs`
- `echo-sdk-host/src/core_profile/facade/tools.rs`
- `sdks/typescript/test/lsp-diagnostics-host.test.js`
- `sdks/python/tests/test_lsp_diagnostics_host.py`
- `sdks/java/src/test/java/com/echoagent/sdk/LspDiagnosticsHostTest.java`

## 三语言与无损值

TS/Python/Java经官方ACP transport消费同一accepted合同；严格value/error投影保留大整数canonical text、Unicode/path/time/bytes、handle/operation/retryable/details。ACK只在已消费或显式处理snapshot/replay coverage后提交；提交失败/取消不写“已成功ACK”，gap不autoACK，迟订阅有界feed与terminal replay自然EOF已有真实场景。close保同一transport/cleanup owner，不伪关闭。LSP原29request及后继42diagnostic共同语料比较结果、完整typed错误和真实请求/通知副作用，均通过。

仍缺：native值、错误、ACK、关闭使用实际consumer与对应官方transport证据；新retention/pending ACK窗口须匹配修复后源。最终三语言完整门禁、精确生成签名和published pin独立待验，不以旧whole counts替当前源。

实际入口/测试锚点：

- `scripts/check-language-sdks.sh`
- `sdks/typescript/src/client.ts`
- `sdks/python/src/echo_agent_sdk/client.py`
- `sdks/java/src/main/java/com/echoagent/sdk/EchoAgentClient.java`
- `sdks/typescript/test/lsp-host.test.js`
- `sdks/python/tests/test_lsp_diagnostics_host.py`
- `sdks/java/src/test/java/com/echoagent/sdk/FactoryCleanupLifecycleTest.java`

## 逐能力、Finding 与 deferred

具体family与最低充分断言由 behavior-evidence-matrix.md 及41个独立family Behavior承载；本组图复用其关联，不重建第三库存。新Channel入站Finding已补入本地账本；全部Finding对象保留其状态及历史关联；数量只是本次文件盘点，不声称GitHub实时Issue数量。已修候选不自行resolved，必须归档repair/verification/rereview与精确交付。deferred由deferred-capabilities.md按能力分组，最终全inventory更新后重核，不扩到每个Rust identity三语言API。

## 风险视角与未闭合项

八风险视角继续needs_review；真实入口/注册/调用及直接反例逐项复核。前阶段framework77de已交付，后继Channelframework最终revision与SDK无patch独立构建仍待、最终accepted/deferred生成裁决、已接受合同的具名失败义务及适用平台约束、Java cleanup与framework父drop各自后继证据、最终独立复审、网站clean-source同步、CI与精确main。npm/PyPI/Maven发布仍属独立工作。

## 当前收据审计

历史 Evidence 的原声明、27个文件hash、具名测试body和fc63原始日志保存在 `evidence/sdk-fc63-historical-20261013/`。原6711日志在 `evidence/sdk-parent-driver-20261013/`，其中55个运行输入的Workflow与tools.rs出现后继变化，不能继承旧全套绿。新Channel MessageHandler consumer静态Finding明确开放：注册/start/send/stop不证明handle/handle_stream/reply可消费，不造新接收wire。41family仍needs_review、8组主路径mapped，snapshot只说明观察来源。

## 后继候选的逐项范围

Workflow正式48条记录已对f5c72源码/cb9Host执行，17/24已接受操作、11预期错误与完整checkpoint/state/lineage断言，七项操作仍未覆盖。Task/Subagent临时三语言四场景对6711Host执行并完全匹配，保留claim/revision/结果关系；尚未纳入正式门禁，也不证明完整context/恢复。工具簇28调用+一次close与testing14值字段由各自有限Evidence承载。Channel入站缺口使用静态调用路径证据，不伪称runtime红或假新receive API。

## 后继有界结论与新缺口

Compute50/7bbd三语言各1正式test/50calls/19typederrors、9oracle及189ff五Pivot边界已限定独立pass；对应六family和invoke投影，不称files API全测。Managed retirement三语言各2+backend20实际验证第八callback与Unsupported无突变；其余三consumer处置仍有限开放。LLM/Workflow0744三组/语言与26raw独立pass证明原raw消费者/完整LLM值/取消/finally，剩7Workflow verbs已可达；Python typed AgentComponentCall丢Hostissuedstream已有专属red/5green/独立PASS，本地resolved，不借旧raw绿。

Channel capturedstart/resourceclose的0744 scope已真red→3units/Hostlib140/三语言各5且独立pass；7bbd早期五场景单列。用户已批准ADR0035三forward行为及必要streamcontrols，原consumerFinding继续open planned至真实实现/生成/取消/owner/stream与三语言证据完成，不再等待产品选择。组级mapped与family needs_review保留；最终whole来源不能沿任一旧focusedepoch冒绿。
