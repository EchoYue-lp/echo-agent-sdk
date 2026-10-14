---
schema_version: 1
id: behavior.sdk-family-git
kind: behavior
status: needs_review
expectation: human_confirmed
risk: high
primary_focus: state_authority
focus:
- time_lifecycle
- failure_concurrency
- contract_evidence
- data_durability
boundary: boundary.sdk-integrations
observed_at: 56929154c1851473e9e4837901b610cecf001b2d
code_refs:
- echo-sdk-host/src/core_profile/facade/mod.rs
- echo-sdk-host/src/core_profile/facade/registry.rs
- echo-sdk-host/src/core_profile/facade/source_operations.rs
- echo-sdk-host/src/core_profile/facade/integrations.rs
- echo-sdk-host/src/core_profile/facade/memory.rs
- echo-sdk-host/src/core_profile/facade/workflow.rs
- echo-sdk-host/src/core_profile/facade/state_delivery.rs
- echo-sdk-host/src/core_profile/facade/tools.rs
- echo-sdk-host/src/core_profile/facade/observability.rs
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-candidate-behavior-verification
- evidence.sdk-tool-behavior-candidate
finding_refs:
- finding.sdk-git-branch-switch-dirty-path-overwrite
- finding.sdk-git-worktree-source-poll-stack-overflow
---

# SDK family git

## 重要承诺

对已接受 `git` 能力明确入口、唯一权威、状态/生命周期、失败处置和直接行为证据；未接受 Rust identity 不进入承诺。

## 当前行为

- **入口/可达**：`facade/tools.rs` framework git tools，在 Session cwd；worktree
  enter/exit/list 也必须走已有 framework 工具，Host 不自建 worktree manager。
- **权威/状态**：Git repository/index/worktree 事实与 framework ToolResult/effects。
  status/diff/log/blame/branch/commit、worktree create/enter/exit，失败不能消失用户 dirty state。
- **正常/失败**：非 repo、invalid ref、dirty/conflict、commit failure、worktree close/
  cwd 变化；actual commit SHA/index/files 与返回对应。
- **已有断言**：两Session cwd隔离、exact status/diff；sdk-proof创建指向原HEAD；invalid branch后repo A HEAD/sdk-proof及repo B refs指定值不变；Session A close后B仍可用。
- **最低充分证据**：isolated fixture repo 全 accepted git 操作、dirty preservation、
  invalid ref/commit failure、enter/exit/list 的 filesystem/Git 对账；三语言 effect/error
  对等，不对真实工作树执行破坏性验收。

## 期望行为

上述正常/失败场景须保持接受范围、完整转换和唯一权威，并在最终源码/pin证据及独立复审后闭合。

## 触发、结果与副作用

按上方具体入口和正常/失败记录检查真实主路径。可达性或catalog状态不能证明副作用、错误或结算。

## 失败、重试与恢复

按上方生命周期和最低充分证据检查取消、超时、错误、释放和恢复；不默认成功，不以其它能力族测试替代。

## 证据

阶段：`candidate_runtime_verified`；语义状态仍为 `needs_review`。

两Session cwd隔离、exact status/diff；sdk-proof创建指向原HEAD；invalid branch后repo A HEAD/sdk-proof及repo B refs指定值不变；Session A close后B仍可用。

实际来源：`evidence.sdk-tool-behavior-candidate`，绑定当前仍相同的具名测试文件与实测Host4d436；
28次工具调用+1Session close、10typed RPC error及database success=false拒写分别核对。
reported effects empty只表示已报告事实，实际文件/ref变更另行验证。GET-only记录不证明
无POST，invalid branch没有repo A全部refs前后断言。

仍缺：未比较repo A全部refs前后；git log/blame/commit和worktree enter/exit/list完整生命周期、冲突/失败清理及platform variants仍缺。 正式pin、最后变更后的独立复审与精确main仍待。
归档观察当前源摘要不重绑定旧whole运行；不自动扩大accepted合同。

## 裁决记录

用户要求SDK全能力行为治理，以明确接受跨语言合同为兼容边界；完整Rust inventory仅遥测。

## 当前观察与历史收据边界

Candidate assertion evidence is now preserved as original receipt/source fingerprints and checked against inspected current paths. It supports only named partial cases; current observation binding neither reruns old commands nor upgrades family status.

## Tool validator变化限制

当前SDK direct Tool路径在旧6711以后改变，改用同一framework validator并拒绝超额arity。旧候选运行仅保留在原来源历史审计，不证明新生产路径的完整行为；需结合新增validator具体反例与最终当前门禁，状态不提升。

## 本次有限审查单元

行为/生命周期单元：Sessioncwd隔离、status/diff与明确requestedref副作用。

剩余处置：工具29rows里git原refs断言边界保留，不声称未采集全refs都不变。

本项建模阈值是已接受入口、唯一状态权威、正常生命周期与具名失败/恢复路径可定位，并有直接证据或明确开放原因；不要求所有未定义组合全测，也不由绿色测试自动verified。

## 当前模型重观察范围

review_scope：本对象在真实当前候选Git `56929154c1851473e9e4837901b610cecf001b2d`（源码摘要`17cf243e8d6341db0ce23c5b9ee39977ce4ab100d6ed1b17ec5a7bb26c19e138`）上重新观察。上列code_refs/各scenario source_refs已逐Gitblob读取并与当前文件SHA精确相同，原锚点均存在。模型继续描述原accepted入口、框架业务/终态权威、Host owner/generation与transport消费职责、生命周期及具名失败义务；源码定位核对不证明所有行为正确、运行证据fresh或合同生成闭合。

原Behavior/status、Map场景mapped/needs_review、Rule/Asset处置、Finding/未决义务与Evidence原epoch支持范围全部保留。mapped仅入口模型，旧3ea不是本snapshot本体；历史receipt不得因本模型观察更新冒current运行。12source迁移、5秒物理结算与依赖50warning规则三选择仍未接受，完整生成/CI/main另验，任何新Rustwrapper不自动扩三语言合同。

## 后继 Git 分支切换实际源码审结果

framework_final_review 新有界八视角原source审 C0/I1/M0；具体 finding.sdk-git-branch-switch-dirty-path-overwrite 保open。a634 checkout -- target 被解释为路径恢复，不是分支切换，真实反例尚待执行，不能把静态确定反例写成executed RED。原Git已采status/diff/branch指定refs证据及旧审限制保持，当前缺口由此具名实现义务承接，不再用whole/main待验解释全部未知。SDK仍原薄adapter；新框架修复尚未发布或消费验证，不引用其它builtin10审作本族通过。

## 后继 worktree adapter 实际崩溃

真实Host da873的worktree removal发生栈溢出，原OS栈及同binary反汇编指向SDK巨大async dispatch poll，非递归/新框架Gitauthority。具体 finding.sdk-git-worktree-source-poll-stack-overflow 保open；只私有future分区修复候选尚未执行新Host验证，不能用旧whole通过掩已发现fatal窗口。
