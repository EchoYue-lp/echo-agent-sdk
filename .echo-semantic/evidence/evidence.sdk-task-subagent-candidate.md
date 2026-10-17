---
schema_version: 1
id: evidence.sdk-task-subagent-candidate
kind: evidence
observed_at: fa62f3b3d0df3ed5b14c6d9786dfffce5db57da6
source_refs:
- sdks/shared/testing/task_subagent_fixture.py
- sdks/typescript/test/task-subagent-host.test.js
- sdks/python/tests/test_task_subagent_host.py
- sdks/java/src/test/java/com/echoagent/sdk/TaskSubagentHostTest.java
- docs/sdk/governance/task-subagent-evidence.md
supports:
- behavior.sdk-family-task
- behavior.sdk-family-subagent
- finding.sdk-task-resume-extension-invocation-conflict
limitations:
- Opaqueids/nativedurationsrawkept; normalization compares canonicalrelationships not UUIDequality.
- Storefixtureonlyatomicexpectedrevision/executionCAS; frameworksoleDAG/claim/statusauthority.
- TaskSubagentContext fullDTO and restart/failurematrix unproved; callbackonlyAgentTaskInput string.
- Finalpublishedpin/mainandmatchingwholegatesremainpending.
- Historical cb9 four-case proofs and current 14a five-case Task suites are separate epochs; neither proves the complete SDK or main acceptance.
- Current formal-only Git candidate excludes foreign setup/stream tests awaiting an upstream ACP fix; it does not qualify the dirty source tree's entire Task file.
asset_refs:
- asset.sdk-historical-proof-location
---

# Task/Subagent 共享真实 Host 候选场景

## 支持的结论

本段及下方旧6711/cb9/9db记录保持各自历史epoch。当前header只绑定后继formal-only候选；新增实际五例结果与精确来源单列于后继段，不把旧四例重标为当前执行。

三语言通过公开原生 SDK API 执行四个相同场景：精确 pause/cancel A 保持 B Running、C Pending，随后 B/C 能独立完成；resume A 与 D 复用 canonical B 输出，不重派 B/C 或重写 B 的执行事实；stale relation CAS 与 duplicate execute 在副作用前明确拒绝；Subagent 幂等 receipt 只创建一个 attempt，5ms await 不消费 join，后续完整结果与再次 await 完全相同，Unicode constraint 到达真实 compiled prompt；CRUD-only 后端在 execution CAS 或用户 Agent 调用前拒绝 execute。

## 来源与范围

不可变 Host SHA 6711dd17fe3be13671ee181fce5ee79946f40963e000af293083f114301a39a5。TypeScript/Python 各四项测试、零失败/跳过；Java 在临时输出目录编译公开 API consumer，main 执行四个场景，未伪称四项 JUnit 测试。原始 callback/commit 与独立状态目录保留在 proof。

注册 Store fixture 只原子比较 expected revision/execution vector 并保存 framework 提交的 next graph；它不选择 frontier、创建 claim、决定 status 或实现第二调度器。比较保留相同 claim/revision 的关系、调用次数、关系 revision2、状态与输出。原生成 ID 与 duration 保留在 raw trace，不要求不同运行的 UUID 相同。

临时 consumer 输入以原字节 txt 归档，记录真实执行但不宣称已纳入 SDK CI。首次 TypeScript await fixture 只接受 agent_execute，实际 framework 使用 agent_execute_stream；修正为原 Rust fixture 已支持的两种回调后通过，不将 fixture 错误当成生产缺陷。

## 已知缺口

完整 TaskSubagentContext 字段 round-trip、外部后端故障/crash/restart，以及未列并发/publication 场景仍待。CustomAgent 回调只有 task String，不能伪造直接 TaskSubagentContext 消费证据。正式门禁已承接；最终已发布 pin、匹配的完整语言门禁与精确 main 验收仍待。

## 正式门禁承接后的实际验证

