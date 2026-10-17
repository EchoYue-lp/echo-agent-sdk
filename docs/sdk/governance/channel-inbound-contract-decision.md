# Channel MessageHandler 接受义务裁决

## 人的裁决

2026-10-13，用户明确选择“保留承诺，批准提案范围（推荐）”。继续保留
`MessageHandler::{handle,handle_stream,reply}` 的已接受义务，批准下列三个前向操作的范围。
具体 schema、owner/generation、取消和 stream/reply 生命周期完成设计与独立复审后实现。
此裁决不表示当前缺口已经修复，也不自动接受其它 Rust identity。

现accepted-external-contract包含canonical `echo_agent::channels::MessageHandler::{handle,handle_stream,reply}`，路由bridge:channel_message_handler；这是现有承诺，不由注册成功或send/stop成功代替。完整Rust inventory中的其它别名不增加接受义务。

当前SDK ExtensionChannelPluginProxy.start保存framework提供的handler，但只向语言plugin传handler_id文本；extension/invoke方向是Host→SDK reverse request，不能把它当已接受SDK→Host receive API。ownedentry改造解决七个生命周期await/reentry路径，不自动填入站消费者。

必须明确的一项边界：已接受MessageHandler行为应由哪个既有原生入站consumer触发，或是否需要显式批准新的输入协议才能履约。现契约不存在可调用forwardreceive入口；在查明原接受意图前，不自行新增wire、不偷偷retire已接受handle/stream/reply，不造假的callback测试。

原入站触发缺口仍为 `finding.sdk-channel-message-handler-consumer`，保持 open 至实现、真实消费、验证与复审齐备。完整能力图可以 mapped 并明确该未完成事实；用户裁决只确定预期与操作范围。

## 可审阅的推荐范围

推荐保留这三项接受义务，并为语言 ChannelPlugin 增加进入原 framework handler 的前向输入。使用既有 `_echo_agent/channels/op`、`FeatureOperationRequest`、完整 manager resource handle 和现有消息 DTO；不建立第二个 Agent/Run/调度器。

候选操作只包含三项：

| 操作 | 输入 | 结果与权威 |
|---|---|---|
| `channels.handler.handle` | Session owner、manager handle、channel_id、现有 InboundMessage | 调用该原 entry 已绑定的 MessageHandler，返回现有 OutboundMessage |
| `channels.handler.handle_stream` | 同一 owner/entry 和 InboundMessage | 调用原 handler stream，复用现有 stream handle、next/cancel/close 和结算机制 |
| `channels.handler.reply` | 同一 owner/entry 和现有 OutboundMessage | 调用原 handler reply；返回明确成功或原失败 |

关闭或未启动的 entry、wrong owner/generation、handler 未绑定、消息形状错误在调用之前明确拒绝。取消遵守既有 ACP invocation 和 framework handler token；流与关闭保留实际原 owner，不能因外层请求取消假定结算。callback 重入 metadata 沿现有短锁路径；同步等待自身 mutation 的循环不被默许为安全。

这三个新增前向 wire 操作的范围已批准，尚未写入 accepted catalog/schema 或实现。批准范围只解决三个已有 handler 承诺的真实触发路径；具体 schema、签名、生命周期设计和三语言场景仍须独立审查及验收。

“延期或退役三项原承诺”是未被用户选择的候选方案，当前不采用。
