---
schema_version: 1
id: audit.sdk-integrations-finite-contract-evidence
kind: audit
boundary_ref: boundary.sdk-integrations
lens: contract_evidence
freshness: examined
revision: bd8e42e732588181b7a2589d91e9ffdce02d23e1
finding_refs:
- finding.sdk-tool-pid-observation-window
- finding.sdk-family-handle-argument-projection
- finding.sdk-lsp-response-fidelity
- finding.sdk-run-steer-canonical-turn-context
challenges:
  named-integrations-and-supporting-role:
    revision: bd8e42e732588181b7a2589d91e9ffdce02d23e1
    source_refs:
    - echo-sdk-host/src/core_profile/facade/integrations.rs
    - echo-sdk-host/src/core_profile/facade/observability.rs
    - echo-sdk-host/src/core_profile/facade/state_delivery.rs
    - echo-sdk-host/src/core_profile/facade/source_operations.rs
    - echo-sdk-host/src/core_profile/facade/permission.rs
    - echo-sdk-host/src/core_profile/facade/registry.rs
    - echo-sdk-host/src/core_profile/facade/mod.rs
    - echo-sdk-host/src/core_profile/facade/stream.rs
    - echo-sdk-host/src/core_profile/handles.rs
    - docs/sdk/governance/evidence/a2a-native-consumption/receipt.json
    - docs/sdk/governance/evidence/a2a-native-consumption/manifest.json
    - docs/sdk/governance/evidence/a2a-native-consumption/comparison.json
    - docs/sdk/governance/evidence/a2a-native-consumption/README.md
    - docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/receipt.json
    - docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/inputs-before.json
    - docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/inputs-after.json
    - docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/java-surefire/TEST-com.echoagent.sdk.TelemetryHostTest.xml
    evidence_refs:
    - evidence.sdk-integrations-finite-observation
  specific-family-handle-argument-projection:
    revision: ad35887c5ceb66865a714f3f2d1da86a9c35c903
    source_refs:
    - echo-sdk-host/src/core_profile/facade/mod.rs#family_handle_argument_retains_opaque_resource_address
    - echo-sdk-host/src/core_profile/facade/mod.rs#family_argument_json
    - docs/sdk/governance/evidence/sdk-final-idempotency-rust-gates/combined-receipt.json
    - docs/sdk/governance/evidence/sdk-final-idempotency-rust-gates/original/05-workspace-tests.log
    - docs/sdk/governance/evidence/sdk-final-idempotency-rust-gates/continuation/13-ci-facade-e2e.log
    - docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/receipt.json
    - docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/inputs-before.json
    evidence_refs:
    - evidence.sdk-integrations-finite-observation
  specific-lsp-response-fidelity:
    revision: ad35887c5ceb66865a714f3f2d1da86a9c35c903
    source_refs:
    - echo-sdk-host/src/core_profile/facade/integrations.rs
    - echo-sdk-host/tests/facade_feature_adapters_e2e.rs
    - docs/sdk/governance/evidence/sdk-final-idempotency-rust-gates/combined-receipt.json
    - docs/sdk/governance/evidence/sdk-final-idempotency-rust-gates/original/05-workspace-tests.log
    - docs/sdk/governance/evidence/sdk-final-idempotency-rust-gates/continuation/13-ci-facade-e2e.log
    - docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/receipt.json
    - docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/inputs-before.json
    evidence_refs:
    - evidence.sdk-integrations-finite-observation
  specific-run-steer-canonical-turn-context:
    revision: ad35887c5ceb66865a714f3f2d1da86a9c35c903
    source_refs:
    - echo-sdk-host/src/core_profile/handler.rs
    - docs/sdk/governance/evidence/run-steer-native/initial-red-manifest.json
    - docs/sdk/governance/evidence/run-steer-native/java-original/receipt.json
    - docs/sdk/governance/evidence/run-steer-native/java-diagnostic-fixed/receipt.json
    - docs/sdk/governance/evidence/sdk-final-idempotency-rust-gates/combined-receipt.json
    - docs/sdk/governance/evidence/sdk-final-idempotency-rust-gates/original/05-workspace-tests.log
    - docs/sdk/governance/evidence/sdk-final-idempotency-rust-gates/continuation/13-ci-facade-e2e.log
    - docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/receipt.json
    - docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/inputs-before.json
    evidence_refs:
    - evidence.sdk-integrations-finite-observation
  specific-pid-observation-repair:
    revision: c5c977ef862acbe99684294cca966c559859b6be
    source_refs:
    - sdks/python/tests/test_tool_value_effect_host.py
    evidence_refs:
    - evidence.sdk-tools-bridge-finite-observation
  specific-pid-observation-e58-after:
    revision: e58a3e0c07cf0b6b24d570df07063301a888ddc6
    source_refs:
    - sdks/python/tests/test_tool_value_effect_host.py
    evidence_refs:
    - evidence.sdk-tools-bridge-finite-observation
  named-settlement-family-a2a:
    revision: 04eb6ad50982656b5a8b8ea649619b6a383065ff
    source_refs:
    - echo-sdk-host/src/core_profile/facade/mod.rs
    - echo-sdk-host/src/core_profile/facade/registry.rs
    - echo-sdk-host/src/core_profile/facade/source_operations.rs
    - echo-sdk-host/src/core_profile/facade/integrations.rs
    - echo-sdk-host/src/core_profile/facade/memory.rs
    - echo-sdk-host/src/core_profile/facade/workflow.rs
    - echo-sdk-host/src/core_profile/facade/state_delivery.rs
    - echo-sdk-host/src/core_profile/facade/tools.rs
    - echo-sdk-host/src/core_profile/facade/observability.rs
    evidence_refs:
    - evidence.sdk-integrations-finite-observation
  named-settlement-family-channels:
    revision: 04eb6ad50982656b5a8b8ea649619b6a383065ff
    source_refs:
    - echo-sdk-host/src/core_profile/facade/mod.rs
    - echo-sdk-host/src/core_profile/facade/registry.rs
    - echo-sdk-host/src/core_profile/facade/source_operations.rs
    - echo-sdk-host/src/core_profile/facade/integrations.rs
    - echo-sdk-host/src/core_profile/facade/memory.rs
    - echo-sdk-host/src/core_profile/facade/workflow.rs
    - echo-sdk-host/src/core_profile/facade/state_delivery.rs
    - echo-sdk-host/src/core_profile/facade/tools.rs
    - echo-sdk-host/src/core_profile/facade/observability.rs
    evidence_refs:
    - evidence.sdk-channel-resource-admission
    - evidence.sdk-channel-cooperative-two
    - evidence.sdk-extension-teardown-prefix-verification
    - evidence.sdk-integrations-finite-observation
  named-settlement-family-content-guard:
    revision: 04eb6ad50982656b5a8b8ea649619b6a383065ff
    source_refs:
    - echo-sdk-host/src/core_profile/facade/mod.rs
    - echo-sdk-host/src/core_profile/facade/registry.rs
    - echo-sdk-host/src/core_profile/facade/source_operations.rs
    - echo-sdk-host/src/core_profile/facade/integrations.rs
    - echo-sdk-host/src/core_profile/facade/memory.rs
    - echo-sdk-host/src/core_profile/facade/workflow.rs
    - echo-sdk-host/src/core_profile/facade/state_delivery.rs
    - echo-sdk-host/src/core_profile/facade/tools.rs
    - echo-sdk-host/src/core_profile/facade/observability.rs
    evidence_refs:
    - evidence.sdk-computation-50
  named-settlement-family-git:
    revision: 04eb6ad50982656b5a8b8ea649619b6a383065ff
    source_refs:
    - echo-sdk-host/src/core_profile/facade/mod.rs
    - echo-sdk-host/src/core_profile/facade/registry.rs
    - echo-sdk-host/src/core_profile/facade/source_operations.rs
    - echo-sdk-host/src/core_profile/facade/integrations.rs
    - echo-sdk-host/src/core_profile/facade/memory.rs
    - echo-sdk-host/src/core_profile/facade/workflow.rs
    - echo-sdk-host/src/core_profile/facade/state_delivery.rs
    - echo-sdk-host/src/core_profile/facade/tools.rs
    - echo-sdk-host/src/core_profile/facade/observability.rs
    evidence_refs:
    - evidence.sdk-tool-behavior-candidate
    - evidence.sdk-git-published-consumer-verification
    - evidence.sdk-integrations-finite-observation
  named-settlement-family-lsp:
    revision: 04eb6ad50982656b5a8b8ea649619b6a383065ff
    source_refs:
    - echo-sdk-host/src/core_profile/facade/mod.rs
    - echo-sdk-host/src/core_profile/facade/registry.rs
    - echo-sdk-host/src/core_profile/facade/source_operations.rs
    - echo-sdk-host/src/core_profile/facade/integrations.rs
    - echo-sdk-host/src/core_profile/facade/memory.rs
    - echo-sdk-host/src/core_profile/facade/workflow.rs
    - echo-sdk-host/src/core_profile/facade/state_delivery.rs
    - echo-sdk-host/src/core_profile/facade/tools.rs
    - echo-sdk-host/src/core_profile/facade/observability.rs
    evidence_refs:
    - evidence.sdk-integrations-finite-observation
  named-settlement-family-mcp:
    revision: 04eb6ad50982656b5a8b8ea649619b6a383065ff
    source_refs:
    - echo-sdk-host/src/core_profile/facade/mod.rs
    - echo-sdk-host/src/core_profile/facade/registry.rs
    - echo-sdk-host/src/core_profile/facade/source_operations.rs
    - echo-sdk-host/src/core_profile/facade/integrations.rs
    - echo-sdk-host/src/core_profile/facade/memory.rs
    - echo-sdk-host/src/core_profile/facade/workflow.rs
    - echo-sdk-host/src/core_profile/facade/state_delivery.rs
    - echo-sdk-host/src/core_profile/facade/tools.rs
    - echo-sdk-host/src/core_profile/facade/observability.rs
    evidence_refs:
    - evidence.sdk-mcp-channel-owned-candidate
    - evidence.sdk-integrations-finite-observation
  named-settlement-family-testing:
    revision: 04eb6ad50982656b5a8b8ea649619b6a383065ff
    source_refs:
    - echo-sdk-host/src/core_profile/facade/mod.rs
    - echo-sdk-host/src/core_profile/facade/registry.rs
    - echo-sdk-host/src/core_profile/facade/source_operations.rs
    - echo-sdk-host/src/core_profile/facade/integrations.rs
    - echo-sdk-host/src/core_profile/facade/memory.rs
    - echo-sdk-host/src/core_profile/facade/workflow.rs
    - echo-sdk-host/src/core_profile/facade/state_delivery.rs
    - echo-sdk-host/src/core_profile/facade/tools.rs
    - echo-sdk-host/src/core_profile/facade/observability.rs
    evidence_refs:
    - evidence.sdk-testing-value-candidate
    - evidence.sdk-integrations-finite-observation
  named-settlement-family-topology:
    revision: 04eb6ad50982656b5a8b8ea649619b6a383065ff
    source_refs:
    - echo-sdk-host/src/core_profile/facade/mod.rs
    - echo-sdk-host/src/core_profile/facade/registry.rs
    - echo-sdk-host/src/core_profile/facade/source_operations.rs
    - echo-sdk-host/src/core_profile/facade/integrations.rs
    - echo-sdk-host/src/core_profile/facade/memory.rs
    - echo-sdk-host/src/core_profile/facade/workflow.rs
    - echo-sdk-host/src/core_profile/facade/state_delivery.rs
    - echo-sdk-host/src/core_profile/facade/tools.rs
    - echo-sdk-host/src/core_profile/facade/observability.rs
    evidence_refs:
    - evidence.sdk-eval-improve-topology
    - evidence.sdk-integrations-finite-observation
  named-settlement-integrations:
    revision: 04eb6ad50982656b5a8b8ea649619b6a383065ff
    source_refs:
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
    evidence_refs:
    - evidence.sdk-channel-resource-admission
    - evidence.sdk-mcp-channel-owned-candidate
    - evidence.sdk-integrations-finite-observation
