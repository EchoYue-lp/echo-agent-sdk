---
schema_version: 1
id: evidence.sdk-computation-45-candidate
kind: evidence
observed_at: source:pending
source_refs:
- docs/sdk/governance/evidence/sdk-computation-45-candidate/manifest.json
- docs/sdk/governance/evidence/sdk-computation-45-candidate/native-cross-language-comparison.json
- docs/sdk/governance/evidence/sdk-computation-45-candidate/ACTUAL-RESULTS.md
- docs/sdk/governance/evidence/sdk-computation-45-candidate/corpus.py.txt
- docs/sdk/governance/evidence/sdk-computation-45-candidate/inputs.json
supports:
- behavior.sdk-family-statistics
- behavior.sdk-family-content-guard
- behavior.sdk-family-structured-output
- behavior.sdk-family-data
- behavior.sdk-family-chart
limitations:
- 45 actual native rows/9oracle groups/16typed failures perlanguage on old64c7 only;
  no later189ff source claim.
- Three adjacent Pivot null/index/unknownaggregate defects were independently found
  after this epoch, so old pass cannot close their current findings.
- Exact accepted operations only; no new runtime structured repair configuration or
  fullRustidentity parity.
---

# 计算簇45条共同语料候选

## 支持的结论

原64c7 Host上三原生consumer各45rows、9oracle groups、16typed failures，完整operation/argument/result/error及导出UTF8/JSON字节与sentinel保留对账。原始comparison说明只归一化临时root、WireValue map关联顺序和未承诺tieorder的Polars组/pivot顺序，其余raw数据保留。

## 来源与范围

该epoch覆盖统计、Guard、structuredvalidate与部分Data/Chart代表性场景。它不是正式已发布pin，也不能证明其后Guard/Pivot189ff的新实现。后续审查发现Stringnull/Null列名碰撞、nullindexjoin丢行、unknownaggregate默认值三个实际问题；各自新的红绿/复审来源需单列，不用原45绿掩盖。

## 已知缺口

当前新Host组合匹配语料及独立复审待归档。structured_output只接受validate，不添加运行时repair能力；未接受参数/时间接口不能为测试新增。代表性行为模型已建立，已知缺陷清楚归属，不要求穷举任意未定义组合才能建立能力图。
