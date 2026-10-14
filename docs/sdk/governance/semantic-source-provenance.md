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

原始 [fresh-fetch 收据](evidence/semantic-source-provenance/receipt.json)、[fetch 日志](evidence/semantic-source-provenance/fetch.log)及[逐字节校验](evidence/semantic-source-provenance/SHA256SUMS.json)已归档。未来实际被模型引用的提交也须在最终 squash 前保同名完整 SHA 标签；squash 后不能依赖开发提交仍是 main 祖先，当前工作树或文件 hash 不能替代 Git 原件。

## 取回与复核

本地 `refs/echo-semantic/snapshots` 不会自动随常规 checkout 进入 CI。上述八标签现已实际远端取回验证；其它核验者在 SDK 仓库执行：

```bash
git fetch origin --tags
snapshot_commit=56929154c1851473e9e4837901b610cecf001b2d
git rev-parse "refs/tags/evidence/sdk-governance/$snapshot_commit^{commit}"
git rev-parse "refs/tags/evidence/sdk-governance/$snapshot_commit^{tree}"
git show "$snapshot_commit:.echo-semantic/baseline.md"
```

结果须与上表的 commit/tree 及原收据字节一致。当前 `rust-ci.yml` 只有 Linux 测试 job 显式 `fetch-depth: 0`；`sdk-contract` job 尚未声明该设置。任何执行旧语义版本核验的入口都应明确取回证据 tags；本说明不改 CI，也不凭本地可读声称远端已经可用。

Git 官方说明 annotated tag 保存标签元数据；默认 fetch 只自动取与正在抓取历史相关的 tags，显式 `--tags` 可取全部远端标签。GC 保留 branches/tags 及其它 refs 所引用的对象，所以真实证据 tag 保留 commit/tree，而非复制一套镜像树。[git-tag](https://github.com/git/git/blob/master/Documentation/git-tag.adoc)、[git-fetch](https://github.com/git/git/blob/master/Documentation/git-fetch.adoc)、[git-gc](https://github.com/git/git/blob/master/Documentation/git-gc.adoc)于2026-10-14从官方 Git 源仓 raw 文本实际读取，均 HTTP 200。

基础 Capability/Behavior 模型和客户端审计的材料闭包须分别审查后提交；inventory/behavior closure 保持 open。具体已解决缺陷保其有限三链，不因基础材料可恢复而升级所有能力；12项 source/kind 和5秒物理结算选择、完整生成及 SDK main 验收继续独立。
