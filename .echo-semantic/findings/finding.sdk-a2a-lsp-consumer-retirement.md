---
{
  "schema_version": 1,
  "id": "finding.sdk-a2a-lsp-consumer-retirement",
  "kind": "finding",
  "type": "consolidation_candidate",
  "status": "resolved",
  "severity": "high",
  "primary_focus": "contract_evidence",
  "focus": [
    "contract_evidence",
    "state_authority"
  ],
  "boundary_ref": "boundary.sdk-integrations",
  "behavior_refs": [
    "behavior.sdk-family-a2a",
    "behavior.sdk-family-lsp"
  ],
  "rule_refs": [
    "rule.framework-runtime-authority"
  ],
  "evidence_refs": [
    "evidence.sdk-a2a-lsp-consumer-retirement",
    "evidence.sdk-sandbox-and-canonical-input-consumption"
  ],
  "audit_refs": [
    "audit.sdk-a2a-lsp-consumer-retirement"
  ],
  "decision_refs": [],
  "repair_evidence_refs": [
    "evidence.sdk-a2a-lsp-consumer-retirement"
  ],
  "verification_evidence_refs": [
    "evidence.sdk-a2a-lsp-consumer-retirement",
    "evidence.sdk-sandbox-and-canonical-input-consumption"
  ],
  "rereview_audit_refs": [
    "audit.sdk-a2a-lsp-consumer-retirement"
  ],
  "discovered_at": "961da0e0d17b815653d80cb4bf4973248f543770",
  "decision": "retire",
  "candidate_asset_refs": [
    "asset.sdk-retired-a2a-lsp-consumer-sources",
    "asset.sdk-source-product"
  ],
  "canonical_asset_ref": "asset.sdk-source-product",
  "replacement_refs": [
    "asset.sdk-source-product"
  ],
  "rollback_ref": "Restore protected SDK tag pre-retirement-2026-10-24 at0930eef9f878cf436ed88384030b6bc03d6f6c53 (code/accepted artifacts match task S961) on a new task branch; preserve recovery refs.",
  "delete_paths": [
    "sdks/java/src/main/java/com/echoagent/sdk/A2AArtifact.java",
    "sdks/java/src/main/java/com/echoagent/sdk/A2AError.java",
    "sdks/java/src/main/java/com/echoagent/sdk/A2AMessage.java",
    "sdks/java/src/main/java/com/echoagent/sdk/A2AStreamEvent.java",
    "sdks/java/src/main/java/com/echoagent/sdk/A2AStreamResponse.java",
    "sdks/java/src/main/java/com/echoagent/sdk/A2ATask.java",
    "sdks/java/src/main/java/com/echoagent/sdk/A2ATaskParams.java",
    "sdks/java/src/main/java/com/echoagent/sdk/A2ATaskRequest.java",
    "sdks/java/src/main/java/com/echoagent/sdk/A2ATaskResponse.java",
    "sdks/java/src/main/java/com/echoagent/sdk/A2ATaskStatus.java",
    "sdks/java/src/main/java/com/echoagent/sdk/AgentAuthentication.java",
    "sdks/java/src/main/java/com/echoagent/sdk/AgentCapabilities.java",
    "sdks/java/src/main/java/com/echoagent/sdk/AgentCard.java",
    "sdks/java/src/main/java/com/echoagent/sdk/AgentCardBuilder.java",
    "sdks/java/src/main/java/com/echoagent/sdk/AgentProvider.java",
    "sdks/java/src/main/java/com/echoagent/sdk/AgentSkill.java",
    "sdks/java/src/main/java/com/echoagent/sdk/AuthenticationScheme.java",
    "sdks/java/src/main/java/com/echoagent/sdk/JwtClaims.java",
    "sdks/java/src/main/java/com/echoagent/sdk/JwtConfig.java",
    "sdks/java/src/main/java/com/echoagent/sdk/TaskArtifactUpdateEvent.java",
    "sdks/java/src/main/java/com/echoagent/sdk/TaskState.java",
    "sdks/java/src/main/java/com/echoagent/sdk/TaskStatusUpdateEvent.java",
    "sdks/python/src/echo_agent_sdk/a2a.py",
    "sdks/python/src/echo_agent_sdk/jwt_values.py",
    "sdks/shared/testing/a2a_host_fixture.py",
    "sdks/shared/testing/lsp_diagnostic_server.py",
    "sdks/shared/testing/lsp_fault_server.py",
    "sdks/typescript/src/a2a.ts",
    "sdks/typescript/src/jwt_values.ts"
  ]
}
---

# Retire the approved A2A/LSP SDK consumer subset

## 问题

Approved unsupported retirement removes exactly269 accepted identities/23 routes/39 operations on fixed f7bac; retained5352 identities have no drift.

## 触发条件与影响

The canonical SDK product retains its runtime/lifecycle authorities.29 obsolete supporting files are removed under the exact approved delete paths; protected baseline refs retain their source.

## 处理记录

Existing generation, feature/dispatch/source projection and language callers switched to the retained SDK. Complete local Rust/profile/contract/language/R2/examples qualification passed, and independent source/runtime review returned pass0. The resolved disposition covers this approved source/runtime withdrawal; it does not declare the normal wrapper or the whole Outcome complete.

## 证据

The referenced evidence and audit bind actual source/runtime results. Final normal wrapper receipt publication, staged gate and overall local rereview remain required before local commit. Original failures and historical unknowns remain unchanged.


## 2026-11-04：四测试接收的当前验证引用

沿原Finding补充当前消费者Evidence，status、decision、discovered_at、删除清单及原repair/audit链不变。
本轮589P及三语言定向各2只增加其实际覆盖的当前验证；旧退役闭合不自动变成SDK整体、main或发行完成。
