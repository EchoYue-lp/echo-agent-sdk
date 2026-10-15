---
schema_version: 1
id: map.sdk-product-boundary
kind: capability_map
title: 独立 SDK 产品边界
risk: high
observed_at: 7d0a32a0b049013e366d9a4ed78049c7364cbb98
boundary_refs:
- boundary.sdk-product
behavior_refs:
- behavior.sdk-repository-ownership
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-source-import
- evidence.sdk-source-continuity
- evidence.sdk-framework-pin
- evidence.sdk-wave2-cutover-repair
- evidence.sdk-wave2-cutover-final-verification
- evidence.sdk-compatibility-gates
- evidence.sdk-feature-wiring-verification
- evidence.sdk-current-reviewed-rust-gates
- evidence.sdk-cargo-deny-bootstrap-closure
finding_refs:
- finding.sdk-wave2-cutover-integrity
- finding.sdk-accepted-source-continuity
- finding.sdk-dependency-warning-policy-conflict
- finding.sdk-test-feature-wiring
- finding.sdk-cargo-deny-bootstrap-toolchain-probe
audit_refs:
- audit.sdk-wave2-cutover-final-rereview
- audit.sdk-cargo-deny-bootstrap-closure
related_map_refs: []
scenarios:
  compatibility-gates:
    status: mapped
    source_refs:
    - echo-sdk-protocol/tests/facade_inventory.rs
    - scripts/check-language-sdks.sh
    - sdks/typescript/test/catalog.test.js
    - sdks/python/tests/test_catalog.py
    - sdks/java/src/test/java/com/echoagent/sdk/FacadeParityTest.java
    - echo-sdk-host/src/core_profile/state.rs
    behavior_refs:
    - behavior.sdk-repository-ownership
    rule_refs:
    - rule.framework-runtime-authority
    evidence_refs:
    - evidence.sdk-compatibility-gates
  source-history:
    status: mapped
    source_refs:
    - MIGRATION-SOURCE.md
    - docs/adr/0001-sdk-repository-boundary.md
    behavior_refs:
    - behavior.sdk-repository-ownership
    evidence_refs:
    - evidence.sdk-source-continuity
    - evidence.sdk-wave2-cutover-repair
    finding_refs:
    - finding.sdk-wave2-cutover-integrity
  protocol-and-host:
    status: mapped
    source_refs:
    - echo-sdk-protocol/Cargo.toml
    - echo-sdk-host/Cargo.toml
    - echo-sdk-host/src/core_profile/wire.rs
    behavior_refs:
    - behavior.sdk-repository-ownership
    evidence_refs:
    - evidence.sdk-framework-pin
    - evidence.sdk-wave2-cutover-repair
    finding_refs:
    - finding.sdk-wave2-cutover-integrity
  accepted-contract:
    status: mapped
    source_refs:
    - contracts/sdk/accepted-external-contract.json
    - contracts/sdk/source-contract.json
    - contracts/sdk/inventory-telemetry.json
    behavior_refs:
    - behavior.sdk-repository-ownership
    evidence_refs:
    - evidence.sdk-framework-pin
    - evidence.sdk-wave2-cutover-repair
    finding_refs:
    - finding.sdk-wave2-cutover-integrity
  language-clients:
    status: mapped
    source_refs:
    - sdks/shared/contract-digests.json
    - sdks/typescript/src/index.ts
    - sdks/python/src/echo_agent_sdk/__init__.py
    - sdks/java/pom.xml
    behavior_refs:
    - behavior.sdk-repository-ownership
    evidence_refs:
    - evidence.sdk-framework-pin
    - evidence.sdk-wave2-cutover-repair
    finding_refs:
    - finding.sdk-wave2-cutover-integrity
  wave2-cutover:
    status: mapped
    source_refs:
    - Cargo.lock
    - echo-sdk-host/Cargo.toml
    - echo-sdk-protocol/src/facade.rs
    - contracts/sdk/inventory-telemetry.json
    - sdks/shared/contract-digests.json
    behavior_refs:
    - behavior.sdk-repository-ownership
    rule_refs:
    - rule.framework-runtime-authority
    evidence_refs:
    - evidence.sdk-wave2-cutover-repair
    finding_refs:
    - finding.sdk-wave2-cutover-integrity
  semantic-ci-canonical-source-check:
    status: mapped
    source_refs:
    - .github/workflows/rust-ci.yml
    - docs/sdk/governance/evidence/sdk-semantic-ci-consumer/README.md
    - docs/sdk/governance/evidence/sdk-published-semantic-tool-22e/README.md
    behavior_refs:
    - behavior.sdk-repository-ownership
    rule_refs:
    - rule.framework-runtime-authority
    evidence_refs:
    - evidence.sdk-framework-pin
    review_scope: CIjob定义/唯一官方action provenance/真实base失败关闭及已执行4994原30d资格；mapped不是newremoteexecutionpassed。
    limitations: 新semanticjob尚未执行；SDKcontract13/B12/原task513/Windowsnative/SDKmain另留。

