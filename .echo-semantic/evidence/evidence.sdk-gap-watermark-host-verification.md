---
schema_version: 1
id: evidence.sdk-gap-watermark-host-verification
kind: evidence
observed_at: source:pending
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

# gap-watermark-host-verification

## 支持的结论

当前固定SDK实验副本仅删除ACK last_sent=max(acked)一行，原真实HostE2E同fixture得到snapshotwatermark2→resumedlive2，原sequence>watermark断言真fail101；恢复同行同command同fixturegreen1/0ignored，watermark2→live3/gapfrom4。216SDK+552framework全部buildinputs执行前后核、27归档文件逐SHA匹配，生产events e4ae808未变。

## 来源与范围

原归档receipt/输入hash/二进制保持不变。当前观察只绑定模型与可读取归档，不把原epoch冒称最终current全套通过。

## 已知缺口

Single-variable historicalmechanism reproduction, not full30dSDK or three-language historicalHost result；Oldsource30d snippet links exactcause; priororiginallogs missing not fabricated；Top-leveltaskscope paths=[]/globalnotchecked is not SDK strict/closure；WholelatestSDKpin/generation/threelanguagegap repeat/main separate。
