---
schema_version: 1
id: evidence.sdk-source-continuity
kind: evidence
observed_at: b80cf068b2fb69b62913f23260980b4ce2ebf941
source_refs: [MIGRATION-SOURCE.md, docs/adr/0001-sdk-repository-boundary.md, docs/adr/0028-source-first-multilanguage-sdk-runtime.md, docs/adr/0031-sdk-identity-governance-scope.md, docs/adr/0032-sdk-contract-scope-classification.md, contracts/sdk/source-contract.json, echo-sdk-protocol/src/facade.rs, echo-sdk-host/tests/extension_bridge_e2e.rs, sdks/shared/contract-digests.json]
supports: [behavior.sdk-repository-ownership, rule.framework-runtime-authority, asset.sdk-source-product]
limitations: ["本证据覆盖 source continuity 与文档 ownership，不证明 protocol 已纯化、accepted contract 已分离或完整独立 SDK 门禁已通过。"]
---

# SDK source continuity evidence

## 支持的结论

过滤后的 framework SDK history tip 是 `28d6709c93a7809c1eb3ba744d3e4cc89b0332f7`，
对应原 framework 冻结 revision `c5f7688212d45d5bdcdbf60342605e8bfb176cae` 的
SDK-owned 路径。双父 merge `b80cf068b2fb69b62913f23260980b4ce2ebf941` 保留该父线与
`ea21dfc` source-import 父线；修复基线 `7beee9d416ac304c3cec6df6646f558496ab577d`
进一步把 SDK main 与 clean-pin lineage 合入同一可追溯历史。

源 revision 的 SDK-owned tree 与当前仓均有 602 个文件。Contracts、三语言、scripts、
Host/Protocol（排除各自 Cargo manifest）逐目录一致；初始 import 到冻结 tree 的实际工作树
差异为 13 个文件。两份 Cargo manifest 保留 Git dependency 适配，ACP adapter 文档将相邻
framework example 改为 `c5f7688` 的不可变外链。

## 来源与范围

来源包括 Git filtered history、`c5f7688` archive、逐目录对账、迁移说明、三份 SDK ADR、
source-first design 和 SDK 文档链接检查。

## 验证结果

过滤历史检查确认 31 个 commit、618 个文件和零越界路径；SDK tree 对账确认源/目标均为
602 个文件，排除三处已记录适配后零缺失、零额外、零 blob/mode mismatch。40 个 Markdown
本地链接、Cargo metadata、Rustfmt、全部迁入 JSON、三份 shell 脚本语法和 Java POM XML
检查均为零退出。

## 已知缺口

最终 MR merge commit 与远端 main ancestry 仍需在本分支交付后由 Git 对账证明。Protocol purity、
framework pin、合同分离、Host E2E 与三语言门禁由后续 Evidence 分别证明。

## 2026-10-21 f7bac source-only 后继（运行时尚待验证）

本段仅追加当前来源事实，保留上述历史 observed_at、source_refs、正文与 limitations。
SDK 基准为 `09f670fbbde98bc44a7c710c6419a6ea0d7d469c`，本次由同一任务正式承接
20 个工程文件；协调授权为 `SDK-F7BAC-FORMAL-SOURCE-20261021-01`。
framework 精 main 为 `f7bac62eee04a963f92a3e4308b6ea2336e91cc0`，tree 为
`0acc8d94603b647e4d795c0b2eea20ecb0c3da7d`，sole parent 为
`6849bfcab74d9a55d4f70987adcd13835bb9789d`。协调者已接受 PR197 原 main CI
39605663648 七项 SUCCESS 与 remote-only cold 源码复核；上游验收不证明 SDK Host 通过。

`echo-sdk-host/Cargo.toml` 的唯一 framework pin、`Cargo.lock` 与
`scripts/check-sdk-contracts.sh` 的来源断言同步到该 revision。锁文件 SHA256 为
`1c865a57b7e17587fba2f668d956832cd7b0f6c6167e198304c965227f3debaa`；相对既有
webpki 补丁锁，仅 8 个 framework 包及 3 个上游 vendor 包共 11 条 Git source 变化，
全部 656 包的版本、checksum、依赖边保持。复用上游既有校验器与 14 个纯 Python 测试：
`scripts/check-cargo-deny-baseline.py`、`scripts/tests/test_cargo_deny_baseline.py`，
`.cargo/cargo-deny-duplicate-baseline.json` 仍严格保留原 SDK 已接受的 49 组、107 项
registry 身份；authorization、比较逻辑、命令及警告类别边界不扩。子进程仅固定
CARGO_TERM_COLOR=never，不清洗 raw、不放宽 ANSI/未知渲染、不新增 offline scanner 选项。
`.github/workflows/rust-ci.yml` 接入相同检查，
`docs/sdk/governance/dependency-warning-policy-proposal.md`、`README.md`、
`MIGRATION-SOURCE.md`、`AGENTS.md` 与 `docs/sdk/README.md` 如实同步来源及剩余验收。

A03 在冻结的既有用户镜像缓存及独立 Cargo home 中实际执行一次
`cargo update --offline -p echo_agent --precise f7bac62eee04a963f92a3e4308b6ea2336e91cc0`
和一次 `cargo metadata --locked --offline --no-deps --format-version 1`，均自然退出 0，
产出的完整锁文件逐字匹配上述 SHA。metadata 的 resolve=null，仅为 no-deps 来源观察；
这不是完整依赖图、fresh registry、冷缓存或 yanked 状态验证。A01 的 Cargo 自然退出
UNKNOWN、A02 经授权 SIGINT 的失败原件均保留，不以 A03 成功改写。A03 的来源、缓存
前后身份及实际退出原件仅在当前工作区
`.echo-semantic/local/sdk-parallel-delivery-01a14385/sdk-f7bac-source-candidate-v1/resolver-a03-execution/`
可用；具名 minproof 为 `source-only-final-minproof.json`，9223 bytes，SHA256
`50922abb5e8d499e211ecde403701490e3378c09f77028d4388e29e428a6e230`。

