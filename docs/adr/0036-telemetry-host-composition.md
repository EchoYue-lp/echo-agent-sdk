# ADR 0036: Compose accepted telemetry with the ACP Host

## Status

Accepted technical repair after independently reviewed design 06ec0e0a on 2026-10-14; no new language contract or lifecycle policy is accepted.

## Context

The normal Host already installs a global stderr subscriber. Runtime telemetry initialization cannot replace it; the failed setup drops a metrics provider and synchronously blocks the ACP dispatcher. Three real language clients reproduce initialization noncompletion and an OS sample identifies provider shutdown on the request stack.

## Alternatives

Ignoring subscriber failure, metrics-only export and removing stderr diagnostics lose accepted behavior. Reloading the complete OpenTelemetry layer breaks downcast/context support. A stable layer delegating the original framework tracer and the existing Host settlement collector preserve those authorities.

## Decision

The framework remains the only METRICS/RUNTIME/exporter/sampling owner and exposes a Rust/Host-only composition helper. Host installs it once with a single stderr fmt writer whose original diagnostic filter is ORed with the framework console filter. The OpenTelemetry filter remains independent, preserving traces and RUST_LOG without duplicate formatting. Runtime init/shutdown run in a blocking task retained by an independent collector through the existing Host settlement counter and close chain. RPC cancellation or EOF cannot fake blocking-work completion. Existing init/status/shutdown wire operations, one-shot initialization and shutdown error behavior remain unchanged.

## Consequences

Three-language real initialization, collector receipt/flush, stable span context and blocking-job cancellation/EOF/close proofs are required before delivery. The approved bounded [design](https://github.com/EchoYue-lp/echo-agent-sdk/blob/0ebe15f66d0aa57c7b7a92ae68cd1be106114ae9/docs/sdk/governance/evidence/telemetry-host/design.md) and [public-close evidence](https://github.com/EchoYue-lp/echo-agent-sdk/blob/0ebe15f66d0aa57c7b7a92ae68cd1be106114ae9/docs/sdk/governance/evidence/telemetry-host/publicclose-approved/receipt.json) retain their own source/Host hashes. The current Host uses published framework a634 without a local Cargo patch. These earlier evidence cuts do not substitute for final validation after subsequent Host changes; a temporary diagnostic Cargo patch is not final delivery evidence. No second telemetry state machine or protocol is introduced.
