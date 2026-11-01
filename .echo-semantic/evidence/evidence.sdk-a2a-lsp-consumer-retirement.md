---
{
  "schema_version": 1,
  "id": "evidence.sdk-a2a-lsp-consumer-retirement",
  "kind": "evidence",
  "observed_at": "source:e1864cc87ff524845908f5de781baa189dfed513a9b78551b23d5c59a0e2726b",
  "source_refs": [
    "docs/adr/0042-sdk-a2a-lsp-retirement-boundary.md",
    "echo-sdk-host/src/core_profile/facade/source_operations.rs#resolved_plugin_components_value",
    "echo-sdk-host/src/features.rs",
    "echo-sdk-host/tests/core_profile_e2e.rs",
    "echo-sdk-protocol/src/facade.rs#governance_source_disposition",
    "echo-sdk-protocol/src/inventory.rs#apply_accepted_source_scope",
    "echo-sdk-protocol/src/scalar.rs",
    "echo-sdk-protocol/tests/a2a_lsp_retirement.rs",
    "echo-sdk-protocol/tests/facade_inventory.rs",
    "scripts/check-a2a-lsp-retirement.py",
    "scripts/check-language-sdks.sh",
    "scripts/check-sdk-contracts.sh",
    "sdks/java/src/test/java/com/echoagent/sdk/A2AValueTest.java",
    "sdks/java/src/test/java/com/echoagent/sdk/ParquetRoundtripHostTest.java",
    "sdks/python/tests/test_a2a_task_state.py",
    "sdks/python/tests/test_jwt_values.py",
    "sdks/python/tests/test_parquet_roundtrip_host.py",
    "sdks/shared/testing/parquet_roundtrip_fixture.py",
    "sdks/shared/testing/retirement_plugin_fixture.py",
    "sdks/typescript/test/jwt-values.test.js",
    "sdks/typescript/test/parquet-roundtrip-host.test.js"
  ],
  "supports": [
    "finding.sdk-a2a-lsp-consumer-retirement"
  ],
  "limitations": [
    "behavior_equivalence仅绑定scenario_results中共同保留的声明合同静态对照。批准退出/拒绝和after-only运行单列普通事实，不计matched等价场景；before运行未观察，不声明两次执行相等。",
    "当前Source33、预检参数错误及原r11/r12/ruff/聚合/network失败全部保原；本四metadata独审与Source/Final另验。",
    "原r13 language caller0；原outer1为471对453错误，新判读0/负控1由Root接受，三语言原平台skip照实保留。",
    "当前S仅localGit pin008/e151/13SourceID输入；不授HTTPS、coldCargo、mainCI、native或整个Goal资格。",
    "after_revision只取同任务官方validator报告；字段绑定不等于Source通过。"
  ],
  "evidence_type": "behavior_equivalence",
  "before_revision": "0930eef9f878cf436ed88384030b6bc03d6f6c53",
  "deleted_paths": [
    "sdks/java/src/main/java/com/echoagent/sdk/A2AArtifact.java",
    "sdks/java/src/main/java/com/echoagent/sdk/A2AError.java",
    "sdks/java/src/main/java/com/echoagent/sdk/A2AMessage.java",
    "sdks/java/src/main/java/com/echoagent/sdk/A2AStreamEvent.java",
    "sdks/java/src/main/java/com/echoagent/sdk/A2AStreamResponse.java",
    "sdks/java/src/main/java/com/echoagent/sdk/A2ATask.java",
    "sdks/java/src/main/java/com/echoagent/sdk/A2ATaskParams.java",
    "sdks/java/src/main/java/com/echoagent/sdk/A2ATaskRequest.java",
    "sdks/java/src/main/java/com/echoagent/sdk/A2ATaskResponse.java",
    "sdks/java/src/main/java/com/echoagent/sdk/A2ATaskStatus.java",
    "sdks/java/src/main/java/com/echoagent/sdk/AgentAuthentication.java",
    "sdks/java/src/main/java/com/echoagent/sdk/AgentCapabilities.java",
    "sdks/java/src/main/java/com/echoagent/sdk/AgentCard.java",
    "sdks/java/src/main/java/com/echoagent/sdk/AgentCardBuilder.java",
    "sdks/java/src/main/java/com/echoagent/sdk/AgentProvider.java",
    "sdks/java/src/main/java/com/echoagent/sdk/AgentSkill.java",
    "sdks/java/src/main/java/com/echoagent/sdk/AuthenticationScheme.java",
    "sdks/java/src/main/java/com/echoagent/sdk/TaskArtifactUpdateEvent.java",
    "sdks/java/src/main/java/com/echoagent/sdk/TaskState.java",
    "sdks/java/src/main/java/com/echoagent/sdk/TaskStatusUpdateEvent.java",
    "sdks/python/src/echo_agent_sdk/a2a.py",
    "sdks/typescript/src/a2a.ts"
  ],
  "coverage": [
    "仅对0930真实Git合同与当前共同5352个source/kind的完整entry字典做直接静态相等对照，差异0。",
    "431条共同route只比较身份/操作/签名投影；value.items退役计数差异明确排除，不声明完整route对象或执行行为相等。",
    "当前原16输入复用、r13原18和Parquet/JWT正反控只作after验证事实，不补造before运行。"
  ],
  "scenario_results": {
    "retained-declared-contract-static-identity": {
      "status": "matched",
      "source_refs": [
        "echo-sdk-protocol/src/inventory.rs#apply_accepted_source_scope",
        "echo-sdk-protocol/tests/facade_inventory.rs",
        "scripts/check-a2a-lsp-retirement.py",
        "docs/adr/0042-sdk-a2a-lsp-retirement-boundary.md"
      ],
      "comparison_basis": "共同保留声明合同的直接静态数据对照；不是旧新执行轨迹等价。",
      "before_observation": {
        "revision": "0930eef9f878cf436ed88384030b6bc03d6f6c53",
        "contract_path": "contracts/sdk/accepted-external-contract.json",
        "contract_sha256": "c17df748e964453104149beb361fe897e717dff7a5c51f3d5162d9f4fd7b48d6",
        "declared_entries": 5621,
        "retained_entry_dicts_compared": 5352,
        "runtime_execution": "未在本轮观察或重跑，未知；无before运行matched声明。"
      },
      "after_observation": {
        "contract_path": "contracts/sdk/accepted-external-contract.json",
        "contract_sha256": "56179f0c504eb57bba77374a98a41b4d8f7c094d7ebc0e4a0d2cb650736d4841",
        "declared_entries": 5352,
        "retained_full_entry_dict_drift": 0,
        "route_contract_path": "contracts/sdk/accepted-facade-operation-catalog.json",
        "route_after_sha256": "c8ebea7c65e36902d336fd738d84764d2459910471623ffac8c0824e0e62cf3f",
        "retained_route_identity_operation_signature_count": 431,
        "route_full_object_equality": false,
        "excluded_items_drift": [
          {
            "route": "value",
            "changedFields": [
              "items"
            ]
          }
        ],
        "runtime_validation": "Root已接受after原16/原r13范围，不能倒推before执行。"
      },
      "scope": "matched只指共同source/kind与其完整声明entry字典、共同route身份/操作/签名投影的静态对照；不含已退出269/23/39、插件拒绝、本地值执行、Parquet往返或close运行等价。"
    }
  },
  "command_results": [
    {
      "command": "/bin/bash scripts/check-language-sdks.sh",
      "exit_code": 0,
      "scope": "当前ec29/Code888完整原18 language caller真实0，三语和三quickstart；原outer1与新判读0分列。",
      "raw_log": "/Users/ls/MyWork/code/ylp_agent_learn/lp-agent/echo-agent-sdk/.echo-semantic/local/pre-main-consumer-retirement-20261101-r08/locked-after-proposal-r04/heavy-producer-r01-prep/run-r13-language-resume/steps/18-original-complete-language.raw.log",
      "raw_sha256": "4bb38335c3c920d09683eb10859423af6d569e55764719a6b1173dab71c5761a"
    },
    {
      "command": "/Users/ls/.cargo/bin/cargo test --workspace --all-features --locked",
      "exit_code": 0,
      "scope": "原16内Rust测试实际0，Root已核不变输入闭包复用；非本轮重跑。",
      "raw_log": "/Users/ls/MyWork/code/ylp_agent_learn/lp-agent/echo-agent-sdk/.echo-semantic/local/pre-main-consumer-retirement-20261101-r08/locked-after-proposal-r04/heavy-producer-r01-prep/run-r11-retirement-tests/steps/04-workspace-all-features.raw.log",
      "raw_sha256": "ff23bd77d5d5e1d0171b849c3f4e4c69bc6d4a21efc4d1204ce758186ee00fa5"
    },
    {
      "command": "/bin/bash scripts/check-sdk-contracts.sh",
      "exit_code": 0,
      "scope": "原完整合同/生成caller实际0，Root按不变输入复用。",
      "raw_log": "/Users/ls/MyWork/code/ylp_agent_learn/lp-agent/echo-agent-sdk/.echo-semantic/local/pre-main-consumer-retirement-20261101-r08/locked-after-proposal-r04/heavy-producer-r01-prep/run-r11-retirement-tests/steps/15-original-complete-contract-caller.raw.log",
      "raw_sha256": "5e41b0812b4d8b7f85a5546d9fb0bd3fadde1c0a260739f03c5826ce64617b53"
    },
    {
      "command": "/Users/ls/MyWork/kits/apache-maven-3.9.0/bin/mvn -o -q -Dtest=ParquetRoundtripHostTest -Dsurefire.reportNameSuffix=r12-parquet-green-O2 -DargLine=-Djava.io.tmpdir=/Users/ls/MyWork/code/ylp_agent_learn/lp-agent/echo-agent-sdk/.echo-semantic/local/pre-main-consumer-retirement-20261101-r08/locked-after-proposal-r04/heavy-producer-r01-prep/sdk-r12-parquet-caller-fix-r01/tmp test",
      "exit_code": 0,
      "scope": "Parquet原selector1P/0skip，5calls/2typed errors/3行及zstd/保护输出；原RED1保留。",
      "raw_log": "/Users/ls/MyWork/code/ylp_agent_learn/lp-agent/echo-agent-sdk/.echo-semantic/local/pre-main-consumer-retirement-20261101-r08/locked-after-proposal-r04/heavy-producer-r01-prep/sdk-r12-parquet-caller-fix-r01/03-original-parquet-selector-GREEN.raw.log",
      "raw_sha256": "d3793115f72811f6f70a31b441fcdbf50817b2d8c54357b2d3b0f8651dc5e925"
    }
  ],
  "after_revision": "task:e765c4549f9b989409b94194267faed09f9103ec2eb6d03138c1a545b9746cdd"
}
---

