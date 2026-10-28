# ADR0043：SDK 无损消费 Framework 原生消息

## 2026-10-28：CustomAgent 完整 Message 输入候选

本增量遵从用户开发阶段不保旧版本兼容的最新 Goal。Root 恢复 active Goal 后，已从
本地 WIP `6cb12d3` 增量采用保存的24路径源码。随后独立 Protocol 114 项及原生成器的
候选 schema／fixture／三语言源码资源同步已实际通过；当前改动的 Host 尚未编译或运行。
正式 f7 pin／0dac 锁及历史验证结果保持原输入资格，不能据协议通过声称 Host 已完成验收。
与 NoCompat 清理清单分开，本次不删除合理 Agent API、Store、callback owner 或执行能力。

当前真实注册链为 `factory.rs → apply_extensions_to_agent`／SubagentFactoryAdapter →
`ExtensionCustomAgentProxy` → 既有 run_stream_scoped → 原 callback owner／ACP。
两个 Message+invocation override 原先只匹配 Text，把 role、工具身份及原生消息字段丢掉，
遇到 Parts/ResourceLink 直接报不支持；这是当前能力缺口，不能以“旧接口”免除。

采用既有 `LlmMessageWire`。原 agent_execute(_stream) 的 task、agent_chat(_stream) 的
message 均承载该完整 DTO；operation 继续区分 execute/chat。Text 便利调用先构造
`Message::user`，再走同一转换，不保旧 String wire fallback、不新增操作族或执行器。
两个现有输入 DTO 另携带原 invocation 的 history，继续复用 `Vec<LlmMessageWire>`；
None 与 Some(empty) 分别保留。history 必须实际发送给远端实现方，不能仅保留于本地
callback owner。工作目录、只读上界和进程内 guards 仍由同一个原 invocation/owner 管理，
没有把诊断 wire context 冒作远端权限授权。
Host Role 沿原生 serde 解码，未知值不能通过字符串转换悄悄降为 user。
ResourceLink 的 meta/annotations 随原 content 保留，不从正文推测或重新编码来源。
原 invocation、cancel token、history、只读上界及 callback owner 不重建、不降级。

