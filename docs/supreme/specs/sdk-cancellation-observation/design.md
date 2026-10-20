---
title: SDK cancellation observation adapter
artifact: design
carrier: markdown
---

# SDK cancellation observation adapter

状态：用户已批准 framework B 与 SDK 适配；Root 于2026-10-16授权实施。设计来源为已独审 local c5db9188 候选，详细设计集中于此，ADR0039只记录决策。当前实现基准 SDK `019e700279677a2eb77e265b5038d9d581bc22da` / tree `4097f924fc75380aa73cb015754b8603e92da3d4`；其225项Host构建输入和793项语言输入与原7f14逐SHA相同。当前固定framework仍为00782；上游B设计/API调查源尚未公开交付，未被冒充为已发布revision。

## 问题与权威

### 问题与目标

用户已选择 B：取消宽限期限制观察等待，原执行、claim、资源和物理 cleanup owner 保留，真实结束后才由原 exact CAS 结算。当前三个直接消费者缺口：

1. `facade/subagent.rs:91` 将 background.join 的任意 Err 写为 settled，`facade/mod.rs:279/434` 据此回收容量/删除记录。
2. `facade/stream.rs:184/300/599` 将本地 producer 结束、任何错误或 deadline abort 当作 terminal/结算；workflow 取消时直接丢 source。
3. `extension_bridge.rs:307/1242/1950` 在 callback setup Err 或带 observation 的 Failed 帧处确认 remote terminal/释放 lease；Factory 的显式 close 和最后 Arc Drop 还会关闭未知 child。

目标是现有 RPC 及时返回无损观察错误，同时保留同一个原 owner。没有匹配结算证据时不重执行、不重 join、不伪 EOF、不发布 Failed/Cancelled/Complete。普通成功、普通终态错误、实际 cleanup 顺序和原字段继续保持。

### 权威、兼容范围与复用

唯一行为权威为 framework 已批准的 Design `docs/supreme/specs/2026-10-14-subagent-physical-settlement/design.md` 与 ADR0088；SDK 分层权威为 ADR0034。复用 RuntimeTaskService、TaskRevisionService、BackgroundSubagentHandle、HandleRegistry、CallbackInvocationOwner/ExtensionStreamSink、FactoryInstanceLease、FacadeStreamRecord 及其原通知机制。内部记录可以增补 typed observation/保留资源，不是新的业务状态或 registry。

|责任|唯一 owner|SDK 适配|
|---|---|---|
|取消/暂停请求、claim 和 terminal CAS|framework runtime/revision service|required controller hook 只委托现 exact CAS，保留 true/false/Err|
|Task graph 执行|同一个 RuntimeTaskService 原 spawn/Join|Host 内部原 connection owner 仅一次 execute_to_settlement；控制 RPC 只观察|
|后台 Subagent unknown|原 execution/admission 与同 SubagentDispatchRecord|保 handle/executor/quota，collector 不再次消费 join|
|反向 callback 与 factory child|原 invocation lease、context、issued child/Factory Arc|观察不成为 remote terminal，不自动 AgentClose|
|facade pull stream|原 FacadeStreamRecord 与实际 source/collector|及时返回观察，保 sequence/resource/producer；真正 closure 才释放|

原 B 观察切片不新增 TaskStatus、wire code、操作、language method、公共 identity、scheduler、retry/receipt store 或第二 terminal 分类器；下节单独获用户批准的 Workflow A 仅显式增加一个 capability 与一个 reverse 操作，不扩成完整 Rust 库存镜像。accepted `AgentEvent::from_error` 和 `AgentTurnDriver::drive` 分别继续映射 `standard:session/update` 和 `core:run/_echo_agent/run/start`，不是新增同名三语言 Rust API。其 fallible Rust 签名变化在精确公开 pin 后由唯一 generator 更新摘要，不扩 seed；真实 identity 缺失/kind 退役另给 Root 具体差异。

标准库/Tokio 已覆盖通知、mutex、原 Join/Stream ownership 和弱引用，无需新依赖。F 与应用 C 的具体实现仍由各自 writer 持有，SDK 不复制。

