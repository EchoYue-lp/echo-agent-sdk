---
schema_version: 1
id: evidence.sdk-tool-input-validation-candidate
kind: evidence
observed_at: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
source_refs:
- echo-sdk-host/src/core_profile/facade/tools.rs#validate_tool_input
supports:
- behavior.sdk-family-chart
- behavior.sdk-integrations
limitations:
- Actual prototype invokes existing normal API, not yet formal malformed-input regression suite.
- Current fullRust418 proves newcanonical source only at recorded command; latestclippy/full3language/publishedframeworkhelpergate pending.
- Framework helper remains Rust/Host-only; no crosslanguagepublichelperacceptance implied.
asset_refs:
- asset.sdk-historical-proof-location
---

# Direct Tool 输入校验候选反例

## 支持的结论

旧 SDK direct Tool adapter 绑定参数后直接execute，绕过ToolManager使用的framework schema/custom validator。真实chart普通API传入not_a_chart_type得到success=true及line图，掩盖非法enum。修复后同输入返回framework_error及canonical Tool InvalidParameter；合法bar仍精确生成bar。这个前后反例为真实小prototype调用，不伪称已加入正式回归套件。

## 来源与范围

新Host SHA c52da2d346d8848074b646e5442ecc83890cd68991f08287780280f871b59f89。SDK tools.rs SHA e3224c4ed670a639585a99af09c1f1902c61431a832507a4b835d4dac0b2f70a；复用framework既有validate_tool_input，仅提升Rust可调用visibility及doc，SDK不复制validator。参数绑定拒额外arity，包括额外null。新相同workspacealltargets418测试零failure/ignored；第一次错误re-exportmanager path编译失败和正确tools::execution结果均原字节归档。

## 已知缺口

非法参数须补正式三语言测试及其错误/副作用断言；最新完整Clippy、全语言和framework新publichelper完整门禁仍待自己的来源。旧6711完整结果不能覆盖新validator；不接受该helper成为三语言远程API，不关闭Finding或能力状态。

## 本次 current_source_business_rereview

原已审SDKruntime代码来源Git569/source17cf和治理文稿可恢复Git`9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d`（source0a13）明确分开。独立reader`current_tool_boundary_review`本次实际重读原业务fault/完整字段/副作用/owner/调用方变化与14Rust/newbb4f632原after证据，消息回收UTC`2026-10-14T01:56:14.724478+00:00`，不是原执行时间。结果：**schema/arity/enum/custom-beforeExecute PASS；b909Chart整数合同I1 BLOCK**。

framework唯一validate_tool_input无SDKvalidatorcopy，missing/type/enum/excessnullcustombad executes0/valid1强断言。非法enum原falseLine修已通过，不能推出integerbounds已正确。新b909Chartwidthbug保原无损义务，helperRustHostonly不自动publicAPI。

原观测marker`source:3ea544c47e331b5d29a6624f978f3595619bef39fa5985230351098d38930404`、旧run-time/host/sourcehash/raw和正文保，所有source_refs从真实9dbGitblob读取且锚点存在；现在SDKbe5pin与新测试写入会改变当前文件，不能把历史b909当前源码审升级成未来be5gate。真实9db框架pin为b909，工作树be5after另Evidence处理，不将两个仓tree混写。新增门禁sourceRef只具名Scope，没有以整count判族verified；supports/limits原保、不声明旧3ea整仓可恢复、不改unknown成无缺陷。

原义务和全文历史主张保留以审查，若旧unknown因素（TTL/三getter消费）已有新原证据，只在本段给来源区别，不抹原epoch。ChartI1/5sB历史BLOCK明确不因observed版本合法消失，12source/CI/main/globalacceptedproducer仍pending，41family/lens状态不提升。


## 历史证明存储的有界表示迁移

原id/业务正文/status/supports/limitations/命令及原观察含义不改变；本冻结父 `7f14a1b9c751b0612ceb42224f83f29d09aea46c` 中此对象完整原header/source refs可由公开证据tag恢复。只将具体archive物理依赖交当前位置资产与真实same-role consumer，全部旧义务由显式Eq/CT承接，不把存储重观察冒原业务新审或旧失败变成功。
