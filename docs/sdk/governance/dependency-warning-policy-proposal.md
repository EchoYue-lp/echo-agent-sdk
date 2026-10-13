# SDK 依赖重复版本警告规则裁决提案

Status: proposed

尚未接受，也未修改 AGENTS.md、deny.toml、audit 配置、依赖或锁文件。本提案准备可审阅的具体规则差异，由用户选择后才执行；现有工具 exit0 不能越过仓库零警告约束宣称交付门禁通过。

## 已确认的冲突

SDK AGENTS.md 第72–73行要求：

> 开发提交执行 focused 验证；合并前执行全部适用 Rust、contract、Host E2E、三语言、cross-platform、dependency 和文档门禁，所有命令必须零失败、零警告、零格式差异。

但同仓 deny.toml 的既有 `[bans] multiple-versions = "warn"` 会在正常受支持的依赖图上产生重复版本提示。真实 `cargo deny --workspace --all-features --locked check` 返回0，报告50条 `warning[duplicate]`；配置、advisory、license、source 其它警告均0。真实 `cargo audit --deny warnings` 返回0、无警告。因而“两个工具退出0”与“所有门禁零警告”是不同结论，现状尚有明确规则冲突，不能由助手自行解释豁免。

证据来自 [当前依赖门禁收据](evidence/sdk-current-dependency-policy/README.md)、[完整 deny 输出](evidence/sdk-current-dependency-policy/deny.log)、[原 policy 结果](evidence/sdk-current-dependency-policy/policy-result.json) 和两个工具的独立 run-command 终态。原四项 advisory exceptions 原样保留，不能把本提案理解为新增漏洞、维护状态、来源或许可证豁免。

## 实际50组及版本分类

绑定的 Cargo.lock SHA-256：`a42cfbc993a3eabb714a6b221472600d424fa411226659458b7286d490f7e837`。

绑定的 deny.toml SHA-256：`b18307d3e0afa05d0c1ad32004b59cb0a7fe62136e90037bec17738545b3013f`；原 deny.log SHA-256：`14aeb79543361a51ee5b2b0c298d2fbc150c068275597eeb831d9382238476cd`。

下面每组版本逐条取自此次 deny 输出的实际 lock entries，并核对都存在于 Cargo.lock。不能仅列锁文件全版本：例如 schemars 的锁文件还包含0.9.0，但本次 all-features workspace 检查选择的重复组只有0.8.22与1.2.2，未把未选版本加入基线。

50组中10组跨数字 major，40组数字 major 都是0但跨0.x minor兼容线，没有同一兼容线内的纯补丁重复组。这里“同 major”仅是版本号分类，不表示可以安全互换；0.x minor变化通常属于不同兼容线，具体升级还需按原消费者约束验证。

