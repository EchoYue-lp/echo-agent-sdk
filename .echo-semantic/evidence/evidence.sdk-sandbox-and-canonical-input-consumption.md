---
{
  "schema_version": 1,
  "id": "evidence.sdk-sandbox-and-canonical-input-consumption",
  "kind": "evidence",
  "observed_at": "source:0e84c61ef79502743c58bf928b9a9ff208621580323d7ca6efb01a21f242731b",
  "source_refs": [
    "echo-sdk-protocol/tests/sandbox_command_projection.rs",
    "echo-sdk-host/src/core_profile/sandbox_command_owner_tests.rs",
    "echo-sdk-protocol/src/facade.rs",
    "echo-sdk-protocol/src/inventory.rs",
    "echo-sdk-host/src/core_profile/extension_bridge.rs",
    "echo-sdk-host/src/core_profile/facade/source_operations.rs",
    "sdks/python/src/echo_agent_sdk/permission_mode_values.py",
    "sdks/python/tests/test_permission_mode_values.py",
    "sdks/python/tests/test_retired_delete_boundary_host.py",
    "docs/adr/0044-sdk-source-disposition-native-callback.md",
    "docs/adr/0045-sdk-permission-mode-canonical-inputs.md",
    "echo-sdk-protocol/src/scalar.rs",
    "echo-sdk-host/tests/core_profile_e2e.rs",
    "sdks/java/src/test/java/com/echoagent/sdk/A2AValueTest.java",
    "sdks/python/tests/test_jwt_values.py",
    "sdks/typescript/test/jwt-values.test.js",
    "docs/adr/0042-sdk-a2a-lsp-retirement-boundary.md",
    "contracts/sdk/accepted-external-contract.json",
    "contracts/sdk/accepted-facade-operation-catalog.json"
  ],
  "code_refs": [
    "echo-sdk-protocol/tests/sandbox_command_projection.rs",
    "sdks/python/tests/test_permission_mode_values.py",
    "sdks/python/tests/test_retired_delete_boundary_host.py"
  ],
  "supports": [
    "behavior.sdk-extension-bridge",
    "behavior.sdk-language-clients",
    "rule.framework-runtime-authority",
    "finding.sdk-a2a-lsp-consumer-retirement"
  ],
  "limitations": [
    "仅记录具名 Sandbox 值投影、原物理 owner、精确八模式输入及退役 DELETE 的消费证据，不提升整体能力或场景状态。",
    "d683、035、75a 的执行资格分别保留；正式源通过内容桥承接，不把旧阶段重标为新源重新执行。",
    "本地原件只在当前工作区可用；当前精确 SDK main CI、平台新原生、全局未知及发布仍独立验收。",
    "本轮source0e84只绑定正式83工作树的当前代码观察与after-only工程结果；候选7978、旧source32f和原d683/035/75a各守其原纪元。",
    "Rust完整589与三语言各2定向并非完整三语言重跑；没有before轨迹对照，不新增behavior_equivalence或matched场景。",
    "本轮独立strict相对83只检当前增量，不替代main747接收的normal Source/Final、canonical删除预检、fresh风险、公开来源、平台或GUI验收。"
  ],
  "command_results": [
    {
      "command": "/Users/ls/.rustup/toolchains/stable-aarch64-apple-darwin/bin/cargo fmt --all",
      "exit_code": 0,
      "scope": "本机正式83当前工作树的after-only结果，不是before重放或完整SDK交付",
      "raw_log": "/Users/ls/MyWork/code/ylp_agent_learn/lp-agent/echo-agent-sdk/.echo-semantic/local/sdk-candidate-landing-20261104-r01/format-write.raw.log",
      "raw_sha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    },
    {
      "command": "/Users/ls/.rustup/toolchains/stable-aarch64-apple-darwin/bin/cargo fmt --all -- --check",
      "exit_code": 0,
      "scope": "本机正式83当前工作树的after-only结果，不是before重放或完整SDK交付",
      "raw_log": "/Users/ls/MyWork/code/ylp_agent_learn/lp-agent/echo-agent-sdk/.echo-semantic/local/sdk-candidate-landing-20261104-r01/format-check.raw.log",
      "raw_sha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    },
    {
      "command": "/Users/ls/.rustup/toolchains/stable-aarch64-apple-darwin/bin/cargo check --workspace --lib --no-default-features --locked --offline",
      "exit_code": 0,
      "scope": "本机正式83当前工作树的after-only结果，不是before重放或完整SDK交付",
      "raw_log": "/Users/ls/MyWork/code/ylp_agent_learn/lp-agent/echo-agent-sdk/.echo-semantic/local/sdk-candidate-landing-20261104-r01/featureless-library.raw.log",
      "raw_sha256": "78e23a9de04bb4d9a52f3c5273228236d056f2448835ca41275a909a8b49108a"
    },
    {
      "command": "/Users/ls/.rustup/toolchains/stable-aarch64-apple-darwin/bin/cargo clippy --workspace --all-targets --all-features --locked --offline -- -D warnings",
      "exit_code": 0,
      "scope": "本机正式83当前工作树的after-only结果，不是before重放或完整SDK交付",
      "raw_log": "/Users/ls/MyWork/code/ylp_agent_learn/lp-agent/echo-agent-sdk/.echo-semantic/local/sdk-candidate-landing-20261104-r01/strict-clippy.raw.log",
      "raw_sha256": "0c98c517cc2d9a68b5f48eb5df79f939655cc15b6fa543df6b732debfe7497da"
    },
    {
      "command": "/Users/ls/.rustup/toolchains/stable-aarch64-apple-darwin/bin/cargo test --workspace --all-targets --all-features --locked --offline -- --test-threads=1",
      "exit_code": 0,
      "scope": "本机正式83当前工作树的after-only结果，不是before重放或完整SDK交付",
      "raw_log": "/Users/ls/MyWork/code/ylp_agent_learn/lp-agent/echo-agent-sdk/.echo-semantic/local/sdk-candidate-landing-20261104-r01/workspace-all-targets-all-features-r02.raw.log",
      "raw_sha256": "c7d0707134105becf63df480985acb2532de6fc63e8873f272954d4756a29546"
    },
    {
      "command": "/Users/ls/MyWork/code/ylp_agent_learn/lp-agent/echo-agent-sdk/sdks/python/.venv/bin/python -B -m pytest -q -p no:cacheprovider tests/test_jwt_values.py",
      "exit_code": 0,
      "scope": "本机正式83当前工作树的after-only结果，不是before重放或完整SDK交付",
      "raw_log": "/Users/ls/MyWork/code/ylp_agent_learn/lp-agent/echo-agent-sdk/.echo-semantic/local/sdk-candidate-landing-20261104-r01/python-jwt.raw.log",
      "raw_sha256": "3251a439eaf420074405113236df804a09ae5c0383b2a37edb3f1466f72d4bbc"
    },
    {
      "command": "/Users/ls/.nvm/versions/node/v22.19.0/bin/npm run build",
      "exit_code": 0,
      "scope": "本机正式83当前工作树的after-only结果，不是before重放或完整SDK交付",
      "raw_log": "/Users/ls/MyWork/code/ylp_agent_learn/lp-agent/echo-agent-sdk/.echo-semantic/local/sdk-candidate-landing-20261104-r01/typescript-build.raw.log",
      "raw_sha256": "ecaabe720cdb6e456b6ea808f53ecdcf6b96fa864c581db00d35c2ce4fe6ee46"
    },
    {
      "command": "/Users/ls/.nvm/versions/node/v22.19.0/bin/node --test test/jwt-values.test.js",
      "exit_code": 0,
      "scope": "本机正式83当前工作树的after-only结果，不是before重放或完整SDK交付",
      "raw_log": "/Users/ls/MyWork/code/ylp_agent_learn/lp-agent/echo-agent-sdk/.echo-semantic/local/sdk-candidate-landing-20261104-r01/typescript-jwt.raw.log",
      "raw_sha256": "777ca02e806cdac8628d74c7ebe56b5e33dbc3aa2a8bb58fcd797d207c187b99"
    },
    {
      "command": "/Users/ls/MyWork/kits/apache-maven-3.9.0/bin/mvn -o -Dtest=A2AValueTest test",
      "exit_code": 0,
      "scope": "本机正式83当前工作树的after-only结果，不是before重放或完整SDK交付",
      "raw_log": "/Users/ls/MyWork/code/ylp_agent_learn/lp-agent/echo-agent-sdk/.echo-semantic/local/sdk-candidate-landing-20261104-r01/java-jwt.raw.log",
      "raw_sha256": "c33712918e4ed83b3a539cb7073ada3b868e905f2edfbc0b63260187a950145e"
    }
  ]
}
---

# Sandbox 与 canonical 输入的实际消费者证据

## 支持的结论

Sandbox 的 Rust 本机类型和资源仍由 Framework 拥有。SDK 复用唯一 descriptor，
仅投影原六个可序列化字段；原 `resource_guards`／`cancel` 不变成远程 handle 或
静默丢弃的输入。`echo-sdk-protocol/tests/sandbox_command_projection.rs` 的四项
控制检查真实 inventory、值 route、语言 target、三个 CommandKind、可选字段与非法
形状；其它 source/kind 不继承该投影。Host 的17项 owner 控制沿原回调 owner 检查
取消优先级、observer drop、stream、超时和实际物理结束前的资源保留。

PermissionMode 的策略仍归 Framework。`sdks/python/tests/test_permission_mode_values.py`
的61项参数化消费保留八个准确 ID、wire roundtrip、默认及 helper 真值，拒绝旧 alias、
大小写／空白、未知和非文本输入，不在 SDK 新建权限策略或退回默认值。

`sdks/python/tests/test_retired_delete_boundary_host.py` 的三项测试使用真实 Host：
本地三条公开入口拒绝 retired DELETE，前后只出现正控 agent.describe 请求；
未知／近似身份不误命中，已关闭 session 保持原关闭优先级；直接向 Host 提交旧
source／digest 的非法请求得到原 typed failure。它不重建旧 unmanaged 删除实现，
也不把拒绝响应当成新的 managed 删除能力。当前 accepted 从5352退役11并接受3个
具名 with_id 方法后为5344项／430路由；native 新增 Rust source 为零，边界见 ADR0044／0045。

## 来源与范围

Framework 来源为 HTTPS `340993201cc937fc8e4783b66be399ef9f14e3d5`，tree
`e6c28f37a881f41bb549452614313557b7c91c68`；SDK 锁为
`c4488065aa4a94a8a073e427aeddfefe7670d93b709b82555b39f2431954e1c7`。
Rust cfg 修复后的035冻结为 `03524059a9fbd2143881be09df9f0daab7494da236d22b3064dde7bed75da38c`，
清单为 `394dc720ef5f90ec4e6dbf465c95ab22f244c870fb898a44b4a8c0ee5bcc7a04`。
最后75a冻结为 `75a4c8308a17537f25240c046bf8ad27f48de5379083f045e3245606b83b3489`，
1302文件清单为 `2b866517fc5898d17a7e3c68e37ae21f4b8dfb7fe9cac7e9e6e79aff63bf4a2e`；
相对035只改两份 Python 测试的 lint／格式及 lambda 变量捕获，Rust 等1300文件不变。

以下均为已执行原命令与其原自然退出结果，不是本次文档编辑后重跑重型验证：

| 消费者 | 原命令与范围 | 实际结果及原件身份 |
| --- | --- | --- |
| Protocol 值投影与 Host owner | `cargo test --workspace --all-features --locked`，035 原第04项 | 全部588通过、0失败／忽略／过滤；其中上述 Protocol 四项及 Host owner17项的成功名称均在原 raw 中，不能再相加。raw SHA256 `6d1b332ae70a72cc5ed842d547de62e6f83f1db37282bd4e3f96631d04aee32f`；native receipt `19f3b9f9e130e50b2e6e86dc4545717625a676783ea50fee551fecde3620416c`，自然0。 |
| Python 八模式和真实 DELETE | 原 `uv run --no-project --with pytest --with pytest-asyncio --with agent-client-protocol==0.12.1 python -m pytest -q tests/test_permission_mode_values.py tests/test_retired_delete_boundary_host.py`，75a | 61＋3＝64通过、0跳过；真实 Host SHA256 `53fcac8445323a823d73c8636f5eef128c88511a79bb20c3faea8eb14ad78af9`。raw `57569a910006f88c8ae1e0153d3131e3f84799c20fa184bd175e481f961f25fa`；native receipt `dc7d091c03ba7da6aa7b3cad9fa3f6bf4480633c8044122d3b02173039ddc6c8`，自然0。 |
| 完整三语言、包资源和三个 quickstart | 原第18项 `/bin/bash scripts/check-language-sdks.sh`，75a、实际第16项正常 dev1 Host 与第17项 collector | r02完整入口自然0，TS473发现／472通过／1原 Windows skip；Python546通过 item＋18通过 subtest／1原 Windows skip；Java86 suites、345 tests、0 failure/error、1原 skip。Java实际JDK21、target17，不冒原生JDK17。raw `e1ef6b69fa6314a4f621f8ec7042aa2a371602c4fdf03e01c29b50546b588250`；native receipt `77679f14fe351c5714320f98abd68c3827d1ef20f37a7fdd97a469da4f94a084`。 |

原件统一位于本地
`.echo-semantic/local/sdk-u03-consumer-fixes-20261030/full-SDK-HTTPS340-execution-20261031-r01/`
之下：Rust 原第04项在 `full18-newC-revalidation-receipts-r01/current-cfg-fixed-full18-validation-r01/04-workspace-all-features.*`；
Python64原件在同一 current-cfg 目录的 `18-complete-language-execution-r01/ruff-test-fix-r01/related-two-Python-tests-original-dependencies-real-current-Host.*`；
完整语言在 `18-complete-language-after-python-test-fix-r02/18-original-complete-language.*`。
末次完整18封存 `original18-r02-complete-native-and-full-after-successor-seal.json`
SHA256 为 `1378e1871a2bb955bdd8d17f4b62ab06f0fe5e9b489e5392a15cbbe2ef6ecc10`；
分阶段来源索引在 `post18-readonly-formal-adoption-preparation-r01/full18-qualification-by-stage-and-source-epoch-sealed.json`，
SHA256 为 `eef63709f45815394212e3a7f1362d02ca9f8db312ac42d8b65abb20bfe4c471`。

完整18中第05／06／07项保留原d683来源及适用输入证明，不重标为035或75a重跑；
第17项是 Framework 独立锁1442下 collector 的 no-run 构建，不是 SDK 测试体。
各阶段有重叠，不合计为一个新测试总数。Python XML 的565统计与547平铺 testcase
差异来自原 pytest9.1.1 subtest emitter；原 f339 校验失败保原，完整 after 继任
`3629b1366cdeb5d8ee8d4c76d4d333d893c2817db4f47d4a5951f76642069dac`
实际自然0，原 XML、skip 及结果未改。

## 已知缺口

正式 SDK 以真实基准 `3fcb8d066e8a6b6560d7991f06b4255121f2a49b` 接回候选60文件。
代码、锁、工程配置、生成合同、测试和 examples 与75a内容身份一致；七份文档仅作
当前来源／统计／历史时态校准、ADR0044已有决策标题补全及 README 末尾空行清理。
随后本 Evidence 和两个原 owner Map 的 evidence_refs 在同一任务有界扩至63文件；
不改 Map 场景状态、Audit、Finding、Rule 状态或全局 baseline。
当前桥接、直接检查及正常 Source/final 的原件留在
`.echo-semantic/local/sdk-https340-full18-adoption-20261031/`，其自然结果单独核对。

原第08项编译失败、首轮18 Ruff失败、fixture guard1、Python原 f339 after1、
历史 source115／402，以及本次正常 Source 首轮4项均保留原件，不能反写为通过。
本 Evidence 不宣称这些工具缺陷已修复，不消费或关闭旧未知。查询返回的五项
needs_review／unknown、平台新原生、精确 SDK main CI、冷来源和发布仍是独立验收边界；
task Source/final 即使通过也只表示当前任务确定性合同成立，`globalClosure: not_checked`。


## 2026-11-04：当前正式工作树的实际验证

本节源码观察为 `source:0e84c61ef79502743c58bf928b9a9ff208621580323d7ca6efb01a21f242731b`；原 `observed_at=source:32f6f4ffd5e636129f9e2d8ace391a4af9b1012a5744b44816e896d0f44db333` 的历史
正文及d683/035/75a原运行不改。HEAD为83加四测试154行与一份SDK说明；保留
`core_profile_e2e.rs` 的HEAD沙箱夹具修复，生产、pin340、锁c448及生成合同不变。

五条原工程命令自然0，完整Rust18组589P/0F/0I；scalar定向4P已包含在全套中，不重复相加。
三语言本轮仅JWT退役合同各2P，TS先重新build。Rust真实剩余工具族E2E在全套中实际通过；
新旧运行前后等价未测，不新增matched声明。完整三语言、包资源和quickstart只保上方各原纪元资格。

首轮249P/18F/101是本轮运行器漏传既有 `ECHO_AGENT_SDK_CONSUMER_ROOT`；同程序同selector
缺目录101/补目录0验证后，只纠正环境，原完整argv重跑自然0。原101与诊断原件保留，未改测试、
断言或预算。原件在 `local/sdk-candidate-landing-20261104-r01/`，相对本仓 `.echo-semantic/`。
20程序及原FP/dep-info/build输出已保全，准确自有target官方clean0；不从构建、归档推导GUI或发行资格。
