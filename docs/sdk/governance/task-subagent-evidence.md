# Task/Subagent 共享候选行为证据

本组原四场景与后继matching Task一例纳入正式三语言测试发现规则：TypeScript文件由`npm test` glob
发现，Python文件由pytest测试目录发现，Java公开`TaskSubagentHostTest`包含五个具名
JUnit `@Test`。三者共享确定性HTTP响应门和已接受的RevisionedTaskStore回调；不接受新API。

共享fixture只执行原子的expected relation revision/expected execution比较、保存完整next
图及原始commit记录。候选next由真实framework传入；fixture不生成claim/status，不计算DAG，
不调度任务，不拥有取消、恢复或terminal。Agent回调门只控制已被framework调用的真实客户
实现何时返回，不能替代framework运行结果。

| 场景 | 精确行为与副作用对账 |
|---|---|
| Task pause/resume | 暂停A时B继续running、C pending；释放B后B/C完成；恢复A后A第二次执行，B/C不重跑，D消费同一canonical B result；每次后续执行commit保留完全相同B execution事实 |
| Task cancel | 只取消A，B/C继续完成；执行计数A1/B1/C1/D0，取消不会变成图级取消 |
| Subagent await/receipt | 并发相同idempotency dispatch只产生一实例；5ms短await返回未结算且无result；冲突payload明确错误；释放后完整Unicode结果被重复await原样复用，terminal后guidance不重新派发 |
| CRUD-only Store | execution_cas_v1=false时Task execute明确FeatureUnavailable并保留TaskRun handle；Agent invocation=0、Store/execution commits无新增，Task仍pending |
| Matching Task observation | 原callback/claim派生execution身份和同Popen保持；held A仍Running，B独立CAS完成，C/D在下一wave等待；原join/Complete后A唯一exact CAS，C/D消费原A/B结果，B execution不重写，四次执行各1 |

Task场景还检查stale relation update不修改Store、duplicate execute不接管现运行、running
claim关联canonical revision、完整TaskResult供依赖消费。
Cancel场景只有A/B/C三个Task，D是已注册但没有创建的目标；D0不证明取消后的依赖后代阻断。
Pause场景才创建依赖A/B的D，用于证明resume后依赖结果复用。opaque id和native duration保在原始
记录，三语言比较仅规范关系、canonical revision、计数、状态和结果，不要求随机id相同。

原草稿三语言运行使用Host
`6711dd17fe3be13671ee181fce5ee79946f40963e000af293083f114301a39a5`，原文件、原始回调/
commit状态、日志与三语言比较收据完整归档于
[evidence/sdk-task-subagent-20261013/archive-manifest.json](https://github.com/EchoYue-lp/echo-agent-sdk/blob/0ebe15f66d0aa57c7b7a92ae68cd1be106114ae9/docs/sdk/governance/evidence/sdk-task-subagent-20261013/archive-manifest.json)。
正式测试路径与环境改造的运行单独归档，不重绑定草稿epoch或旧whole-source摘要。

正式入口：`sdks/shared/testing/task_subagent_fixture.py`；
`sdks/typescript/test/task-subagent-host.test.js`；`sdks/python/tests/test_task_subagent_host.py`；
`sdks/java/src/test/java/com/echoagent/sdk/TaskSubagentHostTest.java`。
没有个人仓库绝对路径或草稿路径。`ECHO_AGENT_SDK_REQUIRE_HOST_E2E=1`时缺失/空Host
必须失败，不能跳过。每个场景各有独立work/state/backend，原始证据保在场景目录。

后继14a独立formal-only Git候选对三语整个Task文件/class各实际5例通过，TS重建含examples、Python加载候选src、Java clean重编译。新源码和详细日志/限制见[原Task Evidence](../../../.echo-semantic/evidence/evidence.sdk-task-subagent-candidate.md)。原ordinary foreign setup/stream场景不在本候选，仍由F ACP后继修复后另验；不把dirty工作树的formal子集冒称该工作树整套通过。

Task时序按[Accepted ADR0060](https://github.com/EchoYue-lp/echo-agent/blob/9886502c83d6b1f6fa9212eb221db0e5874deaee/docs/adr/0060-task-execution-store-cas-and-local-interruption.md#L139)及[正式Task API](https://github.com/EchoYue-lp/echo-agent/blob/9886502c83d6b1f6fa9212eb221db0e5874deaee/docs/en/09-tasks.md#L129)：每claim join后立即CAS，但下一wave等本波全部receipt。仍unknown持claim的A与已物理结算Paused的A不同；原经典BC-before-resume不变，新例不要求C在unknown A结算前提前派发。fixture的`--verify-formal`只读原记录，不创建claim/status或成为调度器。

本组对rich constraints只证明客户实际收到的编译prompt字符串保留指定约束，并不证明
TaskSubagentContext完整DTO直接传给CustomAgent callback。TaskRun多revision变更、嵌套委派、
失败/重试/restart全矩阵、TaskSubagentContext所有字段与Host shutdown竞态仍需独立证据。
候选临时framework组合和冻结Host不替代正式published pin、独立构建、最终完整门禁或
精确main验收；规范化输出不是DAG/store状态的第二权威。
