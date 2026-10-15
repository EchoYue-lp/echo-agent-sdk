# cargo-deny action stable override initialization — Phase1–3 proposal

Status: read-only investigation/proposal. SDK workflow, preflight, toolchain and lock are unchanged. No CI cancellation or new runtime execution performed.

Original722 dependency job succeeds but original log116172597289 line310 contains real `error: override toolchain stable-x86_64-unknown-linux-musl is not installed`. Action image contains1.85.0; repository intentionally requests stable. Entrypoint runs rustup show, it fails once, then fallback rustup toolchain install syncs stable1.99 before cargo-deny succeeds. Job success does not erase that real error or make it eligible for the accepted fifty duplicate warnings exception.

## Actual official source

The run resolves EmbarkStudios/cargo-deny-action@v2 to `3c6349835b2b7b196a839186cb8b78e02f7b5f25`; current officialv2 resolves to the same SHA. Exact official API responses and decoded action.yml/entrypoint.sh/Dockerfile/README bytes are archived with hashes in provenance.json. Dockerfile installs cargo-deny0.20.2 on rust1.85.0-alpine3.20. README's image1.71.0 sentence is stale; actual Dockerfile and executed log are the authority for this observation.

Action input rust-version is passed as positional argument1. Actual entrypoint order:

1. Nonempty arg1: rustup set profile minimal; rustup default "$1".
2. Credentials/SSH only if separately configured, then shift4.
3. `(cd "$(dirname "$4")"; rustup show || rustup toolchain install)`.
4. `cargo-deny $*` with the original arguments.

Thus `rust-version: stable` invokes rustup default stable *before* the diagnostic show, using the same channel the root rust-toolchain.toml already requests. This is not a downgrade to image1.85 and not quiet/log suppression. Standard rustup default installs the selected unavailable toolchain while setting its default; failure remains fatal under set-e. The exact stable install has not yet been executed in a new action container by this lane. This is a strong source-order diagnosis and a bounded proposal, not a fabricated Docker red/green.

## Minimal fix

Add only `rust-version: stable` to the existing cargo-deny action with block. Keep actionv2/currenttool0.20.2, rootstable, manifest, loglevelwarn, commandcheck, workspace/allfeatures/locked arguments and all four policy categories intact. No target graph/50duplicate group baseline change or advisory ignore. Do not replace the Docker action or pinSDKRust1.85. Proposed complete patch is one-input.patch; workflow.before/candidate preserve original bytes except that line.

An exact YAML BaseLoader AST check passed: after removing the new stable input all nodes equal. The archived official entrypoint was executed with explicitly mocked rustup/cargo-deny command boundaries; old empty arg path records show(error)->install->cargo-deny, new stable path records setprofile->defaultstable->show->samecargo-denyargv. Both exit0 in that model, illustrating why the old job can be green with an error. bounded-validation.json/raw order/stdout/stderr are labeled mocks and do not claim real rustup install/export/advisory coverage.

Root's independent design review and explicit write authorization precede implementation. The actual new remote CI must confirm no initial override error and that audit/deny checks and exact fifty accepted duplicate groups remain valid. An external install failure is still a real failure, not ignored. No warning/error-resolved or full dependency/main claim is made now.

Official source: https://github.com/EmbarkStudios/cargo-deny-action/blob/3c6349835b2b7b196a839186cb8b78e02f7b5f25/entrypoint.sh and action.yml/Dockerfile at that same commit. Web search tool returned404 invalid/v1alpha; officialGitHubAPI200 supplied actual source bytes. No code in third-party repository is changed.
