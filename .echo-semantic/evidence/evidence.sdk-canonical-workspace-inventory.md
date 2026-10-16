---
schema_version: 1
id: evidence.sdk-canonical-workspace-inventory
kind: evidence
observed_at: source:b4a8a9ec1021c8c96dd07ffb2618b089a9d888e70a6002e99f0af9133781a088
source_refs:
- echo-sdk-protocol/src/bin/export_schema.rs
supports:
- rule.framework-runtime-authority
limitations:
- This resolver checkpoint does not establish new Host, language, complete contract-check or SDK main acceptance.
- Full Rust inventory remains telemetry; accepted membership is unchanged.
---

# Canonical generator 的 workspace 成员边界

2026-10-16，公开 framework `9886502c83d6b1f6fa9212eb221db0e5874deaee` 的真实 locked metadata 将三个 vendored 第三方包报告为 `source=null`，但它们均不在 `workspace_members`。原筛选误把这些包当作 framework rustdoc 根，full profile 对 calamine 独立 manifest 的 `--locked` 调用因此拒绝创建缺失的锁文件。原 producer 自然退出1；原失败未被归为缺 nightly、资源中断或接受检查通过。

原生产解析路径的持久回归先实际误收 calamine 而失败，再按真实 workspace 成员筛选通过；完整 generator 单元测试为9 passed、0 failed、0 ignored。保留原 tokio-util re-export 特例、可达 feature 集、全部28 profiles 和 `--locked`，没有修改 vendor、补造锁文件或扩大 accepted 范围。独立 reviewer 对该源码摘要返回 pass，必要复杂度结论为复用原入口与一个纯 metadata 解析 helper，无第二生成器。

随后唯一完整 `export_schema --update` 实际退出0、无信号、无截断，写出117项既有规范产物。accepted 5,621项无新增/退役，只有已批准B影响的 AgentTurnDriver::drive、AgentEvent::from_error 两项及其 route 副本的签名摘要更新；protected-claim schema 变化由已批准A单独约束。本 checkpoint 仅提交解析器修复及本薄证据，生成物与A/B适配留在同一任务中继续验证，避免把尚未验证的Host改动混入局部结论。

原件只位于 ignored `.echo-semantic/local/sdk-cancellation-observation/published-988-*` 及 `.echo-semantic/local/sdk-parallel-delivery-01a14385/`：实际 metadata、before/after、完整生成 runner、资源记录、accepted leaf diff 与独审摘要。原失败日志 SHA256为 `c1462137d98722ce35026854a1b67974cded3f943c4aab80de8345e98743991b`；本轮完整生成日志 SHA256为 `78f11740d83427fb32f16a1b88581d45b6918b0462502f7b0cb8b8ee4c561470`。

文档同步仅需说明本内部生成器边界；语言示例、产品API和官网使用方式没有变化，因此本局部无需修改 examples 或官网。SDK整体A/B文档、三语言、新Host与精确main CI仍由后续实际结果分别验收。