---

# 独立 SDK 产品边界

## 能力范围

覆盖 SDK source history、Rust protocol/Host、合同与 TypeScript、Python、Java clients。

## 入口与输出

入口是 ACP standard 与 `_echo_agent/*` extension；输出是 typed response、event、stream、handle 和稳定错误。

## 行为关系

SDK 只投影 framework runtime；clean pin 将 framework 转换集中在 Host，不改变运行权威。

## 状态与数据流

Framework 拥有 Agent/Run/Task/Subagent 与 terminal 状态；Host 只持有协议寻址和资源生命周期。

## 策略来源与优先级

本仓 ADR、accepted contract 和精确 framework dependency 依次约束 repository、wire 与 build 行为。

## 生命周期与失败路径

Protocol DAG、framework pin、accepted contract、Host E2E 或语言 gate 失败会阻止交付声明；
完整 Rust inventory 漂移只形成遥测信号。
分类样例只检查仍存在的 identity；完整 scope 数量与 intrinsic membership 不冻结兼容面。
Host 结算计数器保留饱和递减与 Acquire/AcqRel 顺序，不引入新的结算权威。

## 权限与敏感信息

SDK 不新增线上权限模型，凭据不得写入合同、日志或迁移证据。

## 用户侧投影

三语言保留惯用 API，同时不得复制 framework 的状态机和恢复权威。

## 场景处置清单

历史、ownership、protocol/Host、accepted contract 和三语言独立门禁均已映射。

## 未展开项

完整 capability 分类和生命周期将在后续 SDK 收敛阶段展开。

## 当前治理快照限制

上述已验证行为绑定历史 `30d7e219`。本轮显式 framework 升级、扩展桥/Task/事件/资源修复及三语言变化仍在实现和反例验证中；当前源码摘要、合同生成、独立复审和 main 验收完成前，不恢复 verified/closure 声明。

## 当前模型重观察范围

review_scope：本对象在真实当前候选Git `56929154c1851473e9e4837901b610cecf001b2d`（源码摘要`17cf243e8d6341db0ce23c5b9ee39977ce4ab100d6ed1b17ec5a7bb26c19e138`）上重新观察。上列code_refs/各scenario source_refs已逐Gitblob读取并与当前文件SHA精确相同，原锚点均存在。模型继续描述原accepted入口、框架业务/终态权威、Host owner/generation与transport消费职责、生命周期及具名失败义务；源码定位核对不证明所有行为正确、运行证据fresh或合同生成闭合。

原Behavior/status、Map场景mapped/needs_review、Rule/Asset处置、Finding/未决义务与Evidence原epoch支持范围全部保留。mapped仅入口模型，旧3ea不是本snapshot本体；历史receipt不得因本模型观察更新冒current运行。12source迁移、5秒物理结算与依赖50warning规则三选择仍未接受，完整生成/CI/main另验，任何新Rustwrapper不自动扩三语言合同。


## 后继722 Linux CI局部开放记录

`finding.sdk-cargo-deny-bootstrap-toolchain-probe` 当前open：现有CI dependency action安装前探测stable-musl工具链未装产生bootstrap error，原fallback后audit/四deny及50duplicate范围不抹error且非漏洞/deny失败；单input rust-version:stable实施已审但新remote验证未到，不自动闭合。

本记录发现观察基准722及PRmerge5d407仅定位后继测试/CI材料，Map原observed_at、所有旧scene/source_refs/status、行为义务和原Evidence审查仍属原epoch；新增只本owner的Finding引用，不复制到所有Map，不提升或失效全8视角/41族。原日志目前Admin完整Hash定位，正式Git归档后再补当期来源，不伪旧Git含未来raw；旧56/4是原切点，安装这三新具体缺陷后56resolved/7open，B3共享物理B与12连续性不改变。


