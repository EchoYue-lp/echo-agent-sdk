---
schema_version: 1
id: evidence.sdk-language-current-first-gate-failure
kind: evidence
observed_at: 56929154c1851473e9e4837901b610cecf001b2d
source_refs:
- docs/sdk/governance/evidence/sdk-language-current-first-gate/scope.json
- docs/sdk/governance/evidence/sdk-language-current-first-gate/receipt.json
- docs/sdk/governance/evidence/sdk-language-current-first-gate/full-language.log
- docs/sdk/governance/evidence/sdk-language-current-first-gate/task-store.json
- docs/sdk/governance/evidence/sdk-language-current-first-gate/task-host.json
- docs/sdk/governance/evidence/sdk-language-current-first-gate/task-fixture.json
supports: []
limitations:
- RootcauseSDK/framework ownership stillbeing debugged
- No localrepair/verify/rereview orfinalwholepass claimed
- EOFspecificscope priorresolved retained; oldTask4historicalgreen not currentresumeproof
---

# language-current-first-gate-failure

## 支持的结论

官方check-language-sdks.sh首次actualTS287/285pass2fail0skip exit1，PYJava未运行。Taskpersisted12CAS实存A paused→pending→running→failed extension_conflict exclusiveinvocation，B/Ccompleted canonicalresult保持、Dpending、agent_calls无A2；这不是可预判纯测试超时。另一项sharedstate_root OS35原log保，fixtureowner修不弱fileauthority。

## 来源与范围

原归档receipt/输入hash/二进制保持不变。当前观察只绑定模型与可读取归档，不把原epoch冒称最终current全套通过。

## 已知缺口

RootcauseSDK/framework ownership stillbeing debugged；No localrepair/verify/rereview orfinalwholepass claimed；EOFspecificscope priorresolved retained; oldTask4historicalgreen not currentresumeproof。

## 对原保存收据的当前审计观察

本对象observed_at现在指真正当前候选Git `56929154c1851473e9e4837901b610cecf001b2d`，只定位已经保存的收据、输入源码副本与manifest的审计观察。原观测字段为`source:pending`；它若是source摘要或pending，均不被伪装为历史执行当时的可恢复整仓Git。当前观察不宣称原运行发生于Git569、不重跑或升级原runtimeepoch。

全部上列source_refs均从该Gitblob实际读取，与当前archive原字节SHA精确相同，原锚点存在；原日志时间、HostbinarySHA、sourcefreeze、错误/通过计数及正文来源保持。原body的epoch/signature/cause限制与supports均为原Behavior义务的历史有界支持，不能由此宣布当前源码verified；旧3ea/9e没有完整Git时仅有原保存收据可恢复，整仓原epoch仍不可恢复，不补造树。

原保存artifact中的具名元数据（完整值仍见原JSON，不改变）：`host_sha256=8d81900442aaf97de4a4287a1cc9d11aae63b15f8f3b5a89b6811347806fdc10`；`startedAt=2026-10-13T19:46:46.023Z`；`finishedAt=2026-10-13T19:48:10.218Z`。

本次只附加观察说明；原正文/supports/limitations保留，原rawReceipt/ArchiveChecksum/运行时间未写入，任何未决unknown/失败仍不自动resolved。源码业务Audit不是此收据观察，另队列保持原revision与复审义务，不能偷换类型或scope。
