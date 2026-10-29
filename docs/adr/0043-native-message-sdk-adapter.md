# ADR0043：SDK 无损消费 Framework 原生消息

## 2026-10-29：第二切片——响应观察与工具错误保值

首切片53路径已单独提交为`b0c597bd17de6428694658257ef8ac530b3756a3`。
本切片从该新基底实施，不重标首片的Fv03/工具链/Host票。上游输入只认冻结API-r01
八文件索引`ae7ae47e5b7d5965aa8fe630fe7f0d20c70f821f80f4cfbdeac683afdfc9b985`与
首个完整2818文件图`91bdd2eb11ad2976aeda5b51e249a5b0a7abfe06a9cb7cb4605fe103fb4836ae`。
该图实际Host消费在Anthropic构造点缺provider_response时编译101、运行0项，原件保留。
后继使用正式WIP复制的2823文件r02图，索引
`fc29176b3bb53f802fa01265badb91ac9479e7748a49a527ff2e13d86de3e379`全部成员SHA/权限
已逐项核验；Anthropic修复来自Framework正式实现，不在SDK补shadow默认字段。
这些输入不代表协议producer B、完整SDK或正式新Git来源通过。

Root核验既有`ChatRequest::tool_choice`本身是external_contract/wire_value/
serializable-value，三语为generated_or_lossless_value，`LlmClient.chat/chat_stream`亦为
既有bridge。因此当前Core的合法字段值必须无损承接：三种字符串模式与canonical
`{type:"function",function:{name}}`对象；不能因未单独接受ToolChoice枚举就削去字段值。
本次沿Core唯一Serialize/Deserialize更新既有字段聚合形状，不新增enum/构造器/variant
独立accepted身份。原String-only临时反控与通过票保为历史，不是最终限制。

Protocol以闭合DTO表示原字段；三语request bridge拒绝未知模式、标签、额外字段、错误
name类型与Unicode White_Space空名，保原名、不trim、不JSON字符串化。None/null沿原
Option语义；Host借Core原serde双向值传，不新增provider选择或请求状态权威。
源码聚合签名后继须由正式新pin下原完整producer列出，不手改accepted或source-contract。

### 边界、数据形状与复用

Framework拥有`ProviderResponseMetadata`、同请求快照、模型历史过滤与
`LlmError::IncompleteResponse`分类；SDK只使用既有Message/Delta/ACP/Codec运输值，
不加事件总线、executor、store、重试器或完成判断。`ProviderResponseMetadataWire`
固定九字段：protocol/provider/request_id，以及model、stop_reason、stop_sequence、status、
incomplete_details、usage。六个可空观察使用既有WireValue：字段缺失与显式`kind:null`
分别保留；前四个仅string/Null，后两个仅map/Null，读写错形均拒绝。嵌套usage保持原
Scalar整数、null、数组和数字样字符串，不从文本猜整数、不相加快照、不跨request合并。

观察仅附原Message与DeltaMessage；SDK已有flattenedDelta wire DTO保留该单一投影，
不在原ChatResponse/ChatChunk顶层重复挂。Message的`tool_result_is_error`保留
None/false/true。canonical Empty在wire中使用Null，不能变成普通空文本而绕过Root唯一
metadata-only历史过滤；必要signed/redacted/opaque reasoning与Role::Custom仍完整保值。
SDK运输合同不等于供应商HTTP请求；实际模型协议序列化仍由Framework剥除观察和运行
批准字段，不在SDK复制第二套过滤/归并规则。

当前Delta没有Serialize；SDK不新增该trait，原`with_delta_blocks`继续保其所有现行值，
包括provider_response。非流式响应转Delta时，若有Delta不能表达的name/tool_call_id/
tool_result_is_error或运行时批准字段，明确拒绝，不压平为同名字段或丢弃以造成功。
Java原严格CallbackMessage codec增固定观察，WireValue JsonNode防御复制；Python沿原
Mapping增TypedDict说明，TS沿原LlmMessage增明确类型。三语不生成执行状态或partial工具。

