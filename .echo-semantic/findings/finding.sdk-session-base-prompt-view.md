---
schema_version: 1
id: finding.sdk-session-base-prompt-view
kind: finding
type: authority_conflict
status: resolved
severity: high
primary_focus: state_authority
focus:
- contract_evidence
- time_lifecycle
boundary_ref: boundary.sdk-integrations
behavior_refs:
- behavior.sdk-family-source-operation
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-integrations-finite-observation
audit_refs: []
decision_refs: []
repair_evidence_refs:
- evidence.sdk-model-read-retained-repair
verification_evidence_refs:
- evidence.sdk-model-read-retained-verification
rereview_audit_refs:
- audit.sdk-model-read-state-rereview
discovered_at: 4f8382fda176415749a625063a946b21cae15f5d
---

# Session base prompt 查询仍返回 ACP 构造缓存

## 问题

Session限定的accepted `Agent::system_prompt`查询读ACP boxed wrapper的构造缓存。原ReactAgent inherent setter已改变该Session的base，查询却返回旧A，违反live Agent唯一权威。零Session的definition快照保持A是原正确语义。本条独立于模型配置载荷、F canonical root-system位置与retained投影；原resolved Memory/Workflow三链不重标或扩用。

## 触发条件与影响

同一新Session原base A为`You are a helpful assistant.`。合法`ReactAgent::set_system_prompt(B)`返回Unit，原owned `current_system_prompt`实际返回`owned base B 原系统提示🙂`，但Session Traitbase getter仍为A。随后Trait setter写runtime override C，原current getter实际C，boxed base仍A，definition仍A。正确after必须base=B、current=C、definition=A，不能将base API映成current API来凑一致。客户端据旧base组合后继配置时会使用过时指令，具体副作用未在该只含配置调用的probe中执行。

## 证据

一项原public SDK/accepted调用probe在不可变Host `0982121cb1e99284b4a7d7e07af35037f9f22bd653528d9716d425f5609407a0`真实自然退出1；firstfailedassert expectedBaseB/actualcachedA，前置current=B断言已通过。原999字节日志SHA256 `124e2328a252353122a2ef7fd786199df20a2789a1b147296980ad7f2dbf07b3`；原scriptSHA256 `49262336a6e7420464a3b33de2fed2573d3381a1e469aa766d621195cf00e5e3`，observed SHA256 `72f0f9ad3d8caff7556511f915021aa943178478e267a6f3e93a6a41dcf8181f`。完整argv、Node22.22.0、80项source/runtime输入与零漂移、typed逐调用结果、Host hash/mtime、stdout/stderr与清理记录保在ignored local的session-system-prompt-probe，只在该本机任务可用；未复制原raw到Git。

实际Host来自原SDK8481/精F6563，4f仅为当前discovery源码谱系，不把含六份未提交测试的整树称本次执行源。Session/Agent/SDK close均resolved，fixture通过SIGTERM回收，后继ps无该probe Host；这些是本次无Run配置probe的清理事实，不推framework执行结算。override C和definition A的实际结果均已收集，但失败后的其余强断言未执行，不假称after通过。

真实源码链为SDK `source_operations.rs::dispatch_agent_context_source`的Some(system_prompt)→session_agent→F `RwLockAgentWrapper.system_prompt`缓存；F `handle.rs`只缓存name/model/system_prompt，owned Trait getter按原合同返回base config。`current_system_prompt`原已从ownedHandle读取runtime override/当前base，原core admission测试证明Traitset→currentgetter，不能覆盖本条inherentset→Traitbase查询。

## 处理记录

### 2026-10-19 原记录（历史）

保持open，修复/验证/复审三链为空。最小修复只将Session分支改为现Authority AgentHandle读取`Agent::system_prompt`并复制owned String；零Session definition及current getter原义保留，不刷ACP缓存、不捏字段、不重建prompt或增加SDK模型状态。原tools/skills/MCP/workingdir/usage等是live委托，max_iterations/permission/thinking等已ownedHandle，沿其既有具名证据承接，不展开猜测性contention或全方法矩阵。

ignored独立source分支与原core admission用例增量准备中，尚未采用或编译。后继必须用当前新Host在同一B/C/definition控制实际after，既有setter/别名/错误准入保持，独立审查与适用门禁后方可关本条；模型newF构造与F root-system producer另行结算。SDK main仍独立。


## 2026-10-20：本地具体闭合

当前状态resolved仅指本Finding的原accepted义务。公开SDK208a消费已接受framework精main356及原唯一生成，私有原cfg/Result/live权威读取和retained producer分别由repair Evidence定位；同原反例的当前Host after、完整Rust14/唯一完整三语入口与3quickstarts、fresh真实锁依赖及精确独审由verification和rereview三链承接。历史before/夹具失败、source/runtime cut及上文当时open记录保留，不降accepted承诺为deferred。

本地闭合不证明SDK精main、适用Linux/Windows CI、nativeHook/Stop或所有未列组合；managed历史Linux退出超时根因未知由其独立Finding保持open。Behavior及全库闭合标志不因本条改变。
