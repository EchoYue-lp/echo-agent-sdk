# 独立复审范围

候选 Host SHA `06bb60d207d8abe5adfe8f0b3a609c79a3865dc69f0e79ed205c0d3b2e9d195b`。

本包只支持三语言 SearchProvider 与 Sandbox 两个有限场景组通过（各 2 项，合计 6 项）；3 个完整 test 文件的第三项普通工具组仍在 DB `describe_table` 的真实 NULL 失真断言失败，不能宣称整个工具能力通过，也不能宣称最终 published framework/main 验收。

- `manifest.json`：实际 Host、9 个 native workspace、15 个运行 source/catalog/client 文件摘要、Java 编译产物摘要及原测试统计。
- `executed-source/`：运行时实际消费者、共享 oracle、catalog/digests、Node dist、Python/Java 客户端实现；Host 源 provenance 由主线程的本候选编译收据绑定，复制客户端源码不替代该证明。
- `python-search/`、`typescript-search/`、`java-search/`：真实 Agent 两次 Run、反向 callback、settlement/replay、完整实际模型请求。成功一次；异常按框架原 retry 三次。标题/URL/UTF8 摘要/空摘要与错误进入真实 Tool message。
- 三个 `*-sandbox/`：每语言正常 `printf`、pending callback cancel、pending callback close。真实 PID、exit/finally、独占 stream identity、正常 `item/output → item/complete payload → complete/EOF` sequence 1/2/3、callback cancel signal、重复 close false、Session cleanup once。callback 使用真实子进程而不是假 counter。关闭返回时和正常 EOF 返回时已经 physically joined；取消 RPC 只表示取消请求，原事实明确可能先于 cleanup。
- `bounded-comparison.json`：9 个 producer 的原始退出码与含义；Python/Node -15、Java 143 都是 Unix SIGTERM。没有把不同编码转换成 0 或成功退出。
- 三个 `*-tools/`：已完整运行的 49 行/48 次 public operation 调用，但正确值 oracle 尚未通过；只为完整性保留，并未进入本次 pass 范围。

反例链：上级 `context-0a0f/` 保留取消信号未传到 pending callback 的真实失败，fixture 5 秒 watchdog 后 finally kill 不能视为 Host 取消成功；`settlement-f852/` 保留正常 terminal 后额外轮询未 fuse 的 Unfold 造成 producer panic，继发 closed-handle 是下一次操作结果。普通 `sandbox_cleanup` 最初未被夹具识别、Java cleanup 误加 finally counter 都属夹具错误，已明确修正，不当生产缺陷或伪通过。

对应命令原输出在上级 `python-fused-candidate-first.log`、`typescript-fused-candidate-first.log`、`java-fused-candidate-first.log` 及 `java-fused-candidate-first-xml/`。三语言全文件统计均为 2 passed / 1 failed / 0 skipped。最初所需 Host 缺失时全文件报错，不以静默 skip 代替 required Host 证据。
