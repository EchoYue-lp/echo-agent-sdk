---
{
  "schema_version": 1,
  "id": "audit.sdk-a2a-lsp-consumer-retirement",
  "kind": "audit",
  "lens": "contract_evidence",
  "freshness": "examined",
  "revision": "source:dd04568ae1f1700a8164b4a114e267b9681927fa63f45513063afe92e94a2e92",
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
      "revision": "source:dd04568ae1f1700a8164b4a114e267b9681927fa63f45513063afe92e94a2e92",
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
      "revision": "source:dd04568ae1f1700a8164b4a114e267b9681927fa63f45513063afe92e94a2e92",
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
      "revision": "source:dd04568ae1f1700a8164b4a114e267b9681927fa63f45513063afe92e94a2e92",
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
      "revision": "source:dd04568ae1f1700a8164b4a114e267b9681927fa63f45513063afe92e94a2e92",
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
