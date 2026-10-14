---
title: Channel MessageHandler 前向消费与原始 owner 闭合
artifact: design
carrier: markdown
---

# Channel MessageHandler 前向消费设计

## 问题与目标

用户于 2026-10-13 明确批准保留 `MessageHandler::{handle,handle_stream,reply}` 已接受承诺，并增加三个前向触发操作；裁决来源为 [channel-inbound-contract-decision.md](../../../sdk/governance/channel-inbound-contract-decision.md)。本设计形成一个结果：TypeScript、Python、Java ChannelPlugin 可以经真实 Host 调用原注册 entry 的 handler，完整消费单次、流式、reply 行为及失败与关闭。

现 `ExtensionChannelPluginProxy.start` 保存 framework handler，只向语言插件传 `handler_id` 文本；reverse `extension/invoke` 是 Host→语言方向，不能替代语言→Host consumer。现 `channel_inbound_from_wire` 尚未进入前向主路径且把无法解析的 timestamp 默认为零。注册只检查 handler 名称，`channels.manager.start` 可以另传 handler。三项原承诺保持 accepted，Finding `finding.sdk-channel-message-handler-consumer` 保持 open，直至真实行为、独立复审和最终交付齐备。

## 目标行为

插件完成注册与成功 start 后，调用方提供 Session handle、完整 manager resource handle、channel_id 和 typed message。Host 只调用该 manager 原 entry 当前已成功启动的 handler。未启动、starting、start 失败或回滚、stopping、closed 的 entry 均不接受消息。start callback 内的前向调用在 starting 阶段明确失败，避免形成等待 start 完成的循环。

`handle` 返回原 handler 的 OutboundMessage；`handle_stream` 返回受原 Session/manager/entry generation 约束的 stream handle，语言通过 channels 自身 next/cancel/close 消费；`reply` await 原 handler.reply 并返回明确成功或原错误。stream 不隐式调用 reply，调用方需要投递时显式调用 reply。plugin.send 保持已有独立语义。

所有成功和失败通过现有 `FeatureOperationResponse` / `EchoSdkError` 返回。不得拿注册、send、stop 成功代替 handler 消费，不得只模拟 reverse callback 而没有 forward 原生 consumer。

## 范围与非目标

接受范围是三个 handler 操作及其流消费所必需的三个 controls，共六个 `channels` family operation。没有第四个 handler、第二个 wire method、第二个 handler registry/store/run/executor，也不把新增 framework wrapper、receipt、私有 helper 或完整 Rust inventory 自动纳入跨语言承诺。

本切片沿用 SDK 既有 full-governance 目标，不能退役或无限延期已批准的 handler 义务。EKO GUI/TUI、provider 全排列、第三方真实 IM 账号、发布 SDK 包和跨产品 UI 策略不属于此单一结果。文档与 examples 必须随公开契约更新；website 若仅展示安装/品牌而无此 API，交付说明记录检查结果与不适用原因。

## 系统边界

| 层 | 唯一责任与复用 |
|---|---|
| framework 通用机制 | 原 ChannelEntry/ChannelHandle 的 plugin、handler、successful-start generation、调用准入、取消、stop/close 和失败重试；复用 ChannelSession、SessionGeneration、SessionStreamReceipt 的生命周期 fence 机制；这些能力不依赖 EKO 或 SDK。 |
| SDK 适配 | closed family table、typed DTO/WireValue 转换、Session/resource/stream owner 与 generation 校验、现 HandleRegistry 配额、FacadeResourceRecord.lifecycle 许可、FacadeStreamRuntime producer collector、ACP 请求和 reverse callback。 |
| 产品策略 | EKO reviewer、worktree、UI 投影、用户交互策略留应用。本功能没有新增 EKO policy，也不加本地桌面权限门控。 |

