---
schema_version: 1
id: behavior.sdk-language-clients
kind: behavior
status: verified
expectation: human_confirmed
risk: high
primary_focus: state_authority
focus:
- time_lifecycle
- failure_concurrency
- contract_evidence
- data_durability
boundary: boundary.sdk-language-clients
observed_at: 4acd5b81cf3f64b17a7aed546d4aaa2e145e52cb
code_refs:
- sdks/typescript/src/client.ts
- sdks/typescript/src/wire.ts
- sdks/python/src/echo_agent_sdk/client.py
- sdks/python/src/echo_agent_sdk/wire.py
- sdks/java/src/main/java/com/echoagent/sdk/EchoAgentClient.java
- sdks/java/src/main/java/com/echoagent/sdk/WireValues.java
- echo-sdk-protocol/src/scalar.rs
- echo-sdk-protocol/src/error.rs
- contracts/sdk/accepted-external-contract.json
- scripts/check-language-sdks.sh
- sdks/java/src/test/java/com/echoagent/sdk/FactoryCleanupLifecycleTest.java
- sdks/java/src/main/java/com/echoagent/sdk/RunHandle.java
- sdks/java/src/main/java/com/echoagent/sdk/SessionHandle.java
- sdks/typescript/test/session-parent-close.test.js
- sdks/typescript/test/session-parent-close-host.test.js
- sdks/python/tests/test_session_parent_close.py
- sdks/python/tests/test_session_parent_close_host.py
- sdks/java/src/test/java/com/echoagent/sdk/SessionParentCloseTest.java
- sdks/java/src/test/java/com/echoagent/sdk/SessionParentCloseHostTest.java
- docs/adr/0041-session-close-local-child-scope.md
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-session-parent-close-current
- evidence.sdk-source-continuity
- evidence.sdk-event-identity-nested-parent-closure
- evidence.sdk-language-client-lifecycle-current
- evidence.sdk-typescript-stdio-close-specific-closure
- evidence.sdk-java-stdio-close-specific-closure
- evidence.sdk-family-catalog-method-routing-candidate
- evidence.sdk-ack-recovery-verification
- evidence.sdk-stream-cancellation-routing-verification
- evidence.sdk-java-host-early-exit-verification
- evidence.sdk-official-current-language-gate
- evidence.sdk-official-factory-final-language-gate
- evidence.sdk-be5-official-full-language
finding_refs:
- finding.sdk-default-stderr-backpressure
- finding.sdk-event-identity-nested-parent-projection
- finding.sdk-typescript-stdio-close-ownership
- finding.sdk-java-stdio-process-first-close
- finding.sdk-ack-submission-consumption
- finding.sdk-inventory-acceptance-authority
- finding.sdk-language-close-retry
- finding.sdk-language-consumption-ack
- finding.sdk-language-error-fidelity
- finding.sdk-negotiation-response-validation
- finding.sdk-wire-data-fidelity
- finding.sdk-family-catalog-method-routing
- finding.sdk-java-original-child-exit-lifecycle
---

# 三语言与无损值

## 重要承诺

本能力的行为、唯一权威、生命周期、失败路径与无损合同必须有可复核证据。

## 当前行为

本次客户端实际源码与具名生命周期复审已绑定76601，八个风险视角均有界examined、C/I/M0；详见文末当前范围。以下未带本次标识的源/计数描述保留原epoch，不当作当前未修或全量成功。

TS/Python/Java经官方ACP transport消费同一accepted合同；严格value/error投影保留大整数canonical text、Unicode/path/time/bytes、handle/operation/retryable/details。ACK只在已消费或显式处理snapshot/replay coverage后提交；提交失败/取消不写“已成功ACK”，gap不autoACK，迟订阅有界feed与terminal replay自然EOF已有真实场景。close保同一transport/cleanup owner，不伪关闭。LSP原29request及后继42diagnostic共同语料比较结果、完整typed错误和真实请求/通知副作用，均通过。

## 期望行为

保持框架业务权威、Host寻址与资源ownership、三语言无损投影；所有已确认缺陷修复并取得最新反例验证。

## 触发、结果与副作用

源码入口与实际注册/分派路径由本 Map 的 source_refs 定位。输出为 typed response/error、framework committed event 或 generation-fenced handle；所有动态入口仍需逐场景验证。

## 失败、重试与恢复

