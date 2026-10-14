---
schema_version: 1
id: map.sdk-language-clients
kind: capability_map
title: 三语言与无损值
risk: high
observed_at: 76601b242d7c354352f3b5c7d4bf099873746b14
boundary_refs:
- boundary.sdk-language-clients
behavior_refs:
- behavior.sdk-language-clients
- behavior.sdk-family-value
- behavior.sdk-family-intrinsic
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-language-client-lifecycle-current
- evidence.sdk-typescript-stdio-close-specific-closure
- evidence.sdk-java-stdio-close-specific-closure
- evidence.sdk-family-catalog-method-routing-candidate
- evidence.sdk-ack-recovery-verification
- evidence.sdk-stream-cancellation-routing-verification
- evidence.sdk-java-host-early-exit-verification
- evidence.sdk-official-current-language-gate
- evidence.sdk-official-factory-final-language-gate
- evidence.sdk-protocol-finite-observation
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
audit_refs:
- audit.sdk-language-client-lifecycle-trigger-input
- audit.sdk-language-client-lifecycle-result-side-effect
- audit.sdk-language-client-lifecycle-state-authority
- audit.sdk-language-client-lifecycle-data-durability
- audit.sdk-language-client-lifecycle-time-lifecycle
- audit.sdk-language-client-lifecycle-failure-concurrency
- audit.sdk-language-client-lifecycle-permission-external
- audit.sdk-language-client-lifecycle-contract-evidence
- audit.sdk-typescript-stdio-close-specific-rereview
- audit.sdk-java-stdio-close-specific-rereview
- audit.sdk-ack-recovery-rereview
- audit.sdk-java-host-early-exit-rereview
- audit.sdk-protocol-finite-trigger-input
- audit.sdk-protocol-finite-result-side-effect
- audit.sdk-protocol-finite-state-authority
- audit.sdk-protocol-finite-data-durability
- audit.sdk-protocol-finite-time-lifecycle
- audit.sdk-protocol-finite-failure-concurrency
- audit.sdk-protocol-finite-permission-external
- audit.sdk-protocol-finite-contract-evidence
related_map_refs:
- map.sdk-product-boundary
scenarios:
  client-lifecycle-trigger-input:
    status: mapped
    source_refs:
    - sdks/typescript/src/client.ts
    - sdks/python/src/echo_agent_sdk/client.py
    - sdks/python/src/echo_agent_sdk/execution_usage_values.py
    - sdks/java/src/main/java/com/echoagent/sdk/EchoAgentClient.java
    behavior_refs:
    - behavior.sdk-language-clients
    rule_refs:
    - rule.framework-runtime-authority
    evidence_refs:
    - evidence.sdk-language-client-lifecycle-current
    audit_refs:
    - audit.sdk-language-client-lifecycle-trigger-input
    review_scope: Named trigger_input source obligations actually examined, C0/I0/M0;
      mapped is not all-family verified or SDK main acceptance.
  client-lifecycle-result-side-effect:
    status: mapped
    source_refs:
    - sdks/typescript/src/client.ts
    - sdks/python/src/echo_agent_sdk/client.py
    - sdks/python/src/echo_agent_sdk/execution_usage_values.py
    - sdks/java/src/main/java/com/echoagent/sdk/EchoAgentClient.java
    behavior_refs:
    - behavior.sdk-language-clients
    rule_refs:
    - rule.framework-runtime-authority
    evidence_refs:
    - evidence.sdk-language-client-lifecycle-current
    audit_refs:
    - audit.sdk-language-client-lifecycle-result-side-effect
    review_scope: Named result_side_effect source obligations actually examined, C0/I0/M0;
      mapped is not all-family verified or SDK main acceptance.
  client-lifecycle-state-authority:
    status: mapped
    source_refs:
    - sdks/typescript/src/client.ts
    - sdks/python/src/echo_agent_sdk/client.py
    - sdks/python/src/echo_agent_sdk/execution_usage_values.py
    - sdks/java/src/main/java/com/echoagent/sdk/EchoAgentClient.java
    behavior_refs:
    - behavior.sdk-language-clients
    rule_refs:
    - rule.framework-runtime-authority
    evidence_refs:
    - evidence.sdk-language-client-lifecycle-current
    audit_refs:
    - audit.sdk-language-client-lifecycle-state-authority
    review_scope: Named state_authority source obligations actually examined, C0/I0/M0;
      mapped is not all-family verified or SDK main acceptance.
  client-lifecycle-data-durability:
    status: mapped
    source_refs:
    - sdks/typescript/src/client.ts
    - sdks/python/src/echo_agent_sdk/client.py
    - sdks/python/src/echo_agent_sdk/execution_usage_values.py
    - sdks/java/src/main/java/com/echoagent/sdk/EchoAgentClient.java
    behavior_refs:
    - behavior.sdk-language-clients
    rule_refs:
    - rule.framework-runtime-authority
    evidence_refs:
    - evidence.sdk-language-client-lifecycle-current
    audit_refs:
    - audit.sdk-language-client-lifecycle-data-durability
    review_scope: Named data_durability source obligations actually examined, C0/I0/M0;
      mapped is not all-family verified or SDK main acceptance.
  client-lifecycle-time-lifecycle:
    status: mapped
    source_refs:
    - sdks/typescript/src/client.ts
    - sdks/python/src/echo_agent_sdk/client.py
    - sdks/python/src/echo_agent_sdk/execution_usage_values.py
    - sdks/java/src/main/java/com/echoagent/sdk/EchoAgentClient.java
    behavior_refs:
    - behavior.sdk-language-clients
    rule_refs:
    - rule.framework-runtime-authority
    evidence_refs:
    - evidence.sdk-language-client-lifecycle-current
    audit_refs:
    - audit.sdk-language-client-lifecycle-time-lifecycle
    review_scope: Named time_lifecycle source obligations actually examined, C0/I0/M0;
      mapped is not all-family verified or SDK main acceptance.
  client-lifecycle-failure-concurrency:
    status: mapped
    source_refs:
    - sdks/typescript/src/client.ts
    - sdks/python/src/echo_agent_sdk/client.py
    - sdks/python/src/echo_agent_sdk/execution_usage_values.py
    - sdks/java/src/main/java/com/echoagent/sdk/EchoAgentClient.java
    behavior_refs:
    - behavior.sdk-language-clients
    rule_refs:
    - rule.framework-runtime-authority
    evidence_refs:
    - evidence.sdk-language-client-lifecycle-current
    audit_refs:
    - audit.sdk-language-client-lifecycle-failure-concurrency
    review_scope: Named failure_concurrency source obligations actually examined,
      C0/I0/M0; mapped is not all-family verified or SDK main acceptance.
  client-lifecycle-permission-external:
    status: mapped
    source_refs:
    - sdks/typescript/src/client.ts
    - sdks/python/src/echo_agent_sdk/client.py
    - sdks/python/src/echo_agent_sdk/execution_usage_values.py
    - sdks/java/src/main/java/com/echoagent/sdk/EchoAgentClient.java
    behavior_refs:
    - behavior.sdk-language-clients
    rule_refs:
    - rule.framework-runtime-authority
    evidence_refs:
    - evidence.sdk-language-client-lifecycle-current
    audit_refs:
    - audit.sdk-language-client-lifecycle-permission-external
    review_scope: Named permission_external source obligations actually examined,
      C0/I0/M0; mapped is not all-family verified or SDK main acceptance.
  client-lifecycle-contract-evidence:
    status: mapped
    source_refs:
    - sdks/typescript/src/client.ts
    - sdks/python/src/echo_agent_sdk/client.py
    - sdks/python/src/echo_agent_sdk/execution_usage_values.py
    - sdks/java/src/main/java/com/echoagent/sdk/EchoAgentClient.java
    behavior_refs:
    - behavior.sdk-language-clients
    rule_refs:
    - rule.framework-runtime-authority
    evidence_refs:
    - evidence.sdk-language-client-lifecycle-current
    audit_refs:
    - audit.sdk-language-client-lifecycle-contract-evidence
    review_scope: Named contract_evidence source obligations actually examined, C0/I0/M0;
      mapped is not all-family verified or SDK main acceptance.
  behavior-lifecycle-failure:
    status: mapped
    source_refs:
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
    - sdks/java/src/test/java/com/echoagent/sdk/FactoryCleanupLifecycleTest.java
    behavior_refs:
    - behavior.sdk-language-clients
    finding_refs:
    - finding.sdk-ack-submission-consumption
    - finding.sdk-inventory-acceptance-authority
    - finding.sdk-language-close-retry
    - finding.sdk-language-consumption-ack
    - finding.sdk-language-error-fidelity
    - finding.sdk-negotiation-response-validation
    - finding.sdk-wire-data-fidelity
    review_scope: Entry, unique authority, normal lifecycle and named failure obligations
      have been inspected and linked. Mapped records the model, not bug-free/runtime/main
      closure; open Findings remain explicit.
  family-value:
    status: mapped
    source_refs:
    - echo-sdk-protocol/src/scalar.rs
    behavior_refs:
    - behavior.sdk-family-value
    evidence_refs:
    - evidence.sdk-candidate-behavior-verification
    - evidence.sdk-parent-driver-candidate-verification
    - evidence.sdk-protocol-finite-observation
    review_scope: Closed wire/typedvalue namedlossless and invalid inputs examined;Windowsnative/fullacceptedidentity12gen
      boundary remains, noAllRustinventory promise.
  family-intrinsic:
    status: needs_review
    source_refs:
    - sdks/typescript/src/client.ts
    - sdks/typescript/src/wire.ts
    - sdks/python/src/echo_agent_sdk/client.py
    - sdks/python/src/echo_agent_sdk/wire.py
    - sdks/java/src/main/java/com/echoagent/sdk/EchoAgentClient.java
    - sdks/java/src/main/java/com/echoagent/sdk/WireValues.java
    - echo-sdk-protocol/src/scalar.rs
    - echo-sdk-protocol/src/error.rs
    - contracts/sdk/accepted-external-contract.json
    behavior_refs:
    - behavior.sdk-family-intrinsic
    unknown: 该family已有具名候选断言；完整最低充分行为及最终published pin/source、独立复审仍未闭合。testing仅声明库存无运行supports。
    next_step: 沿关联family Behavior的最低充分证据补未列场景；最终源码/二进制/合同冻结后独立复审及精确main对账，不重跑或抹除已有反例。
    evidence_refs:
    - evidence.sdk-candidate-behavior-verification
    - evidence.sdk-parent-driver-candidate-verification
  protocol-finite-trigger-input:
    status: mapped
    source_refs: &id001
    - echo-sdk-protocol/src/capability.rs
    - echo-sdk-protocol/src/scalar.rs
    - echo-sdk-protocol/src/error.rs
    behavior_refs:
    - behavior.sdk-acp
    - behavior.sdk-family-standard
    - behavior.sdk-family-value
    rule_refs:
    - rule.framework-runtime-authority
    evidence_refs:
    - evidence.sdk-protocol-finite-observation
    audit_refs:
    - audit.sdk-protocol-finite-trigger-input
    review_scope: Existing mapkernel observation preserved; successor622d validator/initowner
      source and proofs in exactlinkedEvidence/Audit; old782whole historical;Windows/B12/gen/main
      explicit.
  protocol-finite-result-side-effect:
    status: mapped
    source_refs: *id001
    behavior_refs:
    - behavior.sdk-acp
    - behavior.sdk-family-standard
    - behavior.sdk-family-value
    rule_refs:
    - rule.framework-runtime-authority
    evidence_refs:
    - evidence.sdk-protocol-finite-observation
    audit_refs:
    - audit.sdk-protocol-finite-result-side-effect
    review_scope: Existing mapkernel observation preserved; successor622d validator/initowner
      source and proofs in exactlinkedEvidence/Audit; old782whole historical;Windows/B12/gen/main
      explicit.
  protocol-finite-state-authority:
    status: mapped
    source_refs: *id001
    behavior_refs:
    - behavior.sdk-acp
    - behavior.sdk-family-standard
    - behavior.sdk-family-value
    rule_refs:
    - rule.framework-runtime-authority
    evidence_refs:
    - evidence.sdk-protocol-finite-observation
    audit_refs:
    - audit.sdk-protocol-finite-state-authority
    review_scope: Existing mapkernel observation preserved; successor622d validator/initowner
      source and proofs in exactlinkedEvidence/Audit; old782whole historical;Windows/B12/gen/main
      explicit.
  protocol-finite-data-durability:
    status: mapped
    source_refs: *id001
    behavior_refs:
    - behavior.sdk-acp
    - behavior.sdk-family-standard
    - behavior.sdk-family-value
    rule_refs:
    - rule.framework-runtime-authority
    evidence_refs:
    - evidence.sdk-protocol-finite-observation
    audit_refs:
    - audit.sdk-protocol-finite-data-durability
    review_scope: Existing mapkernel observation preserved; successor622d validator/initowner
      source and proofs in exactlinkedEvidence/Audit; old782whole historical;Windows/B12/gen/main
      explicit.
  protocol-finite-time-lifecycle:
    status: mapped
    source_refs: *id001
    behavior_refs:
    - behavior.sdk-acp
    - behavior.sdk-family-standard
    - behavior.sdk-family-value
    rule_refs:
    - rule.framework-runtime-authority
    evidence_refs:
    - evidence.sdk-protocol-finite-observation
    audit_refs:
    - audit.sdk-protocol-finite-time-lifecycle
    review_scope: Existing mapkernel observation preserved; successor622d validator/initowner
      source and proofs in exactlinkedEvidence/Audit; old782whole historical;Windows/B12/gen/main
      explicit.
  protocol-finite-failure-concurrency:
    status: mapped
    source_refs: *id001
    behavior_refs:
    - behavior.sdk-acp
    - behavior.sdk-family-standard
    - behavior.sdk-family-value
    rule_refs:
    - rule.framework-runtime-authority
    evidence_refs:
    - evidence.sdk-protocol-finite-observation
    audit_refs:
    - audit.sdk-protocol-finite-failure-concurrency
    review_scope: Existing mapkernel observation preserved; successor622d validator/initowner
      source and proofs in exactlinkedEvidence/Audit; old782whole historical;Windows/B12/gen/main
      explicit.
  protocol-finite-permission-external:
    status: mapped
    source_refs: *id001
    behavior_refs:
    - behavior.sdk-acp
    - behavior.sdk-family-standard
    - behavior.sdk-family-value
    rule_refs:
    - rule.framework-runtime-authority
    evidence_refs:
    - evidence.sdk-protocol-finite-observation
    audit_refs:
    - audit.sdk-protocol-finite-permission-external
    review_scope: Existing mapkernel observation preserved; successor622d validator/initowner
      source and proofs in exactlinkedEvidence/Audit; old782whole historical;Windows/B12/gen/main
      explicit.
  protocol-finite-contract-evidence:
    status: mapped
    source_refs: *id001
    behavior_refs:
    - behavior.sdk-acp
    - behavior.sdk-family-standard
    - behavior.sdk-family-value
    rule_refs:
    - rule.framework-runtime-authority
    evidence_refs:
    - evidence.sdk-protocol-finite-observation
    audit_refs:
    - audit.sdk-protocol-finite-contract-evidence
    review_scope: Existing mapkernel observation preserved; successor622d validator/initowner
      source and proofs in exactlinkedEvidence/Audit; old782whole historical;Windows/B12/gen/main
      explicit.
