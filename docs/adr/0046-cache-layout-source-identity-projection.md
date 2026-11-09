# ADR 0046：缓存布局来源身份迁移与既有 SDK 字段投影

## 状态

已批准

## 审批范围

2026-11-09，Root 核准精确提案 `33e483194a3eeff559ee311dc6b822439fb553da78aca31b16519a051080ba97`
及本 ADR 的四条 `struct_field` 来源处置；独立审查为 PASS，未发现阻断或重要问题。
批准仅覆盖下列两旧来源退役、两新来源接受，保留三语公开字段、构造参数及 Host JSON key。
其它身份增删须停止并重新核对。本批准不接受新锁或 quick-xml 风险；accepted 产物仍待最终
Framework main 精确 pin 后用原唯一完整生成器更新和检查。

## 背景与来源

开发依赖为 Framework `4703f0c2d5993ca23c037e24535686d51e90e3bb`。该版本将
`SegmentRanges`、`PromptCacheLayout` 的 Rust 字段 `runtime_context` 改为 `dynamic`，
并使用严格 v2 Tail 投影信封识别连续动态尾段。裸 `[runtime_context:`、Boundary、
v1 和历史中间的旧版本不构成动态尾段。`canonical` 始终是独立分段。

SDK `43dd586e3df773fb2bf7dbd549cb558a6be29b5e` 的 accepted 合同有 5344 个身份，
其中两项仍指向旧字段。它们的 route 均为 `value`，method、operation 均为空。
`apply_accepted_source_scope` 的身份键为精确来源与 `ItemKind`；现有 reexport
`alias_of` 不表达任意改名。不得把已经消失的 Rust 符号记为仍然存在。

依据为本仓 ADR0034 的来源迁移流程、唯一 `export_schema::acceptance_delta`、
`inventory::apply_accepted_source_scope`，以及冻结上游的 `layout.rs` 和
`compression.rs`。完整新 inventory 尚未生成；其 canonical path、签名和 route
须由原唯一生成器验证，不能用源码阅读冒充生成通过。

## 候选方案

保留已经消失的 Rust 来源名会让 accepted 合同与真实 inventory 不符；扩展通用 alias
机制会扩大此次四条来源处置的范围。直接把三语公开字段改成 dynamic 则会改变既有调用方。
因此采用精确退役两条旧来源、接受两条新来源，并用现有薄投影保留三语字段的方案。

## 决策与取舍

保留 TypeScript／Java 的 `runtimeContext`、Python 的 `runtime_context`、
`SegmentRanges` 构造参数和 Host `cacheHints.segments.runtime_context` JSON key。
它们投影上游真实 `dynamic`，不添加三语 `dynamic` 公开别名，不修改 Framework。
三语 `PromptCacheLayout` 只做只读值投影；缓存存储、生命周期和 provider 断点仍归框架。

仅按下列四条记录退役两个旧来源、接受两个新来源；既有三语能力和字段不退役。
不新增通用 alias、registry、协议字段或 schema。原唯一完整 `--update` 在精条目
批准并独立审查后执行，后继原 `--check` 仍必须通过；禁止手改 accepted JSON。

三语算法遵循上游 v2 信封：精确前缀、独立空行终止头、非空 JSON 字符串 marker／version、
`placement=tail`、无额外头字段；只取 system 区域之后的末尾连续块。任何 `system` 消息，
包括普通历史之后携带合法 Tail 的 `system` 消息，都不得进入动态段。正文不会提供头部
元数据。消息和 tools 保持原顺序，输入不变，旧字段名对应的范围与消息切片逐项一致。

## 影响范围

影响限于两条 Rust 字段来源身份及其 SDK 值投影、合同生成检查和边界测试。语言构造参数、
公开字段、Host JSON key、缓存存储及生命周期权威保持原定义；不新增协议或通用别名。
两旧来源与两新来源须由同一次原生成入口处理，不能只修改某一份 seed 或生成物。

## 验收与交付

先保留旧实现的负控失败，再用相同样本核对三语分段、范围和原输入。覆盖空输入、
全部 system、canonical、两条合法 Tail、裸前缀、v1、Boundary、缺版本、非法 JSON、
额外头行、未终止头、正文伪造 version，以及 Tail 后有普通历史的场景。
完整新 Host、三语、原唯一生成器和依赖门禁待最终 Framework main pin 后分别验收。
开发 focused check 通过不代表这些检查完成；新锁和 quick-xml 风险未获本 ADR 接受。

