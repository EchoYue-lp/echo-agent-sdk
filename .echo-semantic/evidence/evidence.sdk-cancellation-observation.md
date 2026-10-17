---
schema_version: 1
id: evidence.sdk-cancellation-observation
kind: evidence
observed_at: source:83c46c4f864bffc071006f0ba8d79faedbb2e17f0c5f86155411930b9aaad4c5
source_refs:
- echo-sdk-host/src/core_profile/extension_bridge.rs
- echo-sdk-host/src/core_profile/facade/stream.rs
- echo-sdk-host/Cargo.toml
- Cargo.lock
- scripts/check-sdk-contracts.sh
- echo-sdk-host/tests/extension_bridge_e2e.rs
- sdks/typescript/test/factory-host.test.js
- sdks/python/tests/test_factory_host.py
- sdks/java/src/test/java/com/echoagent/sdk/LateFactoryHostTest.java
- sdks/typescript/test/llm-workflow-stream-host.test.js
- sdks/python/tests/test_llm_workflow_stream_host.py
- sdks/java/src/test/java/com/echoagent/sdk/LlmWorkflowStreamHostTest.java
- sdks/shared/testing/llm_workflow_stream_fixture.py
- sdks/typescript/test/protocol-host.test.js
- sdks/python/tests/protocol_host_proxy.py
supports:
- finding.sdk-cleanup-retry-owner
- rule.framework-runtime-authority
limitations:
- Seven focused Rust cases cover callback binding, response projection and physical cleanup only.
- No Host Run settlement, foreign execution receipt, restart or entire A/B SDK acceptance is established.
- The saved-profile Host build is independently verified; complete Host/language/platform qualification remains separate from the test binary.
---

# 原 callback 观察与物理回收的有限验证

## 支持的结论

原 callback 在唯一发送前绑定 facade record；sole collector join 不能越过仍未闭合的 callback。错误 Stream ACK、无 peer 证明的普通解码错误只投影原 unknown，缺合法身份时保留同一 source。已有 typed/raw observation 保留其原 foreign 身份。普通真实 final Error 则先释放原 guard/context/lease，再让消费者看见错误；producer join 后原配额可回收。没有新增执行、claim、terminal 或回执权威。

## 来源与范围

### 原callback绑定与投影反例

真实 ACP/CoreProfileState/唯一 invoke 路径的 observer-unwind before 执行1例失败，close提前成功且quota/address逃逸；日志SHA256 `bcbe8583191388edaa4882bcbeb7078ca07d5af7ceb0c5b5a6f11528c093d49e`。另一个真实 wrong-ACK before 执行1例失败：lease/配额保持，但消费者收到普通ExtensionFailed；日志1153B/SHA256 `bb265d01769004e09e0f4b81891e2aa8235b60505aab7bf39a9174b24d347fd1`。两例均先释放同一原回调并观察资源回收，再报告失败；不是编译失败、timeout或transport teardown证明。

最终冻结的 `extension_bridge.rs` SHA256 `61262f35a24c799726e53778cf61301e0633210a06127500baf823b9b78d5889`、`stream.rs` SHA256 `da86f2f99640efe157a9482a5344029ecd511ae56d08bb91618ef2db52b9f521`对应完整来源manifest。优化测试二进制SHA256 `cd62e0d89987052f6438c2162f94363ce420ad9b354b56287803a590dcafa752`，171027408B。原unwind、wrong ACK有/无identity、ordinary final Error typed/raw、context无run的foreign observation typed/raw共七个完整selector，各实际1pass、0fail、0ignore，退出0、无signal、无截断。六控后1209项非忽略输入零漂移；原unwind最后复用同不可变产物补验，日志244B/SHA256 `86bebb839cb6738afac540ffe2a41afa64e7019eb478abbe427e0c521cc5a717`。

原件仅在本机ignored `.echo-semantic/local/sdk-parallel-delivery-01a14385/`：`callback-seven-controls-result.json`、七份runner/raw log、源码manifest、before/after不可变产物。完整raw不提交；哈希不代原件。独立reader核六控原件及源码返回局部pass，后继第七项为主代理实际补验。

### 满队列取消后的原source drain

C1整体独审发现Workflow façade转发的容量1队列满时，第二次sender.send不观察cancel，导致无法进入原source drain。正式ACP/profile原Workflow测试使用ACK、原remote sequence2、反向邮箱已空、facade发送容量0、尚无terminal共同证明进入条件，没有靠sleep或额外pull解堵。先cancel并验证held close未回收，释放同一原gate并等待原callback finished/lease0，旧实现close仍Err、配额仍满，目标产品red成立。before完整日志1599B/SHA256 `d9c6611ba813ec33a0ae83a3b05c8c9e82e794dc56d4efabf867f7daa0d0aa71`实际2fail，其中一chunk控制仅因错误要求ephemeral close必须Booltrue而失败，实际OkBoolfalse/配额已开；两个原因分别保留，不把控制oracle问题冒产品red。

