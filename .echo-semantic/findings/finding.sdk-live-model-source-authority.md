---
schema_version: 1
id: finding.sdk-live-model-source-authority
kind: finding
type: authority_conflict
status: resolved
severity: high
primary_focus: state_authority
focus:
- contract_evidence
- time_lifecycle
boundary_ref: boundary.sdk-integrations
behavior_refs:
- behavior.sdk-family-source-operation
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-integrations-finite-observation
audit_refs: []
decision_refs: []
repair_evidence_refs:
- evidence.sdk-model-read-retained-repair
verification_evidence_refs:
- evidence.sdk-model-read-retained-verification
rereview_audit_refs:
- audit.sdk-model-read-state-rereview
discovered_at: e31e39a5ab29c4b3dc24b2fc349515a2787c6a23
---

# SDK live model 的原配置与 Session 查询未绑定同一权威

## 问题

原 `finding.sdk-live-agent-service-view` 已闭合 Memory/新建 Workflow 两个具体缺陷，不能替本条模型构造与 boxed getter 缺陷作修复证据；最少新建此一条，预算 forwarding 本身正确不重复建 SDK Finding。

## 触发条件与影响

同一真实 Agent/Session 首 Run 完成、accepted set_model(B) 返回成功、第二 Run 完成，实际 loopback HTTP body.model 为 `[memory-model, memory-model]`，未达 `[memory-model, native-next-model]`。原 CLI 选择器 `live model setter` 在 frozen Host `0982121cb1e99284b4a7d7e07af35037f9f22bd653528d9716d425f5609407a0` 实际退出1，原日志1621 bytes、SHA256 `6f34ab2207e7dbb2115114aaefe229257ae41c334f5e2b7addff61131b75d72d`；未注册/替换 client、未建第二 Session，原源码副本/请求/两 completed 状态只留 ignored local。该反例先证明端到端违约，不把所有原因归 SDK。

## 证据

上游 F 原 set_model 仅更新 config 的 producer 违约由 F 自身 Finding 与 `f9906007599a9c9842474a65f4d8cb5a8a5228b8` focused 后继承接；它尚非 main 或 SDK 消费者交付。SDK 的独立适配缺口仍存在：`factory.rs::PreparedAgentDefinition` 只保 FrameworkConfig/Client Arc，build_agent 只 with_llm_client，未保留原 validated LlmConfig 供合法模型重建；`source_operations.rs::dispatch_agent_context_source` 的 Session model_name 读 session_agent 的 ACP boxed wrapper 缓存。F ReactAgent trait 本身读 config，不能把旧 getter A 误归因 trait 优先 LlmClient。

## 处理记录

### 2026-10-19 原记录（历史）

修复必须将默认与 explicit definition 的原 validated LlmConfig 只作为私有构造载荷保留，使用上游既有配置/生成机制；await write 的 Result 原样投射 framework_error，只有成功可返回 Unit。Session getter 沿同一 live AgentHandle 读实际权威，零 Session definition 查询继续是定义；不得从脱敏配置/Client Arc 猜回凭据或增加模型 wire 字段/第二配置权威。Opaque custom client 缺配置必须保留原 typed 错误与状态。

私有凭据不得因新字段进入 Debug/serde/日志。现 PreparedAgentDefinition 仅 derive Clone，无 Debug/Serialize；后继必须复核所有实际新增字段的 type/derive 与显式 snapshot 路径，保持公开 FrameworkConfig 脱敏，不凭这条旧事实预先声称新代码安全。

状态保持 open，修复/验证/复审三链不填成功。真正 after 需公开 F 精确 main/CI 后升级 SDK 原 pin/唯一 generator、当前 Host 同一模型用例 actual HTTP `[A,B]`、Session getter B/definition A、错误不发布及秘密不外泄的相关控制、适用完整门禁和独审；当前 sourceprep/旧 sourcecheck0 不替 after。SDK main 验收独立，承诺不降 deferred。


## 2026-10-20：本地具体闭合

当前状态resolved仅指本Finding的原accepted义务。公开SDK208a消费已接受framework精main356及原唯一生成，私有原cfg/Result/live权威读取和retained producer分别由repair Evidence定位；同原反例的当前Host after、完整Rust14/唯一完整三语入口与3quickstarts、fresh真实锁依赖及精确独审由verification和rereview三链承接。历史before/夹具失败、source/runtime cut及上文当时open记录保留，不降accepted承诺为deferred。

本地闭合不证明SDK精main、适用Linux/Windows CI、nativeHook/Stop或所有未列组合；managed历史Linux退出超时根因未知由其独立Finding保持open。Behavior及全库闭合标志不因本条改变。