| crate | 本次检查实际版本 | 分类 |
| --- | --- | --- |
| block-buffer | 0.10.4, 0.12.1 | 同 major（跨 0.x minor 兼容线） |
| const-oid | 0.9.6, 0.10.2 | 同 major（跨 0.x minor 兼容线） |
| core-foundation | 0.9.4, 0.10.1 | 同 major（跨 0.x minor 兼容线） |
| cpufeatures | 0.2.17, 0.3.0 | 同 major（跨 0.x minor 兼容线） |
| crypto-common | 0.1.7, 0.2.2 | 同 major（跨 0.x minor 兼容线） |
| digest | 0.10.7, 0.11.3 | 同 major（跨 0.x minor 兼容线） |
| foldhash | 0.1.5, 0.2.0 | 同 major（跨 0.x minor 兼容线） |
| getrandom | 0.2.17, 0.3.4, 0.4.3 | 同 major（跨 0.x minor 兼容线） |
| hashbrown | 0.12.3, 0.14.5, 0.15.5, 0.16.1, 0.17.1 | 同 major（跨 0.x minor 兼容线） |
| hashlink | 0.9.1, 0.10.0 | 同 major（跨 0.x minor 兼容线） |
| html5ever | 0.31.0, 0.39.0 | 同 major（跨 0.x minor 兼容线） |
| indexmap | 1.9.3, 2.14.0 | 跨 major |
| markup5ever | 0.16.2, 0.39.0 | 同 major（跨 0.x minor 兼容线） |
| md-5 | 0.10.6, 0.11.0 | 同 major（跨 0.x minor 兼容线） |
| nom | 7.1.3, 8.0.0 | 跨 major |
| phf | 0.11.3, 0.12.1, 0.13.1 | 同 major（跨 0.x minor 兼容线） |
| phf_codegen | 0.11.3, 0.13.1 | 同 major（跨 0.x minor 兼容线） |
| phf_generator | 0.11.3, 0.13.1 | 同 major（跨 0.x minor 兼容线） |
| phf_shared | 0.11.3, 0.12.1, 0.13.1 | 同 major（跨 0.x minor 兼容线） |
| quick-xml | 0.39.4, 0.41.0 | 同 major（跨 0.x minor 兼容线） |
| r-efi | 5.3.0, 6.0.0 | 跨 major |
| rand | 0.8.7, 0.9.5, 0.10.2 | 同 major（跨 0.x minor 兼容线） |
| rand_chacha | 0.3.1, 0.9.0 | 同 major（跨 0.x minor 兼容线） |
| rand_core | 0.6.4, 0.9.5, 0.10.1 | 同 major（跨 0.x minor 兼容线） |
| redox_syscall | 0.5.18, 0.7.5 | 同 major（跨 0.x minor 兼容线） |
| schemars | 0.8.22, 1.2.2 | 跨 major |
| schemars_derive | 0.8.22, 1.2.2 | 跨 major |
| serde_derive_internals | 0.29.1, 0.30.0 | 同 major（跨 0.x minor 兼容线） |
| sha2 | 0.10.9, 0.11.0 | 同 major（跨 0.x minor 兼容线） |
| shlex | 1.3.0, 2.0.1 | 跨 major |
| socket2 | 0.5.10, 0.6.5 | 同 major（跨 0.x minor 兼容线） |
| string_cache | 0.8.9, 0.9.0 | 同 major（跨 0.x minor 兼容线） |
| string_cache_codegen | 0.5.4, 0.6.1 | 同 major（跨 0.x minor 兼容线） |
| syn | 2.0.119, 3.0.3 | 跨 major |
| tendril | 0.4.3, 0.5.1 | 同 major（跨 0.x minor 兼容线） |
| thiserror | 1.0.69, 2.0.20 | 跨 major |
| thiserror-impl | 1.0.69, 2.0.20 | 跨 major |
| tower | 0.4.13, 0.5.3 | 同 major（跨 0.x minor 兼容线） |
| web_atoms | 0.1.3, 0.2.6 | 同 major（跨 0.x minor 兼容线） |
| weezl | 0.1.12, 0.2.1 | 同 major（跨 0.x minor 兼容线） |
| windows-sys | 0.48.0, 0.52.0, 0.61.2 | 同 major（跨 0.x minor 兼容线） |
| windows-targets | 0.48.5, 0.52.6 | 同 major（跨 0.x minor 兼容线） |
| windows_aarch64_gnullvm | 0.48.5, 0.52.6 | 同 major（跨 0.x minor 兼容线） |
| windows_aarch64_msvc | 0.48.5, 0.52.6 | 同 major（跨 0.x minor 兼容线） |
| windows_i686_gnu | 0.48.5, 0.52.6 | 同 major（跨 0.x minor 兼容线） |
| windows_i686_msvc | 0.48.5, 0.52.6 | 同 major（跨 0.x minor 兼容线） |
| windows_x86_64_gnu | 0.48.5, 0.52.6 | 同 major（跨 0.x minor 兼容线） |
| windows_x86_64_gnullvm | 0.48.5, 0.52.6 | 同 major（跨 0.x minor 兼容线） |
| windows_x86_64_msvc | 0.48.5, 0.52.6 | 同 major（跨 0.x minor 兼容线） |
| zip | 2.4.2, 8.6.0 | 跨 major |

## 推荐选择：仅允许明确受控的 cargo-deny duplicate 提示

