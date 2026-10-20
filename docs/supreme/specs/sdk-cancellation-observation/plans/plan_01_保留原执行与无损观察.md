---
schema_version: 4
slug: sdk-cancellation-observation/plan
outcome:
  summary: SDK 沿原 accepted routes 无损报告 B 观察且保留唯一原 physical cleanup owner
  acceptance:
    - 原setup/流内观察、background join unknown和facade
      heldcleanup强反例修复后，原RPC及时返回framework_error/Never/完整identity，不虚构terminal或释放owner/quota，不重复副作用。
    - 上游公开B精确main
      CI通过后固定pin，唯一生成全28profiles，再真实Host三语言完整入口/适用Rust门禁/独立复审0，raw只local。
    - 用户已批准A：仅单protected_claim_v1与workflow_checkpoint_protect_claim
      Unit新增；原Graph绑定Storecap在claim/forksave/effect前拒缺失，原Store
      exact-attempt保护及CRUD/恢复合同由framework和语言provider落实，真实before/after、三语Host及独审待执行。
out_of_scope:
  - Root已授权源码准备及精确Protocol-only before窗口；后继Cargo/Host/语言验证仍按唯一租约，不自行改framework
    pin/lock或生成输出，最终以精确公开上游revision进入整体验证。
  - 不在SDK实现framework/application
    B或Store/Graph保护权威；仅实施用户明确批准的A单cap/单reverse合同，不加其它identity/status/registry/store/scheduler，不改Sandbox固定Item为Result。
  - 不承诺丢失receipt后的eventual恢复，不重join/execute取结果，不自行merge/publish或清历史档案。
design_ref: docs/supreme/specs/sdk-cancellation-observation/design.md
design_sections:
  - ref: design.md § 问题与权威
    digest: sha256:3629b22caa4b967168f42e9ddbb093c22806b31c6e4cb233b4c2cdfb3279d92e
  - ref: design.md § 无损错误与 Task/Subagent owner 设计
    digest: sha256:188f5ee239ad42ce6df094254ed3d6ca9ad1f4dbd900c813c93492f246c016af
  - ref: design.md § Callback、Factory 与真实晚到 closure
    digest: sha256:48f1c787f1c992df8cba5791709d68b5c16142a679a117c060f6cf5a03ac2684
  - ref: design.md § 原 producer 与验收
    digest: sha256:7277c0c031f4994c3d0fc98680a2818ce5c6aacb01a73cb6c8dbdfb192800b90
