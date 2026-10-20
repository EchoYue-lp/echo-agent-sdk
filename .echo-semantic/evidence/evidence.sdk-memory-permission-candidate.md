---
schema_version: 1
id: evidence.sdk-memory-permission-candidate
kind: evidence
observed_at: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
source_refs:
- sdks/shared/testing/memory_permission_fixture.py
- sdks/python/tests/test_memory_permission_host.py
- sdks/typescript/test/memory-permission-host.test.js
- sdks/java/src/test/java/com/echoagent/sdk/MemoryPermissionHostTest.java
supports:
- behavior.sdk-family-memory
- behavior.sdk-family-permission
limitations:
- actual TTL expiration not observed; accepted contract exposes no shortTTL setter
- approval_receipt is new framework field not in current acceptedfield inventory; no automatic API expansion
- generic numeric Store callback roundtrip must preserve WireValue type; current fixture does not claim arbitraryu64store-nativeprojection coverage
- 59 evidence rows per language are not59testcases; formal TS/Python each1test retain same scene assertions.
- New SDK Tool validation and later source changes do not inherit old6711 whole-run pass.
asset_refs:
- asset.sdk-historical-proof-location
---

# Memory、Permission 真实消费者候选

## 支持的结论

三语言临时公开 API consumer 在6711 Host上各记录59条证据，包括52个family operation结果、两个完整typed failure、11种reverse Store callback路由、真实model recall/journal、permission上下文与修改args，以及实际拒绝/批准的Tool文件副作用。正式TypeScript/Python各一次测试已执行同场景；测试计数不替代每项断言。

Memory沿same Session由Store A切到B，保实际semantic registration单一约束与旧A数据；pre-write callback失败不写入，随后真实成功重试。native Store的expiry/prune、dedup和owner错误有直接结果。新native Session recall只把带真实fixture证据的Active记录送入model，Draft/Legacy不进入实际user输入；journal memory_recalled=1、唯一event ID/连续sequence/唯一final_answer。

Permission跨两个Session验证exact args/session_tool cache、once不缓存、narrow revoke、关闭A保B；HumanLoop callback保留完整context与modified args。真实Run拒绝write时无callback/文件，approved_once时一次callback且文件UTF-8内容精确，终态由framework receipt决定。

## 来源与范围

原临时输入、raw输出、callback/model/journal/sideeffect与错误保存于归档，comparison声明各自normalization：独立路径、已逐个验证canonicalu64的时钟、equal-score无序、dedup原key/content的指定等价。confidence使用显式既有f64 wire，不改变native类型。原Host SHA6711dd17fe3be13671ee181fce5ee79946f40963e000af293083f114301a39a5；这些是原来源的具名消费者断言，不冒充新的Tool validator或全scope最新通过。

## 已知缺口

实际30分钟 permission TTL 过期未观察；已接受合同没有短 TTL setter，不能编造新 API 或默认闭合。framework 新增 approval_receipt 尚不属于当前 accepted 字段，不隐式扩范围。正式 Java consumer 已在 cb9 Host 执行一项 JUnit 测试、零 failure/error/skip，覆盖同一59行场景；来源与 XML 见下节。最终 source/pin、匹配的完整语言门禁与精确 main 仍待，所有 Behavior 保持 needs_review。

## 正式Java消费者执行

正式MemoryPermissionHostTest XML记录1test、零failure/error/skip，匹配59row场景而不是59testcase。原临时6711消费者与正式cb9测试的源/epoch保持分开；不将新增实际证据推广TTL过期或未接受approval_receipt。XML SHA 07df10f13df041c6d39ccae32718e4ddae8cf669856773a2d1be953c97d54bd5。

正式cb9语料比较：三语言原生门禁test复制后各1test、59row仍精确对齐，formal comparison/raw inputs/logs单独保留；其classification文字仍写temporary为原receipt字段，不将其伪改为全mainproof。旧6711临时语料/clock/ID原记录保留，新的正式运行只按自己的实际来源支持。

## 本次 current_source_business_rereview

原已审SDKruntime代码来源Git569/source17cf和治理文稿可恢复Git`9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d`（source0a13）明确分开。独立reader`framework_final_review`本次实际重读原业务fault/完整字段/副作用/owner/调用方变化与14Rust/newbb4f632原after证据，消息回收UTC`2026-10-14T01:56:14.724478+00:00`，不是原执行时间。结果：**PASS0 有限原业务义务**。

liveStore59replacement/Activeprovenance/真实deny0effect/HumanLoopContextapprovedonce/两个Session，原30minTTL原569sourcebb4f三个native26clock/cleanup4→0来源已复核；不承诺crossrestart审批持久化，approval_receipt未accepted。

原观测marker`source:3ea544c47e331b5d29a6624f978f3595619bef39fa5985230351098d38930404`、旧run-time/host/sourcehash/raw和正文保，所有source_refs从真实9dbGitblob读取且锚点存在；现在SDKbe5pin与新测试写入会改变当前文件，不能把历史b909当前源码审升级成未来be5gate。真实9db框架pin为b909，工作树be5after另Evidence处理，不将两个仓tree混写。新增门禁sourceRef只具名Scope，没有以整count判族verified；supports/limits原保、不声明旧3ea整仓可恢复、不改unknown成无缺陷。

原义务和全文历史主张保留以审查，若旧unknown因素（TTL/三getter消费）已有新原证据，只在本段给来源区别，不抹原epoch。ChartI1/5sB历史BLOCK明确不因observed版本合法消失，12source/CI/main/globalacceptedproducer仍pending，41family/lens状态不提升。


## 历史证明存储的有界表示迁移

原id/业务正文/status/supports/limitations/命令及原观察含义不改变；本冻结父 `7f14a1b9c751b0612ceb42224f83f29d09aea46c` 中此对象完整原header/source refs可由公开证据tag恢复。只将具体archive物理依赖交当前位置资产与真实same-role consumer，全部旧义务由显式Eq/CT承接，不把存储重观察冒原业务新审或旧失败变成功。
