---
schema_version: 1
id: audit.sdk-git-published-consumer-rereview
kind: audit
boundary_ref: boundary.sdk-integrations
lens: result_side_effect
freshness: examined
revision: fe8451d8f6190ad80405a7717efbd9d505cd74ed
finding_refs:
- finding.sdk-git-branch-switch-dirty-path-overwrite
challenges:
  same-name-dirty-branch-and-original-git-owner:
    revision: fe8451d8f6190ad80405a7717efbd9d505cd74ed
    source_refs:
    - echo-sdk-host/Cargo.toml
    - Cargo.lock
    - echo-sdk-host/src/core_profile/facade/tools.rs
    - echo-sdk-host/src/core_profile/facade/source_operations.rs
    - sdks/typescript/test/git-tool-host.test.js
    - sdks/python/tests/test_git_tool_host.py
    - sdks/shared/testing/git_tool_host_fixture.py
    - sdks/java/src/test/java/com/echoagent/sdk/GitToolHostTest.java
    - docs/sdk/governance/evidence/git-tool-native/README.md
    - docs/sdk/governance/evidence/git-tool-native/before-final-source/counterexample.json
    - docs/sdk/governance/evidence/git-tool-native/after-published-00782/manifest.json
    - docs/sdk/governance/evidence/git-tool-native/after-published-00782/comparison.json
    - docs/sdk/governance/evidence/git-tool-native/after-published-00782/independent-review.md
    - docs/sdk/governance/evidence/git-tool-native/after-published-00782/typescript-run.json
    - docs/sdk/governance/evidence/git-tool-native/after-published-00782/python-run.json
    - docs/sdk/governance/evidence/git-tool-native/after-published-00782/java-run.json
    - docs/sdk/governance/evidence/git-tool-native/after-published-00782/TEST-com.echoagent.sdk.GitToolHostTest.xml
    - docs/sdk/governance/evidence/git-tool-native/after-published-00782/typescript-10wlfR/results.json
    - docs/sdk/governance/evidence/git-tool-native/after-published-00782/python-he3ckh66/results.json
    - docs/sdk/governance/evidence/git-tool-native/after-published-00782/java-7939238124942641491/results.json
    evidence_refs:
    - evidence.sdk-git-published-consumer-verification
---

# SDK Git具体consumer独立复审

## 审查范围

回收current_tool_boundary_review本会话原有限源码与84消费者实际独审，原frameworkFinal Git8lens/source修复作为根义务，模型作者不是新业务审作者。

## 已检查故障假设

paths恢复替branch，文案成功但HEAD不变；dirty丢失或失败副作用；新pubpin/锁不匹配，partial staging假回滚；字段归一时被删；Java未跑靠TS counts；source操作调用变SDK自建Git authority；stack只局部dirty好却merge仍abort。

## 实际实现路径与证据

已发布框架 main00782 的 GitBranchTool 修复使用原分支专用 git switch -- target，无 force/path restore；原F a634源码82cf的同名dirty损失与CLI2、Tool3fail→same4pass/最终cfd保护oracle独审三链保各正式F切点，不把SDKRepo当Framework树。

SDK真实00782 pin/锁88d99，三语言正常SDK消费者首次各28调用、9个工具入口与4个既有source操作、每语6预期错误，实际不可变Host0f1ecf全部通过。只归一各自owned root，完整请求/结果/错误/refs/index/tree/files/physicalworktree全部字段一致；同名dirty switch实际改分支且bytes不变，非法branch/missing-ref/nonrepo/dirty拒绝的完整快照不变（partial staging明示例外，不声称回滚）。原cwd隔离与Session close后peer仍可用来自独立历史evidence.sdk-tool-behavior-candidate/其原epoch，不属于本次84当前PASS范围。worktree dirtyremove返回原typed拒绝且Hostalive，随后merge/remove/list真实完成，这也支持已独立闭合的栈修复，不重新混为branch源bug。

CurrentTool本会话实际源码/84记录/所有字段与Java1/0/0/0 XML独审PASS C0/I0/M0。manifest的source_execution_commit f799是当时工作执行切点；四consumer实际字节哈希未变、executed copies与原raw保存于真正可恢复fe845，不能称f799已包含当时未tracked的这四测试。旧a634 before/failed记录与两种stack failed candidates全保，不拿旧失败覆盖新结果。

这个Evidence observed fe845只定位明确当前修复消费者与原包。Framework Git仍唯一HEAD/heads refs/index/files/worktree权威，Host薄参数、Owner与完整ToolResult投影不产生第二管理器。本Finding同名dirty path恢复的具体数据丢失义务已由根修复＋SDK消费＋实际独审补齐，可本地resolved；Java/三语言完整28现在已实测，不再写待执行。

本slice不是所有平台或任意并发Git变动/远程push，不能由84计数证明所有未列Git行为。Root最新完整Rust/语言与最终SDK main/合同需自己的冻结收据，manifest当时whole pending保历史；B物理残余、12身份迁移/完整生成继续原独立Finding。

## 问题记录

原branch switch数据丢失源修及SDK新pin同强consumer、真实独审足具体resolved。不是全SDKmain结案。

## 残余风险

rootwhole/平台/远程Git并发、B/12各独立。

## 未检查项

任意Git动作组合、未接受方法和全部Rust身份不成为本次新scope。