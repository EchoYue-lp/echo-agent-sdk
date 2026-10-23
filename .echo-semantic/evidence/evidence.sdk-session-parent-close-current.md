---
schema_version: 1
id: evidence.sdk-session-parent-close-current
kind: evidence
observed_at: 4acd5b81cf3f64b17a7aed546d4aaa2e145e52cb
source_refs:
- docs/adr/0041-session-close-local-child-scope.md
- docs/sdk/sdk-core-profile.md
- sdks/java/README.md
- sdks/java/src/main/java/com/echoagent/sdk/EchoAgentClient.java
- sdks/java/src/main/java/com/echoagent/sdk/RunHandle.java
- sdks/java/src/main/java/com/echoagent/sdk/SessionHandle.java
- sdks/java/src/test/java/com/echoagent/sdk/SessionParentCloseHostTest.java
- sdks/java/src/test/java/com/echoagent/sdk/SessionParentCloseTest.java
- sdks/python/README.md
- sdks/python/src/echo_agent_sdk/client.py
- sdks/python/tests/test_protocol_integrity.py
- sdks/python/tests/test_session_parent_close.py
- sdks/python/tests/test_session_parent_close_host.py
- sdks/typescript/README.md
- sdks/typescript/src/client.ts
- sdks/typescript/test/session-parent-close-host.test.js
- sdks/typescript/test/session-parent-close.test.js
- scripts/check-language-sdks.sh
- sdks/java/pom.xml
- echo-sdk-host/Cargo.toml
- Cargo.lock
supports:
- behavior.sdk-language-clients
limitations:
- Only the local Session child-subscription disposal change and the named regression obligations are renewed; old source/runtime epochs remain unchanged.
- The full-language command ran on macOS; its three platform-specific Windows cases were skipped and Oracle JDK 21.0.5 is not Temurin 17 runtime evidence.
- Local EOF is not a synthesized Run terminal or receipt, does not recover discarded Host notifications, and does not acknowledge consumption.
- The historical managed passive-exit cause remains UNKNOWN with its existing human risk acceptance; no causal repair or broader coverage is claimed.
- The original failing semantic job remains failed; the metadata successor requires its own precise main CI result.
---

# Session parent-close local child-scope evidence

## 支持的结论

真实 SDK main `4acd5b81cf3f64b17a7aed546d4aaa2e145e52cb`，tree
`42891bc3e8f61b0c09b4545859b44dee374d9c0e`，修复三语言的本地子订阅处置，
不改变 Host/Protocol、framework pin、accepted5621/454、wire routes 或依赖。
SDK Host 与锁内 framework 仍精确 pin
`f7bac62eee04a963f92a3e4308b6ea2336e91cc0`；相邻 framework 工作树不是本次 SDK
运行来源。

- `SessionHandle` 只在成功父关闭后处置其实际返回的每个本地 Run 对象；不能以 wire ID
  去重丢掉 idempotent alias。父关闭后晚到 start 回复同样关闭本地 scope。
- TS `closeRunScope`、Python `_mark_closed`、Java `closeFromSession` 复用原 feed/mailbox。
  已缓冲事实先读完再 EOF；TS/Java 首次迟订阅不能重建活 mailbox，Python 保留既有
  closed-handle 拒绝。没有虚构 `Completed`、`Cancelled`、receipt 或业务成功。
- 父关闭失败保留子 scope；并发 close 观察原关闭 owner，Python shield/Java dependent
  future 隔离单个观察者取消。scope disposal 结束本地 ACK/child-close 观察等待，
  已提交 RPC/ACK 仍归原完成 owner，不取消、不自动推进 ACK cursor。
- 原 `closed_handle`/`stale_handle`/`host_exited` 可释放本地不可用资源；
  `host_shutting_down` 与 settlement timeout 保留 typed error 与重试能力。

具名测试覆盖成功/失败/重试、晚到回复、首次订阅、缓冲事实、共享关闭、观察者取消、
ACK 暂停及原 RPC 继续、同 wire 本地 alias 和独立 Session 隔离。可核验锚点包括：

| 语言 | 原测试源码与具名场景 |
|---|---|
| TS | `session-parent-close.test.js` 的 `successful parent close finishes an already waiting child stream`、`first child and update subscriptions after parent close are already finite`、`parent disposal releases a child close observer while preserving the original RPC` |
| Python | `test_session_parent_close.py` 的 `test_concurrent_parent_close_observers_cannot_cancel_shared_owner`、`test_disposal_releases_ack_observer_without_cancelling_original_submission`、`test_parent_disposal_includes_all_local_aliases_of_an_idempotent_run` |
| Java | `SessionParentCloseTest` 的 `failedParentCloseRetainsChildAndAllowsRetry`、`concurrentParentClosersShareOriginalRequestEvenIfOneWaiterCancels`，该类12项实际通过 |
| 真实 Host | `session-parent-close-host.test.js`、`test_session_parent_close_host.py`、`SessionParentCloseHostTest` 使用原 pinned Host，在 live-event window 1/128 中确认本地完成、保留缓冲事实和另一 Session；Java一个测试方法内执行两个窗口 |

