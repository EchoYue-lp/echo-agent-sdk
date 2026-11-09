---
{
  "schema_version": 1,
  "id": "audit.sdk-a2a-lsp-consumer-retirement",
  "kind": "audit",
  "lens": "contract_evidence",
  "freshness": "examined",
  "revision": "1eac40fbf224e510ab29aea128ce59c20f8b4a52",
  "source_refs": [
    "docs/adr/0042-sdk-a2a-lsp-retirement-boundary.md",
    "echo-sdk-protocol/src/facade.rs#governance_source_disposition",
    "echo-sdk-protocol/src/scalar.rs",
    "echo-sdk-protocol/tests/a2a_lsp_retirement.rs",
    "echo-sdk-host/src/features.rs",
    "echo-sdk-host/src/core_profile/facade/source_operations.rs#resolved_plugin_components_value",
    "echo-sdk-host/tests/core_profile_e2e.rs#retired_a2a_lsp_addresses_fail_on_the_real_host",
    "scripts/check-a2a-lsp-retirement.py",
    "scripts/check-language-sdks.sh",
    "echo-sdk-host/src/core_profile/handler.rs#close_session_owned",
    "sdks/typescript/src/client.ts",
    "sdks/python/src/echo_agent_sdk/client.py",
    "sdks/java/src/main/java/com/echoagent/sdk/SessionHandle.java",
    "sdks/java/src/main/java/com/echoagent/sdk/RunHandle.java",
    "sdks/java/src/main/java/com/echoagent/sdk/EchoAgentClient.java",
    "sdks/typescript/test/client-stdio-close.test.js",
    "sdks/typescript/test/session-parent-close-host.test.js",
    "sdks/typescript/test/session-parent-close.test.js",
    "sdks/python/tests/test_session_parent_close_host.py",
    "sdks/python/tests/test_session_parent_close.py",
    "sdks/java/src/test/java/com/echoagent/sdk/SessionParentCloseHostTest.java",
    "sdks/java/src/test/java/com/echoagent/sdk/SessionParentCloseTest.java",
    "sdks/typescript/test/managed-retirement-host.test.js",
    "sdks/python/tests/test_managed_retirement_host.py",
    "sdks/java/src/test/java/com/echoagent/sdk/ManagedRetirementHostTest.java",
    "sdks/typescript/examples/quickstart.ts",
    "sdks/python/examples/quickstart.py",
    "sdks/java/src/main/java/com/echoagent/sdk/Example.java"
  ],
  "finding_refs": [
    "finding.sdk-a2a-lsp-consumer-retirement"
  ],
  "challenges": {
    "source-and-runtime-drift": {
      "revision": "1eac40fbf224e510ab29aea128ce59c20f8b4a52",
      "source_refs": [
        "docs/adr/0042-sdk-a2a-lsp-retirement-boundary.md",
        "echo-sdk-protocol/src/facade.rs#governance_source_disposition",
        "echo-sdk-protocol/src/scalar.rs",
        "echo-sdk-protocol/tests/a2a_lsp_retirement.rs",
        "echo-sdk-host/src/features.rs",
        "echo-sdk-host/src/core_profile/facade/source_operations.rs#resolved_plugin_components_value",
        "echo-sdk-host/tests/core_profile_e2e.rs#retired_a2a_lsp_addresses_fail_on_the_real_host",
        "scripts/check-a2a-lsp-retirement.py",
        "scripts/check-language-sdks.sh"
      ],
      "evidence_refs": [
        "evidence.sdk-a2a-lsp-consumer-retirement"
      ],
      "result": "Independent source/runtime review pass0 at dd04568a;61 Rust input hashes/full Host/collector/full Rust and117 artifact results match."
    },
    "support-file-reuse": {
      "revision": "1eac40fbf224e510ab29aea128ce59c20f8b4a52",
      "source_refs": [
        "sdks/typescript/src/index.ts",
        "sdks/python/src/echo_agent_sdk/__init__.py",
        "sdks/python/tests/test_a2a_task_state.py",
        "sdks/python/tests/test_jwt_values.py",
        "sdks/typescript/test/a2a-host.test.js",
        "sdks/python/tests/test_a2a_host.py",
        "sdks/java/src/test/java/com/echoagent/sdk/A2AHostTest.java"
      ],
      "evidence_refs": [
        "evidence.sdk-a2a-lsp-consumer-retirement"
      ],
      "result": "Exact29 removed supporting paths had no outside native imports/types; retained6 lifecycle sources identical toS961."
    },
    "retirement-hidden-reachability": {
      "revision": "1eac40fbf224e510ab29aea128ce59c20f8b4a52",
      "source_refs": [
        "sdks/typescript/test/a2a-host.test.js",
        "sdks/python/tests/test_a2a_host.py",
        "sdks/java/src/test/java/com/echoagent/sdk/A2AHostTest.java"
      ],
      "evidence_refs": [
        "evidence.sdk-a2a-lsp-consumer-retirement"
      ],
      "result": "Real Host old-admission/39-method-operation/19-source and3 typed-plugin probes pass with exact refusal reasons; no successful compatibility stub."
    },
    "retained-lifecycle-and-examples": {
      "revision": "1eac40fbf224e510ab29aea128ce59c20f8b4a52",
      "source_refs": [
        "echo-sdk-host/src/core_profile/handler.rs#close_session_owned",
        "sdks/typescript/src/client.ts",
        "sdks/python/src/echo_agent_sdk/client.py",
        "sdks/java/src/main/java/com/echoagent/sdk/SessionHandle.java",
        "sdks/java/src/main/java/com/echoagent/sdk/RunHandle.java",
        "sdks/java/src/main/java/com/echoagent/sdk/EchoAgentClient.java",
        "sdks/typescript/test/client-stdio-close.test.js",
        "sdks/typescript/test/session-parent-close-host.test.js",
        "sdks/typescript/test/session-parent-close.test.js",
        "sdks/python/tests/test_session_parent_close_host.py",
        "sdks/python/tests/test_session_parent_close.py",
        "sdks/java/src/test/java/com/echoagent/sdk/SessionParentCloseHostTest.java",
        "sdks/java/src/test/java/com/echoagent/sdk/SessionParentCloseTest.java",
        "sdks/typescript/test/managed-retirement-host.test.js",
        "sdks/python/tests/test_managed_retirement_host.py",
        "sdks/java/src/test/java/com/echoagent/sdk/ManagedRetirementHostTest.java",
        "sdks/typescript/examples/quickstart.ts",
        "sdks/python/examples/quickstart.py",
        "sdks/java/src/main/java/com/echoagent/sdk/Example.java"
      ],
      "evidence_refs": [
        "evidence.sdk-a2a-lsp-consumer-retirement"
      ],
      "result": "Original complete language entry natural0: TS369/PY450+18subtests/Java316, each1 Windows-only skip;3 quickstarts and all named R2/managed/stdio entries passed."
    },
    "formal83-current-retirement-regressions-20261104": {
      "revision": "0025d84e1a5a08634c9596092c46ca37eed0304c",
      "source_refs": [
        "echo-sdk-protocol/src/scalar.rs",
        "echo-sdk-host/tests/core_profile_e2e.rs",
        "sdks/python/tests/test_jwt_values.py",
        "sdks/typescript/test/jwt-values.test.js",
        "sdks/java/src/test/java/com/echoagent/sdk/A2AValueTest.java"
      ],
      "evidence_refs": [
        "evidence.sdk-sandbox-and-canonical-input-consumption"
      ],
      "result": "作者对账：四测试154行已接收，HEAD沙箱夹具修复保留；五工程自然0、Rust589P/0F/0I、三语言定向各2P。before运行UNKNOWN，不声明跨版本运行等价、完整SDK或独立审查完成。"
    }
  },
  "boundary_ref": "boundary.sdk-integrations"
}
---

