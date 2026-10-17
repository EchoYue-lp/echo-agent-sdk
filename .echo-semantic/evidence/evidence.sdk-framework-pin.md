---
schema_version: 1
id: evidence.sdk-framework-pin
kind: evidence
observed_at: 7d0a32a0b049013e366d9a4ed78049c7364cbb98
source_refs:
- .github/workflows/rust-ci.yml
- AGENTS.md
- Cargo.lock
- deny.toml
- MIGRATION-SOURCE.md
- README.md
- README.zh.md
- contracts/sdk/accepted-external-contract.json
- contracts/sdk/accepted-external-contract.schema.json
- contracts/sdk/accepted-facade-operation-catalog.json
- contracts/sdk/inventory-telemetry.json
- contracts/sdk/parity-manifest.json
- contracts/sdk/public-api.txt
- contracts/sdk/schema/echo-agent-extension-v1.schema.json
- contracts/sdk/source-contract.json
- docs/adr/0001-sdk-repository-boundary.md
- docs/adr/0032-sdk-contract-scope-classification.md
- docs/sdk/README.md
- docs/sdk/protocol.md
- docs/sdk/sdk-core-profile.md
- echo-sdk-host/Cargo.toml
- echo-sdk-host/src/core_profile/facade/source_operations.rs
- echo-sdk-host/src/core_profile/wire.rs
- echo-sdk-host/src/features.rs
- echo-sdk-protocol/Cargo.toml
- echo-sdk-protocol/src/bin/export_schema.rs
- echo-sdk-protocol/src/capability.rs
- echo-sdk-protocol/src/error.rs
- echo-sdk-protocol/src/event.rs
- echo-sdk-protocol/src/facade.rs
- echo-sdk-protocol/src/inventory.rs
- echo-sdk-protocol/src/methods.rs
- echo-sdk-protocol/src/schema.rs
- echo-sdk-protocol/tests/facade_inventory.rs
- scripts/check-language-sdks.sh
- scripts/check-sdk-contracts.sh
- scripts/check-sdk-inventory-telemetry.sh
- scripts/export-language-sdk-catalog.sh
- sdks/shared/README.md
- sdks/shared/contract-digests.json
- sdks/shared/facade-operation-catalog.json
supports:
- behavior.sdk-repository-ownership
- rule.framework-runtime-authority
- asset.sdk-source-product
limitations:
- 验证在 macOS 本地完成；Windows Host 编译与 Linux job 由独立 SDK CI 继续提供平台信号。
command_results:
- command: cargo fmt --all -- --check
  exit_code: 0
- command: cargo check --workspace --all-features --locked
  exit_code: 0
- command: cargo test -p echo-sdk-protocol --locked
  exit_code: 0
- command: cargo test -p echo-sdk-host --features sdk-core-profile --lib --tests --locked
  exit_code: 0
- command: cargo test -p echo-sdk-host --features sdk-extension-bridge --locked --test extension_bridge_e2e
  exit_code: 0
- command: cargo test -p echo-sdk-host --features sdk-facade-all --locked --test facade_feature_adapters_e2e --test core_profile_e2e
  exit_code: 0
- command: ./scripts/check-sdk-contracts.sh
  exit_code: 0
- command: ./scripts/check-sdk-inventory-telemetry.sh
  exit_code: 0
- command: ./scripts/check-language-sdks.sh
  exit_code: 0
- command: cargo audit --deny warnings
  exit_code: 0
- command: cargo deny check all
  exit_code: 0
asset_refs:
- asset.sdk-historical-proof-location
---

# SDK clean framework pin evidence

## 支持的结论

`echo-sdk-protocol` 的实际 Cargo dependency graph 不包含 `echo_agent`、`echo_core` 或其它
framework crate。Framework EventEnvelope/AgentFailure 转换集中在 Host adapter；Host 和
`Cargo.lock` 精确解析已推送 revision
`1754877996778afac4e4db77ce37c330496760ea`。

阻断兼容面由 `accepted-external-contract.json` 和
`accepted-facade-operation-catalog.json` 组成，`source-contract.json` 只摘要这两个输入。
完整 Rust public inventory、full parity manifest、Host catalog、Cargo.lock 和 framework
revision 不参与 runtime digest；`inventory-telemetry.json` 记录其来源与 scope counts。

