---
schema_version: 1
id: audit.sdk-test-host-binary-streaming-proof-closure
kind: audit
boundary_ref: boundary.sdk-language-clients
lens: contract_evidence
freshness: examined
revision: b315f03bf69365762951f8b83348117c2247c213
finding_refs:
- finding.sdk-test-host-binary-whole-buffer-limit
challenges:
  specific-original-fault-and-after:
    revision: b315f03bf69365762951f8b83348117c2247c213
    source_refs:
    - sdks/shared/testing/file-sha256.mjs
    - sdks/shared/testing/file_sha256.py
    - sdks/java/src/test/java/com/echoagent/sdk/FileSha256.java
    - sdks/typescript/test/file-sha256.test.js
    - sdks/shared/testing/test_file_sha256.py
    - sdks/java/src/test/java/com/echoagent/sdk/FileSha256Test.java
    - docs/sdk/governance/evidence/host-binary-streaming-sha/README.md
    - docs/sdk/governance/evidence/host-binary-streaming-sha/source-freeze.json
    - docs/sdk/governance/evidence/host-binary-streaming-sha/SHA256SUMS
    - docs/sdk/governance/evidence/host-binary-streaming-sha/node20-sparse-run.json
    - docs/sdk/governance/evidence/host-binary-streaming-sha/python310-final-tests-run.json
    - docs/sdk/governance/evidence/host-binary-streaming-sha/java/file-sha256-java-run.json
    - docs/sdk/governance/evidence/host-sha-python-launcher-compat/README.md
    - docs/sdk/governance/evidence/host-sha-python-launcher-compat/source-and-result-receipt.json
    - docs/sdk/governance/evidence/host-sha-python-launcher-compat/file_sha256-after.py
    - docs/sdk/governance/evidence/host-sha-python-launcher-compat/after-import.json
    - docs/sdk/governance/evidence/host-sha-python-launcher-compat/computation-after-run.json
    - docs/sdk/governance/evidence/host-sha-python-launcher-compat/python310-after-run.json
    - docs/sdk/governance/evidence/host-sha-python-launcher-compat/SHA256SUMS
    - docs/sdk/governance/evidence/sdk-language-ci-repairs-final/README.md
    - docs/sdk/governance/evidence/sdk-language-ci-repairs-final/receipt.json
    - docs/sdk/governance/evidence/sdk-language-ci-repairs-final/source-before.json
    - docs/sdk/governance/evidence/sdk-language-ci-repairs-final/source-after.json
    - docs/sdk/governance/evidence/sdk-language-ci-repairs-final/exact-git-source-binding.json
    - docs/sdk/governance/evidence/sdk-language-ci-repairs-final/committed-800-source-binding.json
    - docs/sdk/governance/evidence/sdk-language-ci-repairs-final/java-xml-freshness.json
    - docs/sdk/governance/evidence/sdk-language-ci-repairs-final/SHA256SUMS.json
    evidence_refs:
    - evidence.sdk-test-host-binary-streaming-proof-closure
---

# audit.sdk-test-host-binary-streaming-proof-closure

## 审查范围

回收原独立source/native与材料审查，模型作者不是新源码审者。

## 已检查故障假设

原失败是否被小Host/平台/filteredtests掩盖；exactafter源是否真实Git；错误/owner/完整DTO强断言是否被放宽；focused与whole时点是否混写。

## 实际实现路径与证据

原Node20 2959264016B Host整buffer真实ERR_FS_FILE_TOO_LARGE的6notok保原cancelledCI。Hash28三语1MiB到EOF完整SHA/IO/资源关闭已独审，真实2147483649B sparse旧red及2049分块full05aa6d摘要、Node20/22各6、PY3.10六和Java3methods均0。原17af包不回写；后继系统Python3.9launcher Path|str导入导致第一次whole335/1，唯一futureannotations修复source0aa3已054d提交，formallauncher包真实3.9 beforeimport失败→afterimport0、实际TScomputation完整61共同calls0、PY3.10原helper6 0及独审均保各自cut。PythonSDK最低3.10没降级，系统launcher兼容不新增SDK3.9合同。b0 currentfile_sha256实际是0aa3而非原e090旧源。

## 问题记录

原缺陷before/raw与失败候选原样保；源与证据保存Git b0不是原运行time。

## 残余风险

第四次原官方完整入口已真实exit0：TS336/PY415+18/Java355与76份freshXML/3quickstarts；800输入前后相同，原Host0f1。执行HEAD9dd只有client399961当时未提交，之后b0保存同字节，不假9dd已含after。此前三次完整失败receipt均保：前两次分别因launcher和nonAtomicGap止于TS，PY/Java未达；第三次TS336通过，Python414通过/1失败（另18子测试通过），Java未达。Gap只是独立测试oracle：GET旧水位与稍后REPLAY不是原子equal，9dd具名before红/连续真实序号+terminal独立upperbound after已审，不新生产Finding、不默认ACK缺陷。第四whole整体证据与正式95件归档均已独立复审PASS C0/I0/M0，随后原字节提交为b315f03；本次当期观察只增该真实Git正式引用，候选仍待元材料最后refs轻核及Root授权安装。

## 未检查项

不证明任意新API/新源接受、全8Host所有fault、全平台或SDKmain。


## 正式完整入口出处的当期复核

本次当期观察Git b315f03保存新whole正式包及各原修复源码；source refs旧focused修复链与其运行时间保持，b315不是原执行日期。原第四whole执行HEAD9dd含一未提交client399961，b0继承保存同字节；800before/after/sourceGitBindings逐byte/mode匹配，不假9dd含修后生产源。