---

# Integrations contract_evidence 有限审查

## 审查范围

原frameworkFinal独立review实际原source/790/a634 C0/I0/M0，后A2A bd8三native补proof另scopePASS；模型候选不重业务review或任意allmethods。九运行blob原69b在bd8同，currenthandlesA不同排除。

## 已检查故障假设

明确acceptedfamily菜单及supportingregistry/skill角色，不Catalogdone替behaviour；actual790来源与新A2Abd8互分，14testingfields不Mockruntime；12source与newfullgen另义务。

## 实际实现路径与证据

具体MCP/LSP/channels/analysis/permission/FileState/Trace/内存Delivery/Testing和registry-skill支持角色在共享Evidence每family表逐项列入口/authority/生命周期/错误。原caller/proof位置与actualSHA保。新A2A不虚count：各24step/9acceptedops/8HTTP三native原exactfield比较，首次missing和后补scope明确分开。

## 问题记录

原有限源码reviewscope与后native增量均PASS0，无新增确定源码阻断；材料验收仅确认忠实范围，不把sdkwhole/main/fullaccepted生成虚构通过。

## 残余风险

Delivery内存不crossprocessdurable；checkpoint7/Trace17区别；configuredsanitizer不未知所有secret，TestingValue不Mockbusiness；5秒B/12/最新A/平台/main独立。

