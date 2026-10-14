---
title: Java 原子进程 stdin EOF 与有界关闭
artifact: design
carrier: markdown
---

# Java 原子进程 stdin EOF 与有界关闭设计

状态：已完成独立设计审查（Critical 0 / Important 0 / Minor 0），由 Root 在原 goal 授权范围内批准限定技术修复实施（2026-10-14）。仅同步状态，正文语义保持；实现与验证结果以独立收据为准。

## 问题与目标

现有 Java `closeOwned` 正确保留 factory 构造、callback cleanup、取消路由和可重试 close owner，但末端官方 `StdioAcpClientTransport.closeGracefully()` 先 TERM 直接子进程。真实 telemetry public-close 反例在 exporter ACK 被 held 时 100ms 内已返回，实际 Host 仍 alive；终止 wrapper 不等于 Host 完成清理。原证据是 `admin/telemetry-host/java-publicclose-first.log` 及 `native-java-close/java-public-close-8754400370932725132/wire-observed.json`，属于故障证明，非成功关闭证据。

目标：通过原 public `closeAsync/close`，使已接受的 callback/请求帧真正写出，再关闭同一子进程的 stdin，让 Host 从正常 EOF 进入原 shutdown 链；成功必须等待该 Process 真正退出。保留原 transport 五秒强停 fallback、失败重试及初始化取消/意外退出行为，不改变 framework/Task/telemetry 状态或 wire。

## 复用依据与官方实现事实