提交闭包保留原有 8 个测试/fixture 的 e736 字节与模式：
`echo-sdk-host/tests/extension_bridge_e2e.rs`，`sdks/shared/testing/a2a_host_fixture.py`，
`sdks/typescript/test/a2a-host.test.js`，`sdks/python/tests/test_a2a_host.py`，
`sdks/java/src/test/java/com/echoagent/sdk/A2AHostTest.java`，
`sdks/typescript/test/family-route-host.test.js`，`sdks/python/tests/test_family_route_host.py`，
`sdks/java/src/test/java/com/echoagent/sdk/FamilyRouteHostTest.java`。
FamilyRoute 三项是当前 Windows CI 调用的直接依赖；Store/A2A 另外五项是同一目标已经接受
的补证，不冒称为 Windows 的直接依赖。CI/锁与新增采用 12 项重叠 2 项，因此总数为 20。
framework 保留唯一执行/取消/settlement 权威，SDK 不新增 driver、状态机或公共接口。

固定 canonical 校验器 SHA256 `950a606e3b8225c802d61f1b0a6ce4b71ed0949d7b12ad73aec1cf76dbfd9c66`
在真实 HEAD09f、可恢复 base 对象、精确 stage 新 3 个文件的正常 Git view 中，官方
source_digest 实算为 `74885331db1137991513d5bfe0c831d462b395d21c7089e863de743762aaf7fb`。
该 view 的全部 1225 个 tracked 文件与 S 字节/模式相同；摘要排除 360 个语义文件，
实际绑定 865 个源码/支持文件。S 另有原 4 个受保护、未跟踪 pyc，未删除、移动、忽略或暂存；
S 全输入 1229 文件域（含 pyc）的官方源码摘要为
`0010846b7afc5e655b75c882daf529b029049d95358d674c99871242c8e489b4`，对应 869 个非语义路径。
Baseline 仅绑定可提交 1225 域；两个域及 pyc 身份原件位于本任务 local 的
`sdk-f7bac-formal-source-20261021-01/normal-gitview-source-domains.json`。
该普通 Git view 不是冷恢复或原生 Stop 通过证明，S 的 PF 和受保护输入仍独立核对。

原 e736 未过滤第 12 项失败 101、7a81 诊断失败 101 均未在新 pin 上重验；新的 Host、
原 14 Rust、fresh 三项依赖扫描、完整三语言及三个 quickstart、Windows/main CI 仍待实际
执行。历史 managed 首因仍 UNKNOWN，4 个整体行为 hold 与 behavior_model_closure=open
保持；accepted 5621 identity/kind、454 routes 不扩，不自动生成 117 产物，不提升旧 Audit
或 Finding 状态。示例继续由原三个 quickstart 验证；本次没有公开 API 或官网产品承诺变化，
不新增官网内容。本段来源事实不替代后续工程验收及自然宿主 Stop。

## 2026-10-22 f7bac 阶段资格（其余门禁继续待验）

本节是上述 source-only 切点之后的新执行事实，不改写历史 header、失败或旧“待验”条目。
实际执行仍在 SDK HEAD `09f670fbbde98bc44a7c710c6419a6ea0d7d469c` 的精确暂存 22 路径、
1225 tracked 文件上，committable 源码摘要仍为
`74885331db1137991513d5bfe0c831d462b395d21c7089e863de743762aaf7fb`，
framework pin、11 Git 来源、锁 SHA1c865、accepted 5621/454 与原始 4 个 pyc 均保持。

本阶段原 14 条 Rust 中仅 01/02/03/04/05/12 已实际合格：同源 fmt/fmt-check 为 0；
03 为原 workspace/all-targets/all-features Clippy，04 为原 lib/bins 严格 panic-family Clippy，
均自然退出 0、零警告。05 原 `cargo test --workspace --all-targets --all-features --locked`
实际 13 个 harness（含 main 零用例 harness）、523 passed、0 failed/ignored/filtered：
Host lib223、core33、extension39、facade12、stdio4、governance1、Protocol lib42、
export9、ACP5、coreRPC22、extension contract42、inventory91。两个 ordinary Store
提交后 unregister/disconnect 具名用例均通过。原未过滤 12 bridge-profile 整 target 另为
31 passed、0 failed/ignored/filtered；它与 05 的语义覆盖有重叠，不相加为独立能力数。
managed 断连重启、lost-ack 与背压用例在当前 f7 切点通过，仅证明当前有限回归；
不能据此认定历史 0ebe UNKNOWN 与 e736 原失败同因或关闭历史 Finding。

实际 Cargo、Node、guard、caller 均自然 0，signal/error=null、原 raw 完整不截断，
1225 文件字节/模式、暂存索引、PF、4 个 pyc、旧 19 原件与实际 framework checkout2332
前后相同，remaining producer=[]。05 观察到的 own target 峰值为 4263927808 bytes，
最低双探针可用空间为 11845799936 bytes，没有触发资源边界；未改变已接受的
opt1/assertions/overflow/unwind/jobs1/inc0/debug0/strip 编译配置。
原12两件实际 producer 已独立保存，后续全语言入口仍会自行 build，不能假定它使用该副本。