## 未检查项

任意未接受constructor/sourceoperation/codec/backend和远端物理finally，非这个finite菜单义务。原A2Afuturegen不是重启证明。

## family-handle-argument-projection具体旧缺陷复核

原独立frameworkFinal本session三链artifactreview确匹配：完整opaqueWireHandleUnicode/u64MAXDTO回原resourceparser/Registry，三native returnedHandle正常API回传，不genericrawmap绕过。 这是既有实际source/strongassert复核的回收；此新challenge观察ad的真实文件，不改变原主revision/bd8的第一次Integration/source审范围。

## lsp-response-fidelity具体旧缺陷复核

原独立frameworkFinal本session三链artifactreview确匹配：frameworkonlynullable/hoverrange/Optionkind/codeunion/objectshape解析与SDKthin、native29request/42diagnostic完整字段/nextvalid/PIDclosed复核；12sourcefieldmigration另intent。 这是既有实际source/strongassert复核的回收；此新challenge观察ad的真实文件，不改变原主revision/bd8的第一次Integration/source审范围。

## run-steer-canonical-turn-context具体旧缺陷复核

原独立frameworkFinal本session三链artifactreview确匹配：根修复frameworka634 echo-core/src/agent/mod.rs + src/acp/runtime.rs reservedcanonicalturn/fallback；Hostconsumer严格identity不放宽，3nativeHTTPsecondrequestUnicode/accepted/lateempty/2wait真实证明；oldbefore/diagcompat限制保。 这是既有实际source/strongassert复核的回收；此新challenge观察ad的真实文件，不改变原主revision/bd8的第一次Integration/source审范围。


