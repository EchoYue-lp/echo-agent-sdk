---
schema_version: 1
id: evidence.sdk-model-read-retained-repair
kind: evidence
observed_at: 208a027f587446badaaa2058e6af3240457431d2
source_refs:
- echo-sdk-host/src/config.rs#PreparedHostConfig
- echo-sdk-host/src/factory.rs#PreparedAgentDefinition
- echo-sdk-host/src/core_profile/wire.rs#framework_from_wire
- echo-sdk-host/src/core_profile/handler.rs#build_definition
- echo-sdk-host/src/core_profile/facade/source_operations.rs#session_agent_read_guard
- echo-sdk-host/src/core_profile/facade/source_operations.rs#session_context_read_guards
- echo-sdk-host/src/core_profile/extension_bridge.rs
- echo-sdk-host/Cargo.toml
- Cargo.lock
- docs/adr/0034-sdk-governance-authority-and-lossless-adaptation.md
- docs/sdk/sdk-extension-bridge.md
supports:
- behavior.sdk-family-source-operation
limitations:
- Only the four named source Findings, not the whole SourceOperation family or SDK
  acceptance.
- Framework owns configured model publication and retained transcript repair; SDK
  has no parallel model/cursor/store.
- Callback availability is owner-wide and can reject an external concurrent read;
  it does not identify request origin.
- The historical managed Linux exit timeout and final SDK gates remain independent.
---

# 模型构造、权威读取与retained依赖的具名修复来源

## 支持的结论

精SDK208a源码只修三类薄适配、消费一个框架producer依赖。默认Host和explicit wire从各自原validated LlmConfig与同一prepared Client Arc私有携到Definition，用F construction-only API在extensions前构造；不从scrubbed FrameworkConfig/Client猜凭据、不每Session重建、不在customLLM注入后补seed。public customArc installer仍opaque，live ReactAgent才是模型权威，SDK不增加mutable model cache。set_model真实Result只有success可Unit、错误沿原framework_error；Session model getter同ownedAgent，零Session definition未变。

Session Traitbase system_prompt从owned Agent读取base config，effective current仍runtime override/当前base，zeroSession definition仍原模板。不可把base映current或刷ACP缓存。busy读取得同Agent/Context guard就直接原值投影；不可取且owner有active callback时原ExtensionConflict/Never/op避免父子锁等待，否则普通并发等原锁。owner计数不是请求来源识别。Context guard持到同步消息/统计投影结束，不掉tryguard再盲await；snapshot capture按真实mutation保原arity/error优先，snapshots/latest保读义。

Retained ordinal问题唯一修复owner为已发布framework3560812，其schema3/sparse sidecar与原cursor/CAS保存恢复由上游原治理链和精main七CI接受；SDK wire/RuntimeStore继续逐messages_json String无损委托，未筛batch、改evict、关闭managed、增加DTO/Store/cursor。新Rust constructor/context_only named identities未自动进入accepted；原accepted set_model signature后继经唯一原generator，5621身份/454routes不增退。

## 来源与范围

源码为本页observed_at公开Git208a，root-source SHA为ef417474a460c287533f3b5024f275df2edb65c79d2cca62b63ac146b8bf794d。原ADR0034、两个产品文档与四Finding发现时承诺保一权威。F依赖为[精main3560812](https://github.com/EchoYue-lp/echo-agent/commit/3560812caede84998a1f6f26d7bf3fa81ea0486c)，PR193/treebbacaad与已审8138相同，CI39355472505七SUCCESS及remote-only冷strict由Root原件证明。

原SDK/source构造与两层读取候选、两处arity静态反例、相关source/runtime增量、生成/锁来源与39路径必要性/薄formal均有原独立C0/I0/M0；这里归档已完成的具名修复来源，不伪作者另跑一次业务审查。新live fixture曾E0382和Client有序handler自等，分别只改同连接clone及live两callback任务调度；原responder仍等ALL子RPC断言后回应、Err/panic显式交回，所有生产/helper/2s/selector/父断言不变。它们是测试修正，不能当Host retained生产反例或新增产品策略。

## 已知缺口

本页只作为四Finding的repair链。它不替verification原件/独立rereview，不证明全部方法、callback、Windows、自然Host退出、任意秘密类或SDKmain。原managed Linux根因未知不受此链关闭；完整SDK工程与最终main验收沿原工程Evidence独立。