TS/Python/Java经官方ACP transport消费同一accepted合同；严格value/error投影保留大整数canonical text、Unicode/path/time/bytes、handle/operation/retryable/details。ACK只在已消费或显式处理snapshot/replay coverage后提交；提交失败/取消不写“已成功ACK”，gap不autoACK，迟订阅有界feed与terminal replay自然EOF已有真实场景。close保同一transport/cleanup owner，不伪关闭。LSP原29request及后继42diagnostic共同语料比较结果、完整typed错误和真实请求/通知副作用，均通过。

原569阶段缺口记录（历史，不是本次仍缺清单）：历史fc63完整TS219/Python279+18/Java233已转不可变收据归档；当前6711候选运行原记录为Rust418、TS240/Python300+18/Java264。旧日志不能改称后来源码验证；新Workflow生产变化后整体418/语言门禁不再作为当前完整通过，需其独立后继证据。 全accepted helpers/Host field统一corpus、Windows路径和Task DAG三语言场景仍缺；Diagnostic supporting union/source生成及最终正式pin完整语言/示例/CI须重新绑定最新源码。

## 证据

具名候选运行范围由本组Map关联的child family Behaviors及其 `evidence.sdk-candidate-behavior-verification` 定位；该Evidence只supports具体family，不把其引用冒充本组完整通过。

历史阶段说明（旧manifest/pin已由后继切换，原原文保留）：所有状态保持 needs_review。具名断言的候选通过不证明能力全覆盖：SDK manifest 固定95ec66c6，八crate临时patch组合Task/ACP/LSP源码用于本地验证；最终Channel后继published revision、移除patch后的独立构建、精确main和远端CI仍待。完整Rust inventory仅遥测，最终accepted字段/退役必须由唯一生成器明确裁决。

历史fc63完整TS219/Python279+18/Java233已转不可变收据归档；当前6711候选运行原记录为Rust418、TS240/Python300+18/Java264。旧日志不能改称后来源码验证；新Workflow生产变化后整体418/语言门禁不再作为当前完整通过，需其独立后继证据。 全accepted helpers/Host field统一corpus、Windows路径和Task DAG三语言场景仍缺；Diagnostic supporting union/source生成及最终正式pin完整语言/示例/CI须重新绑定最新源码。

## 裁决记录

用户明确要求完成八项治理，三语言范围只以明确接受合同为准；已授权子智能体执行，主线程负责协调及验收。

## 当前观察、归档和后继边界

原fc63证据已保存为仓库内不可变历史收据，原观察epoch不可恢复为Git时不补造提交。原6711证据归档支持具名局部断言；原27个历史输入中tools.rs已变，具名测试body保持可对照；55个6711运行输入中tools.rs和Workflow已变更。新Workflow语义或其它后继场景必须绑定自己的来源/日志；不把旧整套418/TS240/Python300/Java264冒充最终当前通过。支持见历史审计Evidence及evidence.sdk-parent-driver-candidate-verification的限制，所有needs_review/开放Finding保留。

## 原569模型重观察范围（历史）

review_scope：本对象在真实当前候选Git `56929154c1851473e9e4837901b610cecf001b2d`（源码摘要`17cf243e8d6341db0ce23c5b9ee39977ce4ab100d6ed1b17ec5a7bb26c19e138`）上重新观察。上列code_refs/各scenario source_refs已逐Gitblob读取并与当前文件SHA精确相同，原锚点均存在。模型继续描述原accepted入口、框架业务/终态权威、Host owner/generation与transport消费职责、生命周期及具名失败义务；源码定位核对不证明所有行为正确、运行证据fresh或合同生成闭合。

原Behavior/status、Map场景mapped/needs_review、Rule/Asset处置、Finding/未决义务与Evidence原epoch支持范围全部保留。mapped仅入口模型，旧3ea不是本snapshot本体；历史receipt不得因本模型观察更新冒current运行。12source迁移、5秒物理结算与依赖50warning规则三选择仍未接受，完整生成/CI/main另验，任何新Rustwrapper不自动扩三语言合同。

## 当前客户端有界审查与剩余验收

唯一owner仍为本Behavior，不创建近义生命周期模型。观察Git76601包含TSdd822、Pythonclient30ca/3.10importcd076、Java86efd及正式来源清单；审查不是重标旧569运行。八份audit.sdk-language-client-lifecycle-*分别保存实际输入、结果、副作用、状态权威、消费水位、时间/并发、权限和合同证据故障假设。freshness=examined代表这些具名义务已实际源审，status仍needs_review，不自动证明所有41family/八个全局lens闭合。

