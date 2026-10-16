---
schema_version: 1
id: audit.sdk-cancellation-observation-rereview
kind: audit
boundary_ref: boundary.sdk-resources-streams
lens: failure_concurrency
freshness: examined
revision: source:63bd9a4fce3c7177293460c887500eb369d93fc26d2e51820d7b2b88c58d5c43
finding_refs:
- finding.sdk-cleanup-retry-owner
challenges:
  full-facade-queue-cancellation-must-drain-original-source:
    source_refs:
    - echo-sdk-host/src/core_profile/facade/mod.rs
    - echo-sdk-host/src/core_profile/extension_bridge.rs
    evidence_refs:
    - evidence.sdk-cancellation-observation
  original-callback-binding-and-response-projection:
    source_refs:
    - echo-sdk-host/src/core_profile/extension_bridge.rs
    - echo-sdk-host/src/core_profile/facade/stream.rs
    evidence_refs:
    - evidence.sdk-cancellation-observation
---

# callback-bound / final-response 局部复审

## 审查范围

仅原callback绑定、collector与callback闭合顺序、unknown结果投影和普通final error回收。不是整个A/B runtime或SDK交付审查。

## 已检查故障假设

observer先unwind导致配额提前回收；错误ACK只保owner却让source终止；通用diagnostic覆盖合法foreign identity；普通final Error反而永远持有配额；EOF后的清理被误算成首次错误可见前释放。

## 实际实现路径与证据

原ACP调用构造owner并在唯一发送前绑定，collector同锁合并原事实。独立reader核两个实际before、六份完整after runner/raw/hash、1209项输入零漂移、不可变二进制及当前两源文件，最终局部pass、行动项0。主代理随后同不可变binary补跑原unwind完整selector1pass；对应来源与限制见evidence.sdk-cancellation-observation。

后继reader整体源码审发现满队列发送不观察cancel，主代理先取得真实两chunk目标red，单独保留一chunk控制oracle错误，再最窄select修复。reader亲核before/after、无额外pull、原callback finished/lease0后才验close/handle/配额，增量pass、行动项0。最终配置构建和14a来源由独立reader及协调者另核pass；新共同cut的drain2/unwind1/direct1与三语late各1实际记录由各Evidence分别承载，不能用配置build替代这些运行事实。

## 问题记录

两项Important已修：错误ACK仍投影普通terminal；修复候选覆盖已有typed/raw合法observation。两项Minor已修：精匹配run/attempt；资源释放断言放在首次Err之后、EOF之前。未经后继独审覆盖的范围不继承本pass。

## 残余风险

宽Finding保持open，缺合法身份或原物理回执时可继续保留资源。已保存profile的Host已实际build0/零warning，SHA8bf4a1d6deb647e608153c807ca07785dd31f7d3223a88cc3daf9b835b8bff5c，EH15489244/unwind_info2979504，必要unwind定向1pass；这不是本局部独审或完整工程门禁的替代。

后继配置表示与队列drain修复已构建新Host14a0cb58c44bdb84990262f1324e1a2513513a8b40fc0640a9d36f91f95e27e9，零警告/EH15488572/unwind_info2979480，构建输入与C2对应；这属于主代理实际构建证据，不偷换原局部reader的审查cut。新配置定向和native影响闭包仍独立验收。

再后继C2整体增量reader已独立核新配置cut定向4项及新14a三语late-payload各1的实际原件，pass、行动项0；旧“仍待”的阶段描述不再作为当前状态。宽cleanup及其它原source未知/生命周期、完整平台/SDK/main仍在未检查范围内。

## 未检查项

全部A/B实现、Java native close、任意Graph晚到回执、跨进程持久恢复、完整三语言/平台/精确main CI、宿主nativeHook/Stop成功。
