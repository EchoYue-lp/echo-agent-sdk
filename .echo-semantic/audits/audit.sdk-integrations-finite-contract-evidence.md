---
schema_version: 1
id: audit.sdk-integrations-finite-contract-evidence
kind: audit
boundary_ref: boundary.sdk-integrations
lens: contract_evidence
freshness: examined
revision: bd8e42e732588181b7a2589d91e9ffdce02d23e1
finding_refs:
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
