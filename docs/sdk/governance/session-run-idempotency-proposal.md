# Session / Run idempotency 合同提案

状态：**Approved — 用户明确选择 A；实现与验证进行中**。对应 `finding.sdk-session-run-idempotency-contract`（open / Important；机器类型 `intent_gap`）。此提案只处理已存在的两个 `idempotency_id` 参数，不扩大 Rust public identity 的三语言合同。

## 事实与风险

`SessionCreateRequest`、`RunStartRequest` 接受并校验此字段（`echo-sdk-protocol/src/methods.rs:362,651`），但当前 `session_create`、`run_start` 未读取它（`echo-sdk-host/src/core_profile/handler.rs:262,865`）。公开文档只明确了 Agent create 的 key 重放。响应丢失后再次发送这些已被接受的参数，当前仍可能创建第二个 Session 或再次执行 Run，带来重复模型调用及潜在重复工具副作用；本反例实际证明模型回调重复，不假称已执行重复业务写入。

真实 Host SHA `193e49b8557db9b746a1cc8c6f5cf30bb6d3154a630562f7ef8551b949e878d1` 的四个有界场景：

| 请求 | 当前结果 |
|---|---|
| 同 Agent、同 Session key/payload 重复；同 key 改 cwd | 不同 Session/ACP identity；改 cwd 也创建新实例，无冲突 |
| 原 Session 关闭后同 key 重试 | 又创建新 Session |
| 同 Session、同 Run key/input，前次完成后重复；改 input；公开 RunHandle.close 后重试 | 四个不同 turn，四次真实 `llm_chat` 回调；改 input 不冲突 |
| 原模型回调被 barrier 保持时，同 key 并发重复 start | `invalid_request / after_delay`：已有 active Prompt Turn；未重放原响应，未产生第二次回调 |