framework 计划 API 为 `ChannelHandle::{handle_message,handle_message_stream,reply_message}`，各接受原消息及该调用的 `CancellationToken`，在既有 owned entry 上扩展，不能暴露裸 handler Arc 让 SDK 脱离 entry 生命周期长期调用。流返回 `ChannelMessageStream`：它包含 owned `BoxStream<'static>` 和同一次 admission 的 child `CancellationToken`，提供 Stream 与 `into_parts`；generator 捕获原 handler Arc 与 invocation lease，在其内部创建 trait 的借用流并驱动它。上层 adapter 在自己的满缓冲 send/next 上监听这个精确 token，entry stop 便能解除阻塞并阻止旧 item；不能另 query 当前 generation token 导致 TOCTOU。此 Supporting representation 只属 Rust/Host，不自动扩展 accepted identities。禁止 transmute、泄漏 read guard 或提前释放 lease。单次调用也由原 generation 跟踪的 driver 持有 lease，caller drop 只发取消而不丢弃真实 cleanup owner。entry 的 generation 与 successful-start admission 属于既有 Channel 生命周期，SDK 只检查和注入 owner metadata，不维护另一状态机。

SDK manager/extension/stream 关系仍由 HandleRegistry 唯一管理。start 传入的 MessageHandler 完整 extension handle 必须与每个注册 plugin 原声明绑定一致；相同 implementation_id、不同 generation 也不相等。batch start 在任何 plugin start 副作用前检查全部绑定；一个不匹配就整体拒绝。注册绑定可以是原 ChannelManagerRecord 中的薄 metadata，不能成为第二 handler 执行或关闭 owner。现 proxy 中只为 plugin.start 保留的 handler 副本不得成为 forward authority。

## 核心结构与数据流

所有操作经现有 `_echo_agent/channels/op`，请求仍是 `FeatureOperationRequest`。`handle` 字段为原 Session WireHandle；`signature_digest` 必须匹配唯一生成 catalog；arguments 精确如下，禁止多余参数、raw future、任意 handler selector 或 schema-free opaque message。

inactive、starting、start rollback 及 handler binding mismatch 在任何 reverse call 前返回现有 `InvalidValue`；已关闭或旧 owner/generation 使用现有 `ClosedHandle`。feature 未编译仍用现有 `FeatureUnavailable`。不存在新增错误码或默认成功。

| operation | arguments 的 WireValue 形状 | response.value |
|---|---|---|
| `channels.handler.handle` | `[Handle(manager), String(channel_id), Map(ChannelInboundMessageWire)]` | `Map(ChannelOutboundMessageWire)` |
| `channels.handler.handle_stream` | 同 handle | `Handle(stream)`，kind=Stream、family=channels |
| `channels.handler.reply` | `[Handle(manager), String(channel_id), Map(ChannelOutboundMessageWire)]` | `Null` |
| `channels.stream.next` | `[Handle(stream)]` | 现 FacadeStreamEvent variant：item/complete/failed/cancelled，保留 stream、sequence；item.value 为 outbound map |
| `channels.stream.cancel` | `[Handle(stream)]` | `Bool`，表示现 registry cancellation 是否改变 |
| `channels.stream.close` | `[Handle(stream)]` | `Bool`，沿用现 close 幂等、结算与 tombstone 语义 |

六项同时进入 `FacadeFamily::Channels.operations`、同一 `family_operation_wire_schema`/signature digest 和唯一 catalog 生成链。catalog 支持的 generic facade invoke 只能解析到同一 channels handler；family equality 不变，不借 `workflow.stream.*` 绕过 family guard。三个 controls 复用 `stream::dispatch_control`，不存在平行控制实现。

Typed inbound 字段为 `channel_id,sender_id,chat_id,chat_type,text,message_id,timestamp,attachments`；没有虚构的 `sender`。outbound 为 `channel_id,to,chat_type,text,reply_to,attachments`。chat_type 是 closed direct/group，附件为 `kind,data_base64,filename`，kind 严格为 image/file/audio/video。Unicode 文本不按字节切片；base64 必须有效且字节无损。timestamp 是现 WireU64 的规范十进制字符串，覆盖 `18446744073709551615`，拒绝 JSON number、负数、前导零、溢出、缺失和 null；转换失败明确报错，不默认零。

消息 map 保留 `deny_unknown_fields`：未知字段拒绝。required 字段缺失或 null 拒绝。`attachments` 缺失表示空列表，显式 null 拒绝；`filename` 与 `reply_to` 缺失或 null 均表示 None，空字符串保留 Some("")，不得把空字符串吞掉。message.channel_id 必须与所选原 entry/channel_id 一致；sender_id/chat_id 是 inbound 的原始会话坐标，按 framework 既有 canonical identity 规则验证，不改写为 Session owner 或缺省 unknown。reply 的 channel_id 同样匹配 entry。