客户端不再泛称当前native值/错误/ACK/关闭与Task场景均缺证：原ACK failed/pending不commit、连通replay覆盖、原callback取消地址、独立close waiters、EOF/lateIO/真实子进程、Factory失败重试和Python最低版本已有各自精确来源。原新a634官方完整命令曾exit1的TaskHTTP error保历史；之后同入口100567bytes/exit0、790输入前后同，TS308/PY375+18/Java332及71XML/3quickstarts通过。该单一fixture Connection close修复有受控前后反例，未声称原随机socket直接因果，也不把Store业务权威挪到harness。

TS与Java两specific stdio关闭缺陷的repair/当前same-source verification/实际source rereview链已独立完整，可局部resolved；当前consumer已在已发布a634上通过原public-close XML。其余Finding保持各自真实状态；尤其12项accepted source/kind迁移、5秒物理结算B、完整唯一generator、平台/SDK main仍独立。没有新增测试数分母或未接受Rust identity义务。旧fc63/6711/569阶段的原正文与收据仍保留，依赖50组政策已由人明确接受，不再将其旧waiting文字当当前产品选择。

## 原已保存源码出处 (569 historical)

56929154c1851473e9e4837901b610cecf001b2d : sdks/typescript/test/lsp-host.test.js, sdks/python/tests/test_lsp_diagnostics_host.py. 这两份测试在原SDKSnapshot可恢复，不在本次76601中；原LSP行为与断言义务保留，不伪装为新提交测试源，不宣称8lens审查替代全LSP契约生成验收。


## nested parent局部后继观察

Python与Java原纯值构造入口丢失 accepted runtime.subagent_lineage.parent_event_id：Python先将lineage容器转字符串再查字段，Java只查flat字段。原0ebe同一Python/Java probe各exit1；TS原nested peer exit0。185559仅修原投影：保容器到叶值转换、原flat优先与语言各自空值校验、不变UUID/其余identity字段/immutable更新；不新增API或Host状态权威。实际after同probe各exit0；Python3.10原模块3项与JUnit类3项均0失败/0skip，Java实际Oracle21运行且--release17编译通过，不冒已执行JDK17。SDKcap原源码、具名pure-value测试及材料审PASS C0/I0/M0；本元作者只回收该有限结论。

仅这一个accepted pure-value parent投影局部闭合；不是全部lineage/跨进程事件/Host集成或Linux/JDK17完整语言验证。Map/Behavior仍保持原needs_review。B3、12accepted-source、合同13、PID/Linux和Host退出未知原因独立。历史raw不新增S包，下面Admin定位可保原byte恢复，不伪source_refs已在Git。


## 2026-10-20 具名有限结算

本项整体继续needs_review。原具名子链已有后继并不授整个composite或媒体最终资格；保原header观察版本和历史runtime cut，当前04eb源码定位及受控增量另记。

直接证据：`evidence.sdk-language-client-lifecycle-current`、`evidence.sdk-typescript-stdio-close-specific-closure`、`evidence.sdk-java-stdio-close-specific-closure`、`evidence.sdk-event-identity-nested-parent-closure`。

具体复审入口：`audit.sdk-language-client-lifecycle-contract-evidence`、`audit.sdk-language-client-lifecycle-state-authority`；只读者已按这一个named单元核权威、生命周期、失败与证据边界，不能由共享对象推整八风险组。

源码后继分开核：原Root17be/0bbc具名review与04eb薄增量分别C0。04eb相对17be只在media lookup保两accepted名字到原Tool，三测试wrapper新增可选媒体故事，原fixture五函数/默认HTTP/旧oracles不变；本项所依非媒体生产路径不因共有文件名改变而假报重审或全套新运行。新媒体after另由a17/三语focused证明，未借旧208a14为新源码完整资格。

只结算已审local/wire/owner/ACK等named链；整体managed/platform/main/Hook仍hold，Oracle21不冒CI17。


## 2026-10-22 默认诊断ownership具名后继

