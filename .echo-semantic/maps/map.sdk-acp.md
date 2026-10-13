---
schema_version: 1
id: map.sdk-acp
kind: capability_map
title: ACP 与协商
risk: high
observed_at: source:3ea544c47e331b5d29a6624f978f3595619bef39fa5985230351098d38930404
boundary_refs:
- boundary.sdk-acp
behavior_refs:
- behavior.sdk-acp
- behavior.sdk-family-standard
rule_refs:
- rule.framework-runtime-authority
evidence_refs: []
finding_refs:
- finding.sdk-negotiation-response-validation
audit_refs: []
related_map_refs:
- map.sdk-product-boundary
scenarios:
  behavior-lifecycle-failure:
    status: mapped
    source_refs:
    - echo-sdk-host/src/lib.rs
    - echo-sdk-host/src/factory.rs
    - echo-sdk-host/src/features.rs
    - echo-sdk-host/src/core_profile/mod.rs
    - echo-sdk-host/src/core_profile/state.rs
    - echo-sdk-protocol/src/capability.rs
    - echo-sdk-host/src/lib.rs#run_stdio_with_close_owner
    - echo-sdk-host/tests/stdio_e2e.rs
    - echo-sdk-host/tests/core_profile_e2e.rs#extended_standard_prompt_bridges_shared_core_handles
    behavior_refs:
    - behavior.sdk-acp
    finding_refs:
    - finding.sdk-negotiation-response-validation
    review_scope: Entry, unique authority, normal lifecycle and named failure obligations
      have been inspected and linked. Mapped records the model, not bug-free/runtime/main
      closure; open Findings remain explicit.
  family-standard:
    status: needs_review
    source_refs:
    - echo-sdk-host/src/lib.rs
    - echo-sdk-host/src/factory.rs
    - echo-sdk-host/src/features.rs
    - echo-sdk-host/src/core_profile/mod.rs
    - echo-sdk-host/src/core_profile/state.rs
    - echo-sdk-protocol/src/capability.rs
    behavior_refs:
    - behavior.sdk-family-standard
    unknown: 该family已有具名候选断言；完整最低充分行为及最终published pin/source、独立复审仍未闭合。testing仅声明库存无运行supports。
    next_step: 沿关联family Behavior的最低充分证据补未列场景；最终源码/二进制/合同冻结后独立复审及精确main对账，不重跑或抹除已有反例。
    evidence_refs:
    - evidence.sdk-candidate-behavior-verification
    - evidence.sdk-parent-driver-candidate-verification
---

# ACP 与协商

## 能力范围

`lib.rs::run_stdio_with_close_owner` / factory 组合一个官方ACP connection；core_profile::attach只在该连接注册扩展。编译features与accepted hello/digest校验是协商来源，不通过运行时配置伪造能力。关闭owner保留原actor到framework receipt/cleanup，并实际join actor，panic/abort/config无runtime明确失败。诊断Host4d436b60…的标准stdio4、Hostlib136/core31/bridge34/facade12均通过；plain标准入口有直接证据，不再表述为全部未执行。

## 入口与输出

源码入口与实际注册/分派路径由本 Map 的 source_refs 定位。输出为 typed response/error、framework committed event 或 generation-fenced handle；所有动态入口仍需逐场景验证。

## 行为关系

本能力经 official ACP/profile 或具体 catalog route 到 Host adapter，然后调用框架权威。跨能力调用与三语言消费见 docs/sdk/governance/capability-map.md。

## 状态与数据流

Framework 拥有 Run/Task/Store 的事实与终态；Host 只拥有寻址、可取消资源 owner 与协议发送状态；客户端确认已消费事实。

## 策略来源与优先级

用户八项治理目标与 AGENTS.md 约束行为；accepted external contract 约束兼容；compiled feature/canonical framework service 约束实现。完整 Rust inventory 不扩承诺。

## 生命周期与失败路径

`lib.rs::run_stdio_with_close_owner` / factory 组合一个官方ACP connection；core_profile::attach只在该连接注册扩展。编译features与accepted hello/digest校验是协商来源，不通过运行时配置伪造能力。关闭owner保留原actor到framework receipt/cleanup，并实际join actor，panic/abort/config无runtime明确失败。诊断Host4d436b60…的标准stdio4、Hostlib136/core31/bridge34/facade12均通过；plain标准入口有直接证据，不再表述为全部未执行。

仍缺：标准ACP已有四项真实stdio场景。尚待最终无patch published pin独立构建，以及已接受协议版本、startup/EOF错误签名与平台关闭约束的精确对账；不要求未承诺的平台组合。

## 权限与敏感信息

本地 SDK 不引入线上权限门控。继续保持框架自动行为 permission authority，凭据不进入日志/合同/错误；资源 gate 仅防止错误owner和无意数据丢失。

## 用户侧投影

TypeScript/Python/Java 对已接受能力暴露惯用接口，handle/error/value 必须无损；不能用成功默认值或丢字段改变权威事实。

## 场景处置清单

主场景mapped只表示模型已展开；关联family Behavior仍needs_review，具体Evidence按支持关系与原epoch读取。剩余义务：标准ACP已有四项真实stdio场景。尚待最终无patch published pin独立构建，以及已接受协议版本、startup/EOF错误签名与平台关闭约束的精确对账；不要求未承诺的平台组合。

## 未展开项

组级主路径mapped表示入口、唯一权威、生命周期与具名失败义务已建模；41个family Behavior仍needs_review，开放Finding及最终交付分别保留。旧候选的八crate临时patch与95ecpin保持原运行epoch；当前Host已pin published b909并移除patch，但最终nightlyprofiles/唯一合同生成与SDKwhole源码门禁仍待。frameworkmain交付证据不替SDK精确main/CI。完整Rust inventory仅遥测；accepted字段必须由唯一生成器明确裁决。

标准ACP已有四项真实stdio场景。尚待最终无patch published pin独立构建，以及已接受协议版本、startup/EOF错误签名与平台关闭约束的精确对账；不要求未承诺的平台组合。

## 当前观察、归档和后继边界

原fc63证据已保存为仓库内不可变历史收据，原观察epoch不可恢复为Git时不补造提交。原6711证据归档支持具名局部断言；原27个历史输入中tools.rs已变，具名测试body保持可对照；55个6711运行输入中tools.rs和Workflow已变更。新Workflow语义或其它后继场景必须绑定自己的来源/日志；不把旧整套418/TS240/Python300/Java264冒充最终当前通过。支持见历史审计Evidence及evidence.sdk-parent-driver-candidate-verification的限制，family needs_review和开放Finding保留，主场景mapped不扩运行证据。

## 建模完成与验证状态区别

组级主路径已按入口、唯一权威、生命周期和具名失败义务完成映射，故主场景mapped。family Behavior仍needs_review，已知open Finding与有限证据保留；mapped不等于verified、所有lens covered或最终交付。剩余事项采用明确合同/故障原因，不以无限未枚举组合拒绝建模。
