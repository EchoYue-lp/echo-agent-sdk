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
    "evidence.sdk-a2a-lsp-consumer-retirement"
  ],
  "audit_refs": [
    "audit.sdk-a2a-lsp-consumer-retirement"
  ],
  "decision_refs": [],
  "repair_evidence_refs": [
    "evidence.sdk-a2a-lsp-consumer-retirement"
  ],
  "verification_evidence_refs": [
    "evidence.sdk-a2a-lsp-consumer-retirement"
  ],
  "rereview_audit_refs": [
    "audit.sdk-a2a-lsp-consumer-retirement"
  ],
  "discovered_at": "0930eef9f878cf436ed88384030b6bc03d6f6c53",
  "decision": "retire",
  "candidate_asset_refs": [
    "asset.sdk-retired-a2a-lsp-consumer-sources",
    "asset.sdk-source-product"
  ],
  "canonical_asset_ref": "asset.sdk-source-product",
  "replacement_refs": [
    "asset.sdk-source-product"
  ],
  "rollback_ref": "在新任务分支恢复受保护基底0930eef9f878cf436ed88384030b6bc03d6f6c53及完整代码/合同/客户端；保原恢复ref，不在本任务静默恢复旧执行。",
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
    "sdks/java/src/main/java/com/echoagent/sdk/TaskArtifactUpdateEvent.java",
    "sdks/java/src/main/java/com/echoagent/sdk/TaskState.java",
    "sdks/java/src/main/java/com/echoagent/sdk/TaskStatusUpdateEvent.java",
    "sdks/python/src/echo_agent_sdk/a2a.py",
    "sdks/typescript/src/a2a.ts"
  ]
}
---

# StageA具名消费义务退出

## 问题

沿Plan04/ADR0042仅撤销269到5352、23路由到431和39操作，新增接受0。物理删除仅delete_paths的22条，不能填成eea85 formal-only或历史29条。Root本回合已明确批准同四对象当前资料，不新增产品取舍。

## 证据

原唯一producer精差集与退出代码已由Root接受；关联Evidence/Audit承接原16精输入闭包、当前原18及Root已接受源码/工程复核。resolved仅表示具名业务退出与保留义务有真实事实链，不代当前四metadata独审、正常Source/Final或跨仓/main交付。

## 处理记录

本地JWT正控、ACP/MCP/Task/Subagent/stdio/close、Parquet和三quickstart保留。回滚须成套回到基底及合同，不用假空成功或任意恢复旧执行器。

## 触发条件与影响

只撤销accepted seed但保旧家族/方法/投影会把退役能力重新暴露；按整文件误删本地JWT则会破坏本阶段保留值语义。当前代码、精集合与反控已在本期Root接受范围验证；Source资料遗漏会使已获准22删除缺真实可追溯repairRefs绑定。本对象不扩到eea85路径或新接受。

## 本次resolved验证链的精确含义

repair_evidence_refs所引Evidence中的普通事实记录ADR0042批准的退出和after拒绝；这些行为变化不是等价。verification_evidence_refs只取该Evidence唯一scenario_results内的共同保留声明合同静态对照，不指退役旧执行、JWT/Parquet/close运行前后相同。after当前可用性另由真实工程结果承担。rereview_audit_refs记录现有源码/运行复核及本次I1修正范围，当前metadata增量仍待Root有界复审；resolved限具名批准处置和已直接证明的保留合同，不作完整动态等价或最终交付声明。若共同静态对照不成立，不能继续用matched补绿。
