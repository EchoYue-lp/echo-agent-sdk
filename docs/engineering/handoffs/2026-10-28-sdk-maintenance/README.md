# SDK 维护保存点：待采用工程包

时间：2026-10-28T17:59:53.492232+08:00。用户明确把当前优先级改为保存、提交和清理，暂停继续迭代；原 Goal 和成功标准不取消。

本目录只保全待采用 patch、来源/审查/剩余命令和正确的 NoCompat 分类，不参与产品编译。
Root 统一 stage/commit，本 SID 没有提交、push、merge、改 pin 或占位锁的授权。
唯一实时交回入口为主根 `.echo-semantic/local/parallel-executors-20261027/sdk/handoff.json`；
本目录是可单独保存到 Git 的工程资料，不能把 pending patch 当生产源码已经采用。

## 当前仓库与精确来源

- Git 根：`/Users/ls/MyWork/code/ylp_agent_learn/lp-agent/echo-agent-sdk`；分支 `chore/Echoyue/root-workspace-convergence`，HEAD `525453fc9b9448358cc8a2f46bbcf401b10943ec`，index 空。
- 保存包新增前实际155个dirty路径：114个tracked差异、41个untracked、其中29删除。完整路径与SHA见manifest。
- 正式 Host pin 保持 `f7bac62eee04a963f92a3e4308b6ea2336e91cc0`，正式锁SHA `0dacbb873673a405fed4e820499ff33e40301cefc5905e8c2bc597558156231f`。
- 原受控组合锁 `4a99acce12f0b51975fb88c2b24a94d67c6695dbf46ed35401dec7139948fd2b` 是Framework1490路径组合，不是正式新Git来源；不能拿它替换正式锁制造资格。
- 已正式采用SDK1的一处LSP投影源 afterSHA `8c1a456d6045d5e2104aa321db62df83b71e1dcd36393c41d2858938c0cf14a9`；只静态apply/fmt/Review，原234/三语发生在它之前，未覆盖本处删后组合。
- SDK4四文稿已经逐base/current/candidate采用且静态独审pass。正式产品状态、运行与来源资格仍分别验收。

## 保存与提交分组

| 分组 | 当前状态 | Root提交边界 |
| --- | --- | --- |
| 本目录六份保存资料 | normal preflight新路径已0；pending patch逐字节保存，不含raw大log/binary/密钥 | 可按工程保全独立准备提交，仍须实际staged范围的正常门禁；不等于产品修复交付 |
| 原Plan04的94路径A2A/LSP消费者撤出 | 从具名旧候选接管；当前155路径已有后继叠加，不能把旧520测试票冒当前 | 保留269/23/39撤销证明与5352/431保留合同；重叠文件要按hunk对账，不机械拆commit |
| 原生Message/V6客户端/包资源/Parquet | 已实现未提交；正式f7与当前原生正控不闭合。原234和完整语言仅其受控输入 | 不宣当前正式pin全绿；真实新Git来源/唯一生成器/适用focused门禁仍缺，不用占位pin |
| 只读SearchProvider retry mapper与旧终态夹具修正 | 同源受控Host/完整语言曾通过，源码现保持；Framework仍唯一重试/结算权威 | 与原生消息共享extension_bridge/语言文件；旧二进制与原始失败保留，不继承为本次SDK1删后资格 |
| SDK1源+SDK4文稿 | 前者静态pass；后者精patch/afterSHA一致，文稿不等于删后运行 | 可独立审阅的静态切片；若合产品代码仍需精来源下必要验证，不能借文稿pass |
| rich Message override 24路径候选 | **未正式采用，最终独审未完成，Rust未编译/Host未运行** | 仅保本pending patch和manifest；当前维护cut不继续修或强行并生产 |
| NoCompat分类 | A1 delivery缺字段、A2空指纹豁免是待退旧接受；A3“FinalAnswer皆旧compat”已撤回 | runtime最终通知/回调/Guard安全正文/校验/收尾与21路径必要验收保留；尚无清理代码采用 |

## 验证必须保持各自输入

受控1490 full Host SHA `01e34d1e87ee3417474267186921f5b25bb6ac581267b410b9af2f5a407f8c1c`，
同源collector `53ac572f930c5b36f1bc2e36dced2dc7e42817d184a5d3c5939b9ea399d6a9af`。
原完整语言TS387项（386 pass/1 Windows skip）、PY462 pass/1 Windows skip/18subtests、
Java327项（82 fresh XML/1 Windows skip）及三个quickstart实际0，只覆盖原输入。
后继test-only修复后lib234实际0，保全binary `839de8c67be5001c95fbc7fdf63094d96d7bd13eb4713f2ee801fa298e903304`；
原4c9 binary lib227/7fail/exit101、旧f7拒绝合同/旧Host/其它RED原件继续保护。
当前SDK1在其后改变同一源文件，不能把旧234数字续给它，更不能续给pending rich补丁。

本pending rich候选已具Rustfmt/语法、TS类型/示例、TS/Python raw handler各1项、Python lint/format、
Java4文件release17编译的有限证据；不含Rust typecheck、真实Host/FactoryAdapter外层、schema/包资源生成或三语新Host消费者。
暂停时独审只确认原两Important已修，未审完24路径，明确无整体pass。详见review。

## 恢复与禁止事项

本patch的正式base是当前主根WIP，不是裸HEAD；Root先把对应正式WIP按正常门禁保存为可识别提交，
再逐manifest baseSHA对账才可考虑应用patch。不要直接对新的checkout覆盖或自动apply。
恢复本保存点需保留 `.git`、固定分支/tag、正式dirty源快照、原始binary/log/Evidence及下述单一handoff索引。
完整dirty恢复包只在ignored目录保留，交回索引列其SHA、成员和恢复步骤，不提交raw包/编译制品。

不得把旧Evidence的after/status/matched重绑成新结果，不把外部echo-semantic工具/source排除悄悄修掉，
不更新风险baseline/正式f7/0dac，不restart原App/fixture或恢复auth，不删用户数据。
候选参考Frame v02 manifest `be8dbdc1756a14e80f17b231cab1e288c8ce646e021834a3f554bf710be257ba` 仍只ignored syntax/apply资格。
SDK的Source/Task最终门禁、来源与完整生成缺口见单一handoff实际回执；本保存不标总Goal完成。
