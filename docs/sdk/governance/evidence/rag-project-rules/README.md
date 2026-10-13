# RAG 与 ProjectRules 的原接受能力证据

三语言各自通过官方客户端执行同一套 29 次真实 Host 调用（共 87 次），每个消费者实际向 loopback embedding 服务发送 13 次 HTTP POST。三项测试均通过、无 skip，结果、错误、ordered chunk/rank、来源与规则内容比较一致，见 `receipt.json`、`comparison.json` 和 `raw/{python,typescript,java}/results.json`。

Host 为 SHA256 `0435e0b57fb26184dc98fe69f9d071826bee1c597e4182a89feead4dfef9f802` 的冻结候选，编译 source contract 为 `3cd835dbf40abf1d0aec04e02f3b51411631fcaa50ba7e5f0f2d2d2f500ed9a6`。原始编译 manifest 保留在 `candidate-source-build-manifest.json`。这不是 published pin 或最终 main 验收证据。

## 行为和唯一权威

- RAG：Host 原 `RagToolSet` 将框架 `RagIndexTool` / `RagSearchTool` 绑定到同一 Session 的 `RagStore` 与 `HttpEmbedder`。真实 HTTP 请求中 `model`、Unicode `input`、endpoint 路径被逐项检查，fixture 只替换外部 embedding 服务，绝不代替 Host 或客户端。
- 三个 Unicode chunk 的 content、source、chunk_index、total_chunks、rank 与 similarity_pct 精确核对。另一 Session 和关闭后的新 Session 搜索为空；关闭旧 Session 后继续使用旧 handle 返回 `closed_handle`。
- 第二个 chunk embedding 返回 503、第一条返回坏 JSON、第一条返回 empty data，均得到原 `ToolResult.success=false`、完整 error 和 `failure={permanent,stop,none}`。失败后搜索仍精确返回原三个 chunk，证明批次中部分成功 embedding 未提前提交索引。
- 非法 overlap 返回原 `framework_error`，空 embedding model 在独立 Host 进程中返回原 `invalid_config`；跨 Session 使用 resolver 返回 `invalid_value`。错误 code、message、operation、retryability、details 和 handle/generation 均保留并比较。
- ProjectRules：source constructor 接收 Agent receiver、Session owner 和显式 Path DTO；后续 root / agents_only / resolve 都调用原 `InstructionResolver`。同一根目录到 leaf 的 sources(path,kind,precedence)、注释化 content、project_root Option 与 family resolve/load 原 DTO 精确核对。每目录仅选择一个文件，AGENTS override 优先；agents_only 排除 CLAUDE。
- 同一 resolver 再次 resolve 能看到真实文件编辑。坏 UTF-8 override fallback 到 AGENTS.md，AGENTS.md 变为目录后跳过；另一 Session 的其它根规则独立，全无规则的 family resolve 返回空、load 返回 Option none。原接口 infallible skip/fallback 未被改造为新 IOError 合同。

## 影响与有限结论

本切片只新增共享输入/独立 oracle、三个语言的真实消费者测试及证据，没有修改生产、Protocol、已接受 API 或状态权威。直接影响 `.echo-semantic/maps/map.sdk-integrations.md` 的 rag/project-rules 消费证据（语义 owner 在最终 source cut 时统一绑定）。父任务基准为 `30d7e219cc7d3aef0f75a19217c82898c363f50d`，工作树差异尚未交付。

RAG 是 Session 内存索引，这组证据没有承诺跨 crash 的持久恢复。规则部分证明 resolver 原方法及 family load/resolve 消费，没有把这些调用宣称为 Agent prompt 的实际投影证据。比较仅归一化独立工作根、首次使用顺序对应的 opaque handle ID、typed handle wrapper/native projection 和 map association 顺序；错误字段、generation、ordered source/rank 与 UTF-8 内容不被删除。

**已观察到的未解决缺口：** `rag_chunk_document` 的 `char_count` 在原 framework 使用 `chunk.len()`，实际是 UTF-8 字节数；`中文😀甲` 四个 Unicode 标量值返回 13。`truncated` 同样按字节长度判断，而 preview 采用字符截断。这是三语言共同继承的框架字段语义问题，已向根任务报告。当前 oracle明确保留实际字节字段，以防虚报正确字符数；这组绿色测试不关闭该问题。框架修复后应将该期望改为字符数，并补大于 200 字节但不超过 200 字符的反例。

## 验证与失败记录

Python 用 `uv run python -m pytest tests/test_rag_project_rules_host.py -q -s`，TypeScript 用 `node --test test/rag-project-rules-host.test.js`，Java 用 `mvn -q -Dtest=RagProjectRulesHostTest test`。均设置 `ECHO_AGENT_SDK_REQUIRE_HOST_E2E=1` 与上面精确 binary，runtime 根在 lp-agent 的 `.worktrees/admin/rag-project-rules`；缺少 Host 不能 skip 通过。

`rag-project-rules-python-final.log`、`rag-project-rules-typescript-final.log`、`rag-project-rules-java-final.log` 保存通过输出。Python/TypeScript/Java 的首次错误真实保留为 attempt 日志：工具 positional schema 顺序、Agent receiver、Path DTO、typed record envelope 和 Java handle 重入的 fixture 错误均已修正，没有绕过测试。Java既有 SLF4J provider 警告保留，未额外增加 logger 依赖或声称零警告。

两个新增 Python 文件的 Ruff check 和 format check 均通过。任务范围 semantic-verify 已实际运行；`rag-project-rules-semantic-task.log` 当前报告共享工作树还有未绑定的高风险差异（包括本组新 fixture/test）。该失败交给最终模型 owner 补齐引用并重跑，不能宣称全任务语义门禁已绿。全量语言 gate、独立复审、最终 published pin/main 验收由父任务继续完成。

所有归档文件 SHA256 见 `SHA256SUMS.json`，sources 保存消费者、共享 fixture、合同、直接 Host 依据；原始 HTTP 日志不记录授权 header/key，三个 embedding server 的实际退出状态见 `server-settlement.json`。