formal proof 保存的四份冻结输入在 cb9 Host 上完成三语言各四个正式场景，零失败/跳过，现有 Node/Pytest/Maven 入口可发现。其后 TypeScript 增加 callback jobs 结算等待，Java 加强 canonical B result/执行次数断言；完整差异保存在 review-consumer-delta，当前源码不能宣称与原 formal proof 全部 SHA 相同。

后继 TypeScript 源码29a候选在c52 Host完成四项focused场景，原始日志 `/tmp/sdk-task-subagent-review-callback-green.log`；其余后继运行按各自实际源码、Host 和归档收据支持，未归档的运行不用于扩大本 Evidence。原临时6711输入、formal cb9输入和后继断言增强保持独立 epoch；完整 context/crash 矩阵及最终 pin/main 门禁仍待。

## 本次 current_source_business_rereview

原已审SDKruntime代码来源Git569/source17cf和治理文稿可恢复Git`9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d`（source0a13）明确分开。独立reader`framework_final_review`本次实际重读原业务fault/完整字段/副作用/owner/调用方变化与14Rust/newbb4f632原after证据，消息回收UTC`2026-10-14T01:56:14.724478+00:00`，不是原执行时间。结果：**PASS0 有限原业务义务；5秒物理结算Finding仍OPEN**。

exactCAS/兄弟pausecancelTask4、canonicalBresult复用/原运行contextstring与4getter真实组合原claimpath已读；新clientsetupACK取消addressrepair与Factory8e7twofacts原公开场景通过。没有完整context专用DTO/外部故障全矩阵，generic5sphysical不由cooperativecases关闭。

原观测marker`source:3ea544c47e331b5d29a6624f978f3595619bef39fa5985230351098d38930404`、旧run-time/host/sourcehash/raw和正文保，所有source_refs从真实9dbGitblob读取且锚点存在；现在SDKbe5pin与新测试写入会改变当前文件，不能把历史b909当前源码审升级成未来be5gate。真实9db框架pin为b909，工作树be5after另Evidence处理，不将两个仓tree混写。新增门禁sourceRef只具名Scope，没有以整count判族verified；supports/limits原保、不声明旧3ea整仓可恢复、不改unknown成无缺陷。

原义务和全文历史主张保留以审查，若旧unknown因素（TTL/三getter消费）已有新原证据，只在本段给来源区别，不抹原epoch。ChartI1/5sB历史BLOCK明确不因observed版本合法消失，12source/CI/main/globalacceptedproducer仍pending，41family/lens状态不提升。


## 历史证明存储的有界表示迁移

原id/业务正文/status/supports/limitations/命令及原观察含义不改变；本冻结父 `7f14a1b9c751b0612ceb42224f83f29d09aea46c` 中此对象完整原header/source refs可由公开证据tag恢复。只将具体archive物理依赖交当前位置资产与真实same-role consumer，全部旧义务由显式Eq/CT承接，不把存储重观察冒原业务新审或旧失败变成功。

## 后继14a：formal-only Task消费链

公开988与不可变Host `14a0cb58c44bdb84990262f1324e1a2513513a8b40fc0640a9d36f91f95e27e9` 上，新matching-observation场景通过原Task CRUD/execute、registered Store与CustomAgent实际callback，准入同一Popen。held时A保Running/原claim，B可立即CAS完成；A原物理join=0后，同writer matching Complete及实际EOF使A唯一Running→Completed execution CAS，expected revision/完整claim精等，原结果无损。C/D消费canonical依赖输出；B execution在全部后继commit逐字不变，四次Agent执行各1，没有第二execute获取结果或第二driver。

时序依据为原Accepted ADR0060第139–145行及正式Task API第129–140行：joined attempt立即CAS，但下一wave等全部settlement receipt。A仍unknown持claim不等同已物理结算的Paused。因此首f993日志1509B/SHA256 `f99352e5b4e109ca40bdeccf9f4503ad8df24b4df06e4c8001a718ca630398d7` 的BC-before-A-release超约oracle失败与后继1374B/`337c86e051c4369e0763eca4b97211b955682423c97030eb02f265afe50a5ada` 的可选result省略错误断言均原件保留，不当产品red。原经典pause/resume的BC-before-resume行为保持原强断言。

