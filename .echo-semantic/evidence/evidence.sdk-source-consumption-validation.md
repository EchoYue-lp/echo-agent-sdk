---
schema_version: 1
id: evidence.sdk-source-consumption-validation
kind: evidence
observed_at: source:f51bfb0d8716abba38aca735c79b78ca93680aae607629db88ec49344ec3e353
source_refs:
- sdks/shared/testing/retirement_plugin_fixture.py
- sdks/typescript/test/a2a-host.test.js
- sdks/python/tests/test_a2a_host.py
- sdks/java/src/test/java/com/echoagent/sdk/A2AHostTest.java
- scripts/check-a2a-lsp-retirement.py
- scripts/check-sdk-contracts.sh
- echo-sdk-protocol/src/bin/export_schema.rs#framework_dependency_source
- echo-sdk-host/src/core_profile/handler.rs#request_publication
- echo-sdk-host/src/core_profile/handler.rs#close_session_owned
- echo-sdk-host/src/core_profile/handles.rs
- echo-sdk-host/src/core_profile/state.rs
- sdks/typescript/test/session-parent-close.test.js
- sdks/typescript/test/client-stdio-close.test.js
- sdks/typescript/test/session-parent-close-host.test.js
- sdks/typescript/test/protocol-host.test.js
- sdks/typescript/test/managed-retirement-host.test.js
- sdks/typescript/test/session-run-idempotency-host.test.js
- sdks/python/tests/test_session_parent_close.py
- sdks/python/tests/test_client_close_callbacks.py
- sdks/python/tests/test_session_parent_close_host.py
- sdks/python/tests/test_protocol_host.py
- sdks/python/tests/test_managed_retirement_host.py
- sdks/python/tests/test_session_run_idempotency_host.py
- sdks/java/src/test/java/com/echoagent/sdk/SessionParentCloseTest.java
- sdks/java/src/test/java/com/echoagent/sdk/JavaClientCloseLifecycleTest.java
- sdks/java/src/test/java/com/echoagent/sdk/SessionParentCloseHostTest.java
- sdks/java/src/test/java/com/echoagent/sdk/ProtocolHostTest.java
- sdks/java/src/test/java/com/echoagent/sdk/CallbackHostTest.java
- sdks/java/src/test/java/com/echoagent/sdk/ManagedRetirementHostTest.java
- sdks/java/src/test/java/com/echoagent/sdk/SessionRunIdempotencyHostTest.java
- sdks/python/tests/protocol_host_proxy.py
- sdks/shared/testing/session_run_idempotency_fixture.py
- sdks/shared/testing/managed_persistence_backend.py
- docs/adr/0043-native-message-sdk-adapter.md
- docs/sdk/native-messages.md
- docs/sdk/source-packages.md
- docs/sdk/governance/parquet-consumer-validation.md
- echo-sdk-protocol/tests/native_message_contract.rs
- echo-sdk-host/src/core_profile/native_message.rs
- sdks/shared/testing/native_message_host_contract.json
- sdks/typescript/test/native-message-host.test.js
- sdks/python/tests/test_native_message_host.py
- sdks/java/src/test/java/com/echoagent/sdk/NativeMessageHostTest.java
- sdks/typescript/test/runtime-terminal.test.js
- sdks/python/tests/test_runtime_terminal.py
- sdks/java/src/test/java/com/echoagent/sdk/RuntimeTerminalTest.java
- sdks/typescript/test/host.test.js
- sdks/typescript/test/package-resources.test.js
- sdks/python/tests/test_package_resources.py
- sdks/java/src/test/java/com/echoagent/sdk/PackageResourcesTest.java
- sdks/shared/testing/parquet_roundtrip_fixture.py
- sdks/typescript/test/parquet-roundtrip-host.test.js
- sdks/python/tests/test_parquet_roundtrip_host.py
- sdks/java/src/test/java/com/echoagent/sdk/ParquetRoundtripHostTest.java
supports:
- behavior.sdk-resources-streams
- behavior.sdk-language-clients
- behavior.sdk-agent-session-run
- behavior.sdk-events-recovery
- behavior.sdk-repository-ownership
- behavior.sdk-family-data
limitations:
- 原生正控使用受控模型及本根冻结dirty Framework源码构建的Host，正式f7bac pin与0dac锁不变；不是正式新pin或真实provider资格。
- 旧pin拒绝测试与新受控Host正向使用同一adapter；最终正式来源后的重建及适用全量门禁必须另验。
- 原生Stop继续、完整流与非流、三语言取消恢复矩阵、安全来源、fresh扫描、精确mainCI及发布均未完成。
- supports只支持正文列出的有限场景，不提升关联Behavior或全项目closure状态。
- 完整日志和源码指纹仅在本工作区local目录可用，失去原件后不可仅凭本摘要重建运行证据。
---

