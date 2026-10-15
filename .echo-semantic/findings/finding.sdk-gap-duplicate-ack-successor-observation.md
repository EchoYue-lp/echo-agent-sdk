---
schema_version: 1
id: finding.sdk-gap-duplicate-ack-successor-observation
kind: finding
type: implementation_bug
status: resolved
severity: medium
primary_focus: failure_concurrency
focus:
- result_side_effect
boundary_ref: boundary.sdk-events-recovery
behavior_refs:
- behavior.sdk-events-recovery
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-events-resources-finite-observation
audit_refs:
- audit.sdk-events-resources-finite-failure-concurrency
decision_refs: []
repair_evidence_refs:
- evidence.sdk-events-resources-finite-observation
verification_evidence_refs:
- evidence.sdk-events-resources-finite-observation
rereview_audit_refs:
- audit.sdk-events-resources-finite-failure-concurrency
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


## 已提交修复与有限后验进展（仍open）

三caller修复已ff3真提交，先等待同fullstream且from/watermark超过前次的successor paused gap再设announced基线，模型仍parked/release未发，duplicateACK之后原count等式保；旧假设4gap是dup新effect的受控反例、改正同controlledcount0以及TS4/PY4/Java4rows有限Mac场景已实际独审PASS0。原namefilter skip如包所示，不冒完整入口或新Linux；原ACK产品owner/seq authority不重开。

当前材料观察Git `ff3de936d22c6d5f4e68ece646f0b4cfd77e7c56`仅定位已提交源码和正式包；原722/merge5d407失败运行/发现时间不改，新remote原case仍待。以下实际Git来源可读，不把本地focused当Linux完整验收：

- `sdks/python/tests/test_gap_ack_host.py`，SHA256 `aa462130216730a6541982799e07b679eb4ceb2e9590c6cd0ec161318c3045eb`。
- `sdks/typescript/test/gap-ack-host.test.js`，SHA256 `3ad9a6caa40611e9a18a17f8ea9e3530320549454b482028dd9cb810bd719b5e`。
- `sdks/java/src/test/java/com/echoagent/sdk/GapAckHostTest.java`，SHA256 `017fa75ec0ce091e7def325dfc7dfd2048fbefbe6bfe2067aaccc69aef468737`。
- `docs/sdk/governance/evidence/gap-successor-pause-oracle/README.md`，SHA256 `971ec4281009739b05701ac7db99a4227e0bd84953d01495d70c84bcb7c8a5da`。
- `docs/sdk/governance/evidence/gap-successor-pause-oracle/source-after/manifest.json`，SHA256 `a052d205badc50be2303fd20470799bff48db08c5ecce51733dcbdb67a3b3527`。
- `docs/sdk/governance/evidence/gap-successor-pause-oracle/native/receipt.json`，SHA256 `0c5a251e23860e16470e3f1845ba5d96e3677b8ed3dab042ab30dc151638fda2`。
- `docs/sdk/governance/evidence/gap-successor-pause-oracle/native/controlled-final-receipt.json`，SHA256 `fb2df55a6784b92725963870585b7c28e634c3254a81428e2633cce61a7ef16e`。
- `docs/sdk/governance/evidence/gap-successor-pause-oracle/independent-review.json`，SHA256 `eb72140598ccea84de99f51eae878fbb131bdcede475dd95ecd4a9f8a7849420`。
- `docs/sdk/governance/evidence/gap-successor-pause-oracle/SHA256SUMS`，SHA256 `1a3dca8639303edafdf3aad8e3caa8bca38db4e9fb1b61fd9ebc5c8dbb40486d`。

源4tests已变化，先前第四whole800完整0继续保它自己的执行cut而非声称当前all800exact。生产61Rust/Cargo输入与锁未因此变化；本次不会重复原14Rust或本地大target完整门禁。新CI按focused开发门禁后实跑方案另执行，暂不关闭本Finding或提升Behavior/库存/全goal完成；B3/12、第三工具交付和官方原30d门禁各按真实当前进展处理。


## 本specific缺陷的e58后继局部闭合

原open/原因未知/新Linux待等正文保其原观察阶段，不改失败日志/发现点/既有强断言。当前只本项repair、verification、rereview由指定原owner承接：`evidence.sdk-events-resources-finite-observation` / `audit.sdk-events-resources-finite-failure-concurrency` 的本次具名段/挑战。原真实gap4在dupACK2前已经到达，ff3三caller只增强完整successor接收barrier，原notification count/full frame不变式与e58无filter完整语言/Host验证相接，未改变Host/客户端ACK业务。

本次后验是[e58官方语言job116293151777](https://github.com/EchoYue-lp/echo-agent-sdk/actions/runs/38696176970/job/116293151777)与[Hostjob116293152091](https://github.com/EchoYue-lp/echo-agent-sdk/actions/runs/38696176970/job/116293152091)各自自然终态success；head e58a3e0c07cf0b6b24d570df07063301a888ddc6、actual PRmerge e9c7275c6f5df5652e555aa4f457c65870676b34、tree c48b54ec7a58bfe855d2724f777b1d605789b7a2与已提交源码树一致。语言原raw `/Users/ls/MyWork/code/ylp_agent_learn/lp-agent/.worktrees/admin/sdk-e58a3e0-ci-evidence/116293151777.log`，200420B/SHAaec6371714b2d2bd23612b3b6d139acccf507a541db327ef13c5c744f5115cce；TS343、Python427+18子测试、完整Maven test按set-e成功且三个quickstart均实际到达/finalmarker通过，Node20.20.2、Temurin17.0.20+1。成功静默Maven没有打印Java数量，也没有下载XML，不编Java总数。Host原raw `/Users/ls/MyWork/code/ylp_agent_learn/lp-agent/.worktrees/admin/sdk-e58a3e0-ci-evidence/116293152091.log`，164620B/SHAecc88119b62e9a9311db1dcc7860082a1b18131d584b66889ab6fa1defdf56d8：default35/core111/bridge28/facade44/noBridge selected1+24filtered各原profile通过，不重叠求和；managed restart case ok不倒推原passive-exit原因已查明。两job warning/error annotations均0，只用于各自原切点，不称全部CI或SDKmain已过。Admin language-host-success-facts.json保两job已成功组合；单独language-terminal-verification内Host未终态的资格文字是较早收据阶段，不重写原JSON或误作当前阻断。

仅此已证局部缺陷resolved，不接受B/12风险、不关闭Host退出未知cause或其它原open、不宣SDKmain/continuity1173/归档精简通过。新ACK投影竞态三链与旧#120不作为本项修复证据。
