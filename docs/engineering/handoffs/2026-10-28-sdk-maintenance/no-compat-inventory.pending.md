# SDK：用户不保留旧版本兼容后的清理清单

本文件仅为 ignored 的源码准备与交回材料，不是正式 Goal、Plan、accepted 合同或验证结论。

## 当前权威与本轮边界

已完整读取用户亲改的 `goal-objective.md`，SHA256 为
`cd30e66fd0135d754a42c07169a5d49692c2e779f8975b6934c1ab3da498a5bb`。
用户追加“我们项目尚处于开发阶段，不需要考虑任何兼容问题，只需要把功能完美实现”。
此前与其冲突的旧版本适配义务不再作为成功标准；原始记录及历史执行结果不改写。

SDK 根为 `/Users/ls/MyWork/code/ylp_agent_learn/lp-agent/echo-agent-sdk`，当前 SID 为
`01a17d6c-e195-7f90-a8a4-8da3dc80a009`。本轮只写本 ignored 目录及 SDK 协调回执。
Framework2805 仍按 Root 指令处于 EKO reader 冻结期，本轮不读 live Framework 源码，
不启动 Cargo、Host、语言测试、生成器或新 target，也不修改正式 SDK 源码、文档、锁或合同。
旧 f7 pin、0dac 锁和各批历史资格保留原样；它们不成为继续维护旧语义的理由。

下面的范围来自当前 SDK 源码与直接消费者的有界搜索，不宣称已完成 Framework/EKO 全仓清理。
只标出分支、断言或文段；没有一项授权按整个文件名删除共享能力。

## A. 已确认要退出的两类旧接受行为；FinalAnswer 误判已撤回

### A1：缺 delivery 的旧 receipt 被接受为未知

- 源码：`echo-sdk-protocol/src/methods.rs:827` 明确写明 `delivery: Option<String>`
  只为 delivery accounting 之前的 persisted receipt 保留；`RunReceiptWire::validate:579`
  允许缺失；`echo-sdk-host/src/core_profile/persistence.rs:518` 仅在 `Some("delivered")`
  时要求 committed/observed watermark 相等。
- 旧验收：`echo-sdk-host/src/core_profile/persistence.rs:660`
  `legacy_receipt_without_delivery_recovers_as_unknown`；
  `echo-sdk-protocol/tests/core_rpc_contract.rs:590`
  `receipt_delivery_failure_is_lossless_and_legacy_is_unknown` 的后半段删除字段后仍成功。
- 删除目标：缺 delivery 的旧 receipt 成功解码/成功恢复及对应正控，不删除 receipt、
  delivery_error、失败原因、journal 或恢复能力。不只删除 serde default：Option 缺失仍
  可能解码为 None，必须在新唯一合同中真正禁止缺失。
- 新路径：原 `receipt_wire` 已始终从真实 TurnReceipt 生成 delivery（`wire.rs:700`）。
  以该当前生产路径形成必填的 delivery 事实，沿原 receipt validator 与 settle/recover
  校验贯通；保留现有 not_attempted/delivered/closed/failed 含义，不发明新分类。
- 新验收：当前版本冷启动/重启保留完整 receipt；failed delivery 的错误逐字段无损；
  缺失/非法 delivery 明确拒绝；未结算的 interrupted Run 仍无 terminal/receipt；
  watermark、terminal/receipt 一致性及副作用不重放继续验收。
- 处置：旧“缺字段也恢复成功”要求 `retired_by_user`，没有执行或通过本次新验收。

### A2：缺 Agent 定义身份的恢复记录不再作为可恢复旧资料

- 源码：`persistence.rs:56`、`:75` 对 `agent_config_fingerprint` 缺失默认空字符串；
  `echo-sdk-host/src/core_profile/handler.rs:908`、`:911` 在指纹为空时跳过 Agent 定义
  一致性比较。当前 `record_session:151`、`record_run_started:349` 正常生产路径已写指纹。
- 旧夹具：`echo-sdk-host/tests/core_profile_e2e.rs:8336`
  `seed_recovered_identity_fixture` 明确写 `agent_config_fingerprint: ""`；
  `verify_recovered_identity_admission:8360` 用该记录继续执行。
- 删除目标：空/缺指纹的身份豁免及把它当正常旧恢复输入的成功夹具。不能把所有
  serde(default) 或可选字段一概删除；running Run 的 terminal/receipt 为空是合法状态。
- 新路径：保留原 `handle_session_load`、CorePersistence 和真实 Agent config fingerprint
  权威。由当前版本实际创建并保存的 session/run 再冷重启恢复；关键身份缺失/不匹配
  返回明确错误，不借当前新 Agent、进程 cwd 或旧记录猜测来源。