# SDK 消费接入的有限验证

## 来源与范围

本证据绑定 2026-10-28 主目录 SDK 源码观察，Git 基线为 `525453fc` 加未提交候选。正式源码的 `source:` 摘要由插件原 `source_digest` 计算，未伪装成新的 Git 提交。原始日志、精确 argv、自然退出状态、二进制、源码索引与失败原件位于本仓 `.echo-semantic/local/sdk-native-consumer-20261027/`；协调状态位于主根 `.echo-semantic/local/parallel-executors-20261027/sdk/`。

## 支持的结论

### 原生消息与终态

旧 f7 精确源的 Host adapter 9 项、两条既有转换测试各 1 项、Protocol 7 项均实际通过。首轮两个编译 warning 原件保留，删除三个未用导入后，受影响三条原命令重跑零 warning。74 固定输入零漂移，唯一 wire 候选生成 111 产物，仅主 schema 字节改变；该资格证明协议与旧源明确拒绝，不能作新源正向消费。

独立 consumer 仅持有 manifests／lock，链接原 SDK 源，Framework 为本根 path HEAD `0e404185` 加冻结 dirty1475。首次 build101 暴露生产 import 与 PromptContext 默认字段两个 SDK 接缝，修后原 build 命令自然 0、零 warning；正式 SDK pin／lock 未改。同一保全 Host SHA256 为 `41a88976b80f88c81c204be603195d79822bbce66522bb19fad308f3283bbe06`。SDK adapter 与 wire 的摘要和旧源拒绝测试切点一致。

三语言实际 stdio 各两次注册模型调用、九条公开 envelope，完整 live 与原 journal replay 一致，下一请求逐字段保留首块 provider_source。正式回归已纳入各语言原测试入口；最新 `native-regression-reviewed-ts.log`、`native-regression-reviewed-py.log`、`native-regression-reviewed-java.log` 各 1 pass、零失败／零跳过。测试核验 commentary／final 块、Unicode 正文、原生最终块→兼容 FinalAnswer→唯一末尾 Terminal/settled 的顺序。独立 reviewer 对最终固化测试返回 pass。

运行接线的定向测试 TS23／PY52／Java17 通过，覆盖坏 first_event owner、非法整数、ACK、replay 与 generation；旧 TS ACK 夹具迁移后 19 项通过。TS 原并行取消场景保留原失败和同 argv 修后 1 pass：A 取消并关闭父资源不结束 B，B 释放后自然完成，journal 及身份隔离保留。`native-parallel-runtime-fact.log` 只证明这条真实场景，不能替代完整取消／恢复矩阵。

保全 Host 无活动 reader，所有实际 stdio PID 已退出；自有 Cargo target 在核无 reader 后准确 clean，原件和制品留 target 外。Root 已明确结束 SDK 的 Framework 读窗，后续消费必须重新排窗。

### 源码包与保留数据能力

