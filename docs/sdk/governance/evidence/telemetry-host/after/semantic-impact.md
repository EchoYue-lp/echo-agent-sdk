# 有界 semantic-diff

基准：framework Git be5；SDK Git 30d7e219 加原已授权治理差异。目标：SOURCE-SHA256.json 的 telemetry slice；未发布、未刷新整项目 snapshot。

| 变化 | 语义影响与唯一权威 | 对象与失效范围 | 验证 |
| --- | --- | --- | --- |
| 稳定 OTel layer、ConsoleFilter | 新增 Rust/Host-only composition；原 provider、ID、采样和一次性状态保留 | rule.framework-layer-ownership；SDK rule.framework-runtime-authority；旧 status-only 不能证明 exporter | 5隔离进程、真实collector、parent/trace IDs、滤器负例、单writer |
| 原 init subscriber 边界 | incompatible subscriber 明确拒绝；exporter构建失败不先安装；构建后再安装且失败回滚在原同步调用内结束 | finding.sdk-telemetry-host-init-rollback-block 的 framework 路径 | 原bcb真实挂起；新三语6正常场景及生命周期错误 |
| Host startup 与 async adapter | main一次组合stderr；init/shutdown blocking job由原settlement计数和close链拥有；RPCwaiter不持唯一JoinHandle | behavior.sdk-family-telemetry；map.sdk-integrations family-telemetry；既有Run终态权威不变 | owner5、真实held export→cancel/EOF/close→release→原Host exit |
| accepted wire/catalog | telemetry.init/status/shutdown及strict bool、默认console策略均保留；无新wire、registry或SDK scheduler | 既有accepted兼容面保留，不把新Rust函数作为语言义务 | 三语言相同输入、错误码、业务/导出副作用 |
| TS publicclose遗漏EOF | 独立薄transport lane负责原stdin EOF与既有5秒fallback，不属于framework telemetry状态 | 旧两项wire失败保留；人工EOF公用close绿色已撤回 | 另lane独立复审，最终原public_close或official全语言确认待补 |

未映射的新业务权威：无。原 telemetry Behavior/Finding 保持待最终 source、修复/验证/独审三链；本报告不直接提升模型状态或声称完整SDK完成。