- 新验收：保留原恢复身份防碰撞、load/create 区别、run_id 顺序、并发与幂等验收，
  将空指纹 seed 替换成当前原 owner 真正产生的完整记录；缺失、空、错指纹作负控，
  验证失败时不发布 Session/Run handle、不执行模型/工具、不改写旧用户数据。
- 处置：不完整身份记录仍被接受的要求 `retired_by_user`；新版本恢复本身仍必需。

### A3：运行时最终通知验收保留，原 compatibility-only 分类撤回

Root 后继明确：用户原方案保留“最终回答事件、回调、校验和收尾能力；继续由运行时触发”。
Framework 当前经 Guard 转换后的安全正文仍由 runtime FinalAnswer 承载。仅凭旧文档把
V6 通知称为“兼容 FinalAnswer”，不能判定它为独立旧工具或重复 legacy 终态。
先前 inventory-return.json 中将这类必达断言写为 retired_by_user 的分类错误，本段撤回；
旧回执原件保留，新交回使用 v2，不修改过去测试、日志或 Evidence 的 status/matched。

以下已找到21个测试/共享夹具路径，全部保留其当前必要能力验收，不整体删除或 skip：

| 路径（相对 SDK 根） | 当前验收必须保留的内容 |
| --- | --- |
| TS `native-message-host.test.js`、Python `test_native_message_host.py`、Java `NativeMessageHostTest.java` | 原生块、runtime 最终通知/安全正文、唯一末尾 ExecutionStatus、完整 history/replay |
| TS `host.test.js:142`、Python `test_parallel_sessions_host.py:66`、Java `ParallelSessionHostTest.java:38` | 并行取消、A 关闭不影响 B、真实最终通知、Session 隔离和完整 journal |
| TS `gap-ack-host.test.js`、Python `test_gap_ack_host.py` | gap 水位、ACK、晚订阅、连续 event identity 与真实最终内容 |
| Java `GapAckHostTest.java`、`DelayedSubscriberHostTest.java`、`ExternalAckHostTest.java`、`ReplayTerminalHostTest.java` | ACK、晚订阅、外部ACK、完整replay和真实最后通知 |
| TS `run-steer-host.test.js`、Python `test_run_steer_host.py`、Java `RunSteerHostTest.java` | 两次真实模型输入、Steer 接纳/拒绝、receipt、并行wait和安全最终内容 |
| TS `memory-permission-host.test.js`、Python `test_memory_permission_host.py` | Active记忆真实送模型、未批准记忆不入模型、provenance及最终输出 |
| `sdks/shared/testing/session_run_idempotency_fixture.py` | input/registration幂等、实际provider/finally次数、Session隔离和唯一真实结算 |
| `sdks/shared/testing/telemetry_host_fixture.py` | 正确最终安全正文、实际OTLP flush、metrics/traces与密钥不入日志 |
| `sdks/shared/testing/session_rules_prompt_fixture.py` | ProjectRoot/Instruction作用域、真实模型输入及经过运行时验证的最终输出 |
| `sdks/shared/testing/llm_workflow_stream_fixture.py` | 流式/非流式、typed回传、usage、finally/owner与真实最终通知 |

TS路径均在 sdks/typescript/test/，Python在sdks/python/tests/，Java在
sdks/java/src/test/java/com/echoagent/sdk/。对应当前文档为 native-messages.md 和 ADR0043。

逐producer/合同复核后，只有明确来自旧ThinkTool/FinalAnswerTool或重复legacy终态的
独立义务才可标retired_by_user；当前尚无此来源证据。若一个旧断言误把通知当执行结算，
应改成精确typed状态与内容断言，而不是取消最终通知/安全正文验收。消费者不重复concat
原生块与最终通知，展示以获准的安全正文为准；FinalAnswer也不替代末尾ExecutionStatus。
新accepted/type合同变化必须按原唯一生成链实际对账，不用本清单宣布已迁完。

## B. 需要在新唯一合同中一起切换的接缝，不先当纯兼容删光

### B1：旧 pin 的私有 serde 薄桥要换成新原生 typed 适配

`echo-sdk-host/src/core_profile/native_message.rs:25` 明确为旧 build pin 提供显式拒绝；
私有 NativeAssistantBlock/NativeExecutionStatus/NativeEvent、message_metadata、
with_message_metadata、with_delta_blocks 和 provider-source 探针使用 serde 注入/回读。
它们当前不解析旧正文 wrapper、不根据文案改判来源，也没有把缺字段默认为新功能已支持。
因此不能把整个模块当“旧 wrapper 兼容解析器”删除。

