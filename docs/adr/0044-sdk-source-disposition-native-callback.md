# ADR 0044：SDK 具名旧源退出与现回调合同绑定

## 状态

已批准

## 审批范围与证据等级

Root于2026-10-30依据用户完整Goal，在受控候选内批准下列11个旧source身份退出、3个with_id方法有限接受及native新增Rust source为0，并批准既有精确拒绝的薄适配路线；当前仅开放副本、测试、ADR及Before输入冻结准备。原生消息、无旧格式兼容方向沿用既有批准，不重新建立产品方向。

本状态承接Root机器裁决sdk-accepted11-r02-controlled-candidate-decision.json（SHA256 392f76abe4b5d987f2576c5c000bd1bbceefbe41312a7ce9887e9d3309a1ce88）。本轮立即准备先不写typed拒绝adapter；原full producer、语言export、Maven、Host及RED/GREEN仍须Root后继具名窗口。批准源身份处置不代表新pin、语言运行、native、依赖风险、正式main/CI或发布资格通过。具体运行结果由各自真实来源、命令、raw、退出状态与精确制品记录承担，不由ADR状态代签。

## 背景与正式事实来源

SDK是独立协议/Host/多语言产品；上游Framework拥有执行、任务、事件和持久化权威。沿用 [ADR0001](./0001-sdk-repository-boundary.md)、[ADR0032](./0032-sdk-contract-scope-classification.md)、[ADR0034](./0034-sdk-governance-authority-and-lossless-adaptation.md) 的边界；当前native薄适配以 [ADR0043](./0043-native-message-sdk-adapter.md) 和对应协议/Host源码为正式说明。A2A/LSP既有退役以 [ADR0042](./0042-sdk-a2a-lsp-retirement-boundary.md) 为历史依据，本次不重写该历史决定。

旧接受端点是SDK `3fcb8d066e8a6b6560d7991f06b4255121f2a49b`，Host正式pin仍为Framework `f7bac62eee04a963f92a3e4308b6ea2336e91cc0`；旧accepted5352/catalog431由该提交的Git blob固定。f7的Core AgentCallback中，三个with_id默认向无ID方法转发；state::AgentCheckpoint.current_plan明确仅保旧Store解码/往返，不被ReAct新写或恢复成任务状态；旧delete_persisted_conversation本来只支持legacy unmanaged，managed pair原就拒绝。

候选上游 `340993201cc937fc8e4783b66be399ef9f14e3d5` 已删除这些旧身份，当前仅是本地提交，不冒充已发布正式来源。其 `echo-core/src/agent/mod.rs`、`src/agent/react/run/pipeline.rs` 保真实with_id分派；`src/state/mod.rs` 保稳定请求managed删除和原incarnation clear；`docs/adr/0103-current-storage-records.md` 明确取消current_plan与无稳定身份旧组合删除，并保raw CRUD、当前消息及CAS/owner边界。这些是上游具名定义与正式ADR，不要求SDK维护者依赖某一ignored提案的结论。

SDK的实际合同机制在 `echo-sdk-protocol/src/bin/export_schema.rs::acceptance_delta` 与 `inventory.rs::apply_accepted_source_scope`。真实wire定义在methods.rs/managed.rs；Host callback、checkpoint、native投影及DELETE拒绝分别在core_profile/extension_bridge.rs、wire.rs、native_message.rs、facade/source_operations.rs及facade/mod.rs。三语言现注册/输入值与错误类各自在SDK正式源码中维护。下面每条机器字段的“本提案”“逐项分类”均指本ADR的范围、行为和相应机器处置节，不以ignored报告作为长期事实源。

## 已批准有限集合与非目标

下列完整机器处置节列出精确source/kind：retire-01至retire-11对应current_plan、legacy unmanaged DELETE、无ID三个callback、六个旧Token/Think变体/字段；accept-tool-start/end/error-with-id仅接受三个`echo_core::agent::AgentCallback::on_tool_*_with_id` method，canonical候选在echo_agent::agent。

本批native新增Rust source身份为0，不自动接受父trait、prelude别名、完整AgentEvent/Message/Provider/Store/Task子树或managed API；不接受on_tool_interrupted_with_id Rust身份，不改变其现wire能力。`echo_agent::agent::react::ReactAgent::set_store` 已接受，Result<()>签名后继通过原生成器正常进入既有身份摘要，不传accept-source。

不恢复旧无ID shim、旧计划字符串持久化/迁移、旧Token/Think节奏或无身份组合删除；不新增执行器、validator、Store、审批/取消owner；不部署、发布、推送或自动接受依赖风险。