## 无损错误与 Task/Subagent owner 设计

### 无损错误边界

`wire::framework_error` 使用上游 fallible terminal classification。普通错误保留原 category/terminal_kind/code/retryable/http_status/message；ExecutionNotSettled 映射现有 envelope：

```text
code = framework_error
retryable = never
operation = 实际原操作
handle = 调用方原本绑定的实际已有 handle
fields = observation:not_settled, run_id:<原完整 String>,
         execution_id:<原完整 String，仅 Some 时存在>
```

观察没有虚构的 framework_terminal_kind 等 AgentFailure 字段。message 可按原 Unicode 安全界限裁剪；identity 不裁剪、不 hash、不缺省为空、不从当前 Task 猜回。复用 protocol/error.rs:256 的 validate：field 唯一、value 最多2048字符。合法长 Unicode identity 原样保；超过既有界限明确拒绝且保原 unknown owner，不静默改 identity、不扩大 wire 限额。

反向路径必须先于 `transport_error:682–698` 的 validated decode/fallback 处理观察知识，不能等 react_error 时才识别。对原 RPC error.data/原 stream Failed payload 先作有界结构检查：是否含 observation=not_settled 的 hint；hint 只说明不能证明物理结束，绝不作为可信 identity/成功或 terminal。随后由原 validate 检查完整 envelope，复用一个 helper，合法形状恢复同一个 typed ReactError::ExecutionNotSettled；不匹配 hint 的普通错误维持原转换。exact interruption 的 typed error 使用原 lossless Into<ReactError>，不以 to_string 猜类型。

坏观察形状的两个出口分别定义如下：

- 对外 RPC observer：返回现有 SerializationViolation/Never，绑定原 operation/已有 handle；保有界 validation 诊断，不回显未验证 payload/秘密。重复字段、缺 run_id、超2048字符等都不静默修补。
- 对内 physical knowledge：在验证前由原 CallbackInvocationOwner/FacadeStreamRecord/issued child record 保存 uncertainty、原未被消费的资源与原 invocation context；不把 decoder 的 ExtensionFailed fallback 传入 Other/AgentFailure/Failed。若这一原 AgentInvocationContext.runtime 已实际携带 run_id/execution_id，保留这份原权威事实来构造 ExecutionNotSettled，validation 诊断独立保在同 owner/RPC 错误。不是借当前 Task、其它 Session、错误 payload 或空字符串补 identity。
- 若既无合法观察 identity，又无原 authoritative invocation identity：该内层 Result<AgentEvent> 没有可立即返回的合法 typed observation 值，因此不声称可以及时返回 ReactError。保原 callback/issued owner 和其原 response/stream/closure 通知等待；外层 SDK RPC observer 可读原记录的 validation 拒绝，或者沿既有 RunWait timeout 观察未终态。不得返回 Other/IoError/Failed，也不重新 execute/join。没有可继续 poll 的 source 就保存原 uncertainty/资源，等待原 owner 的真实 closure/fence 通知；这不新增 std::future::pending、永久假任务或第二 registry。没有该原证明时持续 unknown，断连/本地 EOF 不解除；正常来源有 identity 的 B 有界观察仍只由 framework 提供。

这种处理不把 malformed hint 升为可信 framework 事实。它同时拒绝坏输入、保留原物理知识，并明确“没有合法 identity 的 foreign malformed 出口不能即时透传 typed Rust observation”的类型限制。若实现找不到原 owner/通知承接点，必须报告这一具体缺口，不用新状态/协议或终态 fallback 使测试通过。

### Task 与 Subagent 原 owner

FacadeTaskController 新 required request_task_interruption 只调用 TaskRevisionService::request_runtime_task_interruption(run_id, &task.spec.id, claim, disposition)。不实现 claim 状态算法，不把 false/IO 当成功。task execute 仍立即返回 TaskRun handle；其原 connection execution task 选择 execute_to_settlement，一次使用同原 service 的 spawn/Join/CAS，不先 execute 再重新执行取回结果。resolve_dispatch 改用 fallible classification；观察直接传播，不生成 resolution request/terminal CAS。