原始 `facts.json` SHA `d1658d4c77694b7f160d2c51874a72a4570cdf649e16a9c212de71d1bb2c0a8e`；[最小反例包](https://github.com/EchoYue-lp/echo-agent-sdk/blob/0ebe15f66d0aa57c7b7a92ae68cd1be106114ae9/docs/sdk/governance/evidence/session-run-idempotency-counterexample/README.md)保存原请求、响应、错误、回调、结算和 Session index。首脚本误发不存在的 `run/close` 已另存，不能归为产品缺陷。发现时 checkout 为 `76601b242d7c354352f3b5c7d4bf099873746b14`；二进制与运行源码按 SHA 绑定，**不宣称该 Git commit 已包含整套测试源码**。

## 推荐 A：保留参数并限定重放语义

保留两个已接受参数。使用 Host 既有 Registry 所属连接生命周期中的 publication mapping；框架继续唯一拥有 Session/Turn 执行、调度、取消和终态。mapping 只保不可变的原响应或原已执行失败响应、canonical payload 与 issued resource identity，不拥有第二运行状态机。

- **key 作用域**：当前连接、handle generation、操作名、原 owner identity、原 key 的精确字符串。Session create 的 owner 是 Agent handle；Run start 的 owner 是 Session handle。另一操作、owner 或 generation 不共用 key。
- **payload**：在原协议校验/解码后，包含全部有效 Session 参数（含解析后的 working directory、显式 Session identity）或全部 RunInput。排除 key 本身；按确定性字段顺序 fingerprint。仅按既有协议 Option 解码等价处理 absent/null，不改变 WireValue 类型、Unicode、路径或数值。changed payload 在任何新副作用前返回既有 `invalid_request / never`，保原 mapping。
- **首次与重复请求**：首次在新建/执行副作用前预留 key，由原连接生命周期任务及既有 cleanup/settlement owner 持有唯一首次 publication job，不能仅靠 RPC responder 或等待者保活。并发同 key 同 payload 观察同一 publication；完成后返回原完整 SessionCreateResponse 或 RunStartResponse，包括原 handles/stream/first_event，绝不另 mint 或重发通知。**不新增 Host 等待 timer 或错误码**；观察者沿用 ACP/client 原有超时和取消行为，失去观察结果只结束该等待，不清除 key、不取消已开始的原工作。duplicate waiter 取消也只结束自身等待。未提供 key 的行为保持原语义。
- **取消、失败与唯一 owner**：原首请求在副作用 admission 前被取消/中止时，由首次 job 的 owner 释放 reservation 并通知所有观察者；不留下永远 Pending 的 key，后续合法请求可以重新 admission。job 尚未被连接任务接管的 spawn/admission 失败也走此释放路径。副作用一旦开始，首 caller drop/cancel 不能销毁 publication owner；如需取消已开始的 Run，调用原框架取消/cleanup，owner 保持到真实结果或清理完成。完成的结果/失败以不可变 receipt 留在原 key，同 key 不能重新执行。验证或 admission 失败且未开始副作用时返回原 typed error；响应传输丢失不撤销原已成功 publication。这里的 reservation/通知只协调请求 publication，不判断 Run 业务终态。复用 CoreProfileState::begin_settlement/finish_settlement/wait_settlements（state.rs:381/385/409）及原连接关闭 owner；任何退出路径必须完成同一次 settlement，不能让取消的 waiter 留下无人持有的首次工作。
- **完成与关闭**：完成的 Run 仍重放原响应。TS/Python/Java 的本地 RunHandle.close 本身不释放 Host issued Run，因此不使 key 失效。原 Session/Run 或 owner 真正关闭/撤销后，重复 key 返回 `closed_handle / never`，**不创建新资源、不返回无法使用的旧成功 handle**；closed owner 校验优先于 payload 冲突。该 tombstone 在当前连接/generation 内保留。
- **限额与恢复边界**：使用现有资源预算限制 key entries，达到上限在副作用前返回既有 `payload_too_large / after_delay`；不静默淘汰仍需重放或防复活的记录。连接 teardown 先 fence 新 admission，再通过原关闭 owner 取消并等待已拥有工作的实际 settlement，**只在这些 owner 结算后丢弃 mapping**；如原有 bounded close/drain 未结算，保留 key、cleanup owner 和既有重试地址，不把观察超时当作业务完成或丢弃 receipt。EOF/caller drop 不能颠倒此顺序。重启/新 generation 清空新连接的 key scope，不承诺跨连接或跨进程持久幂等；旧进程未结算 owner 的处置仍由原关闭机制负责，不将其假称物理结束。旧 handle 按现有 generation 校验拒绝，不自动恢复旧执行。

这是 request admission/publication 合同，不是任意 RPC 的 exactly-once 保障，也不决定工具内部副作用是否可撤销。不得在 SDK adapter 新建 scheduler、重复执行重试器或终态权威。未新增 wire 参数、错误枚举或三语言 API；公开合同文档和实际三语言消费证据仍需在实施后同步。

## 备选 B：显式延期两个参数

保留现有请求形状，但任何非空 `idempotency_id` 在副作用前统一返回既有 `invalid_value / never`，明确消息为该操作尚不支持此参数；不再接受后静默忽略。合同、文档及接受账本显式标记这两个参数延期，未传 key 的原操作继续可用。该选项修改现有接受承诺，必须由用户选择；不能由实现自行删除或默认批准。

## 依据、取舍与验收

已有 Agent create (`handler.rs:118`、`handles.rs:364`) 使用 canonical config、同 key 原 handle、changed config typed conflict、closed handle 拒绝，是本仓库可复用的 admission 模式；不复制其内部实现或假定其预算淘汰策略适合这次 tombstone 合同。

[Stripe 官方文档](https://docs.stripe.com/api/idempotent_requests)采用 key 识别重复、保存原结果，并比较参数以拒绝误复用；[Google AIP-155](https://google.aip.dev/155)以请求 ID 防止重复处理，并要求定义去重时间范围。这里取其重复识别、结果保留、明确 scope 的共性，按本地 SDK 的 owner/generation 生命周期收敛，不移植线上服务持久层或任意 24 小时期限。官方页面通过直接 urllib 获取，原 HTML SHA/位置见证据包 provenance；web 工具失败未伪记成功。

A 的验收：真实 Host 同 key 顺序/并发均只有一次创建或 provider 调用、原响应相同；changed payload 副作用前冲突；关闭后不复活；预算拒绝、响应丢失及连接/generation 边界可复核。用真实 producer barrier 分别保持首次 admission 前与副作用开始后，再取消/drop 首 caller、取消 duplicate waiter 或注入 EOF：前者 reservation 释放并唤醒、不永久 Pending；后者仍仅一个原工作，owner 保持到真实 result/cleanup，duplicate 不重启；EOF fence→结算→mapping 丢弃顺序及未结算 close 保留可重试 owner 必须可观察。 B 的验收：两个操作传 key 均明确拒绝、零创建/执行副作用，文档与合同同步。两者都须按仓库门禁和独立复审交付；本提案不提前关闭 Finding。

**裁决记录：用户直接选择 A，授权按此有界重放合同实施。** 可信来源为本 goal 中用户对 Root 提交的该问题的明确答复；Root 已向本实现任务传递批准。原待裁决最终稿 SHA 为 `1c91a8c94bbadfc1cab2d5b33edd25c3ea3887ca2e1e259975fa1697726b073b`，原字节保存在 Admin/proposal-history。备选 B 留作历史比较，未被选择。批准不等于实现完成或 Finding 关闭。