## 候选方案、决策与取舍

1. 只退11、不接受三个当前callback来源，会使原shared bridge承诺成员从10少到7；wire仍存在不能代替正确source绑定，因此不采用。
2. 自动接受全部新native Rust值、父类型子树或完整managed API，会增加尚未具名审定的source承诺；已有阻断wire/schema不要求把完整Rust inventory变成SDK菜单，因此不采用。若将来需要额外source字段约束，必须单独精确裁决。
3. 采用本ADR的11 retire＋3 with_id accept / native新增source0：保现回调能力与显式拒绝，source scope和wire/schema各守自身合同。
4. 恢复旧shim、旧checkpoint字段和无稳定身份helper，会重新承担已取消的兼容义务并造成双路径，故不采用。

## 回调、原生消息和checkpoint保留

shared `bridge:agent_callback` / `_echo_agent/extension/register` 的候选10成员为AgentCallback本身、callback_id、callback_kind、on_final_answer、on_iteration、on_think_start、on_think_end及三个with_id；不是10个独立route。精确旧3与新3在同一次完整source transition维护，不能只drop旧行。

TS registerAgentCallback、PY register_agent_callback、Java registerAgentCallback与现CallbackOnToolStart/End/Error DTO继续使用固定wire名字callback_on_tool_start/end/error；agent/call_id/tool与args/result/error、同operation null-unit均保留。call_id的可选/null语义不强改，Host实际传原call_id。原interrupted、thinking/final、观察者等待结算边界保留；不把观察回调扩大为审批/取消或完整ToolResult错误DTO。

AssistantMessageBlockWire、ExecutionStatusWire、LlmMessageWire、usage与ReasoningBlock的已存在阻断schema继续维护。Framework真实phase、request/block/sequence、provider-source、signed/redacted/opaque原件和None/空值差异保留；标准session/update只是投影，完整原件不由UI文本取代。LlmUsage与usage_reported表达请求计数及未知；block.complete、FinalAnswer通知不直接授予Task/Goal成功。runtime的批准、Stop、持久化、取消与实际terminal边界保持。

新Rust值不因AgentEvent/Message父项已接受或wire已存在而自动取得source身份资格；本ADR的native0必须按这层覆盖边界解释，不能宣source逐字段全覆盖。旧Token/Think输入明确拒绝，不承诺旧形状/逐token时间等价。

current_plan仅旧任意文本往返退出，六字段checkpoint的history原字节、skills、blocked reason、cwd、timestamp继续保留。TaskRevisionService/revisioned graph是任务权威；应用TaskRun/PlanRevision及执行投影不迁入checkpoint作为第二计划副本。旧字段输入拒绝，不补None称无损。

## 精确DELETE拒绝与Java异步交付

旧operation `echo_agent::state::delete_persisted_conversation` 不再广告为可执行catalog route，保留的仅是精确拒绝：FeatureUnavailable / Never / 原operation / 原retired文案，零mutation。不从旧conversation_id补造ManagedConversationDelete的operation_id、payload_digest、expected_epoch或有效signature/request，不调用managed owner。

TS/PY在现集中catalog查找边界精确拒绝；Host在原结构/预算校验后、catalog/Session/Store访问前拒绝，归一移除不可达的旧source handler分支，保原clear协调器。未知其它operation及相近名字不被统一重分类。raw memory Store CRUD、raw ConversationStore删除与稳定请求managed协调器各有权威，不互相替代。

Java的异步source public API须正常返回CompletionStage，再以既有CompletableFuture.failedFuture交付精确拒绝。EchoAgentClient.invoke/call各在signature/resolve前处理；SessionHandle.invoke保原closed-session failedClosed第一优先，再在自身resolve前复用同一client精确失败工厂。精确DELETE遇closed/closing client保原host_exited failed-stage形状，不制造request以取得该错误；普通未知操作仍走原catalog同步错误，不能新增全局提前closed gate或广catch。

Java FacadeCatalog直接查找是同步metadata API，保持缺项同步IllegalArgumentException及列表无DELETE；本ADR不承诺所有Java CompletionStage方法统一不抛同步错误。family等不支持该source operation的其它参数/查找合同不重构。原ManagedRetirementHostTest的failure(session.invoke(...))须能正常求值，await后取得typed cause，并继续执行afterDelete的durable/calls零mutation断言，不改测试去容忍同步逸出。

## 历史guard、真实来源与生成影响

