---
schema_version: 1
id: evidence.sdk-task-memory-workflow-finite-observation
kind: evidence
observed_at: 02059dd3014633a23c188e30e73b0634a8c5621d
source_refs:
- echo-sdk-host/src/core_profile/facade/task.rs
- echo-sdk-host/src/core_profile/facade/task_runtime.rs
- echo-sdk-host/src/core_profile/facade/subagent.rs
- echo-sdk-host/src/core_profile/facade/memory.rs
- echo-sdk-host/src/core_profile/facade/workflow.rs
- echo-sdk-host/src/core_profile/facade/source_operations.rs
- echo-sdk-host/src/core_profile/facade/mod.rs
- echo-sdk-host/src/factory.rs
- echo-sdk-host/tests/core_profile_e2e.rs
- echo-sdk-host/tests/extension_bridge_e2e.rs
- docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/receipt.json
- docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/inputs-before.json
- docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/java-surefire/TEST-com.echoagent.sdk.TaskSubagentHostTest.xml
- docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/java-surefire/TEST-com.echoagent.sdk.WorkflowCheckpointHostTest.xml
- docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/java-surefire/TEST-com.echoagent.sdk.MemoryPermissionHostTest.xml
- docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/java-surefire/TEST-com.echoagent.sdk.LlmWorkflowStreamHostTest.xml
supports:
- behavior.sdk-task-subagent
- behavior.sdk-family-task
- behavior.sdk-family-subagent
- behavior.sdk-family-memory
- behavior.sdk-family-workflow
limitations:
- Original CLI author sourceaudit is distinguished from actual independent material/source slices; not renamed an independent new execution.
- Source restored in02059 is byte-identical to reviewed69b for these paths; current mutable factory/core tests or intentA implementation are excluded.
- ContextInheritance value tests do not prove three-language livehistory/memory inheritance E2E; no new context family/API is required.
- TaskContext getter consumption supports four exact Host cases and native runid only, not complete threefield reverseDTO.
- Memory/checkpoint backends are volatile in this accepted path, not crossprocessdurable;5second physicalB/12sourcekind and SDK main remain separate.
---

# Task/Subagent、Memory与Workflow有界观察

## 支持的结论

回收CLI原已完成实际只读8lens审查，无新增已证实源码缺陷，不伪由模型作者另做业务review。该组按现catalog task/subagent/memory/workflow四acceptedfamily定位；inheritance/PromptContext/compression属于既有source_operation/extbridge支持边界，不新增contextfamily。TaskRun/revision/claim/terminal同framework服务；Host只参数/metadata转换及handle；Memory每次调用liveAgentStore，WorkflowGraph/checkpoint原owner保一权威。

## 来源与范围

复核观察Git02059（10个原运行/测试文件与69b逐Gitblob一致），真实辅助native源进入02059后不再伪缺。原审函数范围：task_runtime123与四getter消费者tests845/943/992/1037；subagent91/193/381/404；memory99/333/396；workflow304/449/696；facade selector1828与sourceoperations3321/7298/7326/7575；bridgeE2Ecompressor3418。列出的factory/coretests当前A切片已不同，故本观察仍恢复原源码，不称最新A已被这组验证。

真实a634官方语言收据790inputs same/exit0，TaskJava4、WorkflowCheckpoint1、MemoryPermission1、LlmWorkflow3 XML均0failure/error/skip，各有原三语言对应场景。旧failedTaskHTTP夹具与受控Connectionclose同源前后结果保持自己的资料，不借Task4推其随机socket直接因果。四Host getter场景证明runid、waiver/nonwaivedmissingresult、nondefaultdelegation被真实Host消费；三nativeTask callback只加强runid来自SessionACP，不声称全部contextDTO三字段直接传CustomAgent。

## 已知缺口

ContextInheritance纯值support不替livehistory/memory继承三语E2E（现accepted路径源码/角色明确，补证不得造新API）；Hostcompressor注册消费不推每个hook reentry/cancel已经三native执行。内存Store/checkpoint无额外跨processdurability保证。5秒physicalB/旧12field source身份处置继续原Finding/待人，不由绿色Task场景riskaccept；新意图A幂等caller合同已人批准，其最新实现/原生gate与本原source审切片独立。scope外未接受构造器或全部unlistedsourceoperation不自动纳新完成分母。
