---
schema_version: 1
id: evidence.sdk-988-dependency-recheck
kind: evidence
observed_at: source:586e07f4762c1a26e3caaac90736b76784b55ede63ecf3a26626dbb78bd51ef9
source_refs:
- Cargo.lock
- deny.toml
- .cargo/audit.toml
- docs/sdk/governance/dependency-warning-policy-proposal.md
supports:
- rule.framework-runtime-authority
limitations:
- Deny emits 49 accepted duplicate warnings; exit0 does not mean zero warnings.
- This exact lock and scope recheck does not replace Rust, language, semantic or SDK-main acceptance.
---

# 公开988框架对应SDK锁的实际依赖复验

2026-10-16，SDK真实构建锁SHA256 `5952bfe30d027f5a5109eda5df3ed709cbadc16106c9b34713129b3e6cee138a`，八框架包固定公开 `9886502c83d6b1f6fa9212eb221db0e5874deaee`，无path patch。原deny配置SHA256 `b18307d3e0afa05d0c1ad32004b59cb0a7fe62136e90037bec17738545b3013f`及原四项advisory exceptions保持；本次未增加策略或例外。

实际 `cargo audit --deny warnings` exit0，日志253B/SHA256 `d0729c25c4f7b9dc65ffb89101ea9973fd77afb236622e2572cb36f8ae9d9c8d`；原framework988脚本直接读取SDK自己的锁，生成三vendor的registry-origin审计视图，视图SHA256 `62dc3eaa9193bd3fa218662e2eadc5498b8e19f6fa1c1678e5348d45dd52fa5a`，其官方audit实际exit0，日志327B/SHA256 `bf6d16ceaced47eccd7449fe666f2ce421f5017149aee8e710b49f643eeb24dc`。两锁均656包，逐字段比较仅calamine/docx-rs/rust_xlsxwriter的source/checksum转换，其它字段、依赖边与顶层保持。checksum与固定framework origins一致，不复制脚本、不借上游锁、不替换真实构建锁。开发工具用Python3.12运行，SDK runtime要求仍>=3.10。

实际 `cargo deny --workspace --all-features --locked check` exit0，完整日志700410B/SHA256 `51995772d05fcbc4aa34cef1b4b1c93d97c6c223abf941bf78ec03e0755bda3e`，无signal/截断。当前49条warning全部为duplicate，49组各精确版本集合与原接受的50组表完全一致；唯一减少zip2.4.2/8.6.0重复组，没有新增组/版本或其它warning/error。advisory数据库精确为 `479c888ce4401313bca61ba4aba2d0f6244c4a09`。

独立reader亲自重新解析完整输出和接受表、核runner终态/日志哈希、比两锁全部字段及固定origins，返回pass、行动项0；不是只采信主代理汇总，也未重跑Cargo/Host。原件、派生逐组清单与复审记录仅在ignored `.echo-semantic/local/sdk-parallel-delivery-01a14385/`。本薄证据只记录依赖范围，不宣称整个SDK或当前main完成。
