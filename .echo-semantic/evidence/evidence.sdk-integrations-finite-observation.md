---
schema_version: 1
id: evidence.sdk-integrations-finite-observation
kind: evidence
observed_at: ad35887c5ceb66865a714f3f2d1da86a9c35c903
source_refs:
- echo-sdk-host/src/core_profile/facade/integrations.rs
- echo-sdk-host/src/core_profile/facade/observability.rs
- echo-sdk-host/src/core_profile/facade/state_delivery.rs
- echo-sdk-host/src/core_profile/facade/source_operations.rs
- echo-sdk-host/src/core_profile/facade/permission.rs
- echo-sdk-host/src/core_profile/facade/registry.rs
- echo-sdk-host/src/core_profile/facade/mod.rs
- echo-sdk-host/src/core_profile/facade/stream.rs
- echo-sdk-host/src/core_profile/handles.rs
- echo-sdk-host/src/core_profile/handler.rs
- echo-sdk-host/tests/facade_feature_adapters_e2e.rs#agent_cascade_preserves_same_named_other_manager_and_releases_resources
- echo-sdk-host/src/core_profile/extension_bridge.rs
- docs/adr/0035-channel-handler-forward-consumption.md
- echo-sdk-host/src/main.rs
- echo-sdk-host/src/core_profile/facade/telemetry.rs
- echo-sdk-host/src/core_profile/state.rs
supports:
- behavior.sdk-integrations
- behavior.sdk-family-mcp
- behavior.sdk-family-lsp
- behavior.sdk-family-a2a
- behavior.sdk-family-channels
- behavior.sdk-family-eval
- behavior.sdk-family-improve
- behavior.sdk-family-topology
- behavior.sdk-family-permission
- behavior.sdk-family-state
- behavior.sdk-family-trace
- behavior.sdk-family-delivery
- behavior.sdk-family-testing
- behavior.sdk-family-source-operation
- behavior.sdk-family-intrinsic
- behavior.sdk-family-run
- behavior.sdk-family-telemetry
- behavior.sdk-agent-session-run
limitations:
- Original independent review bound original790/a634 runtime; currentmutable handles/A source is not relabelled as same epoch.
- A2A9operation three-native proof was added after the original missingconsumer conclusion; stale packet pendingreview text is historical, actual subsequent review PASS is separate.
- Testing14 is purevalue codec/controltransport; registry/skill supporting roles do not create new family or allsourceoperation coverage.
- Statecheckpoint7 and selectedTraceRun17 are distinct; volatileDelivery8ops no crossprocessdurability promise; only configured redaction is evidenced.
- Five-second physicalB,12sourceidentity, finalgeneration/platform/SDKmain and latestA/gates remain separate.
asset_refs:
- asset.sdk-historical-proof-location
---

# 集成能力与supporting角色有限观察

## 支持的结论

实际原独立源码审者为 `/root/framework_final_review`，当时runtime source与原790inputs精确一致，Host193e49/frameworka634。原有限scopeC0/I0/M0，而A2A当时明确缺9ops三nativeconsumer；后新增4测试源与bd8包后已有独立有限PASS，不能写成原第一次覆盖。模型作者只回收原审结论并准确补后来证据，不自行创造新的源码review。

## 来源与范围

九个运行文件在原69b与真实bd8逐Gitblob相同，integrationsc54、observability287、state_delivery21ce、sourceops91、permission84、registry5b等原审SHA相同。当前handles已因latestA变化，故本观察取恢复原boundsource，不称最新A已被全部integrations审过。

| family | 有限examined行为/authority/lifecycle/failure与证据 |
| --- | --- |
| mcp | 原prepare quota/debt先于constructor、同Arc manager/client aliases、tools/resources/prompts/readbinary、disconnect原owner；close失败retry/同名隔离/PID具名。 |
| lsp | framework请求/diagnostics完整已接受字段、nullable/kind/code/UTF16与明确malformed；URI隔离/无URI失败；manager.shutdown后derivedclient不可复活。 |
| a2a | 原第一次review具名三native缺证；后补真实193e49/a634三语各24step/9acceptedops/8HTTP、send/get/cancel/SSEartifact/typedremoteerror、owner/fullgen/close和invalidnoeffect，独审PASS另epoch。 |
| channels | 已批准六forwardops fullplugin/handler owner/gen、owned短future/RWclosing复核、metadata重入、actualRPC+remoteTerminal两fact lease、stop peer隔离/failedclose retry，cooperative非5s全physical。 |
| eval | runner/grader/report/config真实消费、Factory失败原cause、helddeadline cleanup保持workspace、trajectory原文件无覆盖。 |
| improve | 原Factory/runner/training配置/结果，取消cleanup与文件保留，公开factory非第二executionowner。 |
| topology | tracker原nodes/edges label/duration/calls/export/Clearwrongowner，无第二Agent运行权威。 |
| permission | live Memory/Agent PermissionService规则/同pair缓存，HumanLoop deadline/modifiedargs；deny0tool/0File、approve1真实File；TTL30min另3语26calls/ownerclose6PIDgone。 |
| state | File checkpoint公开7字段、大整数/纳秒冷恢复、scope隔离；不把Trace17字段/duration误进checkpoint。 |
| trace | JSONL原字节冷读、Run17选定字段/timings.total_duration_ms9007199254740993/nanos、配置sanitizer，不所有未知secret保证。 |
| delivery | accepted8ops/open MemoryEventJournal+MemoryCheckpointStore，同实例recover/exactCAS/nonowner拒绝/supersessionbeforeeffect/aftereffect claimNextNone/单activeeffect；无crossprocessdurable承诺。 |
| testing | accepted14只codec纯payload12normal7invalid，官方controltransport消费，不Mockruntime或genericmap回环Hostbusiness。 |

| supporting role | 有限范围 |
| --- | --- |
| registry/plugin | Session原constructor/query/count/loadmetadata同owner，不新增family；source_operation/intrinsic支持边界；2026-10-19后继合法状态链见文末，不借原count0扩大旧运行范围。 |
| skill | 原Agentregister/unregister/activate/query共享service、skill_info/load具名，不宣任意skill全执行/新Skill操作API |


