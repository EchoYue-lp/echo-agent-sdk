# RAG Unicode 字符计数消费增量：三语言真实行为已验证

Python、TypeScript、Java 各一项完整真实 Host 测试通过，分别 32 次调用、13 次 embedding POST，零失败、零 skip，比较结果完全一致。`after-receipt.json`、`comparison.json`、三个 `rag-project-rules-b909-*.log`、Java Surefire XML 和 `published-framework-raw/` 保留实际结果。

本次实际 binary SHA256 为 `ee9ff37598f4061b0cc33a79d9c58e1ad4481d6592607fb308a28b7b425fc621`，实际 framework Git revision 为已 published 的 `b9099558620f5f9693c736c85c214db14f62a8cb`，无 path patch。构建与 framework exact-main 验收分别见 `sdk-b909-candidate-build.json` 和 `framework-main-b909-acceptance.json`；framework 三个直接源文件从该精确 Git tree 取出归档在 `published-framework-source/`。SDK source contract 仍为局部候选 `3cd835...`，不据此宣称十二个 accepted-source 身份裁决、全 SDK 合同或最终 SDK/main 已通过。

原 `../rag-project-rules/` 的 76 个归档文件 SHA256 已重新检查，全部保持不变。原候选 Host 返回字节数的观察仍作为历史缺陷证据；本增量不覆盖原 raw/result、receipt 或其字节期望。

当前共享 fixture 和三语言消费者已切换为修复后的强字符语义：原四字符 `中文😀甲` 必须返回 char_count 4，三个新增单 chunk 场景分别为 70 个中文字符（210 字节，truncated=false）、199 个中文字符加 emoji（200 Unicode 标量值，truncated=false）、200 个中文字符加 emoji（201 Unicode 标量值，truncated=true，preview 精确为前 200 字符）。三个场景 chunk_size=300、overlap=0；不会改变索引或 embedding 请求。

Python 使用 len(str)，TypeScript 使用 [...str].length，Java 使用 codePointCount，各自对真实 Host 结果作独立断言；preview 使用对应的字符边界截取。共享 oracle 进一步要求原 chunk 与新增三个 chunk 的 count、preview、truncated、total_chunks 完全符合期望。解析原 JSON 数字字段，未填默认值或使用有损浮点转换。

`prepared-source-manifest.json` 保留执行前 `prepared_not_executed` 阶段的历史记录，固定四个消费源 SHA256、旧证据 manifest SHA256 和待执行范围；这些消费源与实际执行时一致。之后实际调用数从 29 增至 32，embedding POST 保持 13，执行结果单独保存于 `after-receipt.json`，未覆盖历史准备记录。

`baseline-continuity.json` 逐语言比较原 29 次调用，除了原 chunk 的 char_count 从 `[13,12,9]` 修正为 `[4,4,3]`，其它 28 行结果、输入、错误字段与副作用观察保持一致；新增三行验证 70／200／201 字符边界。归一化仅限工作根、按首次出现对应的 opaque handle ID、typed handle/native projection 和 map 关联顺序，不删除错误字段或 ordered rank/source。

Ruff check/format 和 Node syntax check 通过。三个原始 embedding server 的实际 PID 在测试后均不存在，记录于各 `server-settlement.json`。Java 仍有原依赖的 SLF4J no-provider 警告，未声称零警告。所有归档文件的 SHA256 见 `SHA256SUMS.json`。

这组证据证明字符计数修复的三语言实际消费，以及原 Session 内存索引、索引失败原数据保留、错误/owner/关闭边界、规则来源与动态 fallback 行为。它不承诺 RAG crash 持久恢复，也不声称 resolver 的直接调用证明 Agent prompt 投影。独立复审、完整 accepted-source 生成、最终 SDK 合同/main 验收仍由父任务闭合。