# StageA当前退役与保留验证

## 来源与范围

只承接当前0930到StageA。observed_at由官方source_digest读取，after_revision将由同任务官方validate_task报告绑定，不猜签名；字段绑定不等于Source/Final通过。

## 支持的结论

本期Root组合接受票：/Users/ls/MyWork/code/ylp_agent_learn/lp-agent/.echo-semantic/local/root-full-framework-window-20261029/SDK-r13-composite-engineering-Root-accepted-20261101-r02.json，SHA256 5a7e4bf0d02582a10747a87d2bbe4868a513e620281187c3265b8a68f5831170。command_results取真实caller完整argv/自然0；原Rust/合同只在Root原16精输入不变闭包复用。唯一完整producer1085argv、269精集合及raw/native/postguard见Root既有SDK0930-stageA-r04-original-generator-semantic-delta-Root-accepted.json，不用缩略命令冒新运行。

Root新r03观察器实际0核471=453+18与453=452+1；原caller失败、cwd错误和未解释计数缺口负控各真实拒绝1，旧outer1保持。Parquet caller仅两行默认目录、生产API与原39不变；原RED1/GREEN0及shared oracle5calls/2typed errors/3行/zstd/protected保原。

## 已知缺口

本根.echo-semantic/local/sdk-stage-a-normal-source-20261101-r01保存Source33完整raw/caller和初次符号错误1；旧115/402与network失败不被重新判绿。原三语言Windows平台skip和历史managed未知不抹。四metadata本身仍待Root有界审查及正常Source/Final。