本地原件仅在当前工作区：
`.echo-semantic/local/sdk-parallel-delivery-01a14385/sdk-f7bac-accepted-runtime/`。
05 raw `f7bac-rust-05-v2-a01.log` 为 72222 bytes、SHA256
`5dd3d4c46cf34c96135e640d5f3aa15e24a63c6bd72883ca758d4339b38c5b75`；
具名 milestone minproof 为 6172 bytes、SHA256
`149517f3aed9be69fad478932723f3c129e0e87416fa9bf6fa1f1724fd35d8d4`，
并链接各真实 Node/guard/caller/source-after 原件。04 a01 的外层日志命名冲突在启动 Cargo
之前被防覆盖检查拒绝，负证据保留；协调者授权修正输出路径后的 a02 实际合格，
不是重跑失败的 Rust case 或改变测试/预算。

本次两个 JS test 的 `node --check` 和三个 Python test/fixture 的 AST/compile-only
已在与 S 全部 1225 文件逐字节/模式相同的冻结副本中自然退出 0。
实际 Node22.22.0、Python3.12.4；未 import 待检模块、未写 pyc、未执行 test case/Host。
该语法资格不证明 Node20/Python3.10 最低版本运行或三语言 E2E。
原件为本任务 local `sdk-f7bac-formal-source-20261021-01/focused-syntax/syntax-final-proof.json`，
12147 bytes、SHA256 `dd6399acd46d24c2d32e31582bc5b26eb943380a1842fe0bcab25245c4e147a4`。

冻结副本 Java target 初始不存在；在 MAVEN_OPTS=-Xmx256m 下执行一次
`mvn -o -q test-compile` 实际自然 0，未运行 Surefire、test case 或 Host，未联网补依赖。
实际 JDK21.0.5，637 个新 class 均为 major61（既有 release17 目标），包括本次两项
A2AHostTest 与 FamilyRouteHostTest，已逐项绑定其源 SHA 和新 class SHA。
这不是 JDK17 runtime、Windows 或完整 Java SDK E2E 通过。
原件为上述任务 local `sdk-f7bac-formal-source-20261021-01/focused-java-compile/java-compile-final-proof.json`，
287696 bytes、SHA256 `1c64c82194932678f9f27650203604a995a586f0c63a1ba2b29363ab06dd7937`。

原06/07/08/09/10/11/13/14、完整 contract/非阻断 Rust telemetry、新 fullfacade Host、
fresh 三项依赖扫描、完整三语言/3 quickstart、Windows native/main CI/冷恢复仍待分别验收；
本阶段 checkpoint 仍需真实材料门禁、必要性复审与任务分支提交，不能由本节自动完成。
本阶段提交只是可继续验收的任务分支 checkpoint，不能替代合入 main 的完整门禁。
4 个整体行为 hold、behavior_model_closure=open 与历史 managed 首因 UNKNOWN 保持。
当前 AGENTS/README 的剩余完整门禁说明仍按原切点保留；AGENTS 可能进入
InstructionResolver 运行时输入，未来必要的 current lead/dated 分界变更须在最终受影响
运行验证前明确评估并验证，不能仅按普通 metadata 盲目复用行为结果。

## 2026-10-22 current lead 运行指令输入后继（受影响验收待执行）

本次仅将 AGENTS.md 当前迁移门禁中容易把单条12与14条总集合混读的阶段句，改为稳定的
A03来源观察边界与本既有 Evidence 的具名日期后继链接。A03 的 no-deps/resolve=null
历史事实、全部规则、pin、合同、政策及旧章节均保留；其它五份来源/政策说明不变。
本后继 committable 源码摘要为 `3f078657a0cae39a512ade2e422c41b6a5863a8dbc2998990df038536bc6b7c2`，源码1225文件中仅 AGENTS.md
这个非语义支持文件变化；另两项为 Baseline 当前摘要与本既有 Evidence 追加。

AGENTS.md 会在启用 project-rules 的路径进入 InstructionResolver 的运行时输入，
因此本次明确不是可以无条件承接全部旧运行结果的普通 metadata 更新。
旧05的523通过只保留其原AGENTS/runtime cut；不宣称新指令输入已取得同一523资格。
原12 bridge-only不含project-rules，既有31通过可依该明确影响边界承接，仍不认定历史
managed UNKNOWN 的因果修复。原01/02/03/04及06等编译、语法资格在实际Rust、manifest、
lock、include、config等编译输入不变的有界等价证明下保留各原epoch；不重标旧全仓摘要。

在此后继指令输入下，待执行原13 fullfacade（core/facade）的实际影响验证、新fullfacade
Host以及完整三语言/3quickstarts。只读复核确认最早读取S AGENTS的是原13：fullfacade
显式启用project-rules，handler/factory传入S cwd；原07虽allfeatures但无可执行Agent
doctest，09默认与10 core-profile未启project-rules，11仅no-run，14指定组合也未启用。
这些原必要gate可依各自实际资格推进，原13必须在最终指令下直接取证，避免先跑再重跑。
当前不冒原13/新Host/整套语言验证已通过，不更新97个
Audit或四个整体行为hold；behavior_model_closure=open、历史首因UNKNOWN及其余依赖、
Windows/main/冷恢复等未完成项保持。没有新增文稿、协议、状态权威或生成117产物。

## 2026-10-22 stable lead 当前输入的有限后继资格

原01/02 formatter、03/04 Clippy、06无默认feature编译、07两crate零用例doctest、
08 Protocol211、09默认Host35、10 core-profile112、11仅编译7个executable等入口已各有
对应原source/runtime epoch的实际资格。编译/语法输入未变的部分按有界等价承接；
07没有可执行Agent doctest，08不执行Agent，09/10未启project-rules，11不执行用例。
原12 bridge-only31也不含project-rules，依这个具体负向影响范围保留；这些分布于不同
feature/runtime切点的结果不是重新跑过的同一全仓数字或新增unique能力总数。

