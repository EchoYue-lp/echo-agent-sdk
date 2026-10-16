---
schema_version: 1
id: evidence.sdk-workflow-protected-claim-human-decision
kind: evidence
observed_at: 335193791c70a6cb077033c3f8f0cb1223e5266e
source_refs:
- docs/sdk/governance/workflow-protected-claim-proposal.md
- docs/adr/0039-sdk-cancellation-observation.md
- docs/supreme/specs/sdk-cancellation-observation/design.md
- docs/supreme/specs/sdk-cancellation-observation/plans/plan_01_保留原执行与无损观察.md
- sdks/typescript/test/typed-bridge.test.js
- sdks/python/tests/test_lifecycle.py
- sdks/java/src/test/java/com/echoagent/sdk/TypedExtensionTest.java
supports:
- behavior.sdk-family-workflow
- rule.framework-runtime-authority
limitations:
- Human accepted the exact reviewed A contract; no implementation, Rust, Host or cross-language after result is asserted here.
- Framework remains sole Graph and CheckpointStore authority; SDK direct-run observation projection has its own independent open Finding.
- Original proposal SHA23c5a59 full text is preserved in the same current decision document, without rewriting historical Proposed wording.
- Protected owner loss can block indefinitely; Memory protection is instance-local and old binaries must not access protected data.
---

# Workflow protected claim 的人的明确 A 裁决

## 支持的结论

用户于2026-10-16明确选择A“保留能力并实施保护合同（推荐）”，由根任务实际答复转交本SDK切片。选择绑定原独审提案 SHA256 `23c5a59ae405ecd71c3e79b6d945e77e8b7927e0e1f0176ad91047147341a09f`，7,288 B，独立设计审C0/I0及最终排版/File限制轻量确认。不再等待同一A/B答复，也不把推荐或预选当授权来源。

接受增量严格只有原CheckpointStore的effect前exact-attempt保护承诺、`protected_claim_v1`、既有component调用内`workflow_checkpoint_protect_claim({checkpoint_id,attempt_id}) -> Unit`及受保护CRUD拒绝；Graph能力查询只Rust/Host、绑定原Store，不增语言query/receipt/unlock/API镜像。缺cap仍可Store注册/查询/无关操作；所有可能Node执行的claim-backed Approved/Modified resume及branch在claim、forksave、effect前canonical feature_unavailable/Never。Deferred、Rejected无Node和fresh执行保明确原范围。

## 来源与范围

同页保留完整原Proposed提案，顶部单独记录本次Accepted A；ADR0039和唯一Design/Plan承接实施。原Memory/File/remote Store拥有durability及保护，SDK只转换委托。用户接受protected owner丢失甚至protect成功但effect尚未启动时可能长期blocked；未授权第二store/scheduler、自动重执行、强制解锁或receipt查询。旧File自动恢复/读改写failclosed不意味着旧delete/clear安全，因此旧binary禁止接触保护数据；Memory不被提升为跨restart持久存储。

## 已知缺口

header仅绑定精确候选中上述决策文稿的当前可恢复引用，不改变原人的答复、原23c5提案内容或下述阶段的未执行范围。

当前三个typed consumer源码承接该单cap/单reverse及默认false形状；候选纯回归的实际结果单独记录，不把本人的选择证据变成完整Host执行证明。Python旧descriptor golden遗漏新默认false已由同文件真实35pass/1fail定位并补齐，候选同scope实际36pass；未为通过而删除字段或降断言。

此为人的受保护期望，不是把当前bug改成risk_accepted，也不关闭任何Finding。协议与三语必须先保真实before、再实现单项DTO/cap/dispatcher，framework原Graph/Store保护与CRUD/heartbeat/restart由其唯一writer实现。新的公开精确pin、唯一28profile生成、实际Host/三语言、适用门禁和独立复审仍待。SDK direct-run stringify具体Finding仍独立等待新pin真实red后修复；原局部已绿、源码审或用户答复不代这些证据。
