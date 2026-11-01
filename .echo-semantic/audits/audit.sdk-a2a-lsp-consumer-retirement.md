---
{
  "schema_version": 1,
  "id": "audit.sdk-a2a-lsp-consumer-retirement",
  "kind": "audit",
  "lens": "contract_evidence",
  "freshness": "examined",
  "revision": "source:e1864cc87ff524845908f5de781baa189dfed513a9b78551b23d5c59a0e2726b",
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
  "finding_refs": [
    "finding.sdk-a2a-lsp-consumer-retirement"
  ],
  "challenges": {
    "exact-retirement-and-typed-admission": {
      "revision": "source:e1864cc87ff524845908f5de781baa189dfed513a9b78551b23d5c59a0e2726b",
      "source_refs": [
        "echo-sdk-protocol/tests/a2a_lsp_retirement.rs",
        "scripts/check-a2a-lsp-retirement.py",
        "docs/adr/0042-sdk-a2a-lsp-retirement-boundary.md",
        "echo-sdk-protocol/tests/a2a_lsp_retirement.rs",
        "scripts/check-a2a-lsp-retirement.py",
        "sdks/shared/testing/retirement_plugin_fixture.py"
      ],
      "evidence_refs": [
        "evidence.sdk-a2a-lsp-consumer-retirement"
      ],
      "result": "批准行为变化的after事实：surface_removed/legacy_rejected，当前269/23/39退出与拒绝经过验证；不是before/after等价，不参与Evidence matched集合。"
    },
    "local-jwt-and-language-cutover": {
      "revision": "source:e1864cc87ff524845908f5de781baa189dfed513a9b78551b23d5c59a0e2726b",
      "source_refs": [
        "sdks/python/tests/test_a2a_task_state.py",
        "sdks/python/tests/test_jwt_values.py",
        "sdks/typescript/test/jwt-values.test.js",
        "sdks/java/src/test/java/com/echoagent/sdk/A2AValueTest.java"
      ],
      "evidence_refs": [
        "evidence.sdk-a2a-lsp-consumer-retirement"
      ],
      "result": "before真实Git源码是义务来源，四localJWT实现同字节；after本地值正控通过。九远程映射退出为另列普通事实，无before运行相等声明。"
    },
    "retained-parquet-and-protected-output": {
      "revision": "source:e1864cc87ff524845908f5de781baa189dfed513a9b78551b23d5c59a0e2726b",
      "source_refs": [
        "sdks/python/tests/test_parquet_roundtrip_host.py",
        "sdks/shared/testing/parquet_roundtrip_fixture.py",
        "sdks/typescript/test/parquet-roundtrip-host.test.js",
        "sdks/java/src/test/java/com/echoagent/sdk/ParquetRoundtripHostTest.java"
      ],
      "evidence_refs": [
        "evidence.sdk-a2a-lsp-consumer-retirement"
      ],
      "result": "after原r13/selector实际通过；before0930无本轮Parquet运行证据，before/after运行比较未知，不列matched等价。"
    },
    "retained-owners-and-examples": {
      "revision": "source:e1864cc87ff524845908f5de781baa189dfed513a9b78551b23d5c59a0e2726b",
      "source_refs": [
        "echo-sdk-host/src/core_profile/handler.rs#close_session_owned",
        "sdks/typescript/src/client.ts",
        "sdks/python/src/echo_agent_sdk/client.py",
        "sdks/java/src/main/java/com/echoagent/sdk/SessionHandle.java",
        "sdks/java/src/main/java/com/echoagent/sdk/RunHandle.java",
        "sdks/java/src/main/java/com/echoagent/sdk/EchoAgentClient.java",
        "scripts/check-language-sdks.sh"
      ],
      "evidence_refs": [
        "evidence.sdk-a2a-lsp-consumer-retirement"
      ],
      "result": "before真实源码/合同为义务来源，after原16/r13通过；无两次运行轨迹相等声明。"
    },
    "retained-declared-contract-static-identity": {
      "revision": "source:e1864cc87ff524845908f5de781baa189dfed513a9b78551b23d5c59a0e2726b",
      "source_refs": [
        "echo-sdk-protocol/src/inventory.rs#apply_accepted_source_scope",
        "scripts/check-a2a-lsp-retirement.py"
      ],
      "evidence_refs": [
        "evidence.sdk-a2a-lsp-consumer-retirement"
      ],
      "result": "真实0930/current共同5352完整entry字典差0、431路由身份/操作/签名投影相等；仅静态合同比较，value.items和before运行明确不在matched范围。"
    }
  },
  "boundary_ref": "boundary.sdk-integrations"
}
---

# StageA有界业务复核

## 审查范围

只核当前22删除、七个验证源码/fixture、269/23/39与5352/431及保留owner、本地JWT、Parquet、示例。依据为ADR0042和本期真实代码/Root原接受事实，不按同名、裸数量或旧eea29删除扩张。

## 实际实现路径与证据

Root已接受40测试源码质量C0/I0/M0与Parquet两行实际RED→GREEN；原16按精输入闭包复用，当前原18及修正观察器由Root5a7e接受。本对象记录业务复核引用，freshness examined不表示无缺陷/全库闭合；四metadata自身独立有界审查由Root后置安排，不在此伪造。

## 未检查项

旧Evidence epoch、managed未知和全部原失败保持。Source/Final、HTTPS/coldCargo/mainCI/native分别验，不改外工具/schema/ignore，不新建validator/executor。

## 已检查故障假设

挑战一：旧家族/方法/源invoke重新聚合；挑战二：JWT本地正控或保留close owner被误删；挑战三：Parquet保护输出或typed error被目录适配绕过。frontmatter逐挑战绑定当前源码与Evidence，具体正反控来自原producer/原16和当前原18，不是新造运行。

## 残余风险

中间StageA不代表eea原生后继合流；HTTPS/cold/mainCI/native和全局行为闭合未由本对象证明。本四metadata本身待Root有界独审，Source/Final只取后继真实结果。

## 问题记录

初次正常Source1/33及当前只读官方诊断16项均保完整raw：前者指出本任务资料/预检绑定不足，后者指出本文版式标题和after尚未绑定、repair/delete元数据未登记。只修当前四资料及正常record，不改外工具/schema/ignore或业务源码，不关闭历史UNKNOWN。

## Root I1与本轮修正

r01把批准退出/拒绝写作matched等价，独审block C0/I1/M0正确；原Source0只结构通过，不补作行为相同。当前退出与after-only运行均移入Evidence普通事实，唯一matched缩为实际可直接对照的共同静态声明合同，before运行未知。Finding验证链同步限于此范围。本次修改仍待Root增量独审，本文不伪称已审自己。