在上述明确的 AGENTS 运行指令输入调整之后，source1225/stage3/PF840 所绑定的
committable 摘要 `3f078657a0cae39a512ade2e422c41b6a5863a8dbc2998990df038536bc6b7c2`
下，原13 fullfacade 的 core33 与 facade12 两个完整 target 实际45通过、零失败/忽略/过滤；
这是最终稳定 lead 下的真实 project-rules/InstructionResolver 影响验证。
原14仍为原精确 argv 指定的一项 canonical source-operation 测试，实际1通过、零失败/忽略、
25 filtered（过滤来自原命令，不是新增跳过）。旧05的523保持其旧 AGENTS/runtime cut；
不把当前45或原 bridge31相加为独立能力数，也不冒新指令下同一523已重验。

原完整合同检查实际 mode=check、28 rustdoc profiles、113 blocking artifacts current，
协议5/42/91各通过且零失败/忽略/过滤；单独非阻断 Rust inventory telemetry gate 也实际0。
两者都观察到4项 telemetry drift、36646 inventory items/rustdoc format61，未更新生成物：
`contracts/sdk/public-api.txt`（19091206→19094482 bytes）、
`contracts/sdk/parity-manifest.json`（37238428→37242669 bytes）、
`contracts/sdk/facade-operation-catalog.json`（900931 bytes同长内容变化）、
`contracts/sdk/inventory-telemetry.json`（1460 bytes同长内容变化）。这是完整 Rust 库存信号，
不扩大 accepted5621 identity/kind、454 routes 或自动生成117产物；实际 generator/provenance
错误仍属阻断。当前原始113阻断合同与 accepted/source/shared catalog 均匹配。

新 fullfacade Host 原显式 build 真实自然0、零编译/链接警告；在没有其它producer时，
真实 stable-target producer 独占流式复制一件到 ignored `sdk-f7bac-qualified-host/echo-agent-sdk-host`。
source/dest 均159744272 bytes、mode0755、SHA256
`2c0bdfecfad72145849825f98caa92423eb240431095c8d98364cd6f8a799a5b`，
源 ns/inode/nlink/dev 前后同，副本独立 inode/nlink1 且纳秒mtime一致，文件及父目录fsync。
copy cap200000000 bytes、入场dual6GiB+actualsize+512MiB reserve与结束6GiB+512MiB均满足。
本件是实际 producer 的独立副本，不代表官方完整语言脚本会使用 outside copy；该脚本仍会
自行 build 并绑定 owned stable-target producer，后继必须真实核前后身份与 fresh Java XML。

上述各命令实际 Cargo/Node/guard/caller 均自然0、signal/error=null、raw完整不截断、
零编译/链接警告、无resource bound；source1225/暂存index3/PF、原4pycs/old19/原12两件及
actual framework2332全前后同、remainingProducer=[]。AGENTS在内的865项非语义输入
在本次有限验证期间未再改；这里只追加已有Evidence，保留全部旧失败与各自source/runtime epoch。

原件仅在本工作区 local `sdk-parallel-delivery-01a14385/sdk-f7bac-accepted-runtime/`：
13 raw3775B SHA43e43f0386df7babec123dd085c388f0572eb57f09fc5d2de2eee9a15bc5c71e；
14 raw1410B SHA551849d0d4442fcf1aba2875f0a672e41d30055ac9c04c9045518e6f26326b45；
contract raw14921B SHA4a2add0bbafaf3c3ff4a97077b70aa0722f9ac4bda9a12ccf0bd78a3f7c07daf；
telemetry raw14326B SHA53e3926d464a5cc50d94bb02918a46bbf7ac4386d407b35bb83283be6d8b429a；
Host/custody小proof3376B SHA4c437734e8ead3ff3a6ab391dd8ad8e0aba5d8f7aa0e8c36cd8dca3e71f28ff0，
并各有真实native.run、guard、caller及source-after原件；文件哈希只核身份不替代原件。

新SDK own4b来源双副本已独立保真准备，但尚未进行该sourcecut的三项依赖扫描；
完整三语言与3quickstarts/fresh XML、Windows native/mainCI/冷恢复仍待各自真实验收。
本节不能把Root来源准备冒SCANPASS、也不把文档边界充分或本机有限绿升级为4whole闭合。
历史managed首因UNKNOWN、behavior_model_closure=open与4整体hold保持，main/registry不发布。


## 2026-10-22 同一 own4b 来源的三项依赖扫描有限资格

在任务分支 `5f0863ccb426677be639382ff848c37927a9b3dd`、当前 PF77c、
1225 tracked/index clean 与非语义865项摘要
`3f078657a0cae39a512ade2e422c41b6a5863a8dbc2998990df038536bc6b7c2`
下，原两项 audit 与一次 cargo-deny 已实际执行；中间 registry-origin view generator
只是审计输入生成，不是第四项扫描。四步的原命令/Node/guard/caller 均自然0，
signal/error=null，raw完整不截断，无resource bound，前后源码、PF、原4pycs、old19
与actual framework2332不变，结束手工 remainingProducer=[]。没有重跑原Rust/合同/Host。

两项 audit 均显式使用 SDK own `audit-db`、`--no-fetch --deny warnings`，
实际加载1348 advisory、检查656 dependency，均零警告。cargo-audit二进制绑定
原版本读取0.22.2，cargo-deny原inner receipt实际0.20.2；工具字节身份前后同。
审计view由f7bac既有helper生成；656包及全部依赖边逐字段核对，仅 calamine0.36.1、
docx-rs0.4.22、rust_xlsxwriter0.82.0 的source/checksum换为既有registry来源，
其余653包全同，实际构建 Cargo.lock SHA256 1c865a57b7e17587fba2f668d956832cd7b0f6c6167e198304c965227f3debaa不变。view172646B，SHA256
`24cc21a58c05ea613450d89dc53e2a63a4c6b6849de65d17f932b1d985f51f5e`。

