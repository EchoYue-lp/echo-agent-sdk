---
schema_version: 1
id: evidence.sdk-dependency-duplicate-human-decision
kind: evidence
observed_at: source:pending
source_refs:
- AGENTS.md
- docs/sdk/governance/dependency-warning-policy-proposal.md
- docs/sdk/governance/evidence/sdk-current-dependency-policy-after-feature-wiring/baseline-comparison.json
- docs/sdk/governance/evidence/sdk-current-dependency-policy-after-feature-wiring/source-before.json
- docs/sdk/governance/evidence/sdk-current-dependency-policy-after-feature-wiring/source-after.json
supports:
- rule.framework-runtime-authority
limitations:
- Human accepted bounded duplicateexception only; 12source/Bphysical/CLIwebsite choices
  not granted
- New governance documentation cut not yet frozen or recoverableGit rebound; currentsourcepending
  intentional untilauthorizednewcut
- Independent finalrule text review stillpending; authorityConflict Finding remainsopen
  until decision/implementation/verification/review refs complete
- No advisoryignore/lockgraph/config change or nonduplicate/compilerwarn exception
---

# 用户接受有界duplicate提示例外

## 支持的结论

用户本次明确答复“接受有界 duplicate 提示例外（推荐）”，由根当前goal消息回收。选择绑定原受审proposal4c940、精确50组crate及actualversions、锁a42cf和原完整deny日志，不是对未来50条数量或任意duplicate放行。SDK AGENTS当前72–81已实际落地：所有命令真实0/零失败/格式差异，compiler/audit/advisory/license/source/config及其它警告仍为0，仅原cargo-deny warning[duplicate]在完整原输出+精确组/版本基线复核且无未经审增量时非阻断；锁/框架pin/features/policy改变须重跑复审。

## 来源与范围

原afterfeature19档工具0/50actualcrateversionmatched、原20档/原4exceptions保持。人的答复解除这项规则选择的“待答”，不替代码/合同/whole行为验证。实际本次读取时AGENTS已改，proposal正文仍Statusproposed，所属文稿owner正在收敛accepted/原人答/date，不能把未完成文稿写成已完成新sourcecut。旧Git569/17cf仍可恢复原切点；新非semantic规则文本会正常改变workingtree摘要，新currentGit/observedmarker待root冻结授权后再绑定，不伪旧569含本次人答规则。

## 已知缺口

独立最终规则文本复审尚未返回，finding.sdk-dependency-warning-policy-conflict当前保持open，等待裁决文本落地与受界组复验/复审三引用结账。这里不是漏洞风险接受，不改变4ignore、不默认关闭Finding或41family/risklens。12sourceidentity迁移与framework5秒物理结算选择尚未答，CLI/website外部交付另待授权；不重复询问已明确的duplicate选择。当前只登记人的有限期望和真实落地阶段，sourcepending待新cut，不能称strict0或最终SDKmain。