## 正式CI保存材料的当期观察（f68）

本段当期观察Git `f68cb49bf421ecdb71dfa4775495c363e31b3324`仅定位正式保存的722运行材料，不把运行时间或源码测试执行标成f68。原发现点`722d09b1a0837f170d4af19cbc3e417d2131b5f3`、PR实际merge5d407、候选所有原Map观察/scenes/status/锚点与对应测试源码SHA保持。正式来源逐路径与Hash如下，可从该Git读取：

- `docs/sdk/governance/evidence/sdk-722-ci-results/README.md`，Git f68 SHA256 `dcd6a9e4922a05527b189a2c03108a3ac73936d5107d022600dada1e570f0b27`。
- `docs/sdk/governance/evidence/sdk-722-ci-results/manifest.json`，Git f68 SHA256 `4eb02f38b55191bfa761cd892b667733dcb26d4462eb655cfd2a22cbd379443e`。
- `docs/sdk/governance/evidence/sdk-722-ci-results/SHA256SUMS.json`，Git f68 SHA256 `c22f92784ad1874ffcb0bb70ad312965e50bda78dcd69871230b2073006f1e44`。
- `docs/sdk/governance/evidence/sdk-722-ci-results/jobs/116172597308.log`，Git f68 SHA256 `70548f9331858291e374f6e751581cfcf854d8767900d03738d5609041880547`。
- `docs/sdk/governance/evidence/cargo-deny-stable-toolchain/README.md`，Git f68 SHA256 `463e816ecca27812b3b371c7ab041e769b785336020ed8c434f056084736c202`。
- `docs/sdk/governance/evidence/cargo-deny-stable-toolchain/original-722-dependency-job.log`，Git f68 SHA256 `59dc929cd16b4f36b4a30a6eaf8e3e3db9d6f22af695e3bf9f1fba094fb5ccff`。
- `docs/sdk/governance/evidence/cargo-deny-stable-toolchain/installed-validation.json`，Git f68 SHA256 `0deab4d51fa6ce53b8a5cf09009c56afed02401c1ee4837244496d253645f5d2`。
- `docs/sdk/governance/evidence/cargo-deny-stable-toolchain/provenance.json`，Git f68 SHA256 `6a4af51067158eb9aaac48dc6e8a0666eb8713974ec5c69cf528fea912c4bc2c`。

正式完整八job范围为5success/3failure；TS336及quickstart通过，Python413通过/2失败/18subtests，Java未达。两个合同job原13连续性阻断与旧证明分别保留，不从CI fail泛化未知生产缺陷。PID原Linux[]唯一cause仍unknown，Darwin2.5925s排队原反例不是该CI唯一cause。bootstrap单input实施只原配置/顺序依据，新remote error结算尚未证明，不提前resolved。原四已闭Mac缺陷不重开、B/12未决保持。

原上文“第三repo工具交付权限待批准”属于候选准备时历史；Root后续已获得第三repo工具修复交付授权，实际工具修补/源码SHA/发布main与新CIjob结果尚未回报，本候选不冒已交付或官方原30d门禁已0。


## 公开canonical语义CI定义的当期模型观察

当前Mapobserved7d0只定位已有模型展示与新增CI定义；原569以及旧段运行时间/验收scope仍保，原六scenarios字典/状态/source_refs未改，已有source路径在7d0逐Git核存在/锚点有效，不声称现有业务场景重跑或全部当前verified。新增scenario只说明SDKProductCI原owner复用公开exact22e官方action且base来自真实event，定义已提交/新job未实跑；已执行全goal资格只属4994/工具22e/localsharedclone，不能推出后继workflow或当前semanticCIgreen。权限/合同/API/运行权威未新增，B12及所有Finding原状态不动。


## dependencybootstrap具名后验关联

原bootstrapFinding已在本Map；本次只把specificrepair/remoteverification Evidence与contract-lensAudit加入原owner引用。原新semanticjob定义场景仍只defined/notexecuted，工具22e的SDK4994原30dactual0与dependencyjob特定SUCCESS不同scope，不能互借成整CI或SDKmain全绿。原Map7d0观察/六旧scenes/所有状态与业务义务保。
