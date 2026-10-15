---
schema_version: 1
id: evidence.sdk-computation-45-candidate
kind: evidence
observed_at: source:2bbd6457d7b6ea031b8999d1466d85f5a5c1d2d994218d45f008ed6d5f1fa276
source_refs:
- docs/sdk/governance/semantic-history.md
supports:
- behavior.sdk-family-statistics
- behavior.sdk-family-content-guard
- behavior.sdk-family-structured-output
- behavior.sdk-family-data
- behavior.sdk-family-chart
limitations:
- 45 actual native rows/9oracle groups/16typed failures perlanguage on old64c7 only; no later189ff source claim.
- Three adjacent Pivot null/index/unknownaggregate defects were independently found after this epoch, so old pass cannot close their current findings.
- Exact accepted operations only; no new runtime structured repair configuration or fullRustidentity parity.
asset_refs:
- asset.sdk-historical-proof-location
---

# 计算簇45条共同语料候选

## 支持的结论

原64c7 Host上三原生consumer各45rows、9oracle groups、16typed failures，完整operation/argument/result/error及导出UTF8/JSON字节与sentinel保留对账。原始comparison说明只归一化临时root、WireValue map关联顺序和未承诺tieorder的Polars组/pivot顺序，其余raw数据保留。

## 来源与范围

该epoch覆盖统计、Guard、structuredvalidate与部分Data/Chart代表性场景。它不是正式已发布pin，也不能证明其后Guard/Pivot189ff的新实现。后续审查发现Stringnull/Null列名碰撞、nullindexjoin丢行、unknownaggregate默认值三个实际问题；各自新的红绿/复审来源需单列，不用原45绿掩盖。

## 已知缺口

当前新Host组合匹配语料及独立复审待归档。structured_output只接受validate，不添加运行时repair能力；未接受参数/时间接口不能为测试新增。代表性行为模型已建立，已知缺陷清楚归属，不要求穷举任意未定义组合才能建立能力图。

## 对原保存收据的当前审计观察

本对象observed_at现在指真正当前候选Git `56929154c1851473e9e4837901b610cecf001b2d`，只定位已经保存的收据、输入源码副本与manifest的审计观察。原观测字段为`source:pending`；它若是source摘要或pending，均不被伪装为历史执行当时的可恢复整仓Git。当前观察不宣称原运行发生于Git569、不重跑或升级原runtimeepoch。

全部上列source_refs均从该Gitblob实际读取，与当前archive原字节SHA精确相同，原锚点存在；原日志时间、HostbinarySHA、sourcefreeze、错误/通过计数及正文来源保持。原body的epoch/signature/cause限制与supports均为原Behavior义务的历史有界支持，不能由此宣布当前源码verified；旧3ea/9e没有完整Git时仅有原保存收据可恢复，整仓原epoch仍不可恢复，不补造树。

原保存artifact中的具名元数据（完整值仍见原JSON，不改变）：`host_sha256=64c7b4966e26acd573ddc440297e443518fc4afef60efbd39aac94da24519fda`。

本次只附加观察说明；原正文/supports/limitations保留，原rawReceipt/ArchiveChecksum/运行时间未写入，任何未决unknown/失败仍不自动resolved。源码业务Audit不是此收据观察，另队列保持原revision与复审义务，不能偷换类型或scope。


## 历史证明存储的有界表示迁移

原id/业务正文/status/supports/limitations/命令及原观察含义不改变；本冻结父 `7f14a1b9c751b0612ceb42224f83f29d09aea46c` 中此对象完整原header/source refs可由公开证据tag恢复。只将具体archive物理依赖交当前位置资产与真实same-role consumer，全部旧义务由显式Eq/CT承接，不把存储重观察冒原业务新审或旧失败变成功。
当前observed source摘要只用于新locator存储表示检查；原observed_at为 `56929154c1851473e9e4837901b610cecf001b2d`，原业务复核仍属该不可变Git。