A2A真实三语各1test/24step/9ops/8HTTP，完整Card/Task/error(-32001/Unicode)/两artifact+complete、localcancel/repeatedclose、foreignSession/futuregeneration/close后地址错误与noeffect，comparison严格fullfield不丢map/variant/sequence。测试源抓取Python/Java成功后、TS最终执行前，具体时序保，不伪未记录beforecapture；未来generation不是restart/stalepriorgen，SSEcancel只能证明Host本地adapter结算，不remotephysicalfinally。packet原pendingreview句保其历史编写状态，后真正材料/source独审PASS由Root与原审者本session补充确认，不改原receipt去伪time。

## 已知缺口

该组八lens限实际accepted入口/具名字段与failure，不使全部41Behaviorverified。TestingMockruntime、registry/skill所有未接受sourceoperation、任意backend/未知secretredaction不纳新承诺；原已接受操作仍保持。Deliverymemory无跨process保证，5秒B/wideTaskFinding/12identity及最新Afinalwhole/main另义务。原source/sideeffect/rawepoch限制保存，不从tool/client单一通过删其它groupFinding。

## 十项旧Integration/ACP具体缺陷的三链回收

frameworkFinal独立reader本session逐11原Finding匹配10closeeligible+1宽cleanuphold：这里仅收10具体defect真实repair/postfixstrongassert/原source复审回收，不凭Map名或wholecount关闭。原SDK业务source范围与publishedframeworka634真实root修复职责分开；framework文件不假放SDKGit。

### agent-close-cascade

Agentmutex/SessionRWfence/facadecleanup与同名2Manager隔离、PID退出、资源quota再create强assert，不Bphysical。 审查视角 state-authority, time-lifecycle；准确source/strongtest：echo-sdk-host/src/core_profile/handler.rs, echo-sdk-host/src/core_profile/handles.rs, echo-sdk-host/src/core_profile/facade/mod.rs, echo-sdk-host/tests/facade_feature_adapters_e2e.rs#agent_cascade_preserves_same_named_other_manager_and_releases_resources。原failure日志/已有beforeepoch/source限制保留；当前原05/13正式源切点逐具名casepassed，closure仅该缺陷。

### channel-message-handler-consumer

批准6forward/streamcontrol真实registeredconsumer/handlerinvoke/reply/originalowner/fullgen、两factcallbacklease/取消native，原start仅存handler的missingconsumer义务已补；不全physicsB。 审查视角 trigger-input, time-lifecycle；准确source/strongtest：echo-sdk-host/src/core_profile/facade/integrations.rs, echo-sdk-host/src/core_profile/extension_bridge.rs, docs/adr/0035-channel-handler-forward-consumption.md。原failure日志/已有beforeepoch/source限制保留；当前原05/13正式源切点逐具名casepassed，closure仅该缺陷。

### delivery-exact-claim

fullcallerclaim_of不重建latestattempt，oldattemptsettle/defer/transition/idonly明确拒且原整snapshot不变，supersessionbeforeeffect/aftereffectclaimNextNone。内存nonDurable不是该claimBug残余。 审查视角 state-authority, result-side-effect；准确source/strongtest：echo-sdk-host/src/core_profile/facade/state_delivery.rs#exact_claim_roundtrip_rejects_delayed_attempt_and_id_only。原failure日志/已有beforeepoch/source限制保留；当前原05/13正式源切点逐具名casepassed，closure仅该缺陷。

### family-handle-argument-projection

完整opaqueWireHandleUnicode/u64MAXDTO回原resourceparser/Registry，三native returnedHandle正常API回传，不genericrawmap绕过。 审查视角 trigger-input, contract-evidence；准确source/strongtest：echo-sdk-host/src/core_profile/facade/mod.rs#family_handle_argument_retains_opaque_resource_address, echo-sdk-host/src/core_profile/facade/mod.rs#family_argument_json。原failure日志/已有beforeepoch/source限制保留；当前原05/13正式源切点逐具名casepassed，closure仅该缺陷。

### lsp-response-fidelity

frameworkonlynullable/hoverrange/Optionkind/codeunion/objectshape解析与SDKthin、native29request/42diagnostic完整字段/nextvalid/PIDclosed复核；12sourcefieldmigration另intent。 审查视角 contract-evidence；准确source/strongtest：echo-sdk-host/src/core_profile/facade/integrations.rs, echo-sdk-host/tests/facade_feature_adapters_e2e.rs。原failure日志/已有beforeepoch/source限制保留；当前原05/13正式源切点逐具名casepassed，closure仅该缺陷。

### mcp-batch-publication

两真实server/quota1的config/file批发布，sharedreservation无未返前缀、原bestEffort不改、aliasrollback/PIDdeath/slotreuse；sameowner不误关peer。 审查视角 failure-concurrency, state-authority；准确source/strongtest：echo-sdk-host/src/core_profile/facade/source_operations.rs#register_mcp_client_resources, echo-sdk-host/tests/facade_feature_adapters_e2e.rs#mcp_config_batch_quota_failure_does_not_publish_unreturned_prefix, echo-sdk-host/tests/facade_feature_adapters_e2e.rs#mcp_file_batch_quota_failure_does_not_publish_unreturned_prefix。原failure日志/已有beforeepoch/source限制保留；当前原05/13正式源切点逐具名casepassed，closure仅该缺陷。

### mcp-manager-owner

sameSession/sameName两manager原Arcaliasmatching，disconnectA另Btools仍可调用；cascade/PID与quota强assert，不能只资源count推owner。 审查视角 state-authority, time-lifecycle；准确source/strongtest：echo-sdk-host/src/core_profile/facade/integrations.rs, echo-sdk-host/tests/facade_feature_adapters_e2e.rs#agent_cascade_preserves_same_named_other_manager_and_releases_resources。原failure日志/已有beforeepoch/source限制保留；当前原05/13正式源切点逐具名casepassed，closure仅该缺陷。

### run-steer-canonical-turn-context

根修复frameworka634 echo-core/src/agent/mod.rs + src/acp/runtime.rs reservedcanonicalturn/fallback；Hostconsumer严格identity不放宽，3nativeHTTPsecondrequestUnicode/accepted/lateempty/2wait真实证明；oldbefore/diagcompat限制保。 审查视角 contract-evidence, state-authority；准确source/strongtest：echo-sdk-host/src/core_profile/handler.rs, docs/sdk/governance/evidence/run-steer-native/initial-red-manifest.json, docs/sdk/governance/evidence/run-steer-native/java-original/receipt.json, docs/sdk/governance/evidence/run-steer-native/java-diagnostic-fixed/receipt.json。原failure日志/已有beforeepoch/source限制保留；当前原05/13正式源切点逐具名casepassed，closure仅该缺陷。

