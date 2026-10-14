# 远端流终态分类保真修复

三个定向测试通过：两个 Tool 测试分别发出远端 Cancelled、Failed(extension_timeout)，各覆盖 read_only/standard/dangerous；第三个 Llm 测试发出远端 Cancelled。所有调用方 token 未取消，原 lease 最后结算为 0。测试真实注册原 extension，使用官方 ACP 双向字节流，Tool 必须经过框架 ToolManager 的 execute_tool_stream_with_context→Complete 路径；没有用 mapper 镜像替代消费路径，也没有模拟本地取消触发远端结果。

合法 descriptor 的同一组 before 测试为 0 passed / 3 failed：两种 Tool 终态都丢成 Permanent/Stop/None，LLM cancellation 变成 Failed。最初 attempt 两 Tool 尚未到 proxy，参数 schema map key 错用普通 string，属于测试夹具错误；原失败日志和收据保持，只有修成合法空 schema 后的 before-valid-descriptor 才是这两个生产反例。LLM 在第一次也已真实复现。after 使用逐字相同测试体，3 passed、0 failed、0 ignored；读取完整日志可见六个 Tool 原字段及一个 Llm terminal，caller_cancelled 均为 false。

生产 diff 只复用已有 tool_react_error/react_error：Tool Failed/Cancelled 用框架 ToolFailure::from_error 生成 Complete.failure，并根据原注册 descriptor risk 声明可能副作用；read_only 为 None，standard/dangerous 为 Possible，恢复动作由框架决定。错误消息原样保留。LLM Cancelled 恢复原 AgentCancelled 分类。没有新 wire、DTO、依赖、transport、状态权威或字符串猜测。无法对应 Rust failure 字段的 SDK opaque details 不被伪造；使用与原非流调用相同的 typed projection 和框架策略。

本包保存同文件初始/合法 before/after 源码、生产-only patch、全部原 command 日志与收据；after 源 SHA c8dc3f1263ae7a84190080211fbde2eb6e6d93dade6f7b9d24da85026355f9ac。定向三测试、scoped clippy --lib --tests -D warnings、目标 rustfmt --check 均退出 0。正式 extension bridge 文档同步一段。示例和 website 不需要新增调用，因现有公开合同形状和 operation 均未改变；只是修复其原异常承诺。

这仅是框架 a634 依赖下当前 Host adapter 的有界 Rust 修复。此前 a634 Rust14/完整语言门禁针对修复前切点，不冒充修复后全套验证；新真实 Host、三语言、完整合同与 main 验收由主线程继续。本 lane 未提交或发布。
