---
schema_version: 1
id: evidence.sdk-integrations-finite-observation
kind: evidence
observed_at: bd8e42e732588181b7a2589d91e9ffdce02d23e1
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
- docs/sdk/governance/evidence/a2a-native-consumption/receipt.json
- docs/sdk/governance/evidence/a2a-native-consumption/manifest.json
- docs/sdk/governance/evidence/a2a-native-consumption/comparison.json
- docs/sdk/governance/evidence/a2a-native-consumption/README.md
- docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/receipt.json
- docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/inputs-before.json
- docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/inputs-after.json
- docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/java-surefire/TEST-com.echoagent.sdk.TelemetryHostTest.xml
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
limitations:
- Original independent review bound original790/a634 runtime; currentmutable handles/A source is not relabelled as same epoch.
- A2A9operation three-native proof was added after the original missingconsumer conclusion; stale packet pendingreview text is historical, actual subsequent review PASS is separate.
- Testing14 is purevalue codec/controltransport; registry/skill supporting roles do not create new family or allsourceoperation coverage.
- Statecheckpoint7 and selectedTraceRun17 are distinct; volatileDelivery8ops no crossprocessdurability promise; only configured redaction is evidenced.
- Five-second physicalB,12sourceidentity, finalgeneration/platform/SDKmain and latestA/gates remain separate.
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