### telemetry-host-init-rollback-block

根修复frameworka634 src/telemetry.rs稳定OTEL/filtermarker/原runtimeoneowner；Hostmain/facade/state blockingcollector先settlement、RPCcancel/EOF不释放原job；原3hangOSsample→realcollectorflush/publiccloseheldreleasePIDs完整source审，noarbitrarynetwork。 审查视角 time-lifecycle, failure-concurrency；准确source/strongtest：echo-sdk-host/src/main.rs, echo-sdk-host/src/core_profile/facade/telemetry.rs, echo-sdk-host/src/core_profile/state.rs。原failure日志/已有beforeepoch/source限制保留；当前原05/13正式源切点逐具名casepassed，closure仅该缺陷。

### trace-publication-quota

maxresources1 unknownmode/jsonlmissingpath/typedwrongpath拒后真正memoryopen成功storeslen1(Registry)，原constructor在register前故不占slot；原05log78ok。不伪IOpermissionfault红测。 审查视角 trigger-input, result-side-effect；准确source/strongtest：echo-sdk-host/src/core_profile/facade/observability.rs#failed_trace_open_does_not_consume_the_only_resource_slot。原failure日志/已有beforeepoch/source限制保留；当前原05/13正式源切点逐具名casepassed，closure仅该缺陷。


## 跨仓根修复来源明确

Runsteer的真正修复源属于framework已发布Git a6346643bda47679a96760f56f45e41fd73b61b0：echo-core/src/agent/mod.rs精确纯UserTextfallback和src/acp/runtime.rs reservedTurn→AgentInvocationContext runtime.turn_id。框架ADR0084/原128+补格式取证及PR185/maintree/CI验收可读，Hosthandler仅consumer，不把它当rootrepair。

Telemetry根修复同framework a634的src/telemetry.rs：稳定委托layer/marker filter单runtime/初始化rollback，SDK本次main/facade/state持有原blockingjob/settlement。原3native挂起与OSstack原件、后after/nativecollector、公用close held→release原PIDdead、当前归档source14与publishedpin，各保原epoch。无假‘returned FrameworkError’（原init未返回），无不经过Host真实flush的信号替代。新shared supports明确包含telemetry/run，避免借原integration不含该roleAudit关闭；对应两个准确Audit新challenge为该真实读者已审scope回收，不是新engine执行。

