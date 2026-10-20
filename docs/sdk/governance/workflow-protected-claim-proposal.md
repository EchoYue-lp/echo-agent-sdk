# Workflow protected claim：A 已接受，实施与验收待完成

用户于2026-10-16明确选择 **A「保留能力并实施保护合同（推荐）」**，由根任务转交本切片。此裁决接受下述一个新 capability、一个 reverse operation、受保护 claim 的 CRUD 限制及 owner 丢失后可能长期阻断的可用性代价；不是已实现、已运行或已通过门禁。B 未选择，不再作为当前待答问题。

裁决绑定独立审阅的原提案 SHA256 `23c5a59ae405ecd71c3e79b6d945e77e8b7927e0e1f0176ad91047147341a09f`（7,288 B），正式记录见 [人的裁决 Evidence](../../../.echo-semantic/evidence/evidence.sdk-workflow-protected-claim-human-decision.md)，实施沿 [ADR0039](../../adr/0039-sdk-cancellation-observation.md) 和唯一 Design/Plan。新协议先取得原实现的真实反例，再修改；最终独立复审、适用门禁、精确 main 仍待。SDK direct-run 投影 Finding 独立保留。

## 裁决前完整提案（历史原文）

以下原文逐字节保留，仅表达当时的 Proposed 阶段；其中“待裁决/尚未批准”不再代表当前状态。

---

# Workflow protected claim：待裁决提案

**Proposed，尚未批准或实现。** 现有 B 取消观察授权不自动接受下面的新公共 capability / reverse operation及受保护记录的CRUD限制。唯一问题：选择 A 增加副作用前的原Store保护并接受未知 owner 可能长期阻断，还是 B 显式延期相应恢复执行？

