# 当前能力模型的有限剩余验收

## 当前有限验收（2026-10-15）

SDK Host 精确依赖已发布框架 `00782b8a`，八个框架包同一 Git 来源、没有 path patch；框架合并后七项 CI 已成功。[14 条 Rust 门禁](evidence/sdk-00782-final-rust-gates/README.md)实际全0，workspace 主计数474，不累加重叠 feature profiles。旧 [00782 完整语言入口](evidence/sdk-language-00782-reviewed-final/README.md)的329/394+18/351与793输入保持其历史切点。[CI 修复后的第四次完整语言入口](evidence/sdk-language-ci-repairs-final/README.md)真实退出0：TypeScript336、Python415加18子测试、Java355/76份fresh XML及三个quickstart；800项执行输入在该次运行前后相同，唯一未提交Python客户端修复随后进入 `b0e43b92`，全部800项已逐Git blob/mode绑定，正式证据在 `b315f03b` 可恢复。本地运行使用Node20、Python3.10和Oracle JDK21，不替代Linux CI Temurin17；后继CI和四个测试已变，不能把这份旧完整通过结果重标成当前源码的整套通过。

当前逐YAML正式Finding对象核对为56项resolved、7项open；旧52/4与56/4分别属于此前切点。本轮大Host流式哈希、telemetry夹具启动异常清理、State/Trace精确时刻oracle与Python3.10 factory超时分类均已有具名修复和同源验证；四项具名Finding的修复、同源验证和独立复审三链已安装，并在 `0cbd02ec` 本地resolved；这不等于远端CI、SDK main或完整Behavior闭合。后继CI722实际五项成功、三项失败，原完整日志保留；依赖bootstrap工具链探测、duplicate ACK前successor gap观察和Tool PID观察窗口三个新具体Finding仍open。对应一行CI输入修复及四个测试修复已形成正常开发提交，源/有限验证/复审证据齐备（Gap最终 `ff3de936`、PID `806012a`）；这些本机focused结果不证明新Linux通过，后续必须原官方语言脚本和依赖job实际验收。Git分支切换与SDK worktree栈布局具体修复保持各自原三链。本地闭合不宣SDK main或外部Issue关闭。12项accepted source/kind迁移，以及cleanup/Sandbox/Task resume三个宽义务关联的5秒物理结算B仍未决，不自动退役合同、改变取消政策或接受风险。

[完整合同只读检查](evidence/sdk-contract-00782-readonly/README.md)真实退出1，保留13项连续性错误：12项提案未批准，另1项已有独立批准的Curator保存项仍按原证据归因；没有update/accept/retire或seed修改，未到达的后续命令不算通过。40格审查是五个in_scope区域的有界风险映射（38 covered、2根manifest not_applicable），不是41族全部verified。库存建模已闭合（inventory_closure=closed），行为模型保持open。历史 `eebd9f95` 官方全项目strict及473fb206源码摘要/308份语义文件一致仅属于该切点；本轮修复和文稿之后，最终sourceDigest与官方全项目strict须重新实际验收。旧task-scope失败日志保留，不通过退役义务或改工具版本伪称通过。当前action升级只证明安装字节/AST与已审候选一致，Node24 action已在CI722实际执行，cargo-deny工具链探测错误和两项测试时序失败也已实际取得；修后源码的零其它工具错误/警告及Linux消费者结果仍待新CI；已有工程CI不执行语义工具。工具UTF8修复的第三仓库交付已获用户授权，当前该仓门禁/最终交付仍在推进、尚无可供SDK固定的已发布修复SHA；SDK独立语义job尚未实现和实跑，后续必须固定真实发布版本，不借Admin补丁冒官方工具通过。Windows job只检查Rust库，不证明Windows原生SDK消费者。完整合同、适用CI、精确SDK main与npm/PyPI/Maven发布分别验收；本轮不发布registry。

具名审查范围见[能力处置](capability-review-dispositions.md)，三链及开放对象见[Finding账本](finding-ledger.md)。历史段中的“当前”“未运行”“open”只描述该段原观察，不作为当前待办；原日志、输入、sourcecut与限制不改写。

## 前一阶段文字与证据（历史，原正文保留）

