---
schema_version: 1
id: evidence.sdk-state-trace-delivery-candidate
kind: evidence
observed_at: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
source_refs:
- docs/sdk/governance/state-trace-delivery-evidence.md
supports:
- behavior.sdk-family-state
- behavior.sdk-family-trace
- behavior.sdk-family-delivery
limitations:
- Focused newcases not full-language rerun or independent review.
- State scope clear is not claimed globally all-or-none; _scope_index only a projection.
- Delivery before-effect claims can supersede; only latest active authority admits effect. Strict claim_next aftereffect returnsnull.
- Native u32claimattempt requires public explicit WireU64 encoding when reusing native decimaltext; no permissiveHost parser or newDTO/remoteAPI.
- Same-instance in-memory Delivery recover does not prove crossprocess/crashdurable effects.
- Full Run comparison covers chosen typedevents/options and sanitizer; not allpossibleenum/lifecycle variants.
- Java existing SLF4J diagnostic notices retained.
- Whole stable publishedframework source/CI/main and finalsemantic review pending.
- Six actual negative requiredHost guard probes are separate from original nine positive scene execution; full final gate remains pending.
asset_refs:
- asset.sdk-historical-proof-location
---

# State、Trace、Delivery 三语言候选证据

## 支持的结论

cb9 不可变 Host 上三语言各执行三个真实场景，合计九次，零失败/跳过。State 完整 DTO 对照、scope A 清理保留 B、文件恢复 generation 与 runtime owner 索引；Trace 指定完整 DTO/options、脱敏 payload、durable 文件 SHA 和重启读取；Delivery effect 前 claim supersession、仅最新 authority 执行 effect、effect 后严格 claim_next=null、defer/reclaim、四次 stale commands 不改 state/sequence，以及同实例 recover 均有具名结果。

## 来源与范围

执行原输入在 executed-test-source，归档 SHA 已逐项验证。当前三个语言测试只加强 required Host 环境变量不得为空白的前置校验；共享 fixture 相同，合法显式 Host 的场景断言未变。current-test-source-comparison.json 保存完整 diff；不宣称空白 Host 的新前置条件已被这次运行证明。

Host SHA cb9b7fcad3946d9428cdef5a7b7258a334ee387ed34ca0612b4f40b262ef20a2。实际语言日志、raw DTO/side effects 保留；比较只归一化 fixture working_dir、独立 ledger attempt_id 和逐条验证有序的时间，其余字段不删除。限定复审的 mailbox_accepted_at/drained_at 保留并断言 null 已纳入离线比较，原6711材料不改写。

## 已知缺口

State scope clear 不宣称全局 all-or-none；scope index 为投影。Delivery 为同实例内存路径，不证明跨进程/crash durable effect。Trace 只证明指定事件/sanitizer，不证明所有 enum/retention/error I/O 组合。完整已发布框架来源、CI/main 与最终语义闭合仍待；旧 whole418 不覆盖新 Tool/Workflow 生产改动。

## Required Host 前置条件增量

后继只加强required=1时Host缺失或空白的前置拒绝，六次actual offline negative probes（两类输入×三语言）明确失败而非skip；结果/patch与formal adoption receipt保留。原九个正向场景没有被重跑后改名，业务输入与断言完全相同，强化前置条件只采用自己的增量证据。

## 本次 current_source_business_rereview

原已审SDKruntime代码来源Git569/source17cf和治理文稿可恢复Git`9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d`（source0a13）明确分开。独立reader`framework_final_review`本次实际重读原业务fault/完整字段/副作用/owner/调用方变化与14Rust/newbb4f632原after证据，消息回收UTC`2026-10-14T01:56:14.724478+00:00`，不是原执行时间。结果：**PASS0 有限原业务义务**。

AgentCheckpoint公开7字段和fixture7正确；Trace选择Run17fields与bigint total_duration_ms纳秒原timestamp，不混为17Checkpoint或大revision。Filecoldrestore、JSONL原bytes、同实例MemoryDeliveryexactclaim/supersession/defer/effects/strictafterclaimnextnull保持；scopeclear不globalallornone。

原观测marker`source:3ea544c47e331b5d29a6624f978f3595619bef39fa5985230351098d38930404`、旧run-time/host/sourcehash/raw和正文保，所有source_refs从真实9dbGitblob读取且锚点存在；现在SDKbe5pin与新测试写入会改变当前文件，不能把历史b909当前源码审升级成未来be5gate。真实9db框架pin为b909，工作树be5after另Evidence处理，不将两个仓tree混写。新增门禁sourceRef只具名Scope，没有以整count判族verified；supports/limits原保、不声明旧3ea整仓可恢复、不改unknown成无缺陷。

原义务和全文历史主张保留以审查，若旧unknown因素（TTL/三getter消费）已有新原证据，只在本段给来源区别，不抹原epoch。ChartI1/5sB历史BLOCK明确不因observed版本合法消失，12source/CI/main/globalacceptedproducer仍pending，41family/lens状态不提升。


## 历史证明存储的有界表示迁移

原id/业务正文/status/supports/limitations/命令及原观察含义不改变；本冻结父 `7f14a1b9c751b0612ceb42224f83f29d09aea46c` 中此对象完整原header/source refs可由公开证据tag恢复。只将具体archive物理依赖交当前位置资产与真实same-role consumer，全部旧义务由显式Eq/CT承接，不把存储重观察冒原业务新审或旧失败变成功。
