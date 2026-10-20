---
schema_version: 1
id: audit.sdk-integrations-finite-time-lifecycle
kind: audit
boundary_ref: boundary.sdk-integrations
lens: time_lifecycle
freshness: examined
revision: bd8e42e732588181b7a2589d91e9ffdce02d23e1
finding_refs:
- finding.sdk-agent-close-cascade
- finding.sdk-channel-message-handler-consumer
- finding.sdk-mcp-manager-owner
- finding.sdk-telemetry-host-init-rollback-block
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
  specific-agent-close-cascade:
    revision: ad35887c5ceb66865a714f3f2d1da86a9c35c903
    source_refs:
    - echo-sdk-host/src/core_profile/handler.rs
    - echo-sdk-host/src/core_profile/handles.rs
    - echo-sdk-host/src/core_profile/facade/mod.rs
    - echo-sdk-host/tests/facade_feature_adapters_e2e.rs#agent_cascade_preserves_same_named_other_manager_and_releases_resources
    - docs/sdk/governance/evidence/sdk-final-idempotency-rust-gates/combined-receipt.json
    - docs/sdk/governance/evidence/sdk-final-idempotency-rust-gates/original/05-workspace-tests.log
    - docs/sdk/governance/evidence/sdk-final-idempotency-rust-gates/continuation/13-ci-facade-e2e.log
    - docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/receipt.json
    - docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/inputs-before.json
    evidence_refs:
    - evidence.sdk-integrations-finite-observation
  specific-channel-message-handler-consumer:
    revision: ad35887c5ceb66865a714f3f2d1da86a9c35c903
    source_refs:
    - echo-sdk-host/src/core_profile/facade/integrations.rs
    - echo-sdk-host/src/core_profile/extension_bridge.rs
    - docs/adr/0035-channel-handler-forward-consumption.md
    - docs/sdk/governance/evidence/sdk-final-idempotency-rust-gates/combined-receipt.json
    - docs/sdk/governance/evidence/sdk-final-idempotency-rust-gates/original/05-workspace-tests.log
    - docs/sdk/governance/evidence/sdk-final-idempotency-rust-gates/continuation/13-ci-facade-e2e.log
    - docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/receipt.json
    - docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/inputs-before.json
    evidence_refs:
    - evidence.sdk-integrations-finite-observation
  specific-mcp-manager-owner:
    revision: ad35887c5ceb66865a714f3f2d1da86a9c35c903
    source_refs:
    - echo-sdk-host/src/core_profile/facade/integrations.rs
    - echo-sdk-host/tests/facade_feature_adapters_e2e.rs#agent_cascade_preserves_same_named_other_manager_and_releases_resources
    - docs/sdk/governance/evidence/sdk-final-idempotency-rust-gates/combined-receipt.json
    - docs/sdk/governance/evidence/sdk-final-idempotency-rust-gates/original/05-workspace-tests.log
    - docs/sdk/governance/evidence/sdk-final-idempotency-rust-gates/continuation/13-ci-facade-e2e.log
    - docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/receipt.json
    - docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/inputs-before.json
    evidence_refs:
    - evidence.sdk-integrations-finite-observation
  specific-telemetry-host-init-rollback-block:
    revision: ad35887c5ceb66865a714f3f2d1da86a9c35c903
    source_refs:
    - echo-sdk-host/src/main.rs
    - echo-sdk-host/src/core_profile/facade/telemetry.rs
    - echo-sdk-host/src/core_profile/state.rs
    - docs/sdk/governance/evidence/sdk-final-idempotency-rust-gates/combined-receipt.json
    - docs/sdk/governance/evidence/sdk-final-idempotency-rust-gates/original/05-workspace-tests.log
    - docs/sdk/governance/evidence/sdk-final-idempotency-rust-gates/continuation/13-ci-facade-e2e.log
    - docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/receipt.json
    - docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/inputs-before.json
    evidence_refs:
    - evidence.sdk-integrations-finite-observation
  named-settlement-family-eval:
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
    - evidence.sdk-factory-child-settlement-verification
  named-settlement-family-permission:
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
    - evidence.sdk-memory-permission-candidate
    - evidence.sdk-real-permission-default-ttl
  named-settlement-family-telemetry:
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
    - evidence.sdk-parent-driver-candidate-verification
