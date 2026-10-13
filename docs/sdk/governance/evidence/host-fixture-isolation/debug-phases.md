# SDK Host fixture isolation debug

Phase 1: Original whole official command Node287 had host.test smoke fail OS35 file-backed authority already open; minimal unchanged two-file Node run reproduced3tests2pass1fail with shared config. Host FileRuntimeStateStore intentionally owns one native root; both host.test smoke and family-route-host spawn independent processes with same ECHO_AGENT_SDK_CONFIG.

Phase 2: Existing parallel-session and state/trace fixtures create a distinct mkdtemp work/host.json/state root per Host. Two smoke routes alone reuse official shared root; memory_path also points to same root. Same framework/Host/client/input operations unchanged.

Phase 3: Single hypothesis: config ownership collision, not broken runtime locks. Keep the same official config model, agent options and limits; copy JSON for each Host instance and replace only sdk_profile.state_root and optional Agent memory_path with paths below independent temp work root. Direct two-file unchanged default Node file concurrency is minimum counterexample. Local fixture repair, no public contract/product/state authority change; no new helper necessary for two short callers.

Phase 4: Preserve all existing semantic assertions and parallel file execution. Each caller retains SDK until close then removes its own temp work. Rerun same two files against exact immutable8d819 and same outer official config; no Cargo/Maven, no test serial or locks disabled.
