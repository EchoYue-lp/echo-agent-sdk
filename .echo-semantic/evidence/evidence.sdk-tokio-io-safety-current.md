---
schema_version: 1
id: evidence.sdk-tokio-io-safety-current
kind: evidence
observed_at: 449934d22f3ab69b867f02cfb64565e073c8660e
source_refs:
- Cargo.lock
- .cargo/cargo-deny-duplicate-baseline.json
- AGENTS.md
- docs/sdk/governance/dependency-warning-policy-proposal.md
- echo-sdk-host/Cargo.toml
- echo-sdk-protocol/Cargo.toml
- scripts/check-cargo-deny-baseline.py
- scripts/tests/test_cargo_deny_baseline.py
- scripts/check-sdk-contracts.sh
- scripts/check-language-sdks.sh
- .github/workflows/rust-ci.yml
supports:
- behavior.sdk-repository-ownership
limitations:
- The original executions used the candidate source:56b1a7e62931cc9edb5c58e3a7be71bd34baf408d9860101da82f8def8e6da9c, not this later Git HEAD; public main 449934d22f3ab69b867f02cfb64565e073c8660e archives the same source bytes. The metadata successor still requires its own precise main CI.
- Fresh dependency scans prove only their exact policy and lock scope; new Tokio Rust gates, rebuilt Host language consumers and precise main CI have separate execution evidence.
- Existing duplicate identities and accepted risks are unchanged; the Tokio unsound advisory is patched, not ignored or accepted as a new risk.
- Original network fetch failures, missing-crate offline failure and disk-pressure interruption remain recorded as failures or interrupted execution.
- The historical managed passive-exit UNKNOWN and its human risk acceptance are unchanged; no global coverage or causal repair is inferred.
---

# SDK Tokio I/O safety exact-lock successor

## 支持的结论

