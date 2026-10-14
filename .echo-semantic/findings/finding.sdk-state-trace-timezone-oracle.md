---
schema_version: 1
id: finding.sdk-state-trace-timezone-oracle
kind: finding
type: implementation_bug
status: open
severity: medium
primary_focus: contract_evidence
focus:
- result_side_effect
boundary_ref: boundary.sdk-language-clients
behavior_refs:
- behavior.sdk-language-clients
rule_refs:
- rule.framework-runtime-authority
evidence_refs: []
audit_refs: []
decision_refs: []
repair_evidence_refs: []
verification_evidence_refs: []
rereview_audit_refs: []
discovered_at: 9a59943e70da06b1af231b1fb22bddc879885f28
---

# State与Trace夹具把时区字符串格式误当时间义务

## 问题

原CI另2条not ok是State/Trace deepEqual时间文本：fixture固定2026-10-13T13:00:00.123456789+08:00，Linux Host实际回2026-10-13T05:00:00.123456789+00:00，同一瞬间和纳秒；Trace started_at/finished_at同类。SDKcap已核framework DateTime<Utc>及local_rfc3339输出：内部UTC，API按Host本地偏移。这个测试oracle擅假+08不属于新公共时区产品政策，不得为修fixture改变生产序列化/纳秒精度/合同。

## 触发条件与影响

本地macOS官方793输入完整语言329/Python394+18/Java351与75XML实际0保留原平台/sourcecut，不能推Linux原生完整通过。本次Linux旧eebd语言job在新9a配置提交后权威终态Cancelled，取消不是成功或完整TAP终态。新运行38643983612仍live，不能预写其结果。

## 证据

原日志仅保存于Admin `/Users/ls/MyWork/code/ylp_agent_learn/lp-agent/.worktrees/admin/sdk-pr5-eebd-ci-evidence/language-cancelled.log`，166086字节，SHA256 `d4a85d86d1af20f0025d157e66dceed3b4893059c2ae67da2370f254f4dad2ca`；未制造futureGit或source:pending Evidence。发现观察Git `9a59943e70da06b1af231b1fb22bddc879885f28`只是当前冻结源码定位，原运行属于eebdCI，不冒9a执行。实际相关源码Git字节如下：

- `sdks/typescript/test/state-trace-delivery-host.test.js`，SHA256 `c882cbedee70a25bd29e29f2396a774396e66cbc129d79dc2e846add80df3b37`。
- `sdks/shared/testing/state_trace_delivery_fixture.py`，SHA256 `1f26d96802f5f1acf677abba02196126226074c141b1820fb2cb4935beec3f25`。

## 处理记录

时区lane先只读方案，修复应分别核exactinstant/nanoseconds及实际accepted本地输出格式，冷恢复/完整DTO/大整数/失败零effect原断言不得弱化。本候选未改fixture/Host/契约，也没有Linux修后验证，保持open。

这些是SDK测试/证据夹具缺陷，复用既有language lifecycle与测试支撑边界，不新增runtime/API/源接受策略/权限或第42能力族。原B三个宽Finding与12项人选择仍独立；本Finding不得借新run尚进行、旧macOS绿或编译budget配置而自动resolved。正式repair/verification/rereview及raw归档真Git冻结后再补三链，当前引用数组为空明确未闭合。


## 后继CI状态观察（原候选之后）

原上文“38643983612仍live”是候选准备当时的状态。Root后续已确认该运行终态failure；本Finding尚未读取其新原始失败日志，不据结果名推断它与旧eebd反例同cause，也不声称修复已验证。原执行版本、发现观察和旧日志Hash不改变。
