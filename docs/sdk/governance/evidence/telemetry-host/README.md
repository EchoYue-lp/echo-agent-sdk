# Telemetry Host 原始反例与技术候选

三个语言使用正常 Host `bcb0389ae8d8a946fab9bb123945100722711029bafa8d9d6cadf95a76c9c345` 和公开 Session.invoke，初始化前 status=false，但 init 在两秒测试观测窗口内未完成，正向成功测试各失败一项，无跳过。没有返回的 FrameworkError 或 after-status 证据，不能从静态 try_init 冲突伪造该返回。

[原始收据](receipt.json) 包含三个真实正向失败、初次配置错误、原长时间未完成及受控清理分类。[OS 线程栈](host-init.sample.txt) 与自有父子进程记录证明请求处于框架 init 的 meter-provider drop/shutdown 同步等待。尚未运行 collector，因此没有导出或 flush 成功结论。

[修复设计候选](design.md) 只提出稳定 OTel layer、原 framework runtime 委托和 Host blocking 调度组合；它不是批准 ADR，也没有改动生产代码。新三语言场景保持正向成功要求，不能把初始化超时改成永久 expected-error 绿色。Java Maven 已执行真实 1 项失败并释放；Cargo 未使用。