数据流为语言 ChannelPlugin → channels/op admission → 原 framework ChannelHandle → 原 MessageHandler → 原生 typed reverse callback（语言 handler 时）→ typed result → 原 FacadeStreamRuntime 或单次 response。HandleRegistry 在许可等待前、许可获取后重复校验同一 owner/generation/closing；start 和 handler 绑定也重复核对可调用的 current entry generation。注册 metadata 与 manager map 的锁仅用于查找/capture，不能跨 callback await。

## 生命周期与异常场景

单次 handle/reply 的调用 owner 保留原 entry 及 handler generation 至原调用结束。单次取消只作用于该 invocation/stream 的 child token，不能关闭整个 handler generation 的准入或影响独立调用；Channel stop、resource/Session close 才关闭该 generation 准入。调用受现 ACP invocation 和 framework cancellation lifetime 管理；waiter drop 不等于外部 Agent/Run 已结算。自定义 legacy handler future 被取消并 drop，只证明本地 invocation 已停止，不能伪造外部 terminal receipt。取消 A 后独立 B 继续消费必须有真实回归。

stream 配额在副作用前由原 HandleRegistry 预留。setup、转换、spawn/attach 或 handle 发布失败时，原 close owner 取消 producer、等待真实 Join/receipt，并回滚保留的 quota/address；未结算 debt 必须仍可重试寻址，不能直接丢弃记录。producer 持有原 entry generation 与 resource 的 shared admission permit，经 capacity=1 的现 bounded sender 发布 outbound items。每次阻塞 setup、next、send 都必须响应 cancellation，不能只在外层 while 检查。

`channels.stream.close` 先取消同一 stream 并关闭新消费准入，再 wait 原 collector；collector 记录实际 JoinError/producer exit 与 pending failure，禁止把 future.cancel 当结算。慢 next、无人消费及满缓冲下 close 都可解除等待，真实 callback lease、producer receipt 和 resource permit 释放后才 tombstone。

Channel stop 必须先对原 entry 当前 generation 关闭 handler 准入并发出 invocation/stream cancellation，再等待 entry write permit，最后执行原 plugin.stop、handler.close；不得先等 write permit 后才取消被 read permit 卡住的 producer。SDK resource.close / Session.close / disconnect 同理：先关闭 registry 准入、对所属 streams 发取消并保留 collector，再等待 resource exclusive permit、原 framework cleanup 和真实 streams 结算。这个顺序扩展既有关闭机制，不能另建 SDK close 状态机。失败或 caller 取消 close 保留同一个 retry owner，不重新开放准入；late exit 仍由原 collector 归档。

stop 后 restart 使用新 entry lifecycle generation；早先 capture 的 future/stream 不能在新 handler 上恢复执行。start 未成功或被取消后保留的 handler 仅是 cleanup debt，不能据 handler.is_some 判定可接消息。handler.close 失败保留原 owner 至 retry 成功，不重复已确认 plugin.stop。

callback 可 await 同 manager `ids/list` 元数据，这些读取不获取 invocation shared permit，不在锁内 await。callback 同步等待自身 mutation、stop、resource close 或同一 handler 的递归 lifecycle 调用不属于支持目标；必须文档说明，不能把 read→queued write→read 的循环宣传为安全。

## 关键取舍与业界依据