### 不完整响应、身份与风险边界

D2 raw partial只供原直接LlmClient内部调用方检查。SDK的AgentFailure保原六字段，
仅保`llm_incomplete_response`、category=llm、failed、retry=false、http=None及有界说明；
不得自动增加raw/partial字段、伪HTTP或发布为成功chunk。错误Display不输出部分正文。
ReAct如何保存metadata-only、避免partial工具回填与Stop/Guard继续由Framework验收，
基础serde或SDK值往返不替代这些生产链路资格。

仅IncompleteResponse.partial中的ToolCall Source允许request_id空字符串表示未观察到
native ID，output_index按真实稀疏5/9保值；D1.request_id仍None。它不是canonical/runtime
身份或成功回放凭据。SDK不生成、回填或跨请求复用ID，不套用AssistantBlock/ExecutionStatus
的非空身份规则；Responses对空native ID的既有拒绝属于上游协议，不能在SDK降级。

### 验收与实际来源

原独立反例已真实0pass/2fail，分别证明旧wire不承载provider_response与tool error观察；
固定record/nullable/嵌套大整数/未知字符串及读写错形定向修复后3项通过；随后补入既有
ToolChoice字段合法Function的原反例0pass/1fail。最终生成后Protocol三组56项、原候选
producer130产物、语言资源export/check与退役checker自然0；TS29/Python13/Java18含
request桥与包资源检查全部通过。原r02适配器ScalarError错误类型及ToolChoice类型
编译101保留，修复只在SDK。旧SDKr03的7+1 Host通过票仍只支持原String-only临时形状。

最终Host使用SDK1280源r04及Framework2831成员冻结索引
`f327f63e34b7b4fea30d1c34f8559431fd2228172a2d427d8f1eac5912ee75c4`，全部SHA/权限
已核验；相对r02的10改/8增不改变Core公开字段或唯一serde。Root隔离Rust1.99.1下，
字段、Source/Empty与D2的资格以该精确组合具名原命令为准，不继承旧Host票；Framework
最小生产check、metadata runtime、两adapter EOF后继与完整消费者均分别验收。
冷恢复和正式新pin仍按各自精确来源取证。
候选方案采用既有WireValue和原serde薄转换；拒绝新顶层extra map、把观察当状态、
复制供应商completion/retry规则或扩SDK有界错误。Root已据固定官方Anthropic/OpenAI
response/delta/usage源裁决D1/D2；本切片只实现该已确认合同，不扩大accepted范围。
正式f7/0dac、5352身份/431routes、完整Rust遥测与外部source402保持独立，精确新pin仍待
实际已审Git来源；本片不因carrier可编译便宣称完整SDK/Parquet/main CI或发布完成。

## 2026-10-29：Reverse 原生输出与原 source EOF 收尾

本节在 SDK `fa153f93892cde1de950977e731f30196297ceef` 上实施已批准输出迁移。
正式 Framework pin `f7bac62`、锁 `0dacbb87` 不变；旧 959+B53 上的五项定向证据保持
原 profile/二进制资格，不能当作本次修复后验证。本节源码完成后仍须新消费窗口验证，
source115 工具闭包失败和正式 source402 不由本实现刷绿。

### 源码事实与唯一权威

