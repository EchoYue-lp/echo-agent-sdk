---
schema_version: 1
id: finding.sdk-default-stderr-backpressure
kind: finding
type: implementation_bug
status: resolved
severity: high
primary_focus: failure_concurrency
focus:
- time_lifecycle
- contract_evidence
boundary_ref: boundary.sdk-language-clients
behavior_refs:
- behavior.sdk-language-clients
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-source-continuity
audit_refs:
- audit.sdk-language-client-lifecycle-failure-concurrency
decision_refs: []
repair_evidence_refs:
- evidence.sdk-source-continuity
verification_evidence_refs:
- evidence.sdk-source-continuity
rereview_audit_refs:
- audit.sdk-language-client-lifecycle-failure-concurrency
discovered_at: 45914042ea7f9e5b05f5111415c52538c0b106c7
---

# 默认诊断管道无读取方，子进程可在初始化前背压阻塞

## 问题

默认启动创建无读取方的诊断管道，可在initialize前因stderr背压停止进展；协议stdout不能替代诊断消费。

## 触发条件与影响

Python SDK默认spawn委托ACP建立stderr=PIPE，却没有在initialize前安排消费者；
TypeScript默认三PIPE，仅显式onStderr才安装data listener。原子进程先写较多诊断时，
写入可因私有诊断管道无人读取而停止，initialize尚未回复；不能把后来返回的Process属性
当成调用方在初始化前排空该SDK私有管道的义务。两个独立实现同属SDK客户端默认I/O
ownership缺口，合并为本项；不归因Host业务、managed历史未知、telemetry subscriber
回滚或夹具pre-try泄漏，不重开其旧resolved Finding。

## 证据

Python19真实默认SDK+官方ACP peer：0stderr控制完成；2MiB B写入停滞、原spawn future未完成，
仅对同一原Process加sole drain后原future继续并自然0。TS21B的独立同源反例在Node22验证
noOnStderr/0reader/nonflowing、原promise pending，唯一data干预后同原child初始化、
精2MiB/EOF/自然0。TS21a01只是晚订阅EOF观测失败UNKNOWN，保原件，不当生产反例。
2MiB是本机反例输入，不是新增诊断阈值、全平台pipe容量或任意父sink的吞吐承诺。

## 处理记录

修复代码切点为source:a92b96b149d204822aa6ffdb42b512e7257293e2eb1a9511d46a169c87da578b。Python只使用现有ACP transport_kwargs(stderr=None)
继承应用fd2；TS无callback默认inherit，有onStderr保原PIPE/listener，内部stderr类型如实nullable。
stdin/stdout仍ACP，close/EOF/原清理时限及状态权威不变；不增logger、callback、后台drain任务、
依赖、公开API或第二套transport。显式重定向父stderr为pipe时，其消费者由应用持有，
SDK不承诺解除应用自己的背压；README已说明默认诊断ownership。

PY20现有生命周期模块实际10PASS（新默认0/2MiB/25B Unicode-LF-CRLF三例+原七例），
新三例真实SDK/官方ACP initialize、同原Process close自然0、原字节完整、stdout无污染，
没有额外child drain/transport override/替换child。TS22现有stdio模块16PASS（新默认三例、
原callback2MiB一例、既有close十二例），默认诊断完整进入应用fd2，显式callback2MiB精收且
父stderr空；Node官方Agent/ndJsonStream，无Python前置或手写RPC替代协议。

24另用既有五个真实Host场景完成PY/Java各一publicclose及TS三种rpc_cancel/input_eof/public_close：
每场Rust child与原Python proxy均自然0、信号为空、完整stderr已在proxy真正终止后取得，
无已确认Python fatal/BrokenPipe/Traceback；held/ACK/flush/原cancel请求身份断言保持。
所有结果保各自source/命令/feature切点，详细命令、SHA、清理与本机原件正文定位见复用Evidence。
代码、原件及五场真实收据已由Root和各只读owner独立C0/I0/M0；现有failure-concurrency Audit
只新增本项具名challenge，旧revision/其它风险单元不重标。

## 结算边界

本项三链支持本机有界resolved；原e391完整语言自然1仍保历史，当前完整三语/3quickstarts、
平台与精main等验证独立待执行。46个finite verified/4整体hold、behavior_model_closure=open、
历史managed UNKNOWN不变；不扩大accepted5621/454或把Rust inventory遥测变成外部API。
