---
schema_version: 1
id: finding.sdk-test-host-binary-whole-buffer-limit
kind: finding
type: implementation_bug
status: resolved
severity: medium
primary_focus: contract_evidence
focus:
- failure_concurrency
boundary_ref: boundary.sdk-language-clients
behavior_refs:
- behavior.sdk-language-clients
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-test-host-binary-streaming-proof-closure
audit_refs:
- audit.sdk-test-host-binary-streaming-proof-closure
decision_refs: []
repair_evidence_refs:
- evidence.sdk-test-host-binary-streaming-proof-closure
verification_evidence_refs:
- evidence.sdk-test-host-binary-streaming-proof-closure
rereview_audit_refs:
- audit.sdk-test-host-binary-streaming-proof-closure
discovered_at: 9a59943e70da06b1af231b1fb22bddc879885f28
---

# Host二进制证明全量读入超过Node单buffer限制

## 问题

Node20 Linux真实Host文件为2,959,264,016字节；取消后的原语言CI日志实际保留8条not ok，其中6条由同一个Host身份SHA证明的readFileSync/readFile全量buffer错误触发：Channel文件级入口、computation、FileAuthority、Git、Memory/Permission、RAG/Rules。错误为ERR_FS_FILE_TOO_LARGE / greater than 2GiB，发生在测试证明辅助路径；不能据此宣这些产品能力运行失败或以本地较小Host通过否认反例。

## 触发条件与影响

本地macOS官方793输入完整语言329/Python394+18/Java351与75XML实际0保留原平台/sourcecut，不能推Linux原生完整通过。本次Linux旧eebd语言job在新9a配置提交后权威终态Cancelled，取消不是成功或完整TAP终态。新运行38643983612仍live，不能预写其结果。

## 证据

原日志仅保存于Admin `/Users/ls/MyWork/code/ylp_agent_learn/lp-agent/.worktrees/admin/sdk-pr5-eebd-ci-evidence/language-cancelled.log`，166086字节，SHA256 `d4a85d86d1af20f0025d157e66dceed3b4893059c2ae67da2370f254f4dad2ca`；未制造futureGit或source:pending Evidence。发现观察Git `9a59943e70da06b1af231b1fb22bddc879885f28`只是当前冻结源码定位，原运行属于eebdCI，不冒9a执行。实际相关源码Git字节如下：

- `sdks/typescript/test/channel-handler-forward-host.test.js`，SHA256 `cf35bb797e9b57129f24b17e1b3d1b3eb88247e9a80fe6e0531acd56b97a5be5`。
- `sdks/typescript/test/computation-host.test.js`，SHA256 `2be1d98be5a168126be07e441db18f3b71435928d0e13d24ea534f56b0ec75ea`。
- `sdks/typescript/test/file-authority-identity-host.test.js`，SHA256 `a9d1878b8ca82d2f03811f9edf295a16266721d11a231839da2640e6e701c7c3`。
- `sdks/typescript/test/git-tool-host.test.js`，SHA256 `480a601890f3d57607dda72f282b72224ca606d50518e3e87e52c0e58dbf9d55`。
- `sdks/typescript/test/memory-permission-host.test.js`，SHA256 `68eadc1a375fe6ea37071edc7b90e75b07f3d6786c127be234b87d61c10e9764`。
- `sdks/typescript/test/rag-project-rules-host.test.js`，SHA256 `aa025686e377c656caaad082f05dce967878cc340e63c6c181b6c2947e70874c`。

## 处理记录

ToolValue已获授权复用三语言流式Host哈希helper并更新本域调用者，不含Telemetry或State时区lane。必须验证真实完整SHA未减字节、各原消费/错误/效果断言保持，以及大文件受限读取；当前候选没有修复或后验，保持open。

这些是SDK测试/证据夹具缺陷，复用既有language lifecycle与测试支撑边界，不新增runtime/API/源接受策略/权限或第42能力族。原B三个宽Finding与12项人选择仍独立；本Finding不得借新run尚进行、旧macOS绿或编译budget配置而自动resolved。正式repair/verification/rereview及raw归档真Git冻结后再补三链，当前引用数组为空明确未闭合。


## 后继CI状态观察（原候选之后）

原上文“38643983612仍live”是候选准备当时的状态。Root后续已确认该运行终态failure；本Finding尚未读取其新原始失败日志，不据结果名推断它与旧eebd反例同cause，也不声称修复已验证。原执行版本、发现观察和旧日志Hash不改变。


## 后继具名局部闭合候选

原Node20 2959264016B Host整buffer真实ERR_FS_FILE_TOO_LARGE的6notok保原cancelledCI。Hash28三语1MiB到EOF完整SHA/IO/资源关闭已独审，真实2147483649B sparse旧red及2049分块full05aa6d摘要、Node20/22各6、PY3.10六和Java3methods均0。原17af包不回写；后继系统Python3.9launcher Path|str导入导致第一次whole335/1，唯一futureannotations修复source0aa3已054d提交，formallauncher包真实3.9 beforeimport失败→afterimport0、实际TScomputation完整61共同calls0、PY3.10原helper6 0及独审均保各自cut。PythonSDK最低3.10没降级，系统launcher兼容不新增SDK3.9合同。b0 currentfile_sha256实际是0aa3而非原e090旧源。

第四次原官方完整入口已真实exit0：TS336/PY415+18/Java355与76份freshXML/3quickstarts；800输入前后相同，原Host0f1。执行HEAD9dd只有client399961当时未提交，之后b0保存同字节，不假9dd已含after。此前三次完整失败receipt均保：前两次分别因launcher和nonAtomicGap止于TS，PY/Java未达；第三次TS336通过，Python414通过/1失败（另18子测试通过），Java未达。Gap只是独立测试oracle：GET旧水位与稍后REPLAY不是原子equal，9dd具名before红/连续真实序号+terminal独立upperbound after已审，不新生产Finding、不默认ACK缺陷。第四whole整体证据与正式95件归档均已独立复审PASS C0/I0/M0，随后原字节提交为b315f03；本次当期观察只增该真实Git正式引用，候选仍待元材料最后refs轻核及Root授权安装。
原before正文、discovered_at和所有旧raw时间保；resolved仅Admin候选，Root整体/正式新包独审PASS和最终版本绑定后才准安装，B/12残余不关。


## 正式完整入口出处的当期复核

本次当期观察Git b315f03保存新whole正式包及各原修复源码；source refs旧focused修复链与其运行时间保持，b315不是原执行日期。原第四whole执行HEAD9dd含一未提交client399961，b0继承保存同字节；800before/after/sourceGitBindings逐byte/mode匹配，不假9dd含修后生产源。
