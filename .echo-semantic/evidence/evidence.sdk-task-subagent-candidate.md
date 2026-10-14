---
{
  "schema_version": 1,
  "id": "evidence.sdk-task-subagent-candidate",
  "kind": "evidence",
  "observed_at": "source:3ea544c47e331b5d29a6624f978f3595619bef39fa5985230351098d38930404",
  "source_refs": [
    "docs/sdk/governance/evidence/sdk-task-subagent-candidate-20261013/proof.json",
    "docs/sdk/governance/evidence/sdk-task-subagent-candidate-20261013/archive-index.json",
    "docs/sdk/governance/evidence/sdk-task-subagent-candidate-20261013/files/task_subagent_fixture.py.txt",
    "docs/sdk/governance/evidence/sdk-task-subagent-candidate-20261013/files/task-subagent-host.test.js.txt",
    "docs/sdk/governance/evidence/sdk-task-subagent-candidate-20261013/files/test_task_subagent_host.py.txt",
    "docs/sdk/governance/evidence/sdk-task-subagent-candidate-20261013/files/TaskSubagentCandidate.java.txt",
    "docs/sdk/governance/evidence/sdk-task-subagent-candidate-20261013/files/scenario-plan.md.txt",
    "docs/sdk/governance/evidence/sdk-task-subagent-candidate-20261013/files/README.md.txt",
    "docs/sdk/governance/evidence/sdk-task-subagent-20261013/formal-proof.json",
    "sdks/shared/testing/task_subagent_fixture.py",
    "sdks/typescript/test/task-subagent-host.test.js",
    "sdks/python/tests/test_task_subagent_host.py",
    "sdks/java/src/test/java/com/echoagent/sdk/TaskSubagentHostTest.java",
    "docs/sdk/governance/task-subagent-evidence.md",
    "docs/sdk/governance/evidence/sdk-task-subagent-20261013/review-consumer-delta/manifest.json",
    "docs/sdk/governance/evidence/sdk-task-subagent-20261013/review-consumer-delta/task-subagent-host.test.js.patch",
    "docs/sdk/governance/evidence/sdk-task-subagent-20261013/review-consumer-delta/TaskSubagentHostTest.java.patch",
    "docs/sdk/governance/evidence/sdk-task-subagent-20261013/review-consumer-delta/sdk-task-subagent-review-callback-green.log"
  ],
  "supports": [
    "behavior.sdk-family-task",
    "behavior.sdk-family-subagent"
  ],
  "limitations": [
    "Opaqueids/nativedurationsrawkept; normalization compares canonicalrelationships not UUIDequality.",
    "Storefixtureonlyatomicexpectedrevision/executionCAS; frameworksoleDAG/claim/statusauthority.",
    "TaskSubagentContext fullDTO and restart/failurematrix unproved; callbackonlyAgentTaskInput string.",
    "Finalpublishedpin/mainandmatchingwholegatesremainpending.",
    "Four formally adopted tests perlanguage pass on cb9; final complete source/publishedpin/gates remainpending.",
    "Formal original input hashes and subsequent TypeScript callback-wait/Java assertion enhancements are separate execution epochs; no current four-source equality claim."
  ]
}
---

# Task/Subagent 共享真实 Host 候选场景

## 支持的结论

三语言通过公开原生 SDK API 执行四个相同场景：精确 pause/cancel A 保持 B Running、C Pending，随后 B/C 能独立完成；resume A 与 D 复用 canonical B 输出，不重派 B/C 或重写 B 的执行事实；stale relation CAS 与 duplicate execute 在副作用前明确拒绝；Subagent 幂等 receipt 只创建一个 attempt，5ms await 不消费 join，后续完整结果与再次 await 完全相同，Unicode constraint 到达真实 compiled prompt；CRUD-only 后端在 execution CAS 或用户 Agent 调用前拒绝 execute。

## 来源与范围

不可变 Host SHA 6711dd17fe3be13671ee181fce5ee79946f40963e000af293083f114301a39a5。TypeScript/Python 各四项测试、零失败/跳过；Java 在临时输出目录编译公开 API consumer，main 执行四个场景，未伪称四项 JUnit 测试。原始 callback/commit 与独立状态目录保留在 proof。

注册 Store fixture 只原子比较 expected revision/execution vector 并保存 framework 提交的 next graph；它不选择 frontier、创建 claim、决定 status 或实现第二调度器。比较保留相同 claim/revision 的关系、调用次数、关系 revision2、状态与输出。原生成 ID 与 duration 保留在 raw trace，不要求不同运行的 UUID 相同。

临时 consumer 输入以原字节 txt 归档，记录真实执行但不宣称已纳入 SDK CI。首次 TypeScript await fixture 只接受 agent_execute，实际 framework 使用 agent_execute_stream；修正为原 Rust fixture 已支持的两种回调后通过，不将 fixture 错误当成生产缺陷。

## 已知缺口

完整 TaskSubagentContext 字段 round-trip、外部后端故障/crash/restart，以及未列并发/publication 场景仍待。CustomAgent 回调只有 task String，不能伪造直接 TaskSubagentContext 消费证据。正式门禁已承接；最终已发布 pin、匹配的完整语言门禁与精确 main 验收仍待。

## 正式门禁承接后的实际验证

formal proof 保存的四份冻结输入在 cb9 Host 上完成三语言各四个正式场景，零失败/跳过，现有 Node/Pytest/Maven 入口可发现。其后 TypeScript 增加 callback jobs 结算等待，Java 加强 canonical B result/执行次数断言；完整差异保存在 review-consumer-delta，当前源码不能宣称与原 formal proof 全部 SHA 相同。

后继 TypeScript 源码29a候选在c52 Host完成四项focused场景，原始日志 `/tmp/sdk-task-subagent-review-callback-green.log`；其余后继运行按各自实际源码、Host 和归档收据支持，未归档的运行不用于扩大本 Evidence。原临时6711输入、formal cb9输入和后继断言增强保持独立 epoch；完整 context/crash 矩阵及最终 pin/main 门禁仍待。