cargo-deny保留原workspace/all-features/locked四类check及正常online fetch。
原始700410B/10659行完整输出保留49个warning[duplicate]，既有严格helper对当前
SDK精确baseline4fba逐组比对：49组、107个version/source身份完全匹配；其它警告0，
末行advisories/bans/licenses/sources均ok。原natural inner、外层比较、Node、guard、
caller分别0，非ANSI清洗、offline、政策阈值放宽或借用framework的duplicate基线。

两件SDK own DB全程actual HEAD为`4b587f00996ebe5c05a73eaef105ef241e876414`、
tree为`523058420679c8a4901e43e8e5fcb1e1406e7436`；各1367个物理nonGit文件的
SET/SHA/mode、origin、config/hooks/info规则与工具/政策身份全保持，未见锁残留或
alternate/link。deny正常fetch改写FETCH_HEAD、ORIG_HEAD、index、refs及其logs、
shallow操作记录，逐项保留实际变化；初始1392文件的完整复制身份只表示copy-as-of，
不宣称扫描后所有Git内部字节/metadata仍同，也未把HEAD或正文变更当作可忽略记账。

原件仅在本工作区local `sdk-parallel-delivery-01a14385/sdk-f7bac-formal-source-20261021-01/final5f-policy3-candidate/actual-output/`：
`policy3-finite-minproof.json`（42822B，SHA256
`5bd1389fa3dcefd5fb7a300015aaf10e7741cdaf364243914a0b1a01441dcdde`）
列出原argv/时点、四步native.run/guard/caller、external-before/after、实际49组107身份，
以及`registry-origin-field-delta-proof.json`的逐字段结果。
原audit raw319B/SHA6fe500c00206190efd2525baa3d66e771477fe6d8e48a203c727bb391dc9f239；
origins-audit raw577B/SHA02b50901829344245d5b0fe74d5ac8596a80da3d4684191424ed7995bf05ae33；
deny原raw700410B/SHA753f436253d8806e3801dfb289efab47550e4f61cbc9ac08a1f19673c6ff1869，
其inner receipt与外层167B/SHA95e534c0cf4f88f78e33bac3c3e020f456244f6e52147a2c23ee6b2af5f1dd2d
各自保留，不把仅有hash视作命令已经执行的替代证明。

另有一次生成后只读字段reader误用ambient Python3.10，导入tomllib时因Python3.10缺模块失败1；
明确使用既有Python3.12读取同一已生成原件后实际0。主协调者已接受该reader环境修正；
原失败保留并与scanner分列，没有scanner/generator重试或安装依赖，不声称所有命令零失败。

本节仅补齐三项依赖扫描的有限资格。完整三语言/3quickstarts及freshXML、Windows native、
mainCI/冷恢复仍待分别验收；4whole/behavior_model_closure=open与历史managed首因UNKNOWN
均不升级。没有新增公共合同、协议或状态权威，不更新117生成物、97Audit或Baseline。


## 2026-10-22 telemetry测试退出JSON原子发布修复（完整语言仍失败）

SDK e391fe72 的唯一原完整语言入口实际自然1：TypeScript360项/358通过/1失败/
1项Windows专用NOT_RUN_PLATFORM，失败为 telemetry slow flush (input_eof) 在最终
host-exit.json存在后JSON.parse报Unexpected end，读取到不完整JSON；未保存读取瞬间字节，
下述确定性反例证明该窗口机制，不把受控0bytes冒作原whole瞬间快照。Python/Java、全部3quickstarts未执行；
保留原raw121327B/SHA70d8408d7a2472b787d69a9c77b07f49d2a437267871564a8413f7bd6ebe8ca8，
不得借已有49政策或Rust绿冒当前whole通过。原script/Node/guard/caller各1，
无resource bound/source drift；actualHost及独立copy均159744272B/2c0bdf/0755。
该失败原件及7045B/SHA175a7798708f127366c12ba0367009bbc8483af431ea0f5b33d7b863d75682c6
有限proof在任务local `sdk-f7bac-formal-source-20261021-01/e391-wholelanguage-candidate/actual-output/`。

只读沿TS/Python/Java同一exists-then-parse协议追到共享fixture最终路径直接write_text。
受控单次原writer AST反例在创建0bytes后暂停，Node原exists/read/JSON.parse实际1；
写完放行并join后同payload68B读取0。1538B结果SHA2ad3e6ff0bf39ee304fe8cdff8ef28791bf1bb9c44e7190245158e35415ea490
位于同任务local `sdk-f7bac-formal-source-20261021-01/telemetry-json-publication-probe/actual-output/result.json`；
该probe的pid/returncode是明确的synthetic输入，不冒新Host运行。

本修复源码归属为
`sdks/shared/testing/telemetry_host_fixture.py::publish_host_exit`；纯回归归属为
`sdks/python/tests/test_telemetry_host.py::test_host_exit_publication_is_complete_before_visible`。
二者都是SDK测试夹具边界，下面的发布和反例结论仅指这两个实际修改路径。

本修复切片仅SDK-owned shared telemetry fixture：复用同仓tmp→replace模式，
真实父进程child.wait返回的原pid/code仍为唯一来源；publish_host_exit把完整JSON先写
同一隔离目录的临时路径，再rename发布最终host-exit.json。三语言原有exists/read/
returncode0、hold/ACK/EOF强断言及全部超时不变；不耦合managed持久性backend，
没有framework/Host生产/API/新协议或状态权威变化。

