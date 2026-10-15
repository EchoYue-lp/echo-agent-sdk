---
{
  "schema_version": 1,
  "id": "baseline.repository",
  "kind": "baseline",
  "source_snapshot": {
    "base_revision": "30d7e219cc7d3aef0f75a19217c82898c363f50d",
    "content_digest": "6edc2e168a20af9b7a2c7c16552f6b7db536d2af41ed0a95b3e65b9994d0d897"
  },
  "inventory_closure": "closed",
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
      "path": ".gitattributes",
      "status": "supporting"
    },
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
      "status": "covered",
      "refs": [
        "audit.sdk-events-resources-finite-trigger-input",
        "audit.sdk-task-memory-workflow-finite-trigger-input",
        "audit.sdk-tools-bridge-finite-trigger-input",
        "audit.sdk-integrations-finite-trigger-input",
        "audit.sdk-acp-agent-run-finite-trigger-input",
        "map.sdk-agent-session-run",
        "map.sdk-task-subagent",
        "map.sdk-events-recovery",
        "map.sdk-resources-streams",
        "map.sdk-integrations",
        "map.sdk-extension-bridge"
      ],
      "review_scope": "Host具名主路径：Agent/Session/Run、Task/Subagent、events/resources、builtins/reverse和integrations各有限故障假设已实际审；这里只投影已审范围。",
      "limitations": "B>5秒物理工作未停止的明确缺陷仍open；不以合作取消、局部cleanup、最新whole计数反证B；12/平台/main单独。"
    },
    {
      "region": "echo-sdk-host",
      "lens": "result_side_effect",
      "status": "covered",
      "refs": [
        "audit.sdk-events-resources-finite-result-side-effect",
        "audit.sdk-task-memory-workflow-finite-result-side-effect",
        "audit.sdk-tools-bridge-finite-result-side-effect",
        "audit.sdk-integrations-finite-result-side-effect",
        "audit.sdk-acp-agent-run-finite-result-side-effect",
        "map.sdk-agent-session-run",
        "map.sdk-task-subagent",
        "map.sdk-events-recovery",
        "map.sdk-resources-streams",
        "map.sdk-integrations",
        "map.sdk-extension-bridge"
      ],
      "review_scope": "Host具名主路径：Agent/Session/Run、Task/Subagent、events/resources、builtins/reverse和integrations各有限故障假设已实际审；这里只投影已审范围。",
      "limitations": "B>5秒物理工作未停止的明确缺陷仍open；不以合作取消、局部cleanup、最新whole计数反证B；12/平台/main单独。"
    },
    {
      "region": "echo-sdk-host",
      "lens": "state_authority",
      "status": "covered",
      "refs": [
        "audit.sdk-events-resources-finite-state-authority",
        "audit.sdk-task-memory-workflow-finite-state-authority",
        "audit.sdk-tools-bridge-finite-state-authority",
        "audit.sdk-integrations-finite-state-authority",
        "audit.sdk-acp-agent-run-finite-state-authority",
        "map.sdk-agent-session-run",
        "map.sdk-task-subagent",
        "map.sdk-events-recovery",
        "map.sdk-resources-streams",
        "map.sdk-integrations",
        "map.sdk-extension-bridge"
      ],
      "review_scope": "Host具名主路径：Agent/Session/Run、Task/Subagent、events/resources、builtins/reverse和integrations各有限故障假设已实际审；这里只投影已审范围。",
      "limitations": "B>5秒物理工作未停止的明确缺陷仍open；不以合作取消、局部cleanup、最新whole计数反证B；12/平台/main单独。"
    },
    {
      "region": "echo-sdk-host",
      "lens": "data_durability",
      "status": "covered",
      "refs": [
        "audit.sdk-events-resources-finite-data-durability",
        "audit.sdk-task-memory-workflow-finite-data-durability",
        "audit.sdk-tools-bridge-finite-data-durability",
        "audit.sdk-integrations-finite-data-durability",
        "audit.sdk-acp-agent-run-finite-data-durability",
        "map.sdk-agent-session-run",
        "map.sdk-task-subagent",
        "map.sdk-events-recovery",
        "map.sdk-resources-streams",
        "map.sdk-integrations",
        "map.sdk-extension-bridge"
      ],
      "review_scope": "Host具名主路径：Agent/Session/Run、Task/Subagent、events/resources、builtins/reverse和integrations各有限故障假设已实际审；这里只投影已审范围。",
      "limitations": "B>5秒物理工作未停止的明确缺陷仍open；不以合作取消、局部cleanup、最新whole计数反证B；12/平台/main单独。"
    },
    {
      "region": "echo-sdk-host",
      "lens": "time_lifecycle",
      "status": "covered",
      "refs": [
        "audit.sdk-events-resources-finite-time-lifecycle",
        "audit.sdk-task-memory-workflow-finite-time-lifecycle",
        "audit.sdk-tools-bridge-finite-time-lifecycle",
        "audit.sdk-integrations-finite-time-lifecycle",
        "audit.sdk-acp-agent-run-finite-time-lifecycle",
        "map.sdk-agent-session-run",
        "map.sdk-task-subagent",
        "map.sdk-events-recovery",
        "map.sdk-resources-streams",
        "map.sdk-integrations",
        "map.sdk-extension-bridge"
      ],
      "review_scope": "Host具名主路径：Agent/Session/Run、Task/Subagent、events/resources、builtins/reverse和integrations各有限故障假设已实际审；这里只投影已审范围。",
      "limitations": "B>5秒物理工作未停止的明确缺陷仍open；不以合作取消、局部cleanup、最新whole计数反证B；12/平台/main单独。"
    },
    {
      "region": "echo-sdk-host",
      "lens": "failure_concurrency",
      "status": "covered",
      "refs": [
        "audit.sdk-events-resources-finite-failure-concurrency",
        "audit.sdk-task-memory-workflow-finite-failure-concurrency",
        "audit.sdk-tools-bridge-finite-failure-concurrency",
        "audit.sdk-integrations-finite-failure-concurrency",
        "audit.sdk-acp-agent-run-finite-failure-concurrency",
        "map.sdk-agent-session-run",
        "map.sdk-task-subagent",
        "map.sdk-events-recovery",
        "map.sdk-resources-streams",
        "map.sdk-integrations",
        "map.sdk-extension-bridge"
      ],
      "review_scope": "Host具名主路径：Agent/Session/Run、Task/Subagent、events/resources、builtins/reverse和integrations各有限故障假设已实际审；这里只投影已审范围。",
      "limitations": "B>5秒物理工作未停止的明确缺陷仍open；不以合作取消、局部cleanup、最新whole计数反证B；12/平台/main单独。"
    },
    {
      "region": "echo-sdk-host",
      "lens": "permission_external",
      "status": "covered",
      "refs": [
        "audit.sdk-events-resources-finite-permission-external",
        "audit.sdk-task-memory-workflow-finite-permission-external",
        "audit.sdk-tools-bridge-finite-permission-external",
        "audit.sdk-integrations-finite-permission-external",
        "audit.sdk-acp-agent-run-finite-permission-external",
        "map.sdk-agent-session-run",
        "map.sdk-task-subagent",
        "map.sdk-events-recovery",
        "map.sdk-resources-streams",
        "map.sdk-integrations",
        "map.sdk-extension-bridge"
      ],
      "review_scope": "Host具名主路径：Agent/Session/Run、Task/Subagent、events/resources、builtins/reverse和integrations各有限故障假设已实际审；这里只投影已审范围。",
      "limitations": "B>5秒物理工作未停止的明确缺陷仍open；不以合作取消、局部cleanup、最新whole计数反证B；12/平台/main单独。"
    },
    {
      "region": "echo-sdk-host",
      "lens": "contract_evidence",
      "status": "covered",
      "refs": [
        "audit.sdk-events-resources-finite-contract-evidence",
        "audit.sdk-task-memory-workflow-finite-contract-evidence",
        "audit.sdk-tools-bridge-finite-contract-evidence",
        "audit.sdk-integrations-finite-contract-evidence",
        "audit.sdk-acp-agent-run-finite-contract-evidence",
        "map.sdk-agent-session-run",
        "map.sdk-task-subagent",
        "map.sdk-events-recovery",
        "map.sdk-resources-streams",
        "map.sdk-integrations",
        "map.sdk-extension-bridge"
      ],
      "review_scope": "Host具名主路径：Agent/Session/Run、Task/Subagent、events/resources、builtins/reverse和integrations各有限故障假设已实际审；这里只投影已审范围。",
      "limitations": "B>5秒物理工作未停止的明确缺陷仍open；不以合作取消、局部cleanup、最新whole计数反证B；12/平台/main单独。"
    },
    {
      "region": "echo-sdk-protocol",
      "lens": "trigger_input",
      "status": "covered",
      "refs": [
        "audit.sdk-protocol-finite-trigger-input",
        "evidence.sdk-protocol-finite-observation",
        "map.sdk-acp"
      ],
      "review_scope": "Protocol纯合同/codec、协商/feature/error/wire与已列官方ACP消费者；对应SDKcap原8lens真实源码审和修后广告/初始化子切片。",
      "limitations": "纯值持久含义仅字段/编码与可恢复源；不拥有Host执行终态/业务store。Windows native未执行，完整producer12未批准。"
    },
    {
      "region": "echo-sdk-protocol",
      "lens": "result_side_effect",
      "status": "covered",
      "refs": [
        "audit.sdk-protocol-finite-result-side-effect",
        "evidence.sdk-protocol-finite-observation",
        "map.sdk-acp"
      ],
      "review_scope": "Protocol纯合同/codec、协商/feature/error/wire与已列官方ACP消费者；对应SDKcap原8lens真实源码审和修后广告/初始化子切片。",
      "limitations": "纯值持久含义仅字段/编码与可恢复源；不拥有Host执行终态/业务store。Windows native未执行，完整producer12未批准。"
    },
    {
      "region": "echo-sdk-protocol",
      "lens": "state_authority",
      "status": "covered",
      "refs": [
        "audit.sdk-protocol-finite-state-authority",
        "evidence.sdk-protocol-finite-observation",
        "map.sdk-acp"
      ],
      "review_scope": "Protocol纯合同/codec、协商/feature/error/wire与已列官方ACP消费者；对应SDKcap原8lens真实源码审和修后广告/初始化子切片。",
      "limitations": "纯值持久含义仅字段/编码与可恢复源；不拥有Host执行终态/业务store。Windows native未执行，完整producer12未批准。"
    },
    {
      "region": "echo-sdk-protocol",
      "lens": "data_durability",
      "status": "covered",
      "refs": [
        "audit.sdk-protocol-finite-data-durability",
        "evidence.sdk-protocol-finite-observation",
        "map.sdk-acp"
      ],
      "review_scope": "Protocol纯合同/codec、协商/feature/error/wire与已列官方ACP消费者；对应SDKcap原8lens真实源码审和修后广告/初始化子切片。",
      "limitations": "纯值持久含义仅字段/编码与可恢复源；不拥有Host执行终态/业务store。Windows native未执行，完整producer12未批准。"
    },
    {
      "region": "echo-sdk-protocol",
      "lens": "time_lifecycle",
      "status": "covered",
      "refs": [
        "audit.sdk-protocol-finite-time-lifecycle",
        "evidence.sdk-protocol-finite-observation",
        "map.sdk-acp"
      ],
      "review_scope": "Protocol纯合同/codec、协商/feature/error/wire与已列官方ACP消费者；对应SDKcap原8lens真实源码审和修后广告/初始化子切片。",
      "limitations": "纯值持久含义仅字段/编码与可恢复源；不拥有Host执行终态/业务store。Windows native未执行，完整producer12未批准。"
    },
    {
      "region": "echo-sdk-protocol",
      "lens": "failure_concurrency",
      "status": "covered",
      "refs": [
        "audit.sdk-protocol-finite-failure-concurrency",
        "evidence.sdk-protocol-finite-observation",
        "map.sdk-acp"
      ],
      "review_scope": "Protocol纯合同/codec、协商/feature/error/wire与已列官方ACP消费者；对应SDKcap原8lens真实源码审和修后广告/初始化子切片。",
      "limitations": "纯值持久含义仅字段/编码与可恢复源；不拥有Host执行终态/业务store。Windows native未执行，完整producer12未批准。"
    },
    {
      "region": "echo-sdk-protocol",
      "lens": "permission_external",
      "status": "covered",
      "refs": [
        "audit.sdk-protocol-finite-permission-external",
        "evidence.sdk-protocol-finite-observation",
        "map.sdk-acp"
      ],
      "review_scope": "Protocol纯合同/codec、协商/feature/error/wire与已列官方ACP消费者；对应SDKcap原8lens真实源码审和修后广告/初始化子切片。",
      "limitations": "纯值持久含义仅字段/编码与可恢复源；不拥有Host执行终态/业务store。Windows native未执行，完整producer12未批准。"
    },
    {
      "region": "echo-sdk-protocol",
      "lens": "contract_evidence",
      "status": "covered",
      "refs": [
        "audit.sdk-protocol-finite-contract-evidence",
        "evidence.sdk-protocol-finite-observation",
        "map.sdk-acp"
      ],
      "review_scope": "Protocol纯合同/codec、协商/feature/error/wire与已列官方ACP消费者；对应SDKcap原8lens真实源码审和修后广告/初始化子切片。",
      "limitations": "纯值持久含义仅字段/编码与可恢复源；不拥有Host执行终态/业务store。Windows native未执行，完整producer12未批准。"
    },
    {
      "region": "sdks",
      "lens": "trigger_input",
      "status": "covered",
      "refs": [
        "audit.sdk-language-client-lifecycle-trigger-input",
        "audit.sdk-protocol-finite-trigger-input",
        "evidence.sdk-language-client-lifecycle-current",
        "evidence.sdk-protocol-finite-observation",
        "map.sdk-language-clients"
      ],
      "review_scope": "三客户端官方transport、DTO/error/wire、ACK消费、factory与close-owner具名审查，加622d协商修复和真实初始化退出事实。",
      "limitations": "不把客户端typedcause保证泛化到Rust转换；旧运行receipt按原cut，最新完整语言另ref；Windows未验/main未交付。"
    },
    {
      "region": "sdks",
      "lens": "result_side_effect",
      "status": "covered",
      "refs": [
        "audit.sdk-language-client-lifecycle-result-side-effect",
        "audit.sdk-protocol-finite-result-side-effect",
        "evidence.sdk-language-client-lifecycle-current",
        "evidence.sdk-protocol-finite-observation",
        "map.sdk-language-clients"
      ],
      "review_scope": "三客户端官方transport、DTO/error/wire、ACK消费、factory与close-owner具名审查，加622d协商修复和真实初始化退出事实。",
      "limitations": "不把客户端typedcause保证泛化到Rust转换；旧运行receipt按原cut，最新完整语言另ref；Windows未验/main未交付。"
    },
    {
      "region": "sdks",
      "lens": "state_authority",
      "status": "covered",
      "refs": [
        "audit.sdk-language-client-lifecycle-state-authority",
        "audit.sdk-protocol-finite-state-authority",
        "evidence.sdk-language-client-lifecycle-current",
        "evidence.sdk-protocol-finite-observation",
        "map.sdk-language-clients"
      ],
      "review_scope": "三客户端官方transport、DTO/error/wire、ACK消费、factory与close-owner具名审查，加622d协商修复和真实初始化退出事实。",
      "limitations": "不把客户端typedcause保证泛化到Rust转换；旧运行receipt按原cut，最新完整语言另ref；Windows未验/main未交付。"
    },
    {
      "region": "sdks",
      "lens": "data_durability",
      "status": "covered",
      "refs": [
        "audit.sdk-language-client-lifecycle-data-durability",
        "audit.sdk-protocol-finite-data-durability",
        "evidence.sdk-language-client-lifecycle-current",
        "evidence.sdk-protocol-finite-observation",
        "map.sdk-language-clients"
      ],
      "review_scope": "三客户端官方transport、DTO/error/wire、ACK消费、factory与close-owner具名审查，加622d协商修复和真实初始化退出事实。",
      "limitations": "不把客户端typedcause保证泛化到Rust转换；旧运行receipt按原cut，最新完整语言另ref；Windows未验/main未交付。"
    },
    {
      "region": "sdks",
      "lens": "time_lifecycle",
      "status": "covered",
      "refs": [
        "audit.sdk-language-client-lifecycle-time-lifecycle",
        "audit.sdk-protocol-finite-time-lifecycle",
        "evidence.sdk-language-client-lifecycle-current",
        "evidence.sdk-protocol-finite-observation",
        "map.sdk-language-clients"
      ],
      "review_scope": "三客户端官方transport、DTO/error/wire、ACK消费、factory与close-owner具名审查，加622d协商修复和真实初始化退出事实。",
      "limitations": "不把客户端typedcause保证泛化到Rust转换；旧运行receipt按原cut，最新完整语言另ref；Windows未验/main未交付。"
    },
    {
      "region": "sdks",
      "lens": "failure_concurrency",
      "status": "covered",
      "refs": [
        "audit.sdk-language-client-lifecycle-failure-concurrency",
        "audit.sdk-protocol-finite-failure-concurrency",
        "evidence.sdk-language-client-lifecycle-current",
        "evidence.sdk-protocol-finite-observation",
        "map.sdk-language-clients"
      ],
      "review_scope": "三客户端官方transport、DTO/error/wire、ACK消费、factory与close-owner具名审查，加622d协商修复和真实初始化退出事实。",
      "limitations": "不把客户端typedcause保证泛化到Rust转换；旧运行receipt按原cut，最新完整语言另ref；Windows未验/main未交付。"
    },
    {
      "region": "sdks",
      "lens": "permission_external",
      "status": "covered",
      "refs": [
        "audit.sdk-language-client-lifecycle-permission-external",
        "audit.sdk-protocol-finite-permission-external",
        "evidence.sdk-language-client-lifecycle-current",
        "evidence.sdk-protocol-finite-observation",
        "map.sdk-language-clients"
      ],
      "review_scope": "三客户端官方transport、DTO/error/wire、ACK消费、factory与close-owner具名审查，加622d协商修复和真实初始化退出事实。",
      "limitations": "不把客户端typedcause保证泛化到Rust转换；旧运行receipt按原cut，最新完整语言另ref；Windows未验/main未交付。"
    },
    {
      "region": "sdks",
      "lens": "contract_evidence",
      "status": "covered",
      "refs": [
        "audit.sdk-language-client-lifecycle-contract-evidence",
        "audit.sdk-protocol-finite-contract-evidence",
        "evidence.sdk-language-client-lifecycle-current",
        "evidence.sdk-protocol-finite-observation",
        "map.sdk-language-clients"
      ],
      "review_scope": "三客户端官方transport、DTO/error/wire、ACK消费、factory与close-owner具名审查，加622d协商修复和真实初始化退出事实。",
      "limitations": "不把客户端typedcause保证泛化到Rust转换；旧运行receipt按原cut，最新完整语言另ref；Windows未验/main未交付。"
    },
    {
      "region": "contracts",
      "lens": "trigger_input",
      "status": "covered",
      "refs": [
        "audit.sdk-protocol-finite-contract-evidence",
        "evidence.sdk-protocol-finite-observation",
        "map.sdk-product-boundary",
        "finding.sdk-accepted-source-continuity"
      ],
      "review_scope": "精确 defining-source/ItemKind seed 与已批准 delta 校验；新路由、错kind、unknown新增、draft/substr退役反例已真实审。",
      "limitations": "InventoryAcceptance旧autoaccept逻辑局部已修资格，与12 accepted-source continuity待人选择独立；不自动retire、不声称full28生成通过。"
    },
    {
      "region": "contracts",
      "lens": "result_side_effect",
      "status": "covered",
      "refs": [
        "audit.sdk-protocol-finite-contract-evidence",
        "evidence.sdk-protocol-finite-observation",
        "map.sdk-product-boundary",
        "finding.sdk-accepted-source-continuity"
      ],
      "review_scope": "producer apply 在正式输出写入前；invalid/missing seed 原子拒绝不 mutation；不承诺任意输出I/O全部事务。",
      "limitations": "InventoryAcceptance旧autoaccept逻辑局部已修资格，与12 accepted-source continuity待人选择独立；不自动retire、不声称full28生成通过。"
    },
    {
      "region": "contracts",
      "lens": "state_authority",
      "status": "covered",
      "refs": [
        "audit.sdk-protocol-finite-contract-evidence",
        "evidence.sdk-protocol-finite-observation",
        "map.sdk-product-boundary",
        "finding.sdk-accepted-source-continuity"
      ],
      "review_scope": "唯一 accepted seed/canonical producer；inventory telemetry、alias/signature变化不接新能力，完整缺失正确阻写。",
      "limitations": "InventoryAcceptance旧autoaccept逻辑局部已修资格，与12 accepted-source continuity待人选择独立；不自动retire、不声称full28生成通过。"
    },
    {
      "region": "contracts",
      "lens": "data_durability",
      "status": "covered",
      "refs": [
        "audit.sdk-protocol-finite-contract-evidence",
        "evidence.sdk-protocol-finite-observation",
        "map.sdk-product-boundary",
        "finding.sdk-accepted-source-continuity"
      ],
      "review_scope": "已提交 seed/catalog/digest 属可恢复 artifact；snapshot tags 与 fresh remote proof保各观察版本，不是运行持久化。",
      "limitations": "InventoryAcceptance旧autoaccept逻辑局部已修资格，与12 accepted-source continuity待人选择独立；不自动retire、不声称full28生成通过。"
    },
    {
      "region": "contracts",
      "lens": "time_lifecycle",
      "status": "covered",
      "refs": [
        "audit.sdk-protocol-finite-contract-evidence",
        "evidence.sdk-protocol-finite-observation",
        "map.sdk-product-boundary",
        "finding.sdk-accepted-source-continuity"
      ],
      "review_scope": "CLI生成一次性验证→apply→输出；没有运行调度器、租约或后台状态；生命周期仅生成前失败不写。",
      "limitations": "InventoryAcceptance旧autoaccept逻辑局部已修资格，与12 accepted-source continuity待人选择独立；不自动retire、不声称full28生成通过。"
    },
    {
      "region": "contracts",
      "lens": "failure_concurrency",
      "status": "covered",
      "refs": [
        "audit.sdk-protocol-finite-contract-evidence",
        "evidence.sdk-protocol-finite-observation",
        "map.sdk-product-boundary",
        "finding.sdk-accepted-source-continuity"
      ],
      "review_scope": "错delta/缺失 source 在 mutation 前拒绝，完整 missing集合与 partial retire不掩余项；没有新增并发生成序列化承诺。",
      "limitations": "InventoryAcceptance旧autoaccept逻辑局部已修资格，与12 accepted-source continuity待人选择独立；不自动retire、不声称full28生成通过。"
    },
    {
      "region": "contracts",
      "lens": "permission_external",
      "status": "covered",
      "refs": [
        "audit.sdk-protocol-finite-contract-evidence",
        "evidence.sdk-protocol-finite-observation",
        "map.sdk-product-boundary",
        "finding.sdk-accepted-source-continuity"
      ],
      "review_scope": "精确human-approved接受边界；输出仅用户指定本地artifact，无新增网络/运行执行权限；12source仍未批准。",
      "limitations": "InventoryAcceptance旧autoaccept逻辑局部已修资格，与12 accepted-source continuity待人选择独立；不自动retire、不声称full28生成通过。"
    },
    {
      "region": "contracts",
      "lens": "contract_evidence",
      "status": "covered",
      "refs": [
        "audit.sdk-protocol-finite-contract-evidence",
        "evidence.sdk-protocol-finite-observation",
        "map.sdk-product-boundary",
        "finding.sdk-accepted-source-continuity"
      ],
      "review_scope": "具名八scope及producer测试真实05/08，当前seed不改；库存数不作完成分母，full28未通过不冒通过。",
      "limitations": "InventoryAcceptance旧autoaccept逻辑局部已修资格，与12 accepted-source continuity待人选择独立；不自动retire、不声称full28生成通过。"
    },
    {
      "region": "Cargo.toml",
      "lens": "trigger_input",
      "status": "covered",
      "refs": [
        "audit.sdk-feature-wiring-rereview",
        "audit.sdk-dependency-duplicate-policy-rereview",
        "evidence.sdk-current-reviewed-rust-gates",
        "evidence.sdk-feature-wiring-verification",
        "evidence.sdk-dependency-duplicate-policy-verification",
        "evidence.sdk-protocol-finite-observation",
        "map.sdk-product-boundary"
      ],
      "review_scope": "根manifest精确members/default-members为Protocol和Host、resolver3；不是动态输入validator。原feature接线与后继正式14配置门禁覆盖真实workspace构建准入。",
      "limitations": "仅根workspace构建元数据与已审build/dependency/feature证据；不自动覆盖Host runtime/newGit I1/B/12/平台/main。"
    },
    {
      "region": "Cargo.toml",
      "lens": "result_side_effect",
      "status": "covered",
      "refs": [
        "audit.sdk-feature-wiring-rereview",
        "audit.sdk-dependency-duplicate-policy-rereview",
        "evidence.sdk-current-reviewed-rust-gates",
        "evidence.sdk-feature-wiring-verification",
        "evidence.sdk-dependency-duplicate-policy-verification",
        "evidence.sdk-protocol-finite-observation",
        "map.sdk-product-boundary"
      ],
      "review_scope": "根manifest只选择构建成员；运行副作用仍由各实际Host/Protocol模型拥有。实际fmt/lint/tests/examples按正式收据来源通过，不从成员清单制造执行成功。",
      "limitations": "仅根workspace构建元数据与已审build/dependency/feature证据；不自动覆盖Host runtime/newGit I1/B/12/平台/main。"
    },
    {
      "region": "Cargo.toml",
      "lens": "state_authority",
      "status": "covered",
      "refs": [
        "audit.sdk-feature-wiring-rereview",
        "audit.sdk-dependency-duplicate-policy-rereview",
        "evidence.sdk-current-reviewed-rust-gates",
        "evidence.sdk-feature-wiring-verification",
        "evidence.sdk-dependency-duplicate-policy-verification",
        "evidence.sdk-protocol-finite-observation",
        "map.sdk-product-boundary"
      ],
      "review_scope": "Workspace成员与解析模式唯一来自此manifest；没有第二业务Registry/Session/Task状态，依赖框架pin在Host manifest属于原唯一来源。",
      "limitations": "仅根workspace构建元数据与已审build/dependency/feature证据；不自动覆盖Host runtime/newGit I1/B/12/平台/main。"
    },
    {
      "region": "Cargo.toml",
      "lens": "data_durability",
      "status": "covered",
      "refs": [
        "audit.sdk-feature-wiring-rereview",
        "audit.sdk-dependency-duplicate-policy-rereview",
        "evidence.sdk-current-reviewed-rust-gates",
        "evidence.sdk-feature-wiring-verification",
        "evidence.sdk-dependency-duplicate-policy-verification",
        "evidence.sdk-protocol-finite-observation",
        "map.sdk-product-boundary"
      ],
      "review_scope": "该文本和依赖graph由真实Git/locked输入保全；当前source provenance tags有remote恢复证据。不推应用运行持久化。",
      "limitations": "仅根workspace构建元数据与已审build/dependency/feature证据；不自动覆盖Host runtime/newGit I1/B/12/平台/main。"
    },
    {
      "region": "Cargo.toml",
      "lens": "time_lifecycle",
      "status": "not_applicable",
      "refs": [
        "audit.sdk-feature-wiring-rereview",
        "audit.sdk-dependency-duplicate-policy-rereview",
        "evidence.sdk-current-reviewed-rust-gates",
        "evidence.sdk-feature-wiring-verification",
        "evidence.sdk-dependency-duplicate-policy-verification",
        "evidence.sdk-protocol-finite-observation",
        "map.sdk-product-boundary"
      ],
      "review_scope": "纯workspace声明，无运行actor、租约、deadline或后台工作；运行生命周期由Host子路径已有审查负责。",
      "limitations": "仅根workspace构建元数据与已审build/dependency/feature证据；不自动覆盖Host runtime/newGit I1/B/12/平台/main。",
      "reason": "纯workspace声明，无运行actor、租约、deadline或后台工作；运行生命周期由Host子路径已有审查负责。"
    },
    {
      "region": "Cargo.toml",
      "lens": "failure_concurrency",
      "status": "not_applicable",
      "refs": [
        "audit.sdk-feature-wiring-rereview",
        "audit.sdk-dependency-duplicate-policy-rereview",
        "evidence.sdk-current-reviewed-rust-gates",
        "evidence.sdk-feature-wiring-verification",
        "evidence.sdk-dependency-duplicate-policy-verification",
        "evidence.sdk-protocol-finite-observation",
        "map.sdk-product-boundary"
      ],
      "review_scope": "纯workspace声明不维护并发状态或恢复流程；编译特性/config错误由实际feature矩阵返回失败，不产生第二调度器。",
      "limitations": "仅根workspace构建元数据与已审build/dependency/feature证据；不自动覆盖Host runtime/newGit I1/B/12/平台/main。",
      "reason": "纯workspace声明不维护并发状态或恢复流程；编译特性/config错误由实际feature矩阵返回失败，不产生第二调度器。"
    },
    {
      "region": "Cargo.toml",
      "lens": "permission_external",
      "status": "covered",
      "refs": [
        "audit.sdk-feature-wiring-rereview",
        "audit.sdk-dependency-duplicate-policy-rereview",
        "evidence.sdk-current-reviewed-rust-gates",
        "evidence.sdk-feature-wiring-verification",
        "evidence.sdk-dependency-duplicate-policy-verification",
        "evidence.sdk-protocol-finite-observation",
        "map.sdk-product-boundary"
      ],
      "review_scope": "根manifest无运行权限/凭据/网络端点；现Cargo依赖来源/license/advisory检查与50exact例外另具独审，不由该文件授予或放宽本地Agent权限。",
      "limitations": "仅根workspace构建元数据与已审build/dependency/feature证据；不自动覆盖Host runtime/newGit I1/B/12/平台/main。"
    },
    {
      "region": "Cargo.toml",
      "lens": "contract_evidence",
      "status": "covered",
      "refs": [
        "audit.sdk-feature-wiring-rereview",
        "audit.sdk-dependency-duplicate-policy-rereview",
        "evidence.sdk-current-reviewed-rust-gates",
        "evidence.sdk-feature-wiring-verification",
        "evidence.sdk-dependency-duplicate-policy-verification",
        "evidence.sdk-protocol-finite-observation",
        "map.sdk-product-boundary"
      ],
      "review_scope": "Protocol无framework依赖、Host精确依赖框架，两个成员/默认成员的构建边界与已接受SDK独立产品规则一致；full28生成12缺失不被workspace编译代替。",
      "limitations": "仅根workspace构建元数据与已审build/dependency/feature证据；不自动覆盖Host runtime/newGit I1/B/12/平台/main。"
    }
  ]
}
---