唯一语言 producer 将 schema／catalog／digest 同步到 shared 和六份包资源。TS 真实 tgz、Python wheel／sdist、Java 隔离 JAR 的默认资源加载已验证；当前 277056 schema 切点的 `package-current-schema-ts.log`、`package-current-schema-py.log`、`package-current-schema-java.log` 再次定向 TS5／PY4／Java2 全绿，零失败／零跳过。包保持 private／publish=false／source-only，未上传。

Parquet 三语言各经同一旧精确 Host 完成五次调用，独立 oracle 保中文、emoji、浮点、空值、错误详情、输入及已有输出，六个篡改负控制均拒绝。包摘要变化后通过显式保全旧合同再次实际运行，三语各 1 pass；这是真实旧 pin 数据合同，既不冒当前 schema 握手，也不证明新 Frame 的压缩依赖安全。

## 已知缺口

正式 pin、完整生成、fresh 四漏洞／十 unsound、source402 与 main／CI 仍按各自原义务处理；本证据不新增风险接受，不扩 Rust inventory 的跨语言承诺。

## 2026-10-28 原保留行为的当前复验

Root 进一步授权沿原退役 R2 定位真实保留义务，并用保全 Host 复验。执行前逐文件核对 188 项 SDK Host 相关记录路径，包含 Rust／协议／合同及 SDK 正式 manifests／lock，与保存的 SDK 构建冻结索引相同；它不是完整传递依赖构建清单。41a889 的实际构建使用独立 consumer manifests／lock 和旧 F1475 回执，不能把正式 f7 manifest／0dac 锁当作 dirty Frame 构建输入。整个 SDK 并非只新增 tests/docs：构建后 TS client 与 runtime_terminal、Python _runtime_terminal、Java RuntimeTerminal 四个生产文件已修复；本次冻结这些当前语言源码作为独立消费输入。前后全 SDK 已登记文件零漂移，Host 仍为 41a889…，没有读取新 Frame 源或启动 Cargo。

原定向入口实际 TS61／PY51／Java43，均自然 0、零失败／零跳过，共 155 项。这个计数包含客户端受控 official ACP／真实子进程所有权测试与真实 Host 场景，不能称 155 项全部是原生 Host 场景。原始 argv、自然退出、日志 SHA、七份 Java XML、逐文件源码索引及保全观察位于本任务 `retained-r2-current-20261028/`；它们不是新的源码引用或发布制品。

| 当前支持的义务 | 实际入口与边界 |
| --- | --- |
| 父 close、重试、晚到 child、observer 取消 | 三语言原 SessionParentClose 测试；失败保持原 child／RPC owner，成功后结束本地 scope，缓冲事实与原 ACK 保留，不伪造业务 terminal。 |
| 真实父子资源关闭及独立 Session | 三语言 SessionParentCloseHost；TS 包含 active default128 原 provider 取消／finally 与 settled ACK-paused1。Python／Java 两窗口的该用例是在已结算 Run 上关闭，不冒三语都覆盖 active 同一故事。 |
| 协议正负控和自定义工具取消 | 原 ProtocolHost 和 Java CallbackHost，包含版本／digest／capability／limit 拒绝、opaque handle、缺 envelope、Agent／Session／注册／Run close 重试、异步回调异常，以及原工具取消、原 call_id／输入与 interrupted 副作用后结算。代理只注入指定协议错误，不伪造成功 Agent 或 terminal。 |
| stdio 与单一关闭 owner | 原 TS client-stdio-close、Python client_close_callbacks、Java JavaClientCloseLifecycle；分别保 EOF／原子进程退出、失败重试、同一关闭 owner、取消单个观察者不取消原提交等各自实际断言，受控 transport 不冒真实业务 Host。 |
| 幂等与副作用 | 原三语言 SessionRunIdempotencyHost：同 key 同 payload 的完整返回相同，不重复模型副作用或通知；异 payload 拒绝，pre-effect busy 可重试，不同 Agent／Session key scope 隔离。 |
| managed 清理与恢复 | 原三语言 ManagedRetirementHost 的 disabled／enabled 两分支；不支持时无 durable 变更，支持时精确 generation／revision receipt、重复清理无新变更，重启 Host 后新 generation 仍保原投影。原 HTTP backend 仅扮演存储接口，原 Host／Framework 结算。 |