TaskRunCompletion 对物理 owner 丢失的 unknown 保存观察并唤醒等待者，不能 settled=true 或 remove_task_execution。Session/connection close 返回 pending/unknown 并保原记录，不能永久等在只期待 settled 的 Notify；普通 joined terminal 按原完成路径只释放一次。

SubagentDispatchRecord 保持原 terminal result slot，并在同一记录存 typed observation。collector 仅真正 terminal 才填 settled；unknown 唤醒 await 返回原 typed error，但不回收 quota、不删记录、不因为一次 join 已返回而拒绝继续 cancel。重复 await 读同一个结果，不再次 join/dispatch。close/cancel 只可信号原 token，观察不成为关闭完成事实。

必须区分两个出口：(1) runtime 的 bounded observer 退出，原 owner 仍进行、以后可交原真实 receipt；(2) foreign/background 返回 unknown 后没有 lateclosure 入口。F 当前 BackgroundSubagentHandle::join 是 single-consume，unknown 后没有 authoritative lateclosure 查询接口，is_finished 只描述本地槽。第二类保 unknown，不承诺 eventual 恢复，不以事件名、缺失 claim/registry、restart/disconnect、重复 join 猜原工作停止；此切片不新增查询/恢复接口。

## Callback、Factory 与真实晚到 closure

观察必须在 setup RPC answer、stream Failed carrier、generic mapping 和资源析构前识别。wire receive sequence 继续按原 stream/full generation/owner 校验；收到观察可推进真实收到帧的 transport watermark，但不能设置 remote_terminal_received、semantic terminal_seen 或释放 invocation。晚到真实 terminal 仍过同 sink 的顺序/身份及 matching invocation 检查，wrong child/gen 或其它执行的 frame 不能释放。

observer 从原错误通道及时得到 observation；后台原 callback owner 保存 lease、AgentInvocationContext/resources、原 Factory Arc、原 response/remote terminal 观察。setup Err 没有可继续 poll 的 producer 时，在原 issued child/invocation record 保存 typed uncertainty 和原资源，不造永久 pending future/self-Arc/mem::forget/replacement execution。Factory create failure、finish_factory_result、close_factory_stream、显式 close、Drop 自动清理都须识别 unknown：不得 AgentClose、重试 mutation、释放 child/parent/配额；普通已确认 close 失败原 retry 合同保持。

为保原 Arc 如需 registry 支持，只扩展已 issued child 的原记录，不建新 map。保留边不得形成 self-Arc cycle，必要 back-edge 用 Weak；具体字段图由 implementation review 证明。不能仅置 unknown 布尔阻止 close 却丢掉原 Factory Arc。disconnect 只结束 delivery，不证明 remote physical stop；原 Host close authority 在未知下不能执行成功后的 extensions/streams 全释放路径。

有真实晚到 closure 的出口仅使用原 physical authority 的匹配证明：CustomAgent 原 actual response 加 validated remote terminal；有真实 physical source 的原 source EOF加原要求的终态/receipt。原 owner 原子释放同一 lease/resources一次，然后允许原清理；重复 close/terminal 不重复 AgentClose/CAS。观察后的本地 EOF、callback Err返回、本地 Join 或 token取消不满足门。没有匹配原 receipt/authoritative fence 的出口保持 unknown，无清除承诺。

## 原 producer 与验收

### Facade producer、workflow 与 Sandbox

FacadeStreamRecord 分开保存 observer error 和 actual producer closure，复用原 record/Notify/collector。next 检查 observation 后走原 RPC error envelope，不投 FacadeStreamEvent.failed、不推进 terminal sequence、不 finish_record。cancel 可确认原请求已被接受，不能声称 producer 已停；close deadline 只返回未结算观察，不 abort 原 source/tombstone/回收资源。unknown collector 即使本地任务结束也不能标 physical settled。

