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
    "sdks/typescript/test/parquet-roundtrip-host.test.js",
    "contracts/sdk/facade-operation-catalog.json",
    "contracts/sdk/inventory-telemetry.json",
    "contracts/sdk/parity-manifest.json",
    "contracts/sdk/public-api.txt",
    "contracts/sdk/schema/echo-agent-extension-v1.schema.json",
    "contracts/sdk/source-contract.json",
    "echo-sdk-protocol/src/catalog.rs",
    "echo-sdk-protocol/tests/core_rpc_contract.rs",
    "sdks/python/tests/test_a2a_host.py",
    "sdks/python/tests/test_lsp_diagnostics_host.py",
    "sdks/python/tests/test_lsp_host.py"
  ],
  "supports": [
    "finding.sdk-a2a-lsp-consumer-retirement"
  ],
  "limitations": [
    "behavior_equivalence仅绑定scenario_results中共同保留的声明合同静态对照。批准退出/拒绝和after-only运行单列普通事实，不计matched等价场景；before运行未观察，不声明两次执行相等。",
    "当前Source33、预检参数错误及原r11/r12/ruff/聚合/network失败全部保原；本四metadata独审与Source/Final另验。",
    "原r13 language caller0；原outer1为471对453错误，新判读0/负控1由Root接受，三语言原平台skip照实保留。",
    "当前S仅localGit pin008/e151/13SourceID输入；不授HTTPS、coldCargo、mainCI、native或整个Goal资格。",
    "after_revision现以固定canonical原source_digest实际得到的source:e1864cc87ff524845908f5de781baa189dfed513a9b78551b23d5c59a0e2726b定位同一StageA业务源码；原task:e765与c152、Source18/Final19任务回执保留为历史，不因此授仓库级strict或exact-main CI通过。"
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
  "after_revision": "source:e1864cc87ff524845908f5de781baa189dfed513a9b78551b23d5c59a0e2726b"
}
---

# StageA当前退役与保留验证

## 来源与范围

只承接当前0930到StageA。observed_at保持原source:e186不变；便携候选的after_revision使用固定canonical原source_digest已实际算出的同一source:e186。执行期原after_revision曾由同任务validate_task报告绑定为task:e765，历史任务证明原样保留。源码身份字段不授本次Source/Final或仓库级CI通过。

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


## StageA 自有资料便携校准与原任务历史

本候选仅修正本 Evidence 的当前业务源码身份与同次路径依据，以及 baseline 的摘要；不改变 before_revision、已有 matched 场景、其静态比较范围、command_results、Finding 状态或任何业务实现。observed_at仍为原source:e186；after_revision从任务身份改为同一真实业务源码的source:e186。固定原函数的已完成Phase3最小检查为1/1/12，不是一次完整仓库Source gate。

原Source18和Final19两份原caller自然0、scope=task、globalClosure=not_checked，sourceSnapshot为task:e765c4549f9b989409b94194267faed09f9103ec2eb6d03138c1a545b9746cdd，contentDigest为c152f4e81e8729b0389035abd6ad4c4c81ba1799fcaaf973debdf9af1198e809。原任务回执及其判断不改写、不升格。PR14原semantic job的14项失败也不因候选文稿变成已修复；同一canonical action仍有删除预检运行态接口缺口。

下列各项补充同次变更的真实事实和验收边界，不新增等价matched场景。代码与生成物身份取原0930 Git blob和已冻结StageA文件，运行事实复用原具名caller；不将源审查改称新执行，不用只有文件名的一行充当验证。

### contracts/sdk/facade-operation-catalog.json：完整目录与 accepted 目录分开

该文件是原export_schema生成的完整facade目录，StageA直接文件读取为39个family、772条route，0930为41个family、798条route；完整目录的total_items为26719。它与accepted-facade-operation-catalog的431条已接受route不是同一个集合，不可混写计数。A2A/LSP消费退出及具名source disposition来自ADR0042和当前inventory/facade实现；原完整check-sdk-contracts.sh（r11第15项）自然0提供阻断生成物的当前一致性事实，不证明全部framework公共库存退役，也不证明每条route的动态调用。

### contracts/sdk/inventory-telemetry.json：原遥测漂移保留

该文件自标rust_public_inventory_telemetry、blocking_scope=external_contract，明确把完整Rust库存及framework revision排除于runtime negotiation。当前提交内容仍记录f7bac62e来源和5352个external_contract项；这只是该遥测文件保存的身份，不能冒当前StageA localGit008精来源证明。原第15项完整日志第77行明确TELEMETRY DRIFT，第187行报告116 blocking artifacts current、1 telemetry artifact drifted；这份非阻断漂移继续保留，不宣称全Rust inventory漂移0，不新增ignore或更改classification。telemetry内容如何另行刷新仍pending，本候选不修改它。

### contracts/sdk/parity-manifest.json：完整Rust inventory不是外部承诺

当前原文件entries为26719条，0930为26716条；这些是完整inventory entries，不等于5352条accepted external contract。该文件由原唯一export_schema生成链维护，approved retirement只对已具名消费/accepted范围退出，不以SDK不消费为理由删除合理框架API。原第15项完整producer/check自然0是其生成与阻断边界的证据；本候选没有重跑28 profiles或对全部inventory项给新的动态等价结论，原1项telemetry漂移不在这里被抹除。

