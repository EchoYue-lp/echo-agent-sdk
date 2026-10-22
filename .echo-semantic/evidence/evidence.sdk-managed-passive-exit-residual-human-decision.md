---
schema_version: 1
id: evidence.sdk-managed-passive-exit-residual-human-decision
kind: evidence
observed_at: a9f267f636cd4d1553a8bff335c3203ab0b8a5ac
source_refs:
- echo-sdk-host/tests/extension_bridge_e2e.rs#managed_commit_disconnect_restart_replays_same_receipt
- echo-sdk-host/src/main.rs
- echo-sdk-host/src/bounded_stdio.rs
- .echo-semantic/findings/finding.sdk-managed-restart-passive-host-exit-timeout.md
supports:
- finding.sdk-managed-restart-passive-host-exit-timeout
- behavior.sdk-extension-bridge
- behavior.sdk-family-bridge
limitations:
- Human accepted only the historical first Host passive-exit timeout cause remaining UNKNOWN; no causal repair or exit-code-zero claim.
- The original ten-second wait and subsequent new-failure gate remain unchanged; no retry, kill-as-pass or producer authority change.
- Two bridge composites share this one decision; resources and language-client accepted obligations use their own independent evidence.
- Raw historical failure and all runtime source epochs remain preserved; current regression success does not rewrite the original failure.
---

# 历史首Host被动退出超时的唯一人类残余裁决

## 支持的结论

真实用户于2026-10-22T03:28:19.660Z在主协调聊天
`01a143e5-5a92-7293-8655-f880c3f698ae`明确回复请求
`call_99aef848c7e94bae949ed3f0d3425314`：

> 接受此历史残余（推荐）

原问题为：

> 是否接受这项精确的历史残余风险：旧 CI 曾在首个 Host 的 10 秒被动退出等待处超时，原因至今未知；当前 main a9f267f6 的原 Linux 用例、九项 CI 和冷恢复均已通过。接受后仅将该 Finding 记为 risk_accepted，保留原失败和未知原因，不声称已修复、不放宽 10 秒断言；后续同类失败仍阻断验收。

这是真实人的实际答复，由Root于03:28:35Z后向本SDK sole writer转交；不是推荐项预选、Agent代答或超时默认。
仅`finding.sdk-managed-restart-passive-host-exit-timeout`改为risk_accepted，两个bridge共享本决定，不重复裁决。
原实现缺陷分类、原失败、未知原因保留，repair/verification/rereview修复三链不伪填。

## 来源与范围

历史失败为[原Linux CI run38674982406](https://github.com/EchoYue-lp/echo-agent-sdk/actions/runs/38674982406)
的job116226422816：首次Host的10秒passive wait超时；首场drive/projection已通过，第二Host尚未启动。
缺少那次Host的EOF/cleanup/最终退出现场，不能从后继通过反推出旧原因。
当前[SDK PR7](https://github.com/EchoYue-lp/echo-agent-sdk/pull/7)精main
`a9f267f636cd4d1553a8bff335c3203ab0b8a5ac`、tree01ac、soleparent09f、服务器签名valid；
[原pushCI39709057560](https://github.com/EchoYue-lp/echo-agent-sdk/actions/runs/39709057560) attempt1九job全SUCCESS。
[Linux Host原job119595762490](https://github.com/EchoYue-lp/echo-agent-sdk/actions/runs/39709057560/job/119595762490)
原日志具名case实际ok、bridge31/0fail/0ignore/0filter；10秒wait未放宽。原代码丢弃首Host ExitStatus，故只证明wait返回，不称首Host exitcode0。
only-GitHub-main+36必要refs新空仓fixed950a strict/base09f实际0只证明源码/历史引用可恢复，不代替行为执行。

## 已知缺口

后续同类新失败仍阻断验收并保现场；不得延长/移除10秒门限、kill/retry-as-pass或把transport关闭当framework receipt。
Store/commit/claim与真实owner结算权威不变。历史原因继续UNKNOWN，允许在新真实证据出现后再调查；本次不以继续无界RCA阻塞已接受SDK范围的交付。
本记录不代表任何registry发布/官网部署，也不提升未接受Rust API、任意后端或全Windows测试承诺。