以下“当前/待回归/open”均是原阶段文字；现在以前述状态和Finding对象为准，原日志/sourcecut/限制不改写。

41项能力的行为与证据范围见[逐能力处置表](capability-review-dispositions.md)。下表只列当前真实状态；后附旧阶段记录保留原切点，不能当作当前待办或新的通过证据。

## 当前验收状态（2026-10-14）

本轮客户端小里程碑已形成真实 Git：Java `59fde9d3`、TypeScript `cf6a042e`、Python `1ca513a1`，正式证据随后提交为 `76601b24`。三语言客户端八个具名风险视角均经实际独立源码审查记为有界 `examined`，C/I/M0；TS/Java 两项具体 stdio 关闭缺陷具完整修复、同源验证与具体复审链，已本地 resolved。当前 a634 的14条Rust门禁实际全0，依赖 audit/deny 和精确50组基线独审通过；官方完整语言同入口重跑退出0（TS308、PY375+18、Java332/71XML、3quickstarts），790项输入前后同。原退出1/98d714/TaskHTTP连接夹具证据保留，后继只证明受控Connection close机制，未声称原socket直接因果。完整生成、12项source/kind与5秒B选择、当前最终结构/平台/SDK main仍独立，不把客户端局部闭合扩为41族或八个全局视角verified。

### 前一a634接线阶段记录（历史，以页首客户端里程碑为当前）

SDK 已精确依赖框架 main `a6346643bda47679a96760f56f45e41fd73b61b0`，八个框架包同一 Git 来源、无本地 path patch，锁文件 SHA `a7a331cf`。框架 PR185 与合并后 main 的七项 CI 均实际成功，合并树与已审 `6d9be119` 树 `093a48f0` 完全相同，精确 main strict 已通过；验收收据为工作区 admin/framework-main-a634-acceptance.json（SHA `0d9b7097`）。SDK 新 pin 的14条门禁仍在运行，目前已取得7步成功（含 workspace 461项测试、零失败/跳过），不声明全部14条或当前完整语言通过。依赖 audit 一次实际通过、deny 的50组与已接受基线一致，独立复审尚待。此前 be5/旧 Host 的14Rust、Chart61、TTL和完整语言结果按原证据切点保留，不冒充本次 a634 执行。

此前 be5 切点的完整三语言脚本已用同一官方入口执行完毕并退出0：TypeScript 290项、Python 368项加18子测试、Java 320项和67份XML全部通过，三个quickstart完成。759份执行输入前后哈希一致，实际构建Host与不可变bcb0389候选完全相同。此前两项测试夹具/oracle缺陷已修并独立复审通过；原94950字节失败日志保留为历史，不改成成功。语言94项正式归档与外部超时oracle18项正式归档均已冻结并通过独立复审；该结果属于其执行切点，不覆盖后继正在实现的缺陷修复与新测试。此前b909/旧Host结果不冒此次be5来源。正式记录见[本次完整语言门禁](evidence/sdk-language-be5-reviewed-final/README.md)及[外部超时oracle](evidence/sdk-external-timeout-oracle/README.md)。

仍待人的选择为12项已接受 source/kind 兼容迁移与5秒物理结算 B 提案；此外 Run.steer、Telemetry 及 stdio 关闭修复仍待当前 SDK 消费者与正式材料收口，不能概括为只等选择。完整合同生成、新源码切点的完整门禁/strict、适用 Linux/Windows CI 与精确 SDK main 验收仍未完成；CLI/官网外部交付另待授权，registry 发布独立。41项能力保持 `needs_review`，八个风险视角不因测试计数自动转为 verified/covered。

当前新增进展与未完成项：

