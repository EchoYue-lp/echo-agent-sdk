---
schema_version: 1
id: map.sdk-resources-streams
kind: capability_map
title: 资源、流、bounded transport
risk: high
observed_at: source:3ea544c47e331b5d29a6624f978f3595619bef39fa5985230351098d38930404
boundary_refs:
- boundary.sdk-resources-streams
behavior_refs:
- behavior.sdk-resources-streams
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-stream-producer-verification
- evidence.sdk-channel-resource-admission
- evidence.sdk-sandbox-natural-eof-verification
finding_refs:
- finding.sdk-cleanup-retry-owner
- finding.sdk-family-handle-argument-projection
- finding.sdk-stream-producer-settlement
- finding.sdk-channel-resource-operation-admission
- finding.sdk-channel-external-callback-cooperative-close
- finding.sdk-sandbox-natural-eof-buffered-result
audit_refs:
- audit.sdk-stream-producer-settlement-rereview
- audit.sdk-channel-resource-admission-rereview
related_map_refs:
- map.sdk-product-boundary
- map.sdk-events-recovery
- map.sdk-agent-session-run
- map.sdk-integrations
scenarios:
  behavior-lifecycle-failure:
    status: mapped
    source_refs:
    - echo-sdk-host/src/core_profile/handles.rs
    - echo-sdk-host/src/core_profile/facade/mod.rs
    - echo-sdk-host/src/core_profile/facade/stream.rs
    - echo-sdk-host/src/bounded_stdio.rs
    - echo-sdk-host/src/config.rs
    - echo-sdk-host/src/core_profile/facade/stream.rs#accepted_cancel_between_snapshot_and_terminal_commit_returns_cancelled
    - echo-sdk-host/src/core_profile/facade/stream.rs#cancelled_pull_during_failed_cleanup_preserves_error_and_terminal_sequence
    - echo-sdk-host/src/core_profile/facade/mod.rs#family_handle_argument_retains_opaque_resource_address
    - sdks/typescript/test/lsp-host.test.js
    behavior_refs:
    - behavior.sdk-resources-streams
    finding_refs:
    - finding.sdk-cleanup-retry-owner
    - finding.sdk-family-handle-argument-projection
    - finding.sdk-stream-producer-settlement
    review_scope: Entry, unique authority, normal lifecycle and named failure obligations
      have been inspected and linked. Mapped records the model, not bug-free/runtime/main
      closure; open Findings remain explicit.
---

# 资源、流、bounded transport

## 能力范围

统一Registry验证资源kind/owner/generation并保留可重试cleanup owner；typed Handle参数仅投影完整DTO后回到原resource parser/Registry，不转换任意process-local值。stream唯一collector记录真实producer成功/错误，panic/abort/EOF不伪Complete；初次recv感知cancel，未交付terminal不提前消费sequence。accepted cancel与terminal commit在Registry锁下原子决定先后；failed cleanup/caller abort保留owner。quota/pending reservation、MCP同名exact Arc、manager关闭派生LSPclient不复活已有候选units/真实Host证据。

## 入口与输出

源码入口与实际注册/分派路径由本 Map 的 source_refs 定位。输出为 typed response/error、framework committed event 或 generation-fenced handle；所有动态入口仍需逐场景验证。

## 行为关系

本能力经 official ACP/profile 或具体 catalog route 到 Host adapter，然后调用框架权威。跨能力调用与三语言消费见 docs/sdk/governance/capability-map.md。

## 状态与数据流

Framework 拥有 Run/Task/Store 的事实与终态；Host 只拥有寻址、可取消资源 owner 与协议发送状态；客户端确认已消费事实。

## 策略来源与优先级

用户八项治理目标与 AGENTS.md 约束行为；accepted external contract 约束兼容；compiled feature/canonical framework service 约束实现。完整 Rust inventory 不扩承诺。

## 生命周期与失败路径

统一Registry验证资源kind/owner/generation并保留可重试cleanup owner；typed Handle参数仅投影完整DTO后回到原resource parser/Registry，不转换任意process-local值。stream唯一collector记录真实producer成功/错误，panic/abort/EOF不伪Complete；初次recv感知cancel，未交付terminal不提前消费sequence。accepted cancel与terminal commit在Registry锁下原子决定先后；failed cleanup/caller abort保留owner。quota/pending reservation、MCP同名exact Arc、manager关闭派生LSPclient不复活已有候选units/真实Host证据。

仍缺：producer/next/cancel/terminal原子scope已局部resolved且独立复审；cleanup retry与typed handle argument具名Finding保留。Agent cascade、资源发布回滚及最终组合owner审查按现有明确合同分别处理，不能借stream局部通过宣称全资源无缺陷。

## 权限与敏感信息

本地 SDK 不引入线上权限门控。继续保持框架自动行为 permission authority，凭据不进入日志/合同/错误；资源 gate 仅防止错误owner和无意数据丢失。

## 用户侧投影

TypeScript/Python/Java 对已接受能力暴露惯用接口，handle/error/value 必须无损；不能用成功默认值或丢字段改变权威事实。

## 场景处置清单

主场景mapped只表示模型已展开；关联family Behavior仍needs_review，具体Evidence按支持关系与原epoch读取。剩余义务：producer/next/cancel/terminal原子scope已局部resolved且独立复审；cleanup retry与typed handle argument具名Finding保留。Agent cascade、资源发布回滚及最终组合owner审查按现有明确合同分别处理，不能借stream局部通过宣称全资源无缺陷。

## 未展开项

组级主路径mapped表示入口、唯一权威、生命周期与具名失败义务已建模；41个family Behavior仍needs_review，开放Finding及最终交付分别保留。旧候选的八crate临时patch与95ecpin保持原运行epoch；当前Host已pin published b909并移除patch，但最终nightlyprofiles/唯一合同生成与SDKwhole源码门禁仍待。frameworkmain交付证据不替SDK精确main/CI。完整Rust inventory仅遥测；accepted字段必须由唯一生成器明确裁决。

producer/next/cancel/terminal原子scope已局部resolved且独立复审；cleanup retry与typed handle argument具名Finding保留。Agent cascade、资源发布回滚及最终组合owner审查按现有明确合同分别处理，不能借stream局部通过宣称全资源无缺陷。

## 当前观察、归档和后继边界

原fc63证据已保存为仓库内不可变历史收据，原观察epoch不可恢复为Git时不补造提交。原6711证据归档支持具名局部断言；原27个历史输入中tools.rs已变，具名测试body保持可对照；55个6711运行输入中tools.rs和Workflow已变更。新Workflow语义或其它后继场景必须绑定自己的来源/日志；不把旧整套418/TS240/Python300/Java264冒充最终当前通过。支持见历史审计Evidence及evidence.sdk-parent-driver-candidate-verification的限制，family needs_review和开放Finding保留，主场景mapped不扩运行证据。

## 建模完成与验证状态区别

组级主路径已按入口、唯一权威、生命周期和具名失败义务完成映射，故主场景mapped。family Behavior仍needs_review，已知open Finding与有限证据保留；mapped不等于verified、所有lens covered或最终交付。剩余事项采用明确合同/故障原因，不以无限未枚举组合拒绝建模。
