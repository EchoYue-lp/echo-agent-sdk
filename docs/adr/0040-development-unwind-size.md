# ADR 0040: Bound development unwind metadata with optimization

- Date: 2026-10-17

## Status

Accepted

## Context

The F988 all-facade Host's unoptimized development binary exceeded Darwin's
16 MiB exception-frame limit. The existing `-A linker_messages` hid the linker
warning without repairing the oversized table. Callback cleanup depends on
unwinding, so changing to `panic = "abort"` would remove required behavior.

## Decision

Use development `opt-level = 1`, with explicit debug assertions,
overflow checks and `panic = "unwind"`. The four values have one owner,
the existing `.cargo/config.toml` `[profile.dev]`; they are not duplicated in
the workspace manifest. Remove the previous macOS warning suppression.
Keep the test panic strategy at Cargo's unwind behavior; do not add an ignored
test-profile panic setting. Release settings and public SDK contracts are unchanged.

## Alternatives and tradeoffs

Keeping opt0 and suppressing the warning leaves the defect present. Aborting on
panic loses destructor cleanup. Full release optimization adds unnecessary
development build cost. Opt1 retains the required checks and unwind behavior;
it can increase compilation time and change stepping quality.

Cargo supports profiles in configuration and discovers configuration from the
command's current directory and its ancestors ([profiles](https://doc.rust-lang.org/cargo/reference/profiles.html),
[configuration discovery](https://doc.rust-lang.org/cargo/reference/config.html#hierarchical-structure)).
The supported SDK build/test commands first enter this repository or a member
directory, so they load this configuration. An invocation from outside the
repository using only `--manifest-path` does not automatically load it; such
an invocation is outside this measured development-profile guarantee. Release
profile behavior is unchanged. This location choice retains real required
development configuration and introduces no additional warning policy.

## Evidence and limits

An actual jobs1/debug0/incremental0/strip-symbols development build with opt1,
assertions, overflow checks and unwind exited0 without warnings. Its immutable
Host SHA256 is `45fc7b267941864f4528f55d8d7373d048f5afc824b2fb389c7c8c8fbc680abc`.
`otool` measured `__eh_frame = 15488956` bytes and retained
`__unwind_info = 2979520` bytes. The complete build log SHA256 is
`38a77d8bbe1dc615908e54609e95907c902926d2ed1eb69ba1c0a7776482b2ad`.
Raw inputs, rustc invocation, log and section measurements remain only under
ignored `.echo-semantic/local/sdk-parallel-delivery-01a14385/`.

That trial preceded the callback-binding repair. The current optimized test
binary separately passes the real callback observer-unwind guard/lease/quota
case and six error/identity controls. A test binary alone does not establish
the Host's panic strategy.

The subsequent actual build from the initially saved manifest profile exited0 with no warnings
and no opt/assertion/overflow/panic environment overrides. Its immutable Host
SHA256 is `8bf4a1d6deb647e608153c807ca07785dd31f7d3223a88cc3daf9b835b8bff5c`,
159609616 bytes; `__eh_frame = 15489244`, `__unwind_info = 2979504`.
The full log is 38978 bytes, SHA256
`180bf84154fd95c43dc2a11e195a00cf7d207ab4e702eddf3a2922c39a6ea99e`.
The real rustc command has opt1/debug-assertions enabled and no panic-abort;
that initial manifest explicitly preserved overflow checks and unwind. All861 frozen
non-semantic inputs matched after the build. One necessary callback-unwind
test under these saved settings also actually passed without a profile
override: log452 bytes/SHA256
`4791f027c2198e40adbf9c640179bc75beabea8f69471ea641b7a679539bb91d`.
The trial and initial saved-profile build are separate source cuts.

The final configuration representation was then actually built from the SDK
root, after the original Workflow queue-drain repair. It exited0 with zero
warnings: immutable Host SHA256
`14a0cb58c44bdb84990262f1324e1a2513513a8b40fc0640a9d36f91f95e27e9`,
159609616 bytes; `__eh_frame = 15488572`, `__unwind_info = 2979480`.
The untruncated complete log is 1550567 bytes, SHA256
`1167645df095cf3b370c820f96305562820e1c27a8d2c4ee44c41cbd7d19ea15`.
There were no OPT/ASSERT/OVERFLOW/PANIC environment overrides or
RUSTFLAGS/ENCODED/target-Rust-flags overrides. The previously authorized
jobs1/debug0/strip-symbols/incremental0 resource settings remained active;
this is not a claim of no environment configuration at all. The real Host
rustc command uses opt1/debug-assertions and no panic-abort, `-A`, `-D`, or
linker-message suppression; configuration owns overflow checks and unwind.
Cargo omits an explicit overflow flag here: rustc's documented default enables
overflow checks with debug assertions, and the saved configuration explicitly
sets them true with no off override ([rustc overflow checks](https://doc.rust-lang.org/rustc/codegen-options/index.html#overflow-checks)).
All 63 frozen Rust/Cargo/config inputs matched after the build; all 174
Host/protocol/contract compilation inputs matched the exact C2 candidate.

Raw commands, measurements and the immutable Host remain only in the local
artifact directory. Related focused/native verification and full Rust,
language, platform and exact-main CI acceptance retain their separate scopes.

The final configuration cut subsequently passed four targeted Rust cases
(unwind1, direct projection1, queue cancellation2) and one real late-Completed
Host case in each language, all without failures or skips. These are bounded
runtime checks; complete SDK/main acceptance remains pending.

This build-only change needs no new public example or website feature content.


## 2026-10-31 后继候选：仅优化 Host 测试包

当前 HTTPS Framework `340993201cc937fc8e4783b66be399ef9f14e3d5`、SDK lock
`c4488065aa4a94a8a073e427aeddfefe7670d93b709b82555b39f2431954e1c7`
组合的原完整 `cargo test --workspace --all-features --locked` 已实际退出 101。
Host 单元测试为 267 通过；`core_profile_e2e` 为 33 通过、1 失败，失败属于
Facade 错误分层负控制。剩余 Rust 与 Doc 测试尚未到达，不能记为通过。

链接器另对 Host 的单个 `lib test` 报告 `__eh_frame` 超过 16 MiB。
其实际程序 SHA256 为
`6cda9f47bbcb40b76fa59f9ba5db0c9f7e5c9b192cab5bb24e4a9ecc748b26f8`，
`otool` 测得 `__eh_frame = 16853884` 字节，比 16 MiB 多 76668 字节，
`__unwind_info = 3235952` 字节。此为真实体积问题，不能按已知假阳性放行。

沿用本 ADR 的单一配置权威，候选只在 `.cargo/config.toml` 增加
`[profile.test.package.echo-sdk-host] opt-level = 2`。Cargo 的 test profile
继承 dev，包级 override 仅调整该测试包的优化级别；既有断言与溢出检查仍启用，
test harness 保持 unwind，不新增 test panic 配置，不改变 dev 或 release。
这是待测量的单一优化假设，可能增加编译成本；目前没有优化后体积或零告警结论。
单纯拆分 Rust 源文件仍链接同一个 libtest，不能作为修复证据；本候选不拆公开 API、
测试 target 或 runner，也不更改测试发现、执行和断言。

验证必须保留原完整矩阵及后续 Rust/Doc 目标、原回调 unwind 控制，并保存实际
编译参数与 native fingerprint/profile。只有实际 `__eh_frame` 回到 16 MiB 以下、
`__unwind_info` 保留、编译无告警且完整测试通过，才能确认本组合的改进。
原 101、链接器告警、原始日志和旧段尺寸继续保留；若该假设失败，按新证据重新判断，
不盲目枚举优化参数，不增加 warning suppression 或 `panic=abort`。

依据沿用 [Cargo profiles](https://doc.rust-lang.org/cargo/reference/profiles.html)
中的 test 继承和包级 override 规则。该编译与负控制更正不新增公共功能，
`examples` 与官网无新接口或用法需要同步；原示例和完整语言验收仍须实际执行。
