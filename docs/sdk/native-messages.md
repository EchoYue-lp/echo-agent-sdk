# 原生消息与 SDK 执行事实

## 响应观察后继（独立于已保存的 reverse-status 首切片）

当前候选在Message/Delta上无损携带`provider_response`固定九字段。model/stop/status
等观察的字段缺失与WireValue Null分开；usage沿原WireValue map及Scalar保嵌套值，
不叠加快照或猜测`"00123"`这类文本。`tool_result_is_error`的未提供、false、true分别保留。
示例：`{"protocol":"responses","provider":"fixture","stop_reason":{"kind":"null"}}`
表示明确观察到null；省略stop_reason表示没有观察。均不表示执行成功或重试许可。

Empty消息保持canonical Null/Empty身份；SDK不把metadata-only观察变成模型可见空文本。
实际模型序列化的观察过滤由Framework负责，必要原生reasoning回放继续保留。D2错误
跨SDK仅原有界六字段；raw partial不自动进入AgentFailure、事件、工具执行或批准正文。
准确冻结carrier来源、Source空ID与稀疏坐标边界、实际有限验证及正式pin缺口见ADR0043。

Reverse CustomAgent 流沿原 `ExtensionStreamWriter` 发送：`chunk` 可以承载完整
assistant_message、非终态 execution_status 和 FinalAnswer/错误/取消通知；原生
`complete` 只接受原 phase=Terminal 的 settled/failed/cancelled 状态。FinalAnswer
通知不结束原流，块 complete、ACK、普通 String 和 source EOF 都不能产生执行成功。
既有普通 complete 仍只表示运输末尾，不代表 nativeSettled。

三语言 Message 示例的 `forwardNativeOutput`／`forward_native_output` 只转运实现方
提供的真实原生事件：逐项发送原身份、顺序、phase 和 reasoning，读到原 producer EOF
后提交原终态；缺终态或终态后出现新数据时报错。不要为普通 String 实现手写 Started
或 Settled。Java 使用原 JsonNode，u64 序号保持十进制文本，不经默认 BigInteger 数字编码。
Host 沿同一个 callback/guard/lease 收尾，Factory 在原 source EOF 后关闭。

本轮已在具名冻结v03上完成当前Host九个定向方法、同一Host三语言原生stdio各一项，
以及相关三语值/示例验证。正式f7/0dac来源未切换；这些有限结果不代替完整SDK、
正式新来源、全生成或平台/main验收，准确epoch和剩余项见ADR0043。

v03消费同时移除了Checkpoint旧current_plan字段，六个当前字段与history原字节保持。
工具callback沿原with_id入口；无稳定managed请求身份的旧删除操作明确不可用，
不自动生成删除ID。accepted逐行新来源差异尚待正式producer，不能据计数未变推断行为全等。

`assistant_message` 投影 Framework 的原生消息块：`block_id` 和 `request_id` 固定归属，`sequence` 保留原顺序，`delta` 和 `complete` 描述增量。`phase` 为 reasoning、commentary、partial_answer、final_answer 或 unknown；缺失为 unknown。同一响应包含工具时，正文阶段不因此改变；SDK 不根据文案重新分类。`provider_source` 另保留协议、原始 request／item、output_index／content_index、content_kind 和原 phase；未知原 phase 保留文本，语义阶段仍可为 Unknown。工具调用及增量使用同一来源结构，运行时确认不伪造 provider 来源。

签名思考、redacted 密文和 Responses opaque 原状态沿既有 `LlmReasoningBlockWire` 保留。它们不能转换为普通进度。序号沿 SDK 的 WireU64 十进制字符串，不能先经 JavaScript number 导致精度损失。

最终候选、块结束和执行结算分别处理。`AgentStreamChunkWire::AssistantMessage` 包含 final_answer 阶段时仍是普通块，不能放入 `AgentStreamTerminalWire`；真正终态只能来自 Framework 原 owner，SDK 不重试、不续跑模型，也不把消息结束当 TaskRun 目标完成。

`AgentEventWire::ExecutionStatus` 逐字段投影 `execution_id`、`request_id`、`sequence`、`phase`、`status`、`reason`。阶段的 Thinking／Acting／Terminal 保留原生拼写；状态包括 started、waiting、tool_pending、candidate、settled、cancelled、failed。SDK 不产生这些状态，source-operation 分类转交原 Framework。V6 公开合同由原 ManagedStream 收到原 EOF、join 原 producer／physical owner 后依序发批准原生块、兼容 FinalAnswer、最后单次 ExecutionStatus；ExecutionNotSettled 不补终态。SDK 沿最后事实关闭，不能在 final phase／空 FinalAnswer 或较早错误处截断尾部。