最窄修复只将阻塞发送改为cancel/send select，取消后进入既有同source drain；oracle接受合法Bool并仍检查原handle tombstone、同registry配额重开与再次close false。同两例after2pass/0fail/0ignore，完整718B/SHA256 `427a29cafc2550453edb3085bcc524cfc9d3120a83527224aff7c897ffb92e4d`；1212输入零漂移，不可变before/after二进制分别f2fe325cbd5fbd18c72d2749d5768489bb957e9375163616ec680977767787f4/e313e1e4e3ab8e21279165b40a9827e028da47e2543d6c73b26c38f20ce7d701。增量reader核源码/实际原件返回pass、行动项0。最初测试私有字段访问的编译失败保留，未算行为red。

### 最终配置共同cut定向

在下述14a Host对应的保存配置与同一Rust源码上，新测试binary SHA256 `f6584098564c0d86e3068cb33741bbdeb3c6c2f892d7eca1b72cea4f24baf80c`、171490848B；同两drain例再次实际2pass，日志718B/SHA256 `4a73c9325d0df2254e0df697047ac3defc633c93e12f694af33a9879bf0c9697`。同binary原unwind完整selector实际1pass，日志244B/SHA256 `7ee59f0d9319f64a150b52410a7b7ae5c16aff4b1c6839927f55ad8fec6bee89`；direct-run完整selector另实际1pass，由Workflow Evidence记录。均exit0/无signal/无截断/零warning，63项编译输入零漂移。此处没有重跑旧red或把旧七控计数累加成整体测试数。

## 已知缺口

### 最终配置Host的独立实际构建

原四dev键唯一移到.cargo/config.toml后，从SDKroot实际构建零警告Host14a0cb58c44bdb84990262f1324e1a2513513a8b40fc0640a9d36f91f95e27e9，159609616B；EH15488572低于16MiB，unwind_info2979480保。完整1550567B日志SHA256 `1167645df095cf3b370c820f96305562820e1c27a8d2c4ee44c41cbd7d19ea15`，exit0/无signal/无截断。63项Rust/Cargo/config输入零漂移，174项实际编译输入与精确C2候选逐字相同。无OPT/ASSERT/OVERFLOW/PANIC及RUSTFLAGS/ENCODED/targetFLAGS覆盖；原jobs1/debug0/strip/inc0资源设置保留。此构建不直接替代新共同cut的运行验收；原8bf与更早45fc结果保各自切点，详见ADR0040。

当前header绑定精确staged Git候选，不是混入未stage语言准备的S whole-source。候选174项Host/protocol/合同实际编译输入与最终配置Host及上述测试所在S逐字一致；其余9个独立未验native/fixture/pipe诊断路径未纳此候选。原七控的Rust内容身份与旧epoch保持原样，候选原始身份/内容对照仅在本机local保存。

profile后继文稿/配置变化不冒充旧whole-source测试；测试所编Rust内容身份单列。保存配置已实际构建零警告Host8bf4a1d6deb647e608153c807ca07785dd31f7d3223a88cc3daf9b835b8bff5c并重新测EH/unwind，必要unwind定向也通过，实际来源见ADR0040。当前source观察包含尚未独立交付的语言准备，checkpoint需再按实际提交引用核对。宽cleanup Finding保持open；B真实Task/foreign场景、Java关闭根因、完整Host/Rust/三语、语义strict/nativeHook/Stop及精确main CI另行验收。内部修复不增加示例调用或官网功能内容；正式约束由ADR0039/0040和原extension-bridge文档承载。


## F6563/ac433 的有限后继（2026-10-17）

本次 header 只标识下列有限后继源码再观察；原 header Git335193791c70a6cb077033c3f8f0cb1223e5266e 及上方七控/14a/C2正文、结果、限制全部保持历史，不把它们计为新 Host 验证。当前宽 cleanup、Sandbox、Task、Graph、managed 五 Finding 仍 open。

当前 Host 固定公开 framework6563d26570cc4a8d93c86b7e37443b0559476807、真实锁90d5b82ef54a56d22fc079f9065e86e817a81b2431e265bf1d47ebe3897f40d5；八framework和三vendor仅Git来源变化。唯一生成28profiles/117产物及完整合同入口实际0，accepted5621 identity/kind与454routes双向差0、五个blocking/shared文件逐字不变；唯一新RunEntry::wait_result仅Host/Rust遥测。生成log26893B/caea4f0218d7fc2f6509eb400d1c8d2b7c252e7695b8770387c8f3240f88ad1d，check14763B/76e9b990da3f03e5e0820f9d474d383dfab593d22631ed900409912a92a3626f；Protocol5/42/91实际通过。

