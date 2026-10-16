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
