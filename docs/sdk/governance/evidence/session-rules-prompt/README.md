# Session cwd 规则进入真实模型请求

本包证明默认 Session cwd 的规则实际进入模型 system 消息。三语言各一个正常原生 SDK 测试，各运行三个 Session/Run，共九次真实 loopback HTTP 模型请求；三行归一化结果完全一致，零失败、零跳过。Host 为已发布 framework `be5f0a8` 的不可变构建，具体 SHA 与消费源见 [receipt.json](receipt.json)。独立审查状态以任务评审结论为准，本包不替代全 SDK 门禁或 main 验收。

| 场景 | 实际模型请求与结果 |
| --- | --- |
| A1 | cwd=A/nested，system 有 A 根 AGENTS 与 nested CLAUDE V1，无 B 规则；custom system 与原 user 保留 |
| B1 | cwd=B/nested，system 有 B 根/nested 规则，无 A 规则；custom system 与原 user 保留 |
| A2 | 修改 A/nested/CLAUDE 后新建 Session，system 有 V2、无 V1/无 B；返回对应 V2 响应 |

测试只通过正常 `createAgent → createSession(cwd) → startRun → wait/outcome/replay` 消费能力，不调用 rules.resolve，不向消息添加规则。独立 HTTP 服务保存 Host 实际 POST 的 `messages`，检查 system 的必含/排除项与 user 原文；只有检查成功才返回 `RULES_OBSERVED_*`。共享校验器再次从原 POST 检查字段，并严格核对终态 `final_answer.data` 的字符串类型和值，而非仅检查请求次数。`rules-after` 保存运行后规则文件，A 的 V1 原文见共享夹具/场景原件。

[comparison.json](comparison.json) 保存三语言相同结果；各语言目录保存原请求、原生 replay、规则、配置和日志。Java XML 只对应 `SessionRulesPromptHostTest` 一个测试；同一 Maven 命令还运行另一个文件权威测试，不能把该另一个测试算作本包规则场景。`executed-consumer-sources` 为四份真实执行源取证副本，不是第二套运行时。

最初启用 `enable_tools=false` 被 Host 启动合同拒绝；随后校验器遇到 Python 3.9 不支持 `zip(strict=...)` 及 TS wrapper/Python envelope 公共结果形状差异。这些夹具错误日志保留在 `fixture-failure-history`，没有冒充生产反例。最终用有效 Host 配置和明确公共形状再次运行三语言，所有原 system/user、隔离和终态断言保留。

本包只要求新 Session A2 读取新文件，不承诺旧 Session 热加载；不新增 `auto_project_rules` setter、wire 操作、调度器或状态权威。生产代码、接受合同和示例没有变更；示例/网站不需要因补证新增 API 说明。现有 12 项合同身份与物理结算选择仍独立开放，不由该测试默认裁决。
