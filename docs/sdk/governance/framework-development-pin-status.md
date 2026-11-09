# Framework main 精确 pin 与开发验证边界

## 当前状态（2026-11-09）

当前 Host 的唯一精确 Git 来源为 Framework main `3413d64dae23353c6d02c546ac374deded855827`，
tree `82a242196acd66a92cf0ad3a8708fa2e14e83117`。SDK 锁 SHA-256 为
`95e3caabaf5e72fc71ad1d6b79964eae1883b297f0006667e39c4c3a6eb8c5e5`，共 654 包；
完整 locked metadata 的 13 个 Git 包来自同一 HTTPS revision，独立 Cargo checkout 的 HEAD 精确匹配。

[ADR0046](../../adr/0046-cache-layout-source-identity-projection.md) 已批准并由原唯一完整生成器落实
四条 `struct_field` 来源处置：两项 `runtime_context` 退役、两项对应 `dynamic` 接受。
accepted 仍为 5344 项身份／430 条路由；保留行 `ContextProjection` 只增加派生 `Default`，
不接受 `Default::default` 身份。TypeScript／Java 的 `runtimeContext`、Python 的 `runtime_context`、
构造参数及 Host `cacheHints.segments.runtime_context` 保留原名，映射真实 Rust `dynamic`。
canonical 独立；动态段只取严格 v2 Tail 信封的末尾连续块，任何 system 消息都不进入动态段。

本次实际验证：

- 原唯一完整生成器完成 26 个 profile／136 个产物；后继文档链接修正仅改变一处 schema description，
  已由原 producer 再生成，完整合同 `--check`、来源连续性、语言导出与遥测检查均退出 0。
- 本机适用的完整 Rust／Host 矩阵、Clippy、示例与 rustdoc 均在最终有效轮退出 0、零编译警告。
  首次工具组件缺失、LLVM 加载警告和 rustdoc 警告的原始轮次保留，不冒充零警告结果。
- 原 `scripts/check-language-sdks.sh` 完整入口 `L1-original-full-languages-r04` 自然退出 0、零警告：
  TypeScript 528 项中 527 项通过、1 项原 Windows 原生跳过；Python 601 项通过、1 项同类跳过，
  另有 18 个子测试通过；Java 400 项测试零失败、零错误、1 项同类跳过，86 份 fresh XML；
  三个 quickstart 均到达并成功。完整语言入口绑定真实 Host `91ac95d2…`／collector `997305e7…`，
  1306 个 primary 输入、781 个最小视图输入及工具／policy 前后无漂移。
- SDK 的本次精确风险接受已由用户独立批准：仅限 3413／锁95／654包，原 107 项 duplicate 身份
  加唯一 `quick-xml 0.42.0`，为 49 组／108 项。原 14 项控制及同一真实 scanner 输出的正式 checker
  已由协调会话执行通过；原四项 audit ignore、deny 策略和 parser 不变。精确政策见
  [依赖裁决](dependency-warning-policy-proposal.md)，不继承 Framework／CLI 的风险接受。

本次记录时，固定 c0bc 原全树 strict Source／Final 尚未执行，精确 SDK main CI 尚未完成；
本机结果不代替 Windows／Linux 原生、任意真实 provider 或发布验收。原 SDK main 的历史绿票
及本次 Framework main CI 不替代这些 SDK 门禁。

原始工程回执保存在主根 `.echo-semantic/local/integrated-acceptance-20261108-r01/`，
SDK 验证目录为 `sdk-main-validation-r01/published-main-3413-full-r01/`：
`L1-original-full-languages-r04/receipt.json`、`L1-original-full-languages-r04/qualification-with-runtime-metadata.json`
及 `gap-ack-test-invariants-fix-r01/final-handoff.json` 分别记录原入口、明细和最终交接。
完整语言原始日志 SHA-256 为 `18790f51062fa83108eac28f49ba9258396e27e68b2133f56627000af0520cd5`。
行为与 API 权威仍在本仓合同和 ADR；上述主根材料只保存阶段验证证据。

## 分阶段历史记录

以下保留各当时切点的来源、失败与未完成状态，不作为本页当前验证状态。

当前 SDK 已通过正常 Cargo 解析切到 Framework 已发布 main
`3413d64dae23353c6d02c546ac374deded855827`。实际完整 locked metadata 的 13 个 Git 包
均来自同一 HTTPS canonical source 与独立 Cargo checkout，HEAD 精确匹配且跟踪源码干净。
此前 main `01da9f44f98bb08eba5bd0704b726c1c2517bb19` 与开发 pin 4703 的完整树相同；
3413 的树为 `82a242196acd66a92cf0ad3a8708fa2e14e83117`，后继只修改上游一处测试及注释。
ADR0046 引用的布局与信封解析定义哈希保持不变。

