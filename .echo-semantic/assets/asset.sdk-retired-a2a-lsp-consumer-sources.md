---
schema_version: 1
id: asset.sdk-retired-a2a-lsp-consumer-sources
kind: asset
title: SDK A2A/LSP source surface selected for retirement
asset_type: protocol
status: deprecated
risk: high
observed_at: 961da0e0d17b815653d80cb4bf4973248f543770
boundary_refs:
- boundary.sdk-integrations
code_refs:
- sdks/java/src/main/java/com/echoagent/sdk/A2AArtifact.java
- sdks/java/src/main/java/com/echoagent/sdk/A2AError.java
- sdks/java/src/main/java/com/echoagent/sdk/A2AMessage.java
- sdks/java/src/main/java/com/echoagent/sdk/A2AStreamEvent.java
- sdks/java/src/main/java/com/echoagent/sdk/A2AStreamResponse.java
- sdks/java/src/main/java/com/echoagent/sdk/A2ATask.java
- sdks/java/src/main/java/com/echoagent/sdk/A2ATaskParams.java
- sdks/java/src/main/java/com/echoagent/sdk/A2ATaskRequest.java
- sdks/java/src/main/java/com/echoagent/sdk/A2ATaskResponse.java
- sdks/java/src/main/java/com/echoagent/sdk/A2ATaskStatus.java
- sdks/java/src/main/java/com/echoagent/sdk/AgentAuthentication.java
- sdks/java/src/main/java/com/echoagent/sdk/AgentCapabilities.java
- sdks/java/src/main/java/com/echoagent/sdk/AgentCard.java
- sdks/java/src/main/java/com/echoagent/sdk/AgentCardBuilder.java
- sdks/java/src/main/java/com/echoagent/sdk/AgentProvider.java
- sdks/java/src/main/java/com/echoagent/sdk/AgentSkill.java
- sdks/java/src/main/java/com/echoagent/sdk/AuthenticationScheme.java
- sdks/java/src/main/java/com/echoagent/sdk/JwtClaims.java
- sdks/java/src/main/java/com/echoagent/sdk/JwtConfig.java
- sdks/java/src/main/java/com/echoagent/sdk/TaskArtifactUpdateEvent.java
- sdks/java/src/main/java/com/echoagent/sdk/TaskState.java
- sdks/java/src/main/java/com/echoagent/sdk/TaskStatusUpdateEvent.java
- sdks/python/src/echo_agent_sdk/a2a.py
- sdks/python/src/echo_agent_sdk/jwt_values.py
- sdks/shared/testing/a2a_host_fixture.py
- sdks/shared/testing/lsp_diagnostic_server.py
- sdks/shared/testing/lsp_fault_server.py
- sdks/typescript/src/a2a.ts
- sdks/typescript/src/jwt_values.ts
consumer_refs:
- docs/adr/0042-sdk-a2a-lsp-retirement-boundary.md
behavior_refs:
- behavior.sdk-family-a2a
- behavior.sdk-family-lsp
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-a2a-lsp-consumer-retirement
finding_refs:
- finding.sdk-a2a-lsp-consumer-retirement
candidate_refs:
- asset.sdk-source-product
---

# SDK A2A/LSP consumer sources

## 资产身份

The exact29 retired language/helper/peer files are observed at intactS961; their current disposition is deprecated and the approved consumer subset is removed.

## 来源与消费者

They previously served the269 explicitly retired SDK identities. The retained canonical product is asset.sdk-source-product; no replacement A2A/LSP executor or successful stub exists.

## 生命周期

Caller cutover, static closure, current real Host refusals and retained Rust/language/R2/examples are qualified in the linked current evidence. Historical source remains recoverable through preserved refs.

## 候选关系

The approved retire decision keeps shared owners in the canonical SDK and removes only the exact supporting subset.

## 未知与限制

The framework old pin still contains its Rust definitions as inventory-only facts. Normal wrapper/staged/mainCI and whole cross-repository Outcome are separate gates; historical unknowns are unchanged.
