---
schema_version: 1
id: finding.sdk-managed-restart-passive-host-exit-timeout
kind: finding
type: implementation_bug
status: risk_accepted
severity: medium
primary_focus: time_lifecycle
focus:
- failure_concurrency
- contract_evidence
boundary_ref: boundary.sdk-extension-bridge
behavior_refs:
- behavior.sdk-extension-bridge
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-source-continuity
- evidence.sdk-managed-passive-exit-residual-human-decision
audit_refs:
- audit.sdk-managed-retirement-rereview
decision_refs:
- evidence.sdk-managed-passive-exit-residual-human-decision
repair_evidence_refs: []
verification_evidence_refs: []
rereview_audit_refs: []
discovered_at: 0ebe15f66d0aa57c7b7a92ae68cd1be106114ae9
---

# Managed restart测试第一次Host被动退出超时（根因未明）

## 问题

0ebe实际Host job116226422816在managed_commit_disconnect_restart_replays_same_receipt报Error: Elapsed(())。源码唯一裸Elapsed传播定位第一次`timeout(Duration::from_secs(10), host.child.wait()).await??`。第一场景drive与projected_items>0/first.is_ok已通过，第二Host spawn/restart和后续receipt比较尚未执行。原bridge27pass/1fail，不写后续facade已执行。

## 触发条件与影响

官方ACP首场景返回后期待原Host passive exit，10秒未观察完成。尚不能证明EOF已物理抵达、哪个cleanup阶段阻塞、deadline后是否紧随退出，也不能推出ManagedStore数据丢失、receipt不一致、B5秒策略改变或所有Host闭合失效。

## 证据

原SDK Git `0ebe15f66d0aa57c7b7a92ae68cd1be106114ae9`的echo-sdk-host/tests/extension_bridge_e2e.rs#managed_commit_disconnect_restart_replays_same_receipt，SHA7e2312d1fec10663fbde463d989a008a566dd75723a61ce2f26423dfd7dfa2ba。运行38674982406/job116226422816原raw125473B/SHAf3e74a8ea43aef15cc3f256b06365a0a29d02998ab2ba1795e5d5ce3f80d9a86；Admin `sdk-managed-restart-exit-diagnostic/investigation.json`（5179B，SHA256 `05bd6192742a8819792048de7d07e68fb647d338017ceef0220dd472cc7607c9`）。仅Admin原件不伪新Git sourceRef。

## 处理记录

现只新增具体open义务。诊断候选只为原timeout附上下文、复用stderr/counters，不增deadline、kill/retry或改receipt断言；尚未提交时after sourceRef为null。已有本机exact1case通过不足关闭原Linux超时，必须保原失败/核新诊断运行。无新增公共API/权限或productdecision；三链空保留待真实因果与后验，原开放B/12不由此变更。

## 2026-10-19：当前本地验证与精确main Linux待验

状态仍open，原Linux运行38674982406/job116226422816的10秒第一次Host被动退出根因仍未知。当前8481ba5823aa190cf76859d6af0570898c432bbd原完整Rust矩阵里的相同managed restart case已在本机实际通过，诊断保原10秒/同Host/passive wait，无kill/retry-as-pass；这是当前revision的本地回归证据，不能称已复现或修复历史Linux根因。没有填写虚构repair/verification/rereview三链。唯一剩余验收入口是最终合入后的精确SDK main Linux原套件及其原日志/CI Artifact；若失败按原phase诊断继续，不能以本地PASS或已经合并为豁免。平台验收成功后仍分别保“当前回归通过”和“历史根因未知”，不得杜撰生产修复。


## 2026-10-20 同生产源码首次Linux PASS的强反证

失败源是SDK0ebe15f66d0aa57c7b7a92ae68cd1be106114ae9，首次后继Linux PASS精head为e58a3e0c07cf0b6b24d570df07063301a888ddc6（原run38696176970，同bridge28/0fail/0ignore）。逐Git对象核：两版Host production src树都为b601874d724c56f68c1d22cccc1fb814bb86f3f0；Protocol src树同6fff1a9cd6b8bfd8167b88dc4b1c6b24c717c330；Cargo.lock Git blob同017220fd23f465cbdbbced2c87da03a01e7050d8；rust-ci.yml Git blob同96ae6cbc5bd46f712a55096d74040ca8d42c0140，CI命令未变。F00782/ACP2.1.0/Tokio1.53.1未变。唯一Host文件差异是原10秒passive wait失败后的额外诊断，没有延长期限、强制kill或retry-as-pass。由此首次PASS没有可指认的生产repair，不能借后来的Factory lateUnit/CustomAgent等修复归因本次历史超时。

原失败发生first-host-exit十秒wait处：首场景drive、projected_items>0和first.is_ok已过，第二Host尚未spawn。缺的是那次失败Host是否实际读EOF、阻在哪个cleanup/callback/Run、还是cleanup后runtime进程退出的现场证据。已审链中Responder持outgoing sender而非ChildStdin；transport future持stdin，connect_with foreground结束在非closing分支会drop background/transport，没有找到确定自持环。EOF后的ACP pendingError/cancel→原managed invoke释放guard/lease→F boundeddrain路径已有；这只是已检查路径与未找到反例，不能推出原失败没有阻塞。Tokio stdin blocking read仍仅候选，不是已证原因。

当前main.rs的Git blob8f0bec708035e625848f0613f6979e03fbd97b3d及bounded_stdio.rs的3edcc69eedeccb32587e65aae58a05113e0918b3自失败源到当前86fe逐字同；这不声称整个后继Host源码未变化。当前a17/9ef原14和whole通过是新的匹配回归资格，不是历史因果repair。原失败日志与首PASS日志由原读者按完整SHA独核，保各自运行cut；Git树/锁/CI字节对照的本任务ignored记录managed-history-production-comparison.json1223B/79b0a71990e58cbbdf9029c9c019016069fd6885339a4d8cee56c6878d996bcb不是新的运行或报告事实源。e58原必要tag已存在，不新增tag/报告/测试。

## 当前未知义务与最后裁决边界

状态继续OPEN，type保持implementation_bug，repair/verification/rereview三链不填；原Audit关联只保存具体反证与unknown frontier。最后精SDKmain Linux原套件及日志只能再提供当前回归：若PASS仍分别记录“当前回归通过”和“历史根因未知”，不能以诊断/一次PASS关闭。所有可执行工作结束后，若仍缺真实因果，只把这一精确残余交由人决定接受或继续真实RCA；risk_accepted须人的decision_refs，本材料没有替人裁决，也不删原10秒断言或新增CI。


## 2026-10-22 真实人的历史残余接受

人的明确选择已到齐，当前状态为risk_accepted，不是resolved。唯一裁决见
`evidence.sdk-managed-passive-exit-residual-human-decision`；两个bridge复用同一决定。
原run38674982406/job116226422816的首次Host10秒被动退出超时与首因UNKNOWN完整保留。
当前精main a9f267f6 的原Linux bridge整套31通过，其中具名managed restart case实际ok；
原10秒wait返回后代码丢弃首Host ExitStatus，不能称其原生退出码0。九CI与冷恢复证明当前回归，不提供旧失败因果repair。
repair_evidence_refs、verification_evidence_refs、rereview_audit_refs继续空，不把人的风险接受填成修复三链。
不延长/删除10秒断言、不增加kill/retry-as-pass、不豁免后续同类新失败，也不改变Store/receipt/claim权威。
上文OPEN/仍待段落保其历史时态；本日期决定是当前处置，不删除负证据或制造旧现场。
