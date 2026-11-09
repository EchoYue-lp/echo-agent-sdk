# SDK 依赖重复版本警告规则裁决

## 2026-11-09 SDK 新锁的独立精确裁决

用户对列出精确 pin、锁和新增身份的本次问题直接回答：“同意，仅本次 SDK 精确绑定（推荐）”。原问题与回答保存在主根 `.echo-semantic/local/dependency-exact-binding-20261109-r01/authorization.json`，SHA256 `b940901e6c755ea39725be1ad184c6713aa6c9c388f9c29e8d1b4d2ffd305733`。

本次仅接受 Framework main `3413d64dae23353c6d02c546ac374deded855827` 和 SDK 锁 `95e3caabaf5e72fc71ad1d6b79964eae1883b297f0006667e39c4c3a6eb8c5e5`（654 包）：原49组107个 name/version/source 身份全部保留，仅 quick-xml 组新增 `0.42.0`／`registry+https://github.com/rust-lang/crates.io-index`，现49组108身份。唯一机器权威为本仓 `.cargo/cargo-deny-duplicate-baseline.json`；cargo-deny0.20.2、原四策略命令、严格parser及原四项 audit ignore不变，不新增 allow/ignore。

批准前 actual／canonical 默认 fresh audit均自然0、0漏洞及0额外warning（原4ignore保持），原四策略 deny scanner自然0；旧正式基线因锁变化拒绝的gate1与候选同raw校验0分别保留。批准后须执行原14控制并对完整原raw／receipt正式严格复核，不把候选结果改写成旧正式成功。本次不接受未来 pin、锁、身份或风险，不继承 Framework／CLI 裁决，不豁免完整 SDK、Host、三语、平台或精确 main CI。下方旧锁、旧来源和失败仍按原切点解释。

这是独立SDK有界duplicate提示接受：原107身份加1身份，不称所有依赖风险已修复或零warning deny。实际图固定13个同源Git包、同精确3413 checkout；新版calamine0.38关闭原canonical CFB漏洞，本文不将该修复外推到其它风险。公共SDK API与示例没有因本基线字段维护而新增；examples和官网不需因此另改，完整代码与示例验证仍按原SDK矩阵执行。

Status: accepted

正式采用后的原14项控制已实际自然0；原未改严格parser对完整原scanner0的raw／receipt执行正式复核自然0，`authorization=ACCEPTED_BY_HUMAN`、`matches_exact_baseline=true`、49组108身份。原raw为695282B／SHA256 `14a615b61b4518ea4b84315bd6be2e1823f2e82e527bc5d091f8145048575616`，未重跑scanner，旧正式gate1保留。采用后执行位于主根 `.echo-semantic/local/dependency-exact-binding-20261109-r01/formal-sdk-r01/execution.json`，SHA256 `5b9ad60aeda348f539f4144400e4b11b6cb66b20eae2a77021ec8342d67c1576`。本次actual／canonical fresh audit各自然0、原4ignore下0漏洞与0额外warning的原件为主根 `.echo-semantic/local/integrated-acceptance-20261108-r01/sdk-main-validation-r01/dependency-candidate-3413-r03/execution.json`（SHA256 `8fba40c1516e36ebafdde6e1ebd0fa6f8b17b57b9918bb5d25758bbb9aeb0e89`）。复核只签依赖输入范围，不冒完整SDK工作源无变化、Host／三语或精确main CI完成。

## 2026-10-31：SDK HTTPS340 组合锁的精确绑定复核

本段提案只更新机器基线的锁摘要为c448、Framework pin为340及本轮验证来源；原7f0a／0dac／f7字节与旧verify自然1完整保留。49组107条registry identity数组、原人类有界duplicate接受、tool0.20.2、原normalized命令及policy／audit／allow／ignore全部原样，不继承Framework或CLI例外。

原官方audit0.22.2实际0，fresh RustSec DB为2295466c／1395条；原官方deny0.20.2四检查实际0，完整raw698424B／SHA721a86e0，严格原解析器只见原49组107身份、零新/少/变组与其它诊断。原policy14实际0；本提案不重扫、不称候选接受gate已通过。

