---
schema_version: 1
id: finding.sdk-git-branch-switch-dirty-path-overwrite
kind: finding
type: implementation_bug
status: resolved
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
evidence_refs:
- evidence.sdk-git-published-consumer-verification
audit_refs:
- audit.sdk-git-published-consumer-rereview
decision_refs: []
repair_evidence_refs:
- evidence.sdk-git-published-consumer-verification
verification_evidence_refs:
- evidence.sdk-git-published-consumer-verification
rereview_audit_refs:
- audit.sdk-git-published-consumer-rereview
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

## 已发布根修复与SDK消费者具体本地闭合

已发布框架 main00782 的 GitBranchTool 修复使用原分支专用 git switch -- target，无 force/path restore；原F a634源码82cf的同名dirty损失与CLI2、Tool3fail→same4pass/最终cfd保护oracle独审三链保各正式F切点，不把SDKRepo当Framework树。

SDK真实00782 pin/锁88d99，三语言正常SDK消费者首次各28调用、9个工具入口与4个既有source操作、每语6预期错误，实际不可变Host0f1ecf全部通过。只归一各自owned root，完整请求/结果/错误/refs/index/tree/files/physicalworktree全部字段一致；同名dirty switch实际改分支且bytes不变，非法branch/missing-ref/nonrepo/dirty拒绝的完整快照不变（partial staging明示例外，不声称回滚）。原cwd隔离与Session close后peer仍可用来自独立历史evidence.sdk-tool-behavior-candidate/其原epoch，不属于本次84当前PASS范围。worktree dirtyremove返回原typed拒绝且Hostalive，随后merge/remove/list真实完成，这也支持已独立闭合的栈修复，不重新混为branch源bug。

CurrentTool本会话实际源码/84记录/所有字段与Java1/0/0/0 XML独审PASS C0/I0/M0。manifest的source_execution_commit f799是当时工作执行切点；四consumer实际字节哈希未变、executed copies与原raw保存于真正可恢复fe845，不能称f799已包含当时未tracked的这四测试。旧a634 before/failed记录与两种stack failed candidates全保，不拿旧失败覆盖新结果。

这个Evidence observed fe845只定位明确当前修复消费者与原包。Framework Git仍唯一HEAD/heads refs/index/files/worktree权威，Host薄参数、Owner与完整ToolResult投影不产生第二管理器。本Finding同名dirty path恢复的具体数据丢失义务已由根修复＋SDK消费＋实际独审补齐，可本地resolved；Java/三语言完整28现在已实测，不再写待执行。

本slice不是所有平台或任意并发Git变动/远程push，不能由84计数证明所有未列Git行为。Root最新完整Rust/语言与最终SDK main/合同需自己的冻结收据，manifest当时whole pending保历史；B物理残余、12身份迁移/完整生成继续原独立Finding。
原前文“未RED/未修/新pin待”属发现阶段，原discovered_at/旧body保留；本段补齐实际运行与源码/消费者复审，不claimSDKmain或关闭外部issue。
