# SDK 当前锁图依赖门禁

实际原CI命令cargo audit --deny warnings与cargo deny --workspace --all-features --locked check均以官方run-command执行，退出0、无截断。cargo-audit0.22.2 / cargo-deny0.20.2；只解析锁图和数据库，没有Rust编译，也未改源码、锁、frameworkpin、例外或策略阈值。

Audit实际fetch官方https://github.com/RustSec/advisory-db.git，载入1310条advisory，扫描658个locked依赖；数据库HEAD6aed84668c23d7f535077a9fe918bf1fa0f6a5bc与独立git ls-remote官方HEAD一致。Audit cache与deny独立cache分别clean并都匹配这一freshrevision，不能把之前framework历史1310/1329资料当当前fetch。rustsec-database-manifest固定1310markdown摘要，deny-database-provenance记录自己的缓存路径/revision/count。数据库不是新风险裁决来源，既有配置例外本轮保持。

Audit --deny warnings返回0。Deny advisories/bans/licenses/sources全部ok；原bans.multiple-versions=warn策略报告50条warning[duplicate]，配置/advisory/license/source warning0。本包不宣称deny输出零WARN，也未为治理零警告增加ignore或删除依赖，原完整702116字节输出保留。四个原advisory exceptions是RUSTSEC-2026-0194、0195、RUSTSEC-2023-0071、RUSTSEC-2025-0141，拥有者和复核日期保持原输入内容，本轮没有新增或重新授权风险接受。

source-before/after固定7个输入摘要，全部一致；Cargo.lock中8个framework包均为真实Gitb9099558620f5f9693c736c85c214db14f62a8cb，无localpatch。源码本轮其它runtime变化不影响这个锁图结论；后续锁或frameworkpin改变要重新验证，不能把本包绑定到未来revision。完整accepted生成/main验收仍由主线程闭合。本包不涉及npm/PyPI/Maven发布或外部Git写入。
