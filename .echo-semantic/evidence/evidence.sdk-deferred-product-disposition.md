---
schema_version: 1
id: evidence.sdk-deferred-product-disposition
kind: evidence
observed_at: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
source_refs:
- docs/sdk/governance/evidence/sdk-deferred-product-disposition/receipt.json
- docs/sdk/governance/evidence/sdk-deferred-product-disposition/group-membership.json
- docs/sdk/governance/deferred-capabilities.md
supports:
- rule.framework-runtime-authority
limitations:
- Oldinventory snapshot; final sourcegenerator must recompute after successor pin
- No acceptance changed, no extra API, no identity count completion denominator
---

# deferred-product-disposition

## 支持的结论

旧27c库存epoch canonicaldeferred1448唯一分入14个产品能力组，无交叉accepted/遗漏/alias重复，全部有source_paths。每组具已接受主能力、唯一framework机制、Rust/Host保留对象与延期原因/将来精确接受条件。只是库存处置完整，不是能力测试或新sourceinventory验收。

## 来源与范围

原归档receipt/输入hash/二进制保持不变。当前观察只绑定模型与可读取归档，不把原epoch冒称最终current全套通过。

## 已知缺口

Oldinventory snapshot; final sourcegenerator must recompute after successor pin；No acceptance changed, no extra API, no identity count completion denominator。

## 新规则文稿切点的有限观察

本次观测Git`9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d`对应实际新治理文稿/正式TTL归档切点，源码摘要`0a13f994adc90d29746d23c7e071dfb55d8c12eb92c5000b1785092bdc10392f`。原marker`source:pending`及原正文/inputhash/runtimeepoch保；全部source_refs在本Gitblob=currentbyte/anchors核实，旧Git569仍可恢复，不把新的governance文稿Git当过去运行日期或重新执行。

本对象只观察原receipt/人选择/当前政策或接受门禁文稿。依赖用户只明确接受原4c940的50groupduplicate边界，不批准12source/B5或外部CLI-W；28profile12missing correctfail/fullgen未通过，oldcounts/deferred14仍旧27c库存，不autoacceptRustAPI。

原supports/limitations保留，观察不提升41family/8lens、Finding闭合或accepted成员。运行型证据仍原569/17cf/bb4f/b909，不借newcut冒freshSDK代码执行；仅当时原强cases及已完成新scope可用。
