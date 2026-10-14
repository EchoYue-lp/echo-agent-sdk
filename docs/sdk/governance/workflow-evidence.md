# Workflow checkpoint 候选行为证据

本组覆盖现有接受合同24项 Workflow 操作中的17项。共享场景包含48条记录：
47次 Workflow 调用和1次 Session close，三个原生SDK分别调用真实Host，11个具名RPC错误
与合法结果、状态和清理均有对应断言。没有新增API、Store、调度器或DAG权威。

| 已证明行为 | 精确检查 |
|---|---|
| 路由与中断 | pure Router yes/no路径、gate/end前中断、path/steps和完整state |
| checkpoint | 17字段的load/restore，id/run/revision/continuation/context/optional/timestamp保留 |
| 标签、defer与恢复 | label/tags与generation推进；defer不执行、不消费；恢复只消费一次，重复明确错误 |
| branch与更新 | 原checkpoint保留；child run identity独立、graph revision相同，fork lineage关系；更新与拒绝理由无损 |
| 状态、owner和关闭 | SharedState读写/keys/snapshot；跨Session graph/state拒绝；关闭后地址失效、另一Session继续 |
| 资源配额 | 两个resource slot占满时第三次明确失败；Session close后原额度可由另一Session重新使用 |
| 非法checkpoint形状 | 两个resume入口拒绝17字段数组，原canonical checkpoint与状态完全不变 |

冻结旧Host `6711dd17fe3be13671ee181fce5ee79946f40963e000af293083f114301a39a5`
实际接受Checkpoint数组：`resume_exact`继续执行并消费父checkpoint；`resume_with_state`
还应用了指定的非法输入更新。三个SDK均取得这一红色反例。修复后的Host
`cb9b7fcad3946d9428cdef5a7b7258a334ee387ed34ca0612b4f40b262ef20a2`
拒绝两种数组输入，合法object仍可恢复。framework始终拥有canonical load、claim/CAS、
恢复执行和checkpoint消费权威；Host仅在反序列化前检查已接受对象形状。

Checkpoint的continuation含普通业务属性`kind=node`。现有WireValue codec允许未知kind
包装透传，因此测试用公开的显式WireValue.Map区分业务JSON与协议包装，generation和
step_count显式编码为u64。它不承诺任意native plain object自动roundtrip，也不更改SDK
codec或checkpoint内容。

三语言比较仅将UUID/RFC3339时间映射为保留内部同一性的稳定标签，并对SharedState.keys
的HashMap无序集合排序。path、tags、context列表与其它有序字段均不排序；数字、状态、
参数和错误字段保留。error details缺失与null按相同可选字段语义比较。

正式入口为：

- TypeScript `test/workflow-checkpoint-host.test.js`，由既有`npm test`文件glob自动发现。
- Python `tests/test_workflow_checkpoint_host.py`，由既有pytest测试目录自动发现。
- Java公开JUnit类`WorkflowCheckpointHostTest`，由既有Surefire规则自动发现。
- 三者复用`sdks/shared/testing/workflow_checkpoint_fixture.py`；fixture只准备/解析/断言，
  不发送ACP调用，不提供模拟Host或第二Workflow状态。

`ECHO_AGENT_SDK_REQUIRE_HOST_E2E=1`时缺少`ECHO_AGENT_SDK_HOST`立即失败。
每组使用独立临时work/state目录；可设置`ECHO_AGENT_SDK_RETAIN_EVIDENCE=1`保留成功原始结果，
失败结果始终保留。测试源码没有硬编码个人仓库路径或草稿/tmp路径。

红绿原始日志、输入、48条响应、源hash、runner差异与比较收据归档于
[evidence/sdk-workflow-checkpoint-20261013](evidence/sdk-workflow-checkpoint-20261013/archive-manifest.json)。
旧workflow源码是按精确逆差分重建并与旧冻结manifest SHA核对的材料，明确不是事前backup；
真正反例来源为旧不可变二进制执行。历史收据不重绑定新的整仓摘要。

本组没有覆盖其余7项stream/cancel/extension操作、并发claim完整矩阵、graph revision错误
组合或跨重启恢复。默认MemoryStore不承诺跨进程持久化。本组属于临时组合framework输入的
候选局部证据，不能代替正式published pin、最新完整门禁、独立构建或精确main验收。