原Plan04 BASELINE `0930eef9f878cf436ed88384030b6bc03d6f6c53` 与5352/431/f7历史target保留；269 source、23 routes、39 operations退出、零新增和retained全行零变化只属于原历史两个端点。旧guard脚本blob、原ADR0042及raw/receipt不被改写为新epoch的结果。

后继正常维护在现guard分历史Plan04与本次当前transition，不增加另一套accepted权威。当前scope须精确为旧集合减本11、加本3，拒绝其它增退/alias冒canonical/退役能力回流；保10成员回调与wire能力。当前retained签名、catalog、schema/source-contract的允许差异必须由原full update实际产生，再经独审绑定精确输出/源/锁/ADR/命令，不能只改5352/431数字、删drift检查或动态拿新输出当批准。

原umbrella仍精确要求f7 HTTPS SourceID，deny/各仓风险合同独立。私有fileGit340只形成候选资格；正式pin及umbrella期望值只在真实已发布精确来源获接受后正常同步，不放宽任意rev/fileGit、不伪URL、不修改原风险基线来消除失败。最终实际来源应补入正式provenance/文档，原本地候选记录保其历史等级。

第一次明确source delta须用原完整--update及下列14条flags/ADR ref；成功写入新seed后后继原--check不带这些flags。原producer、parser、scope函数不为此修改。完整生成前的新signature/aggregate/catalog差异不可得，不提前构造。共享语言catalog与包内副本仍由原export入口维护，不手改生成物。

## 验收影响与交付条件

验收分别覆盖：原完整source/contract生成及后继check；精确11/3 source和10成员bridge；三语言真实Host start/end/error与原interrupted/关闭；原生消息/历史/签名/usage/终态；旧字段和旧事件拒绝；DELETE本地无RPC/ID、直接Host拒绝及backend零mutation；未知操作和Java关闭优先负控；raw/clear/managed owner原回归；历史/当前guard与精确来源/依赖门禁。

Java精确DELETE的client.invoke、client.call、Session.invoke调用本身须不抛且返回stage，await取得typed cause；原managed-retirement两个enabled分支必须到达durable-after，不能用fixture吞异常、skip或结果总数代替。直接catalog同步查找与其它未知操作保持原合同。所有结果绑定实际冻结组合，新full生成、Host、语言与正式main/CI分别取证。

ADR批准不替代这些实际验收；旧u01自然1和0产物写入保原，不追标通过。示例、SDK说明、包内合同副本与官网适用性随实际产品范围同步核对，未涉及项写明原因。

## 回退

只允许整体回到同一旧SDK3f、Framework f7、原0dac锁、原accepted/catalog/schema/source-contract及三语言包内副本/来源门禁的相容组合，并重新核对实际来源和适用门禁。不能把旧seed单独拷进新Framework、恢复旧shim/字段/无ID删除路径或生成新请求身份来“兼容”当前状态。

回退保留本次所有真实失败、raw、候选/正式边界及旧历史证据，不修改用户数据或把已发生副作用补记成功。是否执行回退仍由实际阶段的Root明确指令决定，本受控批准不授予自动切换、push/merge或发布许可。

## 精确机器处置记录

以下14组Action/Kind/Source/Reason/Compatibility/Rollback/Replacement与R01机器记录原样一致；本ADR正文已包含其正式解释与边界。无额外native、managed、父trait或set_store新增接受。

## retire-01

- Action: retire
- Kind: struct_field
- Source: echo_agent::state::AgentCheckpoint::current_plan
- Reason: 上游340中该具名旧身份已退出；能力保留和允许退出边界见逐项分类及Root有限复核。
- Compatibility: 按 ADR0103 取消旧任意 Option<String> decode/roundtrip，无自动迁移或补 None；不把 task graph 称为该字段的等价替代。
- Rollback: 仅整套回到原SDK3f/f7/0dac及原合同；不向当前Framework恢复旧shim或第二owner。
- Replacement: unsupported: 旧 current_plan 往返退出，旧字段输入明确拒绝；当前六字段 checkpoint 保留，计划权威仍是 TaskRevisionService/TaskRun graph。

## retire-02

- Action: retire
- Kind: function
- Source: echo_agent::state::delete_persisted_conversation
- Reason: 上游340中该具名旧身份已退出；能力保留和允许退出边界见逐项分类及Root有限复核。
- Compatibility: 按 ADR0103 退出无稳定身份的 legacy unmanaged 组合删除；保留 raw CRUD、原 incarnation clear 和现 managed owner，未新增 managed accepted API。
- Rollback: 仅整套回到原SDK3f/f7/0dac及原合同；不向当前Framework恢复旧shim或第二owner。
- Replacement: unsupported: FeatureUnavailable / Retryability::Never，保留原 operation；不调用 managed helper、不生成新请求身份。typed 拒绝须按本提案修正到 catalog 查找之前的薄边界。