### contracts/sdk/public-api.txt：完整公开库存仍是独立记录

当前文件SHA256为ae4be5e65c129c0391c0633fe29072a35ebd7ec1f50f45683cede6efc8f1266a，来自已冻结StageA，保留原全量公开Rust库存记录。它随原producer输入变化，不能当作每条类型/函数都由三语言执行的证明，也不扩accepted外部合同。具名保留静态合同相等仍只限前述共同5352 entry字典及431条route身份/操作/签名投影；原第15项check日志的阻断通过和telemetry漂移分列，不新增所有历史profile可恢复或全库存0漂移声明。

### contracts/sdk/schema/echo-agent-extension-v1.schema.json：仅退出两个旧wire方法

0930到当前的直接JSON对照显示method_catalog从58到56，删除_echo_agent/a2a/op和_echo_agent/lsp/op；259个definitions对象直接比较相同。该变更与catalog.rs退役的两个family入口相互对应，不把它描述为全部ACP或所有DTO退出。原第15项完整check中的extension_contract、acp_baseline及a2a_lsp_retirement等原测试提供当时验收事实；这不是本次重新生成或外部provider回放。

### contracts/sdk/source-contract.json：协商摘要仍只绑定 accepted 两输入

当前文件使用sha256-length-prefixed-v1，精输入为accepted-external-contract.json（10141243B，SHA56179f0c504eb57bba77374a98a41b4d8f7c094d7ebc0e4a0d2cb650736d4841）和accepted-facade-operation-catalog.json（612298B，SHAc8ebea7c65e36902d336fd738d84764d2459910471623ffac8c0824e0e62cf3f），aggregate_digest为sha256:6c7bdc12662a5f0941a3e57cf0139f079ae2a96c72bff498af3f4ee386a0e44d。原第15项check核该生成合同；它不以完整inventory/telemetry或Cargo.lock充当协商摘要，更不证明本地Git取得等于HTTPS发布或mainCI通过。

### echo-sdk-protocol/src/catalog.rs：消费wire入口实际退出

与0930的精diff显示MethodDescriptor的参数/结果匹配及METHOD_CATALOG移除_echo_agent/a2a/op、_echo_agent/lsp/op，其余family入口继续保留。这个变更执行ADR0042的SDK消费退役，不是删除框架所有合理公共API。原第06项protocol-default与第15项完整合同测试提供原运行事实，具体拒绝还由现有a2a_lsp_retirement及语言catalog断言覆盖；本候选不新增或放松wire validator。

### echo-sdk-protocol/tests/core_rpc_contract.rs：保留family的绑定断言

原family_methods_are_bound_to_the_facade_route_table只从预期family集合移除FacadeFamily::A2a、FacadeFamily::Lsp，其余绑定断言沿原实现。原第06项protocol-default日志第98行实际记录该测试ok；这证明当时保留family映射断言执行，不证明被批准退出的旧family还可运行，也不自动扩成所有operation动态等价。

### sdks/python/tests/test_a2a_host.py：从旧Host场景转为目录拒绝

原HTTP/SSE peer、owner和错误场景随已批准A2A消费退役退出；当前test_retired_a2a_operations_are_not_catalog_entries对a2a.client.open、a2a.discover、a2a.task.stream.open调用FacadeCatalog.resolve并要求KeyError且消息包含absent。它是当前客户端catalog拒绝断言，不是旧真实Host场景仍运行、更不是before/after行为等价。该文件属于原r13完整Python suite的当时来源，原语言caller0与完整Python452P/1skip+18subtests按已接受范围保留；没有该单测试独立行级运行票，不据聚合结果补造独立caller或逐项新运行。

### sdks/python/tests/test_lsp_diagnostics_host.py：diagnostics不再解析为operation

当前test_lsp_diagnostics_is_not_a_catalog_operation断言FacadeCatalog.resolve(lsp.client.diagnostics)抛KeyError(absent)，替代退役的诊断通知/替换/故障/runtime settlement旧Host场景。它只证明所写的目录拒绝目标，不能继续宣称本轮观察了旧LSP服务进程清退。原r13完整suite及修正observer的资格如上保留；单项独立运行/旧before运行没有新证据，保持未知，不置matched。

### sdks/python/tests/test_lsp_host.py：剩余旧LSP family调用也被拒绝

当前test_lsp_family_is_retired_from_catalog枚举lsp.manager.open、lsp.client.hover、lsp.client.goto_definition并逐个要求FacadeCatalog.resolve抛KeyError(absent)。原通用值、错误、manager关闭与不可复活Host场景已随SDK消费表面退出；这里没有把拒绝当作旧成功轨迹相等。原r13聚合运行范围及缺独立单项票的边界保持，不因文件名仍带host就声称真实LSP Host已被本轮重放。

## 固定canonical删除预检接口仍未闭合

当前resolved Finding/retire决策、22项delete_paths、canonical/replacement资产和原Evidence链继续保留。固定c0bc action仍读取Final已消耗且禁止Git跟踪的本地preflight；它没有在本次允许范围内消费这些正式删除授权的便携入口。本两文件候选不生成preflight，不改schema/root，不改workflow、工具、exclusion、risk或失败原票，不问用户扩大外部工具范围。13项自有资料即使后续正式修正，仍须保留该接口缺口和其后尚未穷尽检查的限制。
