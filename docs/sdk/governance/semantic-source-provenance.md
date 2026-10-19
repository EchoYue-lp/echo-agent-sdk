# 语义观察版本的来源保存

语义模型引用具体 SDK Git 版本，保存的是当时实际源码、文稿与原始收据。旧代码不能换成新代码后仍冒用旧观察版本。下列本地私有快照已核为 SDK 自身的可恢复候选树，不是顶层 workspace、CLI 或外部应用快照；它们不代表完整合同、能力或产品发布验收。

Root 已创建并 atomic push 八个 annotated tag `evidence/sdk-governance/<完整40位commit>`，推送退出0。2026-10-14 07:49:21 UTC 的全新空 bare 仓库从 SDK 远端逐个 exact tag refspec 取回，未使用本地 alternates，八个 tag object 的 peeled commit/tree 均与原件完全相同。它们是证据引用，不是 SemVer release，没有 SDK main merge、GitHub Release 或 registry 发布，也不新增运行时权威；不得重新指向另一版本。原 planned 稿 SHA `77fc8c50…` 保留在 Admin 历史目录，不冒充此次实际完成收据。

## 可审来源清单（2026-10-14）

| 真实 commit | 真实 tree | 原本地 snapshot ref 后缀 | 用途／当前 observed_at 对象数 |
| --- | --- | --- | --- |
| `56929154c1851473e9e4837901b610cecf001b2d` | `e03dd518d2165f1f82db33039e4043893dcac621` | `sdk-candidate-17cf-20261013t224716z` | 早期当前源码与历史收据重观察；115 |
| `9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d` | `6e221776435c6844ef595c250f3599a4e4807ccf` | `sdk-governance-ttl-20261014t005404z` | TTL 正式取证与已接受依赖政策文稿；24 |
| `47579c895657c182eceacfbb63865b04ec55a4a1` | `da9193cc5ba83219953479f33509b33f6264c1d4` | `sdk-be5-reviewed-20261014t023749z` | be5 运行证据及对应文稿切点；9 |
| `4febaba8b00851be63b256d1dce013fb566b3bb6` | `21adc4f2185a2b1768decb6f34c1a232ee38b564` | `sdk-final-doc-reviewed-20261014t024441z` | 四份摘要修订后的可恢复文稿切点；正文历史引用保留 |

本地完整 ref 前缀是 `refs/echo-semantic/snapshots/`。计数仅指当前正式对象顶层 `observed_at`，不是完成度；前三版本另分别有22、1、1个 Audit 顶层 revision 引用，不重复计为行为覆盖。原运行时间、源码/Host SHA 和支持范围以各原收据为准，不能推断这些命令在 snapshot commit 时重新执行。

本次另外四个已取回标签如下，仅增加来源可恢复性，不增加能力完成分母：

| 真实 commit | 真实 tree | 来源用途 |
| --- | --- | --- |
| `76601b242d7c354352f3b5c7d4bf099873746b14` | `1c6514b46007578e40eec44d351922c82f78fbed` | 客户端与正式运行证据 |
| `69b6c17f8eb766fe8cdd0e2c878369f1bed268d6` | `53def42a45207f534e8b7f06e8a78a1e4c4832d2` | Host 源码闭包 |
| `cd5b78467f77ac28919cf9dda6877662bed744f4` | `da8aeccb60d4fb7579170ac0e75dc515cbd4711c` | 后继回归来源 |
| `bd8e42e732588181b7a2589d91e9ffdce02d23e1` | `1c7bc45b615dad1ae1ae07a1a077bc247a7b36b4` | 后继 A2A 消费来源 |

