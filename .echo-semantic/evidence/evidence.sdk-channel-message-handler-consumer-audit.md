---
{
  "schema_version": 1,
  "id": "evidence.sdk-channel-message-handler-consumer-audit",
  "kind": "evidence",
  "observed_at": "source:3ea544c47e331b5d29a6624f978f3595619bef39fa5985230351098d38930404",
  "source_refs": [
    "echo-sdk-host/src/core_profile/extension_bridge.rs#ExtensionChannelPluginProxy",
    "echo-sdk-host/src/core_profile/extension_bridge.rs#ExtensionChannelMessageHandlerProxy",
    "echo-sdk-host/src/core_profile/facade/integrations.rs#channels.manager.start",
    "echo-sdk-host/src/core_profile/mod.rs",
    "echo-sdk-protocol/src/methods.rs#ChannelMessageHandlerDescriptorWire",
    "contracts/sdk/accepted-external-contract.json"
  ],
  "supports": [
    "behavior.sdk-family-channels",
    "behavior.sdk-extension-bridge"
  ],
  "limitations": [
    "Source-path audit only, not runtime red/green or final API design.",
    "Existing ChannelPlugin start/send/stop passes do not prove inbound MessageHandler handle/handle_stream/reply consumption.",
    "No new receive operation or expanded accepted API is introduced by this Finding."
  ]
}
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