现有Python telemetry文件中的纯确定性回归不依赖Host/collector且不受Host skip约束：
Event在实际writer写到一半时固定观察最终path仍absent；release/join后原读取须获得
完整精payload。对提取的旧直接发布行为该回归明确失败1；单点atomic修复后同回归
1通过/0skip，Ruff格式化/检查、format--check、两文件AST+compile-only全部0。
每命令30秒/256MiB内，无真实Host/collector/Cargo/整套语言/扫描或安装依赖；
中间Ruff I001/BLE001失败原件保留，收窄测试异常捕获；另根目录Ruff与原入口Python
工作目录的import分组推断不同，补核后恢复原import分组，最终以原Python工作目录重验0。
本阶段原件在同任务local `sdk-f7bac-formal-source-20261021-01/telemetry-atomic-publish-fix/`，
`publisher-old-direct-red.*`保原反例；`direct-validation-python-cwd-proof.json`8952B/
SHA46f89db773cd379d20c8ad07f4aab011a9c405f9d837abd4978231cb430cbbd8
列最终5条实际命令、native/Node收据和资源边界。当前committable非语义摘要为`252ed07318cd0684867d1e5e0aeeb6be548967fe18df3daa6e5a341f60d1f037`；
这两项夹具/测试变化不重标旧3f工程/Host/三policy的各自source/runtime epoch。

本节只支持JSON发布窗口的修复与纯回归，不是完整语言后验或生产Host修复。
同次现场Python proxy在Rust child正常退出后出现daemon buffered-stdin finalization fatal，
其它模式出现stdout BrokenPipe，分别保真实stderr，尚未修复/验证，不用本原子写修复
解释或关闭；历史managed首因UNKNOWN、四whole hold与behavior_model_closure=open保持。
公开API/语言示例和网站功能未变，因此本测试夹具切片不改examples/echo-website；
后续完整入口、平台/精main与具名残余结算仍须另行真实验收。


## 2026-10-22 telemetry proxy独立退出缺陷子项（本机纯回归已验，整体后验待执行）

Owner是SDK测试夹具 `sdks/shared/testing/telemetry_host_fixture.py::proxy`，
与前节JSON发布窗口、旧Host-init/pre-retry缺陷、历史managed未知原因分列；不新增Finding。
在已提交publication修复4566a3b上，单次真实proxy/明确synthetic Python child反例分别观察：
父stdin保持open时proxy自然-6，stderr为_enter_buffered_busy stdin锁fatal；父stdin正常EOF而
stdout消费者先关闭时proxy自然120，stderr为TextIOWrapper stdout BrokenPipe。两件真实
parent child.wait均记录各自synthetic PID/returncode0；没有cleanup信号、重试或残留进程。
各一次观察不是“保证必现”，更不是Rust Host或SDK运行验收。
原件在任务local `sdk-f7bac-formal-source-20261021-01/telemetry-proxy-lifecycle-probe-v2/actual-output/`：
`probe-minproof.json`4990B/SHA3b2b9eb16047e5f6ded6ffefce79c7e4484f38019cf1cd3bc6b43193c3acd622，
含原argv、自然proxy/child/cleanup码、两stderr、资源和实际sameUID/PGID剩余空的观察。

根因边界：原daemon input_loop可在父stdin仍open时阻塞sys.stdin.buffer并保有Python
buffered-reader锁；原输出读者关闭后forward虽接住BrokenPipe，Python输出缓冲仍在
interpreter finalflush再报错。真实child已退出0不能替代proxy的真实退出状态。

本修复继续复用同一child/Popen/input_lock/control/child.wait与publish_host_exit；
父stdin用os.read取原始字节、保完整行及EOF尾帧，child管道无Python缓冲；
`sdks/shared/testing/telemetry_host_fixture.py::write_all`按实际os.write返回长度写完全部
字节，不把UTF8分块当字符切割。输出消费者关闭后仍排空原child stdout/stderr直到EOF，
不再留下Python输出pending buffer，也不以os._exit、延长超时、无条件join开放stdin或
吞掉真实child stderr替代退出。private host_args仅让纯回归传显式Python interpreter argv；
真实CLI默认[host,--config,...]与三语言wire/API和原强断言完全不变。

直接验证归属 `sdks/python/tests/test_telemetry_host.py`：新增不受HOST skip约束的
`test_proxy_partial_writes_preserve_utf8_bytes`与
`test_proxy_reaps_child_without_buffered_stdio_finalization`（两个param值）。
连同原publication回归，共4项实际PASS/0skip；执行真实同proxy函数+明确synthetic child，
父stdinopen模式须代理自然0且逐字节收到212992B含中文/emoji/CRLF输出，提前关闭输出
读者模式须原child被排空且自然0；两者exit receipt的原childPID/code和代理码分别核对，
stderr精确等于synthetic预期字节、无fatal/BrokenPipe泄漏，整个实际UID/PGID无残留。
纯部分write回归另验证3字节前缀多次写与0进度报错，不伪transport状态机或mock exit0。
测试源采用显式sys.executable+script，未加平台skip；本机运行不冒Windows实测。