---

# 三语言与无损值

## 能力范围

TS/Python/Java经官方ACP transport消费同一accepted合同；严格value/error投影保留大整数canonical text、Unicode/path/time/bytes、handle/operation/retryable/details。ACK只在已消费或显式处理snapshot/replay coverage后提交；提交失败/取消不写“已成功ACK”，gap不autoACK，迟订阅有界feed与terminal replay自然EOF已有真实场景。close保同一transport/cleanup owner，不伪关闭。LSP原29request及后继42diagnostic共同语料比较结果、完整typed错误和真实请求/通知副作用，均通过。

## 入口与输出

源码入口与实际注册/分派路径由本 Map 的 source_refs 定位。输出为 typed response/error、framework committed event 或 generation-fenced handle；所有动态入口仍需逐场景验证。

## 行为关系

本能力经 official ACP/profile 或具体 catalog route 到 Host adapter，然后调用框架权威。跨能力调用与三语言消费见 docs/sdk/governance/capability-map.md。

## 状态与数据流

Framework 拥有 Run/Task/Store 的事实与终态；Host 只拥有寻址、可取消资源 owner 与协议发送状态；客户端确认已消费事实。

## 策略来源与优先级

用户八项治理目标与 AGENTS.md 约束行为；accepted external contract 约束兼容；compiled feature/canonical framework service 约束实现。完整 Rust inventory 不扩承诺。

