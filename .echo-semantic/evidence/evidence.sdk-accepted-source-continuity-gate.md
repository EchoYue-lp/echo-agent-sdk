---
schema_version: 1
id: evidence.sdk-accepted-source-continuity-gate
kind: evidence
observed_at: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
source_refs:
- docs/sdk/governance/accepted-source-continuity-proposal.md
- echo-sdk-protocol/src/bin/export_schema.rs
- docs/adr/0034-sdk-governance-authority-and-lossless-adaptation.md
- docs/sdk/sdk-core-profile.md
- docs/sdk/governance/capability-review-remaining.md
- docs/sdk/governance/behavior-evidence-matrix.md
- echo-sdk-protocol/src/facade.rs
- echo-sdk-protocol/tests/facade_inventory.rs
- echo-sdk-host/tests/core_profile_e2e.rs
- echo-sdk-host/tests/extension_bridge_e2e.rs
- echo-sdk-host/src/core_profile/extension_bridge.rs
- echo-sdk-host/src/core_profile/facade/task_runtime.rs
- echo-sdk-host/src/core_profile/facade/source_operations.rs
- contracts/sdk/accepted-external-contract.json
- contracts/sdk/accepted-facade-operation-catalog.json
- contracts/sdk/source-contract.json
- contracts/sdk/inventory-telemetry.json
- contracts/sdk/parity-manifest.json
- contracts/sdk/public-api.txt
- contracts/sdk/facade-operation-catalog.json
- sdks/shared/facade-operation-catalog.json
- sdks/shared/contract-digests.json
- sdks/typescript/test/secret-scan-host.test.js
- sdks/typescript/test/intrinsic-tool-values.test.js
- sdks/python/tests/test_intrinsic_tool_values.py
- sdks/java/src/test/java/com/echoagent/sdk/IntrinsicToolValueTest.java
- sdks/python/tests/test_secret_scan_host.py
- sdks/java/src/test/java/com/echoagent/sdk/SecretScanHostTest.java
- scripts/check-sdk-contracts.sh
- scripts/check-language-sdks.sh
- AGENTS.md
supports:
- finding.sdk-accepted-source-continuity
- behavior.sdk-repository-ownership
limitations:
- Bounded compatibility slice on published framework 00782 only; no SDK main or release claim
- Original 22e change gate does not accept installed 0.5 schemaVersion 2 preflight; record its actual result separately
- Installed 0.5 task scope retains historical reference and bounded-file limitations; no tool or preflight bypass
- Raw command outputs are local only; no stage archives or source copies are committed
asset_refs:
- asset.sdk-historical-proof-location
---

# accepted-source-continuity-gate

## 支持的结论

原全28profiles正确invariantpass后发现12旧acceptedexactsource/kind，producerexit1/artifactsWrittenfalse。diagnostic改完整报告missing与8regressiongreen，不改变scopepolicy。提案7迁移+5旧field退役与保主能力只是proposed，当前无人的批准，不声明迁移或最终gen成功。

## 来源与范围

原归档receipt/输入hash/二进制保持不变。当前观察只绑定模型与可读取归档，不把原epoch冒称最终current全套通过。

## 已知缺口

Correctfail is evidence of gate, not a currentcontract pass；No seed/artifactwrite; no automaticretire or obsoletefieldrestoration；One finitehumancompatibilityselection pending completecandidate; noDecisionKind fabricated。

## 新规则文稿切点的有限观察

本次观测Git`9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d`对应实际新治理文稿/正式TTL归档切点，源码摘要`0a13f994adc90d29746d23c7e071dfb55d8c12eb92c5000b1785092bdc10392f`。原marker`source:pending`及原正文/inputhash/runtimeepoch保；全部source_refs在本Gitblob=currentbyte/anchors核实，旧Git569仍可恢复，不把新的governance文稿Git当过去运行日期或重新执行。

本对象只观察原receipt/人选择/当前政策或接受门禁文稿。依赖用户只明确接受原4c940的50groupduplicate边界，不批准12source/B5或外部CLI-W；28profile12missing correctfail/fullgen未通过，oldcounts/deferred14仍旧27c库存，不autoacceptRustAPI。

原supports/limitations保留，观察不提升41family/8lens、Finding闭合或accepted成员。运行型证据仍原569/17cf/bb4f/b909，不借newcut冒freshSDK代码执行；仅当时原强cases及已完成新scope可用。