# echo-agent-sdk 语义基线

## 源码快照

`base_revision`保留已发布SDK `30d7e219cc7d3aef0f75a19217c82898c363f50d` 作为可恢复祖先谱系。最终来源观察为SDK Git `e75e607a7f4e956399c9d3b4ed14213e6f303ef5`；官方source_digest(workingtree)与source_digest_at_revision(该Git)均为 `473fb206df1cb4ee900ca23e54c955e7956c1e6363079d22d78acd35d4a3e334`，4516个非语义文件逐树可恢复、没有剩余未入Git的摘要文件。内容摘要提供当前完整性，不冒称它是30d7树摘要。各Evidence/Audit继续按自身真实观察Git及运行输入读取，不因最终baseline更新而全部改成同一版本。

## 仓库区域

现有路径区域不变。根Cargo.toml、contracts、Protocol、Host及三语言SDK为五个in_scope；docs、脚本、CI、锁和其它根配置承担各自supporting角色。当前官方逐路径分类无缺失/重叠；历史归档中的源码、实验夹具和raw日志是取证材料，不成为新runtime authority。Git已跟踪及未忽略未跟踪路径均纳分类，路径数量不作能力完成分母。

## 能力图与边界

九张既有Map映射八个能力owner和仓库产品边界；既有41族索引不新增第二registry或第42族。Protocol/Host/语言/合同生成、框架执行权威与SDK薄转换的关系由现有Behavior/Rule及具名风险Audit/Evidence记录。已接受合同以唯一seed和accepted产物为准，完整Rustinventory只遥测；不知道或未接受的能力不得暗转accepted。

