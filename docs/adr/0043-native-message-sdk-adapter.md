# ADR0043：SDK 无损消费 Framework 原生消息

## 状态

用户已确认原生消息切换与 SDK 薄适配方向；本 ADR 记录该方向的实施边界。当前实现已通过受控冻结 Framework 源码构建的同一真实 Host 三语言 stdio 有限正控；正式 framework pin、完整生成、依赖安全、完整语言矩阵及发布资格未完成。已确认设计方向和有限消费通过均不代表整体实现验收。

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
