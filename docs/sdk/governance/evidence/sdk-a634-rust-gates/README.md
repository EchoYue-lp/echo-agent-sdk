# a634 候选 SDK Rust 门禁证据

14 个原始命令全部真实退出 0、日志未截断；workspace all-targets/all-features 测试合计 461 passed、0 failed、0 ignored。这里只归档 Rust 门禁，不代表完整合同、三语言或 SDK main 验收，也不代表全部 Rust public identity 已接受或全部 Finding 已关闭。

框架精确 Git revision 为 `a6346643bda47679a96760f56f45e41fd73b61b0`，Cargo.lock SHA256 为 `a7a331cf94a24842e0b892d44f842e5663434009c9187b4306af7df524344bb3`。SDK HEAD 仍是 `30d7e219cc7d3aef0f75a19217c82898c363f50d`；执行对象是包含任务修改的工作树候选，不把该 Git HEAD 冒充修改后源码 revision。真实输入切点由 `runtime-inputs-start.json` 的 61 项内容摘要给出；归档时逐项重读当前文件，61 项均匹配。原始记录未提供独立 after-run 文件，manifest 仅明确记录归档时实际核对事实。

原始目录是 `/Users/ls/MyWork/code/ylp_agent_learn/lp-agent/.worktrees/admin/sdk-a634-rust-gates`。14 个日志、receipt 和 before-start 输入文件全部按原字节复制，原路径、时间和命令保留在 receipt；其 SHA256 为 `a7d0f47f9283c885d116fb44b4cc46a59eb8eb68fdfcf78648197c3b86a9d3a4`。manifest 为归档元数据，不修改原始记录。

第 11、12 项桥接编译与 E2E 明确设置 `RUSTFLAGS=-A linker_messages -D warnings`；其它项的 command 未覆盖 RUSTFLAGS，沿用当时 repository `.cargo/config.toml` 的 macOS linker flag。没有把两种执行配置混称为全部显式严格 RUSTFLAGS。所有日志编译 warning/error 诊断均为 0；workspace 日志保留 10 条 ACP 发送负例错误响应的运行时 WARN，不将这些事件隐藏或计成编译警告。

本次只有文档原始证据归档；没有重新执行 Cargo、Maven、npm 或测试。其它正式门禁与最终交付由主线程单独结算。
