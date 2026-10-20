# 当前能力治理与交付账本

> **历史草稿，已被正式维护文档取代。** 本页原阶段文字及证据范围保留供追溯，不是当前事实源。当前状态以[能力图](capability-map.md)、[Finding账本](finding-ledger.md)和[有限剩余验收](capability-review-remaining.md)为准；本草稿不覆盖它们，也不新增接受或裁决。

接受范围是明确跨语言合同。41个family是索引；完成单位是可定位的行为、唯一权威、生命周期、失败处置，以及直接证据或明确开放原因。完整Rust public inventory仅遥测，不扩成所有identity三语言API。

组级主路径已建立映射，`mapped`只表示上述关系与义务已展开。family仍`needs_review`，风险lens不因测试总数改成covered；开放Finding不妨碍如实记录已审模型，也不伪称缺陷已消失。

|能力组|已审且有边界的证据|剩余具体义务|
|---|---|---|
|ACP/Agent/Session/Run|标准stdio4、同官方连接actor、规范receipt、create/load原生turn7→8、kill→Interrupted和多Session隔离具名场景|最终当前来源组合准入/关闭反例；不把局部actor pass扩到整个Agent cascade|
|Task/Subagent|正式三语言各4场景：exact pause/cancel、B/C继续、resume复用B、CAS/stale/重复execute、short/fullawait幂等、CRUD-only拒执行|完整已接受context字段对账与最后组合复审；不添加无接受依据的API|
|事件与ACK|原snapshot/pageACK、oversized/retention/prune/futureACK/sendfailure已有具名证据|新retentiongap ACK coverage与pendingreplayACK后firstoversizedgap真实反例的最后组合/复审；#120继续open|
|资源与扩展桥|stream三轮真实redgreen与完整scope独立pass，本地producer/next/cancel/terminal缺陷resolved；factory、managed与close owner分别保原输入收据|actor局部pass不代所有owner；managed尚缺的具体acceptedcaller/operation审计，不以11DTO计数证明11真实操作|
|Workflow|正式48row=47calls+1close，17/24acceptedoperations，完整checkpoint/state/lineage和11typederror共同对照|原Evidence列出的7accepted操作与共享stream合同；默认MemoryCheckpointStore不追加跨重启承诺|
|Memory/Permission|正式三语言单场景各59证据row：live Store切换、Active recall、11反向Store路由、HumanLoop上下文与真实拒写/approvedonce文件副作用|30分钟TTL实际过期未观察且无接受短TTLsetter；未接受approval_receipt不暗扩scope|
|State/Trace/Delivery|三语言各3实际场景，文件冷恢复/完整指定DTO、TraceJSONL原字节、exactclaim/effect/supersession|scope clear不宣全局all-or-none；accepted内存Delivery同实例recover不伪称跨进程durableeffect|
|MCP/Channels|原cb9四场景保历史；后继7bbd五正式场景/语言保持真正PID/resources/prompts/bytes、失败重试与ownedcallback重入，source/raw SHA逐项匹配|七个Channelownedawait和Sessionclosing pre-read准入后继新Host五场景/语言已实际通过，独立复审待归档；acceptedMessageHandler入站consumer仍有一个明确合同裁决边界|
|工具/计算/值|工具28calls+1close、testing14payloadfields的12positive/7invalid；计算旧64c7三语言各45rows/9oracles/16typederrors|新Toolvalidator/run_bound_tool、Guard/Pivot189ff后继需匹配源码证据；旧整体418不覆盖它们|

有限逐family处置见 [能力审查表](capability-review-dispositions.md)，历史与当前证据按epoch归档。局部缺陷有repair/verification/独立rereview齐备即可本地resolved，不以main尚未合并单独阻止；缺复审或新真实反例必须明确保留。

Channel唯一待裁决单位见 [入站合同边界](channel-inbound-contract-decision.md)：原已接受handle/handle_stream/reply不能由plugin注册/send/stop代替，也不能把Host→SDK reverse invoke伪作新的SDK→Host入口。未明确原承诺的consumer/API选择前保持open，不擅新增wire或退役accepted。

正式交付仍需可恢复已推送framework successor、移除临时patch后的精确SDK pin、唯一完整accepted/deferred生成/check、匹配最后源码的门禁/独立复审、适用CI与精确main验收。用户已授权framework与SDK提交、推送、PR及squash合main；门禁通过后执行，无需重复询问。官网/CLI消费者按所属仓库边界对账。npm/PyPI/Maven发布仍不在本目标。
