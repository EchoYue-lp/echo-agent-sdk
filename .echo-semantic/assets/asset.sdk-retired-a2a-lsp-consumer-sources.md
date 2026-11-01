---
{
  "schema_version": 1,
  "id": "asset.sdk-retired-a2a-lsp-consumer-sources",
  "kind": "asset",
  "title": "StageA已批准退出的SDK A2A/LSP消费源",
  "asset_type": "protocol",
  "status": "deprecated",
  "risk": "high",
  "observed_at": "0930eef9f878cf436ed88384030b6bc03d6f6c53",
  "boundary_refs": [
    "boundary.sdk-integrations"
  ],
  "code_refs": [
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
    "sdks/java/src/main/java/com/echoagent/sdk/TaskArtifactUpdateEvent.java",
    "sdks/java/src/main/java/com/echoagent/sdk/TaskState.java",
    "sdks/java/src/main/java/com/echoagent/sdk/TaskStatusUpdateEvent.java",
    "sdks/python/src/echo_agent_sdk/a2a.py",
    "sdks/typescript/src/a2a.ts"
  ],
  "consumer_refs": [
    "docs/adr/0032-sdk-contract-scope-classification.md",
    "echo-sdk-host/src/features.rs",
    "sdks/typescript/src/index.ts",
    "sdks/python/src/echo_agent_sdk/__init__.py"
  ],
  "behavior_refs": [
    "behavior.sdk-family-a2a",
    "behavior.sdk-family-lsp"
  ],
  "rule_refs": [
    "rule.framework-runtime-authority"
  ],
  "evidence_refs": [
    "evidence.sdk-a2a-lsp-consumer-retirement"
  ],
  "finding_refs": [
    "finding.sdk-a2a-lsp-consumer-retirement"
  ],
  "candidate_refs": [
    "asset.sdk-source-product"
  ]
}
---

# StageA SDK退役源资产

## 资产身份

code_refs观察真实main0930上本阶段删除的22条支持源；四本地JWT模块/类与三个未再接入的旧peer不在删除表。权威是ADR0042精269 source/kind，不按文件名扩删合理能力。

## 来源与消费者

原SDK A2A/LSP家族、方法、39操作和23路由退出，没有成功空壳。本地JWT值保留不等于恢复九项A2A远程映射义务。

## 生命周期

当前5352/431、旧地址/三插件拒绝与保留owner由关联Evidence记录本期实际结果。不继承eea历史29路径全删断言。

## 候选关系

canonical仍为asset.sdk-source-product；不新增执行器、validator、Store或状态权威。

## 未知与限制

008/e151为Root另批准后继；13SourceID/现checkout相等不等于HTTPS获取。eea九个native/with_id/资源后继原ref全保，最终正确main基底三方合流；旧Think/Token暂存不是最终旧工具目标。