依据是 [Graph claim Finding](../../../.echo-semantic/findings/finding.sdk-workflow-resume-observation-claim.md)；[SDK direct-run 投影 Finding](../../../.echo-semantic/findings/finding.sdk-workflow-direct-run-observation-projection.md)属于独立、既有 B 内修复，不由本选择替代。固定公开 F 源为 [f0a6cd1 / tree9ee1](https://github.com/EchoYue-lp/echo-agent/blob/f0a6cd1c13706c20955209b698c1f891c09d78b3/echo-orchestration/src/workflow/graph.rs#L1286)，SDK build pin仍00782；当前仅有源码可达证据，没有本提案的新 Rust/Host 执行证据。

## A（推荐）：保留能力，先在原 Store 保护 claim 再执行

F 的原 Graph 及其 CheckpointStore 继续唯一拥有 claim 和结算，SDK只消费能力并无损转换；不建第二 Store、scheduler 或 receipt/query API，不用公共Checkpoint字段、label/tag夹带控制知识。

|明确增量|拟合同|
|---|---|
|Rust CheckpointStore|新增 supports_protected_claims() -> bool，默认 false；async protect_claim(&self, id: &str, attempt_id: &str) -> Result<()>，默认沿原 ack/renew 的明确不支持错误，不空成功|
|Graph能力投影（仅Rust/Host）|新增最小 supports_protected_claims() -> bool，委托该Graph实际绑定的原Store；SDK据此用既有feature_unavailable/Never准入拒绝，F同样检查。禁止拿latest注册Store替换旧Graph能力或解析Other字符串；不新增语言操作/receipt查询|
|注册 capability|仅 workflow_checkpoint_store 可声明 protected_claim_v1: true；缺省/false仍可注册、查询与无关操作，非法类型/其它component声明true在发布handle前拒绝；原 heartbeat/CAS 校验保持|
|唯一 reverse 增量|既有 agent_component_call 内增加 workflow_checkpoint_protect_claim，input为原 checkpoint_id、attempt_id；成功 result为同 operation 的 unit variant，不增endpoint/新errorcode|
|既有破坏性CRUD|未结算protected claim不得被delete、clear或save覆盖；原owner真实ack/requeue后才恢复普通CRUD，未保护/已结算路径不受新增限制|
|三语言/合同|TS/Python/Java 的闭合DTO、操作集合和dispatcher须显式新增该一项及cap字段，并同步唯一生成器/schema/fixture/文档；完整Rust库存不自动成为语言接受范围|

**适用范围**：所有可能执行 node 的 claim-backed Approved/Modified resume*，包括 SDK resume / resume_exact / resume_with_state，以及 branch_from 发起的新执行；不能假设只有Agent节点能返回未结算观察，也不在SDK复制节点判型。缺cap在 claim、branch_from的fork保存及Node副作用前，用既有 feature_unavailable/Never 与原operation/handle明确拒绝；F也须对原Rust调用默认拒绝，不靠解析错误字符串补分类。Deferred仍load/validate不claim；Rejected不执行node，保原claim+ack；fresh、无既有claim的run保持原入口与B生命周期，不被本提案一概封禁。branch是新fork，不能当作原unknown attempt的自动重试。

**保护与失败**：原Store按 checkpoint_id + attempt_id 原子检查并提交保护；wrong/stale attempt拒绝，同attempt幂等。Graph先取得原claim，再收到保护提交的明确成功，才能启动该attempt的Node副作用。仅返回unit而未在同Store提交保护不满足能力承诺。写失败、错误/缺失结果或成功回执丢失都不得启动Node，保原owner与不确定性，不凭timeout/EOF/heartbeat停止或absence解锁。已有protected claim不可自动过期重投，也不能由既有delete/clear/save覆盖绕过。clear必须在同一Store authority内先原子检查再整体拒绝，不能部分清除后才发现保护；原save_if_generation的active-claim冲突语义保持，不能覆盖保护。只有原owner真实结束后的ack/requeue可解除，sameattempt保护幂等不等于结算，stale/wrongattempt ack或requeue不得清保护。已物理结束的一般业务错误仍按原规则requeue，未保护的旧lease恢复规则不被无差别改写。

**持久化、重启和代价**：保护继承原Store的durability：Memory仅同Store实例，不新增跨进程持久承诺；File及声明持久的远端Store才要求跨重启保持。native Memory/File仅改变原私有claim表示，公共Checkpoint字段原样。File必须使用让旧reader明确失败关闭的版本化私有表示；只加bool会被旧serde忽略、继续TTL回收，不合格。renew/read/write都必须保保护知识。保护后owner丢失，甚至保护已落盘但Node尚未来得及启动就崩溃，也可能长期阻断；本范围不提供强制解锁、receipt查询或自动重执行。回滚不得把旧二进制直接指向保护数据、删保护位或退回旧格式求可用；需保兼容reader，或先取得原owner真实结算证明再按独审规则迁移格式；不承诺自动无损降级。

## B：显式延期该段已接受执行

暂不接受新cap/操作；上述 claim-backed Approved/Modified node execution（含触发它的branch）在claim/副作用前明确 feature_unavailable/Never，不接受后静默重投。保Store注册/CRUD、Deferred、Rejected和fresh无claim路径及其原错误政策。合同、文档与示例需明确这段已接受能力被延期；不能把B写成所有Workflow不可用或将deferred任意StepFn扩大为SDK API。代价是当前合法恢复执行在这段范围内不可用，Finding继续open。

## 验收与裁决边界

A须真实证明：protect先于effect、same/wrong attempt、绑定旧Graph后替换latestStore不换能力、写失败/丢回执零Nodeeffect、protected超过原TTL及重启仍不重领、heartbeat错误不丢原执行、真关闭后原ack/requeue且无重复副作用、旧reader失败关闭（限自动恢复及读改写路径；旧delete/clear不解码新claim，仍能破坏保护，禁止旧binary接触保护数据）、复用File既有.echo-checkpoint-store.lock下fs2独占锁完成exact-attempt CAS与持久提交而不新增锁或依赖、protected delete/save/混合clear全有或全无拒绝、同ID复用后的stale ack/requeue不得误清新attempt，以及三语真实Host能力缺失/类型错误与正常控制。B须证明拒绝发生在claim/effect前且保留路径正常。两者都须独立复审、适用门禁及精确main验收；源码调查、字段存在或既有绿色不代新证据。

取舍参考根任务已实际读取的 [Temporal取消类型](https://typescript.temporal.io/api/namespaces/common#activitycancellationtype)、[Activity执行](https://docs.temporal.io/activity-execution)与[活动失败探测](https://docs.temporal.io/encyclopedia/detecting-activity-failures)：取消请求、等待真实结束和timeout触发重试是不同事实。本项目保留原physical owner，刻意不把云系统的超时重试当作重复本地副作用的许可。此为产品可用性与恢复保证选择，当前未作风险接受。
