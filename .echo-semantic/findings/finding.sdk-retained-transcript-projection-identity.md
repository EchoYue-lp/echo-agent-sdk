---
schema_version: 1
id: finding.sdk-retained-transcript-projection-identity
kind: finding
type: implementation_bug
status: resolved
severity: high
primary_focus: data_durability
focus:
- state_authority
- contract_evidence
boundary_ref: boundary.sdk-integrations
behavior_refs:
- behavior.sdk-family-source-operation
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-integrations-finite-observation
audit_refs: []
decision_refs: []
repair_evidence_refs:
- evidence.sdk-model-read-retained-repair
verification_evidence_refs:
- evidence.sdk-model-read-retained-verification
rereview_audit_refs:
- audit.sdk-retained-identity-rereview
discovered_at: 88a573b7dbe2cd4878427ea35eb43322dbf11089
---

# retained transcript 在 compaction 后被分配新 ordinal 并再次投影

## 问题

本条跟踪 SDK 已接受持久化消费承诺的 framework producer 依赖，唯一修复 owner 是 framework，未认定 SDK proxy 生成重复批次。模型适配另有独立 Finding，预算 forwarding 正确不另建。旧 Issue 105/106 或 Workflow Graph checkpoint Finding 不覆盖本条 retained transcript identity。

## 触发条件与影响

原旧 pin `6563d26570cc4a8d93c86b7e37443b0559476807`、Host `0982121cb1e99284b4a7d7e07af35037f9f22bd653528d9716d425f5609407a0` 上，新 live consumer 用例在同一 Session 显式绑定 A，实际一条 shell provider 调用，压缩仅淘汰末尾 assistant，再正规保存 compressed AgentCheckpoint 后下一轮。04 的消息保留/回调/真实 RunStore 用例断言通过，原日志563 bytes、SHA256 `d3b1aa202079f1b8c8ecf3421654e5f8d03135a1e0e8837dd9fbb9181499b06d`；它没有验证 retained ordinal 不变，不能据绿色用例关闭本条。

## 证据

原 typed 数据在同 generation/epoch1 有六批九项：旧 user/tool-call/tool-result 的 ordinal0/1/2，在 compaction 后被重新投影为4/5/6，再追加新轮7/8；原三项 canonical 内容除时间/identity 等六字段外逐字段一致，checkpoint69旧 fingerprint 与 checkpoint77新 fingerprint 相同。对照表明是同一批 retained 可见消息重新分配 identity，不能把一条真实 shell 调用解释为三次新业务事实。完整原批次/原 messages_json/源指纹/测试程序只保在 ignored local，未改原件。

F `src/agent/snapshot.rs` 原前序 suffix 对当前 prefix 的匹配使 `[U,T,R,F]→[U,T,R]` overlap 为0，再从 next_ordinal 分配新值；coordinator 优先 durable cursor，单加旧 realign 调用也会被覆盖。现有 [persistence concepts](https://github.com/EchoYue-lp/echo-agent/blob/6563d26570cc4a8d93c86b7e37443b0559476807/docs/en/41-persistence-concepts.md) 与 [ADR0056](https://github.com/EchoYue-lp/echo-agent/blob/6563d26570cc4a8d93c86b7e37443b0559476807/docs/adr/0056-durable-transcript-projection-settlement.md) 的原 identity/ordinal 与持久化承诺由 F 原 reader 复核，本条不是新增 policy。

## 处理记录

### 2026-10-19 原记录（历史）

修复与 before/after 由 F 原治理分支用真实 managed FileConversationStore/RuntimeStateStore 完成。SDK 必须继续无损转发，禁止过滤批次、换掉正常 evict 集、关闭 managed、改 None、增加 Store/DTO 或并行 cursor；新公开精确 main 后才升级 SDK 原 pin/唯一生成/当前 Host，增强原 retained identity oracle并实际验证。当前状态 open，三链空，不将修复设计、旧 selector通过或 producer-only green 当成消费者已解决。

01 descriptor 参数错误、02/03 缺压缩后持久 checkpoint 的 fixture 前提错误，均保留其真实失败/源码与诊断；内存 CompressionCheckpoint 不等 AgentCheckpoint，04 的正确显式 commit 仍保留。它们与这里的 retained identity producer 缺陷分开，不用生产改动“修”原错误 oracle。


## 2026-10-20：本地具体闭合

当前状态resolved仅指本Finding的原accepted义务。公开SDK208a消费已接受framework精main356及原唯一生成，私有原cfg/Result/live权威读取和retained producer分别由repair Evidence定位；同原反例的当前Host after、完整Rust14/唯一完整三语入口与3quickstarts、fresh真实锁依赖及精确独审由verification和rereview三链承接。历史before/夹具失败、source/runtime cut及上文当时open记录保留，不降accepted承诺为deferred。

本地闭合不证明SDK精main、适用Linux/Windows CI、nativeHook/Stop或所有未列组合；managed历史Linux退出超时根因未知由其独立Finding保持open。Behavior及全库闭合标志不因本条改变。