## 2026-10-15 已批准切片的实际执行进展

独立任务起点为SDK a48854a792d146c823e89fcfa2237f2559290c15，Host/framework pin保持
00782b8aeddefe2ea8c566922cbdee6aedc80d70，八包无path patch。用户明确接受原精确12项，
正式授权锚点复用ADR0034；第2项B和第3项工具A/B另已批准、由独立上游实施待交付。
原proposed和correct-fail段落是原发现切点，不再表示本次仍待同一人裁决。

首次唯一generator使用完整28profiles及精确13 retire/12 accept参数，实际退出0，
117artifacts写入；其中新增接受仅七Security替代及此前TaskExecution/DiagnosticCode。
第二次修正getter Rust/Host-only分类后的普通完整update亦实际0/117artifacts。
集合验证实际0：5622→5621，无额外成员变化；七Security signatures全部相同，旧alias和
三个operation保留；getter全部alias仍method、host_or_rust_only、不进入accepted。
完整生成另同步19个原成员signature、derived traits及少量facade alias路径，未扩接受范围。

原完整check的28profiles及117文件比较全部匹配，随后protocol测试87通过/3失败，
完整脚本退出101。根因是旧测试把全Rust遥测当语言义务，及Task result已接受后仍重复
accept；修复保原28helper集合和所有supported traits/streams断言，explicit pre-acceptance
fixture不修改生产seed。其后全facade90项实际通过；新增2个specific deferred source的
负例确认不进入accepted/catalog、不改变blocking digest。Host raw负例还需新构建执行，
不得借静态审阅称运行通过。

早期两条Host/check命令因磁盘预算中断，保留SIGTERM；未形成新Host验证。已确认无Host
后只清理本target/debug可重建缓存，保留历史fixture/源码/日志；后续统一debug0、
incremental0、jobs2。正式完整check与工程/语言验证仍需读取实际最终结果后收口。

原始输出仅本机可用：lp-agent/.worktrees/admin/sdk-accepted-continuity-01a13eb8保存
前两次完整update、真实路由/分类红绿与初始语义失败；当前SDK的
.echo-semantic/local/01a13eb8-0557-7af1-a74b-6b28ea0d00a0保存后继check、membership、
精确signature差分、资源中断及后续工程结果。不复制日志/源码/XML或巨型输出入Git。
原件丢失时本段不能作为新的执行证明；正式source_refs仍绑定源码及可执行回归。


## 完整合同原入口的后继成功结果

`full-contract-check-low-debug.log`记录同一scripts/check-sdk-contracts.sh在2026-10-15
09:25:11–09:42:14 UTC的完整实际退出0；28profiles全部重建、117artifacts匹配，
0telemetry drift、36521inventory items。随后ACP5、extension39、facade91全部通过、
0failed/0ignored；依赖provenance仍00782八包同源。独立reviewer已读原件核范围未缩减。
这是真实后继成功，不覆盖原101或资源SIGTERM，Rust/newHost/语言与语义检查另验。


## Factory门禁原失败与测试观测修正

首次workspace all-targets/all-features测试真实101：Host lib187/core E2E33全部通过，
包括4项Task getter原生消费和deferred原始调用明确错误；bridge33pass/1fail为
cancelled_factory_stream_closes_and_releases_instance最后注册probe仍见quota2。
原case未改focused实际0，说明不能从该失败倒推永久泄漏，也不能拿focused0抹workspace失败。

根因事实：SDK factory_closed写在responder.respond之前；它不证明Host消费正确close
响应。生产close_factory_instance只承诺原close future成功后释放，不承诺该SDK入口marker
后的立即容量。故不为测试方便增加新的生产ordering/API。Root明确确认最小fixture路线。

E2E保quota2、原单10s总预算、原未确认close时拒绝断言，并增强精确
PayloadTooLarge/AfterDelay/register/limit2错误元组。放行后只对这个注册发布前、无budget/
序号/index副作用的具体quota条件yield重观测；其它错误立即失败，真实泄漏超时仍失败，
首次成功立即结束并unregister同probe。官方duplex原owner单测同时强持第二次真实close
响应，直接核in_flight1、child未closed、quota拒绝；释放后await两个同owner.close，
核in_flight0/childclosed/probe成功，保failed-first、同child/origin、OnceCell与calls2。
两项focused实际各1pass/0fail，独立定向复审PASS0；完整Rust后继重跑另按实际结果结算。


