---
schema_version: 1
id: audit.sdk-worktree-source-stack-rereview
kind: audit
boundary_ref: boundary.sdk-integrations
lens: time_lifecycle
freshness: examined
revision: 3958f5482c38749ec65cd20ab145917e918452a2
finding_refs:
- finding.sdk-git-worktree-source-poll-stack-overflow
challenges:
  actual-stack-chain-and-thin-git-route:
    revision: 3958f5482c38749ec65cd20ab145917e918452a2
    source_refs:
    - echo-sdk-host/src/core_profile/facade/source_operations.rs#dispatch_non_git_source
    - echo-sdk-host/src/core_profile/facade/source_operations.rs#dispatch_git_worktree_source
    - echo-sdk-host/src/core_profile/facade/source_operations.rs#dispatch_git_worktree_create
    - echo-sdk-host/src/core_profile/facade/source_operations.rs#dispatch_git_worktree_list
    - echo-sdk-host/src/core_profile/facade/source_operations.rs#dispatch_git_worktree_merge
    - echo-sdk-host/src/core_profile/facade/source_operations.rs#dispatch_git_worktree_remove
    - echo-sdk-host/src/core_profile/facade/mod.rs
    - docs/sdk/facade-feature-adapters.md
    - docs/sdk/governance/evidence/sdk-worktree-source-stack/before-diagnosis/root-cause-summary.json
    - docs/sdk/governance/evidence/sdk-worktree-source-stack/before-diagnosis/crash-source-reference.json
    - docs/sdk/governance/evidence/sdk-worktree-source-stack/thin-entry-candidate/static-impact.json
    - docs/sdk/governance/evidence/sdk-worktree-source-stack/thin-entry-after/README.md
    - docs/sdk/governance/evidence/sdk-worktree-source-stack/thin-entry-after/manifest.json
    - docs/sdk/governance/evidence/sdk-worktree-source-stack/thin-entry-after/build/build-manifest.json
    - docs/sdk/governance/evidence/sdk-worktree-source-stack/thin-entry-after/build/build-run.json
    - docs/sdk/governance/evidence/sdk-worktree-source-stack/thin-entry-after/build/clippy-run.json
    - docs/sdk/governance/evidence/sdk-worktree-source-stack/thin-entry-after/build/git-feature-off-check-run.json
    - docs/sdk/governance/evidence/sdk-worktree-source-stack/thin-entry-after/build/inputs-before.json
    - docs/sdk/governance/evidence/sdk-worktree-source-stack/thin-entry-after/build/inputs-after-all-checks.json
    - docs/sdk/governance/evidence/sdk-worktree-source-stack/thin-entry-after/build/poll-frame-comparison.json
    - docs/sdk/governance/evidence/sdk-worktree-source-stack/thin-entry-after/measurement-correction/receipt.json
    - docs/sdk/governance/evidence/sdk-worktree-source-stack/thin-entry-after/native/observation-manifest.json
    - docs/sdk/governance/evidence/sdk-worktree-source-stack/thin-entry-after/native/typescript-run.json
    - docs/sdk/governance/evidence/sdk-worktree-source-stack/thin-entry-after/native/python-run.json
    - docs/sdk/governance/evidence/sdk-worktree-source-stack/thin-entry-after/native/full-field-comparison.json
    - docs/sdk/governance/evidence/sdk-worktree-source-stack/thin-entry-after/native/typescript/results.json
    - docs/sdk/governance/evidence/sdk-worktree-source-stack/thin-entry-after/native/python/results.json
    - docs/sdk/governance/evidence/sdk-worktree-source-stack/thin-entry-after/executed-source/echo-sdk-host/src/core_profile/facade/source_operations.rs
    - docs/sdk/governance/evidence/sdk-worktree-source-stack/thin-entry-after/executed-consumer-source/typescript.mjs
    - docs/sdk/governance/evidence/sdk-worktree-source-stack/thin-entry-after/executed-consumer-source/python.py
    evidence_refs:
    - evidence.sdk-worktree-source-stack-repair
---

# SDK worktree source 栈缺陷实际复审

## 审查范围

framework_final_review本会话已实际源码、有限consumer、原测量与6080校正增量审；模型作者只归档该已完成结论，不新模拟source审。

## 已检查故障假设

外层Box::pin仍构造巨大parent future；只拆四臂未绕过宽poll；dirtyremove好但merge还abort；量化漏probe；切新框架pin掩原因果；新thread/manager/业务规则复制；全28/Java用TS7或oldfull冒成功。

## 实际实现路径与证据

原851的source dispatcher源码91bd与不可变da873/a634真实remove第25请求abort/原OS栈相符。两个后继候选失败也保独立epoch：333aggregate只搬四臂但仍失败；per-operation dd336可dirty拒绝后活，但合法merge仍在旧宽parent poll链abort。不能把这些失败覆盖为成功或将静态格式检查当修复。

最终3958保存源码2df：同步薄入口在构造巨大非Git async future之前按闭合四个Git source operation分流；Git router及四私有boxed future复用原framework调用体。nonGit主体去掉四Git臂后token exact，四helper full tail bytes exact；Sessionowner、mutation guard、参数/DTO/error转换、同三个borrowed输入生命周期、feature-off fallback保持。没有新task、线程stacksize、第二Git检查器/manager/registry、publicAPI或业务权威。

同a634/.45因果pin的实际不可变03e795 Host首次完成TS7与PY7原source请求：create/list、dirtyremove精确framework_error/never/fulloperation/nullhandle/details，完整repo HEAD/refs/index/tree/files与两个worktree快照不变，之后list成功证明Host活；合法merge真实sourceHEAD/files进入main，cleanremove目录消失而branch保留，最后仅mainworktree。完整7rows只归一自己临时根，其请求/结果/错误/副作用全字段一致，不丢字段、不自动default。

同profile原二进制只LLDB target/disassembly无launch测量：entry constructor6080B、router112B，create/list/merge/remove poll分别76272/76752/78400/76576B；nonGit仍909552B但不进入Git4链。原1984派生数漏4KiB probe，现32+4096+1952=6080，旧元/42checksum与原disassembly保；不是OS总stacklimit证明。61Rust/config输入前后与执行档同，build/production strictclippy/git-feature-off check真实0。

framework_final_review本会话真实静态+actualafter源码/consumer/原raw/最后6080Minor增量复审PASS C0/I0/M0，Root据此冻结3958。归档manifest“finalMinorpending”及root333候选README属于当时历史，正文后来已明确minor校正和独审，不假修改旧收据状态。此三链仅结清SDK Git4source致命poll栈缺陷。

Java尚未执行，三语言完整28语料、新发布00782/.46 pin、完整Rust14/语言全套/合同/main与SDK GitBranch语义缺陷消费者验证均另验；旧诊断a634/.45不称安全发布或新pin证明。B/12和宽资源义务不从本局部resolved关闭。

## 问题记录

原具体栈缺陷修复/验证/独审三链齐备，局部resolved，非wholeGit。

## 残余风险

新pin、Java、完整三native28及baseline/sourcefreeze/whole/main独立；a634 GitBranch仍原消费者未验。

## 未检查项

任意其它大nonGit call链、OS所有栈阈值/所有平台与物理B完整Producer强停均不由本次保证。