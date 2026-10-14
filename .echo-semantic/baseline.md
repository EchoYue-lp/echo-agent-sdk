---
{
  "schema_version": 1,
  "id": "baseline.repository",
  "kind": "baseline",
  "source_snapshot": {
    "base_revision": "30d7e219cc7d3aef0f75a19217c82898c363f50d",
    "content_digest": "0a13f994adc90d29746d23c7e071dfb55d8c12eb92c5000b1785092bdc10392f"
  },
  "inventory_closure": "open",
  "behavior_model_closure": "open",
  "map_refs": [
    "map.sdk-product-boundary",
    "map.sdk-acp",
    "map.sdk-agent-session-run",
    "map.sdk-task-subagent",
    "map.sdk-events-recovery",
    "map.sdk-resources-streams",
    "map.sdk-extension-bridge",
    "map.sdk-integrations",
    "map.sdk-language-clients"
  ],
  "regions": [
    {
      "path": ".cargo",
      "status": "supporting"
    },
    {
      "path": ".github",
      "status": "supporting"
    },
    {
      "path": ".gitignore",
      "status": "supporting"
    },
    {
      "path": "AGENTS.md",
      "status": "supporting"
    },
    {
      "path": "Cargo.lock",
      "status": "supporting"
    },
    {
      "path": "Cargo.toml",
      "status": "in_scope"
    },
    {
      "path": "LICENSE",
      "status": "supporting"
    },
    {
      "path": "MIGRATION-SOURCE.md",
      "status": "supporting"
    },
    {
      "path": "README.md",
      "status": "supporting"
    },
    {
      "path": "README.zh.md",
      "status": "supporting"
    },
    {
      "path": "contracts",
      "status": "in_scope"
    },
    {
      "path": "deny.toml",
      "status": "supporting"
    },
    {
      "path": "docs",
      "status": "supporting"
    },
    {
      "path": "echo-sdk-host",
      "status": "in_scope"
    },
    {
      "path": "echo-sdk-protocol",
      "status": "in_scope"
    },
    {
      "path": "rust-toolchain.toml",
      "status": "supporting"
    },
    {
      "path": "scripts",
      "status": "supporting"
    },
    {
      "path": "sdks",
      "status": "in_scope"
    }
  ],
  "boundaries": [
    {
      "id": "boundary.sdk-product",
      "map_ref": "map.sdk-product-boundary",
      "risk": "high"
    },
    {
      "id": "boundary.sdk-acp",
      "map_ref": "map.sdk-acp",
      "risk": "high"
    },
    {
      "id": "boundary.sdk-agent-session-run",
      "map_ref": "map.sdk-agent-session-run",
      "risk": "high"
    },
    {
      "id": "boundary.sdk-events-recovery",
      "map_ref": "map.sdk-events-recovery",
      "risk": "high"
    },
    {
      "id": "boundary.sdk-extension-bridge",
      "map_ref": "map.sdk-extension-bridge",
      "risk": "high"
    },
    {
      "id": "boundary.sdk-integrations",
      "map_ref": "map.sdk-integrations",
      "risk": "high"
    },
    {
      "id": "boundary.sdk-language-clients",
      "map_ref": "map.sdk-language-clients",
      "risk": "high"
    },
    {
      "id": "boundary.sdk-resources-streams",
      "map_ref": "map.sdk-resources-streams",
      "risk": "high"
    },
    {
      "id": "boundary.sdk-task-subagent",
      "map_ref": "map.sdk-task-subagent",
      "risk": "high"
    }
  ],
  "coverage": [
    {
      "region": "echo-sdk-host",
      "lens": "trigger_input",
      "status": "needs_review",
      "unknown": "Host complete entrypoint inventory remains open"
    },
    {
      "region": "echo-sdk-host",
      "lens": "result_side_effect",
      "status": "needs_review",
      "unknown": "Host side-effect settlement inventory remains open"
    },
    {
      "region": "echo-sdk-host",
      "lens": "state_authority",
      "status": "needs_review",
      "unknown": "Host adapter authority review remains open"
    },
    {
      "region": "echo-sdk-host",
      "lens": "data_durability",
      "status": "needs_review",
      "unknown": "Host durability boundary review remains open"
    },
    {
      "region": "echo-sdk-host",
      "lens": "time_lifecycle",
      "status": "needs_review",
      "unknown": "Host lifecycle inventory remains open"
    },
    {
      "region": "echo-sdk-host",
      "lens": "failure_concurrency",
      "status": "needs_review",
      "unknown": "Host concurrency and recovery review remains open"
    },
    {
      "region": "echo-sdk-host",
      "lens": "permission_external",
      "status": "needs_review",
      "unknown": "Host external-effect and permission review remains open"
    },
    {
      "region": "echo-sdk-host",
      "lens": "contract_evidence",
      "status": "needs_review",
      "unknown": "Host contract evidence inventory remains open"
    }
  ]
}
---

# echo-agent-sdk 语义基线

## 源码快照

当前观察绑定SDK治理工作树、可恢复base30d7与排除.semantic后的实际源码摘要。历史Candidate receipts已仓库原字节归档；observed_at current仅定位材料观察，不重命名原fc63/6711/cb9/c52运行或声明最终pin/main。

## 仓库区域

Rust protocol/Host、合同和三语言 SDK 是生产范围；文档、CI、脚本和根配置是支撑范围。

## 能力图与边界

SDK ownership、protocol/Host边界及明确accepted合同已映射。八组主路径与41个family已逐项展开入口、唯一权威、生命周期、失败处置、具名证据或开放原因；主路径mapped不等于Behavior verified或全部风险lens covered。

## 覆盖网格

库存和行为模型保持 open，尚未声称八个风险视角全覆盖。

## 未知与缺口

capability inventory与八视角审查保持open，动态入口见Discovery；缺口按Finding和有限能力处置表列明。历史protocol/pin/contract验证只按其原证据snapshot读取，不能外推当前后继全套通过。

## 闭合结论

该基线可用于迁移预检和严格源码快照校验；独立 Host、合同与三语言门禁由
`evidence.sdk-framework-pin` 证明，发布状态仍由后续仓库交付决定。

## 当前观察保持开放

41个family Behavior与风险lens仍needs_review，八组主路径mapped仅记录已审模型。Finding按自己的repair/verification/rereview处置；family route和stream局部bug已resolved，#120新ACK窗口与Channel inbound等缺口保留open。历史sourceepoch不可恢复时保存原Evidence及原始日志/输入hash，不制造Git提交。当前历史审计只引用可恢复归档；changedworkflow/tools拥有各自有界successorEvidence，新Channelinbound明确staticFinding。正式accepted supporting字段生成、无patch独立build、最后已接受合同的具名失败义务、真实publishedpin/CI/main仍未完成。
