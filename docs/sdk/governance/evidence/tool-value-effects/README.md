# Tool 值、副作用与反向消费补证据（进行中）

本轮针对已接受的 files、shell、web、database、research 五簇有限场景。三语言使用原 public `Session.invoke` 和相同共享输入；夹具只准备文件、SQLite、HTTP/SSE、校验结果，不向 Host 发送 ACP。完整 Rust inventory 不决定兼容范围。

当前二进制是内部候选 `echo-sdk-channel-forward-candidate-host`，SHA-256 `0435e0b57fb26184dc98fe69f9d071826bee1c597e4182a89feead4dfef9f802`，编译源 `3cd835dbf40abf1d0aec04e02f3b51411631fcaa50ba7e5f0f2d2d2f500ed9a6`；这些结果不等于最终 published framework pin/main 验收。

## 当前结果与反例

普通工具共同语料 48 行/47 次原生调用，三语言真实执行完输入再由同一个 oracle 检查。原始结果见 `before-0435/{python,typescript,java}-tools/results.json`。严格整体验证仍失败，没有跳过或改用默认值：

1. `describe_table(sqlite_url, "records")` 成功却返回四行、六列全 `null`。同一数据库真实 `PRAGMA table_info` 含 INTEGER/TEXT、主键、非空和默认值 7；三个 `database-red-counterexample.json` 保存 authority rows、实际 payload 和不变的前后数据库 hash。根因路径为 framework `echo-tools/src/database.rs::format_db_rows` 使用静态列类型判断动态 PRAGMA 值；源码 owner 正在修复。
2. 普通 `web_fetch(loopback/large, 8)` 返回约 120 万字符且 `truncated=false`，没有 artifact。SDK `facade/tools.rs` 的 `ToolContext` 丢失既有 Agent output_artifacts 配置；framework 的 WebFetch 只有收到配置才保存完整内容、截断预览。Host adapter owner 正在复用原配置修复。
3. 已接受 `sandbox.extension.run_stream` 按 catalog canonical 拼写调用，却未命中 Host 的 `shell.sandbox.extension.run_stream` 分支，落入一般 Tool dispatcher 对 extension Handle 做 JSON 投影并失败。三语言都在 producer 创建前报 `invalid_value`；此时没有真实进程，不能视作已验证取消。Host owner 正在修复 canonical 路由。

`java-second-xml` 是原始 Maven 全类结果：3 个测试，1 pass、1 failure、1 error、0 skipped。Python/TypeScript 的单独原 Sandbox 日志是相同路由反例。SDK 的字段转换仍由原框架 Tool schema/input validator 唯一校验，不新增验证或调度权威。

## 已验证的有限行为

普通语料实际 payload 和前后文件快照已保存：Unicode 文件 create/append/update/move/transactional patch/delete、失败更新/重复创建/过时 hash/多文件 patch 回滚，文件 read/diff/list/grep/glob/repo_map/code_search；原 artifact persist 后四页 UTF8 连续游标、完整 SHA-256/byte offsets 和坏游标/变更后拒绝；Shell `printf` 的 command_output、`ls` stderr/非零、默认允许 `tail -f` 的 1 秒真实超时及 producer PID 退出；SQL 两页无重复遗漏、游标绑定 query+limit、坏 URL 的 RPC error 与缺库/SQL/过时游标的 ToolResult failure。整套 oracle 将在三个真实问题修复后的二进制上重跑。

SearchProvider 三语言独立测试已全部通过，实际注册原 agent_component 并由真实 Agent/Run 的 `web_search` 工具消费。共同场景为 1 次成功加框架原有 3 次失败重试；完整标题、URL、UTF8 摘要、空摘要进入实际模型请求，异常信息保持 `provider failed 中文🙂`。提供方收到完整查询与原框架全量获取限额 10，页面限额 2 属于 Tool 分页策略。`before-0435/*-search` 保留原 callback、Run terminal、replay 与模型请求。这不是固定 DuckDuckGo 网络成功证据。

Sandbox 原生 producer 消费者已写三语言各 success、pending invocation cancel、pending close+重复 close 三个有限子场景，修复 canonical 路由后执行。这些测试明确证明 callback 所拥有的真实 `printf`/`tail` 进程结算；不会把这个证明冒充默认 ShellTool 的后代进程清理或回答之后的 stream 取消。

## 外部服务范围与失败记录

四个研究搜索均使用原工具固定官方端点，每语言各一次最小查询/limit=1。已观察 arXiv、PubMed、ClinicalTrials 的真实 success payload（包括缺省字段及实际列表）；Semantic Scholar 返回真实 429，错误文本/分类保留。它们是外部时点观察，不要求三语言所得论文完全相同，不以 loopback PDF 代替官方搜索，也不新增端点设置来伪造稳定成功。实际响应可复核，不宣称固定服务在后续时刻必可用。

`python-first/second/third/fourth`、`java-first`、早期 Search 日志同时保留夹具错误历史：canonical 操作原拼写、Record 外包 value、UTF8 页数、SQL 文本分页 notice、Java 显式 WireValue 必须保持 JsonNode、SSE/明确 full-auto/typed request 字段。它们不是生产缺陷证据；生产反例以上述具体路径、原始 payload 和三语言复现为准。一次默认 HITL 夹具等待仅终止了 exact work/config 匹配的本轮 Host，并保存 own PID 观察；修正测试为原公开 full-auto 后真实 Run 正常完成。