delivery_ref: null
todos:
  - id: preserve-observation-and-task-owner
    summary: 无损适配fallible错误边界与原Task exact请求及内部owner等待
    files:
      - echo-sdk-host/src/core_profile/wire.rs
      - echo-sdk-host/src/core_profile/facade/task_runtime.rs
      - echo-sdk-host/src/core_profile/facade/mod.rs
    acceptance:
      - 双向完整None/Some identity/普通cause；真实setup/stream
        decoder前保存坏观察uncertainty，RPC SerializationViolation与原context typed
        identity分开，无identity保原owner等待不得Other终态；required hook只委托现exact
        CAS，false/IO不假成功；只一次execute_to_settlement，unknown不置completion
        settled或removeowner。
      - 搜索并迁移Host全部直接/into-style terminal转换，观察不进入Task resolution、官方session
        update Error或Runterminal，不增语言Rust同名API。
  - id: retain-subagent-observer-record
    summary: singleconsume后台join观察不成为terminal和容量回收事实
    files:
      - echo-sdk-host/src/core_profile/facade/subagent.rs
      - echo-sdk-host/src/core_profile/facade/mod.rs
    acceptance:
      - collector只消费一次join；unknown保record/executor/handle/quota且唤醒await，cancel/close不删；真terminal对照回收；不用is_finished/二次join推physical。
  - id: retain-callback-factory-and-source
    summary: 原callback、Tool/LLM和facade源在观察及取消后持有physical cleanup资源
    files:
      - echo-sdk-host/src/core_profile/extension_bridge.rs
      - echo-sdk-host/src/core_profile/facade/stream.rs
      - echo-sdk-host/src/core_profile/facade/workflow.rs
      - echo-sdk-host/src/core_profile/facade/mod.rs
      - echo-sdk-host/src/core_profile/handles.rs
      - sdks/typescript/src/client.ts
      - sdks/python/src/echo_agent_sdk/client.py
      - sdks/java/src/main/java/com/echoagent/sdk/EchoAgentClient.java
      - sdks/java/src/main/java/com/echoagent/sdk/ExtensionStreamWriter.java
    acceptance:
      - setup Err和ACK后观察不记remote terminal、不drop lease/context/Factory
        Arc、不自动AgentClose；wrongowner/gen不能清，原真实lateclosure一次释放，无selfcycle/假永久task。
      - workflow取消/发送失败drain原source，facade观察不生成failed/EOF/terminalsequence，deadline不abort；Sandbox及Tool固定Item不造IoError/Failed/ToolResult/Complete/fakeEOF、不改Item；ToolFailure前拦观察，LLM透typed
        Err但低层lease仍保，缺receipt保持unknown。
      - Store/managed有合法ReactError槽时透观察；Hook/intervention/callback/TaskStore等无槽仅显式观察保原owner等待，不造Unit/deny/Backend；普通best-effort和unknown-commit政策保持。
      - 原公开writer/notify/回调controller仅完整Host合法framework_error/Never/not_settled保持open及原取消地址，局部cancel不覆观察；畸形/近似和普通failed仍终态，Rust空白/Unicode
        scalar/duplicate/所有已知字段界限同Host。
      - 新增Workflow两个owner分开：F Graph及其CheckpointStore负责NotSettled
        claim/requeue/heartbeat/expiry/restart，SDK不复制；非流workflow.extension.run
        stringify生产先保留，真实新pin强red后仅typed观察走唯一wire projector，ordinary原错误政策保。
      - 已批准A：CheckpointStore proxy只委托protect_claim及cap，Graph
        getter投影原绑定Store；缺false仍允许注册/无关操作，受影响Approved/Modified
        resume与branch在claim/forksave/effect前canonical
        FeatureUnavailable/Never。SDK不复制保护算法/CRUD authority。
      - A Host proxy只读原注册cap snapshot，protect仅单原typed reverse→sameop
        Unit。Graph缺cap canonical
        guard与direct-run投影生产保持，待公开Fmain精pin/唯一producer后原Host有效red先行；不以编译失败或旧schema握手失败冒red。
  - id: prove-existing-public-consumers
    summary: 原受控反例和同一真实Host三语言corpus证明观察与物理事实分离
    files:
      - echo-sdk-host/tests/extension_bridge_e2e.rs
      - echo-sdk-host/tests/core_profile_e2e.rs
      - echo-sdk-host/tests/facade_feature_adapters_e2e.rs
      - sdks/typescript/test/factory-host.test.js
      - sdks/python/tests/test_factory_host.py
      - sdks/java/src/test/java/com/echoagent/sdk/FactoryHostTest.java
      - sdks/typescript/test/task-subagent-host.test.js
      - sdks/python/tests/test_task_subagent_host.py
      - sdks/java/src/test/java/com/echoagent/sdk/TaskSubagentHostTest.java
      - sdks/shared/testing/task_subagent_fixture.py
      - sdks/typescript/test/stream-cancellation-routes.test.js
      - sdks/python/tests/test_stream_cancellation_routes.py
      - sdks/java/src/test/java/com/echoagent/sdk/StreamCancellationRoutesTest.java
      - sdks/shared/testing/cancellation-observation-cases.json
      - echo-sdk-protocol/tests/extension_contract.rs
      - sdks/typescript/test/typed-bridge.test.js
      - sdks/python/tests/test_lifecycle.py
      - sdks/java/src/test/java/com/echoagent/sdk/TypedExtensionTest.java
      - sdks/shared/testing/workflow_checkpoint_fixture.py
      - sdks/typescript/test/workflow-checkpoint-host.test.js
      - sdks/python/tests/test_workflow_checkpoint_host.py
      - sdks/java/src/test/java/com/echoagent/sdk/WorkflowCheckpointHostTest.java
      - sdks/typescript/test/llm-workflow-stream-host.test.js
      - sdks/python/tests/test_llm_workflow_stream_host.py
      - sdks/java/src/test/java/com/echoagent/sdk/LlmWorkflowStreamHostTest.java
      - sdks/shared/testing/llm_workflow_stream_fixture.py
    acceptance:
      - 原模块unit先old red再samecase
        after；真实consumer完整error/Runget无terminal/claim资源/执行和factoryclose次数/原release副作用和peer隔离；source/binary精确，不以case名或总数替代。
      - Root先准入必要shared
        fixture/原E2E路径和唯一producerlease；首失败保留，普通成功/失败/取消与unknown对照同测。
      - 三client原sender隔离before真失败后samecase
        after0；真正Host强例用Session.start_run/Run.get无terminal与入站后同PID物理producer真实release/join，不以Eval.failed或Host退出证明。
      - 同一真实Run区分setup保留issuedstream但故意不发后继terminal与stream真实续传：held时Run无terminal且Session原准入仍拒绝；stream真实release/join+matching
        FinalAnswer/EOF后原framework独一receipt/结果/replay终态一次且execute一次，setup只backend退出且未发Complete不补Host
        receipt，不冒充根本无source。不把stream永久Running当成功。
      - facade真实ACP/profile Workflow direct-run强unit源码经实际reverse
        callback；原工作callback内准入、guard/Join仍held、原invocation地址保、完整观察字段与ordinary消息/无details对照；新pin租约后先真实projection
        red再修/after。当前未编译/执行，不用旧Host或缺variant失败代替。
      - A协议/三语旧源码先实际JSON解码/公开dispatcher
        red（非不存在类型编译红），原cap未知字段strict与已知legacy控制保；实现后同case完整Unit/identity/wrongcomponent/wrongtype/oldHost组合与真实Host
        preeffect零副作用等精确验证。
      - A
        optional同corpus：Graph绑定legacy后注册新Store不换cap，4入口preeffect拒且fresh/Deferred/Rejected控制保；protect提交后持住真实Unit回应Node0，释放后sameattempt
        Node+ack；protect提交后显式Error不准Node/ack/requeue（非网络丢包证明）；实际registeredLlm
        callback内原Popen/foreignIDs未知时不重领，原PID真release/join也不伪Hostreceipt。原router
        corpus业务oracle保，Javahelper仅真10s超时清理；本阶段全部native未运行。
  - id: pin-generate-and-document-approved-slice
    summary: 上游公开B后整合精确pin和唯一合同生成并维护必要长期说明
    files:
      - Cargo.toml
      - Cargo.lock
      - contracts/sdk
      - docs/adr/0039-sdk-cancellation-observation.md
      - docs/adr/0034-sdk-governance-authority-and-lossless-adaptation.md
      - .echo-semantic/maps/map.sdk-integrations.md
      - .echo-semantic/findings/finding.sdk-workflow-resume-observation-claim.md
      - .echo-semantic/findings/finding.sdk-workflow-direct-run-observation-projection.md
      - .echo-semantic/evidence/evidence.sdk-workflow-observation-source-gaps.md
      - docs/sdk/governance/finding-ledger.md
      - docs/sdk/governance/capability-review-remaining.md
      - echo-sdk-protocol/src/methods.rs
      - .echo-semantic/evidence/evidence.sdk-workflow-protected-claim-human-decision.md
      - docs/sdk/governance/workflow-protected-claim-proposal.md
    acceptance:
      - Root单独移交pin/lock/生成路径后，只固定公开main且精确CI通过revision；唯一generator全28profiles，只显式接受用户已批准A单cap/单reverse增量，保其它accepted范围/原wire路由操作合同，B的Rustsource签名摘要真实更新审diff不冻旧digest/不接无关漂移，真实identity/kind缺口具体报Root。
      - ADR及必要用户文档解释Never和原owner观察，examples/官网判适用；新源适用全门禁及独审后交付，main不增raw/sourcecopy，尚未运行/限制如实记录。
      - 已接受A决策及原23c5提案历史必须可复核；公共Checkpoint字段不变，不新query/unlock/version协商，不把Rust/Host-only
        getter自动成语言API；旧File reader
        failclosed与旧delete/clear绕过限制、Memory实例durability和长期blocked成本文档准确。
