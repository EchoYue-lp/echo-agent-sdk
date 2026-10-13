---
schema_version: 1
id: evidence.sdk-python-agent-component-stream-projection
kind: evidence
observed_at: source:pending
source_refs:
- sdks/python/src/echo_agent_sdk/client.py
- sdks/python/README.md
- docs/sdk/governance/evidence/sdk-successor-model-reviews/llm-workflow-stream.json
supports: []
limitations:
- Original raw workflow proof does not prove typedhelper
- Repair source exists in progress; no green/rereview claimed here
---

# python-agent-component-stream-projection

## 支持的结论

独立审查已实际解码合法stream payload，原typed AgentComponentCall成功却hasattr(stream/raw)=false；这是静态可执行原只读反例，不伪称Host红绿。父已指派既有optionalWireHandle窄修，最终验证与rereview待补。

## 来源与范围

原归档receipt/输入hash/二进制保持不变。当前观察只绑定模型与可读取归档，不把原epoch冒称最终current全套通过。

## 已知缺口

Original raw workflow proof does not prove typedhelper；Repair source exists in progress; no green/rereview claimed here。
