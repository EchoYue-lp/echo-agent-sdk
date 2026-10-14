---
schema_version: 1
id: evidence.sdk-task-memory-workflow-finite-observation
kind: evidence
observed_at: b7e6c68fbaa91cc97ad5fef4ff3d5a61b187345a
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
- docs/sdk/governance/evidence/sdk-final-idempotency-rust-gates/original/05-workspace-tests.log
- docs/sdk/governance/evidence/sdk-final-idempotency-rust-gates/step10-import-fix/10-ci-core-profile-after.log
- docs/sdk/governance/evidence/sdk-final-idempotency-rust-gates/combined-receipt.json
- echo-sdk-host/src/core_profile/facade/workflow.rs#optional_workflow_fields_reject_malformed_values
- echo-sdk-host/src/core_profile/facade/memory.rs#malformed_namespace_never_writes_and_replacement_store_is_live_authority
- echo-sdk-host/src/core_profile/facade/memory.rs#explicit_invalid_limits_are_not_defaulted
- echo-sdk-host/src/core_profile/facade/subagent.rs#collect_result
- echo-sdk-host/tests/core_profile_e2e.rs#subagent_short_await_preserves_result_and_dispatch_receipt
- echo-sdk-host/src/core_profile/facade/task_runtime.rs#request_live_interrupt
- echo-sdk-host/tests/core_profile_e2e.rs#task_pause_is_exact_and_resume_reuses_canonical_dependency_result
- echo-sdk-host/tests/core_profile_e2e.rs#task_cancel_is_exact_and_pending_sibling_continues
- echo-sdk-host/tests/core_profile_e2e.rs#task_rpc_shares_the_session_task_authority
- echo-sdk-host/tests/core_profile_e2e.rs#task_execute_and_control_settle_through_the_runtime
- echo-sdk-host/src/core_profile/facade/workflow.rs#agent_node_uses_replaced_live_llm_after_session_services_capture
- docs/sdk/governance/evidence/live-service-llm-proof/receipt.json
- docs/sdk/governance/evidence/live-service-llm-proof/independent-review.json
- docs/sdk/governance/evidence/live-service-llm-proof/manifest.json
- docs/sdk/governance/evidence/live-service-llm-proof/focused.log
- docs/sdk/governance/evidence/live-service-llm-proof/host-lib.log
- docs/sdk/governance/evidence/live-service-llm-proof/historical-30d/echo-sdk-host/src/core_profile/facade/workflow.rs
- docs/sdk/governance/evidence/live-service-llm-proof/historical-30d/echo-sdk-host/src/factory.rs
supports:
- behavior.sdk-task-subagent
- behavior.sdk-family-task
- behavior.sdk-family-subagent
- behavior.sdk-family-memory
- behavior.sdk-family-workflow
limitations:
- Original CLI author sourceaudit is distinguished from actual independent material/source
  slices; not renamed an independent new execution.
- Source restored in02059 is byte-identical to reviewed69b for these paths; current
  mutable factory/core tests or intentA implementation are excluded.
- ContextInheritance value tests do not prove three-language livehistory/memory inheritance
  E2E; no new context family/API is required.
- TaskContext getter consumption supports four exact Host cases and native runid only,
  not complete threefield reverseDTO.
- Memory/checkpoint backends are volatile in this accepted path, not crossprocessdurable;5second
  physicalB/12sourcekind and SDK main remain separate.
---

# Task/Subagent、Memory与Workflow有界观察

## 支持的结论

回收CLI原已完成实际只读8lens审查，无新增已证实源码缺陷，不伪由模型作者另做业务review。该组按现catalog task/subagent/memory/workflow四acceptedfamily定位；inheritance/PromptContext/compression属于既有source_operation/extbridge支持边界，不新增contextfamily。TaskRun/revision/claim/terminal同framework服务；Host只参数/metadata转换及handle；Memory每次调用liveAgentStore，WorkflowGraph/checkpoint原owner保一权威。

## 来源与范围

复核观察Git02059（10个原运行/测试文件与69b逐Gitblob一致），真实辅助native源进入02059后不再伪缺。原审函数范围：task_runtime123与四getter消费者tests845/943/992/1037；subagent91/193/381/404；memory99/333/396；workflow304/449/696；facade selector1828与sourceoperations3321/7298/7326/7575；bridgeE2Ecompressor3418。列出的factory/coretests当前A切片已不同，故本观察仍恢复原源码，不称最新A已被这组验证。

真实a634官方语言收据790inputs same/exit0，TaskJava4、WorkflowCheckpoint1、MemoryPermission1、LlmWorkflow3 XML均0failure/error/skip，各有原三语言对应场景。旧failedTaskHTTP夹具与受控Connectionclose同源前后结果保持自己的资料，不借Task4推其随机socket直接因果。四Host getter场景证明runid、waiver/nonwaivedmissingresult、nondefaultdelegation被真实Host消费；三nativeTask callback只加强runid来自SessionACP，不声称全部contextDTO三字段直接传CustomAgent。

## 已知缺口

ContextInheritance纯值support不替livehistory/memory继承三语E2E（现accepted路径源码/角色明确，补证不得造新API）；Hostcompressor注册消费不推每个hook reentry/cancel已经三native执行。内存Store/checkpoint无额外跨processdurability保证。5秒physicalB/旧12field source身份处置继续原Finding/待人，不由绿色Task场景riskaccept；新意图A幂等caller合同已人批准，其最新实现/原生gate与本原source审切片独立。scope外未接受构造器或全部unlistedsourceoperation不自动纳新完成分母。

