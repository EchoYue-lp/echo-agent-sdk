---
schema_version: 1
id: behavior.sdk-resources-streams
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
boundary: boundary.sdk-resources-streams
observed_at: a9f267f636cd4d1553a8bff335c3203ab0b8a5ac
code_refs:
- echo-sdk-host/src/core_profile/handles.rs
- echo-sdk-host/src/core_profile/facade/mod.rs
- echo-sdk-host/src/core_profile/facade/stream.rs
- echo-sdk-host/src/bounded_stdio.rs
- echo-sdk-host/src/config.rs
- echo-sdk-host/src/core_profile/facade/stream.rs#accepted_cancel_between_snapshot_and_terminal_commit_returns_cancelled
- echo-sdk-host/src/core_profile/facade/stream.rs#cancelled_pull_during_failed_cleanup_preserves_error_and_terminal_sequence
- echo-sdk-host/src/core_profile/facade/mod.rs#family_handle_argument_retains_opaque_resource_address
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-source-continuity
- evidence.sdk-stream-producer-verification
- evidence.sdk-channel-resource-admission
- evidence.sdk-llm-workflow-stream
- evidence.sdk-sandbox-natural-eof-verification
- evidence.sdk-events-resources-finite-observation
finding_refs:
- finding.sdk-cleanup-retry-owner
- finding.sdk-family-handle-argument-projection
- finding.sdk-stream-producer-settlement
- finding.sdk-channel-resource-operation-admission
- finding.sdk-sandbox-natural-eof-buffered-result
---

# 资源、流、bounded transport

## 重要承诺

本能力的行为、唯一权威、生命周期、失败路径与无损合同必须有可复核证据。

## 当前行为

统一Registry验证资源kind/owner/generation并保留可重试cleanup owner；typed Handle参数仅投影完整DTO后回到原resource parser/Registry，不转换任意process-local值。stream唯一collector记录真实producer成功/错误，panic/abort/EOF不伪Complete；初次recv感知cancel，未交付terminal不提前消费sequence。accepted cancel与terminal commit在Registry锁下原子决定先后；failed cleanup/caller abort保留owner。quota/pending reservation、MCP同名exact Arc、manager关闭派生LSPclient不复活已有候选units/真实Host证据。

## 期望行为

保持框架业务权威、Host寻址与资源ownership、三语言无损投影；所有已确认缺陷修复并取得最新反例验证。

## 触发、结果与副作用

源码入口与实际注册/分派路径由本 Map 的 source_refs 定位。输出为 typed response/error、framework committed event 或 generation-fenced handle；所有动态入口仍需逐场景验证。

## 失败、重试与恢复

统一Registry验证资源kind/owner/generation并保留可重试cleanup owner；typed Handle参数仅投影完整DTO后回到原resource parser/Registry，不转换任意process-local值。stream唯一collector记录真实producer成功/错误，panic/abort/EOF不伪Complete；初次recv感知cancel，未交付terminal不提前消费sequence。accepted cancel与terminal commit在Registry锁下原子决定先后；failed cleanup/caller abort保留owner。quota/pending reservation、MCP同名exact Arc、manager关闭派生LSPclient不复活已有候选units/真实Host证据。

原569阶段缺口（历史，当前见后继有限审查）：全部resource/stream种类的慢消费者、close故障和quota回滚三语言同场景矩阵未覆盖；framework driver生命周期与最新后继源码与 retained cleanup/transport的精确独立复审、最终pin一致验收仍待。

## 证据

本组Map中的stream/Registry具名测试锚点定位候选断言；共享资源义务另关联events、agent与integration Map的family Evidence，后者只supports具体family。本组没有据此建立完整通过Evidence。

所有状态保持 needs_review。具名断言的候选通过不证明能力全覆盖：SDK manifest 固定95ec66c6，八crate临时patch组合Task/ACP/LSP源码用于本地验证；最终Channel后继published revision、移除patch后的独立构建、精确main和远端CI仍待。完整Rust inventory仅遥测，最终accepted字段/退役必须由唯一生成器明确裁决。

全部resource/stream种类的慢消费者、close故障和quota回滚三语言同场景矩阵未覆盖；framework driver生命周期与最新后继源码与 retained cleanup/transport的精确独立复审、最终pin一致验收仍待。

## 裁决记录

用户明确要求完成八项治理，三语言范围只以明确接受合同为准；已授权子智能体执行，主线程负责协调及验收。

## 当前观察、归档和后继边界

原fc63证据已保存为仓库内不可变历史收据，原观察epoch不可恢复为Git时不补造提交。原6711证据归档支持具名局部断言；原27个历史输入中tools.rs已变，具名测试body保持可对照；55个6711运行输入中tools.rs和Workflow已变更。新Workflow语义或其它后继场景必须绑定自己的来源/日志；不把旧整套418/TS240/Python300/Java264冒充最终当前通过。支持见历史审计Evidence及evidence.sdk-parent-driver-candidate-verification的限制，所有needs_review/开放Finding保留。

## 当前模型重观察范围

review_scope：本对象在真实当前候选Git `56929154c1851473e9e4837901b610cecf001b2d`（源码摘要`17cf243e8d6341db0ce23c5b9ee39977ce4ab100d6ed1b17ec5a7bb26c19e138`）上重新观察。上列code_refs/各scenario source_refs已逐Gitblob读取并与当前文件SHA精确相同，原锚点均存在。模型继续描述原accepted入口、框架业务/终态权威、Host owner/generation与transport消费职责、生命周期及具名失败义务；源码定位核对不证明所有行为正确、运行证据fresh或合同生成闭合。

