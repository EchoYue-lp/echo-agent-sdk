# MCP 与 ChannelPlugin 候选行为证据

四个共享场景通过三语言原生 SDK 调用真实 Host，复用既有接受入口，不新增 MCP/Channel
API。正式 Java 类是公开 `McpChannelHostTest`，四个独立 JUnit `@Test` 各有独立工作目录。
原草稿 Java 使用隔离 javac/main 完成四场景，其输出不计作四个 JUnit 测试。

| 场景 | 实际结果和副作用检查 |
|---|---|
| MCP stdio | 两个真正 Python MCP server；exact tools/capabilities/resources/templates/prompts、Unicode文本和binary blob、tool call/ping；同名不同Session/Manager的A断连后PID死亡且派生alias关闭，B继续存活可调用，B自身Session关闭后PID死亡 |
| 注册McpTransport | 反向初始化/通知和关闭回调；首次关闭明确失败，精确client仍可寻址、owner/slot保留；第二次关闭成功，旧alias失效，全局4个resource slot可重新使用 |
| ChannelPlugin正常关闭 | 两个plugin实际启动自己拥有的子进程；image/file/audio/video附件的base64 bytes、Unicode filename和reply_to到反向send参数完全一致；stop_exact A仅关闭A，B仍健康，Session关闭后B进程死亡 |
| ChannelPlugin失败重试 | 首次A stop失败时A/B进程仍活；第二次stop只关闭A，A stop计数2/B0；Session关闭后B stop计数1且进程死亡 |

客户端仅实现用户注册扩展的真实协议回复/副作用，不模拟 Host 的owner、生命周期或执行
事实。MCP stdio fixture是MCP server，不是SDK Host模拟器。资源/manager/Session的handle
校验、配额和关闭仍走原Host与framework authority。

附件含业务字段 `kind=image/file/audio/video`。现有WireValue codec允许未知协议kind包装
透传，因此测试用公开的显式WireValue.Map区分普通业务JSON；附件内容没有被改写，原始
四种bytes、filename和reply_to逐项比较。不能据此声称任意native plain object自动roundtrip。

所有正常场景记录与真实反例分别归档，见
[evidence/sdk-mcp-channel-20261013/archive-manifest.json](evidence/sdk-mcp-channel-20261013/archive-manifest.json)。
原四场景冻结Host为 `cb9b7fcad3946d9428cdef5a7b7258a334ee387ed34ca0612b4f40b262ef20a2`。
three-language-proof只比较明确共同投影：9项MCP返回值和进程清理事实、transport关闭顺序/
次数和slot复用、channel start/send参数及stop计数/running。随机PID、handle等保留原始值，
不要求跨独立进程一致；没有把局部投影全等冒充每项callback均已比较。

正式测试由原npm test文件glob、pytest测试目录和Maven Surefire自动发现。设置
`ECHO_AGENT_SDK_REQUIRE_HOST_E2E=1`时Host缺失或空字符串立即失败。各场景原始结果、MCP
请求与PID记录保留在独立临时目录，没有修改实际用户仓库或添加Agent权限门控。

**原冻结 Host 的反例与剩余边界**：

- 真正ChannelHealth reverse callback内请求同manager的ids，外层manager mutex持有到
  callback返回，造成循环等待。实际反例在定界观察门后释放callback，ids才立即完成。
  这是保留的历史红色证据；下述后继 owned-operation 场景使用新 Host 验证修复，
  原四场景不曾豁免此反例，也未把失败测试 skip 为绿色。
- 当前accepted MessageHandler注册并未取得真实入站consumer。plugin只收到handler_id，
  没有接受的client→Host入站入口。测试不伪造handle/stream/reply回调；相关Finding不关闭。
- caller取消held close后的清理、全部通知与注册/撤销失败、MCP工具派生alias全生命周期、
  所有channel并发路径及断连重启矩阵尚未证明。

原冻结 Host 的框架 Box 引用不能在 manager 锁外调用。后继候选依照框架
`ChannelManager::get_owned/start_all_owned/stop_owned/stop_all_owned` 提取原 entry 的引用或
future；SDK 在短 guard 范围结束后再 await，覆盖 start、stop、stop_exact、stop_all、health、
send 及 resource/Session close。metadata 固定为注册时声明；原实例、handler 和已停止事实
继续归框架同 entry。SDK 不新增插件 map、不按 name 重建 proxy、不复制批量清理循环。

候选新增一个三语言真实 Host 场景：start/send/health/stop 回调内等待同 manager ids/list，
A health 停驻时 B 同 manager 及另一 Session 的 channel 可操作；resource close 首次失败
保留同 owner，成功停止的 B 不重复 stop，重试成功才释放三个资源 slot；Session close
回调重入应由既有 closing admission 快速返回 closed_handle。关闭后的拒绝与活动期间
metadata 可读是不同结果，测试分别断言。Session 的 admission 检查必须在等待 lifecycle
read guard 前执行，并在取得 guard 后再次检查，避免 close 持 write guard 等待 callback。