- [Run.steer canonical turn 缺传](../../../.echo-semantic/findings/finding.sdk-run-steer-canonical-turn-context.md)：框架身份传递及精确 legacy fallback 已随 PR185/main `a634` 交付；三语言此前诊断红绿保原切点。SDK 新 pin 的最终 Host/完整语言与版本证据尚待，本地账本保持 open，不冒称当前消费者已验收。
- [Telemetry 初始化回滚阻塞](../../../.echo-semantic/findings/finding.sdk-telemetry-host-init-rollback-block.md)：框架 subscriber 组合与日志字段修复已随 `a634` 交付，原三语言挂起/OS栈反例保留；SDK blocking-work/公开关闭候选证据按各原源切点保存，新 pin 完整验收未完成，保持 open。
- [TypeScript stdio 关闭](../../../.echo-semantic/findings/finding.sdk-typescript-stdio-close-ownership.md)：最终dd822源、10真实child、精确实体public-close及a634同源完整语言验证/独立复审三链已齐，具体Bug本地resolved；旧缺fixture的12项包保历史，不宣SDK main。
- [Java stdio 先 destroy](../../../.echo-semantic/findings/finding.sdk-java-stdio-process-first-close.md)：最终86efd源19+72及本次whole原TelemetryXML2个真实case通过，具体修复/同源验证/独审三链齐备，本地resolved；b89/3aa切点保历史，完整goal/main未关闭。
- Python 3.10 兼容局部验证已通过（六个既有测试与 Rules 的合计19项 focused，以及 Rules 的真实3.10/3.12请求结果）；[文件替换权威](evidence/file-authority-identity-replacement/three-language-final/README.md)与[Session规则实际注入](evidence/session-rules-prompt/README.md)仅证明各自具名场景。当前新源码切点的整套 suite 尚未重验，先前290/368/320完整通过仍属于原归档切点。

证据入口：

- [Chart共享61场景](evidence/sdk-chart-published-pin/ci-regression/README.md)
- [be5最终14条Rust门禁](evidence/sdk-chart-published-pin/rust-gates-reviewed-final/README.md)
- [真实默认30分钟TTL](evidence/sdk-real-permission-ttl/README.md)
- [新pin依赖检查重试与原失败关联](evidence/sdk-dependency-be5-current/retry-once/README.md)
- [已接受的有界依赖警告政策](dependency-warning-policy-proposal.md)

## 八个风险视角的当前有限边界

|风险视角|已经具名审查的范围|仍需完成或保留的边界|
|---|---|---|
|trigger_input|协商、能力准入、Task CAS、工具验证、Channel六个已批准操作的输入与错误|12项source/kind兼容选择未答；不得自动删除旧身份或接受全部Rust接口|
|result_side_effect|Chart共享61场景、工具实际文件/artifact、RAG标量与审批拒写副作用|完整三语言已同官方入口通过；外部provider仅证明原时点响应，不保证未来服务可用|
|state_authority|框架Task/Run/Store唯一权威，Host原Registry、取消地址和资源owner|5秒后实际生产者未停止却结算claim的问题仍由B提案处置，不能由合作取消场景替代|
|data_durability|文件冷恢复、Trace原字节、journal/replay/ACK、managed已接受消费者|内存Delivery没有额外跨进程持久性承诺；最终合同/来源一致性与重启场景按冻结切点验收|
|time_lifecycle|Factory失败重试、真实进程/回调结算、stream EOF、默认30分钟TTL到期|TTL只证明易失Session缓存到期/读取不刷新与显式cleanup；post-answer取消只覆盖已列出的原stream窗口，不外推所有后台工作|
|failure_concurrency|ACK提交事实、原lease两事实门禁、资源关闭和前缀/EOF竞态的局部修复|原5秒物理结算仍开放；最新两项测试缺陷已修复，同一完整语言入口已取得退出0终态；最后证据冻结与结构验收尚待完成|
|permission_external|真实审批上下文、TTL、已接受的精确50组依赖提示例外|12项迁移/B策略仍未批准；CLI/官网外部交付另授权，不扩大本地权限或其它警告例外|
|contract_evidence|明确合同边界、be5/Rust/Chart/TTL及新pin依赖实际验证和复审|完整28 profile生成仍待12项选择；最终私有快照、全项目strict、跨平台CI与SDK main尚未完成|

## 历史阶段记录（原切点保留，以下不是当前状态）

下文保留此前b909与更早运行、当时未观察TTL、当时duplicate政策proposed及Factory门禁失败的处理过程。当前TTL已经实测、duplicate规则已接受并实现、新pin工具检查已通过、Factory局部修复与be5 Rust门禁已经通过；这些历史句子不恢复为当前阻塞，也不重写原日志或Evidence。