---

# Integrations time_lifecycle 有限审查

## 审查范围

原frameworkFinal独立review实际原source/790/a634 C0/I0/M0，后A2A bd8三native补proof另scopePASS；模型候选不重业务review或任意allmethods。九运行blob原69b在bd8同，currenthandlesA不同排除。

## 已检查故障假设

quota先reservation、初始化/derivedowner/cleanup retry、两factcallbacklease、analysisdeadline后cleanup与workspace保持、TTL真实30分钟、A2A localSSEclose真实只settleadapter。

## 实际实现路径与证据

具体MCP/LSP/channels/analysis/permission/FileState/Trace/内存Delivery/Testing和registry-skill支持角色在共享Evidence每family表逐项列入口/authority/生命周期/错误。原caller/proof位置与actualSHA保。新A2A不虚count：各24step/9acceptedops/8HTTP三native原exactfield比较，首次missing和后补scope明确分开。

## 问题记录

原有限源码reviewscope与后native增量均PASS0，无新增确定源码阻断；材料验收仅确认忠实范围，不把sdkwhole/main/fullaccepted生成虚构通过。

## 残余风险

Delivery内存不crossprocessdurable；checkpoint7/Trace17区别；configuredsanitizer不未知所有secret，TestingValue不Mockbusiness；5秒B/12/最新A/平台/main独立。

## 未检查项

任意未接受constructor/sourceoperation/codec/backend和远端物理finally，非这个finite菜单义务。原A2Afuturegen不是重启证明。

## agent-close-cascade具体旧缺陷复核

原独立frameworkFinal本session三链artifactreview确匹配：Agentmutex/SessionRWfence/facadecleanup与同名2Manager隔离、PID退出、资源quota再create强assert，不Bphysical。 这是既有实际source/strongassert复核的回收；此新challenge观察ad的真实文件，不改变原主revision/bd8的第一次Integration/source审范围。

## channel-message-handler-consumer具体旧缺陷复核

原独立frameworkFinal本session三链artifactreview确匹配：批准6forward/streamcontrol真实registeredconsumer/handlerinvoke/reply/originalowner/fullgen、两factcallbacklease/取消native，原start仅存handler的missingconsumer义务已补；不全physicsB。 这是既有实际source/strongassert复核的回收；此新challenge观察ad的真实文件，不改变原主revision/bd8的第一次Integration/source审范围。

## mcp-manager-owner具体旧缺陷复核

原独立frameworkFinal本session三链artifactreview确匹配：sameSession/sameName两manager原Arcaliasmatching，disconnectA另Btools仍可调用；cascade/PID与quota强assert，不能只资源count推owner。 这是既有实际source/strongassert复核的回收；此新challenge观察ad的真实文件，不改变原主revision/bd8的第一次Integration/source审范围。

## telemetry-host-init-rollback-block具体旧缺陷复核

原独立frameworkFinal本session三链artifactreview确匹配：根修复frameworka634 src/telemetry.rs稳定OTEL/filtermarker/原runtimeoneowner；Hostmain/facade/state blockingcollector先settlement、RPCcancel/EOF不释放原job；原3hangOSsample→realcollectorflush/publiccloseheldreleasePIDs完整source审，noarbitrarynetwork。 这是既有实际source/strongassert复核的回收；此新challenge观察ad的真实文件，不改变原主revision/bd8的第一次Integration/source审范围。


## 2026-10-20 具名后继的有界结算

主header revision/freshness及原挑战仍保原审查cut；这里只记录下列named单元，原17be/0bbc读审与04eb受控media增量分开，旧Evidence运行不重标。任何一个row不能推整八风险组或整composite资格。

- `behavior.sdk-family-eval`：有限named单元结算；证据`evidence.sdk-eval-improve-topology`、`evidence.sdk-factory-child-settlement-verification`。
- `behavior.sdk-family-permission`：有限named单元结算；证据`evidence.sdk-memory-permission-candidate`、`evidence.sdk-real-permission-default-ttl`。
- `behavior.sdk-family-telemetry`：有限named单元结算；证据`evidence.sdk-integrations-finite-observation`、`evidence.sdk-parent-driver-candidate-verification`。
