---
schema_version: 1
id: evidence.sdk-tools-bridge-finite-observation
kind: evidence
observed_at: 622d7931e01556161441d22de0697f6f35f24f17
source_refs:
- echo-sdk-host/src/core_profile/facade/tools.rs
- echo-sdk-host/src/core_profile/facade/registry.rs
- echo-sdk-host/src/core_profile/facade/source_operations.rs
- echo-sdk-host/src/core_profile/facade/integrations.rs
- echo-sdk-host/src/core_profile/facade/mod.rs
- echo-sdk-host/src/core_profile/extension_bridge.rs
- echo-sdk-host/tests/extension_bridge_e2e.rs
- echo-sdk-host/src/core_profile/facade/memory.rs#unsupported_store_operation_keeps_memory_cause_without_mutation
- echo-sdk-host/src/core_profile/facade/workflow.rs
- echo-sdk-host/tests/facade_feature_adapters_e2e.rs#workflow_cancel_preserves_canonical_failure_in_rpc_and_stream_terminal
- echo-sdk-host/src/core_profile/wire.rs
- echo-sdk-protocol/src/managed.rs
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
- behavior.sdk-family-workflow
- behavior.sdk-family-memory
- behavior.sdk-family-eval
- behavior.sdk-family-state
- behavior.sdk-integrations
limitations:
- Ten builtin families and six reverse integration roles are named finite scopes, not all Rust identities or new APIs.
- Earlier whole790 language input is before the remotecause patch;3same-test genuine registered proxy results are postpatch, not a fabricated new fullnative gate.
- Network/provider proof is a timestamped response, not future availability; volatile Rag/checkpoint backends do not add crossprocessdurability.
- Five-second physicalTask B,12sourcekind, new SessionRunA and final whole/source/main remain separate.
asset_refs:
- asset.sdk-historical-proof-location
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

## 后继原三域错误原因精确独立复审与闭合

后继frameworkFinal独立有限源码审真实PASS C0/I0/M0明确补齐原Workflow/Memory/Eval三个错误域：Memory1532的liveStore.search_with→framework_error/semanticunsupported测试396，category=memory/terminalfailed/code=memory/retryfalse/原operation，旧namespacevalue/len1保无mutation；Eval91bd原run_all factoryErr由FailedEvalAgent第一次取Mutex<Option<originalReactError>>而非displaytext重造，真实EvalRunner对Cancelled/Timeout/Llm429 FailedResult保原violation/清空workspace（不伪每项RPCthrow或三native全部错误域已执行）；Workflow f631 graph.run和startup/stream每eventErr原canonicalconverter，实际Host2190取消RPC完整Cancelled/Never/operation/causequartet，stream failed variant仍Cancelledcause非Complete。Memory/Eval/wire当前和622d相同；workflow当前90e6仅cfgtest LLMA→B新增，生产prefix/原错误路径不变，未在这次审/未终态测试不借成果。原最新finalRust05/13日志逐具名实际ok。 本段是补到旧remoteTool/Llm专用scope之后的实际新有限复核，不改原scope当时仍未决的历史句子；合原remote3真红绿，本Finding原所有具名原因丢失root义务三链足够可局部闭合，非全桥资源或physicalB/12/main。

## 三个具名工具/桥适配缺陷的局部闭合

本次依据 current_tool_boundary_review 的真实原源码审和逐 Finding 有限 closure 复核，C0/I0/M0；622d 是本次具体源码与保存收据的观察版本，不声称原运行重跑于此。

### finding.sdk-extension-pending-stream-owner

原初始应答被外层 ToolManager timeout/drop 的缺陷由原 PendingStream RAII 守护 sink、已发 stream 地址和 callback/reentry guard。具名 unit 实际 drop 后资源释放，两个真实 Host 回归分别 outer timeout before ACK 与 cancelled initial response，保原 Session owner/低配额再入强断言；最新正式原05逐名已执行通过。不是只借 teardown prefix 修复或整体测试数。

### finding.sdk-managed-store-adaptation

原 managed capability/deadline/revision/receipt 适配义务结合既有 managed-retirement 独审与 supporting-operation-boundary：missing capability 先于 mutation 拒绝，deadline 过期不 commit，commit 后丢 ACK 保 OutcomeUnknown，disconnect/restart 使用同 exact receipt，typed checkpoint 大 revision 与 subsecond 字段无损；具名 tests 在最新原05真实通过。第八个 native consumer 与 backend20 证据保原 epoch，不宣全部11 forward；三逆 DTO 保 schema/RustHost supporting 边界，无已接受 forward 不新造 API。