新锁为 `95e3caabaf5e72fc71ad1d6b79964eae1883b297f0006667e39c4c3a6eb8c5e5`，
共 654 包。与此前 main01da 的锁 `bec16bc8bd12e2dbe468409e1c4a320cd6fb56b0bc9cbdcc7be9b600cc690c6b`
相比仅 13 个 Git 包的精确 revision 变化；639 个 registry 包的完整记录、校验和及依赖边不变。
此前 01da 的来源预检和六项本地准入控制已通过；3413 的完整 metadata 来源核对已通过。
完整生成与 SDK 验证窗口已由 Root 开启，各步骤须按其实际结果结算。
Framework 精确 main CI 由 Root 独立核验；SDK 全部适用门禁及精确 main CI 仍须执行。

原 340/c448 下 canonical-origin fresh audit 命中 calamine 0.36.1 的 RUSTSEC-2026-0436，
原 actual0／canonical1／deny0 的独立原件和历史全部保留。本次通过正常 Cargo 解析 Git pin 和必要依赖，
使用最终代码源内的 calamine 0.38.0；不以 registry patch 混合来源，也不把版本变化本身当作安全门禁通过。

本轮保留 SDK 既有外部 accepted 范围和七字段 custom-agent LlmUsage wire：旧输入明确对应
`observation=None`，typed helper 对不可表达的 Some 明确报错；真实 live 事件仍沿原 EventEnvelope／
WireValue 保留观察字段。旧两字段 ContextProjection 使用原 replacing 构造，保持 Boundary／version=None。
新增字段与三语言消费者断言不构成新 API 授权，实际运行资格须由后续原完整入口给出。

原 SDK audit 四项 ignore、deny 策略和 duplicate 精确基线保持原样。新的锁或 duplicate 身份没有自动接受，
不能继承 Framework／CLI 风险决定；真实最终锁扫描与 Review 后，由 Root 处理必要精确裁决。

此前 4703 开发窗已执行 `cargo check --workspace --all-targets --all-features --locked`：首次因上游
`SegmentRanges::runtime_context` 已改名而真实退出 101；Host 保留旧 JSON key、取值改为真实
`dynamic` 后，同条复验退出 0、零编译警告，1305 个源码输入及六个工具前后无漂移。
新增 Host 分段控制仅已通过类型检查，尚未执行；首次失败原件不覆盖。

后继缓存布局适配见 [ADR0046](../../adr/0046-cache-layout-source-identity-projection.md)：
Root 已精确批准两旧来源退役与两新来源接受，保三语旧公开字段、构造参数及 Host JSON key。
三语算法已按严格 v2 Tail 对齐；连续尾段遇到任意 system 消息即停止，包括历史之后携带合法
Tail 的 system。此前三语分段定向用例各 31 项通过；后继独审发现合法多模态 system 内容的
canonical 文字视图遗漏，已按 `MessageContent::as_text` 聚合 Text 与 ResourceLink，并保留
动态段只能读取首个 Text part 的独立判据。新增用例在三语中分别取得 13 项实际失败后，
当前各 54 项通过；Python 两文件 Ruff 检查与格式检查通过。原 31 项用例完整保留。
Java 夹具修复和 mtime 导致复用旧 class 的验证问题均保留原件，并以实际旧算法 RED 和当前
算法 GREEN 分别重编译核对。以上仅为 intrinsic 定向资格，不代表真实 Host 或完整三语言通过。

原唯一完整生成器已在 3413 上实际执行成功，完成全部 26 个 profile，写入 136 个既有产物，
其中六份文件内容实际变化。accepted 身份仍为 5344 项，增退严格为批准的两旧来源与两新来源；
schema 字节不变。唯一额外保留行差异是 ContextProjection 派生元数据增加 Default，已由 Root
具名核准并记录于 ADR0046，不增加 Default::default 身份或公开 API。

首次完整生成因冻结 nightly 缺 ARM 标准库而真实退出 1；从同版本、同编译器哈希的本机完整
安装恢复缺失组件后，标准库预检 0，同一完整生成命令复验 0。旧失败及两版工具记录分开保留。
原精确来源连续性检查器按实际生成行绑定 13 项具名退役、五项具名接受及 14 项保留行变化。
完整窗口继续按原 --check、Rust、Host 和三语入口执行，各步骤按实际结果记录，
依赖风险精确接受及实际 main CI 仍由 Root 独立收口。

实际结果独立保存在主根工程回执；不代表完整生成、Host／三语言运行、跨平台或发布门禁已通过。
正式 pin、生成合同和完整 gate 按最终 main 身份另行收口。SDK 生命周期、取消、guard、终态及用户数据边界
继续沿原权威，不新增 SDK runtime 或远端资源支持声明。
