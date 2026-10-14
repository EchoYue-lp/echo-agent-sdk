# Real default Permission TTL proof scope

Only admin scripts/outputs changed. No SDK source/test/docs, configuration TTL, clock setter, mock, Cargo/Maven target writes, or new public API.

Each native SDK uses its own original Host bb4f process, Agent and two Sessions, with unique local state_root. Public Session.invoke only. Four entries in A: two parameter-scoped approvals, two tool-scoped approvals. B has no entries. Read stats confirms ttl_ms1800000, initial matches/nonmatches/peer isolation/once false, early cleanup0, stats still2+2.

Wait uses each process's actual monotonic elapsed time from after the final recorded approval plus1500ms safety. No interim approval renewal. is_approved uses framework read locks and Instant elapsed; expiration does not delete records. At expiry all4 matches false, stats still4, explicit cleanup4 then second cleanup0/stats0. A fresh record returns true, showing live manager. This final fresh1 is intentionally destroyed with Session/Agent/SDK owner close and actual Host exit; there is no claimed extra clear_cache/revoke call or explicit final cache count0.

Cache is volatile and per Session authority. This proof does not establish durable permissions across restarts. Initial Java source-file runner failed compilation before any Host launch because it called package-private WireHandle.toJson; preserved runner-first-compile-error.log. Relaunched Java uses only public id/generation/kind getters, no SDK visibility change. Python/TS earlier timers remain original and genuine. Source-file Java compilation is isolated by the Java launcher; existing target classes and dependency classpath are read-only.

Parent confirmed this finite scope and instructed continuing original loaded runners without restart or additional criteria. Expiry outcome is pending while status phase is waiting_real_default_ttl; initial approved true is not completion proof.
