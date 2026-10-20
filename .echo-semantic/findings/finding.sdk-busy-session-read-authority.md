---
schema_version: 1
id: finding.sdk-busy-session-read-authority
kind: finding
type: authority_conflict
status: resolved
severity: high
primary_focus: state_authority
focus:
- failure_concurrency
- contract_evidence
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

# 忙时Session只读查询把锁不可取伪成空状态

## 问题

Session-scoped `Agent::working_dir`沿ACP boxed wrapper同步try_read，锁忙时返回None并投射为JSON null，不能区分“实际未配置目录”与“原authority此刻忙”。同一wrapper的其它集合/值默认也需按其实际authority读取路径评估，但本条只以一项真实cwd反例确认，不推全部方法或新矩阵。永久model/base prompt构造缓存另两Finding；本条不要求修改正确的framework同步Trait或增加SDK缓存。

## 触发条件与影响

已配置work cwd的同一Session，accepted `ReactAgent::load_mcp_config`持原AgentHandle写锁等待合法MCP initialize。TCP barrier收到原initialize后先不回应，此时并发cwd getter真实返回null；释放后load成功并有一原client，后继cwd仍同work。该写操作没有改变目录，故null是锁冲突被误当不存在，消费者可能据此做出错误目录决策；本probe未执行该下游副作用。

## 证据

原Host `0982121cb1e99284b4a7d7e07af35037f9f22bd653528d9716d425f5609407a0`的一项原public SDK调用自然退出1。前后cwd存在且不变的断言通过，firstfailedassert为busy getter actualnull/expectedSome。原1249字节日志SHA256 `4cf1853e3a8cc0dcafd228625c39648effd2e55e39c6e6cf59dd5aefbc826379`；probe脚本SHA256 `ed316ded7a217f2a5039b4eb67e5d1173b413cd359ba3bcdfe5cfd9130a0b304`，MCPbarrierSHA256 `05fcd9a894d064e411cb503cae14082158a6cff565444ce25650df49a0d3456d`，typed observedSHA256 `82d80327d42925eaed1030dd0f323b1a557cd95a841a257499796a900124b96c`。ignored local busy-working-dir-probe保存精argv、80输入前后零漂移、原raw、barrier与清理；这些本地原件不在Git，不把source4f整dirty树当Host构建源（Host仍8481/6563）。

TCP initialize arrival在原load持锁路径内，是确定writer已持的执行事实。300ms定时器只作为受控release的fallback，不靠任意sleep争抢锁、不以时间阈值冒状态证明。原load/getter及Session/Agent/SDK均完整await resolved；MCP原PID85902已不存，fixtureSIGTERM回收、ps无该Host。HostSHA与mtime不变，无Cargo/构建或S生产/六份未提交测试修改；不把fixture终止等价framework执行receipt。

## 处理记录

### 2026-10-19 原记录（历史）

保持open/空三链，原owner/receiver保持；可失败authority读取候选设计仍待精确独立复审，不能先当修复结果。不可把普通busy查询返回初始cwd缓存或继续假null；有效读取必须同一owned AgentHandle，保原owner/receiver校验。

不能盲改所有getter为await读锁：原accepted SkillLoadPolicy discovery/reconciliation在Handle.write_async内部await原reverse `skill_load_allows`；同Session只读回调不在已有mutation拒绝集合中，若其await新getter读锁就形成writer→callback→reader→writer环。该锁环是有界源码反证，尚未native执行，不伪报死锁红。原bridge文档只定义mutation重入冲突；修复须明确普通忙读等待与真实回调锁不可取的错误/重入策略，复用已有active-session-callback与typederror机制或其它已审原机制，不新权限闸/Store/snapshot/第二runtime。

最小原MCP before保留，after需同case等待writer释放后返回真实Some，并验证选定合法回调重入路径不挂起、不伪空。新base/model owned读取修复也需遵守同一明确的读权威与重入策略；Agent锁不是唯一层，压缩/Promoter callback持Context时get_messages再等Context也能形成父子环，此为源码反证、未native跑死锁。单数snapshot调用capture有mutation，不能按名称宣全只读安全；原方法分类必须准确。具体guard/typed不可用候选仍待设计独审/当前Host after和适用门禁，不据source-only patch关本条。


## 2026-10-20：本地具体闭合

当前状态resolved仅指本Finding的原accepted义务。公开SDK208a消费已接受framework精main356及原唯一生成，私有原cfg/Result/live权威读取和retained producer分别由repair Evidence定位；同原反例的当前Host after、完整Rust14/唯一完整三语入口与3quickstarts、fresh真实锁依赖及精确独审由verification和rereview三链承接。历史before/夹具失败、source/runtime cut及上文当时open记录保留，不降accepted承诺为deferred。

本地闭合不证明SDK精main、适用Linux/Windows CI、nativeHook/Stop或所有未列组合；managed历史Linux退出超时根因未知由其独立Finding保持open。Behavior及全库闭合标志不因本条改变。
