---
schema_version: 1
id: behavior.sdk-family-event-replay
kind: behavior
status: needs_review
expectation: human_confirmed
risk: high
primary_focus: state_authority
focus:
- time_lifecycle
- failure_concurrency
- contract_evidence
- data_durability
boundary: boundary.sdk-events-recovery
observed_at: source:3ea544c47e331b5d29a6624f978f3595619bef39fa5985230351098d38930404
code_refs:
- echo-sdk-host/src/core_profile/events.rs
- echo-sdk-host/src/core_profile/handler.rs
- echo-sdk-protocol/src/event.rs
- echo-sdk-host/tests/core_profile_e2e.rs
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-candidate-behavior-verification
- evidence.sdk-parent-driver-candidate-verification
- evidence.sdk-ack-recovery-verification
- evidence.sdk-gap-watermark-host-verification
finding_refs: []
---

# SDK family event_replay

## 重要承诺

对已接受 `event_replay` 能力明确入口、唯一权威、状态/生命周期、失败处置和直接行为证据；未接受 Rust identity 不进入承诺。

## 当前行为

- **入口/可达**：framework committed observer → `core_profile/events.rs::StreamDelivery`；
  `handler.rs` 的 run replay/event ACK 和语言 event feed 消费该 stream。
- **权威/状态**：framework `EventJournal` 拥有 identity/generation/committed sequence；
  Host cursor/window 仅管理 delivery。commit → live/gap → replay 或 snapshot coverage → ACK
  → continuation；snapshot covered watermark 与 last-sent 不是同一个量。
- **正常/失败**：#120 gap ACK 后只发送 coverage 后的事件；page cursor ACK、重复/未来/旧
  ACK、retention gap、oversized event、send failure、journal replacement、terminal ordering。
- **已有断言**：Core `ack_window_snapshot_ack_resumes_strictly_after_watermark`、
  `consumed_single_replay_page_ack_resumes_live_after_page_cursor`、multiple-page sibling；
  `sdks/shared/testing/gap_ack_fixture.py` 为真实 Host count/oversized gap 提供 parked/release。
  候选page ACK已实际运行；正式pin验收仍待。
- **最低充分证据**：三语言 snapshot ACK、single/multiple replay-page ACK 同一场景，释放
  model 后断言严格 sequence > consumed watermark、无重复/漏失且 terminal 来自 journal；
  加 journal identity 换代、future ACK rejection、restart replay 和 send failure 后恢复。

## 期望行为

上述正常/失败场景须保持接受范围、完整转换和唯一权威，并在最终源码/pin证据及独立复审后闭合。

## 触发、结果与副作用

按上方具体入口和正常/失败记录检查真实主路径。可达性或catalog状态不能证明副作用、错误或结算。

## 失败、重试与恢复

按上方生命周期和最低充分证据检查取消、超时、错误、释放和恢复；不默认成功，不以其它能力族测试替代。

## 证据

阶段：`candidate_runtime_verified`；语义状态仍为 `needs_review`。

snapshot ACK 与 page cursor ACK 后只续发已覆盖水位之后事件；三语言 count/oversized、两窗口 external ACK、terminal replay ACK 自然 EOF 都实际执行，journal union/event_id 与唯一 terminal 对账；未来 ACK 不推进。另有真实 segmented journal 原生轮转/prune：追加1..6后retained_floor=6，replay返回gap水位5及cursor6；只允许ACK6，拒绝ACK7。replay响应enqueue失败和空future page均不扩大ACK coverage。

实际来源：`echo-sdk-host/src/core_profile/events.rs#real_journal_retention_page_authorizes_only_its_returned_watermark`；`echo-sdk-host/src/core_profile/events.rs#replay_publication_failure_and_empty_future_page_grant_no_ack_coverage`；`echo-sdk-host/tests/core_profile_e2e.rs#ack_window_snapshot_ack_resumes_strictly_after_watermark`；`echo-sdk-host/tests/core_profile_e2e.rs#consumed_single_replay_page_ack_resumes_live_after_page_cursor`；`echo-sdk-host/tests/core_profile_e2e.rs#consumed_multiple_replay_page_ack_resumes_live_after_page_cursor`；`sdks/typescript/test/gap-ack-host.test.js`；`sdks/python/tests/test_gap_ack_host.py`；`sdks/java/src/test/java/com/echoagent/sdk/GapAckHostTest.java`。

仍缺：journal identity替换/错generation的明确故障场景，以及正式pin重启回放和已识别风险的独立复审仍待；原生prune及指定enqueue失败边界已有候选断言，不列为未验证。 published framework pin、最后变更后的独立复审和精确main验收
仍待；本证据不证明未列出的生命周期、字段或错误组合，也不自动扩大accepted合同。

## 裁决记录

用户要求SDK全能力行为治理，以明确接受跨语言合同为兼容边界；完整Rust inventory仅遥测。

## 当前观察与历史收据边界

Candidate assertion evidence is now preserved as original receipt/source fingerprints and checked against inspected current paths. It supports only named partial cases; current observation binding neither reruns old commands nor upgrades family status.

## 本次有限审查单元

行为/生命周期单元：committed journal/live/gap/replay/snapshot ACK的连续覆盖与真实发送事实。

剩余处置：v4新增真实反例已officialconsumer绿与独立exactslice pass；提交消费局部Finding resolved，Host完整scope及新单变量红绿已独立PASS，本地水位bugresolved；最终交付分开。

本项建模阈值是已接受入口、唯一状态权威、正常生命周期与具名失败/恢复路径可定位，并有直接证据或明确开放原因；不要求所有未定义组合全测，也不由绿色测试自动verified。
