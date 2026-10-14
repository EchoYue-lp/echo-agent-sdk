---
schema_version: 1
id: evidence.sdk-channel-message-handler-consumer-audit
kind: evidence
observed_at: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
source_refs:
- echo-sdk-host/src/core_profile/extension_bridge.rs#ExtensionChannelPluginProxy
- echo-sdk-host/src/core_profile/extension_bridge.rs#ExtensionChannelMessageHandlerProxy
- echo-sdk-host/src/core_profile/facade/integrations.rs#channels.manager.start
- echo-sdk-host/src/core_profile/mod.rs
- echo-sdk-protocol/src/methods.rs#ChannelMessageHandlerDescriptorWire
- contracts/sdk/accepted-external-contract.json
- docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/final-receipt.json
- docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/source-build-manifest.json
- docs/sdk/governance/evidence/sdk-language-factory-final/receipt.json
- docs/sdk/governance/evidence/sdk-language-factory-final/source-before.json
- docs/sdk/governance/evidence/sdk-language-factory-final/source-after.json
supports:
- behavior.sdk-family-channels
- behavior.sdk-extension-bridge
limitations:
- Source-path audit only, not runtime red/green or final API design.
- Existing ChannelPlugin start/send/stop passes do not prove inbound MessageHandler
  handle/handle_stream/reply consumption.
- No new receive operation or expanded accepted API is introduced by this Finding.
---

# Accepted Channel MessageHandler consumer source audit

## 支持的结论

Accepted canonical MessageHandler handle/handle_stream/reply map to the
channel_message_handler reverse bridge. Proxies implement typed calls, but
ExtensionChannelPluginProxy.start only stores the supplied Arc handler and sends
handler_id text to the SDK plugin. The stored handler has no consuming call in
that proxy; stop only clears it. channels.manager.start constructs the Handler
proxy and passes it to start_all. This verifies registration, not inbound use.

## 来源与范围

This is direct current-source path evidence: registry/descriptor kinds and
handler_id are checked in integrations; bridge direction is Host-to-SDK
ExtensionInvokeCall, registered on the client as reverse request. A language
plugin's handler_id is not a callable SDK-to-Host receive address. No existing
registered native client operation provides that dispatch path. There is no
runtime counterexample assertion in this source audit, and no synthetic testing
receive operation was created to make it appear reachable.

## 已知缺口

The exact accepted inbound contract and legitimate framework/native consumer
must be investigated before an API change. Plugin start/send/stop evidence stays
valid only for those paths. This Finding remains open through explicit design,
real inbound failure/success/stream/reply lifecycle tests, independent review and
precise final publication; new wire scope is not inferred from a DTO.

## 本次 current_source_business_rereview

原已审SDKruntime代码来源Git569/source17cf和治理文稿可恢复Git`9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d`（source0a13）明确分开。独立reader`sdk_capability_acceptance_audit`本次实际重读原业务fault/完整字段/副作用/owner/调用方变化与14Rust/newbb4f632原after证据，消息回收UTC`2026-10-14T01:56:14.724478+00:00`，不是原执行时间。结果：**PASS0 原缺口到已批准6op消费链的当前有限审查**。

原registeredhandler文本无forward的before静态义务保历史；本次实际读new6closedoperations/原receiver绑定/current569source同code，多nativefinite16/namedinvalid12 zeroCallback/streamseq/nullableReply完整。只保批过的三handler+3controls，不添加handlerClose/new4thmethod；fullgeneratedsource12仍待人。

原观测marker`source:3ea544c47e331b5d29a6624f978f3595619bef39fa5985230351098d38930404`、旧run-time/host/sourcehash/raw和正文保，所有source_refs从真实9dbGitblob读取且锚点存在；现在SDKbe5pin与新测试写入会改变当前文件，不能把历史b909当前源码审升级成未来be5gate。真实9db框架pin为b909，工作树be5after另Evidence处理，不将两个仓tree混写。新增门禁sourceRef只具名Scope，没有以整count判族verified；supports/limits原保、不声明旧3ea整仓可恢复、不改unknown成无缺陷。

原义务和全文历史主张保留以审查，若旧unknown因素（TTL/三getter消费）已有新原证据，只在本段给来源区别，不抹原epoch。ChartI1/5sB历史BLOCK明确不因observed版本合法消失，12source/CI/main/globalacceptedproducer仍pending，41family/lens状态不提升。