41family逐项行为、唯一authority、生命周期、失败与具名证据见capability-review-dispositions.md；8group主路径mapped不等于familyverified/8lenscovered。下表按installed0.5八个风险视角列实际余项，不以未定义“所有组合”增加门槛。

|风险视角|已审且具原epoch证据|当前有限剩余|
|---|---|---|
|trigger_input|ACP协商/feature准入；TaskCAS/CRUD-only拒执行；Toolvalidator；Channelforward6ops typed39与nativeowner/generation/消息12negative|12acceptedsource/kind兼容提案等待用户一次选择；批准后唯一generator精确迁移/退役，TaskContext3getter现路径四真实组合已独立PASS，source身份迁移仍须人的选择，不能自动用same-name/kind替换|
|result_side_effect|普通Tools五族原49rows/48calls/45deterministic4providerobs；RAGb909标量32call；Memory59/Trace原bytes/ToolartifactHTTP完整SHA与effectrollback|Tool/Search/Sandbox最新8d819首次9native全绿73档为after候选，EOFspecific独立source/afterpackagePASS0已闭合，原manifest历史Minor已由根与独立reader核对通过，无剩余action；provider时点429按实际expectedfailure保，不扩大为外部服务可用保证|
|state_authority|framework唯一TaskRun/DAG/claim/terminal/managedcommit；Host同Registry/streamcollector/Channel原entry与handler；12sourcegate正确noSeedwrite|执行批准source迁移时保7Securityalias行为、3Task原生getter绑定、2旧Snapshotowner明确处置，不恢复第二mutation/Plan权威；最终contract/sourcefield消费一致验收|
|data_durability|journal physicalprune1unit、Host水位12units/真singleline红绿、clientACKv4、managed八consumerlostACK/retirement、FileRuntimeStateStore与Tracecoldbytes|最终b909 SDK完整generatedcontract/不可变Host上复跑既有重启/gap/三語矩阵；3managed逆DTO已审supporting/RustHostcaller，无未接受forwardAPI强补；accepted内存Delivery不新增跨进程义务|
|time_lifecycle|Channelforward已实现且原0435/0744/0a/f852native与Custom/coop/prefix具体红绿独立pass；Sandboxproducer实际Exit/cleanup/EOF；Factory/原生closeowner原证据|SandboxnaturalEOF queueResult新after独立PASS已具体闭合；最后源码匹配strongconsumer/native/完整门禁。Task客户端早setupACK之后取消地址保留已原3native红绿/TS19PY39J27与独立PASS；这仅覆盖具名原issuedstream/取消窗口，不外推所有工具stream或超过5秒的物理结算；Permission30minTTL实际过期未观察且无accepted短TTLsetter，不造API|
|failure_concurrency|ACK成功提交消费/pending失败保prefix，Hostsnapshot水位和resourcecapturedstart/closeRW已localresolved；4Custom+prefix/missingterminal独立finalscope|新EOF原candidate已source/after独立PASS本地resolved，最后源与门禁仍独立；其余broadFinding按原repair/verify/rereview实际缺引用收窄账，不借hostlib163或旧wholecount；不新增无限任意race矩阵|
|permission_external|frameworkHumanLoop/callercontext/cache/真实拒写approvedonce；本地用户扩展不新增多tenant/fullauto门禁|TTL未观察边界明确保留；第三方provider只证明原时点request/result/error，不以缺省字段填值；CLI/website对外交付另授权，SDK/framework授权不延伸|
|contract_evidence|明确acceptedseed/canonicalsource/kind唯一阻断范围；完整inventory遥测；LSP29+42值合同，TaskExecution2fields/DiagnosticCode3/Curator1retire已有原授权|完整28profiles发现12missing且noartifactwrite，等待提案批准；随后正式wholeupdate/check supportingvalue3、兼容fixtures、deferred新inventory14group重算、独立build/全语言/CI与精确SDKmain|

Channelforward不再是“待实现”的产品缺口：用户已批准、生产6closedoperations/原typedhandler/streamcontrols已落地并有多epoch有限native证据；specificconsumerFinding最后闭合仍须生成scope/完整source/独立review一致，不能用旧partialcontract候选宣布finaldelivered。