TS／Python 两个原 quickstart 实际自然 0。原 Java Example 实际自然 1，停在 Example.java:68 的 telemetry.status；41a889 的真实 features 只有 acp／human-loop／mcp／subagent。独立原 API 诊断实际得到 `_echo_agent/telemetry/op` 的 `transport_error: Method not found`，不是隐藏成功。另单独诊断旧 FinalAnswer 分类仍返回 completed，但 Java Example 在 telemetry 之后的其余步骤未执行，仍须完整复验。不能删除 telemetry 断言或借旧 full Host 将第三个例子标绿。

Rust 的 publication-before/after-factory、关闭 drain timeout、EOF、receipt budget／registry quota、原 official ACP request.cancel 前后窗口等控制位于 handler.rs 原测试。标准 Host 可执行文件不能执行这些 `#[tokio::test]`，保留语言场景也不替代这些精确故障窗口；当前禁止 Cargo，故列入下一窗口，未重用旧 test binary 充当前 Rust 验收。完整原 520 Rust、全语言矩阵、所有例子、正式新 pin／full profile 与新 Frame 的最终重建仍开放。

这份当前有限复验证据没有 `behavior_equivalence`／`after_revision`／全场景 matched 声明，不修改原退役 Evidence、Finding 或 closure。原 TaskSemantic 等价门禁继续保留；本次真实保留行为补证供后继按实际义务收敛，不能仅刷新旧摘要。

## 2026-10-28 最终 published F1489 的完整 profile 后继

Root 授权最终 published 索引 `0b14e25a9a593987cccab5367fbb4c53ba1fceb21f6dabaa4c682c91a910e87d`，1489 项执行前后逐 SHA 零漂移；旧prepared45e已撤回，不属于本次来源。SDK原Rust阶段1275身份项零漂移。独立consumer仅持三份manifest及自己的lock，原SDK源码／合同为链接，Frame为本根path；原workspace的Polars两patch准确转发至原vendor，没有源码或vendor副本。metadata实际证明Frame及五个vendor source为原路径，profile使用sdk-facade-all，不将正式f7／0dac伪装为此受控构建的来源。

实际full Host build自然0、零编译warning，保全SHA `32d76f9419726a682ae6be564be7d125edcd9de3aca31f933f97088ddb4f0321`。八个原publication故障窗口及一个失败reservation控制全部通过；publication_另匹配一项ACK/replay控制，实际执行10项，不能称233项lib测试全过。原接受前后取消、observer脱离、close-drain timeout、EOF、receipt/registry预算与official ACP request.cancel等精确窗口现有本切点的真实Rust执行结果。

同一full Host实际运行正式native＋原R2定向 TS62／PY52／Java44，均自然0、零失败／零跳过；仍包含原客户端／official ACP／真实子进程控制与真实Host场景，不把计数全称原生Host。三语新增断言实测执行状态事件与模型历史不含adapter_generated；只改测试和中文说明，SDK生产DTO／wire／schema没有新增字段。完整live/replay、来源坐标、Unicode正文及唯一末尾运行结算断言继续保留。

三个原quickstart全部自然0；Java原例的telemetry、旧FinalAnswer分类、prompt、Run状态／receipt及usage都实际执行通过，修复的是所用profile不匹配，没有删除或绕过原例断言。旧最小Host下Java自然1及其诊断保留原件，不被改成旧时态通过。

具名Rust声明／source hash及consumer编译影响记录在本任务 `f1489-published-full-20261028/rust-api-source-observation.json`。完整rustdoc inventory／canonical aliases／accepted聚合签名尚未正式生成；model_streaming=false的HTTP切换未由本SDK批次独立实测，不能继承Framework的HTTP控制结果。

