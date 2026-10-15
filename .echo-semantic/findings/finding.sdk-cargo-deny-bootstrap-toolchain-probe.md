---
schema_version: 1
id: finding.sdk-cargo-deny-bootstrap-toolchain-probe
kind: finding
type: implementation_bug
status: resolved
severity: low
primary_focus: contract_evidence
focus:
- result_side_effect
boundary_ref: boundary.sdk-product
behavior_refs:
- behavior.sdk-repository-ownership
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-cargo-deny-bootstrap-closure
audit_refs:
- audit.sdk-cargo-deny-bootstrap-closure
decision_refs: []
repair_evidence_refs:
- evidence.sdk-cargo-deny-bootstrap-closure
verification_evidence_refs:
- evidence.sdk-cargo-deny-bootstrap-closure
rereview_audit_refs:
- audit.sdk-cargo-deny-bootstrap-closure
discovered_at: 722d09b1a0837f170d4af19cbc3e417d2131b5f3
---

# cargo-deny action安装前探测缺工具链留下真实bootstrap error

## 问题

原722 dependency job日志有override toolchain not installed错误发生在action bootstrap rustup show探测，随后fallback stable1.99安装并四检查成功/50acceptedduplicate保原。Job成功没有抹去先前error，但该错误不是已发现依赖漏洞、deny检查失败或policy例外增量。

CurrentTool已独审官方actionv2原commit3c6349835b2b7b196a839186cb8b78e02f7b5f25入口顺序及最小设计：现有with加rust-version:stable使必要install先于probe，不静默/suppress/downgrade、不改audit/deny参数或锁/50policy。当前workflow workingSHAc788候选已写尚无新CI结果，不能观测722旧tree假包含新input。

## 触发条件与影响

发现基准722/PR实际merge `5d407e160bb5c7ade01be1980ed9a4e5a6888d17`，不冒裸head执行。原本地第四whole800/TS336/PY41518/J35576XML真实0保macOS来源；新Linux反例不倒改旧执行/不批量失效8Host风险审。

## 证据

原Linux语言log Admin `/Users/ls/MyWork/code/ylp_agent_learn/lp-agent/.worktrees/admin/sdk-722-ci-evidence/116172597308.log`，198565B，SHA256 `70548f9331858291e374f6e751581cfcf854d8767900d03738d5609041880547`，内容只读。当前新证据尚未正式Git，不伪source:pending Evidence或futureGit。

- `.github/workflows/rust-ci.yml`，原722 Git源码SHA256 `b37d59b4349e956c8c6eee19f41e8d4dfa1c48a2ca4396b321f6350388b12021`。

bootstrap单独原dependency日志原字节为1,084,474B，SHA256 `59dc929cd16b4f36b4a30a6eaf8e3e3db9d6f22af695e3bf9f1fba094fb5ccff`，实际line310为override stable-musl toolchain未安装；它与语言log分别读取。原dependency raw与官方源/设计位置是 `docs/sdk/governance/evidence/cargo-deny-stable-toolchain/` 当前未提交包，不能据语言log给bootstrap结论；其原exacthash由manifest/provenance列出，下一正式Git后再合法引用。

## 处理记录

正式30文稿包保存原完整error/job事实和官方action源码/设计，但当前只是实施/AST顺序证据，不真实Docker/advisory红绿。需要后继真实remote dependency job warning/error正确结算、四deny类与audit及50exactgroups按原政策审，才可bounded locallyresolved；不得凭设计/命令模拟或旧jobsuccess自动关。原B12/API/schema/Runtime权威不变。

此open候选仅本测试support或CIbootstrap边界，不新增family/API/权限/RiskAccepted。原四已解决Mac具名缺陷保持闭合，56/4是原切点而非含本新缺陷的当前最终总数。全goal30d UTF8工具修复第三repo仍待授权，不能改工具或scope来关闭这些记录。


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


## 已提交修复与有限后验进展（仍open）

单action input rust-version:stable已f68真提交，已有官方actionv2源码/入口顺序/AST单差与独立实施材料PASS；不静默/降级/改锁/deny4policy。此是安装顺序有限证据，新remote warning/errorfree依赖job未运行/未审，原bootstraperror虽job后来success仍保，当前Findingopen而不是漏洞或deny检查failed。

当前材料观察Git `ff3de936d22c6d5f4e68ece646f0b4cfd77e7c56`仅定位已提交源码和正式包；原722/merge5d407失败运行/发现时间不改，新remote原case仍待。以下实际Git来源可读，不把本地focused当Linux完整验收：

