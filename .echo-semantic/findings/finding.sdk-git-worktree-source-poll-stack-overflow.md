---
schema_version: 1
id: finding.sdk-git-worktree-source-poll-stack-overflow
kind: finding
type: implementation_bug
status: resolved
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
evidence_refs:
- evidence.sdk-worktree-source-stack-repair
audit_refs:
- audit.sdk-worktree-source-stack-rereview
decision_refs: []
repair_evidence_refs:
- evidence.sdk-worktree-source-stack-repair
verification_evidence_refs:
- evidence.sdk-worktree-source-stack-repair
rereview_audit_refs:
- audit.sdk-worktree-source-stack-rereview
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

## 后继具体SDK poll栈缺陷本地闭合

原851的source dispatcher源码91bd与不可变da873/a634真实remove第25请求abort/原OS栈相符。两个后继候选失败也保独立epoch：333aggregate只搬四臂但仍失败；per-operation dd336可dirty拒绝后活，但合法merge仍在旧宽parent poll链abort。不能把这些失败覆盖为成功或将静态格式检查当修复。

最终3958保存源码2df：同步薄入口在构造巨大非Git async future之前按闭合四个Git source operation分流；Git router及四私有boxed future复用原framework调用体。nonGit主体去掉四Git臂后token exact，四helper full tail bytes exact；Sessionowner、mutation guard、参数/DTO/error转换、同三个borrowed输入生命周期、feature-off fallback保持。没有新task、线程stacksize、第二Git检查器/manager/registry、publicAPI或业务权威。

同a634/.45因果pin的实际不可变03e795 Host首次完成TS7与PY7原source请求：create/list、dirtyremove精确framework_error/never/fulloperation/nullhandle/details，完整repo HEAD/refs/index/tree/files与两个worktree快照不变，之后list成功证明Host活；合法merge真实sourceHEAD/files进入main，cleanremove目录消失而branch保留，最后仅mainworktree。完整7rows只归一自己临时根，其请求/结果/错误/副作用全字段一致，不丢字段、不自动default。

同profile原二进制只LLDB target/disassembly无launch测量：entry constructor6080B、router112B，create/list/merge/remove poll分别76272/76752/78400/76576B；nonGit仍909552B但不进入Git4链。原1984派生数漏4KiB probe，现32+4096+1952=6080，旧元/42checksum与原disassembly保；不是OS总stacklimit证明。61Rust/config输入前后与执行档同，build/production strictclippy/git-feature-off check真实0。

framework_final_review本会话真实静态+actualafter源码/consumer/原raw/最后6080Minor增量复审PASS C0/I0/M0，Root据此冻结3958。归档manifest“finalMinorpending”及root333候选README属于当时历史，正文后来已明确minor校正和独审，不假修改旧收据状态。此三链仅结清SDK Git4source致命poll栈缺陷。

Java尚未执行，三语言完整28语料、新发布00782/.46 pin、完整Rust14/语言全套/合同/main与SDK GitBranch语义缺陷消费者验证均另验；旧诊断a634/.45不称安全发布或新pin证明。B/12和宽资源义务不从本局部resolved关闭。
原“修补尚在进行/保持open”是before阶段历史，原正文与discovered_at保留；本段只局部resolved此实现缺陷，不将framework branchswitch消费、Java或当前完整产品/main状态抬高。