当前待人的兼容选择有两项：12项source/kind提案（7Security定义迁移、5旧Rustfield身份处置）及framework5秒物理结算B提案；两项均proposed且尚无用户答复。其它工作是已授权的工程验收：最终生成与不可变源冻结、适用全门禁、准确localFinding三链、独立复审、精确main/CI。CLI/网站候选只准备、外部提交/发布尚未授权，registry发布独立。

30minTTL与post-answer取消等证据边界按上表有限保留，不把所有未列组合当闭合否决，也不将每个Rustpublicidentity加入三语言合同。旧fc63/6711/c52/64c7/7bbd/0744/0435/0a/06bb/ee9ff/8d819各原sourcefreeze/rawhash不重写成currentwholepass。

## 首次正式门禁失败与当前处置

首次官方全语言命令TS287/285/2/0skip失败，PYJava未运行。原Taskpause恢复A2的12CAS失败已确认客户端setupACK后删除invocation取消地址，三客户端修复/真实红绿/独立PASS有限闭合；宽TaskFinding仍open只因framework5秒物理结算边界。共享state_rootOS35已按每Host独立配置修复与原并行强断言复审通过，没有串行弱化fileauthority。随后client取消地址修复及fixture隔离后，同一官方入口最新完整已通过（632inputs无漂移/290+35718+320）。原首次失败保持，不改历史；本次只代表b909/8d819/partial3cd835cut，最终新pin/gen后仍匹配重验。

## 取消地址与物理结算分离

三client早setupACK删invocation取消地址根因已3原生真实red→TS19PY39J27零skip/9source27archive独立PASS有限闭合，TaskresumeFinding处理记录的client子根因已修。宽TaskFinding仍open：frameworkouterDrop/5秒强abort5.10s清claim但physicalproduceralive单独Dsource/反例/设计链，不能riskaccept或由clientcooperative绿关闭。proposed supervisor观察窗口选择未批准，12acceptedsource选择也仍未答；现script在b909/8d819/partial3cd835及632输入冻结cut已完整通过，最终新pin/gen后需要匹配门禁，不重复task8MiB失败检查、不将focused计数冒wholegate。

## 当前官方完整语言验证已完成的切点

最新sdk-language-test-nop-complete是同一个官方entry完整exit0，90900bytes原log、67XML、632sources无漂移：Node290、PY357+18、Java320零failure/error/skip，3quickstarts和Ruff175通过，SLF4Jwarning0、负例RPCWARN保。这不是旧分段计数合并；POMprovider只testscope实际CP独立核。

当前真正四类后续是：(1)12acceptedsource/kind兼容迁移的人的选择；(2)framework5秒物理结算Bproposed政策/生命周期选择及必要实施，SDK不自行riskaccept；(3)批准后的完整28profile生成/check/supportingfield签名与最终frameworkpin、无patch独立Rust/合同/语言/全项目strict匹配冻结cut；(4)精确SDKmain/远端CI及外部Issue交付。CLI/website候选交付另授权，npm/PyPI/Mavenregistrypublish独立。41Family及八lens不会因当前一套语言whole绿自动verified，具体TTL/postanswerscope原因继续可复核。

## 当前Rust全门禁新增阻塞

14步ordinarymatrix前4格式/lint通过，第5workspace/alltargets/allfeatures/locked test真实exit101：Hostlib167/core31通过，bridge34中32pass2Factoryfail、后9steps未跑。该历史attempt当时Factory根因尚为hypothesis；当前Factorytwofix已8e7真实红绿/171lib34bridge独立PASS闭合。四TaskContextgetter真实组合与独立PASS只补原字段消费证据，不授权source12迁移或替Factorygate。旧wholelanguage632/bridge43ec来源保原成功；当前reviewed6feature接线修复已有限三链PASS，新的14步rust-gates-reviewed-final现实际全0/zeroCompilerWarnings终态，最新receipt独立reader已actualPASS0C0I0M。

## 依赖警告规则的独立合并阻塞