## 运行时与语言验证

Protocol unit/contract/schema/fixture tests、Host standard/core/extension/facade E2E、TypeScript、
Python、Java 测试与三份真实 Host quickstart 均通过。Host 继续调用 framework 的 Agent、Run、
Task、Subagent、cancel、recovery 和 terminal 权威，没有引入平行状态机。

## 来源与范围

证据来源包括精确 Git dependency metadata、protocol Cargo tree、确定性合同生成、Host 三组
E2E、三语言测试与真实 Host quickstart，以及 SDK 文档和 CI 配置。

## 已知缺口

Accepted artifact、extension schema、fixture、protocol DAG 或 framework provenance 漂移会让
blocking gate 失败；只影响非 external Rust identity 的变化仅出现在独立 telemetry 报告中。
本证据不证明尚未运行的 Windows/Linux CI 环境结果，也不代表已经发布二进制或 registry 包。
依赖审计使用修复后的 `rustls` 版本与显式允许的 framework Git 源；cargo-deny 的重复版本
提示属于现有依赖图告警，不改变审计退出结果。


## 当前框架pin与发布后验观察（00782）

本Evidence本次observed_at定位SDK Git `91c50cf3d8fbb8fea1cdf8069a30be2c2f7f1039` 对已保存来源与当前pin的复核。前述正文及command_results是原 `6f743d1f37b604dbeb315c75b9e608a5254dcfe2`／framework `1754877996778afac4e4db77ce37c330496760ea` 阶段记录，保留原source/time/结果；不是在91c50重新运行，也不证明当前完整合同生成通过。

当前SDK八个framework包精确来自已发布 `00782b8aeddefe2ea8c566922cbdee6aedc80d70`，无path patch；SDK锁SHA `88d99f3880f75315f60c8eca48035eb6151a1a055aeda7f621e689b354d411b3`。Protocol继续无framework依赖，运行权威由原framework持有，SDK仅薄转换/owner/resource地址；本观察不新增运行服务或契约。

