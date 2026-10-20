---
schema_version: 1
id: audit.sdk-retained-identity-rereview
kind: audit
boundary_ref: boundary.sdk-integrations
lens: data_durability
freshness: examined
revision: 208a027f587446badaaa2058e6af3240457431d2
finding_refs:
- finding.sdk-retained-transcript-projection-identity
challenges:
  retained-message-identity-and-opaque-schema3:
    revision: 208a027f587446badaaa2058e6af3240457431d2
    source_refs:
    - echo-sdk-host/Cargo.toml
    - echo-sdk-host/src/core_profile/wire.rs
    - echo-sdk-host/src/core_profile/extension_bridge.rs
    - echo-sdk-host/tests/extension_bridge_e2e.rs
    evidence_refs:
    - evidence.sdk-model-read-retained-repair
    - evidence.sdk-model-read-retained-verification
  named-settlement-family-source-operation:
    revision: 04eb6ad50982656b5a8b8ea649619b6a383065ff
    source_refs:
    - echo-sdk-host/src/core_profile/facade/mod.rs
    - echo-sdk-host/src/core_profile/facade/registry.rs
    - echo-sdk-host/src/core_profile/facade/source_operations.rs
    - echo-sdk-host/src/core_profile/facade/integrations.rs
    - echo-sdk-host/src/core_profile/facade/memory.rs
    - echo-sdk-host/src/core_profile/facade/workflow.rs
    - echo-sdk-host/src/core_profile/facade/state_delivery.rs
    - echo-sdk-host/src/core_profile/facade/tools.rs
    - echo-sdk-host/src/core_profile/facade/observability.rs
    evidence_refs:
    - evidence.sdk-integrations-finite-observation
    - evidence.sdk-model-read-retained-repair
    - evidence.sdk-model-read-retained-verification
  named-settlement-family-state:
    revision: 04eb6ad50982656b5a8b8ea649619b6a383065ff
    source_refs:
    - echo-sdk-host/src/core_profile/facade/mod.rs
    - echo-sdk-host/src/core_profile/facade/registry.rs
    - echo-sdk-host/src/core_profile/facade/source_operations.rs
    - echo-sdk-host/src/core_profile/facade/integrations.rs
    - echo-sdk-host/src/core_profile/facade/memory.rs
    - echo-sdk-host/src/core_profile/facade/workflow.rs
    - echo-sdk-host/src/core_profile/facade/state_delivery.rs
    - echo-sdk-host/src/core_profile/facade/tools.rs
    - echo-sdk-host/src/core_profile/facade/observability.rs
    evidence_refs:
    - evidence.sdk-state-trace-delivery-candidate
    - evidence.sdk-integrations-finite-observation
    - evidence.sdk-model-read-retained-verification
  named-settlement-task-subagent:
    revision: 04eb6ad50982656b5a8b8ea649619b6a383065ff
    source_refs:
    - echo-sdk-host/src/core_profile/facade/task.rs
    - echo-sdk-host/src/core_profile/facade/task_runtime.rs
    - echo-sdk-host/src/core_profile/facade/subagent.rs
    - echo-sdk-host/src/factory.rs
    - echo-sdk-protocol/src/methods.rs
    - echo-sdk-host/src/core_profile/facade/task_runtime.rs#claim_task
    - echo-sdk-host/src/core_profile/facade/subagent.rs#collect_result
    - echo-sdk-host/tests/core_profile_e2e.rs#task_cancel_is_exact_and_pending_sibling_continues
    evidence_refs:
    - evidence.sdk-task-memory-workflow-finite-observation
    - evidence.sdk-model-read-retained-verification
---

# 精SDK208a的具名风险复审承接

## 审查范围

只核SDK已接受retained identity持久化承诺的framework依赖与SDK无损String/原consumer。不新做全族审计，不把header引用恢复当行为验证。记录原独立source/runtime/必要性C0/I0/M0在当前公开Git208a上可恢复的精对象；旧Memory/Workflow及其它Finding链不扩用。

## 已检查故障假设

压缩保留的原U/T/R被重新赋4/5/6；SDK过滤掩重复/新cursor；sidecar named API无意accepted；JSON String丢合法schema3；旧selectorPASS无identity强oracle；把两fixture失败当生产反例；纯hash或F-only green代替SDK当前consumer。

## 实际实现路径与证据

沿本Audit具名challenge的精源、repair与verification Evidence读取。Root原source reader核冻结配置/两helper及两arity静态修正C0，当前newHost和模型/scalar/三语compressor/base/busy/core/live03原raw/inputs/compiled assertions分切片独立C0。两个fixture改动仅同连接clone/dispatch，不改生产和2s/selector/oracles，原handler响应须ALLnested assert后才发，Err/panic显式交回且final facts让fallback不能假PASS；原失败保，不把未执行邻例当执行。

Fmain356/treebbacaad/七CI与上游producer修复独立来源已经接受；SDK仅pin/无损wire String和既有store委托。当前5b6item/三唯一原fact0/1/2各一次、17schema3 String/sidecar及resume、父2Run强assert是在同Host0bbc上实际执行，Code208a没有parallelcursor/filter。生成accepted5621/454无新增，未接受newnamedidentity；原聚合digest未变不假称新签名。

## 问题记录

两次arity来源/错误field静态I1、一项E0382编译、Client自等native1分别有原件及最小后继；它们的修正与复核不能重标为SDK/F原生产根因。最终本范围C0/I0/M0，支持对应Finding只在其原承诺内有界结算，不给整个SDK或其它Finding发通过资格。

## 残余风险

owner active是全局谓词；普通并发也可有冲突，无自动重试。read-only guard可取仍允许callback，不增加本地权限门；standardACP request_permission未支持，原HITL是ExtensionHumanLoop。没有宣所有锁/poison/动态模块行为或未知秘密类；managed Linux退出根因未由此解释。完整SDK工程/平台/main独立。

## 未检查项

完整SourceOperation/intrinsic所有组合、自然Host退出/retirement、任意peerHTTP/Secrets、Windows/Temurin17全部消费者、当前14Rust/唯一官方整语言及qs/依赖/semantic/nativeHook/精SDKmain CI。相关工程可有后继证据，但本页不借尚未完成的门禁为四Finding之外背书。


## 2026-10-20 具名后继的有界结算

主header revision/freshness及原挑战仍保原审查cut；这里只记录下列named单元，原17be/0bbc读审与04eb受控media增量分开，旧Evidence运行不重标。任何一个row不能推整八风险组或整composite资格。

- `behavior.sdk-family-source-operation`：有限named单元结算；证据`evidence.sdk-integrations-finite-observation`、`evidence.sdk-model-read-retained-repair`、`evidence.sdk-model-read-retained-verification`。
- `behavior.sdk-family-state`：有限named单元结算；证据`evidence.sdk-state-trace-delivery-candidate`、`evidence.sdk-integrations-finite-observation`、`evidence.sdk-model-read-retained-verification`。
- `behavior.sdk-task-subagent`：有限named单元结算；证据`evidence.sdk-task-memory-workflow-finite-observation`、`evidence.sdk-model-read-retained-verification`。
