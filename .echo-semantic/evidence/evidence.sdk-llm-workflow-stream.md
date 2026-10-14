---
schema_version: 1
id: evidence.sdk-llm-workflow-stream
kind: evidence
observed_at: 56929154c1851473e9e4837901b610cecf001b2d
source_refs:
- docs/sdk/governance/evidence/llm-workflow-stream/raw/receipt.json
- docs/sdk/governance/evidence/llm-workflow-stream/README.md
- docs/sdk/governance/evidence/sdk-successor-model-reviews/llm-workflow-stream.json
- echo-sdk-host/src/core_profile/extension_bridge.rs
- sdks/shared/testing/llm_workflow_stream_fixture.py
- echo-sdk-host/src/core_profile/facade/workflow.rs
- docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/final-receipt.json
- docs/sdk/governance/evidence/sdk-language-factory-final/receipt.json
- docs/sdk/governance/evidence/sdk-language-factory-final/source-before.json
- docs/sdk/governance/evidence/sdk-language-factory-final/source-after.json
supports:
- behavior.sdk-family-workflow
- behavior.sdk-family-bridge
- behavior.sdk-resources-streams
- behavior.sdk-extension-bridge
limitations:
- Frozen0744 raw Workflow handler does not prove Python typedcall streamprojection
- New Python typedhelper repair must have its own red/green/rereview
- Final wholeSDK and acceptedgeneration separate
---

# llm-workflow-stream

## 支持的结论

sdk_capability_acceptance_audit原独立limitedpass0，由parent回收：0744冻结输入，26raw/19source SHA核；18LLMcallback/finally、3cancel，15Workflowfinally/6producer、3graphguard0、剩7Workflowverbs真实可达。另发现Python typed AgentComponentCall丢stream，独立SpecificFinding保open，不以raw回调绿代typedhelper。

三语言各3groups，原typedLLM完整usage/UTF8/chunk/finish；Workflow rawhandler生产与取消、wrongowner/cancel guard保证无callback。旧close/retry false因anchor级联已closed，不编造合同错误。

## 来源与范围

原归档receipt/输入hash/二进制保持不变。当前观察只绑定模型与可读取归档，不把原epoch冒称最终current全套通过。

## 已知缺口

Frozen0744 raw Workflow handler does not prove Python typedcall streamprojection；New Python typedhelper repair must have its own red/green/rereview；Final wholeSDK and acceptedgeneration separate。

## 对原保存收据的当前审计观察

本对象observed_at现在指真正当前候选Git `56929154c1851473e9e4837901b610cecf001b2d`，只定位已经保存的收据、输入源码副本与manifest的审计观察。原观测字段为`source:pending`；它若是source摘要或pending，均不被伪装为历史执行当时的可恢复整仓Git。当前观察不宣称原运行发生于Git569、不重跑或升级原runtimeepoch。

全部上列source_refs均从该Gitblob实际读取，与当前archive原字节SHA精确相同，原锚点存在；原日志时间、HostbinarySHA、sourcefreeze、错误/通过计数及正文来源保持。原body的epoch/signature/cause限制与supports均为原Behavior义务的历史有界支持，不能由此宣布当前源码verified；旧3ea/9e没有完整Git时仅有原保存收据可恢复，整仓原epoch仍不可恢复，不补造树。

原保存artifact中的具名元数据（完整值仍见原JSON，不改变）：`recorded_at_utc=2026-10-13T15:24:23.578185+00:00`。

本次只附加观察说明；原正文/supports/limitations保留，原rawReceipt/ArchiveChecksum/运行时间未写入，任何未决unknown/失败仍不自动resolved。源码业务Audit不是此收据观察，另队列保持原revision与复审义务，不能偷换类型或scope。

## 本次已复审scope的当前证据重观察

当前观察Git`56929154c1851473e9e4837901b610cecf001b2d`仅据下面明确源码业务复审的已接受路径，回收UTC`2026-10-14T00:39:15.355735+00:00`；原marker`56929154c1851473e9e4837901b610cecf001b2d`、旧runtimeepoch/time/sourcehash/raw全文保留，原support/limits不变。不是过去执行就发生569，不假历史整仓Git恢复。

- `audit.sdk-llm-workflow-stream-rereview` reader`sdk_capability_acceptance_audit`：**PASS / 0C0I0M（本次有界源码业务复审）**；本次原故障检查ExactLLMmessage/usage/finish/optional/nativechunks+finally,missingfinish/failure/cancel consistentterminal；Workfloworiginalgraphstream/result/owner/wrongcancel no callbacks,sevenacceptedverbs realconsumer;do not infer genericfullcontextDTO/extraAPI。after仅Currentnewbb4f LlmWorkflowStreamHostTest 3groups each;reviewtypedhelper/earlyACK routing currentdiff andFactory8e7 actualtwofacts notold43ec claim。

所有新SDKsource/proof refs按真实569byte/anchor读取，原业务与原sourcefreeze记录保持；这不是一切currentfamily/runtimepassed。ordinaryTools已有原五族收据只新增本次明确Database/Artifact检查，不暗称其余方言/provider/工具所有field均新审；原candidate supports=[]不升。任何缺原完整time/tree仍在原limits，12/B/depWarn人未答/fullgeneration/CImain待。