当前same7inputs/freshRustSec6aed audit零warning/denyexit0工具证据真实通过，但deny50duplicatewarning与AGENTS零warning要求冲突，specificdependency-warning-policy-conflict open、SDK合并验收BLOCK。准备dependency-warning-policy-proposal.md proposed，等待根完整审阅/人选后规则一致化与复验，不能自行按旧warn默认豁免；这与12source/5s提案分别处理，不由工具exit0闭合。原matrix前4green/第5twoFactoryfail/余9未跑保历史；Factory具体修复与feature接线当前已独立PASS，新的完整14matrix已actual全0，独立最新receipt已actualPASS0C0I0M，不等SDKcontract/main。

## Factory当前局部闭合与新源码门禁切点

bridge8e7/E2EonlyactualJSON7e231/doc36c730：Factory5old3/2→new5/0；actualtwofacts后inflight1真实red→同步原lease释放green，heldRPC/heldRemote/wronggen仍占用；failedcloseErrcached真实red→只成功缓存/parallelretry2ndsuccess/后续noRPC。27archive/171lib/34bridge/strict2fmt0与源+原24pack独立PASS三链令FactoryspecificFinding本地resolved，宽cleanup只Factory子scope关闭。旧14matrix前4/第5fail/余9未跑是历史attempt；根授权新14Rust完整matrix/新immutableHost+全language运行另验，旧81POM632cutbridge43ec不证明当前8e7通过。12source/5sB/duplicatepolicyproposed仍待人，没seed/config/policy改，无genericphysical5s闭合。

## 当前测试feature接线状态

6文件devdeps/cfg/import窄scope各真实feature原失败→同组合最终0warning（default31+4/core85+13+4/bridge28/facade31+12/noBridge1）与独立PASS已具体resolved；原所有stderr副本保。新14步reviewedfinalmatrix已全部actual0，最后whole收据独立复审已actualPASS0C0I0M，build/newimmutableHost之后官方完整language入口另验，旧8d819/632language0属于bridge43ec切点，不冒8e7新组合。dependency50duplicate规则提案正式source审PASS只是可审候选，userChoice未答仍open；12/B/CLI/W亦未新增答复，无自动approved/seedpolicy改。

## 当前reviewed Rust切点实际终态

最终task-context-consumption144档SHA核，reviewed-final14命令+新Hostbuild实际全0/零compilerwarning/sourcebeforeafter一致，workspace457/0/0ignored按配置原计数不叠能力；bb4f3c2b构建manifest为当前输入authority，8个frameworkpackages真实Gitb909/noPatch。Rustexamplesabsent/doc0case准确，新的bb4f完整三语言+examples入口已actual单entry0，最新twoPack独立reader已actualfinalPASS0C0I0M。原6file接线已独立PASS，最后14新receipt独立reader已实际60inputs/144archives/14raw核PASS0C0I0M，LinuxWindows/contract/main另验；old8d819/bridge43ec81language绿原epoch保。dependency2命令已新manifestcutactual0，50groupcrate+actualversions与proposal精确匹配，规则AGENTS冲突未获人的选择，AGENTS冲突仍BLOCK userpolicyproposed未答；12source/B5s/CLI-W同待不重复询问。Rust当前适用工具门禁已actualverified不等全generatedcontract/globalstrict/main或41/8lens自动verified。

## 最新Factorycut语言与依赖执行证据

正式84files cbf90de语言包采用newbb4f/build5e0/b909真实Gitnopatch/partial3cd835，同官方entry90884bytes未截exit0、632inputsAllSame、Node290/PY35718/Java32067XML/3qs/Ruff175/SLF4J0；旧81/83包仅各历史epoch。19files bc9bde依赖新cut7inputsame/fresh6aed两DBclean/1310/658/audit0deny0，50duplicate每crate/version数组匹配proposal，仍AGENTS规则BLOCK。最新twoPacket独立reader已actualfinalPASS0C0I0M，当前Rust/语言与工具结果都已核；依赖规则BLOCK/全合同/SDKmain不因此闭合。14Rust新cut已独立PASS；LinuxWindowsCI/full28acceptedgen/globalstrict/main及12source/B5s/dep50三个compatibility选择未答，CLI/W外部另授权。