原S三语言matching单例各1实际通过并由reader核原件；随后独立Git候选以公开404d447为父，仅收四Task路径中的formal及原四经典场景，去具名foreign尾部和专用模型回复。共享fixture只做原Store CAS、原Popen admission/join与只读posthoc核对；意外default-model调用仅记录并拒绝，无模型driver。未带Linux诊断，S未验foreign源码保留。候选重新build TS及examples、Python实际加载candidate/src、Java clean重编译，并执行该候选整个Task文件/class：三语各5例，0fail/error/skip，未用名字过滤跳过knownfail。

完整日志TS2206B/SHA256 `a5d443ef06367367c1095b523ef211fa620bb105062f3e9e5d314a42d6d53851`、Python98B/`c0964ffc85325b91914057c5a569780d0149da2f1265407164b39ca6cc0f9f12`、Java1606B/`f3194cc8b4d9dffc67ea7160defca9e934c5b64193b3b8ec7e0ec0752f0299d9`，均exit0/nullsignal/notrunc；JavafreshXML5tests/0fail/error/skip，SHA256 `ebc4b3c763c30f89c6cf8feb1370cd3f04c23c74c6d8017648823476c5042e99`。候选611项语言输入复核零漂移，生产Rust/协议/client实现与404父及14a构建冻结保持；不重建Rust或重跑生成器。raw与原path/副本hash仅保ignored任务local。

本证据仅覆盖具名five-case suite和matching原attempt，不证明超grace、unknown owner丢失恢复、wrong/foreign结算全部组合、完整cleanup或SDK/main。宽Finding保持open；普通turn foreign-source ACP断连是另一条依赖上游F的未验入口，不随本候选标绿。

### 最终formal-only身份输入及新完整套件

最终候选不带未来foreign专用uuid import或缺省ID分支；admit_physical显式要求原非空run_id/execution_id，按原claim精确传递，不分配或替代身份。S的完整foreign版本未改，后继新pin切片再独立纳入。最终fixture SHA256 af6cad66b3f97502d2f9e19f2029690011ac67018d471b392a1e79cae3ab9e03，formatter/Ruff零差异。

此输入变化后重新执行候选整个Task suite，三语各5实际passed、0fail/error/skip，TS重新build源码及examples、Python加载候选src、Java clean重编译；各次611输入fresh before/after零漂移，Host仍同14a。新完整日志TS2200B/SHA256 7b5daa33b2077c300f17d2bc8fbfa48f10cd48cfacc78d138ddc57bd7d6b1e18、Python98B/63508c5b2a39a6f7ed5ced027d886e936576ff17806525a5863c15f00efb0ed8、Java1609B/8a7ca89f01fdcabb6ab6cf5e0cb12775d99a7350658491270abd561cdeac7638；均exit0/nullsignal/notrunc。JavafreshXML5/0/0/0，SHA256 56c9655ac429a11473182cceaa1da64f6cf5f745db6bb67dd59dee0794254008。旧d326等结果保其原输入，不借旧15pass冒新strict fixture验收。上游F656精main/七项CI已由根协调接受；本有限cut仍固定988/14a，不中途切pin。


## 来源身份的无损持久化

原 header 的 source:31483b056427dc3f763899b798d84992cc0301f6bfe1816ccc8acf2952bee11d 与公开 Git fa62f3b3d0df3ed5b14c6d9786dfffce5db57da6 的 canonical source digest 精确相同；只把该 header 改为可冷取回 Git 身份。该提交的原 source_refs 逐 blob 取回核验，不改变正文、原 F988/Host14a 五例结果或审查范围；它不证明后继 F6563/ac433，也不纳入工作树四个 foreign Task 文件。必要 exact-SHA annotated evidence tag 的远端与冷仓回执仅保 ignored local。
