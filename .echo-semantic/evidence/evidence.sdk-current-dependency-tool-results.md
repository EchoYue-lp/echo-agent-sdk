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

## 2026-11-01 Stage-A 008／e151 当前风险绑定补充

本节只记录该私有源自己的依赖风险结果，不改上方historical observed_at、原source_refs、
supports/limitations，也不把旧纪元的状态重签为本次SDK整体通过。Host／根Polars选择及
锁分别是精确0087353fd78eb73ffaa68b255cf4ef807ca96d4a／
e151fb4587c99463e90c82dbe9d1d4f7e76747be1c7f6241d21b74a0e9976a62。

本SDK真实锁audit、原008五源视图audit、官方deny0.20.2四检查、原capture及独立caller
均自然0；原14项policy控制通过。deny698963B完整raw SHA为
e9b2664927608a4b5e4875a1c05f47ea12ad6e67fd3ef95182187d904919f872，
严格原parser核对恰原SDK49组107身份，其它diagnostic0。原4项ignore和b183策略保持，
不把duplicate写成零warning或风险已消失。

各自默认fresh获取的官方RustSec HEAD为2295466c4e9df26647fd67d8fa1c705ac2fc7f81，
tree为a95b9514c0f6e6af352c9c7845720fe73a964d1e；实际SSH传输只限同官方仓库，
stored origin仍HTTPS。Deny从本批SDK新DB正常Git预热后自行fresh fetch，不借F／CLI资格。
Root与独立reader已核自然回执、完整输出和三字段提案，仅维护本Stage-A baseline的
lock_sha256/framework_pin/current_lock_validation；原受接受身份／政策／授权／工具／
命令不变，没有新增人类风险接受。

原件在主根内本地ignored sdk-e151-fresh-risk-r01目录，结果索引SHA为
cb29048185e4815ea40520c78f96c1da1c314d78b108c58a3cdf9aa173fd845b；归属见
[当前SDK风险说明](../../docs/sdk/governance/dependency-warning-policy-proposal.md)。
这些本地原件不冒正式Git归档；822／0dac／013原失败和旧caller结果均保持历史事实。
完整17、Host／三语言、动态source生成、当前task Source/final及mainCI另验；正式仓已具
自身资格按原纪元保留，历史115／402不默认当前Stage-A阻塞。本节不升级其它Finding或global闭合。

## 下列为原历史观测正文（原话与运行纪元保留）

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
