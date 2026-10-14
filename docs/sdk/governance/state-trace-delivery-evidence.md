# State、Trace 与 Delivery 的候选行为证据

正式 Node、pytest 和 JUnit 入口共执行 **9 个真实 Host 场景，全部通过，零 skip**。
本记录支持下列具体行为；不是整个 family、正式 framework pin、远端 CI 或精确 main
验收的完成声明。没有修改生产 API、accepted 合同或 framework 状态权威。

## 可复核入口与快照

- 共享输入和隔离文件故障控制：`sdks/shared/testing/state_trace_delivery_fixture.py`。
- TypeScript：`sdks/typescript/test/state-trace-delivery-host.test.js`。
- Python：`sdks/python/tests/test_state_trace_delivery_host.py`。
- Java：`sdks/java/src/test/java/com/echoagent/sdk/StateTraceDeliveryHostTest.java`。

当前场景使用不可变 Host `echo-sdk-governance-workflow-shape-host`，SHA256
`cb9b7fcad3946d9428cdef5a7b7258a334ee387ed34ca0612b4f40b262ef20a2`。
原临时公开客户端探针使用 Host `6711dd17fe3be13671ee181fce5ee79946f40963e000af293083f114301a39a5`；
两轮证据分别保存，不把原轮结果当作新二进制验证。

[原始及正式测试归档](evidence/sdk-state-trace-delivery-20261013/archive-manifest.json)
保存执行源码、共享输入、完整返回、实际持久化文件、日志 SHA、正式 Java XML 与命令。
[跨语言对比](evidence/sdk-state-trace-delivery-20261013/integrated-cb9/three-language-comparison.json)
比较完整 State/Trace DTO 和 Delivery 当前记录，明确仅归一化每个隔离场景的 cwd、UUID
及真实提交时间；时间格式、顺序和 claim 关联另有断言。原始值继续保留供复核。

每个正式测试动态创建独立临时根；Host 由 `ECHO_AGENT_SDK_HOST` 提供。
`ECHO_AGENT_SDK_REQUIRE_HOST_E2E=1` 且没有 Host 或仅空白 Host 时明确失败；正常语言完整门禁已有该设置。
Node 的 `test/*.test.js`、pytest 的 `tests`、Maven Surefire 的 `*Test` 默认入口会发现本次新测试。
共享 fixture 同时进入既有 Ruff 范围。不需要增加生产依赖或第二套状态机。

## State：身份、文件权威与恢复

保存的 scope ID 与 checkpoint `conversation_id` 刻意不同；`state.checkpoint.get`
使用后者对应的 runtime identity，`state.runtime.list` 使用 scope 归属。完整返回与输入
比较保留 messages、legacy plan artifact、skills、blocked reason、cwd 和纳秒时间。

把已启动 Host 的 `_runtime_owners` 目录替换为普通文件，save 返回 I/O 衍生的明确错误；
恢复目录后核对原 owner 文件未提交新内容，同一 Host 重试成功。损坏指定 owner JSON 时，
get 返回错误而不是假装记录缺失。错误后恢复原始字节，再继续正常调用。

清理 scope A 后，A checkpoint 消失，B 的完整 DTO 和权威 owner 文件保持。
删除 B 的 `_scope_index` 投影后，枚举仍从 owner 权威得到准确 runtime ID。
关闭真实 Host，再创建第二个 Host，generation 为 2，B 的完整 DTO 与文件内容一致。
此过程不是同进程缓存的 reopen。测试不宣称跨所有 runtime 的 scope clear 是原子事务。

## Trace：完整记录、脱敏与真实重启

在 JSONL store 保存包含 Unicode 关联身份、typed events、nullable 字段、非零
usage/cache 计数、纳秒时间及 `9007199254740993` 毫秒值的完整 Run。
加载的整条 Run、list_session summary 与真实 JSONL 首条记录逐字段比较；合成 secret
按 framework retention 策略脱敏，身份和计数不丢失。

配额设为 1。普通文件阻塞目录造成 open 失败后，合法 open 仍成功，证明失败发布不占
资源配额。已打开目录被阻塞时，save 失败且已知 Run 缓存与原文件内容保持；恢复后同一
store 重试成功。关闭真实 Host、启动第二代并重新打开 JSONL store 后，完整脱敏 Run
仍相等，不依赖第一进程的 Weak registry/cache。

这只覆盖选定事件、optional 字段和故障。它不自动证明所有 trace event、retention 分页
或实时 collector 会写入本 facade store。

## Delivery：当前 exact claim 的唯一权威

并发 `claim_next` 在 effect 尚未开始时可以返回 attempt 1 和 2；framework 原有合同允许
未开始的 claim 被更新 attempt 取代。唯一权威指 **当前最新 exact claim**，不是只有一次
RPC 能成功。旧 claim 的 transition 报明确 stale claim 错误，记录和 journal 水位不变。

最新 claim 进入 effect_started 后，两个后续 strict claim_next 均返回 null。
显式 defer 后重新领取 attempt 3。使用旧 claim 进行 transition、defer、settle 均进入
framework 的 exact claim 校验并被拒绝；before/after snapshot 和同一 ledger 的 recover
sequence 完全一致。有效 claim 结算为 Completed，recover 保持相同记录，重复 recover
仍得 sequence 8。

语言通用 native projection 保留整数为规范十进制文本。重复请求时，测试仅对已知
`attempt: u32` 字段使用公开 `wireU64` / `wire_u64` / `WireValues.u64` 构造器，经范围检查
显式编码，其余 claim 字段保持原样；没有放宽 Host 的 string/u32 解析或新增 DTO/API。
原先“一次 RPC 成功”和“plain native claim 可直接重编码”的失败假设及完整日志继续归档，
不将其描述为生产修复或以反序列化错误冒充 stale claim 证据。

当前 ledger 后端是内存 journal/checkpoint；本证据证明同一实例的恢复和 exact identity，
**不证明跨进程 crash durability 或外部效果 exactly-once**。

## 验证与交付限制

实际 focused 命令：Node 指定新 test 文件，pytest 指定新模块，Maven
`-Dtest=StateTraceDeliveryHostTest test`。三语言各 3 项通过；Java 原始 XML立即归档。
共享 Python 文件通过 Ruff check/format；Java 的既有 SLF4J no-provider 通知保留在日志中，
不是编译警告或被隐藏的失败。

既有语言完整脚本会发现新入口，但本记录没有把 focused 结果扩大为最后一次全套语言门禁。
这些候选还需独立复审、正式 published framework dependency/source 验证、最后快照门禁和
精确 main 交付。示例与官网本次无新生产行为要同步；后续正式 pin/文档投影仍按整体交付对账。

Required Host 门禁另有六个明确反例：三语言分别在缺失、空白 Host 下实际触发失败。
这一 guard-only 增量保留正常九场景的原始执行源码及 XML，不将旧 XML 改写成新源码执行；
最终完整语言门禁仍需覆盖该最后增量。