[正式框架main后验](https://github.com/EchoYue-lp/echo-agent-sdk/blob/0ebe15f66d0aa57c7b7a92ae68cd1be106114ae9/docs/sdk/governance/evidence/sdk-framework-00782-main-acceptance/README.md)保留原八个Admin文件字节及清单。实际commit response签名valid、main与已审ca702树同为b77ee48、main运行38614384013七个job成功；source receipt的570项构建输入没有改变，精确main官方strict exit0，当前/Git摘要同399c6b119。完整570输入表及六工程门禁仍在published framework原包；本最小SDK归档不声称包含或重新执行它们。原main-acceptance-source及旧pin包里的CI pending是原时点文字，不被覆盖。

这些是框架signature/tree/CI/source/strict的后验，并非SDK平台、完整生成或main通过。SDK自身00782 Rust14、793输入完整语言及Git84消费者由各已审正式包单独证明；完整合同只读检查真实exit1/13及12待答、B物理结算残余、Windows原生SDK和registry范围保持。supports仍限既有仓库归属/框架唯一权威/源码产品资产，不据框架七CI提升41族verified。


## 已验收治理摘要的同期维护范围

本段只记录六份维护文稿在SDK eebd切点本地语义验收后的事实同步；Evidence主观察91c50及原framework收据/command_results均保持原版本，不将新文稿放进旧Git source_refs或称新执行。当前六文稿候选逐路径和字节如下，均只替页首一处旧“库存/摘要等待”文字，原历史正文不变：

- `docs/sdk/governance/capability-map.md`，SHA256 `0267041d1c4aa2a20ab5627d25e36371dd71e96a09e981be7e82f034abba4f53`。
- `docs/sdk/governance/behavior-evidence-matrix.md`，SHA256 `4fba125182e0b867acaf7cad077b61c09404a8ad2b965d2b62a94cbb58da87db`。
- `docs/sdk/governance/finding-ledger.md`，SHA256 `85a20621b3e218a653d23036bf18b867434dd4adb35ab35d8453a61ee1cd2a84`。
- `docs/sdk/governance/capability-review-remaining.md`，SHA256 `b6bcc7349b4e90b422de69816b04e46e2e385119f9b42bdf1a353e7cfd08708c`。
- `docs/sdk/governance/capability-review-dispositions.md`，SHA256 `2ccf4db5d7f4d1b756a9c0dc4a9c607ba301df01d56f02051020f6ffa6565da9`。
- `docs/sdk/governance/deferred-capabilities.md`，SHA256 `e9bca3d11d6e9c7c496c3224a225ec840a8832d117b7cf37941bfec3935e315c`。

原eebd本地官方全项目strict退出0、source473与Git一致及308正式语义文件对账仍只属于eebd保存切点。此次文稿及baseline摘要c75是后继候选，须按真实当前任务基准重新执行完整strict；先前因缺少同次语义对象依据的exit1保留为真实失败。库存建模closed/行为open、52resolved4open/B3共享物理B与12连续性、人类例外边界、SDK main/Windows native限制均没有改变；不新增能力/接受身份/状态权威。

后继CI设计和固定工具版本尚待实现及实跑，现有工程CI成功不证明执行语义校验。本文只是这些既有维护文稿的精确变更依据，不充当新runtime行为证明或未来CI结果。


## 语言CI编译资源配置同期依据

本次差异只在 `.github/workflows/rust-ci.yml` 的 `language-sdk` job新增三个Cargo环境值：`CARGO_BUILD_JOBS=1`、`CARGO_PROFILE_DEV_DEBUG=0`、`CARGO_PROFILE_TEST_DEBUG=0`。原SDK Git `6d0c73aac1e181b125392f126e9659676eae5219` 中workflow SHA为 `cc519d0da616dcced4fab5984e2efc2a9519a53436aa39aea360770c04e132e3`；本候选当前workflow SHA为 `c42a5197da5bb7b46c9af98fa57b78dc612e4e717a23f781bf9a273a6cdce78c`。这是当前候选文体定位，Evidence主观察91c50与原source_refs/command_results保持历史，不冒旧Git包含新env，也不把配置核对当新执行。

原入口 `scripts/check-language-sdks.sh` 的Host `cargo build` 与 `scripts/build-telemetry-test-collector.sh` 中framework collector `cargo test --no-run`继承job环境。jobs限制的是Cargo编译并发，未设置测试运行threads。dev/test profile debug=0只关闭编译调试信息；本差异不设置strip、debug_assertions、overflow-checks、优化等级或测试线程参数。四行增量没有修改任何原命令、target、feature、locked、timeout、错误/失败规则或其它job。CurrentTool已完成设计及该实现差异有限审查，C0/I0/M0；YAML重复键与其余AST同原的focused静态检查通过，actionlint未安装，不伪其结果。

原793执行输入中workflow这一项现在发生改变，不能再称整个当前793与旧执行逐字节相同；61份Rust/Cargo/config输入与运行代码、锁、生成物、测试和原门禁日志没有因此改变。本slice只验证编译资源配置及语义候选材料；不重写或重跑原14Rust/完整三语言证明。旧eebd运行中的CI没有被取消，本候选CI尚未运行，未声称修复已运行job或新remote0；预算设置不构成新的语义snapshot CI job，不证明远端执行了官方语义工具。52resolved4open、inventoryclosed/behavioropen、B与12连续性/完整生成exit1/13、Windows native与SDK main边界均保持。


## 第四完整入口及四具名闭合后的当前文稿维护

本段只承接当前文稿与已提交具体三链状态的一致性维护，原Evidence observed91c、source_refs/原命令及所有旧运行日期不改。SDK0cbd已提交四个具名缺陷闭合，当前56resolved/4open；新三语完整入口正式包在b315真实Git保存，原执行9dd含一个当时未提交client399961→b0恢复，TS336/PY415+18/Java355/76freshXML/3QS及800相同逐包定位，不冒旧793或未来Linux通过。此处不提供新业务证明或新whole执行结果。

本次唯一七份维护文稿当前字节：
- `docs/sdk/governance/finding-ledger.md`，SHA256 `054780461a861ffe5442b3cceaac948fdf2b2370e7f6003bcecf345e2d20eced`。
- `docs/sdk/governance/capability-map.md`，SHA256 `15ca2524b6b0292657992acd3a141ea362d03abaa8c2a87d0c0c7ec8b1def76b`。
- `docs/sdk/governance/capability-review-remaining.md`，SHA256 `349a31de89a9c0a5bce8f8993f98b34003ca6d511508146f030a872c3b0c31fc`。
- `docs/sdk/governance/capability-review-dispositions.md`，SHA256 `44926eab8aa8612f38ca0ac8f541355467677a6ab19de4ad51e869e4197364a8`。
- `README.md`，SHA256 `9828351422d02002b941a5f1b60fade021b4cdbe74cdc8ee3b3f8739d254e9db`。
- `MIGRATION-SOURCE.md`，SHA256 `45ea97df7083eb5404f70cd94bb1426acef9740236d82b109d4c7d1d1cbd77d7`。
- `AGENTS.md`，SHA256 `19475457a70e09ff30d4343452541dc3f1804c14bd98431f20eb6e0d8db6d5d5`。

这些更正只同步current状态、已执行与未执行范围，历史正文、旧raw与原ADR不改；AGENTS仅当前门禁事实，未改变规则。800运行输入及61Rust/config实际SHA/mode不变，文稿会改变完整source摘要，不能借eebd/旧970c摘要称当前strict0。库存closed/行为open、3个宽Finding共用B物理结算与12项连续性待选择、合同13失败/Windowsnative/远端语义CI及SDKmain边界保；第三仓工具修复仍待授权，原30d差异门禁UTF8失败不被这次文稿slice或无base快照检查改写成通过。


## 722 Linux后继有限修复的当前文稿维护

本段仅对四份已审current文稿及三份既有openFinding进展同步提供精确依据，Evidence原observed91c/旧source_refs/命令/原时间不改，不把这些后继字节写成旧Git内容。当前模型56resolved/7open；722真实PRmerge5d407八job5success3failure，语言TS336及quickstart通过、PY413通过2失败18subtests、Java未达，两个合同job原13阻断不冒通过。三新具名Finding仍open：Gap/PID已有focused修复和有限Mac后验，原Linuxcase新CI尚未执行/审查；PID唯一原Linuxcause仍unknown，bootstrap单input实施后新remote完整error/warning结算尚待，未RiskAccepted或自动关闭。

本次维护文件路径和当前SHA：

- `docs/sdk/governance/finding-ledger.md`，SHA256 `dd90d8a6ba9e4b5d68afeac4eb833c584d52d0abb720badd7bf098b5a3d6dba5`。
- `docs/sdk/governance/capability-map.md`，SHA256 `b21b91114b13f19eaafc5292211ef714d8afdae47d52d0839bf163073dbd59ad`。
- `docs/sdk/governance/capability-review-remaining.md`，SHA256 `b425345e7bb227dc52d8c2d8d274193d07ae6fb0e43caef443f30b62e9dcda8c`。
- `docs/sdk/governance/capability-review-dispositions.md`，SHA256 `f75ee68cb7bdb430cb594345ba91630542bdab9f2b4c7ca2498b063c8e3b509e`。
- `.echo-semantic/findings/finding.sdk-gap-duplicate-ack-successor-observation.md`，SHA256 `8da411664d09d231739f1c142040a0dad42f2c2661aaf659f2789a3ae6fbe181`。
- `.echo-semantic/findings/finding.sdk-tool-pid-observation-window.md`，SHA256 `748205eea0a3c6b1e689d134a98c4dd62e43b22087fd4cbba9bd084285c9ea04`。
- `.echo-semantic/findings/finding.sdk-cargo-deny-bootstrap-toolchain-probe.md`，SHA256 `575657de4fe2226685f0644f4844b4b7c64e98ab9f3e128a370de94fed93160d`。

相对原第四whole800执行输入，真实差异仅5项：workflow的cargo-deny初始化input及四份测试（Gap三语言、PID Python），其余795原输入相同；不能称整个800与旧whole当前byte相同。生产61Rust/Cargo/config和锁仍同原正式门禁，Host0f1为这些已记录focused执行二进制。原第四whole/macOS0保各自cut，现focused开发提交后下一remote验收计划另行执行，不在此复制本地大target/重14矩阵或假远端green。SDK行为模型open、库存closed/40格不动，3宽Finding同B与12连续性待人、原合同13、Windowsnative/main边界不变。

第三repo工具交付权限已获得，当前工具仅PR3/审查门禁进展，尚无可用于SDK官方工具消费的已发布新SHA/main；新语义CIjob未实跑。原历史permissionpending语句按原时点保留，不将现在授权等同交付通过。原30d全goal工具UTF8失败必须用本次实际输出保留；当前无base快照通过也不能取代原goal变更门禁。


## 公开22e全goal资格与新增semantic CI定义

本Evidence当前观察7d0只复核真实保存的旧材料与新增CI定义；原91c/6f743的运行日期、command_results及原框架后验正文保历史，不据本次观察声称它们重新运行。工具22e241f723673330e9d6bec638def0f78968c74c为已公开main，canonical verifier393f55/119导出blob逐原Git同；SDK4994/tree800044在干净本地shared clone（有显式alternates，非freshremote）5438文件同原Git、32历史tag可读、不复制或伪造repo-local preflight。实际PEP723命令strict-snapshot/base30d7/require-change-evidence退出0、174B完整日志、约119s，源eda628前后同；FrameworkFinal独审PASS0。旧invalidUTF8原blob与公开8ec/installed0.5失败日志保持；仅该原全goalUTF8检查器阻断在SDK4994获实际解除，不等installed0.5task513问题也被修或全SDKgoal完成。

新增semantic job定义已7d0提交，直接uses公开action exact22e；唯一官方脚本在GitHubactionstore执行，不import/vendored SDK归档的源码副本，不新validator/服务/边界。checkout fullhistory/tags，base只用真实PRbase或push-before非零40SHA/存在/祖先，未知事件显式失败；action原strict与require-change-evidence=true保留，无fakepreflight/baseHEAD空diff/continue-on-error。仅setup-uv性能cache关闭，不改PEP723依赖或验证；原七job（八matrixentries）与runtime命令不被新job改变。工具资格在原SDK4994且源eda628，而新增workflow会改变当前全source摘要，不能借该资格假新当前树strict或semantic远端job已过。

资格正式23包在117ace保存、新CI定义/来源30包在7d0保存；本次source_refs读取两真实保存cut的7d0后继树，不把原执行写成7d0。相关路径Hash见候选sourcechecks。现semantic job只defined/not yet executed，下一CI需要真实官方同base机器结果；SDK4994原8job自然终态5success3failure，其中dependency特定bootstrap后验有单独三链候选，ACK未见独立fail列表不推所有路径已验、PID只有pids[]截断cause unknown，两个合同job仍原13失败。原本地focused/整体门禁各原cut保，56resolved7open、B3共享物理B/12连续性、Windowsnative/main/registry边界保持。不新产品权限或退约，公开工具main不等SDKmain。


## 公开工具资格及bootstrap局部闭合后的文稿维护

本段只提供四份current维护文稿的精确同期依据，不新增运行证明。原Evidence观察7d0、全部source_refs、公开工具22e资格、semanticjob定义场景和旧运行命令/时间按原sourcecut读取；不回到旧91c版本覆盖新已提交模型，也不把新文稿或当前验证结果塞进旧执行。当前机器模型57resolved/6open，仅CIbootstrap在SDK4994真实dependency后验/精确50版本组复审后具体三链闭合；Gap/PID仍待新Linux原case/完整后验，PID原截断结果pids[]不构成已知PS截断或typedtimeout原因。B3共享物理结算与12连续性裁决、合同13、Windowsnative和SDKmain仍各自未完成。

本次四份已审维护文稿当前SHA：

- `docs/sdk/governance/finding-ledger.md`，SHA256 `5b53e8de5d57008cdc1b30f4f40887b3231df870d482fd7ea0c534ffde7dca22`。
- `docs/sdk/governance/capability-map.md`，SHA256 `ab0d9523cfbfe69c8dde14b94dac7912573af3916a9dd7dfe43c15a8409ee5e4`。
- `docs/sdk/governance/capability-review-remaining.md`，SHA256 `1d6585871b350f4c208f28147ce8bef7973973b4513e5fb70a52050f3762fbb9`。
- `docs/sdk/governance/capability-review-dispositions.md`，SHA256 `715377bf5f91cf794ed81840a473a40152df40e065254ada44973f554eb08828`。

公开Echo Semantic22e/393 canonical在原SDK4994清洁sharedclone对真实30d完整语义变更门禁已实际退出0，旧UTF8失败保历史且不再作该资格的当前阻断；installed0.5 taskscope513不同问题未声称被修。新增SDKsemanticjob定义固定uses22e，基准来自真实PRbase/pushbefore，尚未实际执行，不以本地artifact门禁证明远端job或合并main通过。本次维护会改变完整source摘要，须用真实当前候选树重算并运行同公开工具原30dfullrequirechange；不能借eda628/4994旧结果、空diff/baseHEAD或删除S预检高flags代替。本次不修改运行源码、合同seed/生成产物或任何原日志，不重复Rust14/本地大targetwhole。


## 公开工具commit来源响应原件的精确角色

`docs/sdk/governance/evidence/sdk-semantic-ci-consumer/published-commit.response.json`，SHA256 `bb4cce891a4fde160bb83e490889cd297f7533796b5b66c0ada328b7caba5b56`，48797字节，是已保存的官方GitHub commit API response源码/签名来源见证。它在本Evidence原观察Git7d0已真实存在，与当前原字节相同；payload声明公开EchoSemantic22e241f723673330e9d6bec638def0f78968c74c、tree47d63f99及signature verified，对应consumer provenance。response里附带工具提交diff/源码字段不成为SDK运行模块、公共API或第二validator，也不新增协议/状态权威。本slice只把这一个实际原件列为同期已有ProductCI来源依据，原sourceRefs/支持范围/command_results及所有旧运行时刻不改。原30d本候选首次fullgate缺这条路径的exit1保留，不能把它当SDK业务失败或伪改成通过；补依据后的实际重跑结果另收。


## 源码具名后继与唯一当前状态页的维护依据（286）

本段只绑定四份已独审current文稿与已经提交的具名源码后继，不制造新业务/语言/工程执行。原Evidence观察7d0、source_refs/command_results、原所有sourcecut与运行时刻完整保留；新current状态唯一维护于capability-review-remaining.md，其余三页只指向该页，原独有历史尾部保持原byte。实际对象为58resolved/7open：185559的Python/Java nested parent纯值投影只有该具体缺陷三链闭合；PID已有0ebe原default/wide同一进程的截断因果与c5的-ww修复，但仅Macfocused通过、新Linux后验待；Host原0ebe首被动child.wait10s超时原因仍未知，fd432只补上下文诊断不提高deadline/kill/retry，不把原失败当receipt丢失或B策略结论。

四份维护文稿的当前精确路径/Hash：

- `docs/sdk/governance/capability-review-remaining.md`，SHA256 `608b2bba72d94bbc85fb082c7e85bce370f2590962e67cdbddfa240390654e81`。
- `docs/sdk/governance/finding-ledger.md`，SHA256 `4bc1249e1f44822ad14bfd55c46b2bf7e1e0768049e29012c1ad9e4d83c6e54b`。
- `docs/sdk/governance/capability-map.md`，SHA256 `63a61073ab5ba8f0e265d281dae121c276b8e459855de92274446b2ccd0d5f2c`。
- `docs/sdk/governance/capability-review-dispositions.md`，SHA256 `b18149539111c0b97c09487baee6c4ec55348f06c45b408f0cd91fc101ab894c`。

当前源码保存Git `286dc7542903d66fac747f30f6731cfc47dba4e6`相对原0ebe实际仅以下6源/test字节改变，不声称原800或Rust61当前全部byte仍同旧执行cut：

- `echo-sdk-host/tests/extension_bridge_e2e.rs`，当前Git286 SHA256 `25b43267c672987430118d477683f58ba22d96066257661591d98b36560a19bb`；原推导角色 `test_consumer`。
- `sdks/java/src/main/java/com/echoagent/sdk/EventIdentity.java`，当前Git286 SHA256 `070457f0796290c87b4c3003b7fd5978c51282a80080445d5489c48a9564ec53`；原推导角色 `production_source`。
- `sdks/java/src/test/java/com/echoagent/sdk/EventIdentityTest.java`，当前Git286 SHA256 `03ba5398f2821027a23096427bfab0e1eff4cfca4a1f4c39b8f3bf986744f2cd`；原推导角色 `test_consumer`。
- `sdks/python/src/echo_agent_sdk/event_identity_values.py`，当前Git286 SHA256 `f5943def2bf95941f6947a87bb6b487e96cd8965120bf9a7724153e2fb3ecd4f`；原推导角色 `production_source`。
- `sdks/python/tests/test_event_identity_values.py`，当前Git286 SHA256 `ea68c63d1f8e426ac2371a5f1232576016156a93e7c82e1942e24a5c85a608f2`；原推导角色 `test_consumer`。
- `sdks/python/tests/test_tool_value_effect_host.py`，当前Git286 SHA256 `62bc5fb0a8f5512060ce031303a80cbe1d0b1d1a1fad79c317aa53ac866d06ee`；原推导角色 `test_consumer`。

其中两项生产变化只现有EventIdentity accepted nested parent投影，另外四项是两份lineage具名测试、Python PID观察和Rust被动退出诊断测试；原API/Host业务权威/accepted合同seed与权限policy不变。185559实际probe红绿与PY模块3/J类3、Oracle21运行和release17编译仅原有限执行；c5 -ww后Darwin49rows/48calls仅该平台。原第四whole、之后Linux各fail/pass与测试未达范围各保原epoch，不把当前源码绑定写成新whole工程通过。相关具体证据/审查由已提交EventIdentity E/A与两个open Findings拥有，本Pin不替代其业务审查。

公开22e工具原0ebe远端semantic job116226422797已经对真实30d完整fullmode通过；这是结构/引用/摘要/变更依据门禁，不是language/Host/contract工程全绿。当前这六份文稿/元数据候选仍须同公开22e对原30d fullrequirechange实际复验，不能借旧4994/0ebe或无base快照0冒新结果。新的Linux PID/Host诊断验收未发生，B3/12连续性人类选择、合同13与平台/main范围独立。

用户要求的main文件必要性精简尚未执行任何归档删除；普通fullproject/change门禁不取代显式continuity。原parent391/完整1173诊断与A候选精确95解析修复/B未决定storage合同分别保留，不删义务、不降risk、不以本段接受新B门禁政策或宣称payload可合main。


## 131三项Java测试support与唯一当前页的索引

当前来源保存Git `131bb03f1e87dc8411ebc1f81e87dd927962dc54`，相对0bc仅三份Java测试变更（无framework/pin/锁、SDK生产、accepted seed变更）：

- `sdks/java/src/test/java/com/echoagent/sdk/StreamCancellationRouteHostTest.java`，Git `8156b64a3aee785b98e93adab65744709c0c7496` SHA256 `738b216a99883318191653445a64195e4a2abce6952e57fc975bac9d67bdda72`。
- `sdks/java/src/test/java/com/echoagent/sdk/HostEarlyExitTest.java`，Git `b9bc86ac0c6866283af912ca3da382c495d55240` SHA256 `717edf3e172c1aa3d2a0064c140b57b6a26544e293c69960f27a9ef7770855eb`。
- `sdks/java/src/test/java/com/echoagent/sdk/TaskSubagentHostTest.java`，Git `131bb03f1e87dc8411ebc1f81e87dd927962dc54` SHA256 `f8e4f11495d9bb11174cce45da686d1e1f3597976e6d33959531989c76075cc5`。

具名原Linux四errors、3修复原因、focused1/16/两profile各4及原独审角色由既有 `evidence.sdk-official-current-language-gate` 与 `audit.sdk-language-client-lifecycle-contract-evidence` 的三新challenge承担，本Pin只索引来源，不扩业务支持。0bc CI6success/3failure与MacOracle21 focused/待新Linux边界保留；当前唯一状态页 `docs/sdk/governance/capability-review-remaining.md` 候选SHA256 `bdfb189243a4bb31526c0905e4c8bc29fe7b132fed04ecdbf43893330e67bd8e`，它以本次真实候选树绑定，不伪为131 Git原文。原Pin观察7d0/source_refs/旧命令/历史执行保；58resolved/7open及B/12/合同13/continuity与payload BLOCK保持。当前候选摘要和公开22e原30d完整门禁收据另核，本段本身不声明新gate或新CI已经通过。


## 历史证明存储的有界表示迁移

原id/业务正文/status/supports/limitations/命令及原观察含义不改变；本冻结父 `7f14a1b9c751b0612ceb42224f83f29d09aea46c` 中此对象完整原header/source refs可由公开证据tag恢复。只将具体archive物理依赖交当前位置资产与真实same-role consumer，全部旧义务由显式Eq/CT承接，不把存储重观察冒原业务新审或旧失败变成功。
