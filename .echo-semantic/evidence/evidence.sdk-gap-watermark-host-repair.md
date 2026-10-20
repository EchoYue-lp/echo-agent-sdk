---
schema_version: 1
id: evidence.sdk-gap-watermark-host-repair
kind: evidence
observed_at: 56929154c1851473e9e4837901b610cecf001b2d
source_refs:
- docs/sdk/governance/evidence/sdk-gap-watermark-host-red/receipt.json
- docs/sdk/governance/evidence/sdk-gap-watermark-host-red/SHA256SUMS.json
- docs/sdk/governance/evidence/sdk-gap-watermark-host-red/source-manifest.json
- docs/sdk/governance/evidence/sdk-gap-watermark-host-red/watermark-only-regression.patch
- docs/sdk/governance/evidence/sdk-gap-watermark-host-red/red-host-receipt.json
- docs/sdk/governance/evidence/sdk-gap-watermark-host-red/green-host-receipt.json
- docs/sdk/governance/evidence/sdk-gap-watermark-host-red/red-host.log
- docs/sdk/governance/evidence/sdk-gap-watermark-host-red/green-host.log
- docs/sdk/governance/evidence/sdk-gap-watermark-host-red/events-fixed.rs
- docs/sdk/governance/evidence/sdk-gap-watermark-host-red/events-regressed.rs
- docs/sdk/governance/evidence/sdk-gap-watermark-host-red/fixture-observed.rs
- echo-sdk-host/src/core_profile/events.rs#fn acknowledge
- echo-sdk-host/src/core_profile/events.rs#fn publish_replay
- echo-sdk-host/src/core_profile/events.rs#fn resume_after
- echo-sdk-host/src/core_profile/handler.rs
- echo-sdk-host/tests/core_profile_e2e.rs#ack_window_snapshot_ack_resumes_strictly_after_watermark
- docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/final-receipt.json
- docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/source-build-manifest.json
- docs/sdk/governance/evidence/sdk-language-factory-final/receipt.json
- docs/sdk/governance/evidence/sdk-language-factory-final/source-before.json
- docs/sdk/governance/evidence/sdk-language-factory-final/source-after.json
- docs/sdk/governance/evidence/sdk-language-factory-final/SHA256SUMS.json
supports:
- behavior.sdk-events-recovery
- behavior.sdk-family-event-replay
limitations:
- Single-variable historicalmechanism reproduction, not full30dSDK or three-language
  historicalHost result
- Oldsource30d snippet links exactcause; priororiginallogs missing not fabricated
- Top-leveltaskscope paths=[]/globalnotchecked is not SDK strict/closure
- WholelatestSDKpin/generation/threelanguagegap repeat/main separate
---

# gap-watermark-host-repair

## 支持的结论

当前固定SDK实验副本仅删除ACK last_sent=max(acked)一行，原真实HostE2E同fixture得到snapshotwatermark2→resumedlive2，原sequence>watermark断言真fail101；恢复同行同command同fixturegreen1/0ignored，watermark2→live3/gapfrom4。216SDK+552framework全部buildinputs执行前后核、27归档文件逐SHA匹配，生产events e4ae808未变。

## 来源与范围

原归档receipt/输入hash/二进制保持不变。当前观察只绑定模型与可读取归档，不把原epoch冒称最终current全套通过。

## 已知缺口

Single-variable historicalmechanism reproduction, not full30dSDK or three-language historicalHost result；Oldsource30d snippet links exactcause; priororiginallogs missing not fabricated；Top-leveltaskscope paths=[]/globalnotchecked is not SDK strict/closure；WholelatestSDKpin/generation/threelanguagegap repeat/main separate。

## 对原保存收据的当前审计观察

本对象observed_at现在指真正当前候选Git `56929154c1851473e9e4837901b610cecf001b2d`，只定位已经保存的收据、输入源码副本与manifest的审计观察。原观测字段为`source:pending`；它若是source摘要或pending，均不被伪装为历史执行当时的可恢复整仓Git。当前观察不宣称原运行发生于Git569、不重跑或升级原runtimeepoch。

全部上列source_refs均从该Gitblob实际读取，与当前archive原字节SHA精确相同，原锚点存在；原日志时间、HostbinarySHA、sourcefreeze、错误/通过计数及正文来源保持。原body的epoch/signature/cause限制与supports均为原Behavior义务的历史有界支持，不能由此宣布当前源码verified；旧3ea/9e没有完整Git时仅有原保存收据可恢复，整仓原epoch仍不可恢复，不补造树。

原保存artifact中的具名元数据（完整值仍见原JSON，不改变）：`startedAt=2026-10-13T16:19:35.837947+00:00`；`finishedAt=2026-10-13T16:19:48.840297+00:00`；`startedAt=2026-10-13T16:20:12.630714+00:00`；`finishedAt=2026-10-13T16:20:57.432452+00:00`。

本次只附加观察说明；原正文/supports/limitations保留，原rawReceipt/ArchiveChecksum/运行时间未写入，任何未决unknown/失败仍不自动resolved。源码业务Audit不是此收据观察，另队列保持原revision与复审义务，不能偷换类型或scope。

## 本次 current_source_rereview

这是独立reader现在实际重新读取源码/原故障假设后产生的新业务复审，未把原CodeAudit降为receipt审计。结论回收观测时间UTC `2026-10-13T23:17:58.622944+00:00`（不是原运行时间或原review开始时间），reader `framework_final_review`，当前SDK候选Git `56929154c1851473e9e4837901b610cecf001b2d`（source17cf）。

actual新当前源码业务复审PASS0：eventse4/handlerfa6/coreE2Ebe8，12strongunits与single/multi replaypage/current成功publication/失败无ACK授权/closingcaller均实际读，水位max与原唯一journal送事实保持。原snapshot2live2单变量旧机制反例不变成旧30dwholebuild；当前14/newbb4f语言覆盖的具体case已核。

真实当前SDK source_refs在上方追加，逐569Gitblob读取/currentSHA完全一致/锚点存在；新14Rust与bb4f完整三语632input/54build实际scope对应。原历史审查正文、原source marker `56929154c1851473e9e4837901b610cecf001b2d`、旧inputHash/raw/time不变，原whole3ea/9e缺失仍不造整树；这个新观察不宣称旧运行发生Git569。当前变化的调用方/feature/owner路径已在本次独立源码复审核，不由只存在锚点或count推PASS。

原支持范围、旧failure未知与limits保持；只追加这五故障的当前有界afterproof，特别ordinaryTools这里只承接Clinical，不扩大另外四族新复审。12acceptedsource迁移、5秒物理结算、依赖50warning规则仍未人批，Linux/Windows/main/fullgeneration另验；statusexamined不表示全无缺陷，41/coverage/closure/Finding状态不自动改变。
