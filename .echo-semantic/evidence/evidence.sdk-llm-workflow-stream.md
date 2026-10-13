---
schema_version: 1
id: evidence.sdk-llm-workflow-stream
kind: evidence
observed_at: source:pending
source_refs:
- docs/sdk/governance/evidence/llm-workflow-stream/raw/receipt.json
- docs/sdk/governance/evidence/llm-workflow-stream/README.md
- docs/sdk/governance/evidence/sdk-successor-model-reviews/llm-workflow-stream.json
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