[Tokio shared-state](https://tokio.rs/tokio/tutorial/shared-state) 用短查找锁与原对象所有权区分注册与长操作。本设计延用当前 owned ChannelHandle，避免 manager guard 跨 reverse callback。官方 [Tokio RwLock](https://docs.rs/tokio/latest/tokio/sync/struct.RwLock.html) 明确公平队列下持 read、排 write、再求 read 的死锁风险，因此 close 必须先发 cancellation，metadata 重入绕开长操作许可。

[MCP cancellation 2025-06-18](https://modelcontextprotocol.io/specification/2025-06-18/basic/utilities/cancellation) 将取消通知与执行停止处理分开；[Tokio graceful shutdown](https://tokio.rs/tokio/topics/shutdown) 区分 signal 与 wait。结合 framework 原 [SessionStreamReceipt](https://github.com/EchoYue-lp/echo-agent/blob/95ec66c6083a8feb17a1c28de0d01e1a24ca8a28/echo-integration/src/channels/session.rs) 的 receipt 设计，本项目以真实 producer/handler settlement 为释放依据；通知送达或 waiter drop 不能证明 Run terminal。

候选一是退役三项承诺，用户未选择；候选二是 SDK 裸 Arc handler registry，失去原 entry generation 与 stop ownership；候选三是扩展原 framework entry 并用 SDK 现 typed bridge/stream adapter，本设计选择三。选择理由是通用调用与取消属于框架，而 cross-language wire 和 resource 地址属于 SDK。

## 复用与实现约束

全域概念搜索已核对 framework `echo-integration/src/channels/{manager,session,types}.rs`、SDK `methods.rs`、`facade.rs`、`extension_bridge.rs`、`facade/{integrations,stream,mod}.rs` 和 `handles.rs`。已有定义、注册与真实可达相互区分：DTO/handler proxy 已有，forward consumer 尚无；ChannelHandle owned 生命周期已有，handler invocation API 尚无；stream controls 已有实现，channels table 尚无 aliases。

标准库 Arc/RAII 和现 Tokio RwLock/CancellationToken 足够表达所有权；现 futures/async-stream、FacadeStreamRuntime 和 ExtensionBridge 已覆盖 transport/stream，不加依赖、不复制 scheduler、执行器、store、配额或 receipt ledger。最小自定义扩展仅原 entry 的成功启动准入/调用 scope 与 thin family operation adapter。第三方 SDK 框架 API menu 不以 EKO 是否调用判死。

新 framework wrapper Rust identities 默认 Rust/Host-only。三个原 canonical `echo_agent::channels::MessageHandler` method 保持 acceptance，不借新 wrapper 扩大义务。若最终 rustdoc 暴露新的必要 typed representation，先核对 defining identity/kind 并在 ADR 独立 exact section 明确接受，再由 full generator `--accept-source <kind> <identity> <ADR#section>` 更新；没有源码精确证据不传该 flag。六项 protocol-native family operations 的接受源是本设计和 ADR 0035，进入同一 production generator table；不得手改 accepted JSON、复制另一个 producer 或仅用 candidate schema-only 作为完成证据。

## 验收标准

以下十二组场景必须绑定最终源码 hash/revision；真实 Host→三语言测试覆盖这一个新 consumer，framework/Host focused 并发测试补充内部 lifecycle，不声称 provider 全排列或完整 Rust identity parity。

1. single handle：三个语言各自通过真实 forward 调用原 handler typed callback，记录输入、entry identity 和 outbound 无损结果。
2. handle_stream：三个语言实际消费至少两个不同 outbound items、精确顺序及唯一 complete；reply 不被隐式调用。
3. reply：实际原 handler reply 被调用一次并收到 typed outbound；明确 null 成功和原失败保真。
4. 两个 Session 与相同 channel_id：完整 owner/generation 隔离，跨 Session/旧 manager/旧 stream handle 在 callback 前拒绝。
5. start binding：同名不同 generation、不同 handler、批量任一不匹配均在所有 start 副作用前拒绝；unstarted/starting/failed rollback/stopping 状态拒绝入站。
6. typed fidelity：中文/emoji、最大 u64、四附件 kind、二进制 base64、空和缺失/null optional 全字段 round trip；malformed/unknown/missing required 不调用 handler。
7. quota 与 setup/publication 失败：reserve 不超配额，取消并等待真实 producer，成功回滚或留下可重试 cleanup debt，没有 detached handler。
8. slow next/满缓冲/无人消费与 stream cancel/close：取消解除阻塞，真实 Join、callback lease 与 permit 释放；wrong family controls 不绕过校验。
9. caller cancellation：setup/单次/reply/next waiter drop 和 late producer exit 保留原结算事实，不虚构外部 terminal。
10. Channel stop/restart 与 resource/Session close：旧 generation 不复活；先取消后取 write permit，无 read/write 死锁，close retry 保留原 handler。
11. disconnect 与 handler.close 失败：原进程/真实 callback settlement、owner retry、无残留 producer；PID/外部 Run 证据只在实际启用该原生 consumer 时断言，纯测试 counter 不冒充 OS 清理证明。
12. metadata reentry：handler/plugin callback 实际 await ids/list 返回；编译后同 family 六操作、schema/digest、wrong digest、features、三语言 consumer 和最终独立复审均通过。

最终接受仍要求适用 Rust workspace/feature gates、contract 全生成与 drift、真实语言 Host gates、文档/examples/website 同步检查、独立源码审查、已发布 framework 精确 pin、SDK exact-main 验收。Finding 在这些义务全部满足前保持 open。

## 源码证据与当前交付边界

本设计观察 framework worktree HEAD `95ec66c6083a8feb17a1c28de0d01e1a24ca8a28`，owned Channel 改造是当前工作树候选；SDK HEAD 为 `30d7e219cc7d3aef0f75a19217c82898c363f50d` 且存在其它代理在做的候选改动。以下 hash 是设计时观察，不是完成证明；实施前增量刷新被修改路径。

| 源 | SHA-256 |
|---|---|
| framework channels/manager.rs | `9cfc0cd168ad7f34b8348d0fb145e62eaaee3e67edd19dec3a3d8b6dc5b422f3` |
| framework channels/session.rs | `b8ddb931de85f870fe913ebd462c06f337a12e3d422a87fb25e68b841a539145` |
| framework channels/types.rs | `a89b1b0182d8cdf4d34808d848e48aa2213ce8de194818354a7837822395832e` |
| SDK methods.rs | `9e9deb481eb7850a039fc930971a2f9363db6e7905490165121dd28a57641000` |
| SDK facade.rs | `dd9beced710b11502d9f97a4b5b3e0c77613b1f28f87548f821cab5a6f77a8c5` |
| SDK extension_bridge.rs | `862c7cd6f9d6657cc517c4d5dc0aa5a18717fb49546c9d98f2c3ca5279d2c560` |
| SDK facade/integrations.rs | `20c9080c08cd239a0f7e97e8c8278382388f8c0bdc8cbb763e152bfb0a4b9ea2` |
| SDK facade/stream.rs | `0b660a6a0a5a12e5fdc29f6bb52048ece12a038934a4850cc30f331e327dc885` |
| SDK handles.rs | `4562d730adcee62c4ef6e8a04dfbb4b5cdfc2144b3646c54c7994c27de7e5a8f` |

已冻结的 framework 当前里程碑先独立完成交付。本设计不写入该工作树新 ADR 或改变其 source gates；通用机制的 framework ADR 正文边界见下一节，在后继非 main framework 切片记录并重新跑该新增 API 的全量门禁。旧 23 项通过记录不能证明新增前向 API。SDK 设计已批准的范围保持 implementation pending，独立设计复审后继续原目标实现。

## 后继框架 ADR 的通用机制边界

后继 framework ADR 0085 `channel-handler-invocation-ownership` 应记录：原 ChannelEntry 是 handler 准入、successful-start generation 与 cleanup 唯一权威；候选裸 handler Arc、SDK 平行 registry 与原 entry invocation scope 三者比较；选择 entry scope，因为任何复用框架的 ChannelPlugin 都需要 stop 与 active handler calls/streams 的一致所有权。

新 scope 复用 ChannelSession/SessionGeneration/SessionStreamReceipt 的 cancellation、receipt、lease 模式，保持本地 legacy future drop 和实际 driven terminal 的区别。stop 先关闭入站并取消，再等待调用/stream 真实结束和 entry write、plugin.stop、handler.close；失败保留同一 cleanup owner。自定义 handler 不会凭 cancellation 自动得到外部 terminal。SDK wire、owner address、语言 catalog 和 EKO policy 不进入 framework。

独立框架证据必须覆盖 setup/stream slow next/cancellation、失败 start/rollback、stop/restart stale generation、close retry、metadata reentry 与真实 settlement。创建时先确认 0085 编号仍空闲；本轮已确认目标文件不存在，但未写入框架任何文件。