SDK root 保存配置真实 build0/零warning，独立Host ac4332105734bbedda144b5c033fedc6610de4b5b9d9a5a0c11dd4953a28a87f，159627472B、EH15491256/unwind2978664；保opt1/assertions/overflow/unwind，无行为flags覆盖。原63 native输入0漂移，其中cfgtest文件不宣称均被生产编译。build106727B/33a1467283954e86b67001f980ca7fdb60b97c0276f677973a9e3d95082a1273。

原stream完整selector cancelled_pull_during_failed_cleanup_preserves_error_and_terminal_sequence 只增加同一次dropped waiter/held cleanup/唯一quota1联合断言；cargo test对应原case实际1pass/0fail/0ignore/218filtered，612B/069aba5f74d284ba7a530bf4eaa031ff636779f7e54797c9f8c3fd919c0842e4，同binary8a2ff62751ad2d7d624652dc96d53b409af36c51c99409e8e690b43998a05729。原Sandbox case旧fixture永不回答callback却期待terminal，首1fail不是生产red；同responder实收cancel后held无terminal/cleanup0，释放原Cancelled response后原250ms cleanup/10s finalWait与cleanup1/unregister通过，1pass/0fail/0ignore/33filtered，580B/89fa9c2e00d4706c52e15d435ebe2ec40ff276b2837388a4afe9cec17ff1fa7b，binary4bfab2b8fab8ae93695d0aa0380f043003f0bc570f003886286ff6effcf82116。两次1212输入0漂移，独审各C0/I0/M0；末child.kill只是harness teardown，不证明managed自然退出。原同文件Linux pipe诊断已随测试binary编译，具体失败路径未在本机触发；不凭此关闭Linux10秒passive-exit未知。

原 scripts/check-language-sdks.sh 新Host首轮真实exit1：TS354=351pass3fail0skip，PY/Java未运行，112782B/7e3fa45a75643464d3a0a8c6058c0d136e51845dc380fbf7d79f7a324d837b48及23失败raw原件保留。独审判三处旧oracle；只改原Factory/Workflow/Tool测试和原proxy方向观察。Factory实际cancel signal后原constructor仍held/created0/无AgentClose，父close拒绝；释放同一constructor后原RPCid全部result/error response恰1/no top error/typed Cancelled，same child cleanup1/execute0/failed Eval。Java原cleanup future独立token不因invocation timeout被虚假要求取消；先观察原iid真实timeout、父仍拒绝、future仍held，再release/calls1。Workflow原producer ACK后首chunk确认，held gate释放后实际writer.complete，cancel/close各terminal1、producer完成4/callback5/errors0。Tool实际signal后held wait无terminal/effect0，释放原callback再原5s finalwait/250ms cleanup，不改变生产期限。

02 TS3/PY2/Java3实际通过，日志1920B/fdc9d7518917d95d3994cb535b3fd4d4ab92ad199c231b415882000dd95348f3、98B/03e983c5cda2ebfdb045236a197c294256eaf09974a6e96a2669e21cfd7bcc97、1317B/89a90b95c4e69738c850dad7bfd05ba07d9f48f7dd1654133bbd44d1a9004e8e，freshXML3/0/0/0。其后仅三Factory增加完整同id response唯一性：03 TS1/PY1/Java2全0，1182B/e7bb5601b0222d3d478c9af4e460b8ab4991e943df3488ddf842cee3845480d3、98B/ec3c61992e3bb1a5aac88c5f3c5619a6eccaa445b8e776896e3c8c676f9f6278、892B/c2283af6efe58c55a9cd457cd66441915e8e1ac10c2c3964435fe007faeb5e63；freshXML2/0/0/0 SHA734fef9746eea12bb135c8e732ce577ad7afc5114f11413efd1e72402b52b90a。03 patch3bd9c741a06e56bdb9a59cdfffcfc415229d3d6ea40ab175327bbf39ae96bf17、1212输入0漂移、23raw/58617B/0mismatch、Hostac433不变，独立源码与真实帧复审C0/I0/M0。02其余六文件逐字不变，Workflow3/TS Tool资格保其原02切点，不冒重新执行。

上述成功命令原终态exit0/nullsignal/notrunc；上述哈希不代原件。runner、完整日志、freshXML、帧原件及source输入/精候选依赖对照只保本机ignored .echo-semantic/local/sdk-parallel-delivery-01a14385/ 下 framework-6563-*.json、oracle9-focused-02-results.json、oracle9-focused-03-results.json 与其具名raw目录。当前提交候选排除四个未完成完整PY/Java验证的foreign Task dirty，保fa62 blob；它们不在本段selected测试/Host编译/生成的真实消费闭包。未称整个dirty树或官方full语言通过；原full入口、完整Rust/平台/main及五Finding余项仍须继续。示例无新增API调用，完整quickstart仍由官方入口验证；官网/顶层由Root负责。
