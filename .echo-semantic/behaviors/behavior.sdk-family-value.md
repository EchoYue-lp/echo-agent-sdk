---
schema_version: 1
id: behavior.sdk-family-value
kind: behavior
status: needs_review
expectation: human_confirmed
risk: high
primary_focus: state_authority
focus:
- time_lifecycle
- failure_concurrency
- contract_evidence
- data_durability
boundary: boundary.sdk-language-clients
observed_at: 622d7931e01556161441d22de0697f6f35f24f17
code_refs:
- echo-sdk-protocol/src/scalar.rs
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-candidate-behavior-verification
- evidence.sdk-parent-driver-candidate-verification
- evidence.sdk-protocol-finite-observation
finding_refs:
- finding.sdk-accepted-source-continuity
- finding.sdk-chart-dimension-integer-loss
---

# SDK family value

## 重要承诺

对已接受 `value` 能力明确入口、唯一权威、状态/生命周期、失败处置和直接行为证据；未接受 Rust identity 不进入承诺。

## 当前行为

- **入口/可达**：`echo-sdk-protocol/src/scalar.rs` 及 accepted schema/value routes；Host
  `wire.rs`，TS/Python wire、Java `WireValues.java`/`JsonSupport.java` 的 decode/encode。
- **权威/状态**：accepted DTO 拥有值合同；framework 原字段保持业务事实。无独立 Run/
  store；encoded → validated → projected → round-trip，不能把不支持的值变成 null/default。
- **正常/失败**：u64/i64、纳秒时间/duration、paths/OS encoding、bytes、Unicode、null vs
  absent、variant/record/map duplicates、float limits、opaque handles；每条转换错误可定位。
- **已有断言**：Wire `managed_checkpoint_and_receipts_preserve_large_revisions_and_subseconds`、
  tool effects/model content tests；三语言 protocol-integrity 消费 shared malformed corpus，
  special keys/中文🙂、opaque error handle、missing response envelope。
- **最低充分证据**：同一 accepted field corpus 作三语言 → Host → framework → Host round-trip，
  对超 JS safe integer、subsecond time、非 UTF-8/Windows path 的允许/拒绝边界、invalid
  optional 与 unknown discriminant 作精确断言；platform 不可运行时保留未验证项。

## 期望行为

上述正常/失败场景须保持接受范围、完整转换和唯一权威，并在最终源码/pin证据及独立复审后闭合。

## 触发、结果与副作用

按上方具体入口和正常/失败记录检查真实主路径。可达性或catalog状态不能证明副作用、错误或结算。

## 失败、重试与恢复

按上方生命周期和最低充分证据检查取消、超时、错误、释放和恢复；不默认成功，不以其它能力族测试替代。

## 证据

阶段：`candidate_runtime_verified`；语义状态仍为 `needs_review`。

指定 wire tests 保留大 revision、subseconds、result absent/null、non-UTF8 Unix path；三语言 wire/malformed corpus 和 typed error fields 实际执行。

实际来源：`echo-sdk-host/src/core_profile/wire.rs#managed_checkpoint_and_receipts_preserve_large_revisions_and_subseconds`；`echo-sdk-host/src/core_profile/wire.rs#unix_non_utf8_paths_round_trip_without_loss`；`sdks/typescript/test/wire.test.js`；`sdks/python/tests/test_protocol_integrity.py`；`sdks/java/src/test/java/com/echoagent/sdk/ProtocolIntegrityTest.java`。

仍缺：Windows native path 与所有 accepted field 经 Host/framework 的统一 corpus 尚未完成；语言-local 单测不充当 Host round-trip。 published framework pin、最后变更后的独立复审和精确main验收
仍待；本证据不证明未列出的生命周期、字段或错误组合，也不自动扩大accepted合同。

## 裁决记录

用户要求SDK全能力行为治理，以明确接受跨语言合同为兼容边界；完整Rust inventory仅遥测。

## 当前观察与历史收据边界

Candidate assertion evidence is now preserved as original receipt/source fingerprints and checked against inspected current paths. It supports only named partial cases; current observation binding neither reruns old commands nor upgrades family status.

## 本次有限审查单元

行为/生命周期单元：闭合WireValue标量/容器、u64、路径、时间、Unicode、optional/未知wrapper无损。

剩余处置：Windows原生路径平台门禁与全部明确接受字段source生成；不把process-local Rust类型接受到JSON。

本项建模阈值是已接受入口、唯一状态权威、正常生命周期与具名失败/恢复路径可定位，并有直接证据或明确开放原因；不要求所有未定义组合全测，也不由绿色测试自动verified。

## 当前模型重观察范围

review_scope：本对象在真实当前候选Git `56929154c1851473e9e4837901b610cecf001b2d`（源码摘要`17cf243e8d6341db0ce23c5b9ee39977ce4ab100d6ed1b17ec5a7bb26c19e138`）上重新观察。上列code_refs/各scenario source_refs已逐Gitblob读取并与当前文件SHA精确相同，原锚点均存在。模型继续描述原accepted入口、框架业务/终态权威、Host owner/generation与transport消费职责、生命周期及具名失败义务；源码定位核对不证明所有行为正确、运行证据fresh或合同生成闭合。

原Behavior/status、Map场景mapped/needs_review、Rule/Asset处置、Finding/未决义务与Evidence原epoch支持范围全部保留。mapped仅入口模型，旧3ea不是本snapshot本体；历史receipt不得因本模型观察更新冒current运行。12source迁移、5秒物理结算与依赖50warning规则三选择仍未接受，完整生成/CI/main另验，任何新Rustwrapper不自动扩三语言合同。

## Protocol与广告shape后继有界闭合

原SDKcap独立Protocol8lens source审BLOCK的specificshape现经合法Hostproxy真before修复/newinitowner增量及实际源/材料独审PASS，source622d与原gate取证同。JBigIntegerexactversion/capfeatures全member拒/17canonicalpositiveu64/positivecallbacku32/2duration/nonemptydistinctcaps、Unicodecodepointsortedfeatures；Python officialValidationError仅initialize typed，TSfailedinithelper实际EOF/childexit持有原owner，原primaryerrorcleanupcause保。六genuineJava前误接受、TS/PY两shape/PYtypedoverflow与七afterOnly负例不混；TS37/PYintegrity31/原J21/PY25按freeze各切点，不旧782whole复用为新source完整通过。

具体negotiationFinding修复/反例/verification/actualrereview三链已齐可localresolved，Behaviour仍needs_review/8lensexamined不全familyverified。core_rpc_contract/stdstdio4/plain/mismatchedhello和require_extended/negotiate_hello直接引用真实，旧运行time不因观察622d被重标；Windowsnative、fullgenerator12/B5physical平台/main独立。协议类编译catalog不是全部Rustinventorylanguageparity，无新接口/字段/错误taxonomy。
