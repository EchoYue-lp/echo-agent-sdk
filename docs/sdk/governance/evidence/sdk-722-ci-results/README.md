# SDK 722 CI — completed failure terminal

This archive preserves the Root's saved GitHub job JSON and complete raw logs for run [38658108668](https://github.com/EchoYue-lp/echo-agent-sdk/actions/runs/38658108668). Its PR head is `722d09b1a0837f170d4af19cbc3e417d2131b5f3`; checkout logs show PR merge `5d407e160bb5c7ade01be1980ed9a4e5a6888d17`, merging that head into main `30d7e219cc7d3aef0f75a19217c82898c363f50d`.

The final saved scope is **eight completed jobs: five success, three failure**; the workflow terminal is **completed/failure**. This terminal arrived after an initial seven-job observation in which inventory was still pending. That original candidate manifest and README are preserved in the Admin seven-job snapshot, and the original `sdk-722-ci-initial.json` remains unchanged. The later terminal is supported by its separately saved run and inventory job JSON; it is not backdated into the initial observation. Original job timestamps, step results and raw log SHA values remain in the job JSON and archive manifest. Later current-workflow changes are not represented as already validated by this 722 run.

| Saved completed job | Conclusion | Exact coverage qualification |
|---|---|---|
| linux-quality | success | Formatting, lints and featureless build in the original job |
| linux test (protocol) | success | Original protocol/default containers; grouped counts stay separate |
| linux test (sdk-host) | success | Original default/core/bridge/facade profiles; container counts retain their own boundaries |
| windows-check | success | Workspace compile only; no Windows native behavior claim |
| dependency-policy | success | Audit and four cargo-deny policy categories succeeded; bootstrap error below remains visible |
| source language SDKs | failure | TS passed; Python had two failures; Java was not reached |
| sdk-contract | failure | 28 rustdoc profiles completed; 13 accepted sources require explicit dispositions |
| Rust inventory telemetry | failure | 28 profiles completed; the exact same 13 accepted-source continuity pairs were rejected |

## Language failure

The original 198,565-byte language log, SHA `70548f9331858291e374f6e751581cfcf854d8767900d03738d5609041880547`, records TypeScript 336 passed, 0 failed/skipped, followed by its successful quickstart. Python then recorded **413 passed, 2 failed, 18 subtests passed**:

- `test_gap_ack_host.py:132`, `test_gap_ack_snapshot_continuation[True-window]`: the fixed iteration assertion expected 3 and observed 4.
- `test_tool_value_effect_host.py:113`, `test_tool_value_effect_native_host`: the process-observation assertion saw an empty list.

These are observed failures and do not by themselves establish their final root cause or repair. Python quickstart, Java tests and the Java example were not reached by this sequential job. Earlier local whole-language success and later candidate fixes cannot replace this remote failure.

## Dependency result and bootstrap error

The original 1,084,474-byte dependency log, SHA `59dc929cd16b4f36b4a30a6eaf8e3e3db9d6f22af695e3bf9f1fba094fb5ccff`, retains the successful audit and `advisories ok, bans ok, licenses ok, sources ok` result. It emits the accepted 50 exact duplicate-version warning groups. The saved job JSON concludes success.

It also contains one actual bootstrap toolchain diagnostic: `stable-x86_64-unknown-linux-musl` was not installed for the repository's toolchain file. That error is preserved, rather than hidden by the final job success or described as zero log errors. The bounded [upstream bootstrap analysis](../cargo-deny-stable-toolchain/provenance.json) preserves official action sources and the same original-log SHA. That formal package is present in the same working tree; this archive does not pretend it was already committed or that a later bootstrap fix has run in CI. The complete original dependency log remains here because it is an independent result of this run observation.

## Contract, inventory, profile and warning limits

The original contract log completed 28 profiles before failing on 13 accepted-source disappearances, listed verbatim in `manifest.json`. The final inventory log (61,725 bytes, SHA `bf3fd726a794051b255d725355de9bba75e3df70e293dc7b86e29c59eb2b3e08`) completed 28 profiles and rejected the exact same 13 identity/kind pairs. The archive compared those extracted pairs byte-for-byte; it did not regenerate or update the inventory. No accepted seed or retirement decision was changed to obtain this observation. Public inventory item counts are telemetry and do not stand in for behavior evidence. The Host and protocol test container results are listed individually in the manifest, preserving overlapping feature/profile scope rather than summing them as distinct capability tests.

The eight archived logs contain zero Node runtime deprecation-warning diagnostics from the updated actions. This limited observation does not suppress unrelated runtime, dependency or bootstrap messages; original bytes remain untrimmed. It does not certify a new CI run for a current workflow revision.

## Archive boundary

`original-copy-provenance.json` binds 24 distinct original saved files, including eight complete job logs and JSON responses plus the Root's bounded observation manifests. The checksum index excludes itself. No SDK source, semantic object, accepted contract seed, workflow, CI run, PR message, registry or remote Git ref was changed by this archive task. Main acceptance remains separate: five successful jobs do not override the three failures or the completed/failure workflow terminal.
