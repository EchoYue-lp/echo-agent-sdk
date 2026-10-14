# a634 精确锁图依赖门禁

## 支持的结论

新 framework pin 为 `a6346643bda47679a96760f56f45e41fd73b61b0`，Cargo.lock SHA 为 `a7a331cf94a24842e0b892d44f842e5663434009c9187b4306af7df524344bb3`。八个 framework Git 包全部采用这个精确 source；与旧 be5 锁图相比，只有这八个 source 身份变化。648 个 registry package 的完整版本、checksum、dependency 字段相同，其余锁字段也相同，无本地 patch/replace/path 替换。

原 `cargo audit --deny warnings` 首次因官方 RustSec HTTPS fetch 网络失败退出1。完整原输出和终态保留。一次有界重试仍用原命令，实际退出0，fresh fetch载入1310条安全 advisory、检查658个locked crate依赖，无警告。没有关闭 fetch、配置 proxy、添加 ignore 或改阈值。

原 `cargo deny --workspace --all-features --locked check` 实际退出0；advisories、bans、licenses、sources 四类均ok。输出仍含50条 `warning[duplicate]`，其它warning类别0。每个实际选中 crate 身份和版本数组逐项等于用户已接受的50组基线表，未扩大基线；不是只因为警告数量仍为50便认定相同。原702116字节完整输出保留，不宣称deny零警告。

## 来源与范围

三个原工具过程及完整 stdout/stderr、时间、exitCode存于run.json和各receipt。7个输入文件在执行前后SHA均相同，并保存完整文件字节。工具为cargo-audit0.22.2、cargo-deny0.20.2。

独立 `git ls-remote https://github.com/RustSec/advisory-db.git HEAD` 实际返回0、HEAD `6aed84668c23d7f535077a9fe918bf1fa0f6a5bc`。Audit和deny各自实际数据库缓存均clean、HEAD精确相同、origin为官方GitHub地址（deny缓存省略.git后缀，仍同官方仓）。database-provenance记录各缓存的1317个完整Markdown文件摘要；其中crates目录1290个Markdown与工具载入1310条advisory是不同统计口径，不把文件数伪称advisory数量。Fresh命令与当时独立remote查询证明本轮来源，不能用旧数据库已有HEAD独自当作fresh成功。

原接受政策、逐组基线、旧be5锁图都另存于本包。原四项advisory exception及duplicate提示规则未改，SDK规范依据继续属于原接受决策；本轮仅新pin下重新执行与有界复审。没有Rust编译、Maven、生产源码/锁/配置修改。Root的Rust14矩阵可以独立运行。

## 已知缺口

本包不证明Rust、三语言、合同、CI或精确main验收。独立有限复审尚待；后续输入、framework revision、检查feature范围或政策改变必须重新验证，不自动继承本结论。npm/PyPI/Maven发布不在范围。