本批全部原始argv、自然出口、日志SHA、八份Java XML、SDK分段指纹及最终1489索引原字节保存在上述local目录。相关进程与所有监督句柄终态，target递归lsof及保全可执行文件lsof均自然1、两空，准确自有target clean自然0，6077files／3.0GiB已移除、target不存在。二进制／日志在target外保全，whole-reader-return仅向Root交回，不代表本线自行释放。

独立review对本受控范围返回pass。正式pin／source402／fresh／完整生成／全矩阵／平台mainCI与原TaskSemantic等价义务不随本批关闭；原退役Evidence的旧绑定／matched与所有历史失败保持原样。

## 2026-10-28 保全 full Host 的退役动态拒绝与保留正控

Root 释放 SDK 的 F 源码 reader 后，另行授权仅使用保全32d full Host与本仓源码补动态退役控制。本段没有读取 F 源码、启动 Cargo、创建新 Host／evaluator／控制工具或执行循环，没有重启 target 构建，也不重新持有 F freeze lease。

原三语 A2AHost 入口先实际运行 TS2／PY3／Java3 全过；随后在各自原测试文件加入一项完整分层回归，最终 TS3／PY4／Java4 全部自然0、零失败／零跳过。新共享 retirement_plugin_fixture.py 只准备原 Rust PluginRegistry 测试已有的普通插件目录输入并核对记录帧；原 protocol_host_proxy.py 的 normal 模式只透明转发和记录实际进程终态。成功会话采用当前标准摘要，没有用摘要 override 迎合二进制；原故意旧摘要的用例仍是独立预期拒绝控制。

每个语言都保全了完整 client-frames／frames 及逐请求响应：39个退役family调用精确返回原始RPC `-32601`，不能解释成普通插件解析失败；19个旧source精确返回 `invalid_value`、not a canonical route及对应operation detail，是保留facade入口内的合同拒绝；三个旧LSP typed值精确返回 invalid facade argument，是参数协议层拒绝。地址和typed集合逐项与原fixture比对，非只数错误个数。

普通McpServer及两种diagnostics record共三个typed正控仅通过参数校验，随后因原非canonical retirement.probe被拒绝；本证据明确不称它们操作执行成功。另四个RPC真正成功：Agent::name前后各一次、原PluginRegistry::validate_plugin_dir一次、content-guard.redact_exact一次。普通插件返回非空skills／agents以及hooks／MCP Some路径，ResolvedComponents恰为保留五字段、没有伪空LSP字段。此差分控制证明当前full profile／标准入口可用，不能把所有旧入口失败混称profile缺功能。

三语同一Host SHA32d76f…均实核；每个normal代理记录实际Host PID和exit0，SDK关闭后所记录Proxy／Host PID均不存在，保全Host lsof自然1且stdout／stderr两空。准确argv、自然出口、源码索引、八个原用例和新增分层用例的各轮原件、完整RPC及进程观察在本任务 retired-dynamic-current-20261028/；无新F源码reader或Cargo。

这使原五场景中的caller撤除、typed插件拒绝和退役地址拒绝获得当前受控路径的动态补证；close-owner行为已有published1489九Rust＋158语言补证，三个例子已有当前0。但旧“六owner同字节”和完整三语言入口不被续用：对原observed1eac的当前核对为三份client源码已变、三份仍同字节，字节不是行为验收。完整producer／source-kind／聚合签名、正式Git pin、原完整language脚本和collector仍未闭合。

当前producer的framework_dependency_source要求metadata中的Git source，standard checker精确绑定f7，退役静态checker还要求保留行与旧基线逐字节相等及f7 manifest；受控path source=null不是该合同支持的正式after。局部Semantic的source/task摘要只绑定SDK快照，并不替代上述依赖来源资格；保持上次实际门禁1／未通过，不更改旧Evidence的after_revision／matched或producer／checker／accepted行求绿。