| 真实事实 | 既有权威与适配点 | 本次改动 |
| --- | --- | --- |
| block/request/provider 身份、phase、顺序和 signed/redacted/opaque reasoning | Framework `AssistantMessageBlock`；Host `native_message::{from_framework,to_framework}`；Protocol `AssistantMessageBlockWire` | 沿原 DTO/转换器逐字段传递，不推测 phase，不拼接密文 |
| Started/Waiting/ToolPending/Candidate 与原终态 | Framework `ExecutionStatus`；Host `status_to_framework`；现有 `ExecutionStatusWire` | reverse chunk 增加非终态 status，complete 接受 phase=Terminal 的 settled/failed/cancelled；生成 schema 与 DTO validate 同步 |
| 兼容 FinalAnswer/错误/取消通知 | 原 `AgentEventWire` | 可作为普通 chunk 传递，继续读取原生状态；保留既有 generic transport complete，但不将其升级为 nativeSettled |
| 原 callback 回复、physical guard 和租约释放 | `invoke_owned_producer_stream`、`owned_callback_event_stream` | ACK/EOF/普通 String 不补造状态；沿原 RPC/remote terminal/guard 结算，不增 owner |
| Factory 子实例关闭 | 原 `FactoryInstanceLease`、`close_factory_stream` | 仅同一原 source 真正 EOF 后 close；通知、取消、错误或 observation 不提前关闭 factory |
| Run 事件、ACK/replay 和 history | 原 events mux、三语言 runtime-terminal 消费者、`LlmMessageWire` | FinalAnswer 不停读；输入 history/Role::Custom 与批准空块语义继续保留 |

Framework 提供跨产品的执行、取消、Stop 继续和输出事实；SDK 拥有纯 wire、Host 薄适配
和语言运输；EKO 的目标完成、UI、worktree 等策略不进入 SDK。本次复用现有枚举、
WireU64/I64、ACP 和资源 owner，没有第二套 executor、store、状态机或任务 API。
Framework 的 canonical 输出归并由既有 `echo-core/src/agent/output_projection.rs` 拥有；
SDK 不复制其 Started/identity/确认/终态归并算法，也不通过改正式 pin 假装已消费新来源。

### 三语言与生产者边界

三语言原 stream writer 没有读取 producer 的职责，只有 outer complete 成功后清运输
route。它们复用原 runtime status 校验，对 reverse wire 要求原十进制 u64 字符串、
闭合字段和 chunk/complete 位置，保留原数据，不接受 JSON number 或 adapter_generated。
Java 继续 deepCopy 原 JsonNode；不能用默认 mapper 把 BigInteger 变成 JSON number。

三个 Message 示例新增原生 source 转运函数：普通通知逐项发送，原 terminal status
暂存到原 source EOF 后发送；源异常、缺终态或终态后新数据均不能提交成功。
已有返回 String 的示例保持普通运输语义，不能自行产生 execution_id/Started/Settled。
签名、密文、provider 原坐标和缺失/空 history 均沿原输入值传。

### 候选与取舍依据