# SDK retirement source/runtime rereview

## 审查范围

Independent sdk_docs_review reviewed current source dd04568a, exact269/23/39 subtraction, compiled Host/collector provenance, completed Rust/profile/contract/dependency and full3-language runtime evidence. Source/runtime pass; Critical0,Important0,Minor0. Reviewer ran no Cargo.

## 已检查故障假设

Shared source classes or R2 owners could be removed; raw old addresses might remain reachable; malformed arguments might mask refusal; stale binaries or omitted language/quickstart tests might create false success. Exact source/fixture checks, concrete refusal assertions, current binary SHA and whole-entry results support the bounded conclusions above.

## 实际实现路径与证据

ADR0042 and the existing generator remain the accepted retirement authority. SDK feature graph excludes a2a/lsp, shared adapters keep their retained owners, mandatory-SHA prebuilt validation runs the complete original entry after own target clean. The linked evidence records actual exits and raw hashes.

## 问题记录

Original Important shallow-CI baseline and Minor current A2A stream documentation findings were fixed and independently rereviewed. The Python import-group failure was fixed in the formal Python working directory and the complete original entry reran0. Historical raw failures remain.

## 残余风险

Normal wrapper/staged receipt checks and final overall local rereview are separately required; source/runtime pass does not substitute. Original runtime-capacity failures and controlled tool repair observations remain preserved. Historical unknowns/global closure/Windows native/mainCI and the cross-repository Outcome remain unchanged.