artifact_id: plan:e02b9311-af1a-408b-80a1-8351b200d3c5
lifecycle: ready
---
## Notes

按Root明确handoff执行，当前019e基准；Todo1–4先源码与强测试准备，Cargo/native需租约，Todo5须公开framework main及精确CI通过后才固定pin和唯一生成。局部源码准备不代表五Todo或整体Outcome已完成。


2026-10-16用户明确接受A protected-claim合同，新增工作并入原Todo3/4/5；不是已执行或免门禁，不改变独立SDK direct-run Finding。

A Host可选三语场景只source准备，发布依赖与未证范围见Design；旧45 focused/runtime证据保持原epoch，native/Host新API编译待公开Fmain和Root租约。

2026-10-16后继同范围反例：Workflow extension观察后须转发原late Completed完整结果及顺序，三语言复用原native suites/shared fixture；observer在unknown前先真实unwind的同原callback/guard/lease/quota顺序须先red后修复。它们不新增API、执行权威或任意Graph晚到结果承诺。当前实际局部验证与未验范围以MASTER-PLAN当前检查点为准，原阶段Notes保历史。

2026-10-19当前实现与本地验证已完成：原A/B五Todo的代码、公开framework6563精pin/唯一28profile117生成/完整合同、强反例和原owner验证、8481适用14Rust零警告、新Host098212以及同原官方完整三语入口(354/440加18subtests/377fresh76XML/3quickstarts)和独立有界复审均通过。原绑定Design依赖链末尾的SDK精确main仍未验收，属于本Plan原scope，因此lifecycle保持ready；不能用本机通过代替该原验收。上方source准备/尚未运行Notes保历史。待最终正式材料/当前canonical strict及复审通过、PR合入后精确SDK main CI和必要冷仓验证完成，再用官方artifactCLI标completed并对生命周期文稿做同一最终快照验证。managed历史Linux退出超时根因仍未知，依原Finding追踪；NativeHook/Stop和平台验证不凭本机数据伪填。