保留现有 `multiple-versions = "warn"` 工具策略，明确只有此工具的 `warning[duplicate]` 可以作为受控、非阻断提示。所有原命令仍必须真实退出0；audit/advisory、license、source、配置、Rust/TypeScript编译及其它命令的警告仍必须为0，测试仍零失败、零跳过豁免、零格式差异。本选择不新增 ignore，不隐藏输出，不降低任何非 duplicate 阈值。

这不是对未来所有 duplicate 的无限许可。首次接受仅绑定本表50组、精确版本集合、上述锁文件摘要和完整原始输出；未来锁图、framework revision、检查 feature 范围或策略改变，必须重新运行原命令并复审版本组变化。即便警告条数仍为50，版本或组身份变化也不能自动沿用；数量增加也不得自动更新上限或基线。新 category、新来源/许可/advisory/配置警告须按原要求修复。原始日志、分类、退出码与复审结果必须一并归档，不能用汇总“pass”替代50条实际提示。

接受后 SDK AGENTS.md 第72–73行上引整句的**候选替换文本**为：

> 开发提交执行 focused 验证；合并前执行全部适用 Rust、contract、Host E2E、三语言、cross-platform、dependency 和文档门禁，所有命令必须真实退出0、零失败、零格式差异。编译、audit、advisory、license、source、配置及其它工具警告必须为0；唯一例外是 cargo-deny 原有 multiple-versions = "warn" 产生的 warning[duplicate]，仅在完整原始输出保留、版本组按已接受的精确 Cargo.lock 摘要基线复核且无未经复审的增量时，作为非阻断提示。锁图、framework revision、检查范围或策略变化必须重新验证与复审，不得自动扩大基线或推广为其它警告豁免。

此处是待裁决的文稿，**当前并未替换该句**。不改其余“失败必须修复”、分支、提交、合并规则。

## 另一选择：维持严格零 duplicate 警告

保留 SDK AGENTS.md 原句，并在合并前消除50组 duplicate。需要沿原父依赖约束逐组寻找兼容整合路径，再验证框架精确revision、SDK构建/feature矩阵和语言行为。表中10组跨major及40组跨0.x minor不宜以一次cargo update或删除功能直接合并，不能伪造结果、追加ignore或把warn输出关掉来满足零告警。没有在本提案中推导升级工期，也未断言任何具体组永远无法整合。

此路线可能需要上游依赖升级及适配，但不允许把全部 Rust public identity 强行扩大成三语言 API，也不改变已接受的产品合同边界。若需要改变用户承诺或新增依赖策略，必须另行形成具体可审阅选择。

## 顶层与子仓库规范归属

已读取实际 `/Users/ls/MyWork/code/ylp_agent_learn/lp-agent/AGENTS.md`。顶层第249行要求所有已执行命令零失败/零报错，第254行针对编译警告全部消除；没有与SDK第73行完全相同的“所有dependency工具提示零警告”句子。顶层仍要求与子仓库规则同步，因此接受推荐后，只需在顶层验证规则中补充SDK专属归属说明，**候选新增句**为：

> echo-agent-sdk 的依赖检查只按其独立 AGENTS.md 中经用户接受、绑定精确 Cargo.lock 版本组基线的 cargo-deny warning[duplicate] 特例执行；该特例不适用于 echo-agent、echo-agent-cli、编译警告或其它工具类别，原失败必须修复及提交/分支门禁保持不变。

顶层当前也未修改。正式SDK规则与依赖证据归独立SDK；若接受，SDK AGENTS.md 和顶层这条协作说明应在各自仓库同步交付。framework/CLI/website 不据此改变其依赖政策或发布边界。

## 裁决与实施边界

推荐选项与严格零警告选项互斥。当前20文件依赖证据包及其摘要未修改；这个 proposed 文档不会让Finding或仓库门禁自动闭合。用户裁决后只执行所选规则文本与必要同步/验证，不额外改锁图或生成合同。框架新revision及SDK最终main验收仍需各自精确快照证据，npm/PyPI/Maven发布继续在本项目交付范围之外。