0dac→c448不能称source-only：653包中637个完整块相同、13个共同块有Source／checksum／dependency变化，另有zstd0.13.3→0.14.3、zstd-safe7.2.4→8.2.0、zstd-sys2.0.16→2.1.2替换。13个来源变化包括11个Git pin与2个Polars registry→同Git源（Git锁块不保registry checksum）；echo_tools新增httpdate边、echo_integration新增matchit边。registry URL未增加。四项原ignore所涉包版本/来源保持，旧DB84d943a6与fresh2295466c的同ID package／versions范围也相同。仅FileGit478→HTTPS c448那一步才是13SourceID替换。

是否在原49／107风险例外不扩张的已批准SDK pin切换范围内作准确绑定，由Root据完整差异和独审裁定。三registry版本替换与两条边均是具体复核输入，不以duplicate组相同抹去，也不凭这几处差异预设必须重新问人类。任何新增/减少/变更duplicate身份、registry来源、漏洞或其它诊断继续另列真实新取舍，不能自动扩大例外。

候选仅在C外供审；后继精校准复用同一原raw／实际receipt调用原未改verifier与policy14，不使用capture重扫。完整SDK新版本工程、正常source/final和精mainCI仍各自验收，扫描器0不作运行绿。

## 2026-10-23 Tokio I/O safety 定向补丁

