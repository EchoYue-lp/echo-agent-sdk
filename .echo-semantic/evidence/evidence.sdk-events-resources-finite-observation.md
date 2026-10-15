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
asset_refs:
- asset.sdk-historical-proof-location
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


## finding.sdk-gap-duplicate-ack-successor-observation 的有限三链出处

本段当期观察e58只绑定当前可恢复测试源、原因果/修复和新实际CI，不改原observed_at、旧source_refs/command或其它原缺陷状态。

原722 Python True-window line132 4!=3，不从数量差猜生产重复ACK新effect。字节保留的真实Host/gate证明前一合法ACK的gap4在dupACK2之前已由Host发出；原测试只等任何通知即捕获announced，形成receive-baseline race。三caller修复ff3等同完整stream id/generation/kind、from_sequence及snapshot_watermark都越过原watermark的successor paused gap，模型仍parked且release不存在后才记announced；dupACK后原exact count equality和完整successor frame equality不改，futureACK/replay/covered-noresend/terminal/cleanup强断言都保。

原ff3正式docs/sdk/governance/evidence/gap-successor-pause-oracle/可按其Git恢复：native/receipt.json SHA0c5a251e23860e16470e3f1845ba5d96e3677b8ed3dab042ab30dc151638fda2，controlled-final-receipt.json SHAfb2df55a6784b92725963870585b7c28e634c3254a81428e2633cce61a7ef16e，independent-review.json SHAeb72140598ccea84de99f51eae878fbb131bdcede475dd95ecd4a9f8a7849420。原TS4selected/8 unrelated namefilter skips、PY3.12.4四/0skip、Java1method4rows/0fail/error/skip不是whole；controlled retry正确cwd/1pass/0warning/723B log SHA83b1a076390d03397e5c9f2cb7bdd9aa2b1f7c8015eb12b923f68f590a0f7449保旧warning invocation。原12rows执行HEAD722dirty、retry f68、seal e13、保存ff3分开。FrameFinal原有限源码/controlled before-after/12rows/字节包独审PASS，不冒本次重跑/全family。

本次后验是[e58官方语言job116293151777](https://github.com/EchoYue-lp/echo-agent-sdk/actions/runs/38696176970/job/116293151777)与[Hostjob116293152091](https://github.com/EchoYue-lp/echo-agent-sdk/actions/runs/38696176970/job/116293152091)各自自然终态success；head e58a3e0c07cf0b6b24d570df07063301a888ddc6、actual PRmerge e9c7275c6f5df5652e555aa4f457c65870676b34、tree c48b54ec7a58bfe855d2724f777b1d605789b7a2与已提交源码树一致。语言原raw `/Users/ls/MyWork/code/ylp_agent_learn/lp-agent/.worktrees/admin/sdk-e58a3e0-ci-evidence/116293151777.log`，200420B/SHAaec6371714b2d2bd23612b3b6d139acccf507a541db327ef13c5c744f5115cce；TS343、Python427+18子测试、完整Maven test按set-e成功且三个quickstart均实际到达/finalmarker通过，Node20.20.2、Temurin17.0.20+1。成功静默Maven没有打印Java数量，也没有下载XML，不编Java总数。Host原raw `/Users/ls/MyWork/code/ylp_agent_learn/lp-agent/.worktrees/admin/sdk-e58a3e0-ci-evidence/116293152091.log`，164620B/SHAecc88119b62e9a9311db1dcc7860082a1b18131d584b66889ab6fa1defdf56d8：default35/core111/bridge28/facade44/noBridge selected1+24filtered各原profile通过，不重叠求和；managed restart case ok不倒推原passive-exit原因已查明。两job warning/error annotations均0，只用于各自原切点，不称全部CI或SDKmain已过。Admin language-host-success-facts.json保两job已成功组合；单独language-terminal-verification内Host未终态的资格文字是较早收据阶段，不重写原JSON或误作当前阻断。

|当前测试源码（e58与原修复Git字节同）|SHA256|
|---|---|
|`sdks/python/tests/test_gap_ack_host.py`|`aa462130216730a6541982799e07b679eb4ceb2e9590c6cd0ec161318c3045eb`|
|`sdks/typescript/test/gap-ack-host.test.js`|`3ad9a6caa40611e9a18a17f8ea9e3530320549454b482028dd9cb810bd719b5e`|
|`sdks/java/src/test/java/com/echoagent/sdk/GapAckHostTest.java`|`017fa75ec0ce091e7def325dfc7dfd2048fbefbe6bfe2067aaccc69aef468737`|

仅这些测试源同原c5/ff3。原focused使用较早客户端，TS/PY生产69cb修复了另一个ACK竞态；e58完整官方脚本的实际合并树与输入闭包补足新客户端组合，不把旧source哈希不变推全部输入不变。原强消费者断言在未过滤完整脚本执行，Host0f1只属于原Mac focused，不套远端Linux Host二进制SHA。

具体Finding局部三链引用本段及对应Audit具名故障复核，仍保Host未知退出、B三宽义务、accepted12/合同13、平台/main、全局continuity1173和4365档案必要性精简阻断。没有新runtime状态权威/validator/协议/accepted schema/公共API；examples和官网无需接口同步，代码/测试本次不改。


## 历史证明存储的有界表示迁移

原id/业务正文/status/supports/limitations/命令及原观察含义不改变；本冻结父 `7f14a1b9c751b0612ceb42224f83f29d09aea46c` 中此对象完整原header/source refs可由公开证据tag恢复。只将具体archive物理依赖交当前位置资产与真实same-role consumer，全部旧义务由显式Eq/CT承接，不把存储重观察冒原业务新审或旧失败变成功。
