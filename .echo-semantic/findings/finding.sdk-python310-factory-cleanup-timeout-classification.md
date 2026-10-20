---
schema_version: 1
id: finding.sdk-python310-factory-cleanup-timeout-classification
kind: finding
type: implementation_bug
status: resolved
severity: medium
primary_focus: result_side_effect
focus:
- contract_evidence
- time_lifecycle
boundary_ref: boundary.sdk-extension-bridge
behavior_refs:
- behavior.sdk-language-clients
- behavior.sdk-extension-bridge
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-python310-factory-timeout-closure
audit_refs:
- audit.sdk-python310-factory-timeout-closure
decision_refs: []
repair_evidence_refs:
- evidence.sdk-python310-factory-timeout-closure
verification_evidence_refs:
- evidence.sdk-python310-factory-timeout-closure
rereview_audit_refs:
- audit.sdk-python310-factory-timeout-closure
discovered_at: 9dd50a08a3f33430ea7d9a1b11df3403e9e6abde
---

# Python3.10 Factory cleanup超时错误分类丢失

## 问题

第三次官方完整语言入口真实失败：TypeScript336通过且quickstart完成；Python414通过、1失败、18子测试。`test_protocol_integrity.py::test_cleanup_waits_for_same_pending_construction_and_retries` 原断言期望 `extension_timeout`，实际返回 `extension_failed`。Pythonquickstart、Java suite和Java example尚未运行；旧Java354目录数字不算本次执行。原800项执行输入前后相同，故不能把这次失败说成输入漂移。

## 触发条件与影响

现有 `_Callbacks.ext_method` 在Factory child尚在 `pending_factory_construction` 时，为accepted `agent_close` 用 `asyncio.wait_for(asyncio.shield(...))` 等待原构造settled。client.py原 `except TimeoutError` 捕获builtin类；Python3.10的asyncio TimeoutError并非该类，导致超时绕过构建 `EchoAgentError(extension_timeout, ..., after_delay)` 的分支，后继generic exception转换成 `extension_failed`。这是已支持Python3.10下真实客户端生产错误分类bug，不是新public policy或测试改expect即可通过。

本Finding只记录timeout code/retry语义分支的反例，不从这次分类错误推断child泄漏、原Future被取消、整个Factory生命周期失效、B5秒物理结算已证明失败，或所有八个Host风险视角变无效。原pending construction/shield/同owner cleanup/retry权威保持；需要后验明确回调错误类型无损、原构造记录可继续等/重试及其它错误不误吞。

## 证据

发现源码观察为真实SDK Git `9dd50a08a3f33430ea7d9a1b11df3403e9e6abde`，不冒旧平台命令重跑。原完整日志位于Admin `sdk-language-ci-repairs-retry2/full-language.log`，SHA256 `5954cd5dac9e461ed5de095cb8f2b214b22d39b03762bf9fd5fdf552f1a2c309`；原receipt和derived-execution-boundary保留为失败，未取得最终whole0。仅Admin日志尚不伪装已在Git的正式Evidence，相关数组暂空。

- `sdks/python/src/echo_agent_sdk/client.py`，Git9dd SHA256 `fc843caf105c9e755f8976c4307533a560da98607965f63a3c5d5d21446924a9`。
- `sdks/python/tests/test_protocol_integrity.py`，Git9dd SHA256 `6de7f5e8f9fdbad783b652c362de97e261c687bca21c5eb2fda9a87a5b160e60`。

## 处理记录

Root已分派Tool进行Phase1–3：同一源码/同一断言的真实Python3.10before反例、3.12对照和最小修复提案，设计独审尚待。本候选不提前写修复通过/closed，不允许把SDK最低版本改3.11，不改原expected extension_timeout，不新增public API、取消scheduler或Factory状态authority。

未来局部闭合须绑定真实已提交修复源、具名before/after验证和独立rereview；同次安装resolved仅在这些事实真齐备并达到Root明确的完整入口终态条件后进行。原三个测试support Finding仍conditional，旧Mac793通过保原切点，B3/12未决与SDKmain/registry范围不变。


## 后继具名局部闭合候选

生产修复只有 except builtin TimeoutError→asyncio.TimeoutError 一行。原fc843/test6de7在Python3.10真实失败，3.12原同case通过；probe核asyncio类区别、shield原future未取消且可后续完成。afterclient399961已b0真实Git保存，原测试断言/timeout/skip不改，Python3.10和3.12原ProtocolIntegrity完整文件各31passed0failed，不累计62。deadline/1msfloor/shield/pendingchild owner/handler/finally/successclose回收未动，extension_timeout/after_delay恢复，generic异常不假归timeout。SDKcap方案、源码/native及正式27材料均独审PASS C0/I0/M0。

第四次原官方完整入口已真实exit0：TS336/PY415+18/Java355与76份freshXML/3quickstarts；800输入前后相同，原Host0f1。执行HEAD9dd只有client399961当时未提交，之后b0保存同字节，不假9dd已含after。此前三次完整失败receipt均保：前两次分别因launcher和nonAtomicGap止于TS，PY/Java未达；第三次TS336通过，Python414通过/1失败（另18子测试通过），Java未达。Gap只是独立测试oracle：GET旧水位与稍后REPLAY不是原子equal，9dd具名before红/连续真实序号+terminal独立upperbound after已审，不新生产Finding、不默认ACK缺陷。第四whole整体证据与正式95件归档均已独立复审PASS C0/I0/M0，随后原字节提交为b315f03；本次当期观察只增该真实Git正式引用，候选仍待元材料最后refs轻核及Root授权安装。
原before正文、discovered_at和所有旧raw时间保；resolved仅Admin候选，Root整体/正式新包独审PASS和最终版本绑定后才准安装，B/12残余不关。


## 正式完整入口出处的当期复核

本次当期观察Git b315f03保存新whole正式包及各原修复源码；source refs旧focused修复链与其运行时间保持，b315不是原执行日期。原第四whole执行HEAD9dd含一未提交client399961，b0继承保存同字节；800before/after/sourceGitBindings逐byte/mode匹配，不假9dd含修后生产源。
