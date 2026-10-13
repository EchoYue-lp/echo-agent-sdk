---
schema_version: 1
id: evidence.sdk-extension-teardown-prefix-verification
kind: evidence
observed_at: source:pending
source_refs:
- docs/sdk/governance/evidence/channel-bridge-callback-settlement/known-final-counters/receipt.json
- docs/sdk/governance/evidence/channel-bridge-callback-settlement/known-final-counters/prefix-before-corrected.log
- docs/sdk/governance/evidence/channel-bridge-callback-settlement/known-final-counters/prefix-after.log
- docs/sdk/governance/evidence/channel-bridge-callback-settlement/known-final-counters/prefix-fix.patch
- docs/sdk/governance/evidence/channel-bridge-callback-settlement/known-final-counters/prefix-handler-before.rs
- docs/sdk/governance/evidence/channel-bridge-callback-settlement/known-final-counters/prefix-handler-after.rs
- docs/sdk/governance/evidence/channel-bridge-callback-settlement/known-final-counters/missing-terminal-before.log
- docs/sdk/governance/evidence/channel-bridge-callback-settlement/known-final-counters/missing-terminal-after.log
- docs/sdk/governance/evidence/channel-bridge-callback-settlement/known-final-counters/missing-terminal-after-fresh.log
- docs/sdk/governance/evidence/sdk-successor-model-reviews/extension-prefix-final.json
supports:
- behavior.sdk-family-bridge
- behavior.sdk-family-channels
- behavior.sdk-extension-bridge
limitations:
- Finite fourCustom/I2prefix scope only, not currentHost162/fullbinary/main
- Initialfixturecloseowner/terminatedcompile/staleartifactfailed attempts retained
  and not runtimegreen
- Finalpubpin/gen/matchedallSDK source remains separate
---

# extension-teardown-prefix-verification

## 支持的结论

真实官方Initialize→原ExtendedCloseOwner→同Profile/Registry关闭admission→原issuedstream Chunk1Complete2，旧terminalfilter丢chunk1导致complete2sequence拒red1；fa6保完整owner/generation/bytes和单remoteledger routing后samecasegreen1。Channelactualresponse无terminal唯一guard删后实际callback已返回仍hang10.26秒红1，恢复后首次缓存旧artifact明确stale_build_cache保，freshCompiled48.19sameafter绿1(.32)。17counterfile逐SHA已原Actor独立PASS核并模型归档，当前bridge43ec/handlerfa6精确匹配。

## 来源与范围

原归档receipt/输入hash/二进制保持不变。当前观察只绑定模型与可读取归档，不把原epoch冒称最终current全套通过。

## 已知缺口

Finite fourCustom/I2prefix scope only, not currentHost162/fullbinary/main；Initialfixturecloseowner/terminatedcompile/staleartifactfailed attempts retained and not runtimegreen；Finalpubpin/gen/matchedallSDK source remains separate。
