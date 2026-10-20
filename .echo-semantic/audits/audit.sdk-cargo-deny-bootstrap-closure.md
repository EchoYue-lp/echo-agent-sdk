---
schema_version: 1
id: audit.sdk-cargo-deny-bootstrap-closure
kind: audit
boundary_ref: boundary.sdk-product
lens: contract_evidence
freshness: examined
revision: 9061a9551807de14edb0227249ee07c12b31b0bd
finding_refs:
- finding.sdk-cargo-deny-bootstrap-toolchain-probe
challenges:
  bootstrap-install-before-probe:
    revision: 9061a9551807de14edb0227249ee07c12b31b0bd
    source_refs:
    - .github/workflows/rust-ci.yml
    - docs/sdk/governance/evidence/cargo-deny-stable-toolchain/README.md
    - docs/sdk/governance/evidence/cargo-deny-stable-toolchain/provenance.json
    - docs/sdk/governance/evidence/cargo-deny-stable-toolchain/entrypoint.sh
    - docs/sdk/governance/evidence/cargo-deny-stable-toolchain/action.yml
    - docs/sdk/governance/evidence/cargo-deny-stable-toolchain/installed-validation.json
    - docs/sdk/governance/evidence/cargo-deny-stable-toolchain/one-input.patch
    - docs/sdk/governance/evidence/cargo-deny-stable-toolchain/original-722-dependency-job.log
    - docs/sdk/governance/evidence/cargo-deny-stable-toolchain/design-review.json
    - docs/sdk/governance/evidence/sdk-4994-ci-results/README.md
    - docs/sdk/governance/evidence/sdk-4994-ci-results/manifest.json
    - docs/sdk/governance/evidence/sdk-4994-ci-results/SHA256SUMS.json
    - docs/sdk/governance/evidence/sdk-4994-ci-results/observations/run-terminal.json
    - docs/sdk/governance/evidence/sdk-4994-ci-results/jobs/116211906823.json
    - docs/sdk/governance/evidence/sdk-4994-ci-results/jobs/116211906823.log
    evidence_refs:
    - evidence.sdk-cargo-deny-bootstrap-closure
  exact-main-1e1-delivery-qualification:
    revision: 1e1c4f03f670561d6ca804a56d35967eeea191ce
    source_refs:
    - .github/workflows/rust-ci.yml
    - Cargo.lock
    - deny.toml
    evidence_refs:
    - evidence.sdk-cargo-deny-bootstrap-closure
---

# CIbootstrap有限源与remote事实复审

## 审查范围

回收CurrentTool原官方action/单input设计实施审和SDKcap新4994dependency raw有界审，模型作者不是新业务审者。

## 已检查故障假设

job成功是否掩原error；更改是否省略检查/静默warnings/降toolchain；50数量是否遮数组增量；PRmerge/head是否混；oldbefore是否冒newafter。

## 实际实现路径与证据

原722依赖job先报override stable-musl toolchain未安装，之后fallback成功和4类deny通过不抹该error。f68单input rust-version:stable修复复用action原入口，使必要install/default先于rustup show，不改action版本、工具root约束、锁、command/feature、deny检查或50group政策；设计/AST实施原独审PASS0。新dependency-policy job116211906823实际SUCCESS，PRhead4994、实际checkout merge29a2d95db11189b555fa61d25822e8a1d8043230，不冒裸head执行。新raw中withrust-versionstable与Docker参数stable实际生效，default toolchain install在show之前，原overrideerror不存在；cargoaudit成功、deny4类通过，50warning[duplicate]保原字节，其它warning/error为0。SDKcap已actualbounded rereviewPASS C0/I0/M0并核50crate版本数组与已批准6bc0ed基准完全同；模型作者仅回收该审结论，未把只数50当版本组全对。

## 问题记录

只闭安装顺序bootstrap，原不是漏洞或denyfailed，不能退役旧rule/改风险接受。

## 残余风险

该remote job原raw1085425B/SHA89a84791...和jobJSON当前只在Admin/sdk-4994-ci-evidence，须等正式wholeCI必要归档入真实SDKGit并再次绑定当期观察/refs；本候选ready=false，S保持open。原722 before、原工作流程字节与发现时间保。不因dependency job绿推其它Linux语言、ACK/PID、合同13、B/12、SDKmain或wholeCI全绿。

## 未检查项

不替其它CIjob/nativeWindows/全goal/callback/data contracts作验收。


## 完整4994终态与待正式Git绑定

Root已取得4994全部八job自然终态5success/3failure并完成正式28归档独审：source语言TS336/QS通过、PY414通过1失败和18subtests、Java未达；PID截断输出pids[]仍不推typedtimeout，ACK不在该run失败列表也不自动全覆盖。两个合同job同13连续性阻断。局部dependencybootstrap remote成功足specific闭合资格，不等全CIgreen。正式包目前workingtree已存在，但尚无已宣布提交SHA，因此本候选仍只在Admin不install、不添加这些future source_refs或假observed7d0；真实commit后只增来源/当期revision并轻核。


## 正式9061后验归档与当期来源复核

原上文“remote raw未提交/候选不可装”是Admin前稿封存时点，原字节另保before。现在完整4994原八job后验28正式文件已在真实SDK Git `9061a9551807de14edb0227249ee07c12b31b0bd`保存；本次Evidence/Audit当期观察9061只定位原源码/原后验材料，不冒新9061执行。实际job116211906823为PRhead4994、checkoutmerge29a2d95，在日志中stable必要安装/default先于show，旧overrideerror消失，audit及4deny成功，精确50crate版本数组与原6bc0ed已批准组相同，其它warnings/errors0由SDKcap真实独审PASS0，原raw完整保。

原Findingspecificbootstrap的repair（f68单existinginput）/remoteverification/独立rereview三链现有事实足局部resolved资格；只待本轮元artifact审与Root安装准入，不再把已提交正式包当pending。新来源sourceRefs逐9061Git核而非把futurepaths放4994；discovered722/原失败日志/同source修复时点保。若本轮安装只有bootstrap闭合，整体57resolved6open：Gap/PID仍newLinux后验不全、原PIDuniqueLinuxcauseunknown，B3/12合同待决与全部CI5success3failure/合同13/Windowsnative/SDKmain保持独立。


## 2026-10-20 精确main交付后继

主header revision/freshness和全部原challenge保持原观察，本具名后继只绑定精SDKmain 1e1c4f03f670561d6ca804a56d35967eeea191ce。PR5已MERGED，GitHub签名verified/valid；原push CI [39394908918](https://github.com/EchoYue-lp/echo-agent-sdk/actions/runs/39394908918)确实执行该head，9个job均自然SUCCESS。

精main原dependency-policy job118535120314实际SUCCESS。原action的rust-version:stable生效，日志先安装cargo/std/rustc、将stable-x86_64-unknown-linux-musl设default，再输出rustup当前toolchain；旧未安装override错误不存在。cargoaudit正常结束，deny实际advisories/bans/licenses/sources均ok；保留49条warning[duplicate]而非宣称零warning。原始joblog1049560B/SHA2563d9ef4442cab7d59d3723834f560c96550e1edadf58f15fed04a42c6c1bec269保存在任务ignored证据。原722失败、4994部分失败和9061后验仍是其历史切点，当前通过不覆盖或抹除它们；不从bootstrap成功推历史Host因果已修。