考虑过保留 task/message 字符串再额外加可选 rich 字段；该方案会留下双轨、优先级猜测和
未知字段被忽略的问题，拒绝。也不新造 Message/ResourceLink DTO 或用户文本协议。
官方 [MCP Tools](https://modelcontextprotocol.io/specification/2025-06-18/server/tools)
使用结构化 resource_link；[Anthropic Messages](https://platform.claude.com/docs/en/api/messages/create)
以 role 加 content block 承载消息。它们与本项目既有 Message 模型一致，支持复用完整载体。
EKO ProjectAgentAdapter 只读对照显示原 Message+invocation 整体转发；本次不改 EKO。

回归候选包含原 ACP reverse callback 对实际 dyn Agent 四个 typed 入口的调用：默认 Message、
rich ResourceLink、空 meta 与非空 meta、tool role/name/call_id、Some(empty)/None、原 history
和 scope；旧实现遇到 rich 输入会在 callback 前失败。另覆盖完整 native serde roundtrip、
非法 role、旧 String payload、缺 role 和错误 operation/input 字段负控。协议测试不能
代替 Host 测试。三语言旧消费者更新读取完整 DTO，三个示例进入原编译/检查链路。
TS/Python 示例补 raw handler 普通结果、close、stream 三条 outcome 合同回归，避免把
typed-handler 的裸 result 误用于原 raw handler；这些本地语言回归也不代替 Host 编译。

Root 转交的 Framework canonical consumers v02 manifest `be8dbdc1` 是独立 ignored 候选，
目前仅 syntax/apply 资格。其输入值传与原 history 方向一致；runtime FinalAnswer／
on_final_answer、Guard 安全最终正文和原校验收尾继续保留，不能因两内置工具退役而删除。
无 raw block 的安全正文仍须完整呈现；本请求已呈现的批准最终块不能再重复拼接。
该规则不是本输入修复另造输出投影、事件 shape 或 final owner 的授权。

本批实际结果与剩余边界如下：

- 原纯 Protocol 选择性测试实际为 lib42、native8、extension42、core RPC22，共114项，
  零失败、零忽略、零过滤、零编译 warning；其余 Host／Framework 编译不在该证明内。
- 保全的原 `export_schema --wire-candidate-update`、原语言资源 exporter、其 `--check`
  和精确退役 checker 四条命令均自然退出0。119个允许产物中仅1个 schema、1个有效
  factory fixture及shared／三包4份digest改变；其余113个产物与生成前字节相同。
- 新 schema SHA256 为 `b68bf43ace8f269f4500e873a2dc24a1aef9799f1146d63bd184603b4992dd01`。
  两个输入定义引用完整Message并携带可选history；accepted5352身份／431routes、全部
  catalog／签名及source-contract保持原字节。shared与三语言源码资源逐字节一致，
  不代表tgz／wheel／JAR已重新构建。

唯一 full producer、正式新来源、当前 Host、完整三语言运行和示例执行仍待 Root 后继窗口；
上述部分生成没有执行rustdoc／inventory接受或改变正式pin。原始记录分别保存在本仓任务
目录 `protocol-focused-6cb12d3-20261028/` 与 `wire-generation-6cb12d3-20261028/`。

本轮另修Java `CallbackMessage`的当前合同缺口：沿原严格typed codec承载
assistant_blocks／execution_status／approved_assistant_blocks及ToolCall provider_source，
复用原WireValue、reasoning和协议枚举，不允许未知字段。批准块缺失与显式空数组分别
保留；消息中的candidate状态仅作值传，不令callback的unit结果成为Run完成。
这不新增wire身份，也不授权SDK为String函数结果或普通EOF补造执行终态。
原失败反例与首次修复的局部codec／测试差异均保留；修复后的原Maven定向入口实际
27项通过、零失败／零跳过，包含native字段、缺phase的Unknown、显式null／未知字段
拒绝、u32／u64边界、原通知unit及取消合同；这仍不替代真实Host回调验证。

## 状态

用户已确认原生消息切换与 SDK 薄适配方向；本 ADR 记录该方向的实施边界。历史原生消息增量曾通过受控冻结 Framework 源码构建的同一真实 Host 三语言 stdio 有限正控，该证据不覆盖本次完整Message输入和后继callback修复。当前Protocol／候选资源同步与Host验收分别结算；正式framework新pin、完整生成、依赖安全、完整语言矩阵及发布资格未完成。

## 背景与权威

用户于 2026-10-27 明确要求具名退役 ThinkTool、FinalAnswerTool，统一原生消息与运行时终态。Framework 的 `AssistantMessageBlock`、`AssistantPhase`、既有 EventEnvelope 和 TurnReceipt 拥有事实；SDK 保持 ADR0034 的纯协议／薄 Host adapter 边界，不另建 executor、状态机或完成判断。

## 候选方案

候选一是在各语言按 Token／ThinkStart／FinalAnswer 重建消息阶段与结束；它会丢失 provider 原身份、请求和顺序，还会把最终候选冒作执行结算，予以排除。候选二是投影 Framework 的 canonical block，复用现有事件 mux、ACK／replay、handle 和 stdio；采用此方案。

## 决策

现有 `AgentEventWire` 和 `AgentStreamChunkWire` 增加 `assistant_message`，逐字段保留 block_id、request_id、sequence、phase、delta、complete、provider、原生 reasoning。sequence 沿 SDK 既有十进制 WireU64，覆盖完整 u64 范围；phase 缺失为 Unknown，不从文案猜测。signed、redacted、opaque 的签名、密文、身份及摘要原样转换。

block complete 与 final_answer phase 都是消息事实，不能关闭 Run 或发布运行成功。最终发布和唯一执行终态仍消费 Framework 已结算事实；TaskRun 目标完成属于应用的独立判断。旧 callback／final event／native thinking 配置保留，退役只针对具名内置工具。

Message 与流式增量同样保留 `assistant_blocks`，避免只迁移事件而在历史回传中丢失身份／顺序／签名。Host 共用原 Message 和 reasoning 转换；新字段通过同一原生 Message serde 合同逐字段回读，旧 pin 忽略新字段时明确拒绝。它是边界转换检查，不是新的历史 store 或结束判断。

Message 的 `approved_assistant_blocks` 与原始块分列；空数组保持“禁止原文回退”的含义，不被映射成 None。`provider_source` 无损携带协议／请求／item／原始位置／kind／phase，同用于 assistant block、ToolCall 和 DeltaToolCall。u32 原始位置保持其准确范围，SDK 不把语义 Unknown 覆盖回原始 phase，也不为 runtime block 补造 provider 来源。

ExecutionStatus 只逐字段投影原执行身份、请求、序号、阶段、状态和原因；source-operation 仍调用 Framework 的分类，不在 SDK 推导任务完成。V6 合同由原 ManagedStream 在原 EOF／join producer／physical owner 后发布批准块、兼容 FinalAnswer，最后单次 ExecutionStatus；未结算观察不得补终态。SDK 消费该末尾事实，使用原 envelope sequence 做 ACK／replay watermark，不把 Message 内的 Candidate 坐标当成成功。

## 来源与 accepted 范围

当前正式 manifest 继续 pin `f7bac62`，新 canonical 类型的工作树定义不被伪装为公开合格 revision。协议和 adapter 可先形成可审候选；旧 pin 收到无法支持的新事件时明确拒绝，后继精确来源、Host 消费和正式生成须独立验证。

本切片不自动新增或撤销 Rust accepted 身份。现有 5352 accepted 身份／431 routes 中 ThinkTool、FinalAnswerTool 的精确命中为零；它们目前只见完整 Rust inventory 遥测。原 A2A/LSP 的 269 身份／23 routes／39 operations 撤销证明保留。后继新增 canonical 类型或字段的 accepted delta 必须由实际生成器及明确具名依据列出，不能把 inventory 当成承诺全集。

后继具名旧 COT 控制撤销同样逐项检索：当前 accepted 中 enable_cot／disable_cot／is_cot_enabled／COT_INSTRUCTION 命中亦为零，故具名 source/kind 撤销数为零，不修改旧 A2A/LSP 269 项证明。聚合 AgentConfig／Message 等的实际签名 delta 仍须在正式来源下生成后核准；不能因具名撤销为零便声称聚合签名无变化。只移除 SDK 私有旧 ThinkTool 构造分类行，不扩删原生 thinking、opaque、公共回调或其它合理 API。

## 验证与影响

纯协议验证五阶段、缺失阶段、u64 上界、签名／密文往返和非法身份负控制；final phase 必须仍属于普通流块，不能作为 CustomAgent terminal。Host 和三语言沿唯一 wire/schema 生成及实际消费补证。候选生成使用既有 `--wire-candidate-update`，明确不替代全量生成、正式 pin／安全 source、三语言完整矩阵或原生验收。

正式 SDK 文档与例子随当前真实字段同步；官网由所属执行者绑定已审来源。原 source 402、fresh 4 漏洞／10 unsound、发布授权与 main CI 缺口不因本 ADR 取消。