## 最终工程与真实消费者结果

后继rust-final/rust-gates.json记录同一最终源码上的8条门禁全部实际0：fmt、fmt-check、
workspace all-targets/all-features Clippy -Dwarnings、生产panic/unwrap四项严格Clippy、
workspace all-targets/all-features tests、featureless lib check、all-features doctest、
sdk-facade-all Host build。workspace478pass/0fail/0ignored；187Host lib、33core E2E、
34bridge E2E、12facade等为该总数的组成，不能再相加。225构建输入前后及复审现场相同。
Host实际707116784 bytes，SHA256
0e97169a4265761ab436932e4b413b5850040525172351999ce6676ec939eafe。

首次官方语言入口真实1：TS342pass/1fail/0skip，PY/Java未达，793输入及Host前后相同。
唯一失败是同样把deferred Rust with_effect纳入旧28helper语言映射计数；三语言对应
测试按external筛选保原exact28/逐项Done/所有行为断言，加入specific deferred负例。
独审确认旧28 exact集合完全保留，无新增语言API。TSfocused3/0。

后继language-final/full-language.log为同scripts/check-language-sdks.sh完整实际0，
2026-10-15 10:09:21–10:14:56 UTC：TS343/0fail/0skip，Python427及18subtests，
Java363/0fail/0error/0skip，76份本轮fresh XML且无旧XML混计，三个quickstart完成，
Ruff195文件检查/格式化0。793执行输入before/after相同，Host前后SHA与上述实际build相同。
Node22.22.0、OracleJDK21.0.5；这是本机macOS证据，不冒Linux Temurin17/Windows或main CI。

三个语言的原Security真实Host场景覆盖各3operation/9value/6negative/1raw scan，
保完整matched/secret_type/position形状、中文emoji/byte offset11、redacted span、原错误，
零模型调用及无run-index副作用。Task4原生测试实际经过最终Host lib，确认run_id选图与claim、
waiver跳过及结果进入prompt、缺结果前拒绝、非默认delegation原样传递；不把getter存在当行为证据。

全部本轮原始结果、fresh XML索引和hash保在本任务local目录，源测试和合同留Git。
无新必要性不明归档；原历史包未清理。无framework pin/依赖/CI触发/语言公共API变化，
现有examples随官方语言入口实际编译/运行，官网及其它仓库无需同步。工具c0bc上游已交付，
本goal仍按Root明确指令保持原22e action/验证器，SDK工具整合与归档continuity由父任务承接。


## 原公开22e的精CI候选机器门禁

任务专属detached候选c22e852f8d9febad0cd1013648cd0da7863b72c6/tree
bb5153f346791d8789b344969e2c8c73a132d87f共享原Git objects，未复制Host。
5553文件全内容及Git mode/OID/path与S暂存候选逐项一致，候选自然没有repo-local PF，
原S/E PF不删不改格式。源摘要f03bbdb92d206a6330de140c6a09c736e1357f0cb321f81f02e09da30f34d925。
公开工具22e241f723673330e9d6bec638def0f78968c74c、verify脚本SHA
393f55f92126b2cc817271bcdd46017d78108be54a82a874ba143545be193da0：ordinary
strict实际0，strict加原base30d7e219cc7d3aef0f75a19217c82898c363f50d及
require-change-evidence实际0（canonical-candidate-ordinary.log及full-base.log）。
两者分别执行，不用ordinary冒完整change gate。Finding元数据收口后再次冻结并执行。

安装版0.5最后task-scope退出1/583项，随后显式repository-scope退出1/582项，原件
installed-final-*.log：大合同/manifest/public-api超过有界读取及历史引用解析等既知工具
合同限制。它们不是已通过，不删PF/改工具/删历史引用求绿；原22e在同源精CI型工作树
门禁与此为不同真实模式，工具c0bc/action及归档continuity由父任务另交付。此处不宣
新工具strict/continuity330、全SDK main或远端CI通过。


## 历史证明存储的有界表示迁移

原id/业务正文/status/supports/limitations/命令及原观察含义不改变；本冻结父 `7f14a1b9c751b0612ceb42224f83f29d09aea46c` 中此对象完整原header/source refs可由公开证据tag恢复。只将具体archive物理依赖交当前位置资产与真实same-role consumer，全部旧义务由显式Eq/CT承接，不把存储重观察冒原业务新审或旧失败变成功。
