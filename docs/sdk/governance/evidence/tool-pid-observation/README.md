# 工具 owned PID 观察：有限修复与证据

本切片只改 Python `test_tool_value_effect_host.py` 的 watch_process 分支。原固定50次全局 ps 采样被“发现原独占路径PID，或原 invocation task 已结束”的条件等待替换，外加测试层10秒观察 watchdog。10秒复用此测试已有 fixture-ready 预算，覆盖 request admission / owned producer observation；不是 ACP 默认请求超时、不是框架工具1/5秒执行超时，也不声称产品允许排队的上界。原10ms只是事件循环让步，成功条件始终是实际进程或原调用结算。

原 [Linux CI722 日志](../sdk-722-ci-results/jobs/116172597308.log)在 Git f68 已承接，198565字节 SHA256 `70548f9331858291e374f6e751581cfcf854d8767900d03738d5609041880547`；本包不重复复制。Python shell-signal 的 `len(pids)==1` 得到[]，但原报告未保存当时 task状态、原ToolResult、单调耗时或原始 ps 行。同一CI的TS工具49行/48调用用同样ps参数/匹配通过，不能因此认定Linux截断，也不能把[]解释成从未spawn或已经正确清理。原Linux确切原因仍未知，后续同case/LinuxCI仍须验证；本包不把源码PASS当远端已修。

## 真实受控反例与实现

原源 SHA `b7b59e497d324a2b914e8ab641cfb0b70f9e16523c3d04c7acca7ae211e942a7` 可由 SDK722 Git恢复。Admin受控调用在原 public Session.invoke之前挂 Event，模拟合法调度排队；只运行原 shell-signal参数和 immutable Host `0f1ecf23a2875621caf5128327046cbf6592ce61665c6c6872aed33fdf1931df`。旧50轮实际耗时 **2.592508709秒**，不能把50×10ms名义sleep之和0.5秒当实际时间。请求尚未准入时，原断言失败；放行后真实唯一tail出现，默认/wideps相同，再次exactPID验证后SIGTERM，原 command_output exit-1、permanent/possible/verify_then_retry 及保护文件字节、physicalafter[]均真。它是Mac上的受控反例，不是Linux原故障重演。

完整已审候选为 `dc790509…`。实施阶段Ruff报8个B023，原错误记录保留；只将局部函数引用的task/observed_at显式绑定为参数并格式化，最终源 `8cbcdd12277393999231afafa48d91684232f0ef9408fa5c1b430a74e8b0df57`。独立实现审查确认候选到实际仅参数绑定/格式变化，原722除watch分支外字节保持。最终 Ruff check 和 format check 均0，未使用noqa。设计期候选、提案和manifest保留其当时“待实施”语境，不能冒作运行状态；phase4-source-freeze与后续运行记录才是实施后的实际证据。

len1、再次同PID/PPID/command、仅向该PID发SIGTERM、await同一调用、原ToolResult错误分类/退出码、进程退出及逐行保护文件oracle全部保留，不接受零PID。task已结束但无PID时仍失败，并记录实际结果/异常；watchdog到期也明确失败。wideps仅辅助失败诊断，从不替换成功matcher或signal授权。辅助ps的OSError/CalledProcessError保存专门字段，不覆盖主失败。finally只结算本分支拥有的任务，失败后cancel并await避免遗留；正常结果或异常先由原await传播，不由gather默认吞成成功。框架仍独占工具执行、取消与终态，测试没有另建执行权威。

## 实际 after 证据与范围

- 无Host的三个诊断用例直接AST提取实际nested函数，分别注入辅助ps OSError、CalledProcessError和保留原invocation异常，全部0，主reason/原result或error不丢；不是复制函数实现的镜像测试。
- Python3.12实际完整工具case1方法通过，原49行/48次native调用、五族结果/错误和三个受保护文件逐行检查保持。`native/results.json`保存完整原响应，shell-timeout/signal均真实before单进程、after空，严格分类oracle全通过。不是只选两行或把expected error改成功。四官方research返回仍按原外部观察合同核，不能把当次值当未来固定结果。
- 受控after直接AST提取实际观察函数并使用原10秒watchdog；在旧50样本失败点，候选仍pending，放行请求后发现真实唯一进程并完成原SIGTERM/严格result/physicalexit/protectedSHA。该次旧采样因并行observer与同步ps成本实际 **5.342172375秒**，不改写成before的2.5925。工具从实际请求接收后执行283ms，gate等待不被伪加到工具执行deadline。

上述实际环境为 Darwin/Python3.12.4 与 ACP0.12.1，**不是**CI Linux/Python3.12.3；不编造本地3.12.12或Linux复现。首次native run收据没有记录启动HEAD，不从后来查询反推；精确执行源8cbc在前后均冻结。期间Root仅推进workflow/证据到f68的消息与只读native交错，原结果未丢弃或重复；后受控after在fresh f68路径check0后开始。此前一个base过期check真实失败且没有写入；刷新后才归档。运行前后源体和依赖摘要在after-native-source-and-results.json绑定；原sourcefreeze中nativeExecuted=false保其封存时点，不改成事后状态。

本包只复制必要原件与小场景/结果，不纳入Host二进制、临时Git仓库、运行状态目录、target或缓存。`archive-manifest.json`保原路径/字节/SHA；`SHA256SUMS`覆盖除自身外各文件。新增源码副本只是实际执行来源，不是新SDK生产API。原Tool字节、错误和副作用义务不降级；原Linux原因仍待诊断，完整三语言脚本和新LinuxCI仍各自必需。没有Cargo/Maven、工具实现或公共合同更改。示例和website不受测试观察机制影响，无需新增说明；Finding不能由本局部PASS冒全项目完成。