新测试/预检/影响范围在 `preflight.md`。新代码只新增 shared fixture、三语言 test、此证据目录；正式文档、示例、官网无需引入新的产品 API，本轮验证仍待生产修复与最终统一门禁。

任务语义校验实际已执行（`semantic-task-scope-with-base.log`），当前返回非零：本任务其它并行新差异及本轮 new test 路径仍待共同语义依据绑定。未跳过、未宣称全局闭合；语义 owner 在最终 source freeze 后统一绑定与严格验收。

第四个有限反例：ClinicalTrials `total_results=0` 与实际 `returned=1` 不一致。`clinical-trials-total-probe` 保存同一原官方 GET 对照：默认请求 status 200 仅 `studies,nextPageToken`，没有 totalCount；仅追加原官方参数 `countTotal=true` 后 status 200 得到 `totalCount=386`、1 study。原框架未请求 countTotal 却把缺失读为 0。修复由 coordinator 分配新的窄框架 owner，不改变 nullable 合同；最终成功响应 oracle 要求真实 total_results >= returned，不能用 0 掩盖未知。

`context-0a0f` 归档新候选实际工具及 Sandbox 结果。Web8 截断和完整 artifact 已真实恢复（压缩原文件及 SHA 独立可解），正常 Shell 的 timeout/SIGTERM PID 已退出。新 Sandbox 路由启动真实 producer，但 pending cancel/close 的语言 cancellation signal 仍 false：fixture 5 秒 watchdog 后执行 finally 杀进程，不是 Host 取消成功。该第五个反例已交 original bridge owner；三语言原始日志与 XML 保留。合法 sandbox_cleanup 回调最初未实现导致 fixture 错误，已修；Java cleanup 被误算第二个 finally 的夹具错误也已在下一冻结源码中修正。

共同结果比较器已准备，`context-0a0f-comparison-observation-second.log` 只能表明三语言在列出的字段上共同返回相同观察（包括相同错误的 NULL schema），不能代替独立的正确值 oracle。最终正确值、物理结算与 whole9 测试仍待新修复候选；SQLite 独立文件二进制可不同，因此文件不变哈希在同次调用前后验证，列和值按规范逐字段比较。

`settlement-f852` 保存首次取消修复候选出现的真实回归：正常 Sandbox 完整发送 output、complete payload 后，外流第三个事件为 failed，根因 `Unfold must not be polled after it returned Poll::Ready(None)` / producer_exit=panic。下一次 next 才出现 closed_handle，是继发结果。消费测试已改为在 failed terminal 立即失败并显示完整原 cause，绝不将 failed 当作普通 EOF；该问题由 bridge owner 修复，三语言的失败事实均保留。

`fused-06bb/REVIEW-SCOPE.md` 定义已经实测通过的三语言 SearchProvider/Sandbox 六项有限复审范围；九个真实 Sandbox producer 已退出，正常 sequence 1/2/3、取消信号与 close/EOF 后结算完整，原 -15/143 不改成 0。Maven 全类原统计仍 2 pass / 1 fail / 0 skip，DB/Clinical 正确值 oracle 未完成。不得将本包范围推广为全工具组或精确 main 验收。

`published-framework-b909` 使用真实 Git framework revision `b9099558620f5f9693c736c85c214db14f62a8cb` 构建的 Host `ee9ff375...`：三语言普通工具 49 行/48 调用的严格正确值 oracle 均通过，数据库列定义/default 7/null 正确、ClinicalTrials total >= returned；共同结果比较 45 个确定性行一致，4 个官方搜索保真实时点观察。Python 与 Java 全三个组通过；TypeScript 首次 2 pass/1 Sandbox failure，正常流只收到 output 然后 EOF，缺少完整 Sandbox result 事件。同二进制仅该有限场景复跑通过，首次竞态仍保留为 open，不能以复跑绿覆盖。原始工作目录、源码/客户端/catalog 与实际 framework build 收据均归档，最终 SDK 合同/main 门禁仍单独未完成。

Sandbox 正常结果遗漏竞态已定位：Host adapter 在自然 EOF 路径取消 cleanup child，使 `FacadeStream.next` 在 `recv` 和已就绪 cleanup 之间随机选择，可能绕过队列中已发布的 complete-result item 直接 EOF。原 b909 首次两事件与同包复跑三事件都是有效观察，不能用后者覆盖前者；修复由 adapter owner 保留自然结算事实，显式取消/发送失败仍沿原 signal+drain 路径，消费者不降低 sequence/result 强断言。

`b909-natural-eof-after` 是本轮有限范围最终实际消费者验证：三个完整命令首次均 3 pass/0 fail/error/skip，9 测试；原消费者4源未放宽，16实际运行source及完整原生IO/结果/模型/事件/编译产物摘要归档，45确定性工具行比较一致+4官方服务真实观察。终态正常完整结果事件与EOF sequence1/2/3，9个producer都结算。任务范围语义校验仍须共同model owner完成新差异证据绑定；本包不替代完整合同生成与SDKmain验收。