采用“复用原生状态 DTO + 原流末尾传递”，拒绝从 FinalAnswer、String、ACK 或 EOF
推测成功，拒绝复制 Framework canonical 状态机。Anthropic 官方 SDK 分别定义
[content_block_stop](https://github.com/anthropics/anthropic-sdk-python/blob/main/src/anthropic/types/raw_content_block_stop_event.py)
和 [message_stop](https://github.com/anthropics/anthropic-sdk-python/blob/main/src/anthropic/types/raw_message_stop_event.py)，
区分内容块结束与消息结束；[MCP transports](https://modelcontextprotocol.io/specification/2025-06-18/basic/transports)
也区分连接断开与请求取消。EKO 本地模型下仍需这种区分，以防框架 bug 造成提前释放原资源。
这些来源不授予 SDK 生成 Framework 的执行终态。

### 合同增量与验收

现有 accepted5352 身份/431 routes 保持原字节：`AgentEvent` 的既有聚合签名
`5f0bd791…`、`FinalAnswer` 的 `99e73f3c…`、`Agent::execute_stream` 的 `b5e614a3…`
与 `chat_stream` 的 `95b9f71f…` 原记录均不手改。四条原 route 继续是标准 session/update
或 `_echo_agent/facade/invoke`；本次不新增方法名或 source operation。
ExecutionStatus/AssistantMessageBlock/AgentOutputProjection 在当前 accepted 中具名命中为0；
完整 Rust inventory 仍是遥测，新正式来源下聚合签名必须由唯一 producer 实列，不能预报为零。

本次 wire delta 是原 reverse chunk 新增 execution_status/final_answer/error/cancelled，
原 complete 新增 execution_status；没有新增公开 Rust accepted 身份或扩 accepted 范围。
schema、正负 fixtures 和三语 digest 只能由原 `export_schema --wire-candidate-update`
及 `scripts/export-language-sdk-catalog.sh` 生成，不手改生成 JSON。本轮原 producer、
语言 export/check 均实际0；schema、原负 fixture 和四份 digest 更新，新增五个正负 fixture。
accepted/source-contract、正式 pin/锁和完整 inventory 原字节保持不变。

验收覆盖完整原身份/phase/reasoning/最大 u64、通知后继续读、原生状态运输位置、
未知 owner/ACK/EOF 不补成功、取消与 Stop 继续、真实 guard Drop 与 source EOF 后 factory
关闭。Protocol 原定向 RED 已实际复现 execution_status 不可表示；原 selector 修复后
1/0/0、原生合同9项和extension合同42项实际通过。后继Checkpoint源码增量另列验证，
不以此前结果冒称全部新源或正式来源通过。

### 冻结 v03 的真实消费与必要适配

Host 消费明确使用 `green-source-v03-envelope-trajectory` 的2809文件冻结源：
source manifest `dea107fa74dae82731462484bb30d8fb2dfe34e1e2b272889e028ae7d00a25e4`，
完整文件索引 `f923106eec59c9afd52f2216f4acff92d03f84d58f361e336782486117fbc20d`。
它不读取正在合流 Provider/D1/D2 的正式 Framework，也不伪装成新的 Git pin。

首次真实编译101/20错后，SDK按实际v03接口做以下必要适配，不复活已退役执行路径：

- 原 Token/Think 事件不再转换为 Framework 事件；原生 AssistantMessage 保留明确 phase。
  旧 wire 事件在 Host 明确拒绝，保留的 thinking/final callback 不受该拒绝影响。
- 工具 callback 只用原 with_id 入口，无 call-id 的三个旧 trait override 移除。
- Checkpoint 的 current_plan 随上游退役；Protocol、三语及原 public corpus 同步现行
  六字段，history 原字节、active_skills、blocked_reason、working_dir、timestamp 均保留。
  原 Native Checkpoint 构造器负责恢复，不增加 store 或旧字段兼容分支。
- ModelConfig 未提供的新 facts 使用原类型的 Default，不覆盖已有 wire 配置。
- 旧 unmanaged delete 已无上游函数，Host明确 FeatureUnavailable；不为旧参数补造
  ManagedConversationDelete 身份。既有 managed 删除路径保留。该旧 accepted route
  的最终处置仍须在正式来源下逐项核准，不能声称5352项行为已全量通过。

首次重拍SDK快照时Cargo复用了记录原绝对源码路径的Protocol缓存，r2编译101及两条
unused import warning均保原。保全旧rlib/dep-info、移除失效import后，在无reader时
明确切换原consumer的源链接到新冻结快照；未清缓存、补Tag、改编译flag或伪造mtime。
真实重编后，新增ACP原生输出及八个保留方法各1/0/0、零warning；同一保全Host
`f81d98a6a5fdff92468b4bcb01744b91932da7aece4b815916bdaf41eaad3b2f`
下TS/PY/Java原生stdio各1项通过、零skip，覆盖live/replay、history/provider坐标和
FinalAnswer后唯一执行终态。相关现行值/示例组合TS30、PY33加18个subtests、Java29通过。
它们属于已记录的Rust1.99.0消费epoch；后继全局stable切换时原channels控制遭遇
rustc进程spawn ENOENT/101，原件保留，Rust1.99.1下的原selector重跑另取证。

源码115原错误的逐项核对为42 audit、47 finding、26 behavior：现有owner/behavior反向
归属均存在，局部forward closure未带入七个owner map。此诊断不代表全仓所有引用、
文档事实和source摘要已正确；本仓真实错误应正常修，外部收集合同与source402仍分列。
正式新pin、完整生成/accepted聚合与具名退役、完整SDK/平台/main CI均未因此闭合。
本切片不并入D1 provider_response或D2 IncompleteResponse，待新接口冻结后另接。

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
