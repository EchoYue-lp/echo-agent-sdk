---
schema_version: 1
id: behavior.sdk-language-clients
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
boundary: boundary.sdk-language-clients
observed_at: 56929154c1851473e9e4837901b610cecf001b2d
code_refs:
- sdks/typescript/src/client.ts
- sdks/typescript/src/wire.ts
- sdks/python/src/echo_agent_sdk/client.py
- sdks/python/src/echo_agent_sdk/wire.py
- sdks/java/src/main/java/com/echoagent/sdk/EchoAgentClient.java
- sdks/java/src/main/java/com/echoagent/sdk/WireValues.java
- echo-sdk-protocol/src/scalar.rs
- echo-sdk-protocol/src/error.rs
- contracts/sdk/accepted-external-contract.json
- scripts/check-language-sdks.sh
- sdks/typescript/test/lsp-host.test.js
- sdks/python/tests/test_lsp_diagnostics_host.py
- sdks/java/src/test/java/com/echoagent/sdk/FactoryCleanupLifecycleTest.java
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-family-catalog-method-routing-candidate
- evidence.sdk-ack-recovery-verification
- evidence.sdk-stream-cancellation-routing-verification
- evidence.sdk-java-host-early-exit-verification
- evidence.sdk-official-current-language-gate
- evidence.sdk-official-factory-final-language-gate
- evidence.sdk-be5-official-full-language
finding_refs:
- finding.sdk-ack-submission-consumption
- finding.sdk-inventory-acceptance-authority
- finding.sdk-language-close-retry
- finding.sdk-language-consumption-ack
- finding.sdk-language-error-fidelity
- finding.sdk-negotiation-response-validation
- finding.sdk-wire-data-fidelity
- finding.sdk-family-catalog-method-routing
- finding.sdk-java-original-child-exit-lifecycle
---

# 三语言与无损值

## 重要承诺

本能力的行为、唯一权威、生命周期、失败路径与无损合同必须有可复核证据。

## 当前行为

TS/Python/Java经官方ACP transport消费同一accepted合同；严格value/error投影保留大整数canonical text、Unicode/path/time/bytes、handle/operation/retryable/details。ACK只在已消费或显式处理snapshot/replay coverage后提交；提交失败/取消不写“已成功ACK”，gap不autoACK，迟订阅有界feed与terminal replay自然EOF已有真实场景。close保同一transport/cleanup owner，不伪关闭。LSP原29request及后继42diagnostic共同语料比较结果、完整typed错误和真实请求/通知副作用，均通过。

## 期望行为

保持框架业务权威、Host寻址与资源ownership、三语言无损投影；所有已确认缺陷修复并取得最新反例验证。

## 触发、结果与副作用

源码入口与实际注册/分派路径由本 Map 的 source_refs 定位。输出为 typed response/error、framework committed event 或 generation-fenced handle；所有动态入口仍需逐场景验证。

## 失败、重试与恢复

TS/Python/Java经官方ACP transport消费同一accepted合同；严格value/error投影保留大整数canonical text、Unicode/path/time/bytes、handle/operation/retryable/details。ACK只在已消费或显式处理snapshot/replay coverage后提交；提交失败/取消不写“已成功ACK”，gap不autoACK，迟订阅有界feed与terminal replay自然EOF已有真实场景。close保同一transport/cleanup owner，不伪关闭。LSP原29request及后继42diagnostic共同语料比较结果、完整typed错误和真实请求/通知副作用，均通过。

仍缺：历史fc63完整TS219/Python279+18/Java233已转不可变收据归档；当前6711候选运行原记录为Rust418、TS240/Python300+18/Java264。旧日志不能改称后来源码验证；新Workflow生产变化后整体418/语言门禁不再作为当前完整通过，需其独立后继证据。 全accepted helpers/Host field统一corpus、Windows路径和Task DAG三语言场景仍缺；Diagnostic supporting union/source生成及最终正式pin完整语言/示例/CI须重新绑定最新源码。

## 证据

具名候选运行范围由本组Map关联的child family Behaviors及其 `evidence.sdk-candidate-behavior-verification` 定位；该Evidence只supports具体family，不把其引用冒充本组完整通过。

所有状态保持 needs_review。具名断言的候选通过不证明能力全覆盖：SDK manifest 固定95ec66c6，八crate临时patch组合Task/ACP/LSP源码用于本地验证；最终Channel后继published revision、移除patch后的独立构建、精确main和远端CI仍待。完整Rust inventory仅遥测，最终accepted字段/退役必须由唯一生成器明确裁决。

历史fc63完整TS219/Python279+18/Java233已转不可变收据归档；当前6711候选运行原记录为Rust418、TS240/Python300+18/Java264。旧日志不能改称后来源码验证；新Workflow生产变化后整体418/语言门禁不再作为当前完整通过，需其独立后继证据。 全accepted helpers/Host field统一corpus、Windows路径和Task DAG三语言场景仍缺；Diagnostic supporting union/source生成及最终正式pin完整语言/示例/CI须重新绑定最新源码。

## 裁决记录

用户明确要求完成八项治理，三语言范围只以明确接受合同为准；已授权子智能体执行，主线程负责协调及验收。

## 当前观察、归档和后继边界

原fc63证据已保存为仓库内不可变历史收据，原观察epoch不可恢复为Git时不补造提交。原6711证据归档支持具名局部断言；原27个历史输入中tools.rs已变，具名测试body保持可对照；55个6711运行输入中tools.rs和Workflow已变更。新Workflow语义或其它后继场景必须绑定自己的来源/日志；不把旧整套418/TS240/Python300/Java264冒充最终当前通过。支持见历史审计Evidence及evidence.sdk-parent-driver-candidate-verification的限制，所有needs_review/开放Finding保留。

## 当前模型重观察范围

review_scope：本对象在真实当前候选Git `56929154c1851473e9e4837901b610cecf001b2d`（源码摘要`17cf243e8d6341db0ce23c5b9ee39977ce4ab100d6ed1b17ec5a7bb26c19e138`）上重新观察。上列code_refs/各scenario source_refs已逐Gitblob读取并与当前文件SHA精确相同，原锚点均存在。模型继续描述原accepted入口、框架业务/终态权威、Host owner/generation与transport消费职责、生命周期及具名失败义务；源码定位核对不证明所有行为正确、运行证据fresh或合同生成闭合。

原Behavior/status、Map场景mapped/needs_review、Rule/Asset处置、Finding/未决义务与Evidence原epoch支持范围全部保留。mapped仅入口模型，旧3ea不是本snapshot本体；历史receipt不得因本模型观察更新冒current运行。12source迁移、5秒物理结算与依赖50warning规则三选择仍未接受，完整生成/CI/main另验，任何新Rustwrapper不自动扩三语言合同。
