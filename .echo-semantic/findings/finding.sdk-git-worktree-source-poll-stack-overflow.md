---
schema_version: 1
id: finding.sdk-git-worktree-source-poll-stack-overflow
kind: finding
type: implementation_bug
status: open
severity: high
primary_focus: time_lifecycle
focus:
- result_side_effect
- failure_concurrency
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
discovered_at: ff8a9384db2d0f89994c2487a1990ba1e1e04373
---

# Git worktree source adapter poll 过大栈帧导致 Host 崩溃

## 问题

真实已构建 Host da873 / framework a634 经现有 source_operation remove_worktree 路径时，在 verify_managed_worktree 的 poll 链发生栈溢出。只读原 OS 报告和同一二进制反汇编表明，SDK source_operations::dispatch poll 自身保留1,216,096字节，六个已测 closure/poll prologue合计1,901,744字节（不含未测外层/子调用）。原调用链没有递归证据，不能把它当 fixture oracle 错误或通过增加线程栈规避。

## 触发条件与影响

真实第25个Git consumer场景的dirty worktree removal不能取得预期框架拒绝，Host进程直接abort，影响同connection的运行与清理。Git业务、managed marker/owner/dirty检查仍由framework唯一helper负责；SDK问题是大async dispatcher的执行布局，不应复制Git判断或引入第二state authority。

## 证据

原故障二进制SHA da873b4aa94fe0b9f1f255e22337c16acb3cd719a939d8b1fdc57ec47c7ce444。只读取证保存在工作区 admin/sdk-worktree-stack-readonly/root-cause-summary.json 与 crash-source-reference.json及各反汇编日志；原OS报告SHA b00cbcfe40c26ce2ed5cc1b82face76ab5d50ca339c4775a2114f9ee94917b27，PID25628，captureTime2026-10-15 00:20:11.1327 +0800。这是实际产品Host abort和原栈帧取证，不是为试验而重跑/编译的结果。SDK源码可恢复版本622d/b7e6的 echo-sdk-host/src/core_profile/facade/source_operations.rs；旧源码SHA91bd8f84f3a05b272b52911c7ba52d7cc54537d9f88790cfbb11578a57817a94，当前修补尚在进行，不能将新current文件假称旧执行输入。

## 处理记录

Root已授权唯一源码owner将Git四个原framework调用的原body逐字抽到现有私有BoxFuture分区模式；复用原analysis/agentcontext dispatch的实践，不新增publicAPI、不加线程stacksize、不替代framework检查。修复仍open：需要新编译二进制的同RPC不再fatal、保框架dirty错误/owner及后续活性，并验证真实poll prologue下降及独立scope复审。正式Git证据包/新source Git尚未冻结，引用列表暂空，避免fake futureSource。Framework branch switch另具体Finding，B/12决策与此普通adapter缺陷独立。
