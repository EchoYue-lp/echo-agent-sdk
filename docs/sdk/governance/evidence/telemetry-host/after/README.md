# Telemetry 修复候选：真实导出与原关闭结算

这是 framework `be5` 上的未发布 telemetry 切片和显式本地 Cargo patch Host。它不包含 N 的 ACP 兼容增量，不是最终 pin、完整 SDK、CI 或 main 验收。原始 `bcb` 三语言初始化挂起及 OS sample 仍在上级目录，原日志未覆盖。

## 支持的结论

框架仍由原 METRICS、TELEMETRY_RUNTIME 和初始化锁拥有 exporter、采样、一次性状态。稳定 OTel layer 使用标准 tracer 委托和 downcast；marker 与它使用同一过滤器，不能开启被过滤的 TRACE。单一 fmt writer 合并原诊断与 console filter，Host 不往 ACP stdout 写 console。

5 个隔离进程场景通过：不兼容 subscriber 拒绝且 globals 未发布；构建失败未安装 subscriber；standalone 原生命周期；组合 subscriber 的上下文/单 fmt/flush；并发 init 恰好一个成功。正常发现的 collector utility 空返回不算场景证明。组合场景验证初始化前 span 未导出、有效采样和外部 parent、导出 trace/parent ID、shutdown 边界、重复 init/shutdown。SDK 5 项 focused 测试验证错误布尔值、原 blocking work 的取消等待/连接取消信号/close barrier、真实 JoinError 保留 cause；其中 EOF 信号 unit 不能代替 wire EOF。

三语言各 console=false/true 两个正常 Host 场景通过。公开 Session.invoke 初始化后，由原 Agent 调用本地模型、原 read_file 和第二次模型消费真实文件内容，再公开 shutdown。Collector 的确认前文件快照验证每例 llm.calls=2、input/output tokens=34/10、read_file=1、8 个已结束 span 和4条同 trace parent/child 关联。指标导出周期设为一分钟；成功 shutdown 返回时已收到的快照是证据，不用事后周期导出代替。

额外的三个 wire 场景是 Host/TypeScript 范围：官方 `$/cancel_request`（从真实请求读取 requestId）后 EOF、真实输入 EOF、仅原 public sdk.close。Collector 收到真实 export 但暂不 ACK 时，status 仍可响应；关闭仍 pending、原 Host 未退出。释放响应后原父进程观察同一 Host exit=0。public_close 不注入 EOF、不捕 SIGTERM，也不伪造 reply。其最后现有成功对应中间 TS close 候选，最终 TS close source 审批后的确认仍待，见 receipt；不把 Node 证据扩展成 Python/Java 取消或关闭 API 证据。

## 来源与范围

`SOURCE-SHA256.json` 保存实际 source 与文本快照；`record-values.json` 对 shutdown 返回时保存的原 OTLP 记录执行了字段值和 parent 关联核对。`raw/` 保存全部具名实际失败、编译错误、格式及 lint 结果，`native/` 保留原始语言输出、模型请求和 OTLP 请求。

新候选曾错误地把无过滤 marker 单独组合，导致 TRACE 被意外开启；真实失败的 enabled oracle、ACP describe_chain 调试字符串热栈和慢 native 日志保留。修复后同 oracle 通过。该增量回归属于修复开发过程，不是旧已发布 Host 的原始挂起根因。`typescript-wire-v2.log` 的 public_close 子项曾人工 EOF，已撤回其验收含义；原失败及后续无替代信号的成功均保留。

构建配置使用既有共享 target、jobs=2、dev/test debug=0、incremental=0 和原 macOS rustflags。诊断 workspace 使用显式 framework path patch 和自己的 Cargo.lock；正式 SDK pin 未改。框架只新增已锁版本的测试 collector 依赖边，没有生产依赖升级。官方 `check-language-sdks.sh` 已接测试 collector 构建入口。

文档同步覆盖框架 tracing 文档、两个 README 的真实 by-value API 签名和双仓 ADR。框架无 examples 目录；当前 SDK examples 没有 telemetry 编排，原 API 无破坏性变化，真实可执行例子由三语言此场景进入 CI。官网没有受影响的 telemetry API/配置说明，不需要 UI 或内容变更。

## 已知缺口

独立实现复审、最终 TS close source 确认、N+telemetry 组合框架完整门禁、框架发布与 SDK 精确 pin、最终合同/全语言/CI/main 仍待。无 npm/PyPI/Maven 发布。这里只支持列明配置和场景，不承诺任意网络故障可靠投递、任意活跃 span 跨 shutdown 导出、或把 Rust helper 自动变成三语言 API。
