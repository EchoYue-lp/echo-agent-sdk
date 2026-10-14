---
schema_version: 1
id: evidence.sdk-current-dependency-tool-results
kind: evidence
observed_at: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
source_refs:
- AGENTS.md
- deny.toml
- docs/sdk/governance/dependency-warning-policy-proposal.md
- docs/sdk/governance/evidence/sdk-current-dependency-policy/README.md
- docs/sdk/governance/evidence/sdk-current-dependency-policy/policy-result.json
- docs/sdk/governance/evidence/sdk-current-dependency-policy/SHA256SUMS.json
- docs/sdk/governance/evidence/sdk-current-dependency-policy/source-before.json
- docs/sdk/governance/evidence/sdk-current-dependency-policy/source-after.json
- docs/sdk/governance/evidence/sdk-current-dependency-policy/audit-receipt.json
- docs/sdk/governance/evidence/sdk-current-dependency-policy/deny-receipt.json
- docs/sdk/governance/evidence/sdk-current-dependency-policy/audit.log
- docs/sdk/governance/evidence/sdk-current-dependency-policy/deny.log
- docs/sdk/governance/evidence/sdk-current-dependency-policy/rustsec-database-manifest.json
- docs/sdk/governance/evidence/sdk-current-dependency-policy/deny-database-provenance.json
- docs/sdk/governance/evidence/sdk-current-dependency-policy-after-feature-wiring/source-before.json
- docs/sdk/governance/evidence/sdk-current-dependency-policy-after-feature-wiring/source-after.json
- docs/sdk/governance/evidence/sdk-current-dependency-policy-after-feature-wiring/baseline-comparison.json
- docs/sdk/governance/evidence/sdk-current-dependency-policy-after-feature-wiring/SHA256SUMS.json
supports: []
limitations:
- Toolcommands passed but mergeacceptance blocked by ruleconflict
- No useroverride/ignore/riskacceptance ordependencygraphchange
- A futurepin/lock/source needs actualrerun; cannotblindbind oldresult
---

# current-dependency-tool-results

## 支持的结论

当前工具真实事实：audit--denywarnings exit0/noWarnings；denyworkspaceallfeatureslockedcheck exit0/4checksOK但50warningduplicate。官方freshRustSec6aed846本地远程HEAD同/clean/1310/658lockeddeps，7inputs前后不变、4原exceptions不增，20档逐SHA核。工具结果不是AGENTS零warning规则通过，SDK合并仍BLOCK；proposal未获人批。

## 来源与范围

原归档receipt/输入hash/二进制保持不变。当前观察只绑定模型与可读取归档，不把原epoch冒称最终current全套通过。

## 已知缺口

Toolcommands passed but mergeacceptance blocked by ruleconflict；No useroverride/ignore/riskacceptance ordependencygraphchange；A futurepin/lock/source needs actualrerun; cannotblindbind oldresult。

## 新featurecut工具复验

最新19档after-feature-wiring checksumbc9bde保manifest7ab/locka42/7inputs前后同与fresh6aed两个DB。audit0/deny0且duplicate50的每crate实际版本数组与proposal完全匹配；不是旧20包自动转currentpass。原20包保持原字节，仅工具结果支持，AGENTSzeroWarning规则Finding仍open未humanaccept。两新包独立reader进行，fullgen/globalstrict/main另验。

## 新规则文稿切点的有限观察

本次观测Git`9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d`对应实际新治理文稿/正式TTL归档切点，源码摘要`0a13f994adc90d29746d23c7e071dfb55d8c12eb92c5000b1785092bdc10392f`。原marker`source:pending`及原正文/inputhash/runtimeepoch保；全部source_refs在本Gitblob=currentbyte/anchors核实，旧Git569仍可恢复，不把新的governance文稿Git当过去运行日期或重新执行。

本对象只观察原receipt/人选择/当前政策或接受门禁文稿。依赖用户只明确接受原4c940的50groupduplicate边界，不批准12source/B5或外部CLI-W；28profile12missing correctfail/fullgen未通过，oldcounts/deferred14仍旧27c库存，不autoacceptRustAPI。

原supports/limitations保留，观察不提升41family/8lens、Finding闭合或accepted成员。运行型证据仍原569/17cf/bb4f/b909，不借newcut冒freshSDK代码执行；仅当时原强cases及已完成新scope可用。