## 生命周期与失败路径

TS/Python/Java经官方ACP transport消费同一accepted合同；严格value/error投影保留大整数canonical text、Unicode/path/time/bytes、handle/operation/retryable/details。ACK只在已消费或显式处理snapshot/replay coverage后提交；提交失败/取消不写“已成功ACK”，gap不autoACK，迟订阅有界feed与terminal replay自然EOF已有真实场景。close保同一transport/cleanup owner，不伪关闭。LSP原29request及后继42diagnostic共同语料比较结果、完整typed错误和真实请求/通知副作用，均通过。

原569阶段缺口（历史，已由当前具名client审查收窄）：native值/错误/ACK/关闭使用实际consumer与对应官方transport证据；新retention/pending ACK窗口须匹配修复后源。最终三语言完整门禁、精确生成签名和发布pin另验，不以旧whole语言counts替当前源。

## 权限与敏感信息

本地 SDK 不引入线上权限门控。继续保持框架自动行为 permission authority，凭据不进入日志/合同/错误；资源 gate 仅防止错误owner和无意数据丢失。

## 用户侧投影

TypeScript/Python/Java 对已接受能力暴露惯用接口，handle/error/value 必须无损；不能用成功默认值或丢字段改变权威事实。

## 场景处置清单

主场景mapped只表示模型已展开；关联family Behavior仍needs_review，具体Evidence按支持关系与原epoch读取。剩余义务：native值/错误/ACK/关闭使用实际consumer与对应官方transport证据；新retention/pending ACK窗口须匹配修复后源。最终三语言完整门禁、精确生成签名和发布pin另验，不以旧whole语言counts替当前源。