[官方RUSTSEC-2026-0386](https://rustsec.org/advisories/RUSTSEC-2026-0386.html)
指出Tokio AsyncFd构造器不满足I/O safety，patched区间为`>=1.51.6,<1.52.0`或`>=1.53.3`。
本SDK原main `3daaa573a60d83e7064ff0796c48bc67ddd6297b` 的
[原CI39978621886](https://github.com/EchoYue-lp/echo-agent-sdk/actions/runs/39978621886)
dependency-policy job `120499727576` 实际失败，不能用该main旧运行代替新补丁验证。

候选Cargo.lock SHA256为
`822ee9a6936f25978283d38d4d07e4e8d7e006ec5834bfeaf80542f79e1ed52e`。
656包中仅Tokio1.53.1→1.53.3及checksum变更：新checksum
`ce3335fa71841cda333a58d7615b03901380ecf09d59b3296d21f8bbac0dde4e`
与crates.io官方metadata及实际下载crate相同，版本not yanked。
655其它完整package块、Tokio依赖列表（包括tokio-macros2.7.2）、manifests/features、
SDK framework `f7bac62eee04a963f92a3e4308b6ea2336e91cc0` pin、Host/Protocol源码、
accepted5621/454和CI均不变。这是安全与运行时补丁，不能说成只改无行为含义的元数据。

原`cargo update -p tokio --precise 1.53.3`实际0，同时产生的无关socket2/windows-sys
边回写被逐项识别、保存原diff并排除。完整locked在线metadata实际0且没有重写新锁，
证明保留原边能被Cargo真实解析，不以no-deps/resolve=null充当完整图证据。

fresh `cargo audit --deny warnings --json` 自然0，1351 advisories，数据库HEAD
`84d943a6deeb8cc6c6f632531aaf4f1c62275239`，0漏洞，warnings为空。
原`cargo-deny0.20.2` workspace/all-features/locked四类scanner自然0，完整699920bytes
raw末行`advisories ok, bans ok, licenses ok, sources ok`。49组107个精确duplicate
name/version/source身份与原机器baseline逐项完全相同；只重绑lock SHA与观察说明，
policy、command、tool版本、groups及人类授权不变。复用同raw的精gate实际0，14个policy
tests实际0。duplicate提示完整保留，不称零warning审计，也未增加advisory ignore。

独立reviewer `review_cli_215` 已按官方advisory/metadata/downloaded crate、三仓锁差及原政策
作有限复评PASS（Critical0/Important0/Minor0），仅支持锁变化与风险合同；不能替代运行门禁。

## 来源与范围

header使用已有合同支持的`source:<实际候选摘要>`，baseline比较祖先是长期可恢复的
published main3daaa。该main仍含Tokio1.53.1；本Evidence没有声称新锁已存在于旧Git。
普通Python标准库独立重算872个非.echo文件，路径排序后以`path\0file:sha256\n`
（symlink使用`symlink:target`）聚合。对3daaa原Git树重算复现原公开CI摘要
`02777d702f5c96ea9b2bb980318b7b3a0f2e1c7e698fb954d947591a864b19f1`，
对实际候选算得header摘要。未import/execute插件或伪SessionStart；canonical源码只读核规则。

新的Tokio source锚须在合入后的真实精main/CI验收后，以metadata-only后继续接公开GitSHA。
在此之前不引入会经squash消失的localcheckpoint作为长期来源；未来业务增量不能把当前
source摘要旧锚留成不可恢复引用。

原日志与机器收据保存在跨仓本地任务目录的`evidence/delivery/`，不冒充公开SDK Git路径：

- `sdk-tokio-audit.json`/stderr与`SDK CI dependency`原日志保留Tokio1.53.1红。
- `sdk-tokio-cargo-update-collateral.patch`保留Cargo初始无关边回写。
- `sdk-tokio-lock-delta.json`记录656总包/655等价块、新checksum与未变Tokio边。
- `sdk-tokio-locked-metadata.stderr`保留首offline101仅缺新crate，在线后继JSON实际0。
- `sdk-tokio-after-audit.*`、`sdk-tokio-after-deny.*`保留原HTTPS官方RustSec fetch网络失败。
- `sdk-tokio-after-audit-v2.{json,stderr,receipt.json}`、
  `sdk-tokio-after-deny-v2.{raw.log,receipt.json}`记录后继真实fresh0；仅进程GitConfig将官方
  同repo HTTPS改SSH，未改持久config、不用no-fetch。原deny raw SHA256
  `4c8b345a8fa8134a2cfa30dccf88dd90f6a2a7c3d431e6713d3dee8648f68aac`。
- `sdk-tokio-before-inputs.json`保存61项工程输入和872项tracked非.echo源文件SHA。
- `hash-sdk-source-standalone.py`与`sdk-tokio-source-hash.json`保存普通hash计算规则与逐路径结果。
- `sdk-tokio-gates-commands.json`及`SDK Tokio gate`原日志记录本轮运行；fmt写/查两项已0。
  首次clippy因全盘只余841MiB而只中断自身编译，退出signal2，保原日志；未声称通过。
  新锁后续Rust/contract/完整language尚待新的同入口真实执行，旧Host及旧语言统计不复用。

## 已知缺口

本材料写入时，fresh依赖扫描和有限合同重绑已通过；新Tokio全部Rust门禁、重建原pinned
Host及官方完整三语言/三quickstart、精确mainCI尚待后继日志。macOS运行不能冒充
Linux/Windows，Java目标release17不能冒充Temurin17运行。CI semantic结果与依赖、语言、
Windows各job需分别核对，单一job绿不等于交付完整。

旧Evidence epoch、Behavior/Audit主状态和全局coverage保持。历史managed passive-exit首因
UNKNOWN及原人类risk_accepted不在此修复范围，不虚构因果修复或新风险接受；registry发布
和官网部署也不由本材料自动完成。当前source摘要是明确的临时当前树锚点，不能用它
永久替代真实published source revision。

## 2026-10-24 A03历史限定的非语义澄清

AGENTS只将紧接当前full locked metadata结果的旧句明确为“A03 原 metadata 仅 no-deps”，
区分旧A03/resolve=null与本次完整图结果，不改变权限、依赖或运行策略。新候选source摘要已
按该最终文字重算；原摘要和逐路径清单保存为`sdk-tokio-source-hash-before-a03-clarification.json`。
此前编译结果按实际未改的Rust/manifest/lock输入保留；后续测试继续在同运行实现上执行，
不把旧源码摘要或日志时间改称新一次完整运行。

## 2026-10-24 新锁运行的中间观察（未结算）

同命令v2在新Tokio上Clippy all-target/all-feature与严格panic lint均实际0。
workspace all-target/all-feature尚在codegen时，活跃2GiB磁盘护栏检测free1960767488bytes，
只对本SDK进程组发SIGINT并保留signal2与完整原log，未出现编译诊断错误，未执行测试。
`SDK Tokio gates v2`原收据分别保留两项成功与此中断；不把中断称作workspace通过。
后续nodefault/doc/default/core/bridge/facade/contracts及新Host完整语言尚未启动，
需释放有明确来源和无open句柄的任务缓存后按同命令继续。旧Host/语言运行仍不续称本次。

## 2026-10-24 最终本地后继资格（精main CI另验）

本节是上述中间观察的后继，不改写原网络失败、缺crate或两次磁盘SIGINT记录。
当前新锁 SDK 的14个Rust/contract门禁全部实际0；加上fmt写入与官方完整语言，
16个独立步骤都有成功原件，所有已完成编译日志零warning。fmt/Clippy/严格panic lint、
workspace all-target/all-feature、featureless、doctest、Protocol默认、Host默认/core、
bridge编译/真实测试、facade及无reverse-bridge配置、合同与官方语言均实际完成。
原workspace的13个harness共523 passed，0 failed/ignored/filtered；其它配置的重复
case不叠加为新的unique测试总数。早期clippy按旧AGENTS文字运行，但仅后续A03历史
限定句不同，Rust/manifest/lock等60项工程输入逐字同；该非语义增量及旧source摘要保原件。

唯一`bash scripts/check-sdk-contracts.sh`在nightly-2026-08-06上实际重算28profiles
（default/full及26leaf），113个阻断产物全部匹配，accepted external/catalog及
source-contract均匹配；4个Rust库存遥测产物报告非阻断drift：public-api.txt、
parity-manifest.json、完整facade-operation-catalog.json和inventory-telemetry.json。
它们未被手工修改或自动提升为accepted合同。生成器按已有约定对SDK精pin f7bac62
的framework manifest/原锁做API反射，target明确仍在本SDK目录；此来源不是新Host运行，
不将该反射的旧pin依赖图冒称SDK新Tokio运行证明。

官方`bash scripts/check-language-sdks.sh`完整命令在
`2026-10-23T17:17:53.908410Z`至`17:24:37.614687Z`实际退出0。
它通过SDK根新锁完成sdk-facade-all Host构建，并对该新Tokio产物执行原完整三语言与
三quickstart，未使用历史Host7154或旧whole-language结果：

| 新完整入口 | 实际结果 |
|---|---|
| TypeScript | 381 total，380 passed，0 failed/cancelled，1 Windows-specific skip |
| Python | 463 passed，1 Windows-specific skip，另18 subtests passed；Ruff与197文件格式检查0 |
| Java | 78个报告全在本次命令时段新写；394 total、393执行通过、1 Windows-specific skip、0 failure/error |
| Java执行环境 | XML实记Oracle21.0.5、Mac OS X；pom release17是编译目标，不冒Temurin17运行 |
| examples | 原正式入口的三语言quickstart全部实际执行，原末行`language SDK source checks passed` |

实际新Host SHA256为
`d167b366a0288c91ce9debca8725b11425f05b22e4fb40bc8b280e68291b389d`，
原完整语言中的真实native corpus也记录同一hash。该225108448bytes原产物已逐字校验并
APFS clone保存为本地`evidence/retained-binaries/echo-agent-sdk-host-tokio-1.53.3`；
它不属于公开SDK Git源码引用，也不因hash不同而自动证明所有行为，实际断言仍由各测试支持。

当前Cargo仍使用既有rsproxy-sparse配置；1.53.3实际crate字节与官方crates.io版本metadata
的checksum相同，不把该下载声称为直接canonical-origin/cold-cache来源。
RustSec扫描的官方同repo fresh fetch与该Cargo crate来源是两个独立证据边界。

最终普通source hash仍为header的56b1摘要，872个非.echo源码；root新锁及Rust/manifest/
CI/Host/Protocol/三语言源码没有验证后漂移。官方语言生成了14个未跟踪shared testing
`.pyc`，它们均不存在于冻结源码观察。完成执行并lsof确认0open后，逐SHA记录且将这些
原字节缓存整体移到任务evidence保存，无源码删除；带缓存886文件/8c1a摘要也保留原件。
退役后重新算回872/56b1，未将生成缓存塞入提交或伪装源码。旧78个Java XML另在运行前
完整copy2保存，未拿旧XML充本次计数。

| 最终本地收据（均为任务目录内原件） | SHA256 |
|---|---|
| `sdk-tokio-gates-commands.json`（含早期SIGINT） | `6e3bcc52901d75f961b60a6f42be345daf7db7bcfdbeaf89d708f32f292998a9` |
| `sdk-tokio-gates-v2-commands.json`（含workspace SIGINT） | `65650b4eadd187e8d4461d2143898858debd541ef635020c601a154a45e335f4` |
| `sdk-tokio-gates-v3-commands.json`（后继12步骤全0） | `dd9018d3ff016687cca31242c733e795b52a40ed87ab23061f4ab82db1874bbf` |
| `sdk-tokio-final-local-summary.json` | `bd723d640f17ff21f133d0520269026f8fbc584628e3055043f8eff122bcf3a7` |
| `sdk-tokio-source-hash.json`（退役cache后） | `7646ec3de2270518e0c272da8aaa981f2b3ff900a2fe3e31a7e6256c37dad170` |
| `sdk-tokio-generated-cache-retirement.json` | `9ad8d8e9a407211cb1ab2097e0f35f8daf2da91272721c653a8916d6a46ccf05` |

本地资格只支持本新锁及具名矩阵，仍不是Linux/Windows或精main交付。后继SDK main全部
CI与合入后真实Git observed_at续接，由主代理独立完成；旧UNKNOWN/风险裁决、全局覆盖与
其它Evidence epoch保持，不由本地全绿扩成所有SDK输入/后端/故障组合完成。


## 2026-10-24 公开 Git 锚点续接

原 source: 阶段是上文保留的实际过程。现 [PR12](https://github.com/EchoYue-lp/echo-agent-sdk/pull/12)
已实际合入 main `449934d22f3ab69b867f02cfb64565e073c8660e`，该提交与已审分支的完整 tree 相同；源摘要仍为 `56b1a7e62931cc9edb5c58e3a7be71bd34baf408d9860101da82f8def8e6da9c`。
header observed_at 续接此真实、可从后继main祖先恢复的Git提交；source_refs 现在读取这份公开源码，
不依赖删除后不可达的临时任务分支。原16成功步骤、两个SIGINT、旧HEAD、wall-clock、平台skip和反射／运行来源均保持其原义。

本后继只改.echo元数据，Cargo.lock、依赖policy、Host、Protocol、语言源码、accepted产物和CI配置均无差异。
这是同源观察归档，不伪装成重新执行，也不声称此前 source: 锚本身是Git ref。最终交付以最后纯元数据main提交的完整CI结果为准。
