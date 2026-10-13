# 有限 Tool 值与副作用补证据预检

本轮复用现有任务 `01a12de5-2e2a-7861-9e97-b45cdb727ed3`，SDK linked worktree HEAD `30d7e219`，全部创建文件在该树中；其它参与者的已有差异保留。宿主根 LP 与实际 SDK 根不同，工具调用逐条显式 cwd。semantic-preflight `check` 对五个 NEW 路径通过。

实际搜索并读取 `facade/tools.rs` 的 `family_tools/bind_parameters/dispatch_tool`、`source_operations.rs` 的原 `persist_tool_output/read_artifact_page`、framework files schema、ShellTool 默认策略及真实 child timeout、SQL 游标校验、WebFetch/WebExtract、四研究工具固定端点，以及现三语言 ToolBehaviors/Computation 测试。

决定 extend consumer tests，复用框架 Tool schema 和输入验证器、单一文件/进程/SQL authority；SDK 只转换与 handle 路由。复用原共享 fixture 的输入文件/本地 HTTP/oracle 模式。新增测试夹具与三语言消费者，不改生产、合同生成物或框架，不把 Rust public inventory 自动扩成语言 API。

允许路径：`sdks/shared/testing/tool_value_effect_fixture.py`，`sdks/typescript/test/tool-value-effect-host.test.js`，`sdks/python/tests/test_tool_value_effect_host.py`，`sdks/java/src/test/java/com/echoagent/sdk/ToolValueEffectHostTest.java`，本目录。

验证：Ruff 两新 Python 文件；Node 单文件、Python 单文件、Maven 单完整类（由 coordinator 分配独占），真实 Host 强制模式；共享 oracle 检查 UTF8 bytes、显式字段、游标连续性、错误分类、不可变文件 hash、真实 tail 进程退出。首个差异影响闭包仅消费者与证据；全项目契约/Host/current main 验收由 coordinator 在最后统一执行。
