---
schema_version: 1
id: audit.sdk-java-stdio-close-specific-rereview
kind: audit
boundary_ref: boundary.sdk-language-clients
lens: time_lifecycle
freshness: examined
revision: 76601b242d7c354352f3b5c7d4bf099873746b14
finding_refs:
- finding.sdk-java-stdio-process-first-close
challenges:
  exact-owned-stdio-close:
    revision: 76601b242d7c354352f3b5c7d4bf099873746b14
    source_refs:
    - sdks/java/src/main/java/com/echoagent/sdk/EchoAgentClient.java
    - docs/sdk/governance/evidence/sdk-java-host-graceful-close/final-cleanup-candidate.patch
    - docs/sdk/governance/evidence/sdk-java-host-graceful-close/receipt.json
    - docs/sdk/governance/evidence/sdk-java-host-graceful-close/final-cleanup-source.json
    - docs/sdk/governance/evidence/sdk-java-host-graceful-close/final-cleanup-and-start-run.json
    - docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/receipt.json
    - docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/manifest.json
    - docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/inputs-before.json
    - docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/inputs-after.json
    - docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/java-surefire/TEST-com.echoagent.sdk.TelemetryHostTest.xml
    - docs/sdk/governance/evidence/sdk-java-host-graceful-close/independent-review.json
    evidence_refs:
    - evidence.sdk-java-stdio-close-specific-closure
---

# java 关闭资源owner具体复审

## 审查范围

这是本goal独立reader真实source审查结论的回收，最终源SHA 86efd3959d478723949910ff97058f26a6fd32b3e050b54713f83288a2ca6131；不是从8lens PASS反推该Bug，也没有重新运行源码审查。docs/sdk/governance/evidence/sdk-java-host-graceful-close/independent-review.json

## 已检查故障假设

acceptedwritertail -> originalstdinEOF -> sameProcessexit + native5sforcedfailure/retry + drainerrealrelease; previousearlyexitfindingseparate。EOF/kill/waiter cancel不冒充原child实退；失败重试保原owner，晚到IO失败不成功覆盖。具体counterexamples不被旧wholePASS隐藏。

## 实际实现路径与证据

原before/patch/source/focused与本次whole790 exact-source原收据均列source_refs；独立具体复审与新增8lens实际审查都是PASS C0/I0/M0。共享全语言中实际publicclose原XML和process事实只证明原具名场景。

## 问题记录

此specific源修复具repair/verification/rereview三链，可本地resolved，不等SDK main。其它框架Task/Telemetry权威不改变；原早期夹具/sourceepoch限制保留。

## 残余风险

12source/kind选择、5秒物理结算、完整accepted生成、平台与main单独。此Bug不因全goal未结束而假继续开放，也不因本地闭合宣全部能力verified。

## 未检查项

scope外任意transport/孙进程清理，以及框架Task物理结算政策；不新增wire和publicgetter。