`ExtensionChannelMessageHandlerProxy` 没有 close override，也没有 accepted close reverse
wire；不伪造三语言 handler-close 回调。原 MessageHandler consumer Finding 继续开放。
同 connection 的 extension channel_id 唯一，独立 Session 场景使用不同 channel_id，不能
宣称已证明两套同名 ChannelPlugin extension。原同名 MCP manager 场景保持不变。

候选与旧 Host 的真实红归档到
[evidence/sdk-mcp-channel-20261013/owned-channel/manifest.json](evidence/sdk-mcp-channel-20261013/owned-channel/manifest.json)。
后继 immutable Host SHA256 为
`7bbd1569367b0b1004fbe1cbc537b6869a2bc685bb1ca14e63228b1746aee3b2`。
TypeScript、Python、Java 各五个真实场景均通过，零失败、零错误、零跳过；Java 由
Surefire XML 核对五个独立 JUnit 测试。新 owned 场景三语言共同投影一致：40 条 metadata
回调结果按原 owner/operation/phase 比较，精确 stop 次数 A4/B3/other1、A 停驻期间
B/other 独立可达、三个资源 slot 复用、入站 callback 计数 0。跨 channel 回调顺序
不由注册表承诺，因此没有把随机批量顺序当作跨语言等价要求。全部 18 个测试子进程
在执行结束后已死亡；原始 PID 和独立目录保留。

首轮新 Host 执行曾得到 Node 四通过一失败、Java 四通过一错误：测试误用仅查询 exact
signature 的 `invoke` 调用 family catalog 中的 `facade.resource.close`，请求尚未发到
Host。修正仅使用现有公开 `request`、catalog resolve 的真实签名和 typed resource
handle；没有修改生产路径、复制 digest 或增加 wire。失败记录永久保留。Python 以
`uv run --no-sync --group dev python -m pytest` 与新根 `PYTHONPATH=src` 执行，避免迁移后
旧脚本 shebang/旧 editable 路径；曾中断的同步依赖网络等待也保留在记录里。

新执行使用迁移后的 `lp-agent/.worktrees` 工作区，日志和最终临时场景目录均在该根内；
旧执行的实际路径没有重写。Java 首次 fork 忽略 Maven 进程的 tmpdir 配置，随后通过
`-DargLine` 设置 fork 的路径并重跑五项全绿；原始目录仍按实际事实保存。
所有组合候选证据均不替代正式 framework pin、独立 build、完整门禁和精确 main 交付。


后续独立审查发现资源关闭准入竞态，原五场景的绿色没有覆盖它。Host 原
`FacadeResourceRecord.lifecycle` 只有 close 使用；捕获框架 owned start 后，真实资源
关闭服务可以 stop 并 tombstone handle，随后 start 执行使原插件重新 running。
确定性组合反例记录 starts1/stops1/runningtrue/closedtrue。这是 Host 单元组合反例，
不宣称通过公共 wire 精确调度这个内部窗口，也不当作第六个三语言场景。

资源 lifecycle 现扩为同一 read/write 权威：Channel register/start/stop/health/send 持
shared permit 至真实操作完成，锁后重新核对原 registry；close 取得 exclusive permit
后等待已准入请求，并保留原失败重试 owner。ids/list 不取该 permit。三个 Host 回归
覆盖捕获原 entry、真实 `dispatch_channels` 已准入但尚未取得 manager guard 的 start
与 close 排队，以及等待 permit 后的原 handle tombstone 重检。整套 Host 库测试
140 项通过。新 epoch 的 bin、三语言组合和 gate 另行归档；原 `7bbd1569` 证据只证明
上述第一次 owned adapter epoch，不能替代这个资源准入改动的运行验证。


资源准入后继 Host SHA256 为
`07442919df0108592f93585a76f6a9cbc429f279a81876a70b56b959f53c6c2b`。
同三语言五场景再次全部通过（各 5/0/0/0）；40 条 metadata 共同观察仍一致，18 个
子进程全部终止。Host all-target/all-feature Clippy、生产 panic 四项门禁、全 feature
Host 构建和 fmt 检查均退出 0。完整源码 epoch、原 1 失败反例、三项定向回归、140 项
Host 库测试、编译期间已修正的两个测试错误和新三语言运行记录见
[resource-admission-epoch/manifest.json](evidence/sdk-mcp-channel-20261013/owned-channel/resource-admission-epoch/manifest.json)。
本切片仍等待独立复审、全部最终门禁和已发布 framework pin；新增已批准的入站协议
属于另一个父协调切片，本切片没有添加其 wire。
