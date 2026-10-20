---
schema_version: 1
id: audit.sdk-model-read-state-rereview
kind: audit
boundary_ref: boundary.sdk-integrations
lens: state_authority
freshness: examined
revision: 208a027f587446badaaa2058e6af3240457431d2
finding_refs:
- finding.sdk-live-model-source-authority
- finding.sdk-session-base-prompt-view
- finding.sdk-busy-session-read-authority
challenges:
  original-config-and-live-model:
    revision: 208a027f587446badaaa2058e6af3240457431d2
    source_refs:
    - echo-sdk-host/src/config.rs
    - echo-sdk-host/src/factory.rs
    - echo-sdk-host/src/core_profile/wire.rs
    - echo-sdk-host/src/core_profile/facade/source_operations.rs
    - sdks/typescript/test/memory-permission-host.test.js
    evidence_refs:
    - evidence.sdk-model-read-retained-repair
    - evidence.sdk-model-read-retained-verification
  base-current-definition-distinction:
    revision: 208a027f587446badaaa2058e6af3240457431d2
    source_refs:
    - echo-sdk-host/src/core_profile/facade/source_operations.rs
    - echo-sdk-host/tests/core_profile_e2e.rs
    evidence_refs:
    - evidence.sdk-model-read-retained-repair
    - evidence.sdk-model-read-retained-verification
  busy-original-agent-and-context:
    revision: 208a027f587446badaaa2058e6af3240457431d2
    source_refs:
    - echo-sdk-host/src/core_profile/facade/source_operations.rs
    - echo-sdk-host/tests/extension_bridge_e2e.rs
    - docs/sdk/sdk-extension-bridge.md
    evidence_refs:
    - evidence.sdk-model-read-retained-repair
    - evidence.sdk-model-read-retained-verification
  named-settlement-agent-session-run:
    revision: 04eb6ad50982656b5a8b8ea649619b6a383065ff
    source_refs:
    - echo-sdk-host/src/core_profile/handler.rs
    - echo-sdk-host/src/core_profile/handles.rs
    - echo-sdk-host/src/core_profile/state.rs
    - echo-sdk-host/src/core_profile/persistence.rs
    - echo-sdk-host/src/factory.rs
    - echo-sdk-host/src/lib.rs#manual_close_keeps_active_run_actor_until_cleanup_and_receipt_settle
    - echo-sdk-host/tests/core_profile_e2e.rs#explicit_session_create_reserves_persisted_turn_identity_before_new_run
    - sdks/python/tests/test_parallel_sessions_host.py
    evidence_refs:
    - evidence.sdk-acp-agent-run-finite-observation
    - evidence.sdk-model-read-retained-verification
  named-settlement-family-session:
    revision: 04eb6ad50982656b5a8b8ea649619b6a383065ff
    source_refs:
    - echo-sdk-host/src/core_profile/handler.rs
    - echo-sdk-host/src/core_profile/handles.rs
    - echo-sdk-host/src/core_profile/state.rs
    - echo-sdk-host/src/core_profile/persistence.rs
    - echo-sdk-host/src/factory.rs
    evidence_refs:
    - evidence.sdk-acp-agent-run-finite-observation
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
---

# 精SDK208a的具名风险复审承接

## 审查范围

只核模型原配置/Result/livegetter、base/current/definition与busy读Agent/Context权威三Finding。不新做全族审计，不把header引用恢复当行为验证。记录原独立source/runtime/必要性C0/I0/M0在当前公开Git208a上可恢复的精对象；旧Memory/Workflow及其它Finding链不扩用。

## 已检查故障假设

原prepared client丢validated config/重建丢Arc；custom injection后误补seed；set_model吞Result；Session model/base构造cache；base误映current；busy读取伪None/Vec/0；只修Agent锁却在Context自等；owner全局active被误写为请求来源；capture当只读；invalidarity变等待或错误field变。

## 实际实现路径与证据

沿本Audit具名challenge的精源、repair与verification Evidence读取。Root原source reader核冻结配置/两helper及两arity静态修正C0，当前newHost和模型/scalar/三语compressor/base/busy/core/live03原raw/inputs/compiled assertions分切片独立C0。两个fixture改动仅同连接clone/dispatch，不改生产和2s/selector/oracles，原handler响应须ALLnested assert后才发，Err/panic显式交回且final facts让fallback不能假PASS；原失败保，不把未执行邻例当执行。

configured client原Arc/cfg在extensions前仅构造，不复制runtime；base/config/current三义精分，busy读拿原guard到投影、不可等待回调按既有typed冲突，snapshot capture纳原mutation和arity错误优先。实际forward/error/读取与原before→after完全由两Evidence限定。

## 问题记录

两次arity来源/错误field静态I1、一项E0382编译、Client自等native1分别有原件及最小后继；它们的修正与复核不能重标为SDK/F原生产根因。最终本范围C0/I0/M0，支持对应Finding只在其原承诺内有界结算，不给整个SDK或其它Finding发通过资格。

## 残余风险

owner active是全局谓词；普通并发也可有冲突，无自动重试。read-only guard可取仍允许callback，不增加本地权限门；standardACP request_permission未支持，原HITL是ExtensionHumanLoop。没有宣所有锁/poison/动态模块行为或未知秘密类；managed Linux退出根因未由此解释。完整SDK工程/平台/main独立。

## 未检查项

完整SourceOperation/intrinsic所有组合、自然Host退出/retirement、任意peerHTTP/Secrets、Windows/Temurin17全部消费者、当前14Rust/唯一官方整语言及qs/依赖/semantic/nativeHook/精SDKmain CI。相关工程可有后继证据，但本页不借尚未完成的门禁为四Finding之外背书。


## 2026-10-20 具名后继的有界结算

主header revision/freshness及原挑战仍保原审查cut；这里只记录下列named单元，原17be/0bbc读审与04eb受控media增量分开，旧Evidence运行不重标。任何一个row不能推整八风险组或整composite资格。

- `behavior.sdk-agent-session-run`：有限named单元结算；证据`evidence.sdk-acp-agent-run-finite-observation`、`evidence.sdk-model-read-retained-verification`。
- `behavior.sdk-family-session`：有限named单元结算；证据`evidence.sdk-acp-agent-run-finite-observation`、`evidence.sdk-model-read-retained-verification`。
- `behavior.sdk-family-source-operation`：有限named单元结算；证据`evidence.sdk-integrations-finite-observation`、`evidence.sdk-model-read-retained-repair`、`evidence.sdk-model-read-retained-verification`。