## 未检查项

No remote push/merge/release, installed EKO GUI/native acceptance, external A2A/LSP interoperability or framework physical deletion was performed.


## 2026-10-27 可恢复来源重定位的审查界限

旧审查metadata与正文由 Git `1eac40fbf224e510ab29aea128ce59c20f8b4a52:.echo-semantic/audits/audit.sdk-a2a-lsp-consumer-retirement.md` 和本次写入前完整副本恢复（SHA256 `9e3682897abc3b5f03cf29434f7dcc1072dff3739b9ec65ff1430f399173159a`）。revision `source:dd04568ae1f1700a8164b4a114e267b9681927fa63f45513063afe92e94a2e92` 及全部challenge/result/旧资格文字仍属原历史审查；只把其同字节source_refs定位到1eac，不把它续为本根新运行独审。

Root对本根94路径/1216输入的来源合流只读复审C0/I0/M0，限精确候选/AGENTS前缀/269退出及保留集合，不代替全部动态验收。当前实际109项Protocol定向测试、Host构建及小检查自然0单列于共用Evidence本日补充；fresh官方audit自然1出现四漏洞/十unsound是真实新阻断。完整Rust/三语言/117生成资格和最终TaskSemantic/暂存门禁尚需各自原件，不能由本审查文字或来源重绑推断。


## 2026-11-04：正式83工作树的作者对账

新增具名当前source挑战，旧revision、freshness及四个历史挑战的字段和结论不变。
本段是作者对账；新增资料的独立审查另取实际结果，不以旧pass续签。


## 2026-11-09：原 source0e84 的可恢复源码定位

本次仅把上述源码观察字段定位到 Git `0025d84e1a5a08634c9596092c46ca37eed0304c`。原 Nov4 冻结清单 SHA256
`4e2c5b6e06418b1e3a6f2609cc003f4e1bae44ef9638b143f939cc46d89e9f23` 的1303路径及逐文件SHA，
与该 Git tree 完全一致（业务933、语义370；无缺失、额外或内容差异）。原 `source:0e84c61ef79502743c58bf928b9a9ff208621580323d7ca6efb01a21f242731b`
仍是原纪元记录，不用当前摘要重算替换。原命令结果、before UNKNOWN、独立审查缺口及正文均保留，
本次重定位不把历史运行续签为当前 Framework3413、当前 SDK、main CI、GUI 或发布资格。
写入前原件保存在主根 `sdk-semantic-maintenance-r01/historical-before/`，内容SHA如下：
`3a4e8790d5927f8d0593296f0187abb8fff9e2963067d0b08c585dc34d85858e`。
