---
schema_version: 1
id: evidence.sdk-computation-50
kind: evidence
observed_at: 56929154c1851473e9e4837901b610cecf001b2d
source_refs:
- docs/sdk/governance/evidence/sdk-computation-20261013/receipt.json
- docs/sdk/governance/evidence/sdk-computation-20261013/comparison.json
- docs/sdk/governance/evidence/sdk-computation-20261013/source-freeze-before.json
- docs/sdk/governance/evidence/sdk-successor-model-reviews/computation-50.json
- echo-sdk-host/src/core_profile/facade/tools.rs
- sdks/shared/testing/computation_fixture.py
- docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/final-receipt.json
- docs/sdk/governance/evidence/sdk-language-factory-final/receipt.json
- docs/sdk/governance/evidence/sdk-language-factory-final/source-before.json
- docs/sdk/governance/evidence/sdk-language-factory-final/source-after.json
- echo-sdk-host/src/core_profile/facade/tools.rs#run_bound_tool
- echo-sdk-host/Cargo.toml
- docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/source-build-manifest.json
- docs/sdk/governance/evidence/sdk-count-authority-calibration/framework-main-b909-acceptance.json
supports:
- behavior.sdk-family-statistics
- behavior.sdk-family-structured-output
- behavior.sdk-family-content-guard
- behavior.sdk-family-chart
- behavior.sdk-family-data
- behavior.sdk-family-media
- behavior.sdk-family-invoke
limitations:
- Focused accepted operations only; all family statuses stay needs_review
- Tool file effects are not complete files API evidence
- Final published revision/accepted generation/full gates remain separate
---

# computation-50

## 支持的结论

sdk_capability_acceptance_audit原独立limitedpass0，由parent回收：50archive SHA c8614eac、12compiledinputs、三语言各1test/50nativecalls/19typederrors，statistics/structured/content_guard/chart/data/media直接六family；8个保护文件每调用原字节未变，不表示files API全测。

原64c7/45语料保持历史；新7bbd/50包含189ff后继五Pivot边界，null label碰撞/null index保行/unknownagg拒绝与defaultsum分别明确，独立输入不是新SDK parser。

## 来源与范围

原归档receipt/输入hash/二进制保持不变。当前观察只绑定模型与可读取归档，不把原epoch冒称最终current全套通过。

## 已知缺口

Focused accepted operations only; all family statuses stay needs_review；Tool file effects are not complete files API evidence；Final published revision/accepted generation/full gates remain separate。

## 对原保存收据的当前审计观察

本对象observed_at现在指真正当前候选Git `56929154c1851473e9e4837901b610cecf001b2d`，只定位已经保存的收据、输入源码副本与manifest的审计观察。原观测字段为`source:pending`；它若是source摘要或pending，均不被伪装为历史执行当时的可恢复整仓Git。当前观察不宣称原运行发生于Git569、不重跑或升级原runtimeepoch。

全部上列source_refs均从该Gitblob实际读取，与当前archive原字节SHA精确相同，原锚点存在；原日志时间、HostbinarySHA、sourcefreeze、错误/通过计数及正文来源保持。原body的epoch/signature/cause限制与supports均为原Behavior义务的历史有界支持，不能由此宣布当前源码verified；旧3ea/9e没有完整Git时仅有原保存收据可恢复，整仓原epoch仍不可恢复，不补造树。

原保存artifact中的具名元数据（完整值仍见原JSON，不改变）：`host_sha256=7bbd1569367b0b1004fbe1cbc537b6869a2bc685bb1ca14e63228b1746aee3b2`；`consumers.0.host_sha256=7bbd1569367b0b1004fbe1cbc537b6869a2bc685bb1ca14e63228b1746aee3b2`；`consumers.1.host_sha256=7bbd1569367b0b1004fbe1cbc537b6869a2bc685bb1ca14e63228b1746aee3b2`；`consumers.2.host_sha256=7bbd1569367b0b1004fbe1cbc537b6869a2bc685bb1ca14e63228b1746aee3b2`。

本次只附加观察说明；原正文/supports/limitations保留，原rawReceipt/ArchiveChecksum/运行时间未写入，任何未决unknown/失败仍不自动resolved。源码业务Audit不是此收据观察，另队列保持原revision与复审义务，不能偷换类型或scope。

## 本次已复审scope的当前证据重观察

当前观察Git`56929154c1851473e9e4837901b610cecf001b2d`仅据下面明确源码业务复审的已接受路径，回收UTC`2026-10-14T00:39:15.355735+00:00`；原marker`56929154c1851473e9e4837901b610cecf001b2d`、旧runtimeepoch/time/sourcehash/raw全文保留，原support/limits不变。不是过去执行就发生569，不假历史整仓Git恢复。

- `audit.sdk-computation-50-rereview` reader`current_tool_boundary_review`：**BLOCK / Important1 Chart整数丢失；原50有限场景仍PASS**；本次原故障检查Truecomputedvalues 50row/19typederrors/9oracles,UTF8 guard spans+structuredvalidate optional/no extraAgentrepairconfig；Pivot null label collision expliciterror/nullindexnotlost/unknownaggregate rejects/defaultsum correct;8protectedfiles actualbyte effects。after仅Newbb4f ComputationHostTest/TS/PY one50row group each;actualPolarsunordered normalization limited,not filesAPIfullscope。

原50cases值/9oracle/19错误仍有界有效，但当前b909Chart整数范围新增I1/BLOCK未覆盖，`finding.sdk-chart-dimension-integer-loss`open；Nsourcefix23绿不是SDK569afterproof，原无损期望不能通过limit改成近似/default接受。

所有新SDKsource/proof refs按真实569byte/anchor读取，原业务与原sourcefreeze记录保持；这不是一切currentfamily/runtimepassed。ordinaryTools已有原五族收据只新增本次明确Database/Artifact检查，不暗称其余方言/provider/工具所有field均新审；原candidate supports=[]不升。任何缺原完整time/tree仍在原limits，12/B/depWarn人未答/fullgeneration/CImain待。
