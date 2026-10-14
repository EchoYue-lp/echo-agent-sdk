---
schema_version: 4
slug: 2026-10-13T1800-channel-handler-forward/plan
outcome:
  summary: 闭合 MessageHandler 三项已接受能力的真实 Channel 前向输入与三语言消费生命周期
  acceptance:
    - 设计的十二组场景在最终源码快照有通过证据，TypeScript、Python、Java 均经真实 Host 消费
      handle、handle_stream 与 reply；不能用注册/send/stop 或 reverse 模拟代替。
    - 六个同 channels family operations、typed schema 与输入输出 digest
      由唯一生产生成器形成并通过合同与三语言门禁；原三项接受身份保留，新 framework wrapper 不自动扩大 accepted scope。
    - 独立设计/源码复审及适用仓库全量门禁通过，framework successor 已发布且 SDK 精确 pin，SDK exact-main
      验收后才关闭 consumer Finding。
out_of_scope:
  - 退役或延期三项原 MessageHandler 承诺；扩展其它完整 Rust inventory 为跨语言义务。
  - 第二 handler registry/store/executor、第二 transport method、EKO UI 或 provider 全排列。
  - 改变当前已冻结 framework 里程碑的源码/ADR/验证快照；SDK 包发布。
design_ref: docs/supreme/specs/2026-10-13T1800-channel-handler-forward/design.md
design_sections:
  - ref: design.md § 问题与目标
    digest: sha256:03d7208e60257266234a12daa8deef04060262575c9486aa70d4bfdb956d5471
  - ref: design.md § 目标行为
    digest: sha256:49db04e4bc46fbf577335ed392d14c6b117f84820e0f75e10953df3d709cdf8b
  - ref: design.md § 范围与非目标
    digest: sha256:7e6ec9bc704a681758d0bf689d234bf85ab0f41552be60fb7545daf5b59bc4c3
  - ref: design.md § 系统边界
    digest: sha256:f4816d4b67d65a4b041aa6bbebef93a38b4791a0c685ae8106fbe39aab2b19bd
  - ref: design.md § 核心结构与数据流
    digest: sha256:a7aaf901375461817ddb130238a25197bb8eed38b68790fff0086de57a68aaf1
  - ref: design.md § 生命周期与异常场景
    digest: sha256:49e60724f32e4918a7e89a3a518030dedc470e74e837a215b5e6917435e5f5ed
  - ref: design.md § 关键取舍与业界依据
    digest: sha256:dcf7cda4c2b09efd7c83d2fba01da264b570f905a1e507037211777aa4571171
  - ref: design.md § 复用与实现约束
    digest: sha256:8fede7f1259d6aaa33271264a98c6279aa15fefa40bb8336bb8010e4a9b5b84a
  - ref: design.md § 验收标准
    digest: sha256:4bed940b3e5d3b0f5902a384782088357debb8b626196265b32cc2f78f9413e9
  - ref: design.md § 源码证据与当前交付边界
    digest: sha256:e7d49243df6be5cef72e43c7887684882c5f766a9aa8e79950ceaefe9cc3cc2c
  - ref: design.md § 后继框架 ADR 的通用机制边界
    digest: sha256:919fd613b8feaa2ca167dd4fe3a9775dd399d8c84f5a771e1530f1c4c03197d8
