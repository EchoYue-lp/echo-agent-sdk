# Python3.10 factory cleanup timeout mapping 候选

Status: proposed，Phase1–3只读闭合；真实生产 client.py 未修改。

原完整官方语言 gate 第三次运行 exit1，TS336全部通过和TSquickstart/Ruff195已完成；Python实际1failed414passed18subtests。Java/Pythonquickstart未达，原收据的旧Java354/allFreshfalse不是本次执行。失败测试期望原已接受 extension_timeout，禁止更改测试expected、等待、skip或PATH获得绿。

直接原因：callback `asyncio.wait_for(asyncio.shield(pending), timeout=...)` 抛出 asyncio.TimeoutError。Python3.10它不是 builtin TimeoutError，所以现client3053捕获不到，落入下游通用 Exception→extension_failed/retryable never。Python3.12中它是builtin别名，原测试通过。真实原existing单test一次3.10=1fail，3.12=1pass；小型真实wait_for+shield probe各证明异常身份差异以及原future未cancel且可后续结算。

最小候选只把client的该catch改为 `except asyncio.TimeoutError as error:`，复用同文件processclose4205/4213正确模式。保留原EchoAgentError extension_timeout、message、retryable after_delay，保deadline计算/1msfloor/shield和pendingfuture/childowner不变。这个分支不pop factory_children；只有handler实际close返回result才pop/标closed。factory construction finally仍独自完成pending；agent_close超时不取消它；release后同handle retry并duplicate无重复close，原existing测试覆盖这些权威和生命周期。不得转为builtin通用except/新增state/validator/ABI。

合同依据docs/sdk/sdk-extension-bridge.md203..210：deadline不证明construction停止；pendingconstruction/childcleanup owner保至实际完成，latechild仅close一次，失败保sameowner retry。TSclient相应超时明确extension_timeout/after_delay，Java同等待deadline用extension_timeout；不是新业务超时策略。

reviewable.patch只有1行，尚未应用真实SDK；与原source/testSHA均绑定before-source.json。需Root freshpreflight/sourceowner准入及独审后才实施。建议原exacttest在3.10和3.12同输入after0，再原完整protocol_integrity文件与本次范围相关closecallbacks覆盖；不代完整whole，Root另统一whole。没有Cargo/Maven/Host或完整脚本。

exception-type-observations.json保存本会话原观察，含一次uv --no-project在Admin无project时的真实WARN，不冒零警告repo门禁；probe文件只Admin诊断不生产。未修改客户端、测试、文档/.sem/seed。此修复只保原最低版本3.10行为一致性，不改变公共合同/支持下限。