## 四个原Task/字段逻辑缺陷的准确三链闭合

原CLI实际source作者事实与SDKcap有限逐defect独ref审必须分明，不凭全4counts推缺陷闭。对应原contextA/B/resume物理B的宽Finding不动，LiveStore半闭/WorkflowLLM替换未证不能借此closed。

### adapter-invalid-input

原namespace/tags/optional/limit malformed明确typed拒且mixedns零write；不将boundedpageclamp升级所有u32宽度无损承诺。准确postfixstrongcase echo-sdk-host/src/core_profile/facade/workflow.rs#optional_workflow_fields_reject_malformed_values, echo-sdk-host/src/core_profile/facade/memory.rs#malformed_namespace_never_writes_and_replacement_store_is_live_authority, echo-sdk-host/src/core_profile/facade/memory.rs#explicit_invalid_limits_are_not_defaulted，finalRust原05/10及native4的原source同cut已实际通过；原8lens业务与本closurefactreview分别记，observe622d为这些具名源码/已保存收据定位，不称旧命令重新运行。

### subagent-await-result

单Joincollector memo原resultreceipt，短timeout→重复fullawait同claim/attempt1/result/constraints不耗结果；非restart与全physical保证。准确postfixstrongcase echo-sdk-host/src/core_profile/facade/subagent.rs#collect_result, echo-sdk-host/tests/core_profile_e2e.rs#subagent_short_await_preserves_result_and_dispatch_receipt，finalRust原05/10及native4的原source同cut已实际通过；原8lens业务与本closurefactreview分别记，observe622d为这些具名源码/已保存收据定位，不称旧命令重新运行。

### task-control-locality

原rootcancel全图误推改为exactattempt/controlclaim；pauseA/BRunning/Cpending→B/C完成canonicalB后resumeA2/D1，cancelA兄弟继续，3native4完整输出/次数；Bphysical独立。准确postfixstrongcase echo-sdk-host/src/core_profile/facade/task_runtime.rs#request_live_interrupt, echo-sdk-host/tests/core_profile_e2e.rs#task_pause_is_exact_and_resume_reuses_canonical_dependency_result, echo-sdk-host/tests/core_profile_e2e.rs#task_cancel_is_exact_and_pending_sibling_continues，finalRust原05/10及native4的原source同cut已实际通过；原8lens业务与本closurefactreview分别记，observe622d为这些具名源码/已保存收据定位，不称旧命令重新运行。

### task-runtime-store-authority

SessionServices取liveAgenttaskrevisionservice同Arc，CRUD/runtime同framework service/store无fallback双图；nativeCRUD-onlyexecute featureUnavailable agentcalls0拒前effect/canonicalCAS4scene真实。准确postfixstrongcase echo-sdk-host/src/factory.rs, echo-sdk-host/src/core_profile/facade/task.rs, echo-sdk-host/src/core_profile/facade/task_runtime.rs, echo-sdk-host/tests/core_profile_e2e.rs#task_rpc_shares_the_session_task_authority, echo-sdk-host/tests/core_profile_e2e.rs#task_execute_and_control_settle_through_the_runtime，finalRust原05/10及native4的原source同cut已实际通过；原8lens业务与本closurefactreview分别记，observe622d为这些具名源码/已保存收据定位，不称旧命令重新运行。


这四原逻辑root可局部resolved，不等SDKmain；Taskresume早ACK子cause已闭但同一个knownBphysical残余仍保wideTaskFinding；cleanup/Sandbox各关联这个未决，不新三个重复policyauthority。原内存不crossprocess、12迁移/Afinal/newclientneg/sourcecuts/平台全gate独立。

## Live Store 与新建 Workflow 的真实替换消费补证

本次补齐原 Finding 的两个具体 live authority 义务。Memory 原具名测试在同 Agent 的 Store A→B 后实际读写 B，错误 namespace 零写，旧值与计数强断言已执行。Workflow 新具名测试先由原 SessionAuthorityServices 捕获同一个 AgentHandle，再通过既有公开 set_llm_client 将 A 替换成 B，随后经真实 Host graph.build 与框架 AgentNode 执行。A 的真实 stream 请求数为 0，B 为 1；完整 ChatRequest 保留原 task 与 Unicode system/user，completed graph state 输出 B 的 Unicode 回答与原任务、唯一路径。该证据不是 pure router 的 callback0，也不是 getter 观察。

修复生产路径是原 live Agent 服务读取；workflow 新 d516 文件只增加 cfg(test)，生产 prefix 9ae6 与原修复源一致。b7e6 是该代码与正式包的真实可恢复 Git 观察，不表示早期 Memory 命令重新执行。原 30d captured client/config 源已冻结，作为旧缺陷源码证据，未造旧产品执行红。两次新 fixture 错误（缺合法 owner、读错 state JSON 路径）明确保留。focused 1、Host lib 187/0 ignored、严格 Clippy 和 fmt/check 实际通过；sdk_capability_acceptance_audit 已独立读源码、真实断言和正式包，PASS C0/I0/M0。

此处仅证明替换后新建 graph 使用当前 provider，不承诺已建 graph 中途热换，不新增 SDK setter、RPC 或跨进程持久化。Memory 与 Workflow 原具体 captured authority 缺陷三链足够本地 resolved；B 物理结算、12 身份处置、平台与 SDK main 均独立。
原上文“Workflow LLM 替换未证”的句子为补证前的历史状态；本段只解除该具体缺证，不改变其余历史 scope。