仅增加`finding.sdk-default-stderr-backpressure`与复用`evidence.sdk-source-continuity`、
`audit.sdk-language-client-lifecycle-failure-concurrency`的新challenge链。Python19/20与TS21B/22
各自独立before/repair/native after，24五场真实Host只支持原诊断/退出/ACK/flush范围。
原观察版本与其source_refs/既有场景、status保持；本组仍needs_review（Map仍mapped），
不由这一具体Finding局部resolved升级四whole、managed历史UNKNOWN或完整语言/main。


## 2026-10-22 精main与已接受义务的最终结算

本节是精公开main `a9f267f636cd4d1553a8bff335c3203ab0b8a5ac` 的最终accepted范围结算，
旧正文中的needs_review/仍待、各observed_at和原runtime计数保留为历史，不改称本次执行。
本次逐Git读取全部header code_refs及锚点均存在；当前观察header仅表示该源码复核，旧Evidence运行epoch不重标。
精main PR7 soleparent09f/tree01ac、签名valid，原pushCI39709057560 attempt1九job全SUCCESS，
only-GitHub-main+36必要refs冷strict实际0；accepted5621/454保持，不扩大完整Rust库存或所有可能输入组合。

| 已接受义务 | 最终依据与保留边界 |
|---|---|
| 官方ACP transport与无损值/error/handle/operation投影 | 原accepted5621/454合同、Unicode/path/大整数/bytes/time/错误/重试细节、nested parent及source-operation具名证据；完整Rust inventory只遥测。 |
| ACK/迟订阅/feed/replay与失败不commit | 原ACK/recovery/consumption与failed/pending/cancelled边界，既有八lens和真实consumer证据不被总计数代替。 |
| stdio/Factory/public close与默认诊断 | 原TS/Java具体close与Python3.10、默认stderr19/20/21B/22/24链；五telemetry原Host/proxy自然0仅五场，shutdown waiter transport异常不说成功响应。 |
| 最终完整三语/示例与Java runtime | 精main Linux原入口actual0/三quickstarts，TS363通过+1Windows未运行、Python450通过+1Windows未运行另18subtests；Temurin17，Java原mvn-q整体成功，不冒本地380/76XML为CI逐case结果。 |
| Windows原生路径消费者 | 精main Windows job原TS/PY/Java各1目标case通过、0fail/0CASEskip、各4nativecalls，保UTF16孤立代理，freshPYJavaXML。TS其它namefilter未选不算目标skip；不等于全Windows SDK测试。 |

本组按实际两平台accepted义务verified，不把Mac单机收窄为全部承诺；不额外接受managed历史风险，也不泛推所有Host/remoteowner自然退出。

## 2026-10-23 Session 父关闭的本地子 scope 后继

header 当前源码观察为真实 Git `4acd5b81cf3f64b17a7aed546d4aaa2e145e52cb`；
旧正文、Evidence 的运行 epoch、main CI 和 accepted 风险裁决均按原范围保留。
本次新增证据仅是 `evidence.sdk-session-parent-close-current`。

Session 只在成功收到原 `session/close` 回复后处置本地 Run/feed scope：所有已返回
对象（包括同 wire 的不同本地 alias）都收到该信号；晚到 start 回复立即本地关闭；
缓冲事实可读，之后 EOF，既不合成 terminal/receipt，也不证明遗失的 Host 通知已恢复。
父关闭失败时保留子 scope 供重试。TS/Java 的首次迟订阅保持已结束 mailbox；Python
维持既有 closed-handle 查询/新订阅拒绝，不重开已释放的 Host Run。

并发 close 共享原操作 owner；Python shield 和 Java dependent future 让观察者取消
不能取消原关闭。scope 处置可结束本地 ACK/child-close 等待，但不取消已提交 RPC/ACK、
不推进已确认消费水位。`closed_handle`/`stale_handle`/`host_exited` 可释放本地订阅；
`host_shutting_down`、settlement timeout 等保留 typed failure 和重试状态。权威终态仍归
framework/Host；没有新增 wire 路由、公共 accepted member、全局子句柄 registry 或状态机。

原完整语言入口在 macOS 实际0：TS380通过/1平台skip，Python463通过/1平台skip另18subtests，
Java78 fresh XML共394项、393执行通过/1平台skip、0错误；三quickstart实际执行。原61项工程
输入与当前源码相同，具名独审 PASS 及本地真实Host两窗口证据见新Evidence。
这些不是 Linux/Windows 或 Temurin17 新运行，也不替代当前精main CI；原历史UNKNOWN
接受及其余能力覆盖不因这个有限后继被扩大或重新结算。