原Behavior/status、Map场景mapped/needs_review、Rule/Asset处置、Finding/未决义务与Evidence原epoch支持范围全部保留。mapped仅入口模型，旧3ea不是本snapshot本体；历史receipt不得因本模型观察更新冒current运行。12source迁移、5秒物理结算与依赖50warning规则三选择仍未接受，完整生成/CI/main另验，任何新Rustwrapper不自动扩三语言合同。

## 原历史测试出处

56929154c1851473e9e4837901b610cecf001b2d 中保留 sdks/typescript/test/lsp-host.test.js。这些原测试和具名义务仍属于其已保存源码/收据，不伪装成cd5包含的源码；本次group审查由真实runtime69b、正式cd5证明和各原slice审查支持。

## 当前有界审查与明确剩余

本对象观察绑定真实cd5，所列原运行源码69b在cd5逐Gitblob相同；新Session/RunA实施已变handler/handles/state，不能以本观察宣称最新mutable行为已验证。八份audit.sdk-events-resources-finite-*记录ACK原作者实际只读审查；作者身份不冒独立review，Host events/stream/client各原独立scope和新liveJournal当前Tool独审范围明确保留。

原#120、ACK消费与stream具体缺陷三链resolved继续有效，不重复造关闭对象或操作外部Issue。新增liveJournal场景证明settledRun且Host/connection仍活时目录换代两次明确replay_unavailable/after_delay、原Runreceipt/peerSession完整page不变，restore同inode/原两fileSHA后完整page相同；三语言12/12/9零skip。它是既有正确失败路径补证，不是生产redgreen；不扩为activewritepartialcrash或任意网络故障。

已接受event-replay及共享资源stream的入口、单一authority、正常生命周期/具名错误和恢复已有finiteexamined闭包；映射仅boundedmodel，Behavior保持needs_review，八个全局风险和baseline不因此全covered。宽cleanup与5秒B保持原Finding理由，12身份/完整生成/平台/main独立；其它resourcefamily/sourceoperation未审方法不从这条共享owner路径泛推全部无损。


## 2026-10-20 具名有限结算

本项整体继续needs_review。原具名子链已有后继并不授整个composite或媒体最终资格；保原header观察版本和历史runtime cut，当前04eb源码定位及受控增量另记。

直接证据：`evidence.sdk-stream-producer-verification`、`evidence.sdk-channel-resource-admission`、`evidence.sdk-llm-workflow-stream`、`evidence.sdk-sandbox-natural-eof-verification`、`evidence.sdk-events-resources-finite-observation`。

具体复审入口：`audit.sdk-events-resources-finite-contract-evidence`、`audit.sdk-events-resources-finite-data-durability`、`audit.sdk-events-resources-finite-failure-concurrency`、`audit.sdk-events-resources-finite-permission-external`、`audit.sdk-events-resources-finite-result-side-effect`、`audit.sdk-events-resources-finite-state-authority`、`audit.sdk-events-resources-finite-time-lifecycle`、`audit.sdk-events-resources-finite-trigger-input`；只读者已按这一个named单元核权威、生命周期、失败与证据边界，不能由共享对象推整八风险组。

源码后继分开核：原Root17be/0bbc具名review与04eb薄增量分别C0。04eb相对17be只在media lookup保两accepted名字到原Tool，三测试wrapper新增可选媒体故事，原fixture五函数/默认HTTP/旧oracles不变；本项所依非媒体生产路径不因共有文件名改变而假报重审或全套新运行。新媒体after另由a17/三语focused证明，未借旧208a14为新源码完整资格。

原handle/stream/EOF/cancel具名证据保留；managed历史Linux/main/Hook使整个composite继续hold。


## 2026-10-22 精main与已接受义务的最终结算

本节是精公开main `a9f267f636cd4d1553a8bff335c3203ab0b8a5ac` 的最终accepted范围结算，
旧正文中的needs_review/仍待、各observed_at和原runtime计数保留为历史，不改称本次执行。
本次逐Git读取全部header code_refs及锚点均存在；当前观察header仅表示该源码复核，旧Evidence运行epoch不重标。
精main PR7 soleparent09f/tree01ac、签名valid，原pushCI39709057560 attempt1九job全SUCCESS，
only-GitHub-main+36必要refs冷strict实际0；accepted5621/454保持，不扩大完整Rust库存或所有可能输入组合。

| 已接受义务 | 最终依据与保留边界 |
|---|---|
| kind/owner/generation与typed Handle地址 | 原Registry/完整DTO/opaque resource address与family-handle三链，不引入第二状态权威。 |
| stream生产者终态、EOF/panic/error与terminal次序 | 原producer、sandbox natural EOF、accepted cancel/terminal锁下仲裁和failed cleanup保owner具名链；不以drop/EOF伪成功。 |
| ACK/replay与消费水位 | 原events-resources finite及liveJournal换代明确replay_unavailable/after_delay、Run receipt/peer page与原文件恢复证据；非active-write crash或任意网络故障保证。 |
| admission/quota及cleanup retry | 原Channel resource admission与A2A真实配额回滚/关闭、MCP精Arc/LSP不复活共享owner证据；重试保同owner，不放宽失败路径。 |
| 最终源码/运行/合同 | 原14Rust feature epochs和193输入等价、113阻断合同、main九CI及完整三语/三个quickstarts与冷恢复。 |

本对象不包含历史managed首Host因果义务，不借人的风险接受清掉资源缺口；本次按原accepted资源/流义务独立verified。
未列任意backend/所有输入组合、平台全矩阵或未来API均未新增承诺。
