---
schema_version: 1
id: behavior.sdk-extension-bridge
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
boundary: boundary.sdk-extension-bridge
observed_at: 5bcbad2437e5c19f61101883044c4f67944cb97c
code_refs:
- echo-sdk-host/src/core_profile/extension_bridge.rs
- echo-sdk-host/src/core_profile/handler.rs
- echo-sdk-host/src/factory.rs
- echo-sdk-protocol/src/methods.rs
- echo-sdk-host/tests/extension_bridge_e2e.rs#managed_commit_disconnect_restart_replays_same_receipt
- sdks/shared/testing/managed_persistence_backend.py
- sdks/java/src/test/java/com/echoagent/sdk/FactoryCleanupLifecycleTest.java
- sdks/java/src/test/java/com/echoagent/sdk/LateFactoryHostTest.java
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-channel-message-handler-consumer-audit
- evidence.sdk-managed-retirement
- evidence.sdk-llm-workflow-stream
- evidence.sdk-python-typed-component-stream-verification
- evidence.sdk-managed-supporting-operation-boundary
- evidence.sdk-extension-teardown-prefix-verification
- evidence.sdk-tools-bridge-finite-observation
finding_refs:
- finding.sdk-extension-pending-stream-owner
- finding.sdk-factory-instance-routing
- finding.sdk-managed-store-adaptation
- finding.sdk-sandbox-cleanup-order
- finding.sdk-channel-message-handler-consumer
---

# 扩展桥

## 重要承诺

本能力的行为、唯一权威、生命周期、失败路径与无损合同必须有可复核证据。

## 当前行为

extension register/unregister经descriptor/capability准入，反向call复用官方connection lease/deadline；Tool/LLM/Store/hook/callback由framework消费并决定terminal/commit。initial response前RAII stream reservation避免超时泄漏；cleanup/interrupted/terminal callback使用connection作用域，不因Run取消丢清理。Factory先reserve官方child handle及factory_origin，再publish/rollback，同Factory可创建独立Session实例。managed11 typed DTO/TaskStore execution-CAS显式opt-in，真实deadline-before-work、lost ACK/断连重启同receipt保留commit权威。Bridge34候选通过，只证明具名调用；Java2bad cleanup/failed-future owner有自身264项与独立复审记录，但当前能力未列场景仍不算闭合。

## 期望行为

保持框架业务权威、Host寻址与资源ownership、三语言无损投影；所有已确认缺陷修复并取得最新反例验证。

## 触发、结果与副作用

源码入口与实际注册/分派路径由本 Map 的 source_refs 定位。输出为 typed response/error、framework committed event 或 generation-fenced handle；所有动态入口仍需逐场景验证。

## 失败、重试与恢复

extension register/unregister经descriptor/capability准入，反向call复用官方connection lease/deadline；Tool/LLM/Store/hook/callback由framework消费并决定terminal/commit。initial response前RAII stream reservation避免超时泄漏；cleanup/interrupted/terminal callback使用connection作用域，不因Run取消丢清理。Factory先reserve官方child handle及factory_origin，再publish/rollback，同Factory可创建独立Session实例。managed11 typed DTO/TaskStore execution-CAS显式opt-in，真实deadline-before-work、lost ACK/断连重启同receipt保留commit权威。Bridge34候选通过，只证明具名调用；Java2bad cleanup/failed-future owner有自身264项与独立复审记录，但当前能力未列场景仍不算闭合。

仍缺：每个已接受component operation的合法值、取消、callback reentry、撤销/连接中断后结算尚未逐项完整覆盖；11DTO不等于11真实后端操作。历史fc63完整TS219/Python279+18/Java233已转不可变收据归档；当前6711候选运行原记录为Rust418、TS240/Python300+18/Java264。旧日志不能改称后来源码验证；新Workflow生产变化后整体418/语言门禁不再作为当前完整通过，需其独立后继证据。

## 证据

具名候选运行范围由本组Map关联的child family Behaviors及其 `evidence.sdk-candidate-behavior-verification` 定位；该Evidence只supports具体family，不把其引用冒充本组完整通过。

所有状态保持 needs_review。具名断言的候选通过不证明能力全覆盖：SDK manifest 固定95ec66c6，八crate临时patch组合Task/ACP/LSP源码用于本地验证；最终Channel后继published revision、移除patch后的独立构建、精确main和远端CI仍待。完整Rust inventory仅遥测，最终accepted字段/退役必须由唯一生成器明确裁决。

每个已接受component operation的合法值、取消、callback reentry、撤销/连接中断后结算尚未逐项完整覆盖；11DTO不等于11真实后端操作。历史fc63完整TS219/Python279+18/Java233已转不可变收据归档；当前6711候选运行原记录为Rust418、TS240/Python300+18/Java264。旧日志不能改称后来源码验证；新Workflow生产变化后整体418/语言门禁不再作为当前完整通过，需其独立后继证据。

## 裁决记录

用户明确要求完成八项治理，三语言范围只以明确接受合同为准；已授权子智能体执行，主线程负责协调及验收。

## Channel inbound consumer obligation

Accepted MessageHandler handle/handle_stream/reply has a current static consumer
gap. Registration/start/send/stop receipts do not close it; no unaccepted receive
wire is invented. See finding.sdk-channel-message-handler-consumer and its source
audit Evidence.

## 当前观察、归档和后继边界

原fc63证据已保存为仓库内不可变历史收据，原观察epoch不可恢复为Git时不补造提交。原6711证据归档支持具名局部断言；原27个历史输入中tools.rs已变，具名测试body保持可对照；55个6711运行输入中tools.rs和Workflow已变更。新Workflow语义或其它后继场景必须绑定自己的来源/日志；不把旧整套418/TS240/Python300/Java264冒充最终当前通过。支持见历史审计Evidence及evidence.sdk-parent-driver-candidate-verification的限制，所有needs_review/开放Finding保留。

## 当前模型重观察范围

review_scope：本对象在真实当前候选Git `56929154c1851473e9e4837901b610cecf001b2d`（源码摘要`17cf243e8d6341db0ce23c5b9ee39977ce4ab100d6ed1b17ec5a7bb26c19e138`）上重新观察。上列code_refs/各scenario source_refs已逐Gitblob读取并与当前文件SHA精确相同，原锚点均存在。模型继续描述原accepted入口、框架业务/终态权威、Host owner/generation与transport消费职责、生命周期及具名失败义务；源码定位核对不证明所有行为正确、运行证据fresh或合同生成闭合。

原Behavior/status、Map场景mapped/needs_review、Rule/Asset处置、Finding/未决义务与Evidence原epoch支持范围全部保留。mapped仅入口模型，旧3ea不是本snapshot本体；历史receipt不得因本模型观察更新冒current运行。12source迁移、5秒物理结算与依赖50warning规则三选择仍未接受，完整生成/CI/main另验，任何新Rustwrapper不自动扩三语言合同。

## 当前reverse角色有限审查

Tool/LLM/Store/Hook/AgentCallback/Intervention六角色依原currentTool8lens实际业务审查回收，不新bridgeprotocol。5bcb runtime69b源码同；Tool/LLMremote原因合法3红绿与原独审精确收口，Feature宽Finding其它scope仍open。此Behavior needs_review，原5sB/managedsupporting/noForwardAPI/main等义务保，未接受方法不强造API。
