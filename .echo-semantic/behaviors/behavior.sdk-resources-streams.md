---
schema_version: 1
id: behavior.sdk-resources-streams
kind: behavior
status: needs_review
expectation: human_confirmed
risk: high
primary_focus: state_authority
focus:
- time_lifecycle
- failure_concurrency
- contract_evidence
- data_durability
boundary: boundary.sdk-resources-streams
observed_at: 56929154c1851473e9e4837901b610cecf001b2d
code_refs:
- echo-sdk-host/src/core_profile/handles.rs
- echo-sdk-host/src/core_profile/facade/mod.rs
- echo-sdk-host/src/core_profile/facade/stream.rs
- echo-sdk-host/src/bounded_stdio.rs
- echo-sdk-host/src/config.rs
- echo-sdk-host/src/core_profile/facade/stream.rs#accepted_cancel_between_snapshot_and_terminal_commit_returns_cancelled
- echo-sdk-host/src/core_profile/facade/stream.rs#cancelled_pull_during_failed_cleanup_preserves_error_and_terminal_sequence
- echo-sdk-host/src/core_profile/facade/mod.rs#family_handle_argument_retains_opaque_resource_address
- sdks/typescript/test/lsp-host.test.js
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-stream-producer-verification
- evidence.sdk-channel-resource-admission
- evidence.sdk-llm-workflow-stream
- evidence.sdk-sandbox-natural-eof-verification
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

仍缺：全部resource/stream种类的慢消费者、close故障和quota回滚三语言同场景矩阵未覆盖；framework driver生命周期与最新后继源码与 retained cleanup/transport的精确独立复审、最终pin一致验收仍待。

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
