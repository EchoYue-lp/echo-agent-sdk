---
schema_version: 1
id: evidence.sdk-tools-bridge-finite-observation
kind: evidence
observed_at: 5bcbad2437e5c19f61101883044c4f67944cb97c
source_refs:
- echo-sdk-host/src/core_profile/facade/tools.rs
- echo-sdk-host/src/core_profile/facade/registry.rs
- echo-sdk-host/src/core_profile/facade/source_operations.rs
- echo-sdk-host/src/core_profile/facade/integrations.rs
- echo-sdk-host/src/core_profile/facade/mod.rs
- echo-sdk-host/src/core_profile/extension_bridge.rs
- echo-sdk-host/tests/extension_bridge_e2e.rs
- docs/sdk/governance/evidence/remote-stream-failure-domain/manifest.json
- docs/sdk/governance/evidence/remote-stream-failure-domain/README.md
- docs/sdk/governance/evidence/remote-stream-failure-domain/before-valid-descriptor-run.json
- docs/sdk/governance/evidence/remote-stream-failure-domain/after-run.json
- docs/sdk/governance/evidence/remote-stream-failure-domain/production-only.patch
- docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/receipt.json
- docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/inputs-before.json
- docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/inputs-after.json
- docs/sdk/governance/evidence/sdk-chart-published-pin/ci-regression/regression-review-manifest.json
supports:
- behavior.sdk-family-files
- behavior.sdk-family-shell
- behavior.sdk-family-web
- behavior.sdk-family-database
- behavior.sdk-family-rag
- behavior.sdk-family-chart
- behavior.sdk-family-media
- behavior.sdk-family-data
- behavior.sdk-family-statistics
- behavior.sdk-family-research
- behavior.sdk-extension-bridge
- behavior.sdk-family-bridge
limitations:
- Ten builtin families and six reverse integration roles are named finite scopes, not all Rust identities or new APIs.
- Earlier whole790 language input is before the remotecause patch;3same-test genuine registered proxy results are postpatch, not a fabricated new fullnative gate.
- Network/provider proof is a timestamped response, not future availability; volatile Rag/checkpoint backends do not add crossprocessdurability.
- Five-second physicalTask B,12sourcekind, new SessionRunA and final whole/source/main remain separate.
---

# 内置工具与反向桥有限观察

## 支持的结论

回收current_tool_boundary_review原实际builtin10fam+reverse6role的8lens源审范围，原sourceaudit和后远端cause修复/consumer明确分开。最初两个源码Important已以官方ACP真实注册proxy/ToolManager/Llm调用反例验证并窄修，不因原whole绿色而隐藏；目前有限修复与liveJournal原其它组独审PASS0。

Tool/cwd/FileAuthority/schema/customvalidator走唯一frameworkauthority。Host family_tools→dispatch_tool→validate_tool_input→execute_with_context保持thin；registry/descriptor/kind/gen和Session注入管理owner，不第二retry/DAG/Runterminal。Tool失败原因复用tool_react_error、ToolFailure::from_error与原risk；LLM Cancelled复用AgentCancelled。

## 来源与范围

实际业务source观察69b，所列7运行/测试源码在5bcb仍逐Gitblob相同且currentbytes同，bridgec8dc及tools77c原SHA可复核。旧whole790针对原patch前业务场景；远端流failure-domain合法before3fail→samebodyafter3pass，caller token false，ToolCancelled/extension_timeout各read_only/standard/dangerous原字段保6receipt，LLM原Cancelled1receipt，原lease最终0。初attempt两Tool坏schema未到proxy保为fixture错误而不假生产红；validdescriptor原始log才用于原Tool生产缺陷。

Builtin具名有限审查表：