当前锁 SHA256 为
`822ee9a6936f25978283d38d4d07e4e8d7e006ec5834bfeaf80542f79e1ed52e`。
SDK main `3daaa573a60d83e7064ff0796c48bc67ddd6297b` 的原依赖CI
[39978621886](https://github.com/EchoYue-lp/echo-agent-sdk/actions/runs/39978621886)
在 fresh 1351-advisory 数据库上发现
[RUSTSEC-2026-0386](https://rustsec.org/advisories/RUSTSEC-2026-0386.html)：
Tokio 1.53.1 的 AsyncFd 构造不满足 I/O safety。定向升级到官方已修补的1.53.3；
crate SHA256 `ce3335fa71841cda333a58d7615b03901380ecf09d59b3296d21f8bbac0dde4e`
与 crates.io 官方版本元数据和实际下载原包相同，not yanked。

656包中只改 Tokio version/checksum，655其它包的完整锁块、Tokio依赖边、
framework `f7bac62eee04a963f92a3e4308b6ea2336e91cc0` pin与manifests保持。
Cargo最初定向update产生的无关socket2/windows-sys边回写已保原日志并排除；
恢复原边后full `cargo metadata --locked --format-version 1` 实际0，不是no-deps证明。
初次offline metadata只因新Tokio crate未缓存而101，在线官方包下载后的验证保持原锁。

fresh `cargo audit --deny warnings --json` 实际0、0vulnerabilities、warnings为空，
数据库1351条/HEAD `84d943a6deeb8cc6c6f632531aaf4f1c62275239`。
`cargo-deny0.20.2` 原workspace/all-features/locked四检查scanner自然0；
原49组107项duplicate的精确name/version/source集合完全相同，无新增诊断类别。
机器基线只更新lock摘要与此观察说明，policy/command/tool/groups及原human授权不变；
复用原raw的精确gate实际0，14项gate单测实际0。duplicate提示仍完整保留，不称零warning审计。

两次原HTTPS RustSec fetch网络失败均保留为失败；后继仅进程级把官方同repo传输改为SSH，
默认fresh fetch实际成功，不使用no-fetch、不改持久Git配置或风险例外。
新Tokio运行门禁、patched Host完整语言与精mainCI分别验收；本次鲜扫描不替代这些边界，
旧Host和旧语言运行不重标为新Tokio验证。以下旧锁/扫描只保留其原epoch。

## 新pin来源与机器门禁（历史A03：当时SDK扫描与运行时仍待验证）

当前framework pin为精main `f7bac62eee04a963f92a3e4308b6ea2336e91cc0`，SDK最小锁SHA256
`1c865a57b7e17587fba2f668d956832cd7b0f6c6167e198304c965227f3debaa`。相对既有webpki补丁锁5b5d，仅8framework+3vendor Git来源变，
全部656包的版本、checksum及依赖边保持。A03使用冻结的既有用户镜像缓存，官方Cargo离线update与locked metadata均实际退出0，精锁为上述1c865。
metadata仅no-deps（resolve=null），不证明完整图、fresh registry、冷缓存或yanked状态；fresh SDK扫描、新Host及完整门禁仍待验证。
`.cargo/cargo-deny-duplicate-baseline.json`机械绑定该精确锁，保留原SDK49组/107项
精确version/source身份；原历史50组中zip消失的复验和原始日志归属保留。
复用正式framework校验器及真实彩色parent→plain child反例，锁定cargo-deny0.20.2，
保持严格原raw语法、四类检查与scanner自然退出，child只设置CARGO_TERM_COLOR=never；
不清洗原raw、不接收ANSI、不新增offlineflag、不扩来源/类别或忽略规则。
CI记录原scanner raw与自然退出，再比较精集合；任何新组/版本/来源或未知渲染均拒绝。
原cargo audit --deny warnings保留；新SDK原三policy、14Rust/Host/完整语言/平台验收另行执行。
本段不是当前新锁扫描结果，也不把原e736失败、诊断失败或历史UNKNOWN改为通过。

Human decision: 2026-10-14，用户明确答复“接受有界 duplicate 提示例外（推荐）”。

Reviewed proposal SHA-256: `4c94073065a063d72cc863d658787961bad538250296c56990a5e86b722a83c7`。

用户已选择下述有界 duplicate 提示例外；SDK AGENTS.md 与顶层 SDK 归属说明已由主线程同步落实。deny.toml、audit 配置、依赖和锁文件保持原样。接受只覆盖本表50组及其精确版本集合，不能把工具 exit0 写成输出零警告，也不自动满足其它交付门禁。下列原规则、候选句及严格零警告方案保留为裁决时的历史展示，不是当前再次提问。

当前锁复验（2026-10-20，SDK208a）：真实锁SHA256
`58599a9f95c7b524fe63c97a560d2bac235539570b2ed9ed6165586c793444a3`，
framework精main356。原三项fresh官方扫描actual0，数据库精HEAD999ec89d/1338advisories；
deny实际49组与原接受50组精确版本表相比仅zip消失，无新增组/版本/其它类别。
原锁、deny策略及例外不改，原抓取失败与进程内同官方repo传输恢复保留；独立复审C0。
本次实际来源、原件摘要和边界见[当前三链验证后继](../../../.echo-semantic/evidence/evidence.sdk-model-read-retained-verification.md#2026-10-20同208a源码的完整工程后验)。
这是同一原政策的复验，不把49条duplicate称零warning，也不向CLI或其它警告类别扩例外。

历史锁复验（2026-10-16）：SDK固定公开framework988，真实锁SHA256
`5952bfe30d027f5a5109eda5df3ed709cbadc16106c9b34713129b3e6cee138a`。
fresh audit及三vendor来源视图audit均exit0/无warning；deny exit0但保49条duplicate，
49组精确版本集合与下表完全一致，只有zip重复组消失，无新增组/版本或其它诊断。
原deny/audit配置及四项例外未变，独立原输出/逐字段复核pass，见
[本次薄证据](../../../.echo-semantic/evidence/evidence.sdk-988-dependency-recheck.md)。
这次复验适用原接受政策，不扩基线，不把deny称为零警告；下文50组及旧锁仍保原裁决时点。

## 裁决前已确认的冲突（历史）

裁决前 SDK AGENTS.md 第72–73行的原要求为：

> 开发提交执行 focused 验证；合并前执行全部适用 Rust、contract、Host E2E、三语言、cross-platform、dependency 和文档门禁，所有命令必须零失败、零警告、零格式差异。

但同仓 deny.toml 的既有 `[bans] multiple-versions = "warn"` 会在正常受支持的依赖图上产生重复版本提示。真实 `cargo deny --workspace --all-features --locked check` 返回0，报告50条 `warning[duplicate]`；配置、advisory、license、source 其它警告均0。真实 `cargo audit --deny warnings` 返回0、无警告。因而“两个工具退出0”与“所有门禁零警告”是不同结论，该规则冲突当时不能由助手自行解释豁免；用户现已通过明确裁决闭合这个有界政策选择，实际提示仍须完整记录。

证据来自 [当前依赖门禁收据](https://github.com/EchoYue-lp/echo-agent-sdk/blob/0ebe15f66d0aa57c7b7a92ae68cd1be106114ae9/docs/sdk/governance/evidence/sdk-current-dependency-policy/README.md)、[完整 deny 输出](https://github.com/EchoYue-lp/echo-agent-sdk/blob/0ebe15f66d0aa57c7b7a92ae68cd1be106114ae9/docs/sdk/governance/evidence/sdk-current-dependency-policy/deny.log)、[原 policy 结果](https://github.com/EchoYue-lp/echo-agent-sdk/blob/0ebe15f66d0aa57c7b7a92ae68cd1be106114ae9/docs/sdk/governance/evidence/sdk-current-dependency-policy/policy-result.json) 和两个工具的独立 run-command 终态。原四项 advisory exceptions 原样保留，不能把本提案理解为新增漏洞、维护状态、来源或许可证豁免。

## 实际50组及版本分类

绑定的 Cargo.lock SHA-256：`a42cfbc993a3eabb714a6b221472600d424fa411226659458b7286d490f7e837`。

绑定的 deny.toml SHA-256：`b18307d3e0afa05d0c1ad32004b59cb0a7fe62136e90037bec17738545b3013f`；原 deny.log SHA-256：`14aeb79543361a51ee5b2b0c298d2fbc150c068275597eeb831d9382238476cd`。

下面每组版本逐条取自此次 deny 输出的实际 lock entries，并核对都存在于 Cargo.lock。不能仅列锁文件全版本：例如 schemars 的锁文件还包含0.9.0，但本次 all-features workspace 检查选择的重复组只有0.8.22与1.2.2，未把未选版本加入基线。

50组中10组跨数字 major，40组数字 major 都是0但跨0.x minor兼容线，没有同一兼容线内的纯补丁重复组。这里“同 major”仅是版本号分类，不表示可以安全互换；0.x minor变化通常属于不同兼容线，具体升级还需按原消费者约束验证。

| crate                   | 本次检查实际版本                       | 分类                            |
| ----------------------- | -------------------------------------- | ------------------------------- |
| block-buffer            | 0.10.4, 0.12.1                         | 同 major（跨 0.x minor 兼容线） |
| const-oid               | 0.9.6, 0.10.2                          | 同 major（跨 0.x minor 兼容线） |
| core-foundation         | 0.9.4, 0.10.1                          | 同 major（跨 0.x minor 兼容线） |
| cpufeatures             | 0.2.17, 0.3.0                          | 同 major（跨 0.x minor 兼容线） |
| crypto-common           | 0.1.7, 0.2.2                           | 同 major（跨 0.x minor 兼容线） |
| digest                  | 0.10.7, 0.11.3                         | 同 major（跨 0.x minor 兼容线） |
| foldhash                | 0.1.5, 0.2.0                           | 同 major（跨 0.x minor 兼容线） |
| getrandom               | 0.2.17, 0.3.4, 0.4.3                   | 同 major（跨 0.x minor 兼容线） |
| hashbrown               | 0.12.3, 0.14.5, 0.15.5, 0.16.1, 0.17.1 | 同 major（跨 0.x minor 兼容线） |
| hashlink                | 0.9.1, 0.10.0                          | 同 major（跨 0.x minor 兼容线） |
| html5ever               | 0.31.0, 0.39.0                         | 同 major（跨 0.x minor 兼容线） |
| indexmap                | 1.9.3, 2.14.0                          | 跨 major                        |
| markup5ever             | 0.16.2, 0.39.0                         | 同 major（跨 0.x minor 兼容线） |
| md-5                    | 0.10.6, 0.11.0                         | 同 major（跨 0.x minor 兼容线） |
| nom                     | 7.1.3, 8.0.0                           | 跨 major                        |
| phf                     | 0.11.3, 0.12.1, 0.13.1                 | 同 major（跨 0.x minor 兼容线） |
| phf_codegen             | 0.11.3, 0.13.1                         | 同 major（跨 0.x minor 兼容线） |
| phf_generator           | 0.11.3, 0.13.1                         | 同 major（跨 0.x minor 兼容线） |
| phf_shared              | 0.11.3, 0.12.1, 0.13.1                 | 同 major（跨 0.x minor 兼容线） |
| quick-xml               | 0.39.4, 0.41.0                         | 同 major（跨 0.x minor 兼容线） |
| r-efi                   | 5.3.0, 6.0.0                           | 跨 major                        |
| rand                    | 0.8.7, 0.9.5, 0.10.2                   | 同 major（跨 0.x minor 兼容线） |
| rand_chacha             | 0.3.1, 0.9.0                           | 同 major（跨 0.x minor 兼容线） |
| rand_core               | 0.6.4, 0.9.5, 0.10.1                   | 同 major（跨 0.x minor 兼容线） |
| redox_syscall           | 0.5.18, 0.7.5                          | 同 major（跨 0.x minor 兼容线） |
| schemars                | 0.8.22, 1.2.2                          | 跨 major                        |
| schemars_derive         | 0.8.22, 1.2.2                          | 跨 major                        |
| serde_derive_internals  | 0.29.1, 0.30.0                         | 同 major（跨 0.x minor 兼容线） |
| sha2                    | 0.10.9, 0.11.0                         | 同 major（跨 0.x minor 兼容线） |
| shlex                   | 1.3.0, 2.0.1                           | 跨 major                        |
| socket2                 | 0.5.10, 0.6.5                          | 同 major（跨 0.x minor 兼容线） |
| string_cache            | 0.8.9, 0.9.0                           | 同 major（跨 0.x minor 兼容线） |
| string_cache_codegen    | 0.5.4, 0.6.1                           | 同 major（跨 0.x minor 兼容线） |
| syn                     | 2.0.119, 3.0.3                         | 跨 major                        |
| tendril                 | 0.4.3, 0.5.1                           | 同 major（跨 0.x minor 兼容线） |
| thiserror               | 1.0.69, 2.0.20                         | 跨 major                        |
| thiserror-impl          | 1.0.69, 2.0.20                         | 跨 major                        |
| tower                   | 0.4.13, 0.5.3                          | 同 major（跨 0.x minor 兼容线） |
| web_atoms               | 0.1.3, 0.2.6                           | 同 major（跨 0.x minor 兼容线） |
| weezl                   | 0.1.12, 0.2.1                          | 同 major（跨 0.x minor 兼容线） |
| windows-sys             | 0.48.0, 0.52.0, 0.61.2                 | 同 major（跨 0.x minor 兼容线） |
| windows-targets         | 0.48.5, 0.52.6                         | 同 major（跨 0.x minor 兼容线） |
| windows_aarch64_gnullvm | 0.48.5, 0.52.6                         | 同 major（跨 0.x minor 兼容线） |
| windows_aarch64_msvc    | 0.48.5, 0.52.6                         | 同 major（跨 0.x minor 兼容线） |
| windows_i686_gnu        | 0.48.5, 0.52.6                         | 同 major（跨 0.x minor 兼容线） |
| windows_i686_msvc       | 0.48.5, 0.52.6                         | 同 major（跨 0.x minor 兼容线） |
| windows_x86_64_gnu      | 0.48.5, 0.52.6                         | 同 major（跨 0.x minor 兼容线） |
| windows_x86_64_gnullvm  | 0.48.5, 0.52.6                         | 同 major（跨 0.x minor 兼容线） |
| windows_x86_64_msvc     | 0.48.5, 0.52.6                         | 同 major（跨 0.x minor 兼容线） |
| zip                     | 2.4.2, 8.6.0                           | 跨 major                        |

## 已接受选择：仅允许明确受控的 cargo-deny duplicate 提示

保留现有 `multiple-versions = "warn"` 工具策略，明确只有此工具的 `warning[duplicate]` 可以作为受控、非阻断提示。所有原命令仍必须真实退出0；audit/advisory、license、source、配置、Rust/TypeScript编译及其它命令的警告仍必须为0，测试仍零失败、零跳过豁免、零格式差异。本选择不新增 ignore，不隐藏输出，不降低任何非 duplicate 阈值。

这不是对未来所有 duplicate 的无限许可。首次接受仅绑定本表50组、精确版本集合、上述锁文件摘要和完整原始输出；未来锁图、framework revision、检查 feature 范围或策略改变，必须重新运行原命令并复审版本组变化。即便警告条数仍为50，版本或组身份变化也不能自动沿用；数量增加也不得自动更新上限或基线。新 category、新来源/许可/advisory/配置警告须按原要求修复。原始日志、分类、退出码与复审结果必须一并归档，不能用汇总“pass”替代50条实际提示。

裁决时展示、现已由主线程落实到 SDK AGENTS.md 的**规则替换文本（保留原候选展示）**为：

> 开发提交执行 focused 验证；合并前执行全部适用 Rust、contract、Host E2E、三语言、cross-platform、dependency 和文档门禁，所有命令必须真实退出0、零失败、零格式差异。编译、audit、advisory、license、source、配置及其它工具警告必须为0；唯一例外是 cargo-deny 原有 multiple-versions = "warn" 产生的 warning[duplicate]，仅在完整原始输出保留、版本组按已接受的精确 Cargo.lock 摘要基线复核且无未经复审的增量时，作为非阻断提示。锁图、framework revision、检查范围或策略变化必须重新验证与复审，不得自动扩大基线或推广为其它警告豁免。

当前 SDK AGENTS.md 已采用该规则，并链接本裁决的50组精确版本基线。不改其余“失败必须修复”、分支、提交、合并规则。

## 未选择的历史方案：维持严格零 duplicate 警告

这是当时展示但用户未选择的方案：保留 SDK AGENTS.md 原句，并在合并前消除50组 duplicate。现不按此路线启动依赖升级，保留以下取舍依据。需要沿原父依赖约束逐组寻找兼容整合路径，再验证框架精确revision、SDK构建/feature矩阵和语言行为。表中10组跨major及40组跨0.x minor不宜以一次cargo update或删除功能直接合并，不能伪造结果、追加ignore或把warn输出关掉来满足零告警。没有在本提案中推导升级工期，也未断言任何具体组永远无法整合。

此路线可能需要上游依赖升级及适配，但不允许把全部 Rust public identity 强行扩大成三语言 API，也不改变已接受的产品合同边界。若需要改变用户承诺或新增依赖策略，必须另行形成具体可审阅选择。

## 顶层与子仓库规范归属

已读取实际 `/Users/ls/MyWork/code/ylp_agent_learn/lp-agent/AGENTS.md`。顶层第249行要求所有已执行命令零失败/零报错，第254行针对编译警告全部消除；没有与SDK第73行完全相同的“所有dependency工具提示零警告”句子。顶层要求与子仓库规则同步；主线程已在顶层补充SDK专属归属说明。裁决时展示的**候选新增句（历史）**为：

> echo-agent-sdk 的依赖检查只按其独立 AGENTS.md 中经用户接受、绑定精确 Cargo.lock 版本组基线的 cargo-deny warning[duplicate] 特例执行；该特例不适用于 echo-agent、echo-agent-cli、编译警告或其它工具类别，原失败必须修复及提交/分支门禁保持不变。

顶层该 SDK 专属归属句已由主线程落实；顶层外国已有差异仍保留，本轮并未因文稿更新提交它们。正式SDK规则与依赖证据归独立SDK；SDK AGENTS.md 和顶层这条协作说明按各自仓库规则同步交付。framework/CLI/website 不据此改变其依赖政策或发布边界。

## 裁决与实施边界

用户已选择有界提示例外，严格零警告方案仅保留为历史比较。原20文件依赖证据包及其摘要未修改；接受本政策不会自动关闭Finding或替代源码、测试、合同和仓库门禁。实施仅限所选规则文本、必要规范同步及原命令验证，不额外改锁图、生成合同、ignore或bans阈值。未来新framework revision、锁图、manifest/检查范围变化要重新取得fresh audit/deny结果，并独立复审组身份与版本集合；任何增量不在此次首次50组授权内。框架新revision及SDK最终main验收仍需各自精确快照证据，npm/PyPI/Maven发布继续在本项目交付范围之外。

## Plan04 lock-graph contraction

Removing the SDK A2A/LSP feature requests on the unchanged f7bac62e pin prunes
only jsonwebtoken9.3.1, pem3.0.6 and simple_asn10.6.4, plus the echo_agent
axum/jsonwebtoken edges. No package is added or version/checksum changed. The
49 accepted duplicate identity groups, scanner version, command, authorization
and deny policy remain byte-for-byte equivalent; only the machine baseline
lock digest is rebound to the actual smaller lock. Fresh scanner output must
still match those exact49 groups with no other warnings/errors. This does not
expand the existing exception or reclassify older failed evidence.

The Plan04 lock SHA256 is
`0dacbb873673a405fed4e820499ff33e40301cefc5905e8c2bc597558156231f`.
Fresh `cargo audit --deny warnings` loaded1352 advisories and exited0.
The original workspace/all-features/locked cargo-deny0.20.2 scanner and exact
baseline gate exited0; all49 accepted duplicate groups remain unchanged, with
no other diagnostic category. Raw output and its natural exit are retained in
the task-local receipt; this scan does not substitute for Host/language tests.
