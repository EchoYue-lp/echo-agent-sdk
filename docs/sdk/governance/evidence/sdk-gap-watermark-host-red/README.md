# #120：Host ACK 水位推进的真实反例

同一个真实 Host E2E 场景在只删除 ACK 水位推进一行时失败，恢复该行后通过。
两次都执行 `ack_window_snapshot_ack_resumes_strictly_after_watermark`，均为
1 个测试、0 ignored。原断言、事件输入、窗口配置、Host 启动、snapshot、ACK、
replay 和 live continuation 路径相同；实验副本只增加三个 stdout 观察。

| 阶段 | snapshot 水位 | ACK 后 live 序号 | 结果 |
| --- | --- | --- | --- |
| 旧水位机制 | 2 | 2 | 原 `sequence > watermark` 断言失败；exit 101 |
| 恢复水位推进 | 2 | 3 | 原完整场景通过；后续 gap 从 4 开始；exit 0 |

真实日志：[red-host.log](red-host.log)、[green-host.log](green-host.log)。
命令、时间、退出码、输入校验和日志摘要分别见
[red-host-receipt.json](red-host-receipt.json)、
[green-host-receipt.json](green-host-receipt.json)。

## 单变量与历史关联

SDK 当前源码副本的 `DeliveryInner::acknowledge` 删除以下一行形成反例，再恢复同一行：

```rust
self.last_sent_sequence = self.last_sent_sequence.max(acked);
```

[watermark-only-regression.patch](watermark-only-regression.patch) 保存精确差异；
[events-regressed.rs](events-regressed.rs) 与 [events-fixed.rs](events-fixed.rs)
保存两份源码。固定版本 SHA-256 为
`e4ae808abb8f916a7318faec3643c5a56cd1bbd143e9e88cb447e0cf7127f7de`，
退化版本为 `2b372ce338a031cdd39723e98f1716b25a2986ccfb205709b787ad534ea04218`。

[原始 30d7e219 源码](events-30d7e219.rs) 的 `StreamDelivery::acknowledge`
在 ACK 后直接返回 `Some(inner.last_sent_sequence)`，未将接受的 snapshot ACK
推进到该水位。[Issue #120](https://github.com/EchoYue-lp/echo-agent/issues/120)
即与此缺陷对应。本实验保持当前的其他 queue、gap 授权、失败、协议和框架修复，
**证明旧水位机制的反例；它不是完整 30d7e219 SDK 的构建结果，也不是三语言旧 Host 结果。**

观察 fixture 的原始与执行版本、观察差异一并保存：
[fixture-original.rs](fixture-original.rs)、[fixture-observed.rs](fixture-observed.rs)、
[fixture-observation-only.patch](fixture-observation-only.patch)。红、绿两次执行版本均为
`2e29cbe4e5257b79171adf35b60073566d99e810b375271d10586f13a1bb2d81`。

## 可恢复输入与命令

[source-manifest.json](source-manifest.json) 逐文件固定 216 份 SDK 输入和
552 份框架构建输入。归档
[sdk-experimental-source.tar.gz](sdk-experimental-source.tar.gz) 保存退化 SDK 副本，
[framework-rust-source.tar.gz](framework-rust-source.tar.gz) 保存上游源码和编译引用的
README/测试证书；两者均不包含 target、node_modules、.venv 或 .git。
框架文档中另外 26 份历史源码副本属于非构建证据，不参与构建摘要校验，排除理由和原始
记录保留在 manifest 内。两次运行前后均校验实际 SDK/框架输入，并核对生产 SDK
`events.rs` 未变。

实验运行位置为项目内部
`.worktrees/admin/gap-watermark-red-evidence/source`，上游 `.cargo/config.toml`
指向项目内部 `channel-handler-forward` 框架工作树。共享 target 由协调线程独占分配。

```bash
cargo test -p echo-sdk-host --all-features --locked \
  --test core_profile_e2e \
  ack_window_snapshot_ack_resumes_strictly_after_watermark \
  -- --exact --nocapture
```

两次均使用 `CARGO_BUILD_JOBS=2`、`CARGO_PROFILE_DEV_DEBUG=0`、
`CARGO_PROFILE_TEST_DEBUG=0`、`CARGO_INCREMENTAL=0`；完整环境与绝对路径见运行收据。
恢复归档到新位置时，需将 `.cargo/config.toml` 的八个框架 path 指向解压后的框架目录；
这是位置迁移，不改变归档源码。先运行退化版本，再将 `events-fixed.rs` 复制到
`echo-sdk-host/src/core_profile/events.rs`，使用同一命令运行修复版本。
[prepare.py](prepare.py)、[run.py](run.py) 保存本次实际准备与执行脚本。

首次准备命令未开启 `sdk-core-profile`，执行了零个测试；第二次观察输出使用了
`WireNonZeroU64` 不存在的方法，在编译阶段失败。两份尝试的日志和收据以
`attempt-zero-tests-*`、`attempt-observation-compile-error-*` 原样保留，
明确不计入行为反例或通过证据。修正 feature 和观察输出后，上表两次才实际执行原场景。

[receipt.json](receipt.json) 汇总上述范围；[SHA256SUMS.json](SHA256SUMS.json)
固定归档全部文件。三语言一致性、最终 SDK 完整快照和精确 main 验收另需各自证据。

已调用任务范围 `semantic-verify`，正确带 `--base` 的顶层检查 exit 0，
见 [semantic-task-scope-final.log](semantic-task-scope-final.log)。该检查明确输出
`paths=[]`、`globalClosure=not_checked`：顶层忽略 `.worktrees`，所以这个结果仅核对
顶层预检依据，不能替代 SDK 证据对象关联、完整 SDK 语义检查或最终快照验收。
首次漏传 `--base` 的 CLI 用法错误也独立保留，修正后的命令和范围见
[semantic-task-scope-final-receipt.json](semantic-task-scope-final-receipt.json)。