## 覆盖网格

五个in_scope区域均有八风险格，38个covered具已有实际有界源码审、故障假设和Evidence，根manifest无运行actor/并发状态的两格not_applicable有明确理由。库存闭合只确认路径分类、能力/权威/生命周期/失败的建模范围和有据风险审查已建立，不表示无缺陷或全族verified。covered允许保留已识别未解决Finding；不按测试或identity数量决定。

## 未知与缺口

行为模型闭合仍open：12项已接受source/kind处置待人；cleanup/Sandbox/Task resume三个宽Finding共享5秒物理结算B残余。它们属于两个待裁决单位，不风险接受或隐性退约。Windows原生消费者及适用远端SDK CI/main另需真实验收，不从本地三语言证明推断。原完整合同只读检查退出1的13项连续性错误保持，另1项已独立批准Curator保存的原依据不由本模型重作决定。未到达生成后续命令不算通过。

## 闭合结论

inventory_closure=closed表示本次有据的库存/风险建模闭合；behavior_model_closure=open保上述具体残余，41个Behavior不批量升verified。SDK pin00782的实际14Rust、完整三语言793输入及Git84消费者补证分别按正式包的原sourcecut证明有限行为，SDK main/registry交付独立。最终strict源码摘要验证使用真实fresh预检currentTaskBase `e75e607a7f4e956399c9d3b4ed14213e6f303ef5`，并保原30d全goal差异/36依据闭环、1032历史引用及Runtime793支持核对。原483task的8MiB/current锚、任务输入闭包和真实缺口分类保留；原30d诊断失败不被改写成成功。该门禁仍校验全项目对象、所有路径分类、当前摘要与变更依据，不只是本轮单字段或测试计数。

## 原治理阶段文字（历史，原字节保留）

下列旧“全视角未审”“Channel静态缺口”“Git I1开放”“候选未安装”等只描述原阶段。当前以前述精确范围和对应Finding为准，不抹旧source/time/raw，也不拿旧未跟踪草稿当长期当前事实源。


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

## 最终有限风险覆盖候选（尚未安装）

本40格覆盖四个生产目录及第五个in_scope根Cargo.toml。原32格材料已有限审查；新增根manifest八格只陈述其两个成员/default-members/resolver3的构建元数据，引用真实build/dependency/feature收据；没有运行actor与并发状态的两个视角明确not_applicable，非屏蔽Host风险。各格仅映射已审的有界义务，新Git分支切换I1、B与12问题仍明确open。covered 是有范围的风险审查事实，不表示没有 Finding；B 与12合同处置、平台/远端交付明确保留。原“全Host未审”的未知属于建模早期历史，不再用全部未知掩盖已完成源码审。库存/模型闭合标志与实际源码摘要须在本候选独审并最终冻结后分别判定，不按41数量或测试总数自动变更。
