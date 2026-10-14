---
schema_version: 1
id: evidence.sdk-testing-value-candidate
kind: evidence
observed_at: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
source_refs:
- sdks/shared/fixtures/testing-contract-values.json
- sdks/typescript/test/testing-contract.test.js
- sdks/python/tests/test_testing_contract.py
- sdks/java/src/test/java/com/echoagent/sdk/TestingContractTest.java
- sdks/typescript/src/wire.ts
- sdks/python/src/echo_agent_sdk/wire.py
- sdks/java/src/main/java/com/echoagent/sdk/WireValues.java
- sdks/java/src/main/java/com/echoagent/sdk/JsonSupport.java
- docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/final-receipt.json
- docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/source-build-manifest.json
- docs/sdk/governance/evidence/sdk-language-factory-final/receipt.json
- docs/sdk/governance/evidence/sdk-language-factory-final/source-before.json
- docs/sdk/governance/evidence/sdk-language-factory-final/source-after.json
supports:
- behavior.sdk-family-testing
limitations:
- Only 14 accepted value fields; no testing remote operation or Rust Mock runtime.
- Java uses controlled official ACP value transport, not real Host testing behavior.
- No Rust serde reconstruction or new Mock-specific validator guarantee.
- Published framework signature/full final source/main remains pending.
---

# SDK testing接受值合同候选证据

## 支持的结论

接受范围固定为 **14 个字段 identity，分属 MockToolContext、MockAgentStep 和 StreamChunk 的 field payloads**，均为 `surface=value`，无method/operation。依据 [accepted scope receipt](../../docs/sdk/governance/evidence/sdk-tool-testing-20261013/sdk-testing-value-contract-scope.json)：accepted external contract SHA `901040bc446b2981e6146bfb9b82e93a1d68ff83552c05eb5e68ecd2f119fd45`；共享 fixture SHA `9d7c13f0b03ece4259d7e72e3b0e23c526cc248b43f05b90c7f8574f9216f011`。没有把Rust Mock runtime、测试调度或远程 testing verb接受进SDK。

三语言同一corpus有12个positive payload与7个invalid投影。Unicode身份、u64/i64/duration、optional/variant/opaque unknown wrapper的列出值保持；损坏标量/容器明确拒绝。TypeScript公共 toWireValue/fromWireValue 与Python公共 to_wire/from_wire实际各19 tests全绿零skip，raw encoding/projection和拒绝记录可比较。Java公共 WireValues.value + EchoAgentClient.invoke 在controlled official AcpClientSession/AcpClientTransport上检查相同expected JSON：2 named JUnit tests内部12+7断言，0 fail/error/skip。

## 来源与范围

源码观察身份仅说明归档时读到的当前工作树；运行证据仍绑定下面的实测 epoch、逐文件 SHA 和冻结 Host。归档未重跑业务验证，也没有把旧日志绑定到后来的整仓源码。所有关联 Behavior 保持 `needs_review`，published framework pin、完整门禁、独立复审和精确 main 验收仍待。

归档观察中共享 corpus、三个测试文件和四个wire实现文件SHA全部仍与原 receipt一致。Java最终full Maven使用client源码 SHA `2bad1f8a4ec4ba0a84f2c6faa63007364b5be874303bec0fbb332ab33f55c223`，session45147 exit0；其控制transport不是Host，不归属真实Host Mock runtime行为。该full任务其它类使用Host SHA `4d436b60f1fe8d45c1472c3d4f7587ca0fb368bb50ab161f5d1a54f861d66b22`不等于testing测试通过真实Host执行。

## 实际执行与可复核材料

- TypeScript：19 passed，0 failed/skipped；当前运行日志 SHA `6a4f3ab30f6cae6d5786e6a9798dded7d3c236fc0f139a0255009f1f1336bdb3`。
- Python：19 passed，0 failed/skipped；日志 SHA `b84a5f56832006ce4e2bf9f759666949900308668a781b362e5aab41c6c2b8e8`。
- Java final frozen XML：2 tests，0 failure/error/skip，内部12 positive+7 invalid；XML SHA `3b2133b7a5a96ecc7818e3e15db129ce085d3cb0850751e4576b618776d7f747`。

原始输出和拒绝记录、运行日志、final Java XML均归档。见 [testing receipt](../../docs/sdk/governance/evidence/sdk-tool-testing-20261013/sdk-testing-value-evidence.json)、[archive manifest](../../docs/sdk/governance/evidence/sdk-tool-testing-20261013/archive-manifest.json)。TS/Python有raw编码投影dump；Java仅有动态same-expected断言，未伪造不存在的Java raw transcript。

## 已知缺口

Rust源类型没有serde codec；WireValue payload wrappers不是重建Rust mock对象。invalid cases证明generic lossless value validation，不证明新增Mock专属validator。controlled Java transport只回显值，不拥有Store、Run、terminal或测试执行器。不存在的 testing op 在本组没有新增Host调用证据；声明`operations=[]`与source排除testing属于读取事实，不用catalog done替代运行验证。正式pin/inventory签名对应、最新独立复审与精确main仍待。

## 本次 current_source_business_rereview

原已审SDKruntime代码来源Git569/source17cf和治理文稿可恢复Git`9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d`（source0a13）明确分开。独立reader`sdk_capability_acceptance_audit`本次实际重读原业务fault/完整字段/副作用/owner/调用方变化与14Rust/newbb4f632原after证据，消息回收UTC`2026-10-14T01:56:14.724478+00:00`，不是原执行时间。结果：**PASS0 接受14纯值字段限定**。

12normal7invalid原3types scalar/time/duration/optional/Unicode/unknownwrapper公共codec，Java官方controlledtransport投影与当前languagecase源码核。不是HostMockruntime，不给Rust不serde对象还原/newtestingremoteverb。

原观测marker`source:3ea544c47e331b5d29a6624f978f3595619bef39fa5985230351098d38930404`、旧run-time/host/sourcehash/raw和正文保，所有source_refs从真实9dbGitblob读取且锚点存在；现在SDKbe5pin与新测试写入会改变当前文件，不能把历史b909当前源码审升级成未来be5gate。真实9db框架pin为b909，工作树be5after另Evidence处理，不将两个仓tree混写。新增门禁sourceRef只具名Scope，没有以整count判族verified；supports/limits原保、不声明旧3ea整仓可恢复、不改unknown成无缺陷。

原义务和全文历史主张保留以审查，若旧unknown因素（TTL/三getter消费）已有新原证据，只在本段给来源区别，不抹原epoch。ChartI1/5sB历史BLOCK明确不因observed版本合法消失，12source/CI/main/globalacceptedproducer仍pending，41family/lens状态不提升。
