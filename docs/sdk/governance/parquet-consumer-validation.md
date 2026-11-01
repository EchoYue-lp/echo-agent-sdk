# SDK Parquet 真实消费者验证

SDK 使用现有 `Session.invoke("export_data", …)` 与 `read_data`，不直接调用 Polars，不另建数据执行器。共享夹具只生成确定输入、校验结果；TypeScript、Python、Java 各自经真实 ACP stdio Host 完成五次调用。

案例覆盖 CSV → Parquet → Host 回读 → JSON；逐值检查中文、emoji、浮点数和空值，检查真实 Parquet 容器、Zstandard 帧字节及回读结果。损坏 Parquet 必须返回类型化错误；不支持的导出格式不得覆盖已有输出，输入文件也必须逐字节保留。原始请求、结果、Host SHA-256 和输出摘要留在测试目录。

三条测试由现有 `npm test`、`uv run pytest`、`mvn test` 自动发现，沿完整语言入口及 CI 执行；设置 `ECHO_AGENT_SDK_REQUIRE_HOST_E2E=1` 时，缺少 Host 是失败。定向运行可选对应单文件或 `-Dtest=ParquetRoundtripHostTest`，临时目录必须设置到本轮唯一主根内。

这组验证只证明具名 Host 的保留数据合同与三语言消费。物理帧检查不替代框架对 Parquet codec metadata 的专项检查，更不证明 zstd 漏洞修复、新 framework 来源、独立精确 pin、完整语言矩阵、原生窗口或 main CI 合格。旧 `f7bac62` Host 的结果不能续到后继 WIP 或新 pin；原 fresh 4 漏洞、10 unsound 和 source 402 阻塞继续保留。

本次只新增消费验收，没有公开 API 或产品架构变更，因此不新增 ADR；复用 ADR0042 的保留合同边界。三个 quickstart 未增加数据 API，既有示例仍须随新 pin 单独运行；官网没有对应新增产品能力，本次不修改官网文案，由官网执行者随正式来源统一核对。
