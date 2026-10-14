---
schema_version: 1
id: evidence.sdk-stream-cancellation-routing-repair
kind: evidence
observed_at: 56929154c1851473e9e4837901b610cecf001b2d
source_refs:
- docs/sdk/governance/evidence/sdk-stream-cancellation-routing/README.md
- docs/sdk/governance/evidence/sdk-stream-cancellation-routing/final-source-manifest.json
- docs/sdk/governance/evidence/sdk-stream-cancellation-routing/evidence-sha256.json
- docs/sdk/governance/evidence/sdk-stream-cancellation-routing/client-routing-source-slice.patch
- docs/sdk/governance/evidence/sdk-stream-cancellation-routing/before-source-manifest.json
- docs/sdk/governance/evidence/sdk-successor-model-reviews/stream-cancellation-routing.json
- sdks/typescript/src/client.ts
- sdks/python/src/echo_agent_sdk/client.py
- sdks/java/src/main/java/com/echoagent/sdk/EchoAgentClient.java
- docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/final-receipt.json
- docs/sdk/governance/evidence/sdk-language-factory-final/receipt.json
- docs/sdk/governance/evidence/sdk-language-factory-final/source-before.json
- docs/sdk/governance/evidence/sdk-language-factory-final/source-after.json
supports:
- behavior.sdk-family-task
- behavior.sdk-family-subagent
- behavior.sdk-family-bridge
- behavior.sdk-language-clients
limitations:
- Client cancellation routing localrootcause only
- No fullscript/fullcontract/main orframeworkphysicalworksettlement after5sec
- Originalrootcause210archive andfirstwhole285/2failure preserved
---

# stream-cancellation-routing-repair

## 支持的结论

真实原3clientsetupACK后finally删除invocation取消地址，Host同invocation13cancelroutefalse导致A1无signal/terminal、A2exclusiveconflict。修后原issuedstream完整id/generation/kind/extension保持取消地址直到成功officialterminal/unregister/actualclose；earlycancel/terminalbeforeACK/wronggen/failedsend/failedclose明确。9source27archiveSHA一致，TS19/PY39/J27零skip且Task4每语言/ACK12/23/18包含；独立sdkcapabilityPASS0C0I0M。

## 来源与范围

原归档receipt/输入hash/二进制保持不变。当前观察只绑定模型与可读取归档，不把原epoch冒称最终current全套通过。

## 已知缺口

Client cancellation routing localrootcause only；No fullscript/fullcontract/main orframeworkphysicalworksettlement after5sec；Originalrootcause210archive andfirstwhole285/2failure preserved。

## 对原保存收据的当前审计观察

本对象observed_at现在指真正当前候选Git `56929154c1851473e9e4837901b610cecf001b2d`，只定位已经保存的收据、输入源码副本与manifest的审计观察。原观测字段为`source:pending`；它若是source摘要或pending，均不被伪装为历史执行当时的可恢复整仓Git。当前观察不宣称原运行发生于Git569、不重跑或升级原runtimeepoch。

全部上列source_refs均从该Gitblob实际读取，与当前archive原字节SHA精确相同，原锚点存在；原日志时间、HostbinarySHA、sourcefreeze、错误/通过计数及正文来源保持。原body的epoch/signature/cause限制与supports均为原Behavior义务的历史有界支持，不能由此宣布当前源码verified；旧3ea/9e没有完整Git时仅有原保存收据可恢复，整仓原epoch仍不可恢复，不补造树。

本次只附加观察说明；原正文/supports/limitations保留，原rawReceipt/ArchiveChecksum/运行时间未写入，任何未决unknown/失败仍不自动resolved。源码业务Audit不是此收据观察，另队列保持原revision与复审义务，不能偷换类型或scope。

## 本次已复审scope的当前证据重观察

当前观察Git`56929154c1851473e9e4837901b610cecf001b2d`仅据下面明确源码业务复审的已接受路径，回收UTC`2026-10-14T00:39:15.355735+00:00`；原marker`56929154c1851473e9e4837901b610cecf001b2d`、旧runtimeepoch/time/sourcehash/raw全文保留，原support/limits不变。不是过去执行就发生569，不假历史整仓Git恢复。

- `audit.sdk-stream-cancellation-routing-rereview` reader`framework_final_review`：**PASS / 0C0I0M（本次有界源码业务复审）**；本次原故障检查SetupACK retains originalcancel address complete streamid/gen/kind/extension;earlycancel/terminalbeforeACK mustnot resurrect route；Only officialterminalsuccessful submission orsuccessfulunregister/actualclose releasesexactowner;wronggenfailedsend/failedclose keepsit。after仅Current3nativebeforeafter +routewindows/task4 each/ACK12/23/18;exclude generic>5secphysical。

所有新SDKsource/proof refs按真实569byte/anchor读取，原业务与原sourcefreeze记录保持；这不是一切currentfamily/runtimepassed。ordinaryTools已有原五族收据只新增本次明确Database/Artifact检查，不暗称其余方言/provider/工具所有field均新审；原candidate supports=[]不升。任何缺原完整time/tree仍在原limits，12/B/depWarn人未答/fullgeneration/CImain待。