原Pythoncwd一次Ruffformat、精3testIDs展开4pass、Ruffcheck、format--check及2文件
AST+compile-only五命令均自然0/nullsignal/error/raw完整，各30s/256MiB内，未执行
Rust Host/collector/整套语言/Cargo/扫描或安装依赖。直接原件位于同任务local
`sdk-f7bac-formal-source-20261021-01/telemetry-proxy-fix/direct-validation-proof.json`，
9210B/SHA0e01d67ad6db29e97d8db2d912da78a893bbf49c7846e4cad260b9f1d33fdee6。
当前committable非语义摘要为`8a840da4012d4fb09bb221de9e8791a2161759e8cc8fca15e7e35898ec6ac55b`；旧publication252ed、原whole e391失败1与旧工程、
Host、三policy资格各保其实际epoch，不改写原raw/失败/历史header/source_refs。

子项本机有界修复与直接回归已具上述证据，整体闭合条件仍是最终冻结cut的原完整语言入口
三语言+3quickstarts/freshXML/actualHost/source/resource收据及独立复审；尚未取得该后验，
不自动resolved四whole或历史managedUNKNOWN。Baseline仅刷新当前摘要，inventory/behavior
闭合状态不变；公共合同/产品API/示例和网站能力未变，因此不改examples/echo-website。
main/Windows/冷恢复仍各自验收，非本切片发布授权。


## 2026-10-22 默认stderr共同Finding及三语言真实退出后继

本节对应`finding.sdk-default-stderr-backpressure`，repair与verification复用本Evidence，
rereview复用既有language failure-concurrency Audit的新challenge。代码切点
`source:a92b96b149d204822aa6ffdb42b512e7257293e2eb1a9511d46a169c87da578b`，真实base为45914042ea7f9e5b05f5111415c52538c0b106c7加本次精stage10；
历史Evidence observed_at=b80、source_refs及全部旧正文保留，新的测试精锚只在Audit新challenge。

本切片实际源码/说明归属（唯一owner均为SDK；不改framework/Host生产/wire/依赖）：
- `sdks/python/src/echo_agent_sdk/client.py`：仅给原ACP spawn传stderr=None及原因注释。
- `sdks/python/tests/test_client_close_callbacks.py`：真实默认启动三参数官方ACP peer回归。
- `sdks/python/README.md`：默认诊断继承应用stderr、process.stderr None与父sink的owner。
- `sdks/typescript/src/client.ts`：默认inherit/显式onStderr原PIPE及listener，内部nullable类型不cast/export。
- `sdks/typescript/test/client-stdio-close.test.js`：官方Node Agent/ndJsonStream的新默认三组及callback一组。
- `sdks/typescript/README.md`：既有callback与默认诊断ownership说明。
- `sdks/shared/testing/telemetry_host_fixture.py`：test-only exclusive binary stderr-file fd2保持到真正解释器退出。
- `sdks/python/tests/test_telemetry_host.py`：沿原Process真实wait后读取shared file，无额外drain。
- `sdks/java/src/test/java/com/echoagent/sdk/TelemetryHostTest.java`：复用既有test-only原Process观察与fullfile。
- `sdks/typescript/test/telemetry-host.test.js`：仅原wire三模式移为shared file及真实proxy终态断言；其余原三个函数体不变。

### 默认私有stderr pipe的独立before与after

Python19：真实public SDK与官方ACP peer的0stderr控制通过；2MiB B在握手前停滞，
只对同原Process增加sole drain后同一个原future继续，child自然0。RootT本机原件
`.echo-semantic/local/sdk-governance-acceptance/python-default-stderr-backpressure-probe/actual-result-proof.json`
9557B/SHA45154914c254109ad56b716f52365feca277d691f19427a56d5af81922788924。
TS21B：Node22 noOnStderr/0reader/nonflowing原promise pending，原write后/init marker尚无，
只加同child data reader后原promise完成、精2MiB/EOF/child自然0；RootT同本机根下
`python-default-stderr-backpressure-probe/typescript-candidate/actual-attempt02-result-proof.json`
14224B/SHAa1e1dbaa823286ea1f3bc057e785475df2405093e364cd88163956d0c821f35c。
TS21a01是控制已正常退出后的晚EOF订阅失败，UNKNOWN/native1/outer125原件保，2MiB当次未执行，
不得把它当生产red或改0。上述RootT与下面S local均只本机原件定位，不是Git code_refs/source_refs。

Python20实际原Pythoncwd生命周期模块10PASS/0failerror/0skip（新0/2MiB/25B Unicode-LF-CRLF
三例+原七例）。新三例不加child reader/transport override/换child，应用fd2普通文件收全原字节，
真正默认SDK initialize/close/同原Process自然0，main成功无cleanup替代，应用stdout为空。
原件为S任务local `sdk-f7bac-formal-source-20261021-01/python-default-stderr-fix/`：
`native-three-cases-minproof.json`8901B/SHAd53863614f45957af62b6489e495cfa058749f8a103757d679fdfca7b7a05e9c；
原命令`.venv/bin/python -B -m pytest -q tests/test_client_close_callbacks.py --basetemp <该任务新目录> --junitxml <新XML>`，
native/Node/guard自然0、18.39s、peak162398208B、无bound/guard信号/UIDPGID残留；原Ruff/fmt/syntax同cwd全0。

