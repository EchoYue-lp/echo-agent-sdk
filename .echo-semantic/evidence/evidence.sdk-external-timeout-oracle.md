---
schema_version: 1
id: evidence.sdk-external-timeout-oracle
kind: evidence
observed_at: 47579c895657c182eceacfbb63865b04ec55a4a1
source_refs:
- docs/sdk/governance/evidence/sdk-external-timeout-oracle/README.md
- docs/sdk/governance/evidence/sdk-external-timeout-oracle/archive-manifest.json
- docs/sdk/governance/evidence/sdk-external-timeout-oracle/SHA256SUMS.json
- docs/sdk/governance/evidence/sdk-external-timeout-oracle/raw/source-manifest.json
- docs/sdk/governance/evidence/sdk-external-timeout-oracle/raw/scope.patch
- docs/sdk/governance/evidence/sdk-external-timeout-oracle/raw/actual-java-timeout-row.json
- docs/sdk/governance/evidence/sdk-external-timeout-oracle/raw/unit-after.log
- docs/sdk/governance/evidence/sdk-language-be5-reviewed-final/final-summary.json
supports:
- behavior.sdk-family-research
limitations:
- 外部provider时点响应不保证持续可用
- fixture/oracle正确性不增加schema/acceptedAPI
---

# external-timeout-oracle

## 支持的结论

仅外部研究timeoutoracle修：保原raw11items+checksum与原49rowresults/scenario/baseline/host逐byte正式18checksum/19file，允许官方客户端原已accepted timeout typederror规范形状且不将任意错误当pass。原expectedprovider429/网络失败事实保；源码与18归档增量独审PASS0，当前官方whole368+18测试/三语真实运行通过；不改Host/SDKproduction/参数或隐藏测试失败。

## 来源与范围

本对象绑定真实冻结候选Git`47579c895657c182eceacfbb63865b04ec55a4a1`，具体运行仍由所引原receipt/time/inputmanifest恢复，观察不是重新执行。旧569/9db与b909包保持原epoch不批量重标。

## 已知缺口

外部provider时点响应不保证持续可用；fixture/oracle正确性不增加schema/acceptedAPI。
