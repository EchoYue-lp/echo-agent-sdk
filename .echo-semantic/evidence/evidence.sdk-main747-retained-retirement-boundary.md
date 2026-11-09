---
schema_version: 1
id: evidence.sdk-main747-retained-retirement-boundary
kind: evidence
observed_at: source:f3bbd9c3ae4a6e0c2132f00d1f39be88a0855eb1d8b00265936ddfb0295dfde7
source_refs:
- docs/adr/0042-sdk-a2a-lsp-retirement-boundary.md
- contracts/sdk/accepted-external-contract.json
- contracts/sdk/accepted-facade-operation-catalog.json
- echo-sdk-host/src/core_profile/handler.rs
- sdks/java/src/main/java/com/echoagent/sdk/RunHandle.java
- sdks/typescript/src/index.ts
- sdks/python/src/echo_agent_sdk/__init__.py
- sdks/typescript/test/jwt-values.test.js
- sdks/python/tests/test_jwt_values.py
- sdks/java/src/test/java/com/echoagent/sdk/A2AValueTest.java
- scripts/check-language-sdks.sh
supports:
- finding.sdk-main747-retirement-remainder
- rule.framework-runtime-authority
limitations:
- 仅下列三个有限静态场景matched；四个JWT公开构造和三个fixture独立入口有意移除，行为CHANGED，不宣称全API或全运行等价。
- before747动态未运行，跨版本动态等价UNKNOWN；旧task:f5整体输入未恢复；动态或外部fixture消费者未知。
- 当前after原完整Rust/三语言/quickstart资格各按实际收据，不能充作before；当前main CI与发布另验。
evidence_type: behavior_equivalence
before_revision: 747be937226ce379d0abc04e0f3ba52029da5ed1
after_revision: source:f3bbd9c3ae4a6e0c2132f00d1f39be88a0855eb1d8b00265936ddfb0295dfde7
deleted_paths:
- sdks/typescript/src/jwt_values.ts
- sdks/python/src/echo_agent_sdk/jwt_values.py
- sdks/java/src/main/java/com/echoagent/sdk/JwtClaims.java
- sdks/java/src/main/java/com/echoagent/sdk/JwtConfig.java
- sdks/shared/testing/a2a_host_fixture.py
- sdks/shared/testing/lsp_diagnostic_server.py
- sdks/shared/testing/lsp_fault_server.py
coverage:
- 仅具名7路径批准退役中的目标准入、两条同字节owner和保留suite字面调用边界。
- 另外四个client/Session owner有变化，不标字节相等。
scenario_results:
  target-accepted-admission:
    status: matched
    source_refs:
    - contracts/sdk/accepted-external-contract.json
    - contracts/sdk/accepted-facade-operation-catalog.json
    scope: 两端A2A/LSP/JWT目标accepted身份与family均0；整体5352/431到5344/430属于其他已具名变化。
  two-original-owner-source-bodies:
    status: matched
    source_refs:
    - echo-sdk-host/src/core_profile/handler.rs
    - sdks/java/src/main/java/com/echoagent/sdk/RunHandle.java
    scope: handler.rs和RunHandle.java前后逐字相同，不推广before运行。
  retained-suite-static-caller-cut:
    status: matched
    source_refs:
    - sdks/typescript/test/jwt-values.test.js
    - sdks/python/tests/test_jwt_values.py
    - sdks/java/src/test/java/com/echoagent/sdk/A2AValueTest.java
    - scripts/check-language-sdks.sh
    scope: 限定sdks/scripts/Host/Protocol源码，三个fixture在7删除路径之外before和after字面引用都为0；不能否认before standalone入口。
command_results:
- command: python3 -B .echo-semantic/local/integrated-acceptance-20261108-r01/sdk-semantic-maintenance-r01/check_retained_deletion_boundary.py
  exit_code: 0
  scope: 真实git747与当前源的静态窄对照：7旧blobSHA/18当前r04SHA/目标准入0对0/两owner同字节/保留suite外引用0对0。不是动态重放。
  raw_log: retained-deletion-boundary.stdout.json
  raw_sha256: 81714ea99dad3d93a90ca9c0bf5ab49fc5183f9d87981fb5aa1e2326e80aa6e6
---

# main747七路径退役的有限保留边界对照

## 支持的结论

复用原ADR0042具名批准的A2A/LSP consumer退役。上列matched只表示三个准确静态场景相同，不表示整个SDK或被移除公开API运行等价。

四个JWT文件在main747仍经TS/Python根导出或Java public类被真实构造，当前移除这些公开名字；三个fixture在747均有独立__main__启动能力，当前移除。这些行为明确CHANGED，不是任何入口都不可达的死代码。accepted身份为0不能用来否认公开构造变化。

## 来源与范围

原脚本、命令输出和逐路径blob/SHA在主根 `.echo-semantic/local/integrated-acceptance-20261108-r01/sdk-semantic-maintenance-r01/`。脚本实际自然0，没有编译执行before。`current-seven-deletions-proof.json` SHA256 `9b20f7a89bae76f06f44fec61da1c2ae4b91711c59fbf6c029b9f47495dbd19e` 原样保留全部9个场景的changed、静态matched与unknown，本件不提升报告中的unknown。

当前after完整三语言原入口自然0：TS527P/1原skip、Python601P/18subtests/1原skip、Java400项0F0E/1原skip，三个quickstart通过。原raw SHA256 `18790f51062fa83108eac28f49ba9258396e27e68b2133f56627000af0520cd5`。这只是after资格。

## 已知缺口

before747动态未运行，外部/动态消费者未知，旧task:f5整体输入未恢复；两端整体合同数和另外四个owner代码不同。原Finding/Map/closure不提升。独立审查须裁定有限静态对照是否足够支持这次具名退役；审查前新Finding保持open。
