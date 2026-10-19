---
schema_version: 1
id: finding.sdk-managed-restart-passive-host-exit-timeout
kind: finding
type: implementation_bug
status: open
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
evidence_refs: []
audit_refs: []
decision_refs: []
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
