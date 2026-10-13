# Accepted源连续性：12项有界迁移提案

Status: proposed。尚未取得本提案的人的兼容选择，不是approvedADR、接受/退役授权或generator可执行锚点。

## 重要承诺与正确门禁

明确accepted contract仍是兼容权威；public inventory仅遥测。唯一generator对canonical defining source与kind精确对账，不把同名getter、rootalias、Catalogdone或无SDKconsumer当自动接受/退役。b909 published框架全28profiles已提取并验证profileinvariants，但12个旧accepted source/kind不连续，producer exit1且artifactsWritten=false。没有seed/artifact写入，也不回退成部分最后profile“全量”生成物。

事实来源是[完整缺失清单](evidence/sdk-final-framework-contract/MISSING-SOURCE-CLOSURE.md)及[原始12闭包](evidence/sdk-final-framework-contract/complete-missing-source-closure.json)。旧5622是acceptedcanonicalseed，不是全Rustparity目标。此次仅处置12条差异，不要求所有新的Rustpublicidentity三语言API。

## 已观察行为与分层

7项Security仍可由原echo_agent::security root reexport调用，定义已迁core retention；Host源码仍调用原三个functions。属于canonical source迁移，不是功能删除。

3项TaskSubagentContext从publicfield变private+readonlygetter。Hosttask_runtime实际读取run_id()/delegation_policy()/waived_dependency_ids()，当前methods.rs没有这三字段的完整TaskSubagentContext专用reverseDTO。Host分别用run_id载原snapshot、waived_dependency_ids生成依赖skip提示、delegation_policy构造DispatchRequest；三语言Task4场景证明Task行为/claim/result/取消，不能外推三个Context字段直接传CustomAgent或完整roundtrip。claim/waiver/delegation仍归frameworkcanonicalcontext绑定。

2项Snapshot旧ownerfield已被框架唯一权威取代。AgentRunSnapshot没有Serde及SDK无该field真实consumer只是可核事实，不构成自动授权退役；旧wire_value seed曾接受，仍必须人的明确兼容选择。Skill治理主能力不删，Curator-only setter现明确Err；Task/Plan主能力不删，TaskRun graph执行/Plan artifact保持，不恢复obsolete第二owner。

## 推荐一次有界兼容选择

|旧exact defining source|旧kind|拟处置|保持的能力/权威|
|---|---|---|---|
|`echo_agent::agent::snapshot::AgentRunSnapshot::skill_curator`|StructField|退役旧snapshot ownerfield；明确旧Curator-only owner不支持|skill_mutation_authority原唯一ChangeLog owner；不新语言mutation接口|
|`echo_agent::agent::snapshot::ToolRuntime::plan_state`|StructField|退役旧plan_state field；保TaskRun graph唯一authority|现revisionedTaskRun graph；Plan artifact/Todo projection，不新第二状态机|
|`echo_agent::security::SecretMatch`|Struct|一对一canonical source迁移，保现public root alias及行为|echo_core::utils::retention::SecretMatch|
|`echo_agent::security::SecretMatch::matched`|StructField|一对一canonical source迁移，保现public root alias及行为|echo_core::utils::retention::SecretMatch::matched|
|`echo_agent::security::SecretMatch::position`|StructField|一对一canonical source迁移，保现public root alias及行为|echo_core::utils::retention::SecretMatch::position|
|`echo_agent::security::SecretMatch::secret_type`|StructField|一对一canonical source迁移，保现public root alias及行为|echo_core::utils::retention::SecretMatch::secret_type|
|`echo_agent::security::contains_secrets`|Function|一对一canonical source迁移，保现public root alias及行为|echo_core::utils::retention::contains_secrets|
|`echo_agent::security::redact_secrets`|Function|一对一canonical source迁移，保现public root alias及行为|echo_core::utils::retention::redact_secrets|
|`echo_agent::security::scan_secrets`|Function|一对一canonical source迁移，保现public root alias及行为|echo_core::utils::retention::scan_secrets|
|`echo_orchestration::tasks::runtime::TaskSubagentContext::delegation_policy`|StructField|退役旧public struct_field身份；保Task行为的现Host薄映射；getter Rust/Host-only|现有 delegation_policy() 只读getter，Host薄映射，不自动作为新语言API|
|`echo_orchestration::tasks::runtime::TaskSubagentContext::run_id`|StructField|退役旧public struct_field身份；保Task行为的现Host薄映射；getter Rust/Host-only|现有 run_id() 只读getter，Host薄映射，不自动作为新语言API|
|`echo_orchestration::tasks::runtime::TaskSubagentContext::waived_dependency_ids`|StructField|退役旧public struct_field身份；保Task行为的现Host薄映射；getter Rust/Host-only|现有 waived_dependency_ids() 只读getter，Host薄映射，不自动作为新语言API|

推荐接受7个一对一source迁移并显式退役对应旧canonical身份，同时保持rootfacade兼容别名/同输出shape与既有operation。推荐显式退役另外5个旧Rustfield身份：3context原生语义保现Host getter薄映射，getter保持Rust/Host-only；完整context字段消费证据定向补；2snapshot obsoleteowner字段明确unsupported旧owner，原Skill/Plan能力保持。新的SkillMutationAuthority、TaskRuntime/Channelwrapper不自动接受成全部语言API。旧已批准TaskExecution2fields/DiagnosticCode3 supportingidentities与CuratorState.save退役分别保持原授权，不重复批准或借它们豁免这12项。

这里的“退役身份”只针对旧seed exactsource/kind；不能将整能力族延期、删字段事实或getter存在当作替代合同验证。若用户不接受该兼容处置，保持旧acceptedseed与阻写状态，重新选匹配source/适配方案；不自动复活框架obsoletefield或造第二authority。

## 验证与回滚

协议owner负责唯一exactmigrationtable/compatibility/rollback材料，根审阅完整候选后一次取得人的选择；本稿不新建第二ADR。批准后只有正式approved ADR Action/Kind/Source/Reason/Compatibility/Replacement/Rollback锚点可授权generator，显式命令重跑完整28profileupdate/check，核old/newsourcekind闭包和对应value/operation语义，Context三字段消费需定向补证。

Security三个operations保持既有publicroute/错误签名及Unicode/secretspan，值类型和字段从新canonical source继承；Task三个原生语义通过现Host薄映射及claim绑定保持；现无完整三字段专用DTO roundtrip证据，需要沿既有路径定向补，不新增wire/API；旧snapshotowner返回明确unsupported或从接受库存移出且文档明确迁移，不能默认填值。最终已发布pin、无patch独立构建、三语言matchedconsumer/CI/main另验，不用源码事实替运行证据。

回滚恢复本次前的acceptedseed和生成物、精确匹配的frameworkpin及原路由合同作为一个切片；不可只退一项seed造成其它canonicalalias或Task语义漂移。历史缺失报告/rawprofiles/阻写receipt保持原字节用于复核。

## 当前唯一状态

proposed/open，等待完整兼容候选和人的选择；没有产品决定被默认为humanconfirmed，semantic0.5没有Decision对象类型，使用Finding、Evidence与正式ADR记录事实。不存在新runtime状态权威或第二接受分类器。本文件不写生产/生成JSON，也不调整当前acceptedseed。
