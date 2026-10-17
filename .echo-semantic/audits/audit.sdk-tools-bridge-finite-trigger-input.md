---
schema_version: 1
id: audit.sdk-tools-bridge-finite-trigger-input
kind: audit
boundary_ref: boundary.sdk-extension-bridge
lens: trigger_input
freshness: examined
revision: 5bcbad2437e5c19f61101883044c4f67944cb97c
finding_refs:
- finding.sdk-feature-failure-cause-fidelity
- finding.sdk-tool-input-validator-bypass
challenges:
  builtin-tool-and-reverse-bridge-obligation:
    revision: 5bcbad2437e5c19f61101883044c4f67944cb97c
    source_refs:
    - echo-sdk-host/src/core_profile/facade/tools.rs
    - echo-sdk-host/src/core_profile/facade/registry.rs
    - echo-sdk-host/src/core_profile/facade/source_operations.rs
    - echo-sdk-host/src/core_profile/facade/integrations.rs
    - echo-sdk-host/src/core_profile/facade/mod.rs
    - echo-sdk-host/src/core_profile/extension_bridge.rs
    - echo-sdk-host/tests/extension_bridge_e2e.rs
    - docs/sdk/governance/evidence/remote-stream-failure-domain/manifest.json
    - docs/sdk/governance/evidence/remote-stream-failure-domain/README.md
    - docs/sdk/governance/evidence/remote-stream-failure-domain/before-valid-descriptor-run.json
    - docs/sdk/governance/evidence/remote-stream-failure-domain/after-run.json
    - docs/sdk/governance/evidence/remote-stream-failure-domain/production-only.patch
    - docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/receipt.json
    - docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/inputs-before.json
    - docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/inputs-after.json
    - docs/sdk/governance/evidence/sdk-chart-published-pin/ci-regression/regression-review-manifest.json
    evidence_refs:
    - evidence.sdk-tools-bridge-finite-observation
  specific-tool-input-validator-bypass:
    revision: 622d7931e01556161441d22de0697f6f35f24f17
    source_refs:
    - echo-sdk-host/src/core_profile/facade/tools.rs#direct_tool_validation_precedes_custom_rules_and_execution
    - echo-sdk-host/src/core_profile/facade/tools.rs#validate_tool_input
    - docs/sdk/governance/evidence/sdk-final-idempotency-rust-gates/original/05-workspace-tests.log
    - docs/sdk/governance/evidence/sdk-final-idempotency-rust-gates/combined-receipt.json
    evidence_refs:
    - evidence.sdk-tools-bridge-finite-observation
    - evidence.sdk-tool-input-validation-candidate
---

# Tools/Bridge trigger_input 有限审查

## 审查范围

本session实际currentTool原source审查的builtin10与reverse6role回收，原69b运行文件在5bcb真实Git相同；不是作者新审或全部identity承诺。各familyscope表见sharedEvidence，不把Chart61统计数累为其它能力。

## 已检查故障假设

唯一framework schema/arity/customvalidator、Tool descriptor与owner/kind/gen、URL/cwd/table/cursor、Graph/APIstrictvalue保；不把catalogdone当准入成功。

## 实际实现路径与证据

原工具49/61、Rag32、FileAuthority10等具名字段/文件SHA/真实PID/errorcase支持原业务；Tool/LLMremotecause另合法3red→3green官方注册consumer强断言。code/packet在真实Git5bcb，旧whole790不冒修复后新whole。原sourceindep和本材料质控分别记角色。

## 问题记录

两个原源码Important的remote子根因已具真实反例修复和独立review，scope内finitePASS。宽featurecauseFinding其它Workflow/Memory/Eval仍原未决，不能通过单scope压成全resolved或all-familyverified。

## 残余风险

已接受角色之外任意DTO/constructor无新API保证；network时点、内存非crossprocess、5秒B/12sourcekind/新A后最终wholemain分别保。暂无本组新增确定源码阻断。

## 未检查项

任意codec/network故障重试可靠导出或强kill后远端物理工作已停止，不由本scope强推；原callback所有秘密过滤也未承诺。

## finding.sdk-tool-input-validator-bypass 原故障义务准确复核

原 adapter 跳过唯一 framework schema/custom validator 的具体 bug 已改为同一 validate_tool_input，参数绑定拒 excess arity（含 null），没有第二 validator。真实 prototype not_a_chart_type→假 line 的原成功和修后 typed InvalidParameter 保在旧专属 Evidence；正式 spy 强断言 missing/type/enum/excessNull/customBad 全 executes0、valid 精确 executes1，最新原05真实通过。后继 Chart 整数义务独立已有61原生/独审闭合，不以旧非法 enum 修复替整数证明，不把 helper 自动接受为三语言 API。
current_tool_boundary_review 本次真实有限审查明确该原缺陷 closeeligible；本新 challenge 保主5bcb/旧审正文及已知范围，不借宽 tools family PASS。
