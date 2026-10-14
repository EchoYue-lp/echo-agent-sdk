---
schema_version: 1
id: evidence.sdk-compatibility-gates
kind: evidence
observed_at: 30d7e219cc7d3aef0f75a19217c82898c363f50d
source_refs:
  - docs/adr/0001-sdk-repository-boundary.md
  - docs/sdk/README.md
  - echo-sdk-host/src/core_profile/state.rs
  - echo-sdk-protocol/tests/facade_inventory.rs
  - scripts/check-language-sdks.sh
  - sdks/typescript/test/catalog.test.js
  - sdks/python/tests/test_catalog.py
  - sdks/java/src/test/java/com/echoagent/sdk/FacadeParityTest.java
supports: [behavior.sdk-repository-ownership, rule.framework-runtime-authority, asset.sdk-source-product]
limitations:
  - covers source repository migration and compatibility gates, not published packages or full SDK capability governance
---

# Accepted compatibility gates

## 支持的结论

三语言 catalog 测试和语言脚本直接读取 accepted external contract；全量 inventory 数量、
intrinsic membership hash 与非 external identity 的存在性不再作为兼容承诺。
分类、alias、schema 与生成器一致性仍接受结构检查。

Rust 1.99 把 AtomicUsize::fetch_update 标记为 deprecated。Host 结算计数器改为
compare_exchange_weak 循环，保持饱和减一与原有内存顺序，继续使用 Rust 1.95 已有 API。
不新增 runtime、协议方法、accepted identity 或 framework dependency revision。

## 来源与范围

范围是本次独立仓源码门禁收口；生产合同与 Host framework pin 保持不变。

## 验证记录

三语言源码门禁通过：TypeScript 157 tests、Python 177 tests、Java tests 与三个 live Host
smoke/example 均通过，最终日志为 `/private/tmp/echo-sdk-language-final-verified.log`。
完整 Rust 门禁通过：fmt、all-target/all-feature Clippy、生产 panic/unwrap lint、workspace
all-target/all-feature tests 和 no-default-feature build。新增 mutation 后，最终 protocol
all-target tests 与 Clippy 再次通过，inventory suite 为 78 tests。
`check-sdk-contracts.sh` 的 28 个 rustdoc profiles、accepted artifacts、schema、fixtures
与 78 项 inventory tests 通过；`cargo audit --deny warnings` 和 `cargo deny` 依赖策略通过。
依赖策略保留既有 multiple-version warning 配置，未修改 lockfile 或审计策略。
完整 inventory 遥测重生成通过，95 个阻断制品一致、telemetry drift 为零。完整日志为
`/private/tmp/echo-sdk-final-verified.log`，最新测试日志为
`/private/tmp/echo-sdk-mutation-final.log`。本地门禁全部通过；远端 CI 和子仓最终 gitlink
由 superproject 的跨仓交付验收记录对账。

## 复审与修复

独立复审指出分类样例无条件要求非 accepted identity 存在；现只检查当前仍存在的样例。
全量测试发现 intrinsic identity 的源码位置可重复；唯一性键加入真实 identity path。
两个内存 mutation 测试通过：生产生成函数证明非 external identity path/rule 变化保持
committed source document 不变，而 accepted rule 变化会确定性产生不同摘要。
旧 cutover evidence 绑定历史提交，当前行为、Rule、Asset 与 Map 则绑定本次源码摘要。

## 已知缺口

不声称 npm/PyPI/Maven 发布完成，也不关闭 SDK 全能力治理范围。

## 历史证据锚点

本证据绑定已提交 SDK `30d7e219`；不支持本轮新增治理代码、目标 pin 或修改后合同的验证结论。原工作树 digest 不作为不可恢复的现有证据锚点。