## retire-03

- Action: retire
- Kind: method
- Source: echo_core::agent::AgentCallback::on_tool_end
- Reason: 上游340中该具名旧身份已退出；能力保留和允许退出边界见逐项分类及Root有限复核。
- Compatibility: 删除无ID Rust shim，不删除三语言 callback_on_tool_start/end/error 名称、原字段或观察能力；不引入执行审批权。
- Rollback: 仅整套回到原SDK3f/f7/0dac及原合同；不向当前Framework恢复旧shim或第二owner。
- Replacement: echo_core::agent::AgentCallback::on_tool_end_with_id；保留 bridge:agent_callback 与现 CallbackOnTool* wire DTO/三语注册，见对应 accept 记录。

## retire-04

- Action: retire
- Kind: method
- Source: echo_core::agent::AgentCallback::on_tool_error
- Reason: 上游340中该具名旧身份已退出；能力保留和允许退出边界见逐项分类及Root有限复核。
- Compatibility: 删除无ID Rust shim，不删除三语言 callback_on_tool_start/end/error 名称、原字段或观察能力；不引入执行审批权。
- Rollback: 仅整套回到原SDK3f/f7/0dac及原合同；不向当前Framework恢复旧shim或第二owner。
- Replacement: echo_core::agent::AgentCallback::on_tool_error_with_id；保留 bridge:agent_callback 与现 CallbackOnTool* wire DTO/三语注册，见对应 accept 记录。

## retire-05

- Action: retire
- Kind: method
- Source: echo_core::agent::AgentCallback::on_tool_start
- Reason: 上游340中该具名旧身份已退出；能力保留和允许退出边界见逐项分类及Root有限复核。
- Compatibility: 删除无ID Rust shim，不删除三语言 callback_on_tool_start/end/error 名称、原字段或观察能力；不引入执行审批权。
- Rollback: 仅整套回到原SDK3f/f7/0dac及原合同；不向当前Framework恢复旧shim或第二owner。
- Replacement: echo_core::agent::AgentCallback::on_tool_start_with_id；保留 bridge:agent_callback 与现 CallbackOnTool* wire DTO/三语注册，见对应 accept 记录。

## retire-06

- Action: retire
- Kind: variant
- Source: echo_core::agent::AgentEvent::ThinkEnd
- Reason: 上游340中该具名旧身份已退出；能力保留和允许退出边界见逐项分类及Root有限复核。
- Compatibility: 保 phase、身份/顺序、签名/opaque/redacted 原件、usage_reported 未知和 runtime 批准后终态；不保旧事件形状或逐token发布时间。
- Rollback: 仅整套回到原SDK3f/f7/0dac及原合同；不向当前Framework恢复旧shim或第二owner。
- Replacement: 现已入 wire/schema 的 AssistantMessageBlockWire / ExecutionStatusWire / AgentEventWire::LlmUsage、LlmMessageWire 与既有 thinking/final callbacks；旧 Token/Think 输入继续拒绝，不提供旧形状等价转换。

## retire-07

- Action: retire
- Kind: struct_field
- Source: echo_core::agent::AgentEvent::ThinkEnd::completion_tokens
- Reason: 上游340中该具名旧身份已退出；能力保留和允许退出边界见逐项分类及Root有限复核。
- Compatibility: 保 phase、身份/顺序、签名/opaque/redacted 原件、usage_reported 未知和 runtime 批准后终态；不保旧事件形状或逐token发布时间。
- Rollback: 仅整套回到原SDK3f/f7/0dac及原合同；不向当前Framework恢复旧shim或第二owner。
- Replacement: 现已入 wire/schema 的 AssistantMessageBlockWire / ExecutionStatusWire / AgentEventWire::LlmUsage、LlmMessageWire 与既有 thinking/final callbacks；旧 Token/Think 输入继续拒绝，不提供旧形状等价转换。

## retire-08

