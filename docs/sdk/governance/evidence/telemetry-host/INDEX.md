# Telemetry 有界证据索引

当前不是完整 SDK、最终 framework pin 或 main 验收。框架生产修复已在本地任务提交后参与组合门禁，SDK 仍保留 Java 公开关闭的真实反例；不得用 status 的布尔值或 catalog done 代替下列行为证据。

| 切片 | 实际支持 | 原始入口与限制 |
| --- | --- | --- |
| 原 Host 初始化阻塞 | TS/PY/Java 各一个真实正向失败，初始化前 false、公开 init pending；OS sample 处于 meter-provider rollback shutdown | receipt.json、host-init.sample.txt；不是已返回 FrameworkError |
| subscriber 与 blocking owner 候选 | 原唯一 METRICS/RUNTIME、稳定 downcast/采样/parent、单 fmt、严格 filter、原 Host settlement owner；5隔离场景+1 utility、SDK owner5 | after/README.md、after/receipt.json；旧 7b35/0bac 源码 epoch，不冒最终 source |
| 实际 exporter 与 flush | 三语言各 console=false/true，原 Agent 两次 LLM、read_file、真实 OTLP；shutdown 返回时 calls=2、tokens=34/10、tool=1、8 ended spans/4 parent links | after/record-values.json 和原 native/；仍为旧 0bac pipeline epoch |
| endpoint 配置日志 | 合法 URI query secret 真反例 → 仅移除初始化日志 endpoint 字段 → 最后格式化源 6/0、Clippy/fmt0 | endpoint-log-repair/receipt.json；生产源52724271，新诊断Host4967；不承诺任意日志自动秘密清洗 |
| TS 最终公开关闭 | 精确执行源与完整 fixture 实体、ACK held 时 close pending、释放后同一实际 Host exit0；仅公开 sdk.close | publicclose-current-source/receipt.json；fixture d4516646、client dd822f73、dist5a9cee6a，原1pass/0fail/0skip。旧 publicclose-approved 包缺旧fixture实体，仅保历史，不作最终归档PASS |
| Python 公开关闭 | ACK held 时公开 close pending；释放后原实际 Host exit0，无 fixture EOF 或信号替代 | python-publicclose/receipt.json；1pass/0fail，旧 Host0bac；不是只依据官方 context 的静态猜测 |
| Java 公开关闭 | ACK held 时 closeAsync 在100ms内完成而真实 Host仍存活，反例保留 | java-publicclose-counterexample/receipt.json；1fail/0error/0skip，新Host4967；失败清理的SIGTERM不是结算证明。修复由专属Java transport切片负责 |
| Java 初实现复验 | 同一个原公开 API 场景在初实现b89上真1pass/0fail/0skip，ACK held时close pending，释放后同实际Host PID60113 exit0 | java-publicclose-after-initial/receipt.json；完整执行源/原XML/OTLP已冻结，不冒最终writer/deadline回归或独审完成 |
| Java 3aa 切点复验 | 原公开关闭同例真1pass/0fail/0skip，执行前后4源/10编译class相同，ACK held→close pending→释放→原Host PID68005 exit0 | java-publicclose-final/receipt.json；22项含完整执行源。其后I/O清理Minor修复由最终whole覆盖，本项不冒后续源码结果 |

## 历史与当前边界

新候选曾出现 marker 错误开启 TRACE、内部测试继承 shell WARN、测试错误类型不能使用问号转换等失败。对应原日志保留，分别按实际生产反例、环境/fixture问题和编译错误分类，不能把编译失败当行为红。

`typescript-wire-v2.log` 中 public_close 曾由夹具补 EOF，已明确撤回其验收含义；Host wire cancel/EOF 仍按准确范围记录。最终 TS 公开 close 直接由 SDK 向原 stdin 发送 EOF，不依赖此替代。

框架源码和 SDK adapter、测试/collector 以及最终发布 pin 的证据分别保存。Root 的全量组合门禁、独立复审、正式 framework 发布、SDK 精确 pin、完整合同/语言/CI/main 验收尚需单独闭合；无 npm/PyPI/Maven 发布。