## finding.sdk-tool-pid-observation-window 的具名有限复核

原0ebe同PID default/wide直接取证、仅-ww修复、严格身份/信号前复核/物理after[]及protectedrows的SDKcap实际原source/native审，接e58完整语言/三quickstart和Host终态success；不把旧排队反例或Host另一个超时归因混入。

两revision挑战保修复源与新组合执行输入的准确范围，原top revision/旧challenge/body及freshness含义不变。本次元候选需独立材料审后Root批准安装才生效；当前不伪另一次源码/工程运行。只此specific localclosed，不接受B/12、Host cause、全物理cleanup、全部CI/main或1173连续性义务。


## 2026-10-20 具名后继的有界结算

主header revision/freshness及原挑战仍保原审查cut；这里只记录下列named单元，原17be/0bbc读审与04eb受控media增量分开，旧Evidence运行不重标。任何一个row不能推整八风险组或整composite资格。

- `behavior.sdk-family-a2a`：有限named单元结算；证据`evidence.sdk-integrations-finite-observation`。
- `behavior.sdk-family-channels`：有限named单元结算；证据`evidence.sdk-channel-resource-admission`、`evidence.sdk-channel-cooperative-two`、`evidence.sdk-extension-teardown-prefix-verification`、`evidence.sdk-integrations-finite-observation`。
- `behavior.sdk-family-content-guard`：有限named单元结算；证据`evidence.sdk-computation-50`。
- `behavior.sdk-family-git`：有限named单元结算；证据`evidence.sdk-tool-behavior-candidate`、`evidence.sdk-git-published-consumer-verification`、`evidence.sdk-integrations-finite-observation`。
- `behavior.sdk-family-lsp`：有限named单元结算；证据`evidence.sdk-integrations-finite-observation`。
- `behavior.sdk-family-mcp`：有限named单元结算；证据`evidence.sdk-mcp-channel-owned-candidate`、`evidence.sdk-integrations-finite-observation`。
- `behavior.sdk-family-testing`：有限named单元结算；证据`evidence.sdk-testing-value-candidate`、`evidence.sdk-integrations-finite-observation`。
- `behavior.sdk-family-topology`：有限named单元结算；证据`evidence.sdk-eval-improve-topology`、`evidence.sdk-integrations-finite-observation`。
- `behavior.sdk-integrations`：有限named单元结算；证据`evidence.sdk-channel-resource-admission`、`evidence.sdk-mcp-channel-owned-candidate`、`evidence.sdk-integrations-finite-observation`。