## 未展开项

组级主路径mapped表示入口、唯一权威、生命周期与具名失败义务已建模；41个family Behavior仍needs_review，开放Finding及最终交付分别保留。旧候选的八crate临时patch与95ecpin保持原运行epoch；当前Host已pin published b909并移除patch，但最终nightlyprofiles/唯一合同生成与SDKwhole源码门禁仍待。frameworkmain交付证据不替SDK精确main/CI。完整Rust inventory仅遥测；accepted字段必须由唯一生成器明确裁决。

native值/错误/ACK/关闭使用实际consumer与对应官方transport证据；新retention/pending ACK窗口须匹配修复后源。最终三语言完整门禁、精确生成签名和发布pin另验，不以旧whole语言counts替当前源。

## 当前观察、归档和后继边界

原fc63证据已保存为仓库内不可变历史收据，原观察epoch不可恢复为Git时不补造提交。原6711证据归档支持具名局部断言；原27个历史输入中tools.rs已变，具名测试body保持可对照；55个6711运行输入中tools.rs和Workflow已变更。新Workflow语义或其它后继场景必须绑定自己的来源/日志；不把旧整套418/TS240/Python300/Java264冒充最终当前通过。支持见历史审计Evidence及evidence.sdk-parent-driver-candidate-verification的限制，family needs_review和开放Finding保留，主场景mapped不扩运行证据。

