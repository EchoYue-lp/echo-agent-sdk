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
| registry/plugin | Session原constructor/query/count/loadmetadata同owner，不新增family；source_operation/intrinsic支持边界 |
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
