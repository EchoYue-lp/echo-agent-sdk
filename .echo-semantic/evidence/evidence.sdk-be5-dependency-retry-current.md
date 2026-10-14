---
schema_version: 1
id: evidence.sdk-be5-dependency-retry-current
kind: evidence
observed_at: 47579c895657c182eceacfbb63865b04ec55a4a1
source_refs:
- docs/sdk/governance/evidence/sdk-dependency-be5-current/retry-once/README.md
- docs/sdk/governance/evidence/sdk-dependency-be5-current/retry-once/result.json
- docs/sdk/governance/evidence/sdk-dependency-be5-current/retry-once/accepted-policy.md
- docs/sdk/governance/evidence/sdk-dependency-be5-current/retry-once/source-after.json
- docs/sdk/governance/evidence/sdk-dependency-be5-current/retry-once/audit-receipt.json
- docs/sdk/governance/evidence/sdk-dependency-be5-current/retry-once/deny-receipt.json
- docs/sdk/governance/evidence/sdk-dependency-be5-current/retry-once/db-provenance.json
- docs/sdk/governance/evidence/sdk-dependency-be5-current/retry-once/SHA256SUMS.json
supports:
- rule.framework-runtime-authority
limitations:
- 仅这次lock/pin/group来源可接受，不未来数量自动放行
- 不替代合同或5s物理policy批准
---

# be5-dependency-retry-current

## 支持的结论

初次officialDB网络失败原16包保留，一次有依据retry实际audit0/deny0、两officialDBfreshclean同remote6aed/1310advisories/658lockeddeps。新lock442/be5八Git包，50group每crate实际versions恰匹配用户已接受表，其它WARN0，原4exceptions与配置不变/7inputs前后current一致；独立sdkReader复审PASS0C0I0M。

## 来源与范围

本对象绑定真实冻结候选Git`47579c895657c182eceacfbb63865b04ec55a4a1`，具体运行仍由所引原receipt/time/inputmanifest恢复，观察不是重新执行。旧569/9db与b909包保持原epoch不批量重标。

## 已知缺口

仅这次lock/pin/group来源可接受，不未来数量自动放行；不替代合同或5s物理policy批准。
