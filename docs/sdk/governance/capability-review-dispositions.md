# 41项SDK能力的有限审查处置

当前验收与阻断状态统一见[当前有限验收](capability-review-remaining.md#当前有限验收2026-10-17)。本页只维护其独有能力结构、逐项处置或历史账本；下文历史切点的计数、证据与义务保留。

## 前一阶段文字与证据（历史，原正文保留）

以下“当前/待回归/open”均是原阶段文字；现在以前述状态和Finding对象为准，原日志/sourcecut/限制不改写。

本表是行为模型审查索引，不是运行时状态权威。主路径mapped表示入口/authority/lifecycle/failure已展开；Behavior的needs_review、openFinding与正式交付独立。接受范围只来自明确合同，完整Rustinventory只遥测。

## 当前验收状态（2026-10-14）

SDK Host 已精确依赖已发布框架 `be5f0a84`，八个框架包来自同一 Git revision，没有本地 path patch。Chart 整数无损修复已交付：三语言共享61次调用的边界语料及当前14条 Rust 门禁均通过独立复审。默认30分钟审批 TTL 已用三语言真实等待完成验证。用户已接受并落实精确50组 duplicate 提示例外；新 pin 的 audit/deny 经一次有依据的网络重试后通过，版本组逐项匹配且独立复审通过，其它警告与原例外没有放宽。

最新完整三语言脚本已用同一官方入口执行完毕并退出0：TypeScript 290项、Python 368项加18子测试、Java 320项和67份XML全部通过，三个quickstart完成。759份执行输入前后哈希一致，实际构建Host与不可变bcb0389候选完全相同。此前两项测试夹具/oracle缺陷已修并独立复审通过；原94950字节失败日志保留为历史，不改成成功。语言94项正式归档与外部超时oracle18项正式归档均已冻结并通过独立复审；当前正在完成最后文稿快照与语义验收。此前b909/旧Host结果不冒此次be5来源。正式记录见[本次完整语言门禁](https://github.com/EchoYue-lp/echo-agent-sdk/blob/0ebe15f66d0aa57c7b7a92ae68cd1be106114ae9/docs/sdk/governance/evidence/sdk-language-be5-reviewed-final/README.md)及[外部超时oracle](https://github.com/EchoYue-lp/echo-agent-sdk/blob/0ebe15f66d0aa57c7b7a92ae68cd1be106114ae9/docs/sdk/governance/evidence/sdk-external-timeout-oracle/README.md)。

仍待人的选择只有12项已接受 source/kind 兼容迁移与5秒物理结算 B 提案。完整合同生成、最终源码冻结后的全项目 strict、适用 Linux/Windows CI 与精确 SDK main 验收仍未完成；CLI/官网外部交付另待授权，registry 发布独立。41项能力保持 `needs_review`，八个风险视角不因测试计数自动转为 verified/covered。

证据入口：

- [Chart共享61场景](https://github.com/EchoYue-lp/echo-agent-sdk/blob/0ebe15f66d0aa57c7b7a92ae68cd1be106114ae9/docs/sdk/governance/evidence/sdk-chart-published-pin/ci-regression/README.md)
- [be5最终14条Rust门禁](https://github.com/EchoYue-lp/echo-agent-sdk/blob/0ebe15f66d0aa57c7b7a92ae68cd1be106114ae9/docs/sdk/governance/evidence/sdk-chart-published-pin/rust-gates-reviewed-final/README.md)
- [真实默认30分钟TTL](https://github.com/EchoYue-lp/echo-agent-sdk/blob/0ebe15f66d0aa57c7b7a92ae68cd1be106114ae9/docs/sdk/governance/evidence/sdk-real-permission-ttl/README.md)
- [新pin依赖检查重试与原失败关联](https://github.com/EchoYue-lp/echo-agent-sdk/blob/0ebe15f66d0aa57c7b7a92ae68cd1be106114ae9/docs/sdk/governance/evidence/sdk-dependency-be5-current/retry-once/README.md)
- [已接受的有界依赖警告政策](dependency-warning-policy-proposal.md)

|family|本次行为/生命周期审查单元|行为证据入口（历史审计另列）|明确剩余原因|
|---|---|---|---|
|agent-lifecycle|AgentDefinition/Session创建与Agent cascade，统一Registry owner/generation与关闭准入|evidence.sdk-parent-driver-candidate-verification|并发publication/close已知owner义务由agent-close-cascade与cleanup-retry-owner承接；最终当前组合复审。|
|session|create/load/标准prompt共用framework Session、原生turn恢复identity与同owner清理|evidence.sdk-parent-driver-candidate-verification|持久化错误和Sessionclosing回调重入的新明确窗口；不外推任意resource组合。|
|run|start/wait/steer/cancel、规范TurnReceipt和Interrupted恢复；一个执行终态权威|evidence.sdk-parent-driver-candidate-verification|自然完成/取消竞争及后继Channel/ACK影响按具名Finding，不因任意未列组合拒绝建模。|
|event-replay|committed journal/live/gap/replay/snapshot ACK的连续覆盖与真实发送事实|evidence.sdk-parent-driver-candidate-verification, evidence.sdk-ack-recovery-verification, evidence.sdk-gap-watermark-host-verification|v4新增真实反例已officialconsumer绿与独立exactslice pass；提交消费局部Finding resolved，Host完整scope+新单变量红绿已独立PASS，本地bugresolved；最终交付分开。|
|task|一个revisioned graph/service执行CAS、exact控制及canonical依赖结果|evidence.sdk-parent-driver-candidate-verification, evidence.sdk-task-subagent-candidate|四个真实Task场景及四项getter组合已有限审查；客户端早ACK取消地址子根因已修。原5秒物理结算仍开放，12项身份迁移待人；最终新cut完整语言/合同验收另列。|
|subagent|dispatch/await/control、同一幂等receipt和单collector结果|evidence.sdk-parent-driver-candidate-verification, evidence.sdk-task-subagent-candidate|短await/重复await/同claim结果及取消原窗口已审；5秒强中止不能证明实际生产者停止的问题仍开放。不得新增第二调度器或默许旧claim提前结算。|
|bridge|descriptor/注册撤销、reverse invocation/stream/cancel/deadline及factory child身份|evidence.sdk-parent-driver-candidate-verification, evidence.sdk-managed-retirement, evidence.sdk-llm-workflow-stream, evidence.sdk-python-typed-component-stream-verification, evidence.sdk-managed-supporting-operation-boundary, evidence.sdk-eval-improve-topology, evidence.sdk-search-sandbox-06bb, evidence.sdk-channel-cooperative-two|Factory终态后释放原lease及同owner失败重试已具体闭合，现be5 14Rust门禁已独审；原三逆managed DTO保持Rust/Host支持边界。5秒物理结算B仍待选择，不能全桥verified。|
|standard|标准ACP initialize/session/prompt/cancel/EOF与可选扩展保持角色分离|evidence.sdk-parent-driver-candidate-verification|标准stdio4有实际证据；最终当前pin完整stdio/Profile测试与平台signal待交付。|
|value|闭合WireValue标量/容器、u64、路径、时间、Unicode、optional/未知wrapper无损|evidence.sdk-parent-driver-candidate-verification|Windows原生路径平台门禁与全部明确接受字段source生成；不把process-local Rust类型接受到JSON。|
|intrinsic|明确接受的客户端本地helper构造/getter/不可变modifier|evidence.sdk-parent-driver-candidate-verification|逐接受helper覆盖矩阵，完整Rustintrinsic membership不冻结、不作分母。|
|memory|live Store权威、namespace/CRUD/search/prune/dedup与Active recall|evidence.sdk-parent-driver-candidate-verification, evidence.sdk-memory-permission-candidate|正式59rows支持Store切换/recall/真实effect；没有任意数值nativeStore默认类型保证。|
|workflow|graph/state/checkpoint object、resume/branch/lineage与framework claim权威|evidence.sdk-workflow-successor-candidate, evidence.sdk-llm-workflow-stream, evidence.sdk-python-typed-component-stream-verification|原48行17/24与后继LLM/Workflow三组分别归档；剩7个accepted Workflow verbs已真实可达，流finally/owner/cancel有0744有界证据。Python typed AgentComponentCall.stream丢字段精确Finding已新5green与独立PASS本地resolved；最终字段生成/epoch与合同一致性另验。|
|delivery|完整exactclaim与唯一activeattempt/effect/settlement|evidence.sdk-parent-driver-candidate-verification, evidence.sdk-state-trace-delivery-candidate|三语言内存ledger对账支持；该acceptedopen是memory，不追加未承诺的跨进程durability门槛。|
|state|FileRuntimeStateStore checkpoint/scopes/owner索引与cold恢复|evidence.sdk-parent-driver-candidate-verification, evidence.sdk-state-trace-delivery-candidate, evidence.sdk-managed-retirement, evidence.sdk-managed-supporting-operation-boundary|三语言9scene中state部分支持scope隔离/冷恢复；clear不宣全局all-or-none。|
|trace|显式collector/fileJSONL、完整指定events/usage与sanitizer|evidence.sdk-parent-driver-candidate-verification, evidence.sdk-state-trace-delivery-candidate|三语言JSONL原字节cold读取；未声明所有secret自动redaction或其它backend完整保留合同。|
|eval|真实runner案例/criterion/grader/factory lazy实例与错误cause|evidence.sdk-parent-driver-candidate-verification, evidence.sdk-eval-improve-topology|0a三语3groups/68rows与1secdeadline取消pendingcleanup/workspace实际证据独立PASS；CustomAgentcontext bug局部resolved，最终matchedpin/allSDKgate另验。|
|improve|已接受trajectory/critic/eval组合与原ShareGPT导出字段|evidence.sdk-parent-driver-candidate-verification, evidence.sdk-eval-improve-topology|0a3nativegroups/45factory及trainholdout/trajectory9fields/保存失败恢复限定PASS；最终字段生成/whole源另验，不创造stopreason。|
|permission|framework规则/cache/context、HumanLoop修改args与真实tool effect|evidence.sdk-parent-driver-candidate-verification, evidence.sdk-memory-permission-candidate|默认30分钟TTL已三语言各26次原生调用、真实等待至少TTL+1500ms验证并独审通过；读取不刷新/不删除，cleanup4→0。仅易失Session缓存；不新增短TTL setter或持久审批承诺。|
|mcp|framework client/manager/tools/resources/prompts与注册transport真实关闭|evidence.sdk-parent-driver-candidate-verification, evidence.sdk-mcp-channel-owned-candidate|后继正式5scene/语言支持exactPID/内容/失败重试及ownedreentry；新ownedChannel不冒MCP全覆，pending close cancel明确审计。|
|a2a|现accepteddiscovery/request/SSE与共享stream transport|evidence.sdk-parent-driver-candidate-verification|closedport/SSE具名证据；远端取消恢复具体合同只按已有API，不猜remoteeffect。|
|lsp|framework唯一native parser，nullable/range/completion/diagnostic replacement和processowner|evidence.sdk-parent-driver-candidate-verification|29request+42diag精确支持；新DiagnosticCode supportingaccepted字段生成待最终rev，不复制SDK parser。|
|channels|plugin register/start/send/stop与框架ownedentry的生命周期|evidence.sdk-parent-driver-candidate-verification, evidence.sdk-channel-message-handler-consumer-audit, evidence.sdk-mcp-channel-owned-candidate, evidence.sdk-channel-resource-admission, evidence.sdk-channel-cooperative-two|0435/0744/0a/f852各epoch分别保consumer/资源/coop2证据；cooperativeclose精确bugresolved，teardownprefixI2已真红绿及finalActorPASS局部resolved；最终生成/ownership全接受场景独立匹配。|
|telemetry|compiled广告、typedbool配置及framework init/status|evidence.sdk-parent-driver-candidate-verification|非法bool明确error；真实exporter/collector只在已承诺配置范围验，不假初始status布尔即export成功。|
|topology|frameworkgraph节点/边/metadata与snapshot/stats、Sessionowner|evidence.sdk-parent-driver-candidate-verification, evidence.sdk-eval-improve-topology|0a实际tracker节点边/export原metadata/clear/wrongowner18rows原生限定PASS，不借currentallgates。|
|web|framework明确Tool调用与输入validator/结果/网络错误|evidence.sdk-search-sandbox-06bb|06bb SearchProvider真实成功+框架原3retry进入模型字段已独立PASS；Webartifact原Agentcontext恢复已ee9ff普通三原生强完整HTTP/artifactSHA/max_length oracle独立PASS；SandboxnaturalEOF具体竞态已same1redgreen/73after/独立PASS本地resolved；旧bb4f历史官方whole290/35718/320通过（不是本次be5的368项Python切点），最终新pin/gen另验。|
|files|frameworkcwd/FileAuthority读写原字节与owner/lease|当前无直接支持；历史审计已归档|指定读写/cwd/lease支持；格式化权限/文件保护由现framework，同用户本地交互不新权限闸。|
|shell|framework child/sandbox/取消、输出与cleanup顺序|evidence.sdk-search-sandbox-06bb|06bb Search/Sandbox各3native/9真实producerExit EOF123/cancel/cleanup原计数限定PASS；canonicalroutebugresolved，默认ShellTool后代cleanup不同scope。|
|git|Sessioncwd隔离、status/diff与明确requestedref副作用|evidence.sdk-tool-behavior-candidate|工具29rows里git原refs断言边界保留，不声称未采集全refs都不变。|
|database|已接受查询/DTO/structured_error与真实文件副作用|evidence.sdk-tool-behavior-candidate|原native真实describe全null，DAnyValueRef4red→6green及whole23源码已PASS；b909/noPatch ee9ff三原生普通49rows strongoracle/runtimecells/default7/null正确值独立PASS，具体bugresolved；TS整classSandbox另race不称全绿。|
|rag|同Sessionindex/search共享store/embedding与hitmetadata|evidence.sdk-rag-project-rules|0435 29calls/13POST原有限PASS，charcount字节13错另Finding；D两字段修复已publishedb909/noPatch ee9ff三语32callscalar独立PASS，本地bugresolved；旧0435bytecount不采纳正确。|
|chart|Vega-Lite schema/value/label/UTF8和framework唯一inputvalidator|evidence.sdk-tool-behavior-candidate, evidence.sdk-tool-input-validation-candidate, evidence.sdk-computation-45-candidate, evidence.sdk-computation-50|be5已发布并作为SDK精确pin，旧宽高wrap/负值默默认缺陷已三链本地闭合；共享61场景三语言与14Rust门禁通过独审。原50与b909 BLOCK保历史；最新完整语言已同官方入口通过，最后证据归档/源码冻结与语义验收待完成，不自动全族verified。|
|media|已接受文件/bytes/UTF8统计preview/export原内容|evidence.sdk-tool-behavior-candidate, evidence.sdk-computation-50|50共同语料与原28tool分别限定格式、UTF8及原导出bytes；未接受任意codec不进入证明义务。|
|data|框架data transform/select/sort/export/pivot typed数据与副作用|evidence.sdk-tool-behavior-candidate, evidence.sdk-computation-45-candidate, evidence.sdk-computation-50|原50语料及后继Pivot边界保持各原来源；最新共享语料扩为61调用、包含11项Chart边界，已三语言有限通过并独审。最终整语言/合同/源码验收另列。|
|statistics|指定descriptive/group/统计数值DTO与非法输入错误|evidence.sdk-computation-45-candidate, evidence.sdk-computation-50|原统计50场景支持具名数值与错误断言；扩展后的共享61场景含Chart边界，不能累加为统计能力数量。当前be5完整语言脚本已通过，最后来源冻结与语义验收另验。|
|research|明确PDF/BibTeX操作与provider错误、artifact原字节|evidence.sdk-tool-behavior-candidate|Clinical官方countTotal=true386原probe与D修复source/gate独立PASS；b909/noPatch ee9ff普通三原生total>=returned正确值独立PASS，具体bugresolved；4官方provider结果保时点观察，不固定386。|
|content-guard|Guard方向/redact/UTF8与catalog exactmethod|evidence.sdk-family-catalog-method-routing-candidate, evidence.sdk-computation-45-candidate, evidence.sdk-computation-50|新50共同语料的UTF8 span/redact/错误字段限定pass；新approved Channel输入不由该证据推断任意消息policy。|
|project-rules|frameworkresolver来源/优先级与实际prompt投影|evidence.sdk-rag-project-rules|0435 InstructionResolver真实29sharedcalls/13embeddingPOST对应有限rules部分独立PASS；不是actualprompt/run注入证明，最终accepted路径source定位另验。|
|testing|14accepted字段payload的三语言本地编码/投影|evidence.sdk-testing-value-candidate|12positive/7invalid直接执行，不承诺RustMockruntime或无serde对象重建，无不存在remoteoperation。|
|structured-output|已接受schema/value validate与明确错误path|evidence.sdk-parent-driver-candidate-verification, evidence.sdk-computation-45-candidate, evidence.sdk-computation-50|已接受validate的合法/非法schema、missing/null和payload/error限额由50共同语料证明；不新增Agent repair配置，最终accepted输出字段生成需匹配。|
|source-operation|exactaccepted definingidentity/signature→现framework服务/资源|evidence.sdk-parent-driver-candidate-verification|不是每Rustpubidentity三语言；逐能力sourceconstruct/method/close的明确义务由菜单/Findings跟踪。|
|invoke|catalog准入、signature/feature/owner/generation与薄adapter|evidence.sdk-parent-driver-candidate-verification, evidence.sdk-computation-50|typedHandle正常回传真实红绿有，后继callsite需独立限定审计，nullinputreachability不证明所有操作结果。|

历史fc63的evidence.sdk-candidate-behavior-verification仅保存原receipt审计、supports为空，故不列为当前能力直接支持。各family仍保留该引用用于追溯；本表的其它Evidence按各自原epoch和具名断言范围读取，不能外推为当前全部操作通过。

历史b909中途门禁记录：当前Rustwholeworkspace第5步exit101两Factoryfailure与后9未跑，见evidence.sdk-rust-workspace-factory-gate-failure；当前widebridge/eval/improve不能借旧632languagegreen覆盖变化源码。Taskcontext四getter消费具名Evidence支持行为，不授权oldfieldretire。 当前Factory局部缺陷已修复并复审，be5最终14Rust门禁也已通过；该失败记录不代表当前Rust状态。

## 三语言客户端生命周期横切收口（不是第42个accepted family）

复用behavior.sdk-language-clients与map.sdk-language-clients，源码/正式材料绑定真实Git76601；8个具名客户端风险视角经独立实际source review均examined、C/I/M0。四客户端source精确SHA、790输入前后同及新官方a634完整语言退出0（TS308/PY375+18/J332/71XML/3quickstarts）相互对账；原failed98d714与TaskHTTP受控Connectionclose反例保留，不能拿后继成功抹原失败。TS/Java两个具体stdio owner问题按各repair/verification/rereview链局部resolved，不宣SDK main。family value/intrinsic或其它39族不因这个横切单元自动verified；剩余12身份/5秒B、完整生成与平台/main验收继续独立。

## Events / resource streams 具名有界收口

两原Map和三原Behavior继续唯一拥有此模型；event-replay为accepted family，resource stream是共享权威而非第42族。69b原运行source在cd5相同、8lens作者实际sourceaudit与原独立slices具名examined；live-journal三语言12/12/9补两次typed错误/peer隔离/同inode两文件SHA恢复。#120和stream既有具体resolved链不重开，原externalIssue/main与5秒B、宽cleanup、12source选择继续独立。最新SessionRunA代码变更没有借此旧Git观测称已验，未接受sourceoperation不扩承诺。

## Task/Subagent、Memory、Workflow有限收口

四现family的入口/唯一authority/生命周期/具名失败由02059源码观察和原790三native/四getter消费对应8lens作者审查支持，材料独审单列，不新contextfamily。TaskContext只nativecallbackrunid+四Hostgetter具体case；ContextInheritance纯值不称三nativehistorymemoryE2E、compressor消费不泛推所有hook。这一具体补证边界、内存无crossprocess承诺、5秒B、12字段身份和新A最终consumer/main保持分离。Task/Subagent场景mapped不提升全部Behaviorverified，既有历史例不删。

## 内置十工具与反向桥有限收口

沿原currentTool实际8lens审查，files/shell/web/database/rag/chart/media/data/statistics/research十族及Tool/LLM/Store/Hook/AgentCallback/Intervention六角色按原authority/字段/effect/lifecycle/typederror具名mapped/examined，行为仍needs_review。观察5bcb runtime69b source同；remoteTool/LLM原因3合法真红绿及独review补原whole前scope，Feature宽Finding仅两remote子根因局部闭，其它cause不自动关闭。provider时点、内存无crossprocess、未接受codec/构造器、5秒B/12/新A最终全gate/main明确独立，原4feb取证与未提交FileAuthority附包来源不伪当前Git。

## Integration与supporting角色有限收口

原独立reviewer的MCP/LSP/channels/EvalImproveTopology/Permission/StateTraceDelivery/Testing范围绑定bd8恢复的69b运行源，790原case按实际assertions支持；A2A原consumer缺口后由三语各24steps/9ops/8HTTP与明确typederrors/stream/owner/noeffect另epoch补齐。registry/skill只归现source_operation/intrinsic支持角色，不自动所有methodverified。具体Mapscene仅mapped/8lensAuditexamined，Behaviorneeds_review保。Statecheckpoint7/Trace17字段区别、Deliverymemory无跨process、Testingpurevalue非Mock、网络时点/5sB/12/currentA/main边界全部保。

## ACP与Agent/Session/Run有界收口

原frameworkFinal实际8lens和approvedA修复/native证据按真实8763/da切点回收，标准/plainhello、typedmethods/runtimecreate/load/close/get/wait/steer/replay、oneauthority/close-debt/recovery界限有限mapped/examined。Sessionlist官方inventory不advertise/save自动投影，不新义务；localRunclose调用canonicalcancel/wait再关订阅，不直接退役Hostrecord。A同连接generation publication幂等具体gap已本地resolved，新Java协商/shape缺陷保持open；Bphysical/12sourcekind/fullgenerator/platform/main独立，Behavior仍needs_review，不把运行门禁count反推全capcovered。

## Protocol后继有限收口

现有ACP/standard/value与客户端kernel角色经SDKcap实际8lens source审/新广告shape+initowner原强验证及有限artifactreview具名examined，观察真实622d。原Javaoverflow/member过滤/limits与capsshape缺陷、失败initialize原stdio漏owner具体三链已closed，NegFindinglocalresolved。源码/真实6before与7afterOnly/TS37PY31J21PY25 cuts原样保，旧782whole不新验收；Windowsnative/fullproducer12/Bphysical/新currentwhole及SDKmain仍独立，不扩Rust全部identity/新字段权限。
