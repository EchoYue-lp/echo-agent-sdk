---
{
  "schema_version": 1,
  "id": "evidence.sdk-tool-behavior-candidate",
  "kind": "evidence",
  "observed_at": "source:3ea544c47e331b5d29a6624f978f3595619bef39fa5985230351098d38930404",
  "source_refs": [
    "echo-sdk-host/tests/tool_behavior_governance_e2e.rs",
    "sdks/shared/testing/tool_behavior_fixture.py",
    "sdks/typescript/test/tool-behaviors.test.js",
    "sdks/python/tests/test_tool_behaviors.py",
    "sdks/java/src/test/java/com/echoagent/sdk/ToolBehaviorsHostTest.java"
  ],
  "supports": [
    "behavior.sdk-family-data",
    "behavior.sdk-family-chart",
    "behavior.sdk-family-database",
    "behavior.sdk-family-git",
    "behavior.sdk-family-media",
    "behavior.sdk-family-research"
  ],
  "limitations": [
    "28 tool calls plus 1 Session close; 10 typed RPC errors plus separate database ToolResult failure.",
    "Local candidate Host4d436 and local framework override, not precise published pin/main.",
    "GET-only fixture does not dynamically prove absence of POST; no complete repo A ref comparison.",
    "Provider paging/cancel, other accepted operations and runtime effect-ledger interpretation unproved."
  ]
}
---

# SDK工具簇精确候选行为证据

## 支持的结论

共享场景共 **29 条记录：28 次工具调用 + 1 次 Session close**，在 Rust official ACP 和三个原生语言 SDK 上执行。包含 **10 个 typed RPC error**，另有一个 `database.sql_query` 返回 `ToolResult.success=false` 的拒写结果；两种失败形式分别核对。

| family | 实际支持的局部断言 |
|---|---|
| data | CSV sort/select 的列、行序、schema/value；JSON export 文件内容；缺列/无效格式明确错误，输入与 sentinel 文件不变 |
| chart | bar 的完整 normalized Vega-Lite schema/mark/title/data/encoding/width/height，Unicode、color null omission；缺 data 的 typed error |
| database | SQLite 精确 columns/rows/null；DELETE structured_error、permanent/stop/side_effect=none；后续读取结果和数据库文件 SHA 不变 |
| git | 两 Session cwd 隔离、exact status/diff；创建 sdk-proof 指向原 HEAD；非法分支错误后 repo A HEAD/sdk-proof 与 repo B refs 保持指定值；关闭 A 后 B 仍可读 |
| media | UTF-8 text_stats 精确数值、unique preview/export bytes；输入不变，missing input 错误时 sentinel 不变 |
| research | 真实 loopback HTTP PDF page/text/metadata；503、非 PDF、坏 PDF 错误；截断 preview 和完整 artifact path/SHA/bytes/JSON；BibTeX 字段、逃逸、文件字节及空输入不覆盖 |

ToolResult 的 json/text/structured_error kind、typed data 与 output JSON、框架错误 operation/retryability/canonical details 均实际核对。工具报告的 effects/model_content 为 empty 时，只证明该报告值；实际文件/ref 副作用另行对账，不能从 empty 推断无副作用或补造 runtime EffectLedger 事实。

## 来源与范围

源码观察身份仅说明归档时读到的当前工作树；运行证据仍绑定下面的实测 epoch、逐文件 SHA 和冻结 Host。归档未重跑业务验证，也没有把旧日志绑定到后来的整仓源码。所有关联 Behavior 保持 `needs_review`，published framework pin、完整门禁、独立复审和精确 main 验收仍待。

三语言使用不可变 Host SHA256 `4d436b60f1fe8d45c1472c3d4f7587ca0fb368bb50ab161f5d1a54f861d66b22`，schema `sha256:df48c9a261cdfd7e48ba9379ffbc5d4d7e5327dcc49a766ab67d854592788e61`，accepted source contract `sha256:ce80aa3a7793d4460e082772eb24fa228734d65f98ec997af2087be27b8e015e`。Rust 实测当时 warm target Host 同一 SHA；它使用临时 local framework path override，manifest pin不代表该组合源码。原始构建输入摘要保存在 receipt 的 `rust_provenance`。

归档时工具 fixture、Rust 测试和三语言测试共五个文件 SHA 全部与运行 receipt 相同。这个逐文件一致性不证明整仓未变：Java构造/清理、framework Task 后继和文档已分别演进。current observation 与 tested SHA逐项对账保存在 [archive manifest](../../docs/sdk/governance/evidence/sdk-tool-testing-20261013/archive-manifest.json)。

## 实际执行与可复核材料

- TypeScript native：1 test，0 fail/skip，`sdk-tool-behavior-typescript-final.log` SHA `643c3506664b120c6977a66a24a36a4c6ecd425a79e0c0f387b56c90efa11e35`。
- Python native：1 test，0 fail/skip，`sdk-tool-behavior-python-exact-final.log` SHA `67dfec2af83953e839309a9d8c742a68cf5f7e359befac43cf619c075686bbf2`。
- Java final full Maven：session 45147 exit 0，264 tests/51 suites，ToolBehaviorsHostTest **1 test，0 failure/error/skip**；此 Evidence仅归属该类与共享断言，不把264 tests全部算工具行为。XML SHA `cfa8876cc78b8d32603e85fc5f302e70883a47ffe9061e9463edaec1342f2756`。
- Rust single target：1 passed，0 failed/ignored，`sdk-tool-behavior-rust-valid-hello.log` SHA `a30e539730934f574103a418375ed2a3a91874fc6bd80fd322832b01d4929557`；对应 target Clippy `-D warnings` exit 0，无 compiler warning。

原始日志、XML与历史失败/成功 epoch 描述保留在 [tool receipt](../../docs/sdk/governance/evidence/sdk-tool-testing-20261013/sdk-tool-behavior-evidence.json) 和归档目录。receipt旧字段 `native_calls=29` 是场景记录计数，以上按独立复审收窄为28 tool calls+1close；原收据不改写。

## 已知缺口

没有完整 before/after repo A 全 refs 断言；不能声称非法分支后所有 refs 都已被动态核对。HTTP fixture只记录GET，不能据此证明无POST；直接Tool调用及无新Run journal只是列出的源码/文件断言。四个文献搜索工具固定官方 endpoint，本组未证明其 provider parser、分页或取消。其余 accepted data transformations、media parsers、chart类型、数据库 list/describe/timeout、git commit/worktree 和 platform variants均保留最低充分要求。没有 raster rendering 或 runtime effects完整性声明。