workflow graph/extension workflow 在取消、发送失败/receiver drop 后继续 drain同一个已打开 source。首普通业务 cause 与后继 not-settled physical knowledge 分开保，不相互擦除。若 source 只是 observer wrapper、缺原 physical receipt，即使返回 None 也 unknown，不能凭“同变量 EOF”升级为物理结束；真正 held source 的 late closure 对照才可结算。

Sandbox 固定 Stream<Item=SandboxStreamEvent> 无 Result 槽：不改 upstream API，不造 Failed/IoError。在原 callback owner 保存并通知可表达错误的 Host facade/RPC observer，wrapper 自身继续原 drain；无错误槽的普通 Sandbox consumer 保持 pending/cleanup，不承诺即时 typed error。正常 EOF、真实 Complete、显式 cancel/finally 的既有 fused-source 顺序保持。如当前类型/owner不能满足此承诺，报告精确边界，不自行改上游类型或 wire schema。

### Tool、LLM 与其它直接 terminal 转换闭包

同一 typed observation 不能在 Agent/Sandbox 之外被另一个 infallible failure helper 吞掉。本次已实际有界搜索整个 Host 的 AgentFailure::from、AgentEvent::from_error、ToolFailure::from_error、subagent_status_from_error、react_error/tool_react_error/sandbox_react_error 和 reverse stream adapters；直接发现 extension_bridge.rs:2241 的 tool_stream_failure 调用 ToolFailure::from_error，:3181 将任意 Failed 包成 ToolStreamEvent::Complete(ToolResult)。该点必须在调用 infallible helper前拦住观察，不能只修改 react_error。普通 ToolFailure 的字段/category/recovery/may_have_side_effects继续由原 helper 构造，不改其公共API。

ToolExecuteStream 的 Item=ToolStreamEvent 同样没有 Result 槽，采用与 Sandbox 相同的原 owner 原则：setup 的外层 Result 可以原样返回 typed observation；流内观察在原 invocation/callback/issued resource owner保存并唤醒有错误槽的Host/RPC observer，原 Tool stream继续同源drain/待证，不伪 ToolResult、ToolStreamEvent::Complete、正常EOF或永久假future。原 ToolContext 已有的 run_id/execution_id是允许使用的原权威context，不能用当前Task或错误payload补造。没有合法identity时遵循上文malformed/unknown双出口和原owner等待限制，不宣称固定Item消费者能即时收到typed Err。

LlmChatStream 的 Item=Result<ChatChunk>已有合法Err槽，:3735可以透传typed observation；但必须连同其下层 invoke_stream/extension_event_stream 的原lease/实际source保留一起改，不能先因wire Failed载体/is_terminal释放再在上层恢复typed Err。CustomAgent、Tool、LLM、Sandbox及原Workflow reverse路径共用原低层观察识别和同owner保留规则；对正常terminal/ordinary errors仍使用原各family转换，不扩展邻近未接受feature。

Root最后有界调用方核对补充：普通 Store 的 Result<_,ReactError>及 managed component 的 ReactError 槽透传合法观察；既有 managed unknown-commit、absolute-deadline分类不被替换。RevisionedTaskStore 的固定领域错误、HookResult、InterventionResult、AgentCallback的()、Intent/Skill策略的固定返回值没有合法观察槽。仅收到本次显式not_settled或其坏shape hint时，通用invoke_once保原lease/context/issued记录，并使这些原调用继续等待原真实outcome/closure知识，不转换为Backend字符串、Fail、Unit、默认deny/cancel/Fallback。原callback给不出匹配lateoutcome/receipt时仍显式unknown，不承诺额外恢复接口。普通best-effort通知/错误和managed领域政策沿原分支，不以本次修复要求所有callback新增阻塞；MCP通知轮询退出本身不是Run/物理结算证明，原unknown invocation仍留在原registration。没有增加策略/协议/全域owner registry。

实际实现前再对最终公开B源作同样有界消费搜索，以真实编译签名为准；若某直接转换没有合法非terminal槽，使用已说明原owner边界并报告限制，不能新增terminal_kind/errorcode、改固定Item/API或新增registry使其穷举匹配。F自己的code/pipeline/snapshot修复由F writer承接，SDK不复制这些实现。