## 来源与范围

这份材料在源码观察后补齐 CI 所要求的有限元数据；未调用已禁用的 Echo Semantic
插件、skill、hook，也未制造 SessionStart。CI 原 action/schema 只用于只读核对。
当前源码观察并不把较早运行的 wall-clock 或 base 字段重写成 Git 提交时间。

正式命令 `bash scripts/check-language-sdks.sh` 原收据记录：
`2026-10-23T09:21:06.080Z` 开始，`09:29:39.512Z` 结束，exit 0，
264893 bytes，未截断。该唯一入口执行 TS build/tests/quickstart、Python Ruff/format/
全测试/quickstart、Java Maven全测试/quickstart，并实际读取同一原 Host。

| 原完整入口结果 | 计数与边界 |
|---|---|
| TypeScript | TAP 381 total，380 pass，0 fail/cancel，1 Windows-specific skip |
| Python | 463 passed，1 Windows-specific skip，另18 subtests passed；197文件格式通过 |
| Java | 78 fresh Surefire XML，共394 tests，393执行通过，1 Windows-specific skip，0 failure/error；实际 Oracle21.0.5、Mac OS X aarch64，pom目标release17不等于JDK17运行 |
| quickstarts | 同正式入口的三语言 quickstart 全部实际执行；最终原文 `language SDK source checks passed` |

完整 SDK 输入清单 `final-build-inputs.json` 的61项工程输入与当前4acd源码逐文件
SHA256相同（0 drift）；其中包含三语言运行源码、测试、构建与官方入口。
较早的 focused 清单 `sdk-inputs-after.json` 中17项，除三份 README 后续说明文字外，
14项与当前逐字相同；不能把该 focused 运行称为已经执行后续 README 的新epoch。
后续 README 不改变运行实现，最终61项清单则与当前全部相同。

本地原始证据存于跨仓任务目录
`.worktrees/remaining-findings-goal/{evidence,verification-sdk}/`，它们是本次执行收据，
不是本SDK的公开Git路径。记录其完整摘要供持有该任务目录者复核，不伪装成
`source_refs` 或声称只克隆SDK即可恢复这些本地日志：

| 原始收据 | SHA256 |
|---|---|
| `evidence/full-sdk-commands.json` | `5666cf90bd0bfec652dd8e449a97b180e9411b3b15034b9413543944f74bd308` |
| `evidence/sdk-full-languages.log` | `a71889ffff5f0231e2d768fe2218692a40d6fde839798411cd8c742b5919927c` |
| `verification-sdk/sdk-inputs-after.json` | `8fddfd5ad7f2d42e6ba53d1323d53bf9f5be92faff85ca49ce23d5e6b8532f18` |
| `evidence/final-build-inputs.json` | `2a7bdc9c4c152dcac14abeae69c6ea0082132dfc1aeabd832e9b286b167b0a82` |
| `evidence/completion-audit.json` | `5494674596a7df95e932a3c0d7cc84add563917e69d431df6ce4e349afb7b392` |

原 completion audit 的 `SDK-parent-child-local-scope` 记录独立 reviewer
`review_cli_215: pass`，原 red、Python alias red、focused after、真实Host观察和
full-language原件均保留；当前只是将同源具名结论绑定到公开源码，没有把汇总计数
当作代替断言的独审，也没有声称所有旧审计均在4acd重新运行。

[原 main CI39954703736](https://github.com/EchoYue-lp/echo-agent-sdk/actions/runs/39954703736)
的 semantic job `120417516242` 在 published action `c0bc306` 上自然失败：
旧材料 `b4ccdedb020a8fd5aa514f7ea7b5b59a65150a37db11a79ce3b7a38899f89b76`
不等于当前实际源码摘要
`02777d702f5c96ea9b2bb980318b7b3a0f2e1c7e698fb954d947591a864b19f1`，
且本次高风险语言修复未同时更新语义对象。后继仅补 baseline source绑定、原
language-clients Behavior 与本Evidence，不修改 CI、降低门禁或改写原失败。

## 已知缺口

本次本地运行没有执行 Linux/Windows 原生消费者，也没有在 Temurin17 上新跑完整语言。
Java release17目标编译与 Oracle21实际运行分开记录；三项平台skip不是通过。
真实Host两窗口证明这组父子资源生命周期，不证明所有外部 transport、所有关闭失败、
丢弃通知恢复或历史 managed passive-exit UNKNOWN 已因果修复。

历史 UNKNOWN 的原人类 `risk_accepted` 决定、旧 epoch、Audit主状态与全局coverage均保留。
本证据不接受新的风险、不扩合同、不改行为闭合定义；registry发布、官网部署、冷缓存
与后继精确 main CI 仍需要各自真实证据。摘要正确和同次有限对象更新不能冒称后继
semantic action 已执行通过；本会话禁用插件，因此这里只做普通文件/JSON/路径/来源检查。
