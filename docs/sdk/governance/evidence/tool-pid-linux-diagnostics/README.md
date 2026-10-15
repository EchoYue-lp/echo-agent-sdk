# Linux PID 故障：完整诊断取证补丁

这是 [既有 PID 观察补证](../tool-pid-observation/README.md)之后的诊断增量，不是已确定的 Linux 根因修复。原 [4994 CI 语言日志](../sdk-4994-ci-results/jobs/116211906745.log)为197614字节、SHA256 `8941794e23ac55544bd966c70447f1b86340330dc91ccb7ab4c12417e5fc191e`。TS336及TS quickstart已过；Python414pass/1fail/18subtests，Python quickstart和Java未达。本轮Gap已绿，唯一失败仍为Python工具 `len(pids)==1` 得到[]。本包不重复复制相邻CI原日志，也不声称相邻包在本封包时已经Git提交；其确切恢复提交由主任务后继里程碑记录。

失败在5.028703秒落入len1分支，未触发10秒观察watchdog，所以不能继续以“需要更多观察秒数”解释。observer在无PID时只会随原task完成返回，但原pytest大dict repr把task_done和实际ToolResult截成…；五秒与工具deadline相近只是线索，**不能在未见完整结果前宣称实际timeout原因**。之前的受控调度反例只证明固定轮数能误拒合法排队，并不能代替本次Linux实际原因。

本机没有docker/podman/limactl，不安装大镜像或假称Linux复现。无Host的两个预期失败用例真实证明：dict断言会被pytest省略，而固定marker与JSON字符串在captured stdout中完整保留Unicode和结果字段。其run-command原exit为1，日志清楚写2 failed；它们是诊断呈现的预期反例，不是工程全绿门禁。合成的timeout字段只用于序列化测试，绝不称为4994真实返回。

## 最小实现与边界

唯一源码为Python测试文件，原SHA `8cbcdd12277393999231afafa48d91684232f0ef9408fa5c1b430a74e8b0df57`，新SHA `99e006edfc2a5c5a31d7f57dea6ff39e61ac027521ab5b8973fab8a56bdb0817`。仅修改原`processes`可选诊断sink、observer传参及failurefact输出；原调用默认行为、exact `tail -f <ownedwork>/tail.txt` 匹配与返回对象不变。

复用每轮已经读取的default ps raw，诊断额外筛选executable为tail且argv含独占work路径或PPID等于实际sdk.process.pid的少量行。最多记录初空与首个新ownedPID集合两个快照，仅新PID额外wide ps一次对照。宽松诊断条件从不成为成功PID或signal授权，也不输出完整进程列表、环境或无关进程命令。失败时在原fact中附带pending快照，并用固定 `TOOL_PID_OBSERVATION_FAILURE_JSON ` + json.dumps(ensure_ascii=False)、flush=True输出完整事实，原返回和断言不变。辅助诊断ps错误仍独立记录，不能遮蔽主失败和原task结果/异常。

预算10秒、len1、再次samePID/PPID/command校验、SIGTERM、原调用结果、物理进程消失与全部逐行保护文件oracle保持。不修改工具timeout、SDK生产、Host/框架取消或终态权威，不接受零PID，不跳Linux，也不根据后见结果默认成功。方案先独审PASS，再由原task在Git7d0新鲜路径check0后应用；实际源码逐字节等于已审候选，Ruff check/format check均0。

## 有限验证

三个无Host用例直接AST提取实际候选函数：正常matcher返回与原函数相同；合成 `/usr/bin/tail` 或截断路径只进诊断、仍不进成功结果；初空/首新PID快照上限与wide调用次数保持；辅助ps失败时完整marker保存原reason和ToolResult。这些是纯解析与取证反例，不冒作真实Linux进程证据。

新源码的一次完整本机Python工具case实际1方法通过，49行/48次真实Host调用、五族原字段/错误和三个受保护文件逐行oracle全部保持。两个shell真实before单进程、after空，timeout/permanent类别及完整recovery/side_effect/postcondition不变。`native/results.json` SHA为 `1ad7f3aaab6c843c5369d18d254eaa1eac4d21f28cf85005068ac4265cc0d92f`；原日志870字节，官方收据exit0、非截断。四官方research结果仍是当次外部观察，不用数据稳定性替可用性合同。

运行使用 immutable Host `0f1ecf23a2875621caf5128327046cbf6592ce61665c6c6872aed33fdf1931df`，实际Darwin/Python3.12.4与ACP0.12.1，**不是**Linux CI Python3.12.3。成功路径不生成failuremarker，不能因此说已看见Linux真实失败fact。执行前源码已冻结；另一个Git7d0观察在命令开始后采集，`native-running-source-observation.json`明确其时点，未伪造启动HEAD收据。此局部native已终态并释放租约，无后续Host、Cargo、Maven或完整脚本重复。

22份原件逐字节复制，包含旧/新测试源、精确patch、提案时期状态、实际pure/native收据与小输入/结果。`archive-manifest.json`绑定原始路径、字节数和SHA；`SHA256SUMS`覆盖除自身外各成员。旧35文件包不回改，提案manifest的未实施字样保留其历史时刻。当前切片源码审查与有限本机验证不关闭原Linux Finding；下一真实CI须读取完整task结果和存活期owned-tail参数，再针对确证原因修复。完整语言/合同/main仍分别验收。

这是测试取证实现，没有公共API、产品行为、示例或website变化，不需要新增产品文案或新兼容承诺。材料复审在本次封包后进行，不把封包自身当作远端验收。