4703 开发切点的首轮三语分段定向用例各 31 项通过，两文件 Python Ruff 检查及格式检查通过。
Java 的第一份多模态夹具曾将 JsonNode 误当作原生 Map；修正为原编码器支持的 Map/List
并加入字段往返自检后，已在旧算法取得 10 项实际失败，再在当前算法取得 31 项通过。
恢复旧源码时曾因保留旧 mtime 而复用绿色 class；该次退出 0 未计入旧算法资格，原件保留，
通过刷新源码 mtime、原命令重编译及 bytecode 检查闭合。没有修改 Maven flags 或预算。

在同树的 main 01da 后继独审中，合法多模态 system 消息的 canonical 文字视图缺口已复现。
其修复仅调整三语内部文字视图：Text 无分隔拼接，ResourceLink 加入原前缀和具名字段的
JSON 文字视图，忽略其它媒体；动态段继续只读取首个 Text part 的完整信封。
新增边界用例分别取得 13 项实际失败，修复后各 54 项通过，旧 31 项断言保留。
这不改变本 ADR 的四条来源处置、语言公开字段或 Host wire。

本次不改变模型工具或示例调用；保留 API 的说明由本 ADR 与 SDK 开发状态文档承载。
官网属于其它独立仓库，本窗口不写入；不能发布未验收或仅开发 pin 的行为。

## 保留行事实更新

Root 于本次完整生成后单独核准已存在的
`echo_core::compression::ContextProjection`（`struct`）保留行：仅 `derived_traits`
由 `[Clone, Debug]` 增加 `Default`；移除此列后整行完全相同，签名、route、语言映射及其它列不变。
完整行摘要从 `221e3185b395fa7a97bc7d58d8d499ac721587bb7029d37c9e96f3d6d1e3970b`
变为 `3331449588012d29a05e8a8a0c8b3d4adccd947ddad496e40af661342200638b`。

原连续性检查器只在精确保留行表中追加这一对摘要（13 行变 14 行）。这不增加
`Default::default` 来源身份、三语公开 API 或任何能力退役；本 ADR 的接受／退役处置仍严格四条。
旧两字段构造继续经原 `replacing`，保持 Boundary／version=None。其它保留行漂移仍拒绝。

## retire-prompt-layout-runtime-context

- Action: retire
- Kind: struct_field
- Source: echo_core::llm::cache::layout::PromptCacheLayout::runtime_context
- Reason: 冻结上游已将定义字段改名为 dynamic；旧具名来源不存在。
- Compatibility: 保留三语 runtimeContext／runtime_context 及只读布局能力，按真实 dynamic 投影。
- Rollback: 只整体恢复同一旧 framework pin、锁、accepted 及产品算法并重新验收，不单独恢复旧 seed。
- Replacement: echo_core::llm::cache::layout::PromptCacheLayout::dynamic

## accept-prompt-layout-dynamic

- Action: accept
- Kind: struct_field
- Source: echo_core::llm::cache::layout::PromptCacheLayout::dynamic
- Reason: 以现有框架定义的真实动态尾段字段承接既有只读布局能力。
- Compatibility: 仅来源身份替换；三语旧公开字段不改名，不新增 RPC 或 dynamic 公开字段。
- Rollback: 只整体恢复同一旧 framework pin、锁、accepted 及产品算法并重新验收。

## retire-segment-ranges-runtime-context

- Action: retire
- Kind: struct_field
- Source: echo_core::llm::cache::layout::SegmentRanges::runtime_context
- Reason: 冻结上游已将定义字段改名为 dynamic；旧具名来源不存在。
- Compatibility: 保留三语旧字段和构造参数及 Host runtime_context JSON key，canonical 独立。
- Rollback: 只整体恢复同一旧 framework pin、锁、accepted 及产品算法并重新验收，不单独恢复旧 seed。
- Replacement: echo_core::llm::cache::layout::SegmentRanges::dynamic

## accept-segment-ranges-dynamic

- Action: accept
- Kind: struct_field
- Source: echo_core::llm::cache::layout::SegmentRanges::dynamic
- Reason: 以现有框架定义的真实动态尾段范围承接既有范围投影。
- Compatibility: 仅来源身份替换；三语旧字段和 Host JSON key 投影 dynamic，不混入 canonical。
- Rollback: 只整体恢复同一旧 framework pin、锁、accepted 及产品算法并重新验收。
