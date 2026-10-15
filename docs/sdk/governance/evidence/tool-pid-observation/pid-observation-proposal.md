# 工具 PID 观察调度反例与修复候选

Status: proposed；尚未修改 SDK，原 Linux CI 确切失配原因仍未确定。

## 原失败与反例界限

原 Linux CI722 的 Python shell-signal 场景在 test_tool_value_effect_host.py:113 断言 `len(pids)==1` 时得到空数组。原日志没有当时 task.done、实际请求结果、原始 ps 行或单调时间，不能据此说工具没有启动或已经清理。同一个 CI 的 TS49行/48调用场景使用同样 ps 参数与前缀匹配且通过，反驳“一律 Linux ps 截断”的结论。实际 Mac205字节 ownedtail argv 的 default/COLUMNS80/wide 四组合都能发现 PID，也不能冒作 Linux native 结果。

受控真实Host反例使用既有 immutable0f1 和原 public Session.invoke/shell-signal5秒参数。只在公开调用之前加入 Admin asyncio.Event 调度gate，模拟合法 actor/eventloop排队；没有修改生产 transport、Host、工具实现或协议。旧50轮在 gate尚未准入时任务仍pending，默认与wideps均无ownedPID，原len1断言稳定失败。实际50轮耗时2.592508709秒；0.5秒仅是50×10ms休眠之和，不能误称墙钟预算。释放gate后原session.invoke实际发送，约111ms后发现唯一tail，default/wide原始行同，再次exactidentity核后SIGTERM。原工具返回command_output exit-1 / permanent / possible / verify_then_retry，215ms实际工具duration，physicalafter[]和三个protectedSHA全部相同，clientclose后亦[]。原pending任务未被测试过早取消。

这个反例证明：固定50个全局ps样本不是SDK工具5秒执行合同的准入或创建完成条件。它没有确定CI722是排队迟延、早退typedresult或ps表示差异，下一Linux同case仍需保具体状态诊断；不伪将本Mac反例写成原Linux已复现。

## 最小原测试内候选

仅原 Python 测试当前 watch_process 分支，把固定 `for range(50)` 替换为已有测试中 `asyncio.wait_for(condition_coroutine(), ...)` 模式：条件是当前独占路径的PID出现，或同一原invoke任务settled。保10ms让步，不新增sleep作为成功触发、不更改工具timeout本身、不跳Linux、不接受PID零。观察时间耗尽必须失败并保存状态。

现已实核：Python ACP0.12.1 `Connection(receive_timeout=None)` 默认无请求完成上限，send_request直接await原future；SDK spawn没有传入receive_timeout。因此不能虚构30秒默认requestdeadline，也不能把工具执行5秒作为公开请求排队5秒上限。Root现明确选择测试观测watchdog复用该fixture既有10秒ready条件预算，单独覆盖request admission/owned producer observation；与工具1/5秒执行计时独立。它只让测试失败有界，不声称限制产品合法排队，SDKcap仍需审准具体候选。无PID而task已done则保实际ToolResult/typedexception，然后原len1断言失败；不可把结果default为0或把提前失败当成功。超时需只结束observer，不隐式cancel原工作；原测试失败清理通过SDK.close与原task结算/显式cancel只作失败后的owned cleanup，不能把取消当成功证据。必须先在Admin设计审明确这一新测试watchdog来源和语义，再写原测试。


一旦发现，原 `len(pids)==1`、再次同PID/PPID/command核对、仅向自己的PID发SIGTERM、await原task、完整ToolResult分类/退出码、physicalafter[]和protectedeffects全部保留。原handler与框架仍是执行/取消/终态唯一权威，不自行生成假PID或依扫描结果控制另一个服务。

诊断只保有限observations（elapsed、task.done、PID候选）、结束时原PS/wideowned相关行和已结束的原taskresult/error，不改变结果/错误接受条件。若使用新helper，优先原文件局部私有观察函数以保持同一测试来源，避免第二processauthority；不要扩三语言生产API或目录族。新增有意义回归应在Admin公开请求调度gate真实反例上验证候选观察条件：队列仍pending时不得成功或假报PID，释放后必须strictownedchild及原ToolResult，超时/任务提前失败必须可见，清理一次物理退出。原完整native场景仍需后续同Host与最终LinuxCI核。

## 准入与剩余工作

本阶段只读事实与Admin真实probe已完成，Hostproducer租约已释放。代码修复候选需SDKcap/FrameworkFinal独审、Root明确phase4路径和freshpreflight才可写。当前候选时间预算如何与执行前排队对齐需要明确，不能单纯把50改成500或默改更大秒数。更稳健的合同来源是原公开请求的已有deadline与tasksettled，而不是进程列表样本次数；若原client请求deadline不能在该测试读取，先在Admin设计中解释有限观察上界和其失败分类，再实现。

没有新runtime或框架修改。只有Admin文件写入；无Cargo/Maven/新Host构建、SDKsource/semantic/CI/proof变更。原CIraw70548和全部after原记录保留。

## 当前可审具体候选

`test-tool-value-effect-candidate.py` 和 `test-observer-candidate.patch` 是完整原文件加仅watch_process块变化的Admin候选，真实SDK未改。Root选择的test-only10秒watchdog复用本test既有fixture-ready10秒预算；while条件只原processes匹配或task.done，asyncio.wait_for只取消observer，不取消正在运行的原invoke。无PID但task.done时，signal场景原len1仍失败，诊断保完整已完成结果或异常及原default/wideownPID行。wide只在失败诊断对照，不作成功matcher或signal授权。watchdog超时同样抛失败；finally对ownedtask cancel/await，仅作用失败后资源结算；原task正常result/exception此前已await/传播，gatherreturnexceptions不会把它转成成功。再次samePID验证、SIGTERM与processafter/oracle保原。方案还需要SDKcap读这一具体bytes后准入，不能凭本文件宣运行绿。
