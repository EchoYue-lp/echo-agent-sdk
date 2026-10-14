---
schema_version: 1
id: behavior.sdk-integrations
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
boundary: boundary.sdk-integrations
observed_at: bd8e42e732588181b7a2589d91e9ffdce02d23e1
code_refs:
- echo-sdk-host/src/core_profile/facade/mod.rs
- echo-sdk-host/src/core_profile/facade/registry.rs
- echo-sdk-host/src/core_profile/facade/source_operations.rs
- echo-sdk-host/src/core_profile/facade/integrations.rs
- echo-sdk-host/src/core_profile/facade/memory.rs
- echo-sdk-host/src/core_profile/facade/workflow.rs
- echo-sdk-host/src/core_profile/facade/state_delivery.rs
- echo-sdk-host/src/core_profile/facade/tools.rs
- echo-sdk-host/src/core_profile/facade/observability.rs
- echo-sdk-protocol/src/managed.rs
- echo-sdk-protocol/src/task_store.rs
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-channel-message-handler-consumer-audit
- evidence.sdk-tool-input-validation-candidate
- evidence.sdk-mcp-channel-owned-candidate
- evidence.sdk-channel-resource-admission
- evidence.sdk-integrations-finite-observation
finding_refs:
- finding.sdk-adapter-invalid-input
- finding.sdk-delivery-exact-claim
- finding.sdk-family-handle-argument-projection
- finding.sdk-feature-failure-cause-fidelity
- finding.sdk-live-agent-service-view
- finding.sdk-lsp-response-fidelity
- finding.sdk-mcp-batch-publication
- finding.sdk-mcp-manager-owner
- finding.sdk-trace-publication-quota
- finding.sdk-channel-message-handler-consumer
- finding.sdk-tool-input-validator-bypass
---

# 集成能力与精确 source invoke

## 重要承诺

本能力的行为、唯一权威、生命周期、失败路径与无损合同必须有可复核证据。

## 当前行为

CompiledOperationCatalog按确切accepted identity/signature/feature寻址，薄adapter调用live framework service。memory/workflow读取当前Agent服务，mixednamespace/配置错误在副作用前拒绝；Delivery携完整claim而不以id重建当前attempt；trace构造验证后发布，quota失败可复用。MCP config/file批量SDKalias原子发布保原best-effort native连接权威及exact Arc+Session；工具投影完整ToolResult/effects/artifact/canonical cause。LSP真实29request场景+manager生命周期有30观测/44原始请求；新诊断另42观测/42原始通知跨三语言全字段匹配，severity缺失与integer/text code无损、每URI错误/恢复/清空隔离、无URI走原runtime fatal/PID退出。两份证据各绑定fc63/4d43不同不可变Host，不互冒同一final输入。

## 期望行为

保持框架业务权威、Host寻址与资源ownership、三语言无损投影；所有已确认缺陷修复并取得最新反例验证。

## 触发、结果与副作用

源码入口与实际注册/分派路径由本 Map 的 source_refs 定位。输出为 typed response/error、framework committed event 或 generation-fenced handle；所有动态入口仍需逐场景验证。

## 失败、重试与恢复

CompiledOperationCatalog按确切accepted identity/signature/feature寻址，薄adapter调用live framework service。memory/workflow读取当前Agent服务，mixednamespace/配置错误在副作用前拒绝；Delivery携完整claim而不以id重建当前attempt；trace构造验证后发布，quota失败可复用。MCP config/file批量SDKalias原子发布保原best-effort native连接权威及exact Arc+Session；工具投影完整ToolResult/effects/artifact/canonical cause。LSP真实29request场景+manager生命周期有30观测/44原始请求；新诊断另42观测/42原始通知跨三语言全字段匹配，severity缺失与integer/text code无损、每URI错误/恢复/清空隔离、无URI走原runtime fatal/PID退出。两份证据各绑定fc63/4d43不同不可变Host，不互冒同一final输入。

原阶段缺口（历史，本次finite范围见下）：41个accepted family仍逐项needs_review；旧40项候选Evidence保持其原运行epoch。新工具Evidence仅支持28次tool调用+1Session close、10RPC error与单独database拒写的具名场景；testing新Evidence仅支持14个field payload的纯值投影，Javacontrolled official transport不属于Mock runtime。所有未列场景、provider分页/取消、Windows路径及正式pin/main仍待。

## 证据

具名候选范围由child family Behaviors及对应Evidence定位；旧 `evidence.sdk-candidate-behavior-verification` 保原实测epoch，新 `evidence.sdk-tool-behavior-candidate` 与 `evidence.sdk-testing-value-candidate` 仅补工具局部场景与testing纯值合同。它们只supports具名family，不把其引用冒充本组完整通过。

所有状态保持 needs_review。具名断言的候选通过不证明能力全覆盖：SDK manifest 固定95ec66c6，八crate临时patch组合Task/ACP/LSP源码用于本地验证；最终Channel后继published revision、移除patch后的独立构建、精确main和远端CI仍待。完整Rust inventory仅遥测，最终accepted字段/退役必须由唯一生成器明确裁决。

41个accepted family仍逐项needs_review；旧40项候选Evidence保持其原运行epoch。新工具Evidence仅支持28次tool调用+1Session close、10RPC error与单独database拒写的具名场景；testing新Evidence仅支持14个field payload的纯值投影，Javacontrolled official transport不属于Mock runtime。所有未列场景、provider分页/取消、Windows路径及正式pin/main仍待。

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

## 原代码证据出处

sdks/typescript/test/lsp-diagnostics-host.test.js, sdks/python/tests/test_lsp_diagnostics_host.py, sdks/java/src/test/java/com/echoagent/sdk/LspDiagnosticsHostTest.java 保原569/各已保存Evidence指针与义务；不把未在bd8的辅助测试源fake当前source_refs。

## 当前集成能力与supporting角色有界收口

原frameworkFinal独立source审查基于790/a634真实runtime，后A2A第一次missing具名consumer另由bd8三native24step/9ops/8HTTP补齐并审PASS。这是两个真实epoch，不把原A2Apendingreview文稿当今天未审，也不伪当时已审。九所选runtime69b在bd8逐Gitblob相同；currenthandles新A变动排除，旧source审不借成latestA或全部Host已验。

具体本family和registry/skill支持角色按sharedEvidence表/8Audit定位行为/唯一authority/lifecycle/失败：MCP/LSP owner/PID/全字段/derived不复活；Channel六forward原2facts/RWclose/peer隔离；EvalImprove cleanup/deadline/workspace；topologynodeedge；Permissionlive/TTL另scope；Statecheckpoint7、TraceRun17/nanotime/bigDuration/JSONLcold；Deliverymemory8ops同实例exactclaim；Testing14纯值controltransport非Mock业务。源码/API/模型不成为第二权威。

Behavior保持needs_review；入口具名mapped/examined不自动41verified、fullaccepted生成/main。Delivery无额外crossprocessdurability，configuredredaction不承诺未知所有secret；registry/skill归既有source_operation/intrinsic，无新family或任意skill执行。newAfinal、5秒physicalB/12identity及平台/main另验，支持范围未审的sourceoperationmethod保具体菜单理由，不以无限所有组合扩大义务。