- Action: retire
- Kind: struct_field
- Source: echo_core::agent::AgentEvent::ThinkEnd::prompt_tokens
- Reason: 上游340中该具名旧身份已退出；能力保留和允许退出边界见逐项分类及Root有限复核。
- Compatibility: 保 phase、身份/顺序、签名/opaque/redacted 原件、usage_reported 未知和 runtime 批准后终态；不保旧事件形状或逐token发布时间。
- Rollback: 仅整套回到原SDK3f/f7/0dac及原合同；不向当前Framework恢复旧shim或第二owner。
- Replacement: 现已入 wire/schema 的 AssistantMessageBlockWire / ExecutionStatusWire / AgentEventWire::LlmUsage、LlmMessageWire 与既有 thinking/final callbacks；旧 Token/Think 输入继续拒绝，不提供旧形状等价转换。

## retire-09

- Action: retire
- Kind: variant
- Source: echo_core::agent::AgentEvent::ThinkStart
- Reason: 上游340中该具名旧身份已退出；能力保留和允许退出边界见逐项分类及Root有限复核。
- Compatibility: 保 phase、身份/顺序、签名/opaque/redacted 原件、usage_reported 未知和 runtime 批准后终态；不保旧事件形状或逐token发布时间。
- Rollback: 仅整套回到原SDK3f/f7/0dac及原合同；不向当前Framework恢复旧shim或第二owner。
- Replacement: 现已入 wire/schema 的 AssistantMessageBlockWire / ExecutionStatusWire / AgentEventWire::LlmUsage、LlmMessageWire 与既有 thinking/final callbacks；旧 Token/Think 输入继续拒绝，不提供旧形状等价转换。

## retire-10

- Action: retire
- Kind: variant
- Source: echo_core::agent::AgentEvent::Token
- Reason: 上游340中该具名旧身份已退出；能力保留和允许退出边界见逐项分类及Root有限复核。
- Compatibility: 保 phase、身份/顺序、签名/opaque/redacted 原件、usage_reported 未知和 runtime 批准后终态；不保旧事件形状或逐token发布时间。
- Rollback: 仅整套回到原SDK3f/f7/0dac及原合同；不向当前Framework恢复旧shim或第二owner。
- Replacement: 现已入 wire/schema 的 AssistantMessageBlockWire / ExecutionStatusWire / AgentEventWire::LlmUsage、LlmMessageWire 与既有 thinking/final callbacks；旧 Token/Think 输入继续拒绝，不提供旧形状等价转换。

## retire-11

- Action: retire
- Kind: struct_field
- Source: echo_core::agent::AgentEvent::Token::0
- Reason: 上游340中该具名旧身份已退出；能力保留和允许退出边界见逐项分类及Root有限复核。
- Compatibility: 保 phase、身份/顺序、签名/opaque/redacted 原件、usage_reported 未知和 runtime 批准后终态；不保旧事件形状或逐token发布时间。
- Rollback: 仅整套回到原SDK3f/f7/0dac及原合同；不向当前Framework恢复旧shim或第二owner。
- Replacement: 现已入 wire/schema 的 AssistantMessageBlockWire / ExecutionStatusWire / AgentEventWire::LlmUsage、LlmMessageWire 与既有 thinking/final callbacks；旧 Token/Think 输入继续拒绝，不提供旧形状等价转换。

## accept-tool-start-with-id

- Action: accept
- Kind: method
- Source: echo_core::agent::AgentCallback::on_tool_start_with_id
- Reason: 以当前带真实call_id的既有方法绑定原工具观察能力，替代已删除无ID accepted 源成员。
- Compatibility: 保原bridge与三语言固定callback_on_tool_start wire名字及既有输入/unit结果；不接受父trait子树，不新增取消或审批权。
- Rollback: 仅整套回到原SDK3f/f7/0dac及原合同；不在当前框架补回无IDshim。

## accept-tool-end-with-id

- Action: accept
- Kind: method
- Source: echo_core::agent::AgentCallback::on_tool_end_with_id
- Reason: 以当前带真实call_id的既有方法绑定原工具观察能力，替代已删除无ID accepted 源成员。
- Compatibility: 保原bridge与三语言固定callback_on_tool_end wire名字及既有输入/unit结果；不接受父trait子树，不新增取消或审批权。
- Rollback: 仅整套回到原SDK3f/f7/0dac及原合同；不在当前框架补回无IDshim。

## accept-tool-error-with-id

- Action: accept
- Kind: method
- Source: echo_core::agent::AgentCallback::on_tool_error_with_id
- Reason: 以当前带真实call_id的既有方法绑定原工具观察能力，替代已删除无ID accepted 源成员。
- Compatibility: 保原bridge与三语言固定callback_on_tool_error wire名字及既有输入/unit结果；不接受父trait子树，不新增取消或审批权。
- Rollback: 仅整套回到原SDK3f/f7/0dac及原合同；不在当前框架补回无IDshim。