这两项observer/repair引用原framework真实Git并以公开源码链接复核：[ACP runtime](https://github.com/EchoYue-lp/echo-agent/blob/a6346643bda47679a96760f56f45e41fd73b61b0/src/acp/runtime.rs)、[Agent fallback](https://github.com/EchoYue-lp/echo-agent/blob/a6346643bda47679a96760f56f45e41fd73b61b0/echo-core/src/agent/mod.rs)、[Telemetry](https://github.com/EchoYue-lp/echo-agent/blob/a6346643bda47679a96760f56f45e41fd73b61b0/src/telemetry.rs)。SDKsource_refs只SDK真实adGit文件，不假跨仓filename。widecleanup/sandboxphysicalB/12identity和SDKmain分别保，不因这十closed全部关闭。

## 本任务全目标差异的精确路径依据补齐

此段只给本owner对应实际证据/consumer的可复核输入性质，不定义新runtime authority、不新增接受API、不用路径数量证明行为。原observed_at ad35887c5ceb66865a714f3f2d1da86a9c35c903、原命令时间/Host/数据/限制全部不变；可直接列入source_refs的历史取证副本已在该原观察Git逐blob读取存在，其余当前test输入另由真实db34a76226732d50892be5718cd48836576aea9c和当前完整官方语言包保存，不冒旧运行时刻。

|准确路径|输入角色与范围|当前保存的原字节SHA|
|---|---|---|
|docs/sdk/governance/evidence/a2a-native-consumption/executed-source/sdks/python/src/echo_agent_sdk/client.py|原before/after/contract-basis源码取证副本；只解释此owner原故障/输入，不执行/注册/拥有新状态|30ca4fb5f2a008160290fe3664d16bb70b2d168d57b693f23bab6f2a9628e38e|
|docs/sdk/governance/evidence/telemetry-host/after/source/framework/docs/en/27-tracing.md.txt|原before/after/contract-basis源码取证副本；只解释此owner原故障/输入，不执行/注册/拥有新状态|197e191b6bc2e5275919bf22e0fa023055bd26ad7a6e2a0a08b58779801b009b|
|docs/sdk/governance/evidence/telemetry-host/after/source/sdk/echo-sdk-host/src/core_profile/state.rs.txt|原before/after/contract-basis源码取证副本；只解释此owner原故障/输入，不执行/注册/拥有新状态|beaacc87a18188d89681b6ff2712dcb890f2eed93273071eb244fd0d6f178bbf|
|sdks/python/tests/test_a2a_host.py|该能力真实native测试或共同fixture，当前输入证据；不把Ruff/输入哈希单独当行为PASS|d526d3348482cf5ea9db36b715bd752b3642897e348408611d89f85b870e4dbc|
|sdks/python/tests/test_telemetry_host.py|该能力真实native测试或共同fixture，当前输入证据；不把Ruff/输入哈希单独当行为PASS|304a4bb09a5c5e0b88667bde8d2c66c45a0b37cc4471096ab2dc756103b0909f|
|sdks/shared/testing/a2a_host_fixture.py|该能力真实native测试或共同fixture，当前输入证据；不把Ruff/输入哈希单独当行为PASS|128d81008b104e3ab6e5c25da77af3d5f6b38f8a017acb587ada0319ba1ef8e3|
|sdks/shared/testing/telemetry_host_fixture.py|该能力真实native测试或共同fixture，当前输入证据；不把Ruff/输入哈希单独当行为PASS|d4516646cb4ad760f99632ceb769f9182eb374a40f14267316f763fa95bf8bb9|

本owner表中runtime test/fixture共4项。它们是当前官方whole793的同源输入及各组具名独立证明的定位，不从输入哈希/Ruff本身推业务PASS；当前SourceGit在db34a76226732d50892be5718cd48836576aea9c可恢复。归档项只作为原before/after/contract witness，不执行、不产生第二实现/validator。Java/Python版本兼容与原门禁仍各用实际原收据，旧supports/limitations/运行时间不改，不声称所有族fresh。

## 当前Channel comparison helper的单独有限观察

sdks/shared/testing/compare_channel_forward_results.py 当前字节SHA 0a687973e1e5d5db74fcf01ea2b96afa75bb5b65a69e75f0a8023ec8cbe7293f，Root已对原三语言core+12invalidcode/callback原收据重新比较实际exit0且SDKcap有限源码复审PASS0，正式whole包保存此helper原字节。原d08helper及它执行的旧epoch不被改写成当前0a687版本；Ruff只说明格式/静态检查，不单独证明comparison业务。此项不是新native运行、额外API或原forward全部组合通过。当前保存源在db34a76226732d50892be5718cd48836576aea9c，原本Evidence观察与原raw/limitations仍保持。


## 历史证明存储的有界表示迁移

原id/业务正文/status/supports/limitations/命令及原观察含义不改变；本冻结父 `7f14a1b9c751b0612ceb42224f83f29d09aea46c` 中此对象完整原header/source refs可由公开证据tag恢复。只将具体archive物理依赖交当前位置资产与真实same-role consumer，全部旧义务由显式Eq/CT承接，不把存储重观察冒原业务新审或旧失败变成功。

## 2026-10-19 后继：source Store 与 Plugin 合法消费

原 header、source_refs 和历史运行范围保持原切点。本段只补原 accepted source_operation 的两个后端能力组；framework 仍拥有 Store/Plugin 状态，Host 只拥有 wire/owner/资源地址。没有新增 SDK API、DTO、持久化服务、依赖、framework pin 或 CLI SQLite。

| 实际原 ACP Host 用例 | 已证实消费与边界 |
| --- | --- |
| `core_profile_e2e.rs::stateful_source_resources_preserve_owner_state_and_close` | FileStore.new→两 namespace/Unicode/nested JSON put_batch/flush_public→资源关闭/旧地址 ClosedHandle→同路径重开 get/list；plain SqliteStore.new 的独立写入/关闭/重开由同组另一实例证明。bad-directory 构造 typed FrameworkError、空 namespace、wrong-kind 和 foreign Session 拒绝均实际执行。 |
| 同一 core 用例的 PluginRegistry 状态链 | with_paths 的 state/user/data/project 均来自显式 TempDir；两个有依赖的本地 plugin.json 实际 validate/install/get/list/search/count/revision、配置与 variables、disable/enable 和缺已启用依赖拒绝、scan_scopes/scan_all/diagnostics、同步/异步组件解析与依赖顺序、data_dir_for、uninstall keep_data=true 保留原 marker、最后 close/旧地址拒绝。错误配置/坏 manifest 的具体拒绝与 revision/count 不误成功，不外推任意错误都具事务保证。 |
| `extension_bridge_e2e.rs::source_sqlite_embedder_consumes_original_callbacks_and_reopens` | 原 typed Embedder 注册→同 owner 的 SqliteStore.with_embedder→真实两 put、Semantic 查询的 left/right 顺序、空向量失败未写、foreign/closed 拒绝、同 with_embedder constructor 重开后再次查询。五次 callback 的原 extension/session、唯一 invocation_id、精确 Unicode text 和返回 vector 全部核对。 |

最低充分语言组合使用本轮原官方完整入口中的具名消费者：TS `file-authority-identity-host.test.js`、Python `test_same_length_replacement_rejects_stale_guard_without_writing`、Java `FileAuthorityIdentityHostTest#sameLengthReplacementRejectsOldGuardBeforeWriting` 证明合法 source invoke 的 Path/bytes/资源地址和文件效果；三语言 MemoryPermission 的原同 Agent Store A→B 读写/search/delete、foreign resource 拒绝与原 callback 证明真实 memory/owner 消费。Variant 字段合同由 TS `wire.test.js` 的 typed/unknown discriminator、Python `test_typed_and_unknown_variants_retain_discriminator`、Java `JsonSupportTest#typedAndUnknownVariantsRetainDiscriminator` 保留。Variant codec 仅是组合中的字段证据，本次本地 install 的真实后端消费由上述 Rust ACP Host 链证明；不是 Null admission 或只 round-trip，也不改称各 backend 单独跑过三语言。原完整入口资格统一见 [cancellation Evidence](evidence.sdk-cancellation-observation.md) 的具名完整摘要，不把新增用例借成旧完整入口已执行。

最终 core02 一项通过，bridge01 一项通过；core01/02 是同一用例两个真实时间/源码切点，不重复累计。bridge01 与最终切点仅 core 测试增加 bad-directory 负控不同，其源码/Host 生产/原 lock/profile 逐字相同，因此精确承接原运行，不宣称重跑。两 test target 的 Clippy `-D warnings`、rustfmt check、最终 1212 源码哈希核对均通过；原 reader 对最终源、收据、必要性复审 C0/I0/M0。实际被测试 Host SHA 仍为 `0982121cb1e99284b4a7d7e07af35037f9f22bd653528d9716d425f5609407a0`。

- core02 原日志 556 bytes，SHA256 `ce60c9a702e4b296597748f11a3b742e02313987e001512ca2fe812f17b92df3`。
- bridge01 原日志 570 bytes，SHA256 `27f412527c7949688eab88f2eed1533655155f4d4ac18d0649422a03e903b83b`。
- 两 target Clippy 原日志 381 bytes，SHA256 `ba6c803c35260f9b50f0aaffcfd8a05bc2665ad427f18ef6390706f18d418a0f`。

当前两个原测试文件的精确执行源码 SHA256 分别是 core `03a775b04b96b1773cf1425c5b6d10a8dbf37f3e89d010a9d6b21464bd9ff277`、bridge `3569e65c86e7d21b9215c84311e2e4f603f42aa374a4cf2bace7dfdeaea38273`。原日志、guard、执行源码副本与两个测试程序只留 ignored local，未进入 Git；正式测试源码在原测试文件维护。

普通 embedder Error 在原 framework 中可降级为无 vector 的写入；本组的空向量拒写不替换该政策。语义查询重开使用 with_embedder，plain SqliteStore.new 重开另由 core 证明。keep_data=false 仅执行 base 插件移除/count0，没有带原 data marker 删除的证明。child.kill 仅作测试 teardown，不是自然 Host 退出、framework receipt 或历史 managed Linux 超时已修证据。RemoteGit 安装、任意插件执行、跨进程 crash durability 与其它 accepted 子组不因本段通过扩展。PR5/Plan 保持各自 Draft/ready，后继 runtime 两个真反例、剩余能力对账、最终新源整套门禁和精确 main 独立待完成。


## 2026-10-19 后继：剩余三组 source 消费证据

本段独立承接 FS/artifact、per-file GitCheckpoint 与 Plan task snapshot 三组原 accepted 缺证；不改变原 header/source_refs 或前段历史范围，也不将早期未运行状态抹为成功。

| 能力与原消费者 | 当前直接证据 |
| --- | --- |
| FS 与 artifact scope；原三语言 ToolValueEffect 入口 | 每入口原49行加52个有状态步骤=101行，其中99次RPC行（原48+新51），两个本地action不计RPC；19个distinct accepted source操作。真实guard、完整/不完整行、精确长度/offset、truncate、durable目录/文件与sync/remove、目录替换后的stale guard错误及replacement保留均由Host执行。三个实际conversation/run scope的persist、失败cleanup不误删、按run和conversation清理及peer原字节保留均核实际文件，Unit回复不独立当作完成。外部替换由Node/Python/Java标准文件API只在自有workroot执行，保before/after exists。 |
| GitCheckpoint；原三语言 GitToolHost 入口 | 原28行保持，新7行进入35行；三既有source方法create_checkpoint/rollback_to_checkpoint/cleanup_old_checkpoints走合法per-file链。原checkpoint bytes、tracked/index和untracked输入、HEAD/refs/peer均有实际快照，wrong-target拒绝不改目标，keep1/0真实清理。checkpoint不是Gitref，不外推事务/崩溃持久性。 |
| PlanValidator；原三语言 Computation 入口 | 原76行（含前序Evolution15）逐值保留，加3行到79；合法Vec<Task> snapshot及execution.task_id不匹配、retry_count超过max_retries两个精负控，valid及errors实际消费。不是旧TaskSpec/DAG检查替代，不执行任务或建立第二validator。 |

三个语言的两个相关原入口各自实际通过；GitPlan每语言两项、Java两份fresh XML各1/0/0/0，FS每语言一项和对应fresh XML均通过。实际Host仍为`0982121cb1e99284b4a7d7e07af35037f9f22bd653528d9716d425f5609407a0`，old6563 pin/lock未改，运行闭包804输入前后SHA一致。此处的35/79/101只描述各自payload行，不累计为能力完成分母或新的whole语言门禁。

原直接比较消费者也执行：Git --compare与Computation --compare使用同三原结果实际退出0；FS --compare首个真实before因新Unit被按旧page读取而退出1，原比较器修后同argv/原结果实际0，101行中97matched、4external observation。新分派先复用原每run链oracle，只有实际scope artifact路径作角色归一，原page内容/hash/offset/total/complete和opaque cursor断言保持；同三原结果--verify各退出0并报告99RPC。当前shared fixture除比较器及verify返回计数外，其余runtime函数AST与原native02相同、三个native entry字节相同，因此精确承接native02，不宣称改比较器后重跑Host。

原FS候选错误resource kind已在native前按真实facade_resource修正；原三native01因Python -c外部刺激被ShellTool正确拒绝而失败，原件保留，后继改stdlib action未改Host权限/解析。GitPlan v1 double-Session候选未采用、未运行；v2沿原Session.invoke只添加一次，mandatory Ruff格式AST不变，Git比较器绑定每个dataset自己的checkpoint_ids后Ruff通过。均为测试工具修正，不冒生产修复。两组最终独立source/native/必要性复审C0/I0/M0，原始资料只留ignored local。

- FS typescript: 1232 bytes，SHA256 `7154fc7bccf97ce6bb47d2090a49ea77c796f15e57e07b674c3a30e6654c36ef`。
- FS python: 99 bytes，SHA256 `a8fef0d7fe7d043a137b6932a83ad83fc1fa533644843f3b7d858521d4f458c8`。
- FS java: 936 bytes，SHA256 `4cd609ebfb8e1f8c8fb763af3f9e13a565f23cdab4a4935e1871c696f7494734`。
- GitPlan typescript: 1655 bytes，SHA256 `5e7f7fbb7747243e0eab1f93021e35ccea03de298ac00645cc48655611a082ca`。
- GitPlan python: 99 bytes，SHA256 `38d35e473bdad2a80ecfae77babe2b89db25634cadde3550db62e11c419cff72`。
- GitPlan java: 699 bytes，SHA256 `8c9d0322ad1c28f3da96b7e8490555c36134a0fcd5d1751a5b0548b4bd326f81`。

FS比较before：1165 bytes / `ea36729fe1a3b7cee5e214397de62c3061a38c848a216828d255781c2586f417`；after：27332 bytes / `8ffe4d662aa025433d2d8a0ca65dc8a10c9cf0b7e5c5c8c47de5acc70b3a8a82`。Git比较197 bytes / `598a74dc5ea68628828e57e9a7a79ae17d8f11113afdd1ce652e86a726f4c61c`，Computation比较871 bytes / `20aadf23f1268d42fb6df9fc67581719f861a8c7b0ffbad3c30d46a7072af1c8`。

前段这三组“仍待”是其切片编写时状态，由本后继段的具名实际证据承接。live service分组探针、模型/预算两个原producer与SDK model适配义务、intrinsic/ContextInheritance最后处置、最终新源整套门禁、平台和精确SDK main仍独立待完成；PR5保持Draft，Plan保持ready，不批量提升Behavior。


## 2026-10-19 两项模型与retained投影开放义务

原source/header/history不改。新 `finding.sdk-live-model-source-authority` 跟踪SDK构造配置与Session getter；新 `finding.sdk-retained-transcript-projection-identity` 跟踪accepted transcript承诺的F producer依赖。两条均open，repair/verification/rereview链为空；旧Memory/Workflow、Graphcheckpoint及F105/106不替它们，预算正确forwarding不重复新SDK问题。

模型原before在同Agent/Session、无注册或替换client、两Run completed的实际HTTP body.model为A/A而非A/B，原1621字节日志SHA256 `6f34ab2207e7dbb2115114aaefe229257ae41c334f5e2b7addff61131b75d72d`。旧Host源为8481/精pin6563/二进制098212，before TS源与请求、副本在ignored local保留；Finding的discovered_at不冒作该未提交测试整树执行版本。上游仅config写入的producer原因由F `f9906007599a9c9842474a65f4d8cb5a8a5228b8` focused后继承接，非已发布SDKafter。SDK PreparedAgentDefinition只保FrameworkConfig和ClientArc，build_agent只with_llm_client；原validated LlmConfig载荷缺口、awaitwrite Result传播与Session查询读boxed RwLockAgentWrapper缓存分别仍待SDK适配。F ReactAgent trait实际读config，原public getterA不是trait优先client。零Session定义查询与私有凭据/Debug/serde边界保留，尚未实现或验证after。

旧pin live04证明同一个既有Session晚于构造注册原A/decoyB，再显式source绑定A：实际raw/audit/marker/真实F RunStore保存与append/RuntimeCAS/持久化管理调用/Sandbox reply/Promoter后继messages等具名断言已执行；helper支持六种RunStore方法，actual只save4/load2/append18三种，实际最终两Run completed分别保13/5个events。Intent2和Skill3实际消费，非默认intent config与available_skills fence未在此case native验证。MemoryTriggerSink仅有原合法绑定、has_manager=false/零trigger，不能发明安装manager入口或宣布无条件消费完成。

该case不是整体live行为C0：04实际6batch9item中，compaction保留的旧user/toolcall/toolresult同generation/epoch1却从ordinal0/1/2变为4/5/6，再新增7/8。原内容的canonical比较与已持久checkpointfingerprint相同，一次真实shell调用不能解释成新历史事实。F retained identity既有承诺的suffix/prefix matcher及durablecursor优先路径已由原reader独立确认为producerbug；SDK只无损转发，不过滤batch、不改evict或None、不加Store/DTO/cursor。当前04真实PASS仅是原强assert及上述有限wiring消费，不证明无重投影，新增Finding必须保持open。

01首次fixture注册参数能力错误已被protocol正确拒绝；02/03的缺压缩后正规持久commit由原三点诊断解释：RAM从8条/旧final1变7条/final0，但原最后已save AgentCheckpoint仍含final，managed新turn按该authority恢复。CompressionCheckpoint不等AgentCheckpoint，04仅在压缩后调用原force_checkpoint提交，再保原nextLLM无旧final强assert。原precheckpoint/CAS/receipt/force统计不改，01/02/03日志和源均保，不把这些fixture修正称生产修复。

04实际1pass/0fail，日志563 bytes / `d3b1aa202079f1b8c8ecf3421654e5f8d03135a1e0e8837dd9fbb9181499b06d`；原注册后新Session对照actual1pass / 401 bytes / `8aa84ae43b33f375f4b63d3443141e89c2620c056f47259b79110f36fb0fee8b`；相关单testtarget Clippy-Dwarnings actual0 / 215 bytes / `bc95f4633c6103f23be0c8dffcde9560ca328bbd17d2de32af450aa2d9a72a29`，fmt check0。新2cae→88a元数据切片采用只改变已验证3formal，04 runtime源码精承接，不伪整新dirty树已执行。

04完整实际测试源码SHA256为 `a7ca4c8291112f8cca2e711ab92136dc7b441166c84de118cff7632926ac208d`；原testexec/完整typed数据/元数据guard均target外ignored保留。必须等F真实managed后端producer修复/精mainCI、SDK必要pin与原generator、模型适配与retained identity更强oracle、当前Host消费者after及独审，才可关闭两条；当前无全语言、平台或SDK main完成主张。

## 2026-10-19 accepted intrinsic与source_operation分类承接

原header/source_refs与原运行事实保留。本后继将明确accepted intrinsic551 canonical/12routes/59contract_test桶按八组、source_operation272操作/37receiver按十二组对齐原处置表；该菜单核对只界定承诺，不按计数或catalog done证明行为。ToolCallParams/ToolResult、UTF-8/JSON/usage、policy/budget/retry、局部值/trait、config构造、clock、TurnMode、ACP/Sandbox值分别复用原具名本地测试及8481/Host098212完整语言运行，未新建测试或远程intrinsic接口。该旧whole不覆盖未提交的新运行测试；新Evolution e31、Store/Plugin2cae、FS/GitPlan88a按各自定向source/raw/独审切点读取，不把计数相加成覆盖率。

具体测试锚与十二组正常消费/生命周期/不承诺/待补证并列在`docs/sdk/governance/capability-review-dispositions.md`。原四个Finding本地resolved不扩为全source通过；模型原配置/Result/livegetter与retained投影两新Finding保持open。新scalar01只旧Host下的thinking4000/tool visibility→None强例，源未入本切片；live04只是具名原A消费，RunStore实际save/load/append三类、Intent/Skill默认消费，MemoryTriggerSink managerfalse条件。新三语compressor/token-limit反例与新Fproducer after、完整新pin门禁未结算，不能把4个新压缩相关测试文件准备称已过。

本次仅分类与引用已保存的Evidence，不改变expectation/status、不伪重跑、不新增secret/permission/SDK状态权威，不将缺证暗作deferred。

## 2026-10-19 Session base/current prompt一项原Host真实反例

原header/Source refs与前段各自sourcecut不变。使用原SDK public client入口的现有compiled产物、原MemoryPermission loopback fixture及不可变Host098212的一项accepted配置调用probe：同Session inherent set_system_prompt(B)返回Unit，current_system_prompt=B前置断言通过，而Agent::system_prompt仍构造A。firstfailedassert正是expectedB/actualA，Node进程自然1；随后已收集的TraitoverrideC使current=C，base仍A、零参数definition=A。其后baseB/currentC/definitionA强断言未执行，不报after。

原999 bytes stdout/stderr日志SHA256 `124e2328a252353122a2ef7fd786199df20a2789a1b147296980ad7f2dbf07b3`；scriptSHA256 `49262336a6e7420464a3b33de2fed2573d3381a1e469aa766d621195cf00e5e3`，typed observedSHA256 `72f0f9ad3d8caff7556511f915021aa943178478e267a6f3e93a6a41dcf8181f`。ignored local session-system-prompt-probe保存精argv/Node22.22.0/80输入/原raw/进程清理；80source/runtime前后零漂移，Host098212 hash及mtime不变。Session/Agent/SDK close resolved、fixture SIGTERM回收、后继ps无该Host，且未启动Cargo/构建、未改S生产或六份未提交测试。此没有Run或框架physical receipt主张。

实际执行Host仍原8481/6563，4f是discovery源码谱系。源码已界定SDKboxed缓存的独立视图bug，新增精准finding.sdk-session-base-prompt-view保持open/空三链；模型与投影两open、预算/compressor after及原managed Linux未知不由此结算。F Traitbase getter按合同读取base config，current getter读取runtime override/当前base；最小SDK改为ownedHandle的Traitbase读取，禁止改成currentgetter/刷新cache/重组prompt来抹差异。原core admission已C0的Traitoverride→current与其它getter消费继续承接。新字段/API/方法矩阵/SDK补偿FRootSystem均未引入，当前修复候选尚未采用或编译。

## 2026-10-19 busy Session cwd真实屏障反例

一个原public SDK/原accepted load_mcp_config入口probe使用不可变Host098212：Session原cwd=work；Python stdio MCP收到initialize后向Node TCP屏障报告原PID85902并等待release，确定load持原AgentHandle.write_async。并发cwd getter实际returned_while_writer_held/null。release后原load完整成功、一个真实MCP client，后继cwd=原work且“不变”断言先过，再expectedSome/actualnull的首断言失败，Node自然exit1。300ms仅控制release fallback，不借任意sleep争抢竞态或以时长推状态。

原1249 bytes日志SHA256 `4cf1853e3a8cc0dcafd228625c39648effd2e55e39c6e6cf59dd5aefbc826379`；probe SHA256 `ed316ded7a217f2a5039b4eb67e5d1173b413cd359ba3bcdfe5cfd9130a0b304`、MCPbarrier SHA256 `05fcd9a894d064e411cb503cae14082158a6cff565444ce25650df49a0d3456d`，typed observed SHA256 `82d80327d42925eaed1030dd0f323b1a557cd95a841a257499796a900124b96c`。ignored local busy-working-dir-probe保存完整argv、原source/runtime80项hash与零漂移、raw/typed/barrier/cleanup；原Host仍8481/F6563/098212，discovery源4f，不冒整当前dirty源码运行。load/getter/Session/Agent/SDK cleanup全部resolved，原MCP PID已不存、fixtureSIGTERM与后继ps无Host；HostSHA/mtime不变，未Cargo/build/改S源。清理不当作框架Run receipt。

新增finding.sdk-busy-session-read-authority保持open/空三链，区别base/model永久cache与F root-system/retained producer。普通并发应等原writer后返回真实值，回调读则必须防父子锁等待；owner activecallback是全局事实而非本请求来源，不许伪空/初始cwdcache/新权限门或修改正确F同步Trait来补RPC。现SkillLoadPolicy持Agentwrite awaitreverse只读查询，以及compressor/Promoter持Context awaitreverse get_messages两层锁环是有界源码反证，未native执行；单数snapshot是capture副作用，原classification缺口待设计。精确可失败authority读候选与原错误通道/doc变化尚待独审，未adopt、未after、不称全callback闭合或已修其它默认getter。

## 2026-10-20 精pin356与当前有限消费者after

Root正式接受F精main3560812caede84998a1f6f26d7bf3fa81ea0486c/treebbacaad650b935c516194a9373580fe7747c8256，PR193精squash与已审8138树同、CI39355472505七项SUCCESS和remote-only必要refs冷strict已核。SDK只消费此公开pin；框架拥有configured构造/model publication/预算/内schema3/retained/root-system权威，SDK没有第二model/cursor/Store。SDK原public customArc installer仍opaque，原validated LlmConfig只从Host/env准备和explicit wire原来源私有携到sameArc构造、extensions前安装，customLLM后替换仍清配置；Result只有成功才Unit，原framework_error原样投射。base/current/definition与Agent/Context两层读guard及原typed冲突策略见ADR0034和原产品文档，源码独审C0不冒运行。

真实lock58599a9f95c7b524fe63c97a560d2bac235539570b2ed9ed6165586c793444a3，656包版本/checksum/依赖边不变，只8framework+3vendor Git来源6563→356。原HTTPS自动重试尽自然101（3208B/ab572f31a809420137cd02b61a20636707669d98042453768a307dd54443d503）后，同argv单进程既有同GitHub仓库SSH传输恢复0（3128B/a990be54b4fe4e3294f44f29db987379a2107a52e6af6be1c6f2eb519892d0ac）；canonical HTTPS依赖身份未改，未改global/system/local配置、未pathpatch或手补缓存/锁。来源/必要性独审C0/I0/M0。

唯一原完整28profile/117产物update实际0，原29523B日志SHA256 `1c735efca9880a57ec28aecaa1792fa0b4b7c69913cfe6fbf370df6968345d6b`。accepted5621 identity/kind及454routes均无增退，仅set_model的输入/operation/语言映射保持、返回Unit→Result<()>进入原成功Unit/typederror通道，top/route签名20f0cd…→d89f95…。原Checkpoint/Restored聚合与原字段digest实际不变；新增10canonical=deferred6+HostRust4，没有接受新Ctor/context_only named API。原语言catalog export0为后继epoch：sharedcatalog仅对应signature，contract-digests仅source-contract摘要变、schema摘要不变；原update后/导出前delta文件不改成同时发生。fullcontract脚本actual0，14763B日志SHA256 `ef0f4c8ecc72c7a13dccdcc064c2969cb7bb4f210baae5ae1a2675a15d6e98f5`，117current/0telemetrydrift、catalog/exact356/Protocol无Fdep检查及5/42/91测试全部0fail/0ignore。原生成/合同工程资格独审C0/I0/M0，与runtime分层。

新Host `0bbc72ba1cb08e7264704647566e490afc4918de57cc22fcee8a414fe9e518a8`（159709568B）在原guard9GiB/free5GiB/jobs1/inc0/debug0/strip、保存source opt1/assertions/overflow/unwind下真实build0。原1422807B日志SHA256 `d0c3e019fb347dfb6d569a3200fb716e8c7e6645f2d3bf12aa989e4c64058040`，1216tracked输入零漂移（非实际rustc输入全集），root Host实际rustc opt1/assertionson/无新增warning覆盖；依赖仍有Cargo标准cap-lints，不冒F自身-Dwarnings重新验。MachO EH15491112小于16MiB/unwind2979912非0，零编译warning。二进制已target外冻结，旧098212完整保留；独审C0/I0/M0只授构建与产物来源。

同Host当前有限after分为这些真实cut，原before均保，不合并为整SDK green：

| 原consumer/调用 | 实际after与具名不变量 | 原raw来源 |
| --- | --- | --- |
| 原TS模型setter selector | 1pass/0skip；HostDefault与explicit各同Session两completedRun，HTTP A/B/A/B且每variant offset差恰2；authSHA同原header/endpoint、liveB/definitionA/peergetterA（非peerHTTP）；实际describe/error/journal无随机sentinel，仅本路径隐私边界 | 558B / `011258510ee485bdae5f1a4b095eb9f9a1642b6a52466d416fbe057be9f14dea` |
| 原TS scalar与compressor两selector | 2pass/0skip；opaque不同model仅scalar自身typedFrameworkError/Never/live不变和原registeredLLM后继；thinking4000/工具hidden→None复原。Compressor原A/decoy0、3callback/3tokenizer原owner+postclose、3LLMcompleted、100000预算保护量后80000、evict2及nextLLM消费 | 1090B / `3c4ea787c3eedd9d17eb89843472bf421438671c6c9bc223bfe3a1e4e50bdb90` |
| 原PY/Java compressor各一method | 各actual1pass，JavafreshXML1/0/0/0；同3callbacks/decoy0/tokenizer/budget/evict与nextLLM原verifier。Java实际21.0.5，非Temurin17/非whole语言 | PY98B / `57be52af4623aba7a288aead2dd5009536a86c289e7cfad4320fe4586d4c738b`；Java537B / `60354d8705c13a33f2069f7d00ecde24e97cd5fc95386455739ef554fd89b0a6` |
| 原base/current publicSDK脚本 | actual0，before/after脚本仅expectedHostSHA098→0bbc；inherent后base/current=B，Traitoverride后base=B/current=C/零Sessiondefinition=A，全部强assert执行 | 239B / `df999d6893d3cd90d031f8f0c540a977fdb168c2bcc0d9ed99861389a9768bb9` |
| 原MCP initialize barrier busy脚本 | actual0；原writerheld时getter pending，release/load后同Some；300ms只是release fallback，不作成功延迟断言；load/getter/Session/Agent/SDK完整await、MCPPID不存 | 223B / `3d7f5d0fbbb324cad0b48a88a080760fe12998bb7adbfc4b1e53949d665696f5` |
| 原Rust core admission | 1pass/32filtered；真实CARGO_BIN_EXE同Host0bbc，新base/current/definition与旧owner/错误/关闭断言完整执行 | 540B / `dc3f58150d10f627f2938b8b7006d1d847e4fb761fdcebcf1bcd5ee368066491` |
| 原Rust live Session case03 | 1pass/36filtered；同callback内Promoter先model/base2成功，再Context messages/stats冲突及capture mutation冲突；Policy cwd冲突/2badarity原InvalidValue/Never/METHOD精message；原response等ALL子RPC验证后、父allowed/promoted/2Run及后继强断言都执行 | 563B / `6d4e1f304bbb2e24c2080a7537d707834dcb3fb08fdf5225f766d4a0ba61c4b6` |

前六组按各自1216冻结输入、sourcehash/Hostmtime、原件与直接断言读取；live03新epoch只多test fixture delta，生产Host仍0bbc、helperbody/原livecase及后继字节保。紧邻独立`agent_components_are_consumed_by_new_session_agents`在本次36filtered，只有原源码carry，不能称本次执行；后续完整target自然覆盖，不新增focused矩阵。当前只有一SessionCreateRequest的live场景，不伪两Session/ordinaryprofile。

live01实际E0382编译101（1128B/574216c440fe34beb123fdbc51decbe1a4569924fcbbde0c718be38f2dbbc78d），未执行native；原Audit tokio spawn移动outer connection，新getter helper后借用被拒。单block-local clone保同transport修后，live02编译0但native discovery真实空列表失败（1114B/17f94caaac444e58a04149ec5624a7059aea2d395259dca8d1fba488408a1840），未到retained。根因是测试Client ACP2.1有序入站handler内await block_task，阻塞同actor路由reply；官方ordering/actor源码与原spawn模式证明，未实测Host已发响应，不拿4.31s当Host锁错误。额外诊断只准备未编运行，保ignored后移除；修复仅live Policy/Promoter verification+原typedresponse移同ConnectionTo task，仍同originalresponder在子RPC断言全完后才回，Err/panic显式原responder/errorfact、sendErr传播，非live旧结果保。两夹具修复源码增量均独审C0/I0/M0，03实际成功的因果验证与原失败原件分开，不当产品bug或改oracle/2s/selector。

03 typed数据实际5batches/6items，三唯一原user/toolcall/toolresult同epoch1/gen各只一次ordinal0/1/2，不再旧04的0/1/2→4/5/6重投。新真实framework messages_json中17处schema3/sidecar绑定观察经原RuntimeStore/save/load/resume链；SDK仍原样String，不加DTO/cursor/过滤。原scope/name/receipt/CAS强检查仍在，RunStore helper支持六类但不称六类全消费；MemoryTriggerSink无manager仍仅条件绑定，Intent/Skill仍不夸非默认branch。03完整typed数据2558030B SHA256 `e64eaf923cf0eab110d94f8728f8123b318b23421674ccf38fef0c9fe733a677`，testexec33996528B SHA256 `70ce6aad91159969a7275eabfdf9100be236c43fdce72b68cf8721fa29a88f0c`，源码3aa0019e5a58e781d5d3bd3d575fff0f631ecffa86298966a33c7f120881782b和1216guard零漂移；原getterreply/readfacts未另存，按compiled exactsource+passed strongassert授，不捏rawframe证据。原source/runtime reader当前finite03独审C0/I0/M0，不授wholecallback/自然Hostexit/retirement或main。

所有原件和每epoch inputs/runner/proofs、源码副本、原testexe统一ignored local sdk-parallel-delivery-01a14385，本机可用范围明确；没有raw移Git。当前currentlead/deferred最小后继只更新时间事实；四个source Findings还按repair/verification/rereview三链单独结算，managed原Linux未知保open。原14Rust、同Host唯一官方整语言与3qs、新lock依赖扫描、最终source必要性/currentsemantic/nativeHook/精SDKmain CI及onlyremote-main冷strict仍待；所有Behavior不批量verified，PR5 Draft。