原始 [fresh-fetch 收据](https://github.com/EchoYue-lp/echo-agent-sdk/blob/0ebe15f66d0aa57c7b7a92ae68cd1be106114ae9/docs/sdk/governance/evidence/semantic-source-provenance/receipt.json)、[fetch 日志](https://github.com/EchoYue-lp/echo-agent-sdk/blob/0ebe15f66d0aa57c7b7a92ae68cd1be106114ae9/docs/sdk/governance/evidence/semantic-source-provenance/fetch.log)及[逐字节校验](https://github.com/EchoYue-lp/echo-agent-sdk/blob/0ebe15f66d0aa57c7b7a92ae68cd1be106114ae9/docs/sdk/governance/evidence/semantic-source-provenance/SHA256SUMS.json)已归档。未来实际被模型引用的提交也须在最终 squash 前保同名完整 SHA 标签；squash 后不能依赖开发提交仍是 main 祖先，当前工作树或文件 hash 不能替代 Git 原件。

## 后继正式引用的耐久保存（2026-10-17）

原固定 c0bc 校验器的 `source_reference_observations` 识别出8个正式对象、11个观察组、42个源码引用依赖下列三个后继提交。它们的同名完整 SHA 证据标签在本地和远端均确认不存在后才创建；显式 atomic push 仅推送这三个 annotated refs，禁用自动跟随其它标签，没有覆盖或 force。

| peeled commit（标签后缀） | annotated tag object | tree |
| --- | --- | --- |
| `335193791c70a6cb077033c3f8f0cb1223e5266e` | `bf8eeaa51d96145167c753db10fb9b07debb6f79` | `35b845562b5a5ec8ed2b4b814a37bb8b386b0105` |
| `404d447f1a69fb4af2843272005b894063888ed9` | `5e1b95237d954180a7298899175a539ac7c7655f` | `e5c87574649d2879f7a31a629a56accece8cc95d` |
| `637a308e0d1c7f13339d279eb1f91aedfb403685` | `926b5999f6458f710899e4c660f01f27939b7dec` | `3780487a0265cd5743d85e6635b35571f4baf908` |

2026-10-16 21:09 UTC 的全新空 bare 仓库仅从 SDK 远端显式取回 `main@30d7e219cc7d3aef0f75a19217c82898c363f50d` 和这三个证据 tags；无 alternates、task branch、PR refs 或本地对象导入。实际 fetch 退出0，远端及冷仓的 tag object、peeled commit、tree 均精确相同；8个正式对象的历史内容和42个源码引用逐字节核验一致。完整 observations、push/fetch 原件和逐项摘要保存在 ignored `.echo-semantic/local/sdk-parallel-delivery-01a14385/`。

这次仅证明历史材料可恢复；旧 main 没有当前候选的新材料，没有在旧 main 上运行 c0bc 冒充最终 SDK 验证。当前最终候选仍须绑定其精确 Git 树完成 strict 检查；实际 squash main 后，须再次仅从新 main 和显式证据 refs 冷取回，复核完整源码引用与 strict 结果。后续新引用按同一规则逐项保存，不为所有开发检查点无差别打标签。

### 原formal Task source 的后继持久身份

原Task Evidence/Audit的source摘要31483b056427dc3f763899b798d84992cc0301f6bfe1816ccc8acf2952bee11d经固定canonical算法确认，精确等于公开Git `fa62f3b3d0df3ed5b14c6d9786dfffce5db57da6`；仅将header无损改绑为该Git，原F988/Host14a五例正文、结果和限制不变。该正式引用所需的单一annotated tag `evidence/sdk-governance/fa62f3b3d0df3ed5b14c6d9786dfffce5db57da6` 已明确推送，tag object `a3442d74073b0568d9b5404abf35905bdc2fc59b`、tree `019301c3a2fb823adcf2c989515866481bfd1553`。

2026-10-16 22:22 UTC，新空bare仓仅从远端显式fetch main30d7和此一个tag，actual0、无alternates/taskbranch/PR或本地对象导入；原两正式对象、四Task源码及任务说明共7blob逐字一致、0mismatch。完整回执只保ignored任务local/fa62-evidence-tag-cold-receipt.json。这只补历史可恢复性；不把旧988/14a重标为6563/ac433，不替代最终新main及全部必要refs的冷strict验证。

## 取回与复核

本地 `refs/echo-semantic/snapshots` 不会自动随常规 checkout 进入 CI。上述八标签现已实际远端取回验证；其它核验者在 SDK 仓库执行：

```bash
git fetch origin --tags
snapshot_commit=56929154c1851473e9e4837901b610cecf001b2d
git rev-parse "refs/tags/evidence/sdk-governance/$snapshot_commit^{commit}"
git rev-parse "refs/tags/evidence/sdk-governance/$snapshot_commit^{tree}"
git show "$snapshot_commit:.echo-semantic/baseline.md"
```

结果须与上表的 commit/tree 及原收据字节一致。当前 `rust-ci.yml` 的 Linux 测试 job 和 `semantic` job 均显式设置 `fetch-depth: 0`；执行历史源码引用校验的 `semantic` job 还设置 `fetch-tags: true`。`sdk-contract` job 不执行该语义检查，未单独声明全历史设置。任何执行旧语义版本核验的入口都应明确取回证据 tags；本说明不修改 CI，具体可恢复结论只由对应远端冷取回原件证明。

Git 官方说明 annotated tag 保存标签元数据；默认 fetch 只自动取与正在抓取历史相关的 tags，显式 `--tags` 可取全部远端标签。GC 保留 branches/tags 及其它 refs 所引用的对象，所以真实证据 tag 保留 commit/tree，而非复制一套镜像树。[git-tag](https://github.com/git/git/blob/master/Documentation/git-tag.adoc)、[git-fetch](https://github.com/git/git/blob/master/Documentation/git-fetch.adoc)、[git-gc](https://github.com/git/git/blob/master/Documentation/git-gc.adoc)于2026-10-14从官方 Git 源仓 raw 文本实际读取，均 HTTP 200。

基础 Capability/Behavior 模型和客户端审计的材料闭包须分别审查后提交；inventory/behavior closure 保持 open。具体已解决缺陷保其有限三链，不因基础材料可恢复而升级所有能力；12项 source/kind 和5秒物理结算选择、完整生成及 SDK main 验收继续独立。

## 2026-10-19 新增Finding发现版本的必要exact引用

原source/reader判定这三个实际discovered_at为必要历史引用，沿每个真实版本同名完整SHA约定保存；不为所有开发检查点或私有candidate打tag。Root独占refs操作，显式atomic push仅这三个annotated标签且nofollowTags、actual0，无force/覆盖。

| peeled commit（标签后缀） | annotated tag object | tree |
| --- | --- | --- |
| `e31e39a5ab29c4b3dc24b2fc349515a2787c6a23` | `f89cccdf7d88aab7f2d5a2e8830f2ea495313e99` | `758c6561464107295d7c99e9d20f3ff0096f2418` |
| `88a573b7dbe2cd4878427ea35eb43322dbf11089` | `584323808743a7ef0f5b02742aace6ea8eaaa051` | `6a2e131d1117c44acfbc495dbdbf0fd876b48902` |
| `4f8382fda176415749a625063a946b21cae15f5d` | `df1b02b2403b471e84f7c82cee14799bfcc2f361` | `d57ee3276f2903a480f25bad0532f90d931c71c1` |

Root的新空bare仅从远端exact fetch main30d7与三个必要tags，actual0；无alternates/taskbranch/PR refs或本地对象导入。tag/commit/tree与12个具名source/provenance blobs（每版本的SourceOps、factory、CoreState及本provenance）逐字一致。完整push/冷取回原件保在协调ignored任务local的sdk-three-discovery-evidence-refs-proof.json（10691B / d19bd5000d863754cdf5a52ce4759bc52d5952f336fddda24657dc76a2efd441）及sdk-discovery-refs-cold-proof.json（13341B / 9078a656f29c0a4b13728866e9d6479f2591299268fc1df88c10f89ce9e86dc6）。这是历史发现源可恢复性；不是新main、当前Host、原生Hook或最终strict行为验收。busy读取Finding实际discovery也是4f，不额外给e924或私有9a/8f打tag。最终精main加全部必要refs冷strict仍独立。
