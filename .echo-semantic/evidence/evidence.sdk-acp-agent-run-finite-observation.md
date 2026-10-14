---
schema_version: 1
id: evidence.sdk-acp-agent-run-finite-observation
kind: evidence
observed_at: 8763c67bcfd6d98fe57449c88bb93f28f32bf78f
source_refs:
- echo-sdk-host/src/core_profile/handler.rs
- echo-sdk-host/src/core_profile/handles.rs
- echo-sdk-host/src/core_profile/state.rs
- echo-sdk-host/src/core_profile/mod.rs
- echo-sdk-host/src/factory.rs
- echo-sdk-host/src/core_profile/persistence.rs
- echo-sdk-host/src/lib.rs
- echo-sdk-host/src/features.rs
- echo-sdk-protocol/src/methods.rs
- echo-sdk-protocol/src/capability.rs
- echo-sdk-host/tests/core_profile_e2e.rs
- echo-sdk-host/tests/stdio_e2e.rs
- docs/adr/0037-session-run-idempotent-publication.md
- docs/sdk/governance/session-run-idempotency-proposal.md
- docs/sdk/governance/evidence/session-run-idempotency-implementation/independent-review.json
- docs/sdk/governance/evidence/session-run-idempotency-implementation/manifest.json
- docs/sdk/governance/evidence/session-run-idempotency-native/final-rpc-host/receipt.json
- docs/sdk/governance/evidence/sdk-final-idempotency-rust-gates/combined-receipt.json
- docs/sdk/governance/evidence/sdk-a634-rust-gates/05-workspace-tests.log
- docs/sdk/governance/evidence/sdk-a634-rust-gates/10-ci-core-profile.log
supports:
- behavior.sdk-acp
- behavior.sdk-agent-session-run
- behavior.sdk-family-standard
- behavior.sdk-family-agent-lifecycle
- behavior.sdk-family-session
- behavior.sdk-family-run
limitations:
- Original a634/193e790 evidence and finalA/da873 native13input are separate source epochs; no old whole result is relabelled as newer source.
- Session extension only create/load/close; list is official protocolinventory without accepted runtime advertisement, save is automatic projection not public new operation.
- RunHandle.close uses canonical cancel/wait before closing the local subscription (TS only if nonterminal; PY/Java always attempt); it is not a Hostrun/close wire or Host record retirement. get is snapshot; events/replay/ACK are stream observation.
- A samekey mapping is connection/generation scoped, not restartdurable idempotency or a second execution/terminal authority.
- New Java negotiation/frame bugs are independent defects outside this Hostslice; old complete language result becomes historical after client changes.
- Five-second physicalB,12sourcekind,fullgenerator/platform/main remain separate; cooperativeclosing not a proof of physicalstop.
---

# ACP、Agent/Session/Run与A幂等有限观察

## 支持的结论

本session独立readerframeworkFinal已审A全六源+ADR/proposal approvedA/publicdoc、并必要补读Agentcreate/describe/close、Sessionload/close、Runget/wait/cancel/steer/replay及原a634注册，有限8lens无新增具名高风险反例，C0/I0/M0。语义author只忠实回收其实际report不另假业务审。

FrameworkSessioncreation/turn/connection RunEntry/ledger/receipt仍唯一执行终态；HostimmutablePreparedDefinition/handle/address/publication record及settlement只管理真实工作，不复制Run/TaskDAG/validator。A仅已批准当前连接generation与operation/owner内samekey canonicalpayload重放原完整publication，payloadchanged typedconflict，closedowner不复活；同key失败前effect可retry，调用者cancel不能释放已发生effect的原job。

## 来源与范围

真实观察Git8763已含Aimplementation28f/native9edc/Rust27取证，生产handlerac5/handles8b89/state8e3/modc766/factoryda354及其余直接runtime/current实体逐blob核。原beforeHost193e接受但忽略key的4case→approvedA真实pubtests9（8本scope+1events）、2officialSentRequest.cancel、keyedcore1、busy/duplicate失败后key retry；新Hostda873三语各1test×5组/13输入同、完整响应/typederror/identity/index/seq/nativeprovider事实独立PASS。sameSource证据明确，不借collectorexit0作A行为成功。

原a634 Rust plain/mismatched hello failclosed、recoveredSession旧turn7→8、malformedidentity实际route不publication、killedrunningInterrupted、durabledebtexactretry、settlementcounter通知与retainedunpublishedSessionCleanup具名通过保旧切点。Steer原真实三语诊断因canonicalturn缺传红→第二真实provider请求消费Unicode/lateempty拒绝绿，frameworka634已发布该修复；finalA whole/source观察不能改写旧eb000compat前边界。

Sessionextension仅create/load/close，catalogofficial session_list不表示adapteradvertise/runtime接受；save指自动state/journal/receipt投影不造Session.save义务。Runget就是snapshot，start返回streamhandle+events/replay/ACK；RunHandle.close会使用既有canonical run/cancel及wait后关闭本地观察（TS仅非终态，Python/Java尝试取消并等待）；无Hostrunclose，不直接退役Host Run记录。可执行runtime菜单与official协议库存必须分开。

## 已知缺口

B5秒实际物理工作未停而claim清理原宽TaskFinding依旧open；12source/kind迁移、fullproducer/平台/main独立。最新Java协议数值溢出/数组静默丢帧是客户端独specificBug，不能用此前wholeTS311/PY378+18/J335成功掩盖；本Hostslice unchanged可finiteexamined，后newclientwhole另验。Agent不同Session原隔离/closingcleanup场景具名审过，不穷尽未定义所有组合，更不反证B。新Akey不跨connection/restart，semantic/audit不成为第二运行权威。

## 尚未提交的原始反例资料

docs/sdk/governance/evidence/session-run-idempotency-counterexample/facts.json, docs/sdk/governance/evidence/run-steer-native/initial-red-manifest.json, docs/sdk/governance/evidence/run-steer-native/java-diagnostic-fixed/receipt.json 属实际原before/diagnostic资料，尚未入本次8763，故不伪该Git source_refs。原发现/反例epoch及业务义务保留，模型跨已提交实现与native来源不杜撰旧raw到新commit。
