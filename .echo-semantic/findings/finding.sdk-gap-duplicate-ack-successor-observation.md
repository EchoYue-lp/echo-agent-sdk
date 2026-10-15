---
schema_version: 1
id: finding.sdk-gap-duplicate-ack-successor-observation
kind: finding
type: implementation_bug
status: open
severity: medium
primary_focus: failure_concurrency
focus:
- result_side_effect
boundary_ref: boundary.sdk-events-recovery
behavior_refs:
- behavior.sdk-events-recovery
rule_refs:
- rule.framework-runtime-authority
evidence_refs: []
audit_refs: []
decision_refs: []
repair_evidence_refs: []
verification_evidence_refs: []
rereview_audit_refs: []
discovered_at: 722d09b1a0837f170d4af19cbc3e417d2131b5f3
---

# ACK重复通知断言未等待前次ACK的后继paused gap

## 问题

原Linux完整语言TS336通过及TSquickstart执行，Python413通过/2失败/18子测试，Java未达。其中 `test_gap_ack_snapshot_continuation[True-window]` 在line132记录announced=3后立即重复ACK，再run.get，实见4!=3。新增通知是sequence4、reason ack window exceeded/live delivery paused的gap，原log完整保存；不能从4!=3直接归为重复ACK产品错误。

Root已实际受控同Host确定：gap4在duplicateACK之前已由前一个合法ACK产生，原测试基线只等通知数量>旧before，尚未等successor paused gap结算就捕获announced，导致并发观察错把先前effect当重复ACK新effect。此是现有测试oracle时序问题，不新事件权威、吞ACK或宽去重承诺，也不重开已闭gap水位/ACK提交缺陷。

## 触发条件与影响

发现基准722/PR实际merge `5d407e160bb5c7ade01be1980ed9a4e5a6888d17`，不冒裸head执行。原本地第四whole800/TS336/PY41518/J35576XML真实0保macOS来源；新Linux反例不倒改旧执行/不批量失效8Host风险审。

## 证据

原Linux语言log Admin `/Users/ls/MyWork/code/ylp_agent_learn/lp-agent/.worktrees/admin/sdk-722-ci-evidence/116172597308.log`，198565B，SHA256 `70548f9331858291e374f6e751581cfcf854d8767900d03738d5609041880547`，内容只读。当前新证据尚未正式Git，不伪source:pending Evidence或futureGit。

- `sdks/python/tests/test_gap_ack_host.py`，原722 Git源码SHA256 `79135f92f27f211393f1919b14b6273f3977e0c1ab208a4fad1b06a96910fe10`。

## 处理记录

最小后验应以真实原stream/水位/accepted事件链等待前次ACK对应paused successor，随后再记录exact通知基线并提交duplicateACK；必须保重复ACK无新effect强断言及连续性/fullhandle、futureACK/两window路径，不以任意sleep或容忍额外通知削弱断言。受控before/after源与raw及独审后才能局部resolved，当前open；B物理结算不由本测试关闭。

此open候选仅本测试support或CIbootstrap边界，不新增family/API/权限/RiskAccepted。原四已解决Mac具名缺陷保持闭合，56/4是原切点而非含本新缺陷的当前最终总数。全goal30d UTF8工具修复第三repo仍待授权，不能改工具或scope来关闭这些记录。


## 正式CI保存材料的当期观察（f68）

本段当期观察Git `f68cb49bf421ecdb71dfa4775495c363e31b3324`仅定位正式保存的722运行材料，不把运行时间或源码测试执行标成f68。原发现点`722d09b1a0837f170d4af19cbc3e417d2131b5f3`、PR实际merge5d407、候选所有原Map观察/scenes/status/锚点与对应测试源码SHA保持。正式来源逐路径与Hash如下，可从该Git读取：

- `docs/sdk/governance/evidence/sdk-722-ci-results/README.md`，Git f68 SHA256 `dcd6a9e4922a05527b189a2c03108a3ac73936d5107d022600dada1e570f0b27`。
- `docs/sdk/governance/evidence/sdk-722-ci-results/manifest.json`，Git f68 SHA256 `4eb02f38b55191bfa761cd892b667733dcb26d4462eb655cfd2a22cbd379443e`。
- `docs/sdk/governance/evidence/sdk-722-ci-results/SHA256SUMS.json`，Git f68 SHA256 `c22f92784ad1874ffcb0bb70ad312965e50bda78dcd69871230b2073006f1e44`。
- `docs/sdk/governance/evidence/sdk-722-ci-results/jobs/116172597308.log`，Git f68 SHA256 `70548f9331858291e374f6e751581cfcf854d8767900d03738d5609041880547`。

正式完整八job范围为5success/3failure；TS336及quickstart通过，Python413通过/2失败/18subtests，Java未达。两个合同job原13连续性阻断与旧证明分别保留，不从CI fail泛化未知生产缺陷。PID原Linux[]唯一cause仍unknown，Darwin2.5925s排队原反例不是该CI唯一cause。bootstrap单input实施只原配置/顺序依据，新remote error结算尚未证明，不提前resolved。原四已闭Mac缺陷不重开、B/12未决保持。

原上文“第三repo工具交付权限待批准”属于候选准备时历史；Root后续已获得第三repo工具修复交付授权，实际工具修补/源码SHA/发布main与新CIjob结果尚未回报，本候选不冒已交付或官方原30d门禁已0。