### finding.sdk-tool-input-validator-bypass

原 adapter 跳过唯一 framework schema/custom validator 的具体 bug 已改为同一 validate_tool_input，参数绑定拒 excess arity（含 null），没有第二 validator。真实 prototype not_a_chart_type→假 line 的原成功和修后 typed InvalidParameter 保在旧专属 Evidence；正式 spy 强断言 missing/type/enum/excessNull/customBad 全 executes0、valid 精确 executes1，最新原05真实通过。后继 Chart 整数义务独立已有61原生/独审闭合，不以旧非法 enum 修复替整数证明，不把 helper 自动接受为三语言 API。

原 managed-retirement 与 supporting-operation-boundary 的各自 supports/limits 全保；pending owner 不宣任意外部 producer 物理强停。Sandbox/Task/cleanup 关联同 B 未决，12 身份/平台/main 独立，未升整个 family verified。

## 本任务全目标差异的精确路径依据补齐

此段只给本owner对应实际证据/consumer的可复核输入性质，不定义新runtime authority、不新增接受API、不用路径数量证明行为。原observed_at 622d7931e01556161441d22de0697f6f35f24f17、原命令时间/Host/数据/限制全部不变；可直接列入source_refs的历史取证副本已在该原观察Git逐blob读取存在，其余当前test输入另由真实db34a76226732d50892be5718cd48836576aea9c和当前完整官方语言包保存，不冒旧运行时刻。

|准确路径|输入角色与范围|当前保存的原字节SHA|
|---|---|---|
|docs/sdk/governance/evidence/remote-stream-failure-domain/extension_bridge.after.rs|原before/after/contract-basis源码取证副本；只解释此owner原故障/输入，不执行/注册/拥有新状态|c8dc3f1263ae7a84190080211fbde2eb6e6d93dade6f7b9d24da85026355f9ac|
|docs/sdk/governance/evidence/remote-stream-failure-domain/extension_bridge.before.rs|原before/after/contract-basis源码取证副本；只解释此owner原故障/输入，不执行/注册/拥有新状态|8e7aad2896687394a507f339fb20cba2dbacb93fd42ab38b417534570204e8e9|
|docs/sdk/governance/evidence/remote-stream-failure-domain/extension_bridge.valid-counterexample.rs|原before/after/contract-basis源码取证副本；只解释此owner原故障/输入，不执行/注册/拥有新状态|1653a22e81c1b2c62fb10de02fa77978e6d062f9e1bda6f530290a8a7b0bb2cc|
|sdks/python/tests/test_file_authority_identity_host.py|该能力真实native测试或共同fixture，当前输入证据；不把Ruff/输入哈希单独当行为PASS|6cc4b94dcd2717ef343763e2e3ec37cb33028777afea7284fae29b51d7f74eb1|
|sdks/shared/testing/file_authority_identity_fixture.py|该能力真实native测试或共同fixture，当前输入证据；不把Ruff/输入哈希单独当行为PASS|c9a5d5d00fdfaeff9e1572467c470c075950013e19b31b3e6e30eb7fed5242c3|

本owner表中runtime test/fixture共2项。它们是当前官方whole793的同源输入及各组具名独立证明的定位，不从输入哈希/Ruff本身推业务PASS；当前SourceGit在db34a76226732d50892be5718cd48836576aea9c可恢复。归档项只作为原before/after/contract witness，不执行、不产生第二实现/validator。Java/Python版本兼容与原门禁仍各用实际原收据，旧supports/limitations/运行时间不改，不声称所有族fresh。


## finding.sdk-tool-pid-observation-window 的有限三链出处

本段当期观察e58只绑定当前可恢复测试源、原因果/修复和新实际CI，不改原observed_at、旧source_refs/command或其它原缺陷状态。

原722空PID及Darwin2.592508709秒合法排队反例各保持自身事实；原0ebe真正直接cause为Host18445/tail18521同一0.03058024s记录：default ps把路径截为target/t，-ww保完整tail路径，严格matcher因此漏报；原未信号tail仍正确5003ms timeout，不是从未dispatch或生产shell数据丢失。修复c5只原ps argv加-ww；原原始观察协议806也保先前cut，没有加容量/改budget/放松len1/路径/身份断言。