### Workflow 两项独立开放义务

已接受路径是 declarative Graph 的 AgentNode：SDK workflow.graph.build 复用 Session live registered LlmClient，原 loader 把它交 ReactAgentBuilder/Node；resume、resume_exact、resume_with_state 都委托同一 Graph。固定外部公开源码 [f0a6cd1](https://github.com/EchoYue-lp/echo-agent/blob/f0a6cd1c13706c20955209b698c1f891c09d78b3/echo-orchestration/src/workflow/graph.rs#L1286) 对任意 resume Err 普通 requeue，requeue失败又转Other丢原观察身份。F solewriter拥有Graph及其CheckpointStore的claim/renew/expiry/restart修复与强反例，SDK不得加第二claim/requeue/恢复driver。这里没有扩大 deferred任意StepFn/closure；公开f0a也不是SDK已升级的构建pin。

独立的 SDK facade_admission 非流 workflow.extension.run 仍 error.to_string 后 sdk_error，丢 NotSettled details；相邻 Graph模块已有 wire projector不代表该分支已修。最小修复限定原typed观察走唯一 wire::framework_error，原普通失败的code/消息前缀/Never/operation/details政策保持。当前生产分支必须先保持，等公开新pin与Root Cargo租约，在已准备的真实ACP/profile前向→reverse Workflow callback→前向错误强unit先观察到真实投影断言red，再改生产；缺B variant的编译失败不是red，不临时path patch/拷旧Host凑证据。

反例只在实际Workflow callback里准入一个原foreign工作，原guard/Join在错误返回时仍held；同时核原SDK invocation admission保留、完整run/execution/operation投影，以及另一次ordinary错误严格对照。最后显式release/join原工作，测试transport teardown不当作永久unknown正常结算。Graph claim重领、requeue失败typed身份与heartbeat/expiry/restart由F分别验证，不能拿本projection unit关闭上游Graph Finding。两项精确Finding/源码Evidence及Map/账本保持open；当前没有执行Rust/Host或已修声明。

### 已接受 A：原 Workflow Store 在副作用前保护 claim

用户2026-10-16明确选择A，绑定 [原提案23c5a59及完整历史](../../../sdk/governance/workflow-protected-claim-proposal.md) 和人的裁决Evidence。这里是已批准合同的 SDK 实施设计，不是新的运行结果，也不替代上节 direct-run 投影 Finding。F 独占 Graph/CheckpointStore 的保护、持久化、heartbeat和结算；SDK保薄投影，不写第二 Store、DAG 或恢复循环。

- 唯一Rust trait增量为 `supports_protected_claims() -> bool`（默认false）和 `protect_claim(id, attempt_id) -> Result<()>`（默认明确不支持）。Graph仅Rust/Host getter委托其实际绑定的Store能力；Host不得改读latest Store或匹配Other字符串猜capability。
- 既有agent_component_call仅增加 `workflow_checkpoint_protect_claim`，两个原字符串字段 `checkpoint_id`、`attempt_id`，成功为同operation的Unit result；capability仅 `protected_claim_v1: bool`。三语言闭合DTO/decoder/dispatcher必须明确承接这一项。无新endpoint、错误码、公共Checkpoint字段、receipt查询、强制解锁或语言Graph能力查询。
- 缺省/false仍准Store注册、CRUD及无关操作；错类型或错component声明true在handle发布前拒绝。所有可能执行Node的claim-backed Approved/Modified resume*必须在claim/effect前查实际Graphcap；branch_from在fork保存前拒绝。沿原canonical `feature_unavailable/Never`、实际operation/handle。Deferred load/validate、Rejected无Node的claim+ack和fresh run不被封禁。
- 原Store同checkpoint/attempt原子保护，收到明确成功才允许Nodeeffect；sameattempt幂等，stale/wrong拒绝。写失败/缺回执/丢positiveACK不准effect。真实结算前不得由TTL、heartbeat失败、EOF或absence解除，也不得delete/clear/覆盖绕过；clear在原Storeauthority内先检查并原子整体拒绝，stale ack/requeue不清新attempt。真实closed由原ack/requeue结算，ordinary已物理结束错误的原requeue保持。
- 保护继承原durability：Memory只同实例，File/声明持久remote才跨restart。F File复用原fs2 `.echo-checkpoint-store.lock`内exact-attempt CAS/持久提交，无新锁/依赖；版本化私有格式让旧自动恢复/读改写失败关闭。旧delete/clear不解码仍能破坏，因此禁止旧binary指向保护数据，不声称全面旧writer安全或自动无损降级。
- 依赖/错误分层复用原strict未知字段和accepted contract digest。新provider实际发送该cap时旧Host可在注册解码前拒绝，正常新schema digest亦failclosed；不新增global协商版本。serializer丢cap或主动旧descriptor的意图不可凭空推知，不能把这种组合称为新能力受到保护。

实施顺序：先保当前未实现协议/客户端的真实before（原strict注册cap/新操作decode，测试必须可在旧源码编译），再改对应DTO/dispatcher；F公开精确main后才pin与唯一producer全28profiles，人工审单项accepted增量及已批准B签名迁移。Host在原绑定Graph admission加cap检查、checkpoint proxy仅委托原Storeprotect。原生产 direct workflow stringify仍等新pin真实red后单独修。

验收必须具名覆盖：legacy注册与缺cap preclaim/prefork/effect零副作用、Graph绑定Store不随新registration变化；原same/wrongattempt、保护写失败/丢ACK、TTL/restart与heartbeat不重领；protected delete/save/混合clear原子拒绝和stale结算；真closed后原ack/requeue；旧reader自动路径failclosed与旧CRUD限制；三语言同一真实Host精确输入/Unit与错误。不得用当前源码审或批准文本关闭Finding；ownerlost可长期阻断是已接受代价，不用默认成功、重执行或第二receipt填补。

### A 真实 Host 场景预备与精确发布依赖

本增量只准备源码，Protocol/三客户端已完成的具名pure after不代本节Native/E2E。原Workflow checkpoint fixture和三语言WorkflowCheckpointHostTest增加独立optional A模式，原router-only prepare/resolve/verify及原fresh/CRUD/Deferred/Rejected业务断言保持。

- Store绑定：先注册capfalse legacy Store并构GraphA，再注册protect Store并构GraphB；GraphA的4个resume/branch入口仍按其原Store能力拒绝，claim/forksave/Node均0。legacy fresh Agent run、Deferred和Rejected无Node分别有正常对照，不能全封Workflow。
- 原positive response门：同一真实resume先到Store claim/protect，测试provider已提交protect但故意hold该callback的Unit回应；此时检查Node0，释放原回应后才允许registeredLlm真实Node和同attempt ack。另一个provider控制在protect提交后显式返回Error，检查Node0且无ack/requeue；此是实际error响应，不能称网络丢包/断连。
- 未结算门：registeredLlm的原实际callback内准入唯一外部Popen，foreign run/execution identity当场由该原工作分配并与invocation/extension/PID绑定，不借Checkpoint.workflow_run_id伪HostRun。返回合法typed observation时原claim/protect保留、重复resume及metadata覆盖拒绝；原exact requeue在测试Store本来可执行，由oracle要求unknown时根本未请求，避免fixture替Graph掩错。finally释放并join同PID/stop文件；外部退出不造原Host receipt，source-less unary观察不承诺自动恢复。

测试backend只是原CheckpointStore的语言provider，Memory语义仅同backend实例；不宣称File/跨provider重启、实际protect ACK丢包或Host被动退出已验证。与Graph原生FunctionNode Memory/File反例和SDKAgent/三语言链路分开记录。Java原helper修为ownedfile redirect→原10s wait→read/finally reap，修的是测试deadline，不是Host退出原因。

生产薄层只在原ExtensionAgentComponentProxy实现supports_protected_claims（读其注册snapshot）和protect_claim（单typed reverse→sameop Unit，不缓存保护状态）。原facade Graph缺cap canonical投影和direct-run stringify两条风险分支先保持，必须在新pin真实Host反例后改；前者只用原record.graph getter，Approved/resume_with_state/branch在claim/prefork/effect前FeatureUnavailable/Never及原resource，Deferred/Rejected不误拒。不能拿latest Store或Other字符串猜能力。

依赖按顺序闭合：F完整批准A/B公开main及精确CI/source审→SDK精pin/lock→唯一28profile producer，显式单cap/op接受及B Rust签名变化（不手JSON/digest）→同源Host build→原prepared Workflow direct投影单unit及A场景先真实red→仅对应两风险分支修/after→完整Host三语言与适用门禁/独审/精确main。当前framework仍00782；上述新trait/getter只按已批准冻结形状准备，未把F dirty源当发布pin，未借旧Host或编译失败做新根因red。

### 三语言原 writer 与反向调用出口

复用各 SDK 的公开 ExtensionStreamWriter.failed、原 notify 传输完成回调和原 invocation controller/stream route。完整合法的 framework_error/Never、唯一 observation=not_settled、非空 run_id/可选非空 execution_id 是观察，failed 后同一 writer 仍能按原 sequence 发送真实 Complete；成功发送该观察不删除取消地址。其余 failed 仍按原终态规则关闭，发送失败仍保原地址，wrong generation/owner 不清其它调用。setup 显式错误或抛出的原 typed error 同样保留原取消地址；局部 signal 已取消不能覆盖这份合法显式观察，普通晚到业务结果继续遵守旧 first-cause/cancel 行为。

分类严格对齐 Host 的 observation_from_sdk_error 与 EchoSdkError.validate：所有已知字段的类型、Unicode scalar 字符数、optional/null、operation 非空/256、handle 原 kind/无损 u64 generation/id 界限、fields 最多16、key 非空/128/唯一、value 2048。错误正文保留原字段，分类不补 identity、不改 payload。复用现有 wire/error decoder 的共同校验；空白使用 Rust Unicode White_Space（BOM 与 U+001C 不为空白，NBSP/NEL 为空白），拒绝未配对 surrogate，不能直接用 JS trim/Python isspace/Java strip 推断。仅为本观察分类补齐旧 decoder 缺少的检查，不重写其它错误政策。Host 对畸形 observation hint 的原 owner 安全保留属于 Host；语言 sender 不把畸形/近似 hint 扩成合法非终态。

先以原公开 writer→failed→complete、官方 ACP receive/send 和已有原 controller 测试隔离 sender 缺陷，再同精确新 Host 做跨边界场景；前者不能代替 Run 行为证明。真实 Host 场景必须使用 Session.start_run 与 Run.get/replay/wait，实际 Agent callback 入站后才启动并绑定同一外部物理 producer；finally 明确 release、join、退出码和 stopped binding。观察期间 Run 不终态、AgentClose 为0、原 PID alive；同一 producer 真退出后发送 matching Complete 要区分两类：真正没有可续接原 source/accepted result 的 foreign 出口，外部 backend 退出不自动成为 Host 物理知识，也不能补造 foreign Run 结果。这里 CustomAgent stream setup Error 实际仍有预分配 stream/原 BoxStream，不能把它叫作无 source；本轮 setup 场景故意不发送任何后继 Complete，仅 release/join 外部 PID，证明没有 matching terminal 时 Host 继续保未知。matching setup Error + 真 Complete 能续传真实结果由原 Factory delayed-consumer 强例覆盖，不以本场景反过来禁止结算；stream ACK 后原 source 仍由同一个 framework owner 驱动，真实 FinalAnswer/原 EOF 与资源释放事实齐全时，必须让原 driver/Task exact CAS 承接同一个真实结果并完成唯一 canonical receipt，不能把先前观察永久黏住、丢掉已获得的原结果，也不能重执行或新建第二 driver。普通业务完成后的 React 工具第二轮模型调用保持，held 时模型/constructor/execute 不重复。

### 最小强反例与验收

不以总数/测试名/首次绿当证据；扩展现有 owner 测试，不另建业务 scheduler/fixture terminal ledger。

|反例|必须观察的对照|
|---|---|
|wire 双向 observation|None/Some、合法2048字符内长Unicode identity精确；Never/op/handle保；普通cause全字段不变|
|真实decoder坏观察|setup RPC及stream各注入duplicate/missing/overbound，实际经过transport_error接收入口再到framework消费者；有原context时typed identity来自原context且RPC SerializationViolation，无context时原consumer/source保持未终态/资源且RPC拒绝可观察；不能直接调用helper绕过原fallback|
|Task exact CAS/owner|held cleanup超grace仍Running/同claim/interruption_requested/execute一次；exact A不影响B；真实放行仅一次原CAS；false/IO不假成功|
|background join unknown|原join只消费一次，await原identity/quota不回收/close不remove；真正terminal对照正常回收；不得is_finished/rejoin证明physical|
|setup Err observation|原RPC及时返回观察；drop外层proxy后原lease/context/Factory Arc仍保；AgentClose/新factory/配额释放0|
|ACK后stream observation|不记remote_terminal/terminal_seen、不放lease；wrong gen/owner无作用；同原有效lateclosure释放一次|
|facade workflow|观察及时返回但terminal sequence/handle/quota/原资源不变；close timeout/drop不abort；原physicalsource放行才结算；普通Complete/失败对照保|
|Sandbox Item限制|observation后heldfinally不出现IoError/Failed/fakeEOF；callbackowner/facade地址保；原匹配response/terminal后仅一次释放|
|Tool固定Item|真实低层收到带观察的Failed后不调用infallible ToolFailure完成投影、不出现ToolResult/Complete/fakeEOF；原ToolContext identity及lease/副作用保，普通cancel/timeout仍保原ToolFailure全字段|
|LLM Result槽|真实反向流观察保typed Err及原identity，低层lease未被is_terminal提前drop；同原source/physicalreceipt后一次释放，普通ChatChunk与cause不变|

Rust受控测试用真实模块/decoder及 retained guards/DropProof，先当前原消费者反例，再同条件 after。三语言用现有注册 CustomAgent/Factory、Task Store CAS及真实Host跑同corpus，完整比较errors/details、Runget无terminal、原调用次数/claim/child/quota、副作用和peer隔离。复用 FactoryHost、TaskSubagentHost 与 shared task_subagent_fixture；必要 fixture 路径须 Root 准入。不新language API，不过滤相关失败，执行由唯一producer租约协调。

framework公开main/精确CI/source review全0后才固定SDK pin/lock；唯一generator全28profiles/117artifacts，保原accepted范围并显式加入已批准A单项cap/reverse合同，其余wire路由与操作合同保持；已批准B造成的Rust source签名摘要必须真实生成并逐diff复审，不冻结旧digest、不接纳无关摘要漂移，不手写生成输出。新精确Host、三语言完整入口及适用Rust/feature/dependency门禁，再独立source/原反例/after审查。raw只local/CI Artifact，main仅必要源码/强测试/ADR/精简正式事实。当前不执行这些命令。

### 文档、取舍和回滚

ADR0039解释观察与physicalsettlement；既有SDK错误说明补Never禁止重试mutation。examples/官网无新功能/API，最终检查取消说明是否需最小同步，不复制治理日志。

拒绝默认Failed/abort（重现bug）、新status/registry/receipt（复制权威）、全Rust inventory语言镜像（扩大合同）；选择原owner无损适配。官方Tokio graceful shutdown区分通知和等待完成，JoinHandle drop为detach。本次2026-10-16已实际读取官方页面，不当项目运行证明：[shutdown](https://tokio.rs/tokio/topics/shutdown)、[JoinHandle](https://docs.rs/tokio/latest/tokio/task/struct.JoinHandle.html)。

pin/生成合同/消费者源码整体回滚到同一reviewed SDK/framework快照，不单撤fallible caller、不恢复grace abort。旧00782是回滚输入，不宣称实现B。SDK merge/main CI、unknown physical证明、历史档案必要性/continuity和registry发布分别验收。
