---
schema_version: 1
id: finding.sdk-sandbox-natural-eof-buffered-result
kind: finding
type: implementation_bug
status: resolved
severity: high
primary_focus: failure_concurrency
focus:
- contract_evidence
- time_lifecycle
boundary_ref: boundary.sdk-resources-streams
behavior_refs:
- behavior.sdk-family-shell
- behavior.sdk-resources-streams
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-sandbox-natural-eof-buffered-result-candidate
- evidence.sdk-tool-value-natural-eof-after-candidate
- evidence.sdk-sandbox-natural-eof-repair
- evidence.sdk-sandbox-natural-eof-verification
audit_refs:
- audit.sdk-sandbox-natural-eof-rereview
decision_refs: []
repair_evidence_refs:
- evidence.sdk-sandbox-natural-eof-repair
verification_evidence_refs:
- evidence.sdk-sandbox-natural-eof-verification
rereview_audit_refs:
- audit.sdk-sandbox-natural-eof-rereview
discovered_at: 30d7e219cc7d3aef0f75a19217c82898c363f50d
---

# Sandbox自然EOF吞已缓冲complete结果

## 问题

真实TS首次正常sandboxflow只有outputseq1→EOFseq2，缺完整CompleteResultitem；同二进制同包单次复跑123不能覆盖首次race。

## 触发条件与影响

mod自然EOF取消producercleanupchild，使FacadeStream.next在recv与readycleanup随机select，跳过已发布bufferresult直接EOF。ToolValue普通49rows正确值不因这个另一场景失效，但整9native不能宣全绿。

## 证据

publishedframeworkb909 ee9ff原source/首次failure/同包repeat保原事实。窄adapter修复和具名生产source单test正在准备，不借原4CustomknownActorPASS或其他scope解除。

## 处理记录

同原queue/streamauthority保自然结算，不新增stateAPI；显式取消/发送失败仍原signal+drain。actualredgreen/newbin强原native及独立rereview齐备才resolved，消费者不得降低result/sequence强断言。

## 候选after实际运行

原oldEOF1red→same1green与Hostlib163由adapterreceipt定位；8d819/noPatch b909三完整native首次9pass0fail0skip，强CompleteResult/seq123断言未放宽，73档保。独立framework_final_review正在读，当前specificFinding仍open；不把无reviewcandidate当resolved或wholeSDKmain完成。

## 当前精确本地闭合

独立framework_final_review source+73afterpackage有限PASS0、same1真实redgreen/mod7f1cc/8d819/b909/首次9native/163等引用齐备，specificFinding本地resolved。上文candidatewaitingreader是历史阶段，原ee9fffirstfailure保；最终12source选择/wholegen/currentSDK全门禁/main另验，不借此自动raise41或8lens。
