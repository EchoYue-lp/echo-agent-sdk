# 最终幂等修复后 Rust 门禁组合归档

这里保留三个真实阶段：original 的 1–9 退出 0，第10项 sdk-core-profile 编译因 E0422 缺 WireDuration 失败（原日志/receipt 保持）；step10-import-fix 仅移动两行 cfg import 后同一第10项通过 111 tests；continuation 原第11–14项全部退出 0。没有覆盖失败或把这三段写成一次同切点重跑。

独立 current_tool_boundary_review 已核组合 PASS C0/I0/M0：original 61 项 before/after 相同且逐项对应 Git 9edc8b48，continuation 61 项前后相同且逐项对应 Git 02da045；跨段仅 core_profile_e2e.rs 两行 import 变化。45 项生产源码、6 项配置/pin/lock、测试体不变；原 all-features import 符号集合不变，no-core-profile 不编译整个该测试文件。因此 1–9 可按这一有界等价复核复用，不能冒称都以最后 61 字节集重跑。归档时当前 61 项也逐项匹配 continuation。

workspace all-targets/all-features 为 473 passed、0 failed、0 ignored。各 profile/E2E 计数单独存 combined-receipt，不相加当 unique tests。成功阶段编译 warning/error 诊断为0；workspace 原10条负例 RPC runtime WARN 完整保留。11/12桥接步骤设置明确 RUSTFLAGS=-A linker_messages -D warnings，其余使用原 repository配置，不混淆严格标志范围。

23 个原文件（15 raw logs、3 receipts、4 input maps、1 import patch）均按原字节复制，原运行路径/时间保留。这里的新 combined-receipt 与README只组合已有证据；其归档字节/范围复审尚待主线程承接，不把源码组合审查变成当前全部 SDK 验收。框架精确 Git a634；完整三语言新782-source脚本、十二项 accepted-source 连续性、完整生成、适用CI及SDKmain交付独立结算。本归档未运行任何新 Cargo/Maven 或测试，也未改生产/模型/生成物。
