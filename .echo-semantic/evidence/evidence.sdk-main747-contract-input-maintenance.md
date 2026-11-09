---
schema_version: 1
id: evidence.sdk-main747-contract-input-maintenance
kind: evidence
observed_at: source:f3bbd9c3ae4a6e0c2132f00d1f39be88a0855eb1d8b00265936ddfb0295dfde7
source_refs:
- contracts/sdk/fixtures/extension/v1/extension-factory-instance-origin-valid.json
- echo-sdk-protocol/src/managed.rs
- sdks/python/tests/test_parallel_sessions_host.py
- sdks/python/tests/test_prompt_cache_layout_values.py
- .github/workflows/rust-ci.yml
- scripts/prepare-semantic-ci.py
- scripts/tests/test_semantic_ci_preflight.py
- .github/semantic-preflight-args.json
- docs/sdk/governance/semantic-source-provenance.md
- .gitattributes
supports:
- map.sdk-extension-bridge
- map.sdk-events-recovery
- map.sdk-product-boundary
- rule.framework-runtime-authority
limitations:
- 仅补充本次真实main747完整差异的关键合同输入和CI薄适配依据，不新增应用运行时或状态权威。
- 所有运行按各自真实cwd、输入清单及时间归属；业务源码与这些受测输入保持同字节，规则/文档和语义材料更新不冒重跑。
- 本地脚本与受控Host验证不代表Windows原生、SDK精确main CI、真实provider、Eclip GUI或发布。
---

# main747 合流的关键合同与CI输入依据

## 支持的结论

框架继续拥有任务图、执行终态和缓存分段机制。SDK 仅消费现有合同并保持薄适配；不会为本次来源更新建立第二个任务状态机或缓存权威。

| 路径 | 实际变化与依据 |
| --- | --- |
| extension-factory-instance-origin-valid.json | 工厂 agent_execute 的 task 从字符串修正为现有原生 Message 对象：role=user、content.kind=string、value=任务；不改 operation 或 origin 身份。原唯一生成/完整合同入口实际通过。 |
| managed.rs | AgentCheckpointWire 删除不再存在的 current_plan；共享任务图权威及既有版本化artifact由原框架承担，不在checkpoint保留第二份计划事实。生成schema与完整Protocol/Host验证检查实际形状。 |
| test_parallel_sessions_host.py | 消费现有 from_wire，live和replay整包相等，保留唯一Terminal/settled且在流末尾；不再错误把 final_answer 当成执行物理终态。保持 session取消/隔离和EOF断言。 |
| test_prompt_cache_layout_values.py | 增加v2完整头、严格JSON/Unicode边界、system与history边界、multipart Text/ResourceLink实际文本视图的明确正负控；保留公开 runtime_context 字段。三语言缓存语义以原框架为来源，不在SDK新增策略。 |

## 来源与范围

比较基准为真实main `747be937226ce379d0abc04e0f3ba52029da5ed1`，本件 observed_at 为原c0bc对当前业务文件集计算的实际摘要；本件和其他语义材料不在业务摘要中。

主根 `.echo-semantic/local/integrated-acceptance-20261108-r01/sdk-main-validation-r01/published-main-3413-full-r01/` 保存实际原件：

- `S2-doc-description-full-check/`：`/bin/bash scripts/check-sdk-contracts.sh` 自然0，145项合同检查和136生成物检查通过，raw SHA256 `3cd665ec516e57e742aeaa2b4eabd48ca46c5ab97dcfd7518f45cae4d9a22c8f`，实际cwd为主SDK。
- `R4-workspace-all-features-r02/`：`cargo test --workspace --all-features --locked` 自然0，592通过、0失败、0忽略，raw SHA256 `f260dced654ad4822dc9ad96920ffe701b1d87469614915cae4b5b62403eaacf`。后续只改rustdoc文字时由原生成入口更新同一schema description；不把本次命令改称在后继文档快照重跑。
- `L1-original-full-languages-r04/`：`/bin/bash scripts/check-language-sdks.sh` 自然0；实际cwd为主根内最小同字节受控语言源目录，并非主SDK目录。TS527通过/1原skip，Python601通过/18subtests/1原skip，Java400 tests/0失败/0错误/1原skip；三quickstart通过，raw SHA256 `18790f51062fa83108eac28f49ba9258396e27e68b2133f56627000af0520cd5`。受测文件与主SDK同字节，原脚本、默认预算和断言均保持。

## 已知缺口

本件仅补足同次变更的可追踪解释。历史失败、旧Task身份、删除预检、版本化历史观察和最终原c0bc strict必须各自处理；不能从这四路径依据推断整个语义模型已闭合。示例已由原完整语言入口执行；官网正式文档锚点在最终SDK/CLI主线及独立网站验证后更新，当前不冒已发布。


### 2026-11-09：冷CI的原预检回放

本次新增的prepare-semantic-ci.py及版本化原CLI argv只组织受检Git视图：记录事件真实base与准确candidate/tree，普通checkout base并squash候选；逐blob原字节/mode及index核等树，原c0bc producer自然产生正确base/root/新鲜时间，再恢复准确candidate并保留ignored原件。随后原c0bc Action照常fullstrict，原出口不被恢复成功掩盖；无业务删除时沿原Action直验。没有手写preflight字段或修外部工具/schema。

原10项编排测试-r04实际自然0；同c0bc原producer在专用合成仓的record-r02自然0，恢复C/T且原JSON artifact逐byte相同。它们只证明编排，不能当SDK全strict/冷CI。原消费者兼容测试run的YAML scalar修正只改变呈现形式，解码命令不变。原独审PASS C0/I0/M0，5文件manifest c3d77a1a70c4742a9867f221c9cc75ca032f26395f4832d7fdb5cc079f489b7f；原件在主根sdk-semantic-ci-adapter-r01。示例/产品API/官网行为不受该工程适配影响。


### 原始补丁的格式属性

提交前git diff --cached --check把历史归档unified diff的14个context空行（单个空格）报为trailing whitespace。这些字节是补丁格式，不是代码行空白，不能修剪原证据。新增.gitattributes仅精确标记该一个历史文件为不转换换行、按原始补丁数据保留空白；其余文件仍用原检查，不配置全局忽略、不改检查命令。原件与43dd Git blob逐字相同，SHA256 `d20d9f3ebab5f68f3083bcc98181a4ed3ae6d23523d521ef2510b5adcf2139db`保持。本仓CI的原strict、删除检查和所有其它门禁均保留。版本化原CLI argv仅追加该具体元数据路径的allow。