## 建模完成与验证状态区别

组级主路径已按入口、唯一权威、生命周期和具名失败义务完成映射，故主场景mapped。family Behavior仍needs_review，已知open Finding与有限证据保留；mapped不等于verified、所有lens covered或最终交付。剩余事项采用明确合同/故障原因，不以无限未枚举组合拒绝建模。

## 当前模型重观察范围

review_scope：本对象在真实当前候选Git `56929154c1851473e9e4837901b610cecf001b2d`（源码摘要`17cf243e8d6341db0ce23c5b9ee39977ce4ab100d6ed1b17ec5a7bb26c19e138`）上重新观察。上列code_refs/各scenario source_refs已逐Gitblob读取并与当前文件SHA精确相同，原锚点均存在。模型继续描述原accepted入口、框架业务/终态权威、Host owner/generation与transport消费职责、生命周期及具名失败义务；源码定位核对不证明所有行为正确、运行证据fresh或合同生成闭合。

原Behavior/status、Map场景mapped/needs_review、Rule/Asset处置、Finding/未决义务与Evidence原epoch支持范围全部保留。mapped仅入口模型，旧3ea不是本snapshot本体；历史receipt不得因本模型观察更新冒current运行。12source迁移、5秒物理结算与依赖50warning规则三选择仍未接受，完整生成/CI/main另验，任何新Rustwrapper不自动扩三语言合同。

## 当前八个客户端视角的审查范围

此Map新增八个client-lifecycle-*横切场景，source观察绑定真实76601；对应独立审查均examined、C/I/M0。状态mapped只代表该场景模型和具名审查闭包存在。family-value/family-intrinsic的原needs_review、支持范围及未知字段/唯一generator义务均未提升，不把单一客户端切片变成所有能力通过。原旧pin/replay/阶段whole计数保历史，新官方a634 whole0另有精确790执行清单与71XML；平台/main/12项身份与5秒B仍独立。

## 原已保存源码出处 (569 historical)

56929154c1851473e9e4837901b610cecf001b2d : sdks/typescript/test/lsp-host.test.js, sdks/python/tests/test_lsp_diagnostics_host.py. 这两份测试在原SDKSnapshot可恢复，不在本次76601中；原LSP行为与断言义务保留，不伪装为新提交测试源，不宣称8lens审查替代全LSP契约生成验收。

## Protocol源切点的后继关系

旧Mapobservation/其它族原场景不盲改新版。新8protocolfinite scenes用原已有kernel路径定位，相应622d Evidence/Audit明确newvalidators与initfailureowner truebefore/after，不能catalog/count推allcap或latestwhole。特定广告缺陷局部resolved，statusmapped为具名审查模型不是全globalverified。