TS22原stdio文件16PASS/0failcancelledskip/todo（新默认0/2MiB/25B Unicode-LF-CRLF三例、
既有onStderr 2MiB一例、原close十二例）。使用官方Node ACP peer和维护的metadata/digests，
不加Python前置或手写RPCmirror；只观察同原child对象不加默认drain，不把cleanup信号当成功。
原`npm run build`、`npm run build:examples`、Node--check和`node --test test/client-stdio-close.test.js`
均自然0；closeStdioOwned函数体逐字节保持，内部nullable stderr无export/cast。
本机原件为S任务local `sdk-f7bac-formal-source-20261021-01/typescript-default-stderr-fix/`：
`native-four-cases-minproof.json`9697B/SHA8ecd5e2f582ca0e642de7444a8e5f5e000cef6a016f0f4c11fd446180bedd1f1，
`direct-source-test-phase-proof.json`6178B/SHAe1b5d4da0a7353e1af51e3c22687333581f934b966bc65cad063c1308b3feacd。
原native7.18s、peak258899968B、无bound/guard信号/UIDPGID残留；默认/回调2MiB精SHA均
7995dfceebfb9fa8972d361d53d899f6e268ecd6d823b5737198041fabc02e20，Unicode25B精SHA
45ca338ffaf84a7effbffaae99a3d7a45a05bfe28bdfeebed4fe429f8513db75。显式callback验证用ASCII，
不泛推任意多字节跨chunk解码。原七/十二关闭故障用例各保其intentional失败清理范围，不称其child全自然0。

### 17/23观测与24实际五个真实Host场景

既有proxy采用原child.wait事实，三语言分别记录外层代理真实终态；shared可选fd2捕获在child前
O_EXCL打开并加O_BINARY（POSIX0）后dup2，直到Python解释器最终退出才由OS关闭，不恢复/提前关fd2。
原本机smoke8741B与binary8853B各保其cut；后者65B独立LF/CRLF/中😀/atexit晚输出精等，
是Mac本机证据与Windows源语义补强，不是Windows错误复现或Windows CI通过。17 Java仅单文件
release17编译到独立classes/旧637class+18jar不动是17 asof；24标准Maven正常重编译已授权，
旧三Telemetry类及旧Telemetry XML/text的小副本保，不能再声24缓存不变。

24实际命令顺序为原Python publicclose精pytest一项+freshJUnit、
`mvn -o -q -Dtest=TelemetryHostTest#publicCloseWaitsForRealSlowTelemetryFlush test`原一项+freshJUnit，
以及`node --test --test-name-pattern=telemetry slow flush retains original Host owner test/telemetry-host.test.js`
原三项；Java的-o只该本机focused，不加到最终原whole入口。当前TS源共六项，其余三项被明确namefilter
未执行，Node22实际TAP只报告3PASS/0skip，不冒unfiltered6或平台豁免。

PY1/Java1/TS3合五场均真正Rust child自然0、原Python proxy自然0/信号空、parent_pid关系同原对象，
真实proxy终止后完整host.stderr字节/SHA核对，无三已确认Python fatal/BrokenPipe/Traceback marker。
TS三场shutdown waiter实际以transport_error/closed_handle settled，而非成功shutdown响应；
物理flush结论来自Host0、实际collector记录与ACK/held屏障。input_eof在hold时未调用publicclose，
其close_settled=false是未调用初值，不当作活动close waiter；publicclose与rpc_cancel按原调用/请求身份结算。

原件为S任务local `sdk-f7bac-formal-source-20261021-01/telemetry-focused-five-24/focused-runtime-output/`：
`actual-five-focused-minproof.json`31070B/SHAbdbfa113f0b1f18c38669d457ac3808feda91e7165ed249e314ba4b85f242272，
含三原argv/Node/guard/caller、fresh XML/TAP与五份proxy/Rust receipt/fullstderr/holdflush原件定位。
Py raw98B/SHA93c088ea44701d55ee105654a6587cad21d503ab26286ad5169a7ee0b2eb4f13；Java329B/
SHA26d236bf5169aba60affad492af3adeb0362ea8a9ceb6ade53ac88c1b64c8d84；TS1652B/
SHAa1b55f5508be0d55d60995712c088a1ce3f76fe2f27f59ace9bf5a83eba33619。各native/Node/guard/caller0、
signal/error=null、raw完整、source1225/staged10index/PF/原4pyc/F2332及真实Host+copy2c/collector3d99/
TS22 dist身份全前后同；无budget bound，精UIDPGID及全局producer含collector剩余空。
没有Rust/Cargo/Hostbuild/collectorbuild/whole或扫描重跑。固定opt1/assertions/overflow/unwind、
own8GiB/dual6GiBfloor+512MiB reserve/2s与warm/cold准入保持，不以本机focused冒新全套数字。

### 成熟实现参考与完成边界

默认诊断ownership参考官方MCP TypeScript固定fa56d815：
https://github.com/modelcontextprotocol/typescript-sdk/blob/fa56d815aceceb089a695e765c98a83b8ce75421/src/client/stdio.ts
（Root核8338B/SHA09427858bda9c2005fc8bd480083ed899717f4c521107a17f845ed0f907d8d00），
以及官方MCP Python v1.12.4 stdio_client默认errlog=sys.stderr：
https://github.com/modelcontextprotocol/python-sdk/blob/v1.12.4/src/mcp/client/stdio/__init__.py
（Root核8991B/SHAc18c54e5eda974d3f500db9fae8c70acfd2ea728b2aeee71d5cb323f670ded3e）。
本项目仅用既有ACP None继承与Node显式callback分支；不引库/API，不照搬其session/group/2s清理。

本具体共同Finding由真实repair/verification/rereview局部resolved，实际75resolved+managedUNKNOWN1open=76；
46finite verified与4whole hold、behavior_model_closure=open保持。原e391 whole自然1、旧各epoch/失败
与21a01 UNKNOWN保持；五场不替当前完整三语/3quickstarts、Windows/精main CI等后续验收。
accepted5621 identity/kind及454routes不增退，完整Rust inventory仅遥测不扩三语言合同；旧Rust/合同/
Host/三policy各原source/runtime切点不重标。README两语言已同步；examples调用无变且22 examples编译0，
website没有SDK投影（已有只读C0），CI原glob/入口会覆盖新tests所以无需改job；不新增发布或merge授权。