## 阶段边界

仅退役SDK消费表面；框架旧定义仍为库存，不扩完整Rust镜像承诺。008来源仍只已批准localGit，正式HTTPS/mainCI/native另验。eea九个后继全保原ref，最终正确main基底真实合流；旧Think/Token暂存不恢复最终旧控制工具。原件可在正文定位，但本地产物不作source_refs/code_refs或第二真理源。

## 批准行为变化的普通事实（非等价）

- surface_removed：0930的合同声明5621/454；当前经唯一原producer撤销具名269身份、23路由和39操作后为5352/431。旧入口可声明/执行到当前退出，本来是批准行为变化，不列入scenario_results，不称matched或old/new执行相同。
- legacy_rejected：当前旧feature/digest准入、旧家族方法/source invoke和三插件旧成员的拒绝由本期原Host反控验证；只陈述after拒绝，不重演旧成功scene，也不把拒绝等同旧成功行为。
- 本地JWT：四个现有值模块与0930对应Git blob同字节，正常值正控在after通过；这是一份实现来源不变及当前运行事实，不是两次运行比较。九项A2A远程映射退出另属批准变化。
- Parquet及保护输出：before0930没有本轮新增Parquet测试/夹具运行证据。after由原r13及独立selector验证5真实调用、2typed errors、3精确行/zstd/protected；before运行仍未知，不置matched等价。
- close/owner/示例：0930的实现/合同为保留义务来源；after原16和原r13当前运行通过。不以after成功倒推before轨迹相同，不承诺所有未观察的动态行为等价。

## 共同保留合同的before/after来源

本轮只读从真实0930 Git blob读取contracts/sdk/accepted-external-contract.json，与当前文件按source_paths[0]+kind取共同5352；完整entry字典漂移0。431共同route的身份/operation/signature投影相等，value.items差异单独保留。before运行没有重新观察；此静态合同保持是唯一matched场景，不能代before运行证据。具体源SHA已写入scenario的before_observation/after_observation，原始只读对照位置在本任务local的retained-before-after-static-comparison-r02.json，后者是调查原件不是source_refs或新validator。

原本把退役/拒绝及after-only运行混入behavior_equivalence/matched的r01资料与其Source0完整保留，Root独审I1已记录；不挪旧内容digest给本次内容。