delivery_ref: null
todos:
  - id: original-entry-invocation
    summary: 原 framework entry 可准确准入当前成功启动的 handler 并在关闭时取消和结算真实调用
    files:
      - /Users/ls/MyWork/code/ylp_agent_learn/lp-agent/.worktrees/echo-agent-worktrees/channel-handler-forward/echo-integration/src/channels/manager.rs
      - /Users/ls/MyWork/code/ylp_agent_learn/lp-agent/.worktrees/echo-agent-worktrees/channel-handler-forward/echo-integration/src/channels/types.rs
      - /Users/ls/MyWork/code/ylp_agent_learn/lp-agent/.worktrees/echo-agent-worktrees/channel-handler-forward/echo-integration/src/channels/session.rs
      - /Users/ls/MyWork/code/ylp_agent_learn/lp-agent/.worktrees/echo-agent-worktrees/channel-handler-forward/src/channels.rs
      - /Users/ls/MyWork/code/ylp_agent_learn/lp-agent/.worktrees/echo-agent-worktrees/channel-handler-forward/docs/adr/0085-channel-handler-invocation-ownership.md
      - /Users/ls/MyWork/code/ylp_agent_learn/lp-agent/.worktrees/echo-agent-worktrees/channel-handler-forward/echo-core/src/error.rs
    acceptance:
      - 后继 framework 分支提供 ChannelHandle 三项原 entry invocation，start 未成功的 cleanup
        debt 不可接消息；stop/restart fencing、slow stream、close retry、metadata reentry
        均以真实 receipt 验证。
  - id: typed-forward-admission
    summary: SDK 三个 forward handler 操作与三个 channels stream controls 通过原
      HandleRegistry 和 typed bridge 无损调用
    files:
      - echo-sdk-protocol/src/methods.rs
      - echo-sdk-protocol/src/facade.rs
      - echo-sdk-protocol/src/schema.rs
      - echo-sdk-host/src/core_profile/facade/integrations.rs
      - echo-sdk-host/src/core_profile/facade/mod.rs
      - echo-sdk-host/src/core_profile/facade/stream.rs
      - echo-sdk-host/src/core_profile/extension_bridge.rs
      - echo-sdk-host/src/core_profile/handles.rs
    acceptance:
      - batch start 对完整原 handler 绑定先检查后副作用；forward owner/generation/channel
        一致，typed malformed/unknown/u64 等明确失败，close 先取消后等
        permit/Join，quota/publication 失败无 detached owner。
  - id: native-language-consumers
    summary: 三语言原生 ChannelPlugin 真实调用和消费原 handler 的单次、流式与 reply 并执行取消/关闭
    files:
      - sdks/typescript/src
      - sdks/typescript/test
      - sdks/python/src/echo_agent_sdk
      - sdks/python/tests
      - sdks/java/src/main/java/com/echoagent/sdk
      - sdks/java/src/test/java/com/echoagent/sdk
      - echo-sdk-host/tests/extension_bridge_e2e.rs
      - echo-sdk-host/tests/facade_feature_adapters_e2e.rs
    acceptance:
      - 三个语言真实 Host counter/typed callback 证明三项消费，覆盖两 Session/旧
        generation、Unicode/附件/optional/max u64、慢流与 caller/disconnect/Session
        close；内部 counter 不冒充 OS PID 清理。
  - id: accepted-contract-delivery
    summary: 唯一生成链与正式文档/示例表达同一 accepted 契约并以精确交付证据闭合 Finding
    files:
      - echo-sdk-protocol/src/bin/export_schema.rs
      - contracts/sdk
      - sdks/shared
      - scripts/check-sdk-contracts.sh
      - scripts/check-language-sdks.sh
      - docs/sdk
      - docs/adr/0035-channel-handler-forward-consumption.md
      - .echo-semantic/findings/finding.sdk-channel-message-handler-consumer.md
      - .echo-semantic/behaviors/behavior.sdk-family-channels.md
    acceptance:
      - full generator 更新六 operations 和 typed digest 并通过 drift；新 Rust identity
        保持 Host-only 或有 exact approved ADR source choice；文档/examples/website
        检查有明确处置，独立审查和 final snapshot receipts 足以关闭 Finding。
artifact_id: plan:e7290ec3-7160-4124-b5d7-5efc3a53d661
lifecycle: ready
---



## Notes

framework Todo 在当前冻结里程碑交付之后的后继分支执行；在新 worktree channel-handler-forward 准备实现，其路径列出已核对的原文件位置，不授权改写当前冻结快照。后继通用 API 通过全量门禁并发布后，SDK 使用精确 successor revision。该顺序是本单一 consumer 结果的必要条件；本 Plan 不替代父任务其它治理切片。