后继实际新 pin 已具完整类型时，退出仅为旧 pin 编译而绕过直接类型访问的私有代理与
注入/探测路径，改为对新 canonical Message、DeltaMessage、ToolCall、DeltaToolCall、
AssistantMessageBlock、ExecutionStatus 逐字段 typed 转换；没有旧新双轨 capability 探测。
保留原九项 native roundtrip/边界测试并转换其实现，尤其保留 raw/approved、Some(empty)、
完整 u64/u32、未知 provider phase 原文、signed/redacted/opaque 和非法身份拒绝。
源码签名与新类型尚须 Framework owner 在安全点发布；本轮没有读取冻结 F 来假定 API 形状。

### B2：CustomAgent stream 仍有 FinalAnswer→close 的生命周期耦合

`echo-sdk-host/src/core_profile/extension_bridge.rs:8552` 的 `close_factory_stream`
在 FinalAnswer/Error/Cancelled event 或普通 error 上触发 factory lease close，
EOF 也有 close；`:8715` 等六处真实流入口共用它。
`echo-sdk-protocol/src/methods.rs:4536` 的 AgentStreamTerminalWire 为
FinalAnswer/Error/Cancelled，AgentStreamChunkWire 当前没有 ExecutionStatus。

这是新 typed 发送/接收/完成合同需要核对的接缝，A3 的最终通知验收继续保留。
同样不能因出现 FinalAnswer 就把 CustomAgent result API、close ownership、取消和
ExecutionNotSettled 全部退役。Framework 的 custom-agent 原 owner、实际 callback 结束/
physical join、SDK bridge Complete/Failed/Cancelled 及 Run 执行事实需由唯一 owner 对齐；
正文候选/块结束本身不得释放仍在运行的 owner，也不得补造上层成功。

当前 Complete 若已绑定实际 callback/physical owner，FinalAnswer result 与相应 close 可以是合法收尾，不能因名字而删除；只有实际证明了候选正文提前释放才修该分支。安全点核对时，保留原 late callback、guard/lease/quota、unknown owner、
deadline、只 close 一次及 finally 语义。测试 `custom_stream_releases_original_exclusive_lease_before_terminal_consumer_close`
和 factory 原始 final value/lease 场景不能整项取消；改用新 typed stream 和实际 owner
证据继续验收。当前只做源码风险定位，没有运行复现，不声明该函数所有使用均有缺陷。

### B3：EventIdentity 的平铺 parent 兼容优先分支

`sdks/python/src/echo_agent_sdk/event_identity_values.py:127` 和
`sdks/java/src/main/java/com/echoagent/sdk/EventIdentity.java:53` 明写保留 flat compatibility；
TypeScript `event_identity_values.ts:137` 同时接受 flat 与 nested lineage。
Python 测试 `test_event_identity_values.py:94`、Java `EventIdentityTest.java:56` 固定
flat 优先，Python 还保留空 flat 回退；语言间对空值有不同旧规范。

取消为旧平铺 RuntimeContext 维持优先级的义务。后继以当前原生 RuntimeContext/lineage
的实际 typed 合同为准，只保一个读取路径，并保全 parent、run/turn/execution 身份。
当前 SDK envelope 的 `parent_event_id` 本身是合法传输字段，EventIdentity 的显式
withParentEventId 构造 API 也不是旧兼容；不可连带删除。由于冻结 F 尚未读取，最终
删除哪一条 runtime 输入路径要绑定发布后的 canonical 事实，不能仅凭注释猜 wire。
相关测试拆出旧 precedence 断言取消，nested lineage/immutable identity 正控保留。

## C. 命中旧词但不能按兼容-only 删除的路径