Host 复用 CustomAgent extension bridge、事件 envelope、ACK／replay、handle 及 ACP stdio。纯协议不依赖 framework。当前正式 build pin 仍为 `f7bac62`；该源不能反向接受新 native 事件时返回明确错误，不静默丢字段或降级 Token。同一 SDK adapter 已在本根 Framework 受控冻结源码上构建真实 Host，并通过三语言 stdio 的有限正向消费；该 Host 使用独立 consumer manifests／lock，不改变正式 pin 或来源资格。

LLM Message 和流式 chunk／complete 的 `assistant_blocks` 保留同一有序块，支持历史回传；普通正文和 tool call 不被压成单个无身份字符串。Host 共用一份 Message／reasoning 转换，写入后沿原 Message serde 合同逐字段回读，旧 pin 会忽略的新字段必须转成明确错误。字段无损不等于新来源已经验收。source-operation 的封闭 WireValue 记录中，phase 使用阶段字符串，reasoning 使用明确的 `kind` 标签；不根据任意 provider enum 名猜测。

Message 的 `execution_status` 保留 Candidate 坐标，不能单独证明结算，也不能当成 provider delta 或新 Run 的成功授权。把带该观察的非流式 Message 下转为不含此字段的 provider delta 时明确拒绝，不静默抹掉事实。assistant_blocks 与 execution_status 都只沿原 Message serde 保留；SDK 不创建另一份运行状态。

Framework Rust 的 `ExecutionStatus.adapter_generated` 使用 `serde(skip)`，只在进程内区分原生来源与 Core 兼容合成状态；SDK DTO、事件 JSON 和模型历史不增加该字段。三语言真实 Host 回归检查这两个传输方向均无此字段。Rust public inventory 的实际签名差异须单列核对，wire 无新增字段不代表 accepted delta 为零。

Message 的第三个持久字段 `approved_assistant_blocks` 是经配置输出校验后的呈现投影。`Some(empty)`／JSON 空数组明确表示不展示原始块，必须与字段缺失／null 区分；SDK 无损传递，不用 truthy 判断回退 `assistant_blocks`。原始块继续供协议回放，获准展示内容与原始数据分列，字段级转换不替代 Guard／Critic／Stop 或运行结算。

具名旧 COT 控制（强制 tool 前提示、enable_cot／disable_cot／is_cot_enabled／COT_INSTRUCTION 和 ThinkTool／FinalAnswerTool）在当前 SDK accepted 中精确命中为零。SDK 没有对应三语言控制 API；仅清除内部 ThinkTool 构造分类遗留，完整 Rust inventory 保留真实 pin 的遥测。原生 ThinkingConfig／set_thinking、签名、opaque 和回调继续保留；聚合类型签名变化待正式新 pin 全生成逐项对账，不预报为零。

原 thinking 和 final callback 继续保留，FinalAnswer通知仍沿当前运行时事实消费。
v03已退役的Token/Think事件在Host明确拒绝，不从它们猜阶段；此处不删除原生thinking、
签名或opaque能力。新增canonical身份与已退役Rust身份的正式accepted差异须按ADR0043
由准确新来源的唯一producer实列，完整Rust inventory继续仅作遥测。

仓库三语言原测试入口均包含原生消息 Host 回归，使用 `ECHO_AGENT_SDK_HOST`；强制 Host 验收时设置 `ECHO_AGENT_SDK_REQUIRE_HOST_E2E=1`，缺少 Host 必须失败。共享 fixture 只控制两次模型输入：commentary 后进入 final_answer。测试检查下一请求保留 provider 坐标、原生块与兼容 FinalAnswer、唯一末尾 ExecutionStatus、完整 live／journal replay 一致；这不证明真实 provider 或完整恢复矩阵。

流式／非流式、Stop Hook 继续、取消、父子 handle、事件 mux、历史恢复与完整三语言消费仍需绑定后继正式来源实际验证。客户端按 V6 最后 ExecutionStatus 接线并保留 ACK／replay／generation；当前受控 Host、协议或单元测试不替代正式新 pin、完整生成、安全扫描和完整语言矩阵。