- `.github/workflows/rust-ci.yml`，SHA256 `c788cc19d41f0bfb00b05cc4f5cc2581ea3f14b74809405d547169918028e4c3`。
- `docs/sdk/governance/evidence/cargo-deny-stable-toolchain/README.md`，SHA256 `463e816ecca27812b3b371c7ab041e769b785336020ed8c434f056084736c202`。
- `docs/sdk/governance/evidence/cargo-deny-stable-toolchain/installed-validation.json`，SHA256 `0deab4d51fa6ce53b8a5cf09009c56afed02401c1ee4837244496d253645f5d2`。
- `docs/sdk/governance/evidence/cargo-deny-stable-toolchain/provenance.json`，SHA256 `6a4af51067158eb9aaac48dc6e8a0666eb8713974ec5c69cf528fea912c4bc2c`。
- `docs/sdk/governance/evidence/cargo-deny-stable-toolchain/design-review.json`，SHA256 `d84bf610a85cf43d1d40a6ebaa3a828ed1edb911b712a0d2c915296cd99a4ff3`。
- `docs/sdk/governance/evidence/cargo-deny-stable-toolchain/original-722-dependency-job.log`，SHA256 `59dc929cd16b4f36b4a30a6eaf8e3e3db9d6f22af695e3bf9f1fba094fb5ccff`。
- `docs/sdk/governance/evidence/cargo-deny-stable-toolchain/manifest.json`，SHA256 `b2ff05162836d18c4cf478fff19d45fbc5f8ff953970ca5fa36b910afd4f52c0`。

源4tests已变化，先前第四whole800完整0继续保它自己的执行cut而非声称当前all800exact。生产61Rust/Cargo输入与锁未因此变化；本次不会重复原14Rust或本地大target完整门禁。新CI按focused开发门禁后实跑方案另执行，暂不关闭本Finding或提升Behavior/库存/全goal完成；B3/12、第三工具交付和官方原30d门禁各按真实当前进展处理。


## 新remote局部闭合条件候选（不可安装）

原722依赖job先报override stable-musl toolchain未安装，之后fallback成功和4类deny通过不抹该error。f68单input rust-version:stable修复复用action原入口，使必要install/default先于rustup show，不改action版本、工具root约束、锁、command/feature、deny检查或50group政策；设计/AST实施原独审PASS0。新dependency-policy job116211906823实际SUCCESS，PRhead4994、实际checkout merge29a2d95db11189b555fa61d25822e8a1d8043230，不冒裸head执行。新raw中withrust-versionstable与Docker参数stable实际生效，default toolchain install在show之前，原overrideerror不存在；cargoaudit成功、deny4类通过，50warning[duplicate]保原字节，其它warning/error为0。SDKcap已actualbounded rereviewPASS C0/I0/M0并核50crate版本数组与已批准6bc0ed基准完全同；模型作者仅回收该审结论，未把只数50当版本组全对。

该remote job原raw1085425B/SHA89a84791...和jobJSON当前只在Admin/sdk-4994-ci-evidence，须等正式wholeCI必要归档入真实SDKGit并再次绑定当期观察/refs；本候选ready=false，S保持open。原722 before、原工作流程字节与发现时间保。不因dependency job绿推其它Linux语言、ACK/PID、合同13、B/12、SDKmain或wholeCI全绿。
原上文缺newremote的文字是修复前阶段事实；本段只后继具体bootstrap义务，不抹旧error/原discovered722。若只安装这一项，计数57resolved6open，ACK/PID仍新Linuxpending，B3/12继续原状态。


## 完整4994终态与待正式Git绑定

Root已取得4994全部八job自然终态5success/3failure并完成正式28归档独审：source语言TS336/QS通过、PY414通过1失败和18subtests、Java未达；PID截断输出pids[]仍不推typedtimeout，ACK不在该run失败列表也不自动全覆盖。两个合同job同13连续性阻断。局部dependencybootstrap remote成功足specific闭合资格，不等全CIgreen。正式包目前workingtree已存在，但尚无已宣布提交SHA，因此本候选仍只在Admin不install、不添加这些future source_refs或假observed7d0；真实commit后只增来源/当期revision并轻核。


## 正式9061后验归档与当期来源复核

原上文“remote raw未提交/候选不可装”是Admin前稿封存时点，原字节另保before。现在完整4994原八job后验28正式文件已在真实SDK Git `9061a9551807de14edb0227249ee07c12b31b0bd`保存；本次Evidence/Audit当期观察9061只定位原源码/原后验材料，不冒新9061执行。实际job116211906823为PRhead4994、checkoutmerge29a2d95，在日志中stable必要安装/default先于show，旧overrideerror消失，audit及4deny成功，精确50crate版本数组与原6bc0ed已批准组相同，其它warnings/errors0由SDKcap真实独审PASS0，原raw完整保。

原Findingspecificbootstrap的repair（f68单existinginput）/remoteverification/独立rereview三链现有事实足局部resolved资格；只待本轮元artifact审与Root安装准入，不再把已提交正式包当pending。新来源sourceRefs逐9061Git核而非把futurepaths放4994；discovered722/原失败日志/同source修复时点保。若本轮安装只有bootstrap闭合，整体57resolved6open：Gap/PID仍newLinux后验不全、原PIDuniqueLinuxcauseunknown，B3/12合同待决与全部CI5success3failure/合同13/Windowsnative/SDKmain保持独立。
