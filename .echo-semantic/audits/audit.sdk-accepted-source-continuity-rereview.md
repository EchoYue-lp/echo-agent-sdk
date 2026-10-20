---
schema_version: 1
id: audit.sdk-accepted-source-continuity-rereview
kind: audit
boundary_ref: boundary.sdk-product
lens: contract_evidence
freshness: examined
revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
finding_refs:
- finding.sdk-accepted-source-continuity
challenges:
  exact-accepted-membership-and-consumers:
    revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_refs:
    - docs/adr/0034-sdk-governance-authority-and-lossless-adaptation.md
    - docs/sdk/governance/accepted-source-continuity-proposal.md
    - contracts/sdk/accepted-external-contract.json
    - contracts/sdk/accepted-facade-operation-catalog.json
    - echo-sdk-protocol/src/facade.rs
    - echo-sdk-protocol/tests/facade_inventory.rs
    - echo-sdk-host/tests/core_profile_e2e.rs
    - echo-sdk-host/tests/extension_bridge_e2e.rs
    - echo-sdk-host/src/core_profile/extension_bridge.rs
    - echo-sdk-host/src/core_profile/facade/task_runtime.rs
    - sdks/typescript/test/intrinsic-tool-values.test.js
    - sdks/python/tests/test_intrinsic_tool_values.py
    - sdks/java/src/test/java/com/echoagent/sdk/IntrinsicToolValueTest.java
    - sdks/typescript/test/secret-scan-host.test.js
    - sdks/python/tests/test_secret_scan_host.py
    - sdks/java/src/test/java/com/echoagent/sdk/SecretScanHostTest.java
    evidence_refs:
    - evidence.sdk-accepted-source-continuity-gate
  named-settlement-family-task:
    revision: 04eb6ad50982656b5a8b8ea649619b6a383065ff
    source_refs:
    - echo-sdk-host/src/core_profile/facade/task.rs
    - echo-sdk-host/src/core_profile/facade/task_runtime.rs
    - echo-sdk-host/src/core_profile/facade/subagent.rs
    - echo-sdk-host/src/factory.rs
    - echo-sdk-protocol/src/methods.rs
    evidence_refs:
    - evidence.sdk-task-subagent-candidate
    - evidence.sdk-task-context-direct-getter-consumption
    - evidence.sdk-task-memory-workflow-finite-observation
---

# 已批准SDK源身份迁移的独立复审

## 审查范围

独立只读reviewer审基点a48854a及全部25个受控文件差异，指定裁决全文、ADR0034
全局不变量、精确处置和回滚，无Plan。reviewed path/hash映射的规范JSON SHA256为
e703665f23cbc3bf45056ac72f3257e28ab2208cf7729e3b59f4e8bdcc9c7bf7。
最终结论PASS，Critical/Important/Minor及行动项均0。作者只安装真实复审结论；不替
独立reader重写审查或把后继metadata作为新的业务执行。

## 已检查故障假设

是否额外接受Rust inventory；Security defining-source移动是否改operation/签名/输出；
getter是否误变语言API；Snapshot退役是否删主能力；全profile是否部分冒全量；
测试scope修正是否漏旧28/helper/已支持trait；deferred是否静默default；
Factory callback入口marker是否误作Host结算；历史日志/旧Host是否冒当前成功。

## 实际实现路径与证据

精确5622-13+12=5621，无邻接身份扩张；7项Security签名相同、root alias与旧3operation
保留，3getter4alias各method/host_or_rust_only。唯一generator完整28update与同原完整check
均0，117artifacts/0telemetry drift。旧28exact集合及supported traits断言完整。
Factory只测试修正：精确无副作用quota条件/单10s预算与真实heldACK原lease直接断言，
未改生产ordering/API。强unit、E2E及完整workspace实际通过。

最终Rust8命令全0/workspace478/0/0，225构建输入现场逐hash同。完整官方语言入口0：
TS343/PY427+18subtests/Java363，现场76freshXML全部hash匹配且无fail/error/skip。
793输入before/after/当前同，新实际Host707116784B SHA0e97169a4265761ab436932e4b413b5850040525172351999ce6676ec939eafe。
三个语言Security原始observed各15rows及raw scan，zero-provider/noeffects/UTF8offset11。
原Rust101、TS342/1及资源中断保各epoch，不由后继成功抹除。

## 问题记录

无阻断或非阻断finding。记录安装后的source_snapshot/observed_at/audit引用与Finding状态
是非行为增量，由主代理核最终diff与真实finalgate；不重跑无关消费者。

## 残余风险

只覆盖第1项兼容切片；B/宽cleanup/Sandbox/Task-resume/managed未知原因不闭合。
原22e CI型精候选ordinary strict及原30d变更依据门禁须实际执行，普通strict不能替代后者。
安装0.5 task-scope的历史引用/8MB限制独立记录，不删PF或改工具求绿。上游工具c0bc及
SDK action/归档continuity/main由父任务后继整合；无发布、合main或关闭外部Issue。

## 未检查项

不替Linux/Windows原生消费者、B新pin、全SDK main/CI、第三工具迁移或全局行为闭合验收。


## 2026-10-20 具名后继的有界结算

主header revision/freshness及原挑战仍保原审查cut；这里只记录下列named单元，原17be/0bbc读审与04eb受控media增量分开，旧Evidence运行不重标。任何一个row不能推整八风险组或整composite资格。

- `behavior.sdk-family-task`：有限named单元结算；证据`evidence.sdk-task-subagent-candidate`、`evidence.sdk-task-context-direct-getter-consumption`、`evidence.sdk-task-memory-workflow-finite-observation`。
