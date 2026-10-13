---
schema_version: 1
id: evidence.sdk-channel-resource-admission
kind: evidence
observed_at: source:pending
source_refs:
- docs/sdk/governance/evidence/sdk-mcp-channel-20261013/owned-channel/resource-admission-epoch/manifest.json
- docs/sdk/governance/evidence/sdk-successor-model-reviews/channel-resource-admission.json
supports:
- behavior.sdk-family-channels
- behavior.sdk-resources-streams
- behavior.sdk-integrations
limitations:
- Exact captured-start/resource-close bug only
- Approved handlerforward new3operations/streamcontrols are not implemented/verified
  by0744
- Later source changes require separate current proof
---

# channel-resource-admission

## 支持的结论

host_actor_delta_review原独立PASS/action0，由parent回收：integration20c908/mod51a6/handles4562，0744epoch真red1→3unitsgreen、Hostlib140与三语言各5；生产resource原RW锁七operationread贯穿await/后等Registry复核，closewrite，metadata ids/list不持permit保持重入。

capturedstart曾resource墓碑后Running真红；修后原resource admission锁协调close和start，不新状态权威。0744复用五正式native场景全部绿；7bbd较早ownedcallback场景仍保独立epoch。

## 来源与范围

原归档receipt/输入hash/二进制保持不变。当前观察只绑定模型与可读取归档，不把原epoch冒称最终current全套通过。

## 已知缺口

Exact captured-start/resource-close bug only；Approved handlerforward new3operations/streamcontrols are not implemented/verified by0744；Later source changes require separate current proof。