| family | 已examined行为/结果/authority/失败范围 |
| --- | --- |
| files | schema/arity/cwd；读写移动patchrollback/artifact原SHA；FileGuard保持descriptor；同长度异身份拒写；FileAuthority10+Tool49。 |
| shell | Unicode/stdout/nonzero/signal/timeout；原child/sandboxauthority；pendingcancel/EOF/真实PIDexit与cleanup；Tool49和Sandbox3模式，不Timeout代kill。 |
| web | schema/maxlength/extract字段完整HTTPartifact/SHA/retention；SearchProvider原重试/失败进入模型，具名请求/分页；外部时点。 |
| database | INT/TEXT/null/default7/PK/nullable/SQLxruntimecell；cursor/readonlytransaction/坏SQLtyped；同DBhash拒写。 |
| rag | chunk/overlap200/201scalar/ranksource；liveSessionRagStore/Embedder；第二503/坏JSON/空embedding原index不变；内存无跨process承诺。 |
| chart | 唯一schemavalidator与7mark/数据/checkedu32widthheight；null/omit原绑定；非法type/negative/overflow明确error；shared61不是统计能力数量。 |
| media | 已接受UTF8stats/unique/export原字节；missing/type/arity错误不overwrite；未接受codec不纳义务。 |
| data | sort/select/aggregate/join/Pivotcells；Polars唯一；nullindex/labels碰撞/unknownaggregate拒绝；原Inputexport字节；29/61。 |
| statistics | 已知mean/quantile/sampledeviation；非法列/nonnumerictyped，叶调用无新store；shared61只原统计具名case。 |
| research | 原6工具PDF/BibTeX字段bytes/artifact；HTTP503/坏PDF/4provider语义错误及严格30s观察；只时点。 |

Reverse已接受角色：

| role | 有限入口、权威与证据 |
| --- | --- |
| Tool | register→Sessionadd_tool descriptor/schema/risk/context/artifact原字段；ToolManager真实Completefailure消费，remotecause修复后6risk receipts |
| LlmClient | registrationorder→liveSession；messages/tools/usage/finish/config保，chat/stream实际；原tokenfalse的remoteCancelled分类 |
| Store | liveSession proxy declaredsearchmodes/namespace/CRUD/prune/dedup；rawKV不自动Active；Unsupportedcallback前拒；managedconsumerdeadline/commit/lostACK按原scope，三逆supportingDTO不造forward |
| Hook | HookRegistry按event注册/context投影，frameworkallowdeny/errorfailclosed，不复制注入顺序 |
| AgentCallback | liveAgentcallID/input/results/interruptionconnectionScope，observedcallback单位错误记录不反推Run失败 |
| InterventionCallback | 原tool/think/final阶段消费allowdeny/modifiedargs，policy保持framework，无第二审批owner |


## 已知缺口

该8lens有限审查不意味着全部familyverified；宽feature-failure-cause Finding中的Workflow/Memory/Eval原义务不由Tool/LLM窄scope自动resolved，只记录原remote子根因三链已闭。opaque SDK details无Rust字段则不造新字段，复用原typed转换不声称everydetailsroundtrip。媒体未接受codec和sourceoperation未承诺constructor不扩菜单；网络时点与内存scope/5sB/12及最终新源whole/main保独立。无新wire/API/权限；原examples/UI调用不改。

## 历史及尚未提交原收据的定位

- docs/sdk/governance/evidence/tool-value-effects/b909-natural-eof-after/manifest.json：4febaba8b00851be63b256d1dce013fb566b3bb6。仅保原具名运行来源，不冒5bcb包含这些原件。
- docs/sdk/governance/evidence/rag-project-rules-scalar-delta/after-receipt.json：4febaba8b00851be63b256d1dce013fb566b3bb6。仅保原具名运行来源，不冒5bcb包含这些原件。
- docs/sdk/governance/evidence/file-authority-identity-replacement/three-language-final/receipt.json：original scoped receipt is not yet committed; do not pretend source_ref Git。仅保原具名运行来源，不冒5bcb包含这些原件。

FileAuthority当前three-language-final包尚未进入此Git，故具体source_refs以已提交tools/FileAuthority调用及原790完整执行字段为限；这份作者审查可记examined，但该原补充包的正式交付需要另列实际版本，不伪fullpublished验证。