| 命中项 | 当前源码事实与处理 |
| --- | --- |
| `ConversationProjectionEpochStatusWire::AdoptedLegacy`（protocol managed.rs:94、Host wire.rs:1744、三语言 generated managed bindings） | 只从 SDK 能确认它是 Framework 投影原枚举；不能证明是旧版本迁移还是当前版本已有非托管 Conversation 首次接入。交 Frame owner 定性；不先删通用 Store、CAS、tombstone、projection receipt。若 canonical 新模型退此分支，再由唯一生成器一次同步。 |
| `AgentFailureWire::from_wire_value` 接受 Record 与 Map（error.rs:424） | Map 当前仍由 extension bridge 的 to_wire(failure) 生产，Record 用于 source operation。它们是当前两个活跃生产路径，不能只删 decoder 造成坏接收。需要收敛时 producer/consumer/typed错误一起改；类别/重试/原始错误及取消保留。 |
| `Token/ThinkStart/ThinkEnd/FinalAnswer`、Callback/Intervention API | 新 Goal 取消的是旧日志/协议兼容，不自动授权删除合理通用公开 API。Frame owner 若把同一能力迁到新唯一语义，SDK 随实际合同移除旧协议表面；原生 thinking、决策回调与 CustomAgent 功能不能因 EKO 不用而删。 |
| runtime-terminal 的正文/旧事件不误结算负控、TaskSubagent 的 foreign terminal 不得伪造 receipt 断言 | 这类断言拒绝错误完成，不是强制旧 event 必达。保留其不变式；迁成 canonical 新输入时仍须验证 Candidate/普通错误/外来流不能当本 Run 结算。`LifecycleTest.java:228` 是 BoundedPublisher 显式 close 后排空，不是生产按 FinalAnswer 结束的证据。 |
| `workflow_checkpoint_fixture.py` 的名为 legacy 的 false-capability Store 与文档（sdk-extension-bridge.md:156） | 这是当前可选能力正上界：不支持 protected claim 的 Store 仍可做无关操作，恢复前必须明确拒绝未支持的 claim 行为。保留失败前零副作用和通用 Store；只取消“旧二进制可以访问/不能访问旧数据”的跨版本义务，不能移除当前 capability 拒绝。 |
| config serde(default)、phase Unknown、provider_source=None、reasoning Option | 默认配置、真实未知 phase、非 provider 产生的运行时块、无 reasoning 消息均可能是当前合法输入；没有证据是旧版本 shim。不为 no-compat 把所有 optional/default 改必填。原未知 phase 文本不能抹掉，不能伪造 provider 来源。 |
| WireValue record/map/unknown envelope、ACP meta wrapper、tool stream wrapper | 当前显式协议结构、第三方 ACP 与未知值无损传输能力，不是旧 Agent 正文 wrapper。拒绝畸形当前协议仍必需，保留 tagged envelope 不等于猜测旧消息来源。 |
| OpenAI-compatible、signed/redacted/opaque、incremental/non-streaming、File/InMemory/Sqlite 框架菜单 | 第三方协议与跨产品通用能力，继续保；CLI 不启用 SQLite 不构成删除 Framework 多 Store 理由。本轮不改第三方能力或工具分类。 |
| `facade/tools.rs:382` 的 older family-qualified spelling、media 名字适配 | 单独源码注释不足以确定是旧别名还是当前 accepted route→真实 Tool 名字的薄映射；先对当前 canonical route 检验可达性，不能删当前公开工具能力、造新 taxonomy 或修改分类策略。 |

## D. 未找到时如实说明

已在 SDK Host/protocol 与三语言 runtime src 范围搜索 `agent_message`、`<think>`、
`[think]`、`[final_answer]`、ThinkTool/FinalAnswerTool/COT 控制名，没有定位到当前运行的
旧正文 wrapper 解析器或两内置工具的保留分支。该结果只说明本次 SDK 搜索范围，
不代表 Framework/EKO 没有；不新增一个转换器“识别”旧资料。历史日志中的旧内容
原样留存，不能加载后冒充新 provider 原生消息。

## 安全点实施顺序与验证边界

1. Root/原 owner 确认相关 SDK/Framework reader 交回后再写正式源码；当前只交回本清单。
2. 对新实际 canonical 源确认 B1/B2/B3 和 C 中待定项；同源的类型、Host、protocol、
   三语言、fixture、唯一生成物、所属文档一批闭合。维护当前合同正确性不等于保旧版本。
3. 正式文档后继在 ADR0043/native-messages/必要 protocol/core-profile 段落记录本次
   人类裁决及旧要求退出，保留 ADR0042 的269机器段和历史出处；Root 更新正式 Goal/Plan。
4. 正式源码最后一次改动后执行正常 focused 与原完整消费者；至少覆盖 typed send/receive、
   流式/非流式、raw/approved/history、reload/Fork/cold recovery、幂等、取消、owner join、
   副作用不重放。旧兼容正控实际删除/替换，不新增 skip、ignore 或重放工具制造新数据。
5. 只有已确证的旧目标 compat-only 验收条目（当前A1/A2）写 `retired_by_user` 并指向本次 Goal；新功能条目保持 pending，
   执行后才记录真实结果。旧 Evidence after/status/matched 和历史RED/日志不重绑。

本轮没有通过编译、运行、正式 pin/全 producer、source402/fresh、native 或 main/CI。
不改依赖风险 baseline、外部工具/source 排除，不 stage/commit/push/merge，不标总 Goal 完成。
