---
schema_version: 1
id: evidence.sdk-events-resources-finite-observation
kind: evidence
observed_at: cd5b78467f77ac28919cf9dda6877662bed744f4
source_refs:
- echo-sdk-host/src/core_profile/events.rs
- echo-sdk-host/src/core_profile/persistence.rs
- echo-sdk-host/src/core_profile/handler.rs
- echo-sdk-host/src/core_profile/wire.rs
- echo-sdk-host/src/core_profile/handles.rs
- echo-sdk-host/src/core_profile/state.rs
- echo-sdk-host/src/core_profile/facade/stream.rs
- echo-sdk-host/src/core_profile/facade/mod.rs
- echo-sdk-host/src/bounded_stdio.rs
- echo-sdk-host/src/config.rs
- echo-sdk-protocol/src/event.rs
- docs/sdk/governance/evidence/live-journal-replacement/README.md
- docs/sdk/governance/evidence/live-journal-replacement/receipt.json
- docs/sdk/governance/evidence/live-journal-replacement/manifest.json
- docs/sdk/governance/evidence/live-journal-replacement/comparison.json
- docs/sdk/governance/evidence/sdk-a634-rust-gates/receipt.json
- docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/receipt.json
- docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/inputs-before.json
- docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/inputs-after.json
supports:
- behavior.sdk-events-recovery
- behavior.sdk-family-event-replay
- behavior.sdk-resources-streams
limitations:
- Source audit is the recorded original69b scope, whose referenced runtime blobs are unchanged in cd5; it is not an observation of active Session/Run A mutations.
- ACK author read-only audit is not renamed independent review; existing independent reviews support their exact event/stream/client/native slices.
- Settled cached-journal replacement with a live connection is covered; active-write/partial-write/crash injection is not inferred.
- Five-second physical producer settlement and broad cleanup policy remain open; cancel/EOF/kill requests are not physical completion proof.
- Source operation methods outside the accepted event/stream/resource owner scope, full generation/12 identity choice/platform/SDK main are separate.
---

# Events 与 resource stream 有界观察

## 支持的结论

复用现有 Events/Resources 两张Map和三个Behavior，不新增第42family。event-replay为具体已接受能力，resource stream是多能力共享资源权威。ACK lane原作者实际只读审查了八个风险视角，无新增源码阻断；这份作者审查不伪称独立reviewer。原Host e4水位和12units、clientACKv4及producer/next/cancel/Registry各已有各自精确独立review；live-journal新补证另由currentTool独立有限PASS0。

框架EventJournal唯一committed identity/sequence/terminal；Host StreamDelivery只管理发送窗口/成功发布coverage。publishReplay仅成功responseenqueue授权实际返回尾/retention snapshot；emptyfuture不给权限。fullhandle ACK不退水位，failedresume保已接受ACK并paused重试。客户端仅官方writercompletion后的实际连通coverage提交消费cursor；fetch、pending、failed/cancelled不充洞或消费terminal。

统一Registry验证Session/kind/owner/fullgeneration，资源操作readpermit及关闭后复核；容量1receiver和唯一JoinHandlecollector只记录producer事实。next串行取项，panic/abort/EOF不假Complete，pendingfailure保留，terminal分配与cancel同锁。close先closing/cancel唤醒原工作，再await原settle/writepermit/cleanup，成功才tombstone；失败保同owner/地址retry。known cooperativeDeadline不假全物理停止。

## 来源与范围

观察proofGit cd5，所列11个runtime/protocol源码blob与原审69b逐字节相同。当前workspace的handler/handles/state已因新的Session/Run意图A修改，故不能称本观察是最新mutablesource或整体SDK最新全绿。原作者审查以76601时dirty但实际790输入一致的e4/b78/1d977/95/fa6等源开展；69b后真实提交恢复这些源码事实，cd5只追加回归证据。

原a634 Rust日志events12/stream11及handlequota/admission/failedMcpclose/restartgeneration具名通过；Hostsnapshot/single/multipage ACK严格>水位。原客户端正式native/replay/ACK窗口与同实例恢复强断言分别引用旧Evidence，不用总数代能力。原旧水位singleline真实red/green和Git30d机制链保留，#120已局部resolved，不做外部Issue关闭。

live-journal包真实TS12/PY12/Java9全部零失败/跳过，以相同Host193e49/frameworka634：同一个settledRun、liveHost/connection/cachedjournal，物理rename原目录并保原inode，再创建emptyreplacement。两次公开replay必须得到replay_unavailable/after_delay/原operation/nullhandle/details；原Runreceipt和peerSession整page不变，restore原inode与两文件SHA、完整page相同。WireValue map仅按完整entry multiset序排序，重复值/IDs/hash不删除。初始Python枚举序oracle失败保历史，不冒生产redgreen；测试作者原model业务AST/原用例未改，docstring增量精确注明。

## 已知缺口

这是具名行为、权威、生命周期、错误与恢复的finiteexamined收口，不把所有资源种类/未接受方法穷尽。宽cleanup与5秒B原Finding保，12identity/生成/平台/main独立。两个Map旧仍缺及Channel旧publishedpin句在后继段明确历史，旧raw/Audit/sourceepoch不换新。最新SessionRun意图A与远端typedcause修复各有自己的切片，不能借本资源collector或wireerror证明全部Host转换无损。