原Admin sdk-linux-ps-width-fix/causal-facts.json 2249B/SHAfbee6c2216c49320595c94498209b8daf8d0726255f7121edc08c9b537504538、after-native-summary.json 3004B/SHAe2a8f356aa103bca8f0ba676a0d5930dda5e440a6002f40bf83cdeaa961b2030保存原before/after。Darwin3.12.4/ACP0.12.1本地49rows48calls明确owned PID/PPID/command/root，signal前复核；shell timeout/signal各有实际原PID before与physical after[]，返回完整typed cause/metadata/all protected rows相等。SDKcap原source/native实际有限PASS0，不伪造本次新reviewJSON。原执行HEAD185559但修复未提交，c5是后保存相同62bc字节，不能从185559 Git取它。

本次后验是[e58官方语言job116293151777](https://github.com/EchoYue-lp/echo-agent-sdk/actions/runs/38696176970/job/116293151777)与[Hostjob116293152091](https://github.com/EchoYue-lp/echo-agent-sdk/actions/runs/38696176970/job/116293152091)各自自然终态success；head e58a3e0c07cf0b6b24d570df07063301a888ddc6、actual PRmerge e9c7275c6f5df5652e555aa4f457c65870676b34、tree c48b54ec7a58bfe855d2724f777b1d605789b7a2与已提交源码树一致。语言原raw `/Users/ls/MyWork/code/ylp_agent_learn/lp-agent/.worktrees/admin/sdk-e58a3e0-ci-evidence/116293151777.log`，200420B/SHAaec6371714b2d2bd23612b3b6d139acccf507a541db327ef13c5c744f5115cce；TS343、Python427+18子测试、完整Maven test按set-e成功且三个quickstart均实际到达/finalmarker通过，Node20.20.2、Temurin17.0.20+1。成功静默Maven没有打印Java数量，也没有下载XML，不编Java总数。Host原raw `/Users/ls/MyWork/code/ylp_agent_learn/lp-agent/.worktrees/admin/sdk-e58a3e0-ci-evidence/116293152091.log`，164620B/SHAecc88119b62e9a9311db1dcc7860082a1b18131d584b66889ab6fa1defdf56d8：default35/core111/bridge28/facade44/noBridge selected1+24filtered各原profile通过，不重叠求和；managed restart case ok不倒推原passive-exit原因已查明。两job warning/error annotations均0，只用于各自原切点，不称全部CI或SDKmain已过。Admin language-host-success-facts.json保两job已成功组合；单独language-terminal-verification内Host未终态的资格文字是较早收据阶段，不重写原JSON或误作当前阻断。

|当前测试源码（e58与原修复Git字节同）|SHA256|
|---|---|
|`sdks/python/tests/test_tool_value_effect_host.py`|`62bc5fb0a8f5512060ce031303a80cbe1d0b1d1a1fad79c317aa53ac866d06ee`|

仅这些测试源同原c5/ff3。原focused使用较早客户端，TS/PY生产69cb修复了另一个ACK竞态；e58完整官方脚本的实际合并树与输入闭包补足新客户端组合，不把旧source哈希不变推全部输入不变。原强消费者断言在未过滤完整脚本执行，Host0f1只属于原Mac focused，不套远端Linux Host二进制SHA。

具体Finding局部三链引用本段及对应Audit具名故障复核，仍保Host未知退出、B三宽义务、accepted12/合同13、平台/main、全局continuity1173和4365档案必要性精简阻断。没有新runtime状态权威/validator/协议/accepted schema/公共API；examples和官网无需接口同步，代码/测试本次不改。

唯一current页只维护本次两个具体局部闭合与现有余项：`docs/sdk/governance/capability-review-remaining.md` candidate SHA `1112c4dc5dbefb8e511b2b52aa51845eab388082b0b1bbd4a7994bea4bdf83be`。它是文稿维护依据，不是额外能力或物理回收全称证明。


## 历史证明存储的有界表示迁移

原id/业务正文/status/supports/limitations/命令及原观察含义不改变；本冻结父 `7f14a1b9c751b0612ceb42224f83f29d09aea46c` 中此对象完整原header/source refs可由公开证据tag恢复。只将具体archive物理依赖交当前位置资产与真实same-role consumer，全部旧义务由显式Eq/CT承接，不把存储重观察冒原业务新审或旧失败变成功。
