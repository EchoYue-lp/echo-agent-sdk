---
schema_version: 1
id: finding.sdk-git-branch-switch-dirty-path-overwrite
kind: finding
type: implementation_bug
status: open
severity: high
primary_focus: result_side_effect
focus:
- trigger_input
- state_authority
- data_durability
- contract_evidence
boundary_ref: boundary.sdk-integrations
behavior_refs:
- behavior.sdk-family-git
rule_refs:
- rule.framework-runtime-authority
evidence_refs: []
audit_refs: []
decision_refs: []
repair_evidence_refs: []
verification_evidence_refs: []
rereview_audit_refs: []
discovered_at: ddc60f70f269963d79570156693151d24226b99c
---

# Git branch switch 可覆盖同名 dirty tracked 路径

## 问题

SDK 当前精确 pin 为已发布框架 a634。框架 GitBranchTool 的 switch 分支执行 git checkout -- target；双横线将 target 解释为路径恢复，而不是分支切换。若目标分支与已跟踪文件同名，该命令可覆盖该文件未提交内容，却返回切换成功文案。SDK 原薄工具通路没有第二 Git 状态权威，也不应靠 SDK 绕修。

## 触发条件与影响

同名分支/已跟踪文件且文件 dirty 的输入会走错误路径恢复语义，违反已接受切换分支行为与未提交数据保全。原 status/diff 与 bad branch 场景、其它10工具的通过不能证明这个窗口。

## 证据

framework_final_review 本会话实际新增 Git 有限八视角源码审：C0/I1/M0，确定此具体缺陷；它是静态 source counter，不是已执行 RED。根修复源为 [框架 a634 git.rs](https://github.com/EchoYue-lp/echo-agent/blob/a6346643bda47679a96760f56f45e41fd73b61b0/echo-tools/src/git.rs#L476)，文件SHA256为82cf6591ad8b55438db4ea07482c5bfe06a27dc6766d10d0203639bc37499098。SDK consumer 为 echo-sdk-host/src/core_profile/facade/tools.rs 与 source_operations.rs；原唯一框架Tool/managed helper负责Git事实与worktree，Host只原schema/位置参数转换和完整ToolResult。

## 处理记录

Root 已安排框架窄修和新三语言有限 fixture；当前未声明原实际行为红、修后验证或已发布新pin。正式反例/修复/验证/独立复审将使用各真实源码与归档切点；evidence_refs 暂空避免伪造 future Git。该Finding保持open，B/12决策与此普通实现缺陷独立，不需新增产品裁决。