锁定依赖为 `com.agentclientprotocol:acp-core:0.17.0`。本机二进制 SHA-256：`daa9487c594817032f218aaa2b2707731a8872caedfc11faec0137d1441e2f47`。从 [Maven Central 精确版本源码包](https://repo.maven.apache.org/maven2/com/agentclientprotocol/acp-core/0.17.0/acp-core-0.17.0-sources.jar) 只读核对的 SHA-256：`a9d0217176697808a3fc4caffd8fcf1fd2ef4a61607826160020b56998223a82`，并以实际 jar 的 `javap -p -c` 复核。

| 官方源码位置 | 已核事实与取舍 |
|---|---|
| `client/transport/StdioAcpClientTransport.java:362-392` | 先 complete sinks、`Process.destroy()`，再 waitFor 五秒/强杀。不是 stdin EOF。Process 是 private。 |
| 同文件 `312-352` | protected `handleOutbound` 只收到消息 Flux；实际 OutputStream 在 private writer 内取出。无 closeInput/Process getter；final ProcessBuilder 不能拦截 start。旧 subclass 无安全 EOF 入口。 |
| `agent/transport/StdioAcpAgentTransport.java:109-168` | 公开接收 mapper、InputStream、OutputStream；start 的 handler 签名与 client connect 相同，内部只处理通用 JSONRPCMessage，没有 Agent-role 业务分派。start 的 CAS 在方法调用时发生，不能反复调用或订阅时重建 Process。 |
| 同文件 `227-284` | 原官方 writer 持传入 OutputStream 的 monitor，写完整 JSON 行并 flush；sendMessage 仅入队，不等物理写出。 |
| 同文件 `288-314` | closeGracefully 结束 sinks/调度器，不保证排空输出；awaitTermination 只证明输入 reader 结束，不能证明 Process 已退。 |
| `spec/AcpClientSession.java:174,388-401` | client handler 处理后返回 empty Mono；官方 agent transport 不会再生成一份 RPC response。session close 只 dismiss pending RPC、drain 本地通知，不关闭 Process/transport。 |

继续使用原 `AcpClientSession`、`AcpJsonMapper` 和官方 stream transport 的 codec/framing/writer；标准 Java Process/OutputStream 提供唯一进程与 EOF。既有 [ADR 0034](../../../../adr/0034-sdk-governance-authority-and-lossless-adaptation.md) 的 close owner、失败保留及 [ADR 0036](../../../../adr/0036-telemetry-host-composition.md) 的真实 flush/退出义务继续适用。本设计是这两个既有合同下的本地 transport 修复，不另建 ADR、serializer、JSON-RPC 引擎或公开 transport API。

## 边界与核心结构

在原 private nested `OwnedStdioTransport` 内组合 `AcpClientTransport`，保留一个 Process；由原命令、参数、环境通过 ProcessBuilder 只启动一次。其 stdout/stdin 交给上述官方 `StdioAcpAgentTransport`，connect/start、sendMessage、unmarshalFrom 和版本均薄委托。不能保旧官方 client transport 再启动第二个 Process，也不能反射取其私有字段。

原 `closeOwned` 的 factory 构造/清理、回调取消、资源 tombstone、closeStage、失败重试和 dependent future 结构保持。只调整 native transport 阶段及必要的 expected-exit 标记调用点。新私有信息只表达同一 transport 的帧写出、EOF、Process 退出及 I/O 释放，不表达 Host 消费、Run terminal、telemetry flush 或外部 producer 结算。

现有 dependency 足够；不改 Protocol、generated JSON、catalog、框架 pin/lock、五秒 Task 政策或语言 API。源码范围限 `EchoAgentClient.java` 的 nested transport/必要 close 调用点；直接测试为原 `HostEarlyExitTest`、`JavaClientCloseLifecycleTest`。`TelemetryHostTest` 继续由其 owner 用原真实反例复验。

## 写出完成与 EOF 的严格顺序

- 每个首次订阅并获 native transport 准入的消息只建立一个 retained write receipt；同一发送 Mono 的重复订阅/调用者取消不能重复发送或取消已接受写入。
- 在短锁内为原消息关联一个序列化的私有 future tail；一次只将一个当前消息委托给官方队列，后一消息待前一帧成功 flush 才委托。尾链不是 RPC/运行队列，不解析方法/消息 ID，也不替代官方 codec 或 writer。
- 把标准 OutputStream 观察包装器直接作为官方 delegate 的 outputStream，包装器继续写同一 Process stdin。官方唯一 writer 持该包装器 monitor 写行并 flush；只有底层 flush 成功才能完成当前帧 receipt。写/flush 错误保留原异常并使尾链失败。close、EOF、取消、队列入站及强停绝不触发写成功钩子。
- connect 的唯一启动/ready receipt 是尾链的起点；冷启动期间的发送不能越过 ready 或被并发 ready 回调重排。锁不跨异步等待；完成回调不得在输出 monitor 中递归执行下一帧/close 的用户逻辑。
- native close 在同一准入锁内停止新帧，截取最后一个已接受 tail。原 callback terminal、factory close response等已接受帧都必须完成该 tail。只有成功排空后，才在同一个输出 monitor 内关闭实际 stdin；不是先关 delegate 再假设它排空，也不加额外 RPC barrier。
- 官方 delegate 的 reader 保持到 Process 实际退出，避免 Host 正常关闭时的最后输出堵住。最后再释放 delegate/I/O；其关闭不是 Host 结算证据。

## 生命周期与失败

| 场景 | 目标行为 |
|---|---|
| 启动与重复订阅 | 单一 cached/retained connect 只调用一次 ProcessBuilder.start、一次 delegate.start；其方法级 CAS 不被第二次调用。重复 connect 不创建另一个子进程。 |
| start/delegate startup 失败 | 无 Process 时直接保原失败；Process 已创建则同 owner 关闭其 stdin/退场，必要时按既有 native deadline 强停，并释放已创建 I/O。初始化失败/取消等待者不能遗失该 cleanup。 |
| 意外退出 | 复用原 failure first-cause 与 events/updates/RPC 投影；只有同 Process 的一个真实 exit observer。delegate reader EOF/awaitTermination 不能冒充 Process exit 或业务 terminal。 |
| stderr | 同 Process 的 stderr 持续 drain，按既有诊断日志策略处理，不并入 JSON-RPC/stdout，不新增 endpoint/secret 日志。drain 任务/流与同 owner 退出清理，避免管道填满或遗留读线程。 |
| 正常 close | 原应用清理和本地 session 通知 drain 后，native 阶段依次停止新准入、完成已接受帧、stdin EOF、等待原 Process 实退、释放官方 delegate。成功 future 只能在这些真实事实完成后返回。 |
| expected close 标记 | 不能在 factory cleanup 或仅“开始 close”时提前设置。只有本 owner 真正执行 EOF/强停时才进入 expected exit；EOF 前意外退出仍报告真实 transport failure。 |
| native 五秒边界 | 原 transport 阶段的五秒覆盖其写出 drain、EOF 与退出等待；不把它变成 factory/Task 的新全局预算，也不因多个 close waiter 重置原 attempt deadline。到期才强停同 Process；未写出帧显式失败，不能声称 telemetry flush 成功。 |
| 强停未完成 | 发出 kill 不是退出证明。成功仍须实退；若到期返回既有 timeout/transport 错误，保留同 Process observer 和 cleanup owner，之后 retry 同一资源，不能另启动 Process 或清空未完工作。 |
| 失败与重试 | 沿用现有 EchoAgentException code/retryability/operation，不新增 wire 错误。首个失败 future/cause 保留。EOF、真实 exit 和已成功 I/O 释放只执行一次；未完成释放可重试。资源已全部真实释放后的重复 close 幂等；任何后续本地清理成功不得把先前失败改写成成功写出/flush。 |
| 调用者取消 close future | 只取消该 dependent wait；原 closeStage、写出 tail、Process 观察和清理继续由原 owner 持有。factory 清理先失败时，native EOF 尚未开始，原构造/清理重试语义不变。 |

原 Java early-exit 限制继续准确：Process 退出与“stdout EOF 但子进程仍活”不是同一事实。直接 Process 保证也不是任意孙进程清理保证；telemetry wrapper 场景必须额外核对原真实 Host PID/ACK/退出，不能用 wrapper 已死替代。

## 验收标准

1. 原 telemetry public-close 真实反例：ACK held 时 close 未完成且原 Host alive；release 后 collector 收到原数据、原 Host 退出、public close 完成。直接 Host 的进程实验还须证明 EOF 先于 TERM/强停，五秒内正常退出无 TERM。
2. 受控官方 writer 真实输出实验：在一帧写/flush 中 held，再调用 public close；EOF 不越过已接受 callback 帧。release 后字节逐帧完整、原 terminal notification 写出、同 Process 收到 EOF；flush 错误和五秒强停不能完成假写 receipt。
3. 原九项 early-exit、close retry/dependent-cancel、ACK submission、stream cancellation route、正常 Host/Protocol 关闭回归全部保。新 owned Process 的测试只能观察原真实身份，不给生产添加 getter 或反射。
4. 双初始化订阅/start 失败、stderr 超过管道容量、退出与 close 同时发生、多个 close waiter、写尾链失败/强停重试均有有界事实；无新进程、留存 native task 或官方 subscriber 泄漏。
5. 先原反例红→修复后同场景绿，记录官方依赖 SHA、源码/类/Host SHA、原字节日志和真实 PID/出口。设计审查通过后才实施；完整语言门禁与最终 published framework pin/SDK main 验收另由 Root 调度。当前文稿不宣称修复已验证。
