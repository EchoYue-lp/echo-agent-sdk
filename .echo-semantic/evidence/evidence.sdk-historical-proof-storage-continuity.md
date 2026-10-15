---
schema_version: 1
id: evidence.sdk-historical-proof-storage-continuity
kind: evidence
evidence_type: semantic_continuity
observed_at: source:2bbd6457d7b6ea031b8999d1466d85f5a5c1d2d994218d45f008ed6d5f1fa276
source_refs:
- docs/sdk/governance/semantic-history.md
- docs/adr/0038-sdk-historical-proof-storage.md
- .github/workflows/rust-ci.yml
supports:
- asset.sdk-historical-proof-location
limitations:
- 逐issue/variant严格受控存储表示迁移，完整机器终态另验
- SApply未发生，原产品62resolved4open/平台/业务失败限制保
merge_base_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
predecessor_revisions:
- 7f14a1b9c751b0612ceb42224f83f29d09aea46c
result_snapshot: source:2bbd6457d7b6ea031b8999d1466d85f5a5c1d2d994218d45f008ed6d5f1fa276
resolutions:
  behavior.sdk-repository-ownership:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 7cf42b5a9ffab4e8c9a850aa1b923848bdcd2cea64a9ce58e4ba10c8e7b7173a
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: &id001
    - kind: adr
      path: docs/adr/0038-sdk-historical-proof-storage.md
      content_digest: f33359d36ca0ba8e487940c534625d3b6fd713c334cb3c221229439e719404b5
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  map.sdk-product-boundary#scenario:source-history:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 6816b6f3534d576837e5c54109a55bbb0ed2a41c098da61efa504e70cd84fe56
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  map.sdk-product-boundary#scenario:semantic-ci-canonical-source-check:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 711a764a440c7bab7e0d792abd3bbb7c161057484d7e4162a86fa9210c6b5eb6
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  rule.framework-runtime-authority:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 70ec7721c16dc9e713684fb07b302c3c23c7cc0106fa07a8bd997b6458e703bd
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-parent-driver-20261013/current-production-observation.json
      historical_git_mode: '100644'
      owner: rule.framework-runtime-authority
      source_role: documentation
      observed_revision: 47579c895657c182eceacfbb63865b04ec55a4a1
      issue_id: 59c97fc927d36971a330f01f798b6fabc86bf3f99c0eac5dbe39ddfcfbf2b900
      owner_fingerprint: 70ec7721c16dc9e713684fb07b302c3c23c7cc0106fa07a8bd997b6458e703bd
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 0c866683cd68f5cff78c5c6170eb25df646a6d994526898b806066fd5f509c01
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_cede7a7cbb45f3e2
    - raw_reference: docs/sdk/governance/evidence/sdk-java-host-early-exit/EarlyExitProbe.java
      historical_git_mode: '100644'
      owner: rule.framework-runtime-authority
      source_role: documentation
      observed_revision: 47579c895657c182eceacfbb63865b04ec55a4a1
      issue_id: ec86f8b83b770e23fca6584400dc0a41ee0d39b3c8c48bde5660021f3d3f0aed
      owner_fingerprint: 70ec7721c16dc9e713684fb07b302c3c23c7cc0106fa07a8bd997b6458e703bd
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: e865d5faeb408aafb7fed768e1a81a97065634b50fafcb3c08d1903ea57ef2ec
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_cede7a7cbb45f3e2
    - raw_reference: docs/sdk/governance/evidence/sdk-java-host-early-exit/before-closing-subscriber-EchoAgentClient.java
      historical_git_mode: '100644'
      owner: rule.framework-runtime-authority
      source_role: documentation
      observed_revision: 47579c895657c182eceacfbb63865b04ec55a4a1
      issue_id: 1d17e6951702d91dacb9b6ab2a1d4994eae443b377e2ce2b823da3cf88f41dc9
      owner_fingerprint: 70ec7721c16dc9e713684fb07b302c3c23c7cc0106fa07a8bd997b6458e703bd
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 06c1c0e54bd1fc830b05e981af70ea1f34a84153fc474448005eadc5a9ae0a12
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_cede7a7cbb45f3e2
    - raw_reference: docs/sdk/governance/evidence/sdk-java-host-early-exit/before-late-subscribe-EchoAgentClient.java
      historical_git_mode: '100644'
      owner: rule.framework-runtime-authority
      source_role: documentation
      observed_revision: 47579c895657c182eceacfbb63865b04ec55a4a1
      issue_id: 36e31f953e3ed7e79610f64d5b4fe5abb7cdd13ce9de76da932e35b93361d3d3
      owner_fingerprint: 70ec7721c16dc9e713684fb07b302c3c23c7cc0106fa07a8bd997b6458e703bd
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 206c5cb3aeb254199c6c6ca4c8a2673fe0f4cc500575c5422d9223a0eb702a69
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_cede7a7cbb45f3e2
    - raw_reference: docs/sdk/governance/evidence/sdk-java-host-early-exit/before/EchoAgentClient.java
      historical_git_mode: '100644'
      owner: rule.framework-runtime-authority
      source_role: documentation
      observed_revision: 47579c895657c182eceacfbb63865b04ec55a4a1
      issue_id: 9f789a4f50b6a127e27765ff6ffda63235f9186b794503fbcdafed7b70f2cd8d
      owner_fingerprint: 70ec7721c16dc9e713684fb07b302c3c23c7cc0106fa07a8bd997b6458e703bd
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 5dd37fb7a5ca71b9f8ff5fa89c1c79c41d2f5f34dcc8839a1a6cd83c8d38d37a
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_cede7a7cbb45f3e2
    - raw_reference: docs/sdk/governance/evidence/sdk-python-test-entry/check-language-sdks.after.sh
      historical_git_mode: '100755'
      owner: rule.framework-runtime-authority
      source_role: documentation
      observed_revision: 47579c895657c182eceacfbb63865b04ec55a4a1
      issue_id: 816b096fbc9492e6a0cf713cdaf095bc066cec44e1b3d424fcc5ca866c147df9
      owner_fingerprint: 70ec7721c16dc9e713684fb07b302c3c23c7cc0106fa07a8bd997b6458e703bd
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 8f2ae358ff8131034a3b9c65b9de50d576d24661559a50835d16be40198f020a
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_cede7a7cbb45f3e2
    - raw_reference: docs/sdk/governance/evidence/sdk-python-test-entry/check-language-sdks.before.sh
      historical_git_mode: '100755'
      owner: rule.framework-runtime-authority
      source_role: documentation
      observed_revision: 47579c895657c182eceacfbb63865b04ec55a4a1
      issue_id: 6cf769ac8c7003d2fc13b2fd27898c0872fa1bddf13a167dec41dd1886933657
      owner_fingerprint: 70ec7721c16dc9e713684fb07b302c3c23c7cc0106fa07a8bd997b6458e703bd
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 1fbfcf65c72d87137c7734a97eb7421d435d6a7316a63c736dc8f7a34586a5ab
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_cede7a7cbb45f3e2
    - raw_reference: docs/sdk/governance/evidence/sdk-language-test-nop-complete/executed-source/sdks/java/pom.xml
      historical_git_mode: '100644'
      owner: rule.framework-runtime-authority
      source_role: documentation
      observed_revision: 47579c895657c182eceacfbb63865b04ec55a4a1
      issue_id: 38f886ec8ecf7fb340c1bbaf3c54c57bd09a90d3291957cf4ebcd8862238b07c
      owner_fingerprint: 70ec7721c16dc9e713684fb07b302c3c23c7cc0106fa07a8bd997b6458e703bd
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: b0d92517965cdfcac86ccfbba06efd90cd19ac3300327a0ea379b2ce4dd99c61
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_cede7a7cbb45f3e2
    - raw_reference: docs/sdk/governance/evidence/factory-child-settlement/before-regression-bridge.rs
      historical_git_mode: '100644'
      owner: rule.framework-runtime-authority
      source_role: documentation
      observed_revision: 47579c895657c182eceacfbb63865b04ec55a4a1
      issue_id: ca4ca8813c7051613fd60049a60a453b5ffe0e941822ccff05dabee88b12a0cf
      owner_fingerprint: 70ec7721c16dc9e713684fb07b302c3c23c7cc0106fa07a8bd997b6458e703bd
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: e12b3ce4c6556b52c94a1452d778b55ca6577d6b260d5355a7fa662958466a36
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_cede7a7cbb45f3e2
    - raw_reference: docs/sdk/governance/evidence/factory-child-settlement/after-bridge.rs
      historical_git_mode: '100644'
      owner: rule.framework-runtime-authority
      source_role: documentation
      observed_revision: 47579c895657c182eceacfbb63865b04ec55a4a1
      issue_id: a9eb0e94e89a2aaaa80c8f0eb1ee27799c92154fc78a60d2c568c869cd87fa77
      owner_fingerprint: 70ec7721c16dc9e713684fb07b302c3c23c7cc0106fa07a8bd997b6458e703bd
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 8e7aad2896687394a507f339fb20cba2dbacb93fd42ab38b417534570204e8e9
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_cede7a7cbb45f3e2
    - raw_reference: docs/sdk/governance/evidence/factory-child-settlement/before-e2e.rs
      historical_git_mode: '100644'
      owner: rule.framework-runtime-authority
      source_role: documentation
      observed_revision: 47579c895657c182eceacfbb63865b04ec55a4a1
      issue_id: 22ebd089e3db37ad3104fffd0d1fb8b09cf253351926a7f8ee92761eefee92ab
      owner_fingerprint: 70ec7721c16dc9e713684fb07b302c3c23c7cc0106fa07a8bd997b6458e703bd
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 9c951e262f4557a24d98cef44f3c17981231a6a77108262e9031e03ba4d1c88f
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_cede7a7cbb45f3e2
    - raw_reference: docs/sdk/governance/evidence/factory-child-settlement/before-retry-fixed-bridge.rs
      historical_git_mode: '100644'
      owner: rule.framework-runtime-authority
      source_role: documentation
      observed_revision: 47579c895657c182eceacfbb63865b04ec55a4a1
      issue_id: 84fd59ff7a481aee2420a4398a7c28e70e55ab06769e259ae45cef43773f5635
      owner_fingerprint: 70ec7721c16dc9e713684fb07b302c3c23c7cc0106fa07a8bd997b6458e703bd
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: f863a8d5dea5812825e50142b865974e3d09cf7d9f542b9d02399a4c69a49d02
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_cede7a7cbb45f3e2
    - raw_reference: docs/sdk/governance/evidence/factory-child-settlement/ordering-fixed-bridge.rs
      historical_git_mode: '100644'
      owner: rule.framework-runtime-authority
      source_role: documentation
      observed_revision: 47579c895657c182eceacfbb63865b04ec55a4a1
      issue_id: 9f594c6c775368851058a12b1784dc93798c6924b6a19e905c6ce560d8f46ea5
      owner_fingerprint: 70ec7721c16dc9e713684fb07b302c3c23c7cc0106fa07a8bd997b6458e703bd
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 33fcaff165251156578a917b2a7d62d356895d3c465247e534ae82d34f2fde93
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_cede7a7cbb45f3e2
    - raw_reference: docs/sdk/governance/evidence/factory-child-settlement/before-bridge.rs
      historical_git_mode: '100644'
      owner: rule.framework-runtime-authority
      source_role: documentation
      observed_revision: 47579c895657c182eceacfbb63865b04ec55a4a1
      issue_id: f23bb066bf69226ac2b2a34f7839061aa2ed9574dbd8df9542ea578f02e6cffd
      owner_fingerprint: 70ec7721c16dc9e713684fb07b302c3c23c7cc0106fa07a8bd997b6458e703bd
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 43ecf356e4deb1004158540708b664e6e022ddf4afc9f545d240903c4758fefe
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_cede7a7cbb45f3e2
    - raw_reference: docs/sdk/governance/evidence/factory-child-settlement/after-e2e.rs
      historical_git_mode: '100644'
      owner: rule.framework-runtime-authority
      source_role: documentation
      observed_revision: 47579c895657c182eceacfbb63865b04ec55a4a1
      issue_id: 077a783b2f40c28366c79ee424c7b130953dfeccae1c7a53ed146b60d8e85274
      owner_fingerprint: 70ec7721c16dc9e713684fb07b302c3c23c7cc0106fa07a8bd997b6458e703bd
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 7e2312d1fec10663fbde463d989a008a566dd75723a61ce2f26423dfd7dfa2ba
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_cede7a7cbb45f3e2
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/before-dev-test-dependencies-Cargo.toml
      historical_git_mode: '100644'
      owner: rule.framework-runtime-authority
      source_role: documentation
      observed_revision: 47579c895657c182eceacfbb63865b04ec55a4a1
      issue_id: dec19f975826ac13f539b1c15d6d15a5e6e24301c40a0170d2bc60c20e23be3b
      owner_fingerprint: 70ec7721c16dc9e713684fb07b302c3c23c7cc0106fa07a8bd997b6458e703bd
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: e1ee65763c18fd754bacfec922ba22d68180f3bea3114a25146f7d5bc1651a0e
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_cede7a7cbb45f3e2
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/feature-wiring-current/echo-sdk-host/Cargo.toml
      historical_git_mode: '100644'
      owner: rule.framework-runtime-authority
      source_role: documentation
      observed_revision: 47579c895657c182eceacfbb63865b04ec55a4a1
      issue_id: f69a28ef7417a6525855f77f5e336a94cf31a4fda82e4cd0a60b1d113be7a06d
      owner_fingerprint: 70ec7721c16dc9e713684fb07b302c3c23c7cc0106fa07a8bd997b6458e703bd
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 7ab7770de3757e304dc6e65175c8cba8abc61c229afd015dad78653ff8afd3a6
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_cede7a7cbb45f3e2
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/before-feature-wiring/echo-sdk-host/src/core_profile/handles.rs
      historical_git_mode: '100644'
      owner: rule.framework-runtime-authority
      source_role: documentation
      observed_revision: 47579c895657c182eceacfbb63865b04ec55a4a1
      issue_id: 47b7d13219a1f1497dd7f148cfd41fad2844142aed16d0b94628cc5eabdcb512
      owner_fingerprint: 70ec7721c16dc9e713684fb07b302c3c23c7cc0106fa07a8bd997b6458e703bd
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 8c66cab1884085c2d0591fd8d909623d0c3a07a588c21e0334a098a3e273dca1
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_cede7a7cbb45f3e2
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/feature-wiring-current/echo-sdk-host/src/core_profile/handles.rs
      historical_git_mode: '100644'
      owner: rule.framework-runtime-authority
      source_role: documentation
      observed_revision: 47579c895657c182eceacfbb63865b04ec55a4a1
      issue_id: 2e9e8a8c9d7b068cdc009a47d6c139723db0dfffc800bdde954e1eeae35fc7e9
      owner_fingerprint: 70ec7721c16dc9e713684fb07b302c3c23c7cc0106fa07a8bd997b6458e703bd
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: b78d63d156926a474b800cb0bf35a5627cb471ccf87a9789dcf6d5863b036fd4
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_cede7a7cbb45f3e2
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/before-feature-wiring/echo-sdk-host/src/core_profile/wire.rs
      historical_git_mode: '100644'
      owner: rule.framework-runtime-authority
      source_role: documentation
      observed_revision: 47579c895657c182eceacfbb63865b04ec55a4a1
      issue_id: 176022d8d587034f979cbcb1024e62c5e59e1093905f89248b4705e01e2f56f3
      owner_fingerprint: 70ec7721c16dc9e713684fb07b302c3c23c7cc0106fa07a8bd997b6458e703bd
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 47cafb0e9b836e007adad2ac1e805e6ad7ef4f7ff85dc30dac1afbde3ca082c5
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_cede7a7cbb45f3e2
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/feature-wiring-current/echo-sdk-host/src/core_profile/wire.rs
      historical_git_mode: '100644'
      owner: rule.framework-runtime-authority
      source_role: documentation
      observed_revision: 47579c895657c182eceacfbb63865b04ec55a4a1
      issue_id: cd39f9959e4a13dc5c57c50134ea6cf41754ef4014abef1cb098f264d10db450
      owner_fingerprint: 70ec7721c16dc9e713684fb07b302c3c23c7cc0106fa07a8bd997b6458e703bd
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 4d47c7c9e9d541087e0ef2e48cb9d6a495b357216ee92276ab6a6e267195be37
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_cede7a7cbb45f3e2
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/before-feature-wiring/echo-sdk-host/src/core_profile/facade/source_operations.rs
      historical_git_mode: '100644'
      owner: rule.framework-runtime-authority
      source_role: documentation
      observed_revision: 47579c895657c182eceacfbb63865b04ec55a4a1
      issue_id: 71db72ac1b537fabfd9cbbbaeb193ba78d1aa249611907a6e96a55ff547bd82f
      owner_fingerprint: 70ec7721c16dc9e713684fb07b302c3c23c7cc0106fa07a8bd997b6458e703bd
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: afac5c4f8da6ccda7b99626d7bb26ce6f80b873fd6807d13db133c36ca581a73
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_cede7a7cbb45f3e2
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/feature-wiring-current/echo-sdk-host/src/core_profile/facade/source_operations.rs
      historical_git_mode: '100644'
      owner: rule.framework-runtime-authority
      source_role: documentation
      observed_revision: 47579c895657c182eceacfbb63865b04ec55a4a1
      issue_id: b3bf4eea4149146cb397bd6743ceabe17ba3b721b1abe022586223aa10a0fb96
      owner_fingerprint: 70ec7721c16dc9e713684fb07b302c3c23c7cc0106fa07a8bd997b6458e703bd
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 91bd8f84f3a05b272b52911c7ba52d7cc54537d9f88790cfbb11578a57817a94
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_cede7a7cbb45f3e2
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/before-feature-wiring/echo-sdk-host/src/core_profile/facade/stream.rs
      historical_git_mode: '100644'
      owner: rule.framework-runtime-authority
      source_role: documentation
      observed_revision: 47579c895657c182eceacfbb63865b04ec55a4a1
      issue_id: d6fb7ed5579d8ab346ac42b011e4c0b42e949ad15c8254976e7680893570a947
      owner_fingerprint: 70ec7721c16dc9e713684fb07b302c3c23c7cc0106fa07a8bd997b6458e703bd
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 1defec3e27aade8fdb906841280a5a7a62dc85b8d2770eceffcbbc3ea2fcdfa6
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_cede7a7cbb45f3e2
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/feature-wiring-current/echo-sdk-host/src/core_profile/facade/stream.rs
      historical_git_mode: '100644'
      owner: rule.framework-runtime-authority
      source_role: documentation
      observed_revision: 47579c895657c182eceacfbb63865b04ec55a4a1
      issue_id: 09530f7d1c18f464666a2d74efdf90fa1963ddddc3f6f2974f26994b1a0be9cd
      owner_fingerprint: 70ec7721c16dc9e713684fb07b302c3c23c7cc0106fa07a8bd997b6458e703bd
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 1d977c5a5a33f411c28c8f87ca369e200dcce36d94e6d8b600f174d74013609c
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_cede7a7cbb45f3e2
    current_reference_replacements:
    - raw_reference: docs/sdk/governance/evidence/host-fixture-isolation/after-source/family-route-host.test.js
      historical_git_mode: '100644'
      owner: rule.framework-runtime-authority
      source_role: test_consumer
      observed_revision: 47579c895657c182eceacfbb63865b04ec55a4a1
      issue_id: 6ca973324b1066146fea3a2d793f020f11f9529ed23414d1411fa446978d33e3
      owner_fingerprint: 70ec7721c16dc9e713684fb07b302c3c23c7cc0106fa07a8bd997b6458e703bd
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 0fb0cee6cb24536ccbc36af817f9610e603c2bb54920705e2ab4ad4a89414881
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_cede7a7cbb45f3e2
      replacement_source_ref: sdks/typescript/test/family-route-host.test.js
      replacement_content_sha256: 0fb0cee6cb24536ccbc36af817f9610e603c2bb54920705e2ab4ad4a89414881
      replacement_git_mode: '100644'
    - raw_reference: docs/sdk/governance/evidence/host-fixture-isolation/after-source/host.test.js
      historical_git_mode: '100644'
      owner: rule.framework-runtime-authority
      source_role: test_consumer
      observed_revision: 47579c895657c182eceacfbb63865b04ec55a4a1
      issue_id: ca2731a6668da92f1442541d2d3b0a335041516eaf746108ffeea84a5285ae83
      owner_fingerprint: 70ec7721c16dc9e713684fb07b302c3c23c7cc0106fa07a8bd997b6458e703bd
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 427abceebc699c04e3649a721c10271c5e248951c496fe2d5b86aab89c5b4bda
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_cede7a7cbb45f3e2
      replacement_source_ref: sdks/typescript/test/host.test.js
      replacement_content_sha256: 427abceebc699c04e3649a721c10271c5e248951c496fe2d5b86aab89c5b4bda
      replacement_git_mode: '100644'
    - raw_reference: docs/sdk/governance/evidence/host-fixture-isolation/before-source/family-route-host.test.js
      historical_git_mode: '100644'
      owner: rule.framework-runtime-authority
      source_role: test_consumer
      observed_revision: 47579c895657c182eceacfbb63865b04ec55a4a1
      issue_id: 97c9cbf3f0e88fc2c5370b83e874f1dec319d63949d91fa090221e2059464162
      owner_fingerprint: 70ec7721c16dc9e713684fb07b302c3c23c7cc0106fa07a8bd997b6458e703bd
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 6dbf7c6ced8e509e2de3c7b6e2ab9bf5fbbf64921fcc9bdbecb1e7b6df1c1051
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_cede7a7cbb45f3e2
      replacement_source_ref: sdks/typescript/test/family-route-host.test.js
      replacement_content_sha256: 0fb0cee6cb24536ccbc36af817f9610e603c2bb54920705e2ab4ad4a89414881
      replacement_git_mode: '100644'
    - raw_reference: docs/sdk/governance/evidence/host-fixture-isolation/before-source/host.test.js
      historical_git_mode: '100644'
      owner: rule.framework-runtime-authority
      source_role: test_consumer
      observed_revision: 47579c895657c182eceacfbb63865b04ec55a4a1
      issue_id: d2f93d56aad5db690b1213c2ed740e88a5b977d0d32f8af203bba87b28308926
      owner_fingerprint: 70ec7721c16dc9e713684fb07b302c3c23c7cc0106fa07a8bd997b6458e703bd
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 796aadc7a3449b6bd31d54951b811e0f263a79745dc94e6f0cdd35f70702cd17
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_cede7a7cbb45f3e2
      replacement_source_ref: sdks/typescript/test/host.test.js
      replacement_content_sha256: 427abceebc699c04e3649a721c10271c5e248951c496fe2d5b86aab89c5b4bda
      replacement_git_mode: '100644'
    - raw_reference: docs/sdk/governance/evidence/sdk-java-default-root-repair/after/sdks/java/src/test/java/com/echoagent/sdk/ChannelHandlerForwardHostTest.java
      historical_git_mode: '100644'
      owner: rule.framework-runtime-authority
      source_role: test_consumer
      observed_revision: 47579c895657c182eceacfbb63865b04ec55a4a1
      issue_id: 1e79894d7c259d88bc7a81c8f0ead381616437b9dfef79ff1a8d48c8b5bc00ff
      owner_fingerprint: 70ec7721c16dc9e713684fb07b302c3c23c7cc0106fa07a8bd997b6458e703bd
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: f8ac15231d902abbbd8fd39af4c7a84b5e278998eed9bcb3d3b9c27ffe667fb4
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_cede7a7cbb45f3e2
      replacement_source_ref: sdks/java/src/test/java/com/echoagent/sdk/ChannelHandlerForwardHostTest.java
      replacement_content_sha256: 468f369c7a635ff3d2178794f6fe791bf7eea41bd9eca110f6ba394bfe9f533c
      replacement_git_mode: '100644'
    - raw_reference: docs/sdk/governance/evidence/sdk-java-default-root-repair/after/sdks/java/src/test/java/com/echoagent/sdk/EvalImproveTopologyHostTest.java
      historical_git_mode: '100644'
      owner: rule.framework-runtime-authority
      source_role: test_consumer
      observed_revision: 47579c895657c182eceacfbb63865b04ec55a4a1
      issue_id: 924973aba91a488db9fd8ee5c24b1cabdf1fd1e100bcc83589e6862e1c3e8023
      owner_fingerprint: 70ec7721c16dc9e713684fb07b302c3c23c7cc0106fa07a8bd997b6458e703bd
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: a12a785f80302f0104e47eaeadb7e31919a0f228023ec64ed7b142d173e37a69
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_cede7a7cbb45f3e2
      replacement_source_ref: sdks/java/src/test/java/com/echoagent/sdk/EvalImproveTopologyHostTest.java
      replacement_content_sha256: a12a785f80302f0104e47eaeadb7e31919a0f228023ec64ed7b142d173e37a69
      replacement_git_mode: '100644'
    - raw_reference: docs/sdk/governance/evidence/sdk-java-default-root-repair/after/sdks/java/src/test/java/com/echoagent/sdk/LlmWorkflowStreamHostTest.java
      historical_git_mode: '100644'
      owner: rule.framework-runtime-authority
      source_role: test_consumer
      observed_revision: 47579c895657c182eceacfbb63865b04ec55a4a1
      issue_id: 03db2eed083fc5cfe94baf301c0b08b4dce6b271be9b564754280364867528f0
      owner_fingerprint: 70ec7721c16dc9e713684fb07b302c3c23c7cc0106fa07a8bd997b6458e703bd
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: ddd4827d47ff0329039ad11fc1f0241f9185647fdb37db06a122112d0f38b301
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_cede7a7cbb45f3e2
      replacement_source_ref: sdks/java/src/test/java/com/echoagent/sdk/LlmWorkflowStreamHostTest.java
      replacement_content_sha256: ddd4827d47ff0329039ad11fc1f0241f9185647fdb37db06a122112d0f38b301
      replacement_git_mode: '100644'
    - raw_reference: docs/sdk/governance/evidence/sdk-java-default-root-repair/after/sdks/java/src/test/java/com/echoagent/sdk/ManagedRetirementHostTest.java
      historical_git_mode: '100644'
      owner: rule.framework-runtime-authority
      source_role: test_consumer
      observed_revision: 47579c895657c182eceacfbb63865b04ec55a4a1
      issue_id: 09093393bfaad454be796990102755402486ca53d15f455bd372cb2d22b78351
      owner_fingerprint: 70ec7721c16dc9e713684fb07b302c3c23c7cc0106fa07a8bd997b6458e703bd
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: b635517906e348deeadf19d5849a6a51a873654ba18648e258e2b342d452e695
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_cede7a7cbb45f3e2
      replacement_source_ref: sdks/java/src/test/java/com/echoagent/sdk/ManagedRetirementHostTest.java
      replacement_content_sha256: b635517906e348deeadf19d5849a6a51a873654ba18648e258e2b342d452e695
      replacement_git_mode: '100644'
    - raw_reference: docs/sdk/governance/evidence/sdk-java-default-root-repair/after/sdks/java/src/test/java/com/echoagent/sdk/RagProjectRulesHostTest.java
      historical_git_mode: '100644'
      owner: rule.framework-runtime-authority
      source_role: test_consumer
      observed_revision: 47579c895657c182eceacfbb63865b04ec55a4a1
      issue_id: ccc3d842f7d781797dfb0ee495ff633ee30c846a8d05d5a4e0e4b16303123d2b
      owner_fingerprint: 70ec7721c16dc9e713684fb07b302c3c23c7cc0106fa07a8bd997b6458e703bd
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: e2a917b10dd677825a189b544973eaf18ea52cd014b9f12734801c169837e690
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_cede7a7cbb45f3e2
      replacement_source_ref: sdks/java/src/test/java/com/echoagent/sdk/RagProjectRulesHostTest.java
      replacement_content_sha256: fb9e1525bc9531794ab701714d9c3d842173d9c83e3dda80b7fe99c1fb7e472e
      replacement_git_mode: '100644'
    - raw_reference: docs/sdk/governance/evidence/sdk-java-default-root-repair/after/sdks/java/src/test/java/com/echoagent/sdk/ToolValueEffectHostTest.java
      historical_git_mode: '100644'
      owner: rule.framework-runtime-authority
      source_role: test_consumer
      observed_revision: 47579c895657c182eceacfbb63865b04ec55a4a1
      issue_id: c15a06424b801f879d0cfd030be25b9a52db54920cee31bb575646687c677b3e
      owner_fingerprint: 70ec7721c16dc9e713684fb07b302c3c23c7cc0106fa07a8bd997b6458e703bd
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 461c613be19862bacecf7e238fa0d3e80c1e98127a11279999f36453d1bb2d5b
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_cede7a7cbb45f3e2
      replacement_source_ref: sdks/java/src/test/java/com/echoagent/sdk/ToolValueEffectHostTest.java
      replacement_content_sha256: 5ea14c6727793b1ddeb100b3c479ac7d732dbc6285012f9d87261100c0f96a6c
      replacement_git_mode: '100644'
    - raw_reference: docs/sdk/governance/evidence/sdk-java-default-root-repair/before/sdks/java/src/test/java/com/echoagent/sdk/ChannelHandlerForwardHostTest.java
      historical_git_mode: '100644'
      owner: rule.framework-runtime-authority
      source_role: test_consumer
      observed_revision: 47579c895657c182eceacfbb63865b04ec55a4a1
      issue_id: 5d746ed1f437465dd712cf0251b6dbc7bc75d27fc13e9c99f8b7540994372675
      owner_fingerprint: 70ec7721c16dc9e713684fb07b302c3c23c7cc0106fa07a8bd997b6458e703bd
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 586b14a1d8fba4155cd90ed9d1ce9aa4d13242cae30546ed0f9f8cbf194cbd34
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_cede7a7cbb45f3e2
      replacement_source_ref: sdks/java/src/test/java/com/echoagent/sdk/ChannelHandlerForwardHostTest.java
      replacement_content_sha256: 468f369c7a635ff3d2178794f6fe791bf7eea41bd9eca110f6ba394bfe9f533c
      replacement_git_mode: '100644'
    - raw_reference: docs/sdk/governance/evidence/sdk-java-default-root-repair/before/sdks/java/src/test/java/com/echoagent/sdk/EvalImproveTopologyHostTest.java
      historical_git_mode: '100644'
      owner: rule.framework-runtime-authority
      source_role: test_consumer
      observed_revision: 47579c895657c182eceacfbb63865b04ec55a4a1
      issue_id: b7b8fa31a4de02a787ec9dfde1e489c1e9530c1d3bafdd95e0d9523833f65971
      owner_fingerprint: 70ec7721c16dc9e713684fb07b302c3c23c7cc0106fa07a8bd997b6458e703bd
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 9d72e0891afc40d063557475a0878ebd29eeee739395bbe6509cee45b935ae8c
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_cede7a7cbb45f3e2
      replacement_source_ref: sdks/java/src/test/java/com/echoagent/sdk/EvalImproveTopologyHostTest.java
      replacement_content_sha256: a12a785f80302f0104e47eaeadb7e31919a0f228023ec64ed7b142d173e37a69
      replacement_git_mode: '100644'
    - raw_reference: docs/sdk/governance/evidence/sdk-java-default-root-repair/before/sdks/java/src/test/java/com/echoagent/sdk/LlmWorkflowStreamHostTest.java
      historical_git_mode: '100644'
      owner: rule.framework-runtime-authority
      source_role: test_consumer
      observed_revision: 47579c895657c182eceacfbb63865b04ec55a4a1
      issue_id: f4f2bb7e66dd7d72b42d066360b427c8b3d184b305c3f9fc3ba0f9628eec5195
      owner_fingerprint: 70ec7721c16dc9e713684fb07b302c3c23c7cc0106fa07a8bd997b6458e703bd
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 468e8de51ed7fa5d18b909d3a44af44479957b66e503e425327e58c2e8652f59
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_cede7a7cbb45f3e2
      replacement_source_ref: sdks/java/src/test/java/com/echoagent/sdk/LlmWorkflowStreamHostTest.java
      replacement_content_sha256: ddd4827d47ff0329039ad11fc1f0241f9185647fdb37db06a122112d0f38b301
      replacement_git_mode: '100644'
    - raw_reference: docs/sdk/governance/evidence/sdk-java-default-root-repair/before/sdks/java/src/test/java/com/echoagent/sdk/ManagedRetirementHostTest.java
      historical_git_mode: '100644'
      owner: rule.framework-runtime-authority
      source_role: test_consumer
      observed_revision: 47579c895657c182eceacfbb63865b04ec55a4a1
      issue_id: db4f86a86e4b8a44027c9a868df37c194fd96ea1f976d753e8b581237f4ee31c
      owner_fingerprint: 70ec7721c16dc9e713684fb07b302c3c23c7cc0106fa07a8bd997b6458e703bd
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 6c103962c7f98f6b620923c082f9dc1b1792c99c7d39019f5999697ec00577db
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_cede7a7cbb45f3e2
      replacement_source_ref: sdks/java/src/test/java/com/echoagent/sdk/ManagedRetirementHostTest.java
      replacement_content_sha256: b635517906e348deeadf19d5849a6a51a873654ba18648e258e2b342d452e695
      replacement_git_mode: '100644'
    - raw_reference: docs/sdk/governance/evidence/sdk-java-default-root-repair/before/sdks/java/src/test/java/com/echoagent/sdk/RagProjectRulesHostTest.java
      historical_git_mode: '100644'
      owner: rule.framework-runtime-authority
      source_role: test_consumer
      observed_revision: 47579c895657c182eceacfbb63865b04ec55a4a1
      issue_id: abd15496385a81c34410ea842f05b364a7d3d68ab127b588fd1e281fdc239e8a
      owner_fingerprint: 70ec7721c16dc9e713684fb07b302c3c23c7cc0106fa07a8bd997b6458e703bd
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: d45befd9fc709964ee1d90016f810b1b6528b2c8bcabeadaea5691f5a8e1acdb
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_cede7a7cbb45f3e2
      replacement_source_ref: sdks/java/src/test/java/com/echoagent/sdk/RagProjectRulesHostTest.java
      replacement_content_sha256: fb9e1525bc9531794ab701714d9c3d842173d9c83e3dda80b7fe99c1fb7e472e
      replacement_git_mode: '100644'
    - raw_reference: docs/sdk/governance/evidence/sdk-java-default-root-repair/before/sdks/java/src/test/java/com/echoagent/sdk/ToolValueEffectHostTest.java
      historical_git_mode: '100644'
      owner: rule.framework-runtime-authority
      source_role: test_consumer
      observed_revision: 47579c895657c182eceacfbb63865b04ec55a4a1
      issue_id: e206abe83ed3ec739cd8ee912348a31c7457916d8533924985a85659ca60f22d
      owner_fingerprint: 70ec7721c16dc9e713684fb07b302c3c23c7cc0106fa07a8bd997b6458e703bd
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: da75ee81df5afc5ae911e04bfe297bbbcf9ca58bd49b5f1c02b29f0d4b8a9785
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_cede7a7cbb45f3e2
      replacement_source_ref: sdks/java/src/test/java/com/echoagent/sdk/ToolValueEffectHostTest.java
      replacement_content_sha256: 5ea14c6727793b1ddeb100b3c479ac7d732dbc6285012f9d87261100c0f96a6c
      replacement_git_mode: '100644'
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/before-feature-wiring/echo-sdk-host/tests/core_profile_e2e.rs
      historical_git_mode: '100644'
      owner: rule.framework-runtime-authority
      source_role: test_consumer
      observed_revision: 47579c895657c182eceacfbb63865b04ec55a4a1
      issue_id: df36af75788f783898f330800834a4c1fbdecabf8b09724d68d0d158cd97ac38
      owner_fingerprint: 70ec7721c16dc9e713684fb07b302c3c23c7cc0106fa07a8bd997b6458e703bd
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 8fdd3f2a55e60193feec436bbf4af235bbc9e146e1ca77b9632fb9e25977d709
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_cede7a7cbb45f3e2
      replacement_source_ref: echo-sdk-host/tests/core_profile_e2e.rs
      replacement_content_sha256: 2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c
      replacement_git_mode: '100644'
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/feature-wiring-current/echo-sdk-host/tests/core_profile_e2e.rs
      historical_git_mode: '100644'
      owner: rule.framework-runtime-authority
      source_role: test_consumer
      observed_revision: 47579c895657c182eceacfbb63865b04ec55a4a1
      issue_id: 43692fa2f36fe5d742fc795f773c5a4f36a0733b34b22517bf5878e72c14a825
      owner_fingerprint: 70ec7721c16dc9e713684fb07b302c3c23c7cc0106fa07a8bd997b6458e703bd
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: be8ea7a559a273781c49f25c1cf7b0f3b05f8fd32ddb875b447d944129cbf5ac
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_cede7a7cbb45f3e2
      replacement_source_ref: echo-sdk-host/tests/core_profile_e2e.rs
      replacement_content_sha256: 2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c
      replacement_git_mode: '100644'
  evidence.sdk-accepted-source-continuity-gate:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 5a54c66d9932ed8ff2d265a42902131845a6040ecaaf09c90591a2ca47ae68d3
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-ack-recovery-verification:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 57fab001807d0f7ecd23abad1d986229b26dfafdb96cda448a25ab96b2934de6
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-acp-agent-run-finite-observation:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: f47d24b7189e8e726f52d37e7278e4d848808bbd2b16ed88bfea73021c5cbb6d
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-be5-current-rust14:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: da4f0536ea52cd4e32d8b6cbd11c86513d1abf5320a2304e8da47a0d38d68e09
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-be5-dependency-retry-current:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 523db324350098b2fcf6083d466b9802841d7a878c908dcc445a880e055288d9
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-be5-official-full-language:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 106eeff3a629613d1fe760e48487a5ae05884dd510bac8e90ce519c75c99dca8
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-be5-shared-chart61-ci:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: a3b5d5807897e46e8c230611c1abe36ed8480ad3e5683aed6d765fefc636ca07
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-candidate-behavior-verification:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: d407f96a6d7dc5c9566a804a39e82d6813784eff0c3bea0f736c96b9c84a391a
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-channel-cooperative-two:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 889b44b300f593aa4460ccb8672af9544e0083616e79efcb0c17a82d20039ed4
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-channel-forward-human-scope:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 222ab5b6072e94dbfc1672550a2663b5e5629f25b8ab1f32fe820054aa734ce8
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-channel-message-handler-consumer-audit:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 9fa21bbc9f1bbeec872789115cba08bc2a348b1e69b560b3450e858a67c30962
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-channel-resource-admission:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 030e89e620eb0ac75c8c7f66a496f8d630123d412c43cd8af7950d0eddcfe1a3
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-compatibility-gates:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 9c37ba670eb2c4cc0438baebaf7b34087181edcdf0c8b0b181d3fd0a337fd652
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-computation-45-candidate:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 758f71cc9c4da700d16f9c30a76a130e1f6595c2319e46125340f9b80e4faea8
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-computation-50:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: e543602ce570999e81af1fe2ebfeafd811eb7b9e484dfe1b066e48c0ccdf2a29
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-current-reviewed-rust-gates:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: b45ee28c90cef3812aec9f58f0fa0ec3a6f3ee87eacbb5a9ff7fa56aca1c3b4f
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-deferred-product-disposition:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: ba9828d0a17e8493ee3dcf7239fde0f115dad9383816573b14165fe7ef79e1ba
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-dependency-duplicate-human-decision:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: fb71171f938d91b9eb3714ffd1e4f07de851caa2c6bb08c91db51a2eaaef2275
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-dependency-duplicate-policy-implementation:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 6e58dea719f319f2b3ea4d0d585df611002491dbc03b3d383b04a1b5af21c173
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-dependency-duplicate-policy-verification:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 144efed87fa9dd2c5dd10ba3d0deb823be1f4a2118d8009eb13eb0566e6a248a
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-eval-improve-topology:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: eb63178e83d62b553fd5b5034b18a9a1cf6b65cf7b6997850bd141291f92dc18
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-events-resources-finite-observation:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 0af302d40d9eb5882ca77adf3ac6c1b326b4c0f10d1e86e347ea9a99033a4ff7
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-extension-teardown-prefix-verification:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 3359c56bf04d95cabf3d9cbf448d0c515a2bc0d49514a34d114fa47505648099
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-external-timeout-oracle:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: c1c5a56b4aabbfd3649ecb2e5cf5217c1814f679804f14287220729c9339bcd0
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-factory-child-settlement-repair:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 3bec253476714ca0b4e5e8ffdea27b9a1d49b4aa1856da29bb5b37bc2122dfc9
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-factory-child-settlement-verification:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 889e0db906e8d34731abd3b87b7ed703c1f6d41314000b5fac90e0c241e99374
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-family-catalog-method-routing-candidate:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: c58417499e6f161841303be2b266f9e7bf104dbbdc39ca52d3ebf44965833111
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-feature-wiring-verification:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: a2b3c925d88164fb5180f628362d5997f66274b47f17b520c17ddea93ed98b75
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-framework-pin:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: ef3f8c91b546e7b2beecc84c1af9bbb20bd8d47449592c39680de3db75f83bf4
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-gap-watermark-host-verification:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 15afc174f202233780de11962fa4b3c6141b59a98609f0b86ec5fa4fe852ae62
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-git-published-consumer-verification:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: e45233d047454b216076b6da34b9a6478d9d21216cdfaeac39415267e6e9295d
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-host-pid-publication-oracle:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 9c79e3089d0982d6e9d4f287849a994586bdde349821b19314fb1e943239d90e
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-integrations-finite-observation:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 4e61f4aec047974255bad7b2347ab9ebed75845488fadfbe4d8df212e4d5ebaf
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-java-host-early-exit-verification:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 894671c7b6a1ae862d34cf5630dc8f11da3610e6d16d9e3521bcc263c4a2ede2
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-java-stdio-close-specific-closure:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 95de9f895f9f605f4a18cb61e77b0b020d49ce1bf39d9e44c2d3c8660cf9fad7
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-language-client-lifecycle-current:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 2ce8a6eea087060208c58ae2af44e8dffe69b5186bedfc278650e269e8e6a1ad
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-language-current-first-gate-failure:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 2079a5626ac5d7b1d342751aeb7ee69d7c007783b4d96f2d3c009923d57769a6
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-llm-workflow-stream:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: b7707fddc019e7b328833bc5a2d8241d1a0dc5e36ac9fc839fcb9f7d1de6eadb
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-managed-retirement:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 0c698f45a089bc1691d4a367688cb7ee402bf91b9415ba5d2081045b3054e784
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-managed-supporting-operation-boundary:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 3ab3200f15ef1839689ba7ecf17a5dca78cad6c80048b9192eb4bc8d98381f86
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-mcp-channel-owned-candidate:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: acd704c9a0570630049f375313226a2e6bb7ddfb2169c10c44d3ff95b398c647
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-memory-permission-candidate:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: ca27978a3e7803e0de19ad4c5e2ae5975452d640573805559df169ffcce89422
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-official-current-language-gate:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 7a20a353de86b5abe97e9b08d6fd171740c9cfe5be9aa6c07d37da0b486305cb
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-official-factory-final-language-gate:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: ad0cf08caae5cb7ac9bcd0f7cbff9a0f7e647e96be45a7678d9e643655b36b3c
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-ordinary-tools-b909-verification:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 84d1d031a2dfdc6da2646633eb860e89f52d222f87d56f13bb8379b3a8fed22d
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-parent-driver-candidate-verification:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: d6ce47c3c9b727bc5d7ac8a570ecdfd417d5f856702f8a2fe32eddf125d2714f
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-protocol-finite-observation:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 6a376bae400d8a4ffc573f3ab956ce521220010ac37d3de0ea91fb550bd7301b
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-python-typed-component-stream-verification:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 72964389d72390f98ec1ad884cca153aaf6c8063c99e058431f8a011a3744bf8
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-rag-project-rules:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: ca58b35ffcbdbbc63490e7e8c825a32a250c0d4b429e33318e3c62c47f4f9d53
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-rag-scalar-verification:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: f3726189acdb55e5f4d9c27da8187ee3f4b2961493710f7b92986a390088af53
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-real-permission-default-ttl:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: fc4add24262abb6f5549db06476c99a076d7b2a5b287140c21a9df934bfb87bc
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-rust-workspace-factory-gate-failure:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: d47093317d45719a134830239fdd6308f5fa7fe7c2d0be06bacc1393a88bf3e2
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-sandbox-natural-eof-verification:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 988d0696d0a1faab8152711e70f51250c55442971d974145d8dd1f67743da586
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-search-sandbox-06bb:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: afe0e039e619d0b106d70f503fb0d4d36d3229d55d17e58a2996d4812dd23882
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-source-continuity:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: b51ad6bb9aeb9e2b52d305dc9f93542334b7cc50a3eef378640935a7e17211b8
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-source-import:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 8b1256d5008dfc8affeb719e87b7b3fa757fa33f42d9ea5ce9fa1f1b5c3cf42f
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-state-trace-delivery-candidate:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 50fe94a21cb83f2f18560298f4c5904e8a915c574aaa86f4151ba3b61c95eafe
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-stream-cancellation-routing-repair:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 7981df866366c6de8e8eb2c5a2ab304482e2222d5ed4315396348c5ca44e5746
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-stream-cancellation-routing-verification:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 04b1b27b1eedfe814fb0e5a723aeac2e275ce9212ddf9466653938f89a79b5e2
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-stream-producer-verification:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: ee7e4dabe0d35e2aa60593ded9b937602a5a9b42475b6d830ad277099fcaff65
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-task-context-direct-getter-consumption:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: d8ebbb2ed3b84c77eb81692a98a4dd0f27b25cf70a37b2923f9fb7e0cdf7b7ae
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-task-memory-workflow-finite-observation:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 644c9b590400a35614836e777d33880cb12e2cd4c01d638fc1e10bd62963ff93
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-task-subagent-candidate:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 3e59ba66ee6f098e99e788c6c0dce607df267f075a9be9e31ab81a16c8f8bc53
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-testing-value-candidate:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 871ba0e7ce9033e1c5705ab7e43bae181ab2c12b541373934441337609681753
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-tool-behavior-candidate:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 5e237087df6ad9f6775c4fdb3c269692bc187b1b2d076f2330ea678c49614536
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-tool-input-validation-candidate:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 4a55f5b55ae8b364df8314d40293c63620e75fd885df13c120af78e98426faf3
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-tool-value-natural-eof-after-candidate:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: c7d2fa760b0120b3a468aa8f7f302e34e211f53ba0adc49f61628115fa23321e
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-tools-bridge-finite-observation:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 9f6d853995ad25960296ef71263e855d654a9d4c020c4110b4b0879d678a3229
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-typescript-stdio-close-specific-closure:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 2bba90f70fa23e83e9b4235c040ee158e68967c78035cd1c7769b0b931c8e70d
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-wave2-cutover-final-verification:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 95fb73e85997a9111d90c07b478c172136c2132b31d0eef8c33abd11236eb4c0
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-wave2-cutover-repair:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 483704e2b65266641ebf86c7133d095a2286ae26231fbe740c3eb9b0fe67f3c0
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-workflow-successor-candidate:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: e5663f2a2636cf98939332484afef868789fdff1e0c5422ce8c44cdb62d636df
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-accepted-source-continuity-gate#source:0a7c7cc96b697bddef9af3fd8405e27458c25e029f7b50ba10860b84177e52ef:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 133e6d708909d403502630eebd04a0a31cbd816515ccf2385c0ac25835075e69
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-accepted-source-continuity-gate#source:11b892190a692cabc3371246d40447fcd102f026ffc48e22a1db00858a30cfa4:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 260ed4c5ba9aeaadddc8da5bd17850d8a25c95b076a574c24e4325a5094c3cd7
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-accepted-source-continuity-gate#source:66a681b912b91c36122cb774e75a2e8d6a0987fceab0a041560cc4cc92f184e9:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: e949c920ce5576c83186c50f62e845ec4462733496b34f11519bbc4bc11b39d8
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-accepted-source-continuity-gate#source:dae1b1b89059019d6be6171917a8e6d7f8ddf44a8fad15e0a41dddd1b6ac038b:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 4e47d3e69bdf80f8ea1255ea56ad57cb1d234b6eb9e8a11e2df7cf47a824225b
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-accepted-source-continuity-gate#source:cb3719eca051f3be958e8ee2c8585887fd4b3be3b8ed4166a1bc320bb9dca434:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 312037ed20d9b5356df13e46615766156e09418aed79152d3c98c6b45f8d4954
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-accepted-source-continuity-gate#source:b58b08b799cfd1d3dcc7d1e3f74e0532f337a38a850e515f9d9fce1fcac292ca:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: cabff225a11436e7097d7dde097895d01440b1fa7d51c54fba6ec83be8374631
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-accepted-source-continuity-gate#source:c8d490eb539cbbc8bfd382679c55236023ef0a77a674097677af5fc75372394a:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 10bc1ad4924b8e6196f9500bbaf412d39b81eaf89b8873c6c72f5a01c7ea2a96
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-accepted-source-continuity-gate#source:83ae0ba3deaeca9ca2102092c9cd77dc334814775811ec5a8564af5a68b976f6:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 7083616c3e494b1c6231e288e3c98220f2e115e477872ff308336d37d097cd51
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-ack-recovery-verification#source:594ab84a723a1b513ec1939b99a00a745c698396807110c32475ebab83571c3e:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: c1192c90d5a26671215cfdc0f9c75e9f166b8050c3486235144815227ca6e9d2
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-ack-recovery-verification#source:d9d57f5234c5f6478af52ad529fd9f2ce0bfdfb354580be8de93080597304f72:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 10f4ce04a16726144f9b8d8d7796ac271612a2c53f44cd0dce2d10a050dcfa15
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-ack-recovery-verification#source:d44e9ff012033e928933fa3afd158c8e38267ac29e35c72d7d2d5f1dd0c9d93e:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 0a9931d87a5913a841852c7c90d922407b4a8b3d6856ae601029a9f5b9191d0e
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-ack-recovery-verification#source:707034d09fed4cc7cb0760204e9745252ed18a8267bb026cf22020c726bd2c38:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: b27de72e8352e88461018dc0313dd28b441c892d3c760bc16a08c395a8e6e417
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-ack-recovery-windows/review-final-v4-source-manifest.json
      historical_git_mode: '100644'
      owner: evidence.sdk-ack-recovery-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 1d41ae532d0a69ba866e57a053d515b2fe0fc2219972205d2d6dec80e8be7587
      owner_fingerprint: 57fab001807d0f7ecd23abad1d986229b26dfafdb96cda448a25ab96b2934de6
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 707034d09fed4cc7cb0760204e9745252ed18a8267bb026cf22020c726bd2c38
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_6c759c7ac3125833
  evidence.sdk-ack-recovery-verification#source:cf9fce4fc0ce6daf46ecf60bed34827dab47b6703e44903dfdce03ff0d063770:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 2a9f2cb314a714bed4d0b73cdd6067bf9a25980b5a8102bd1b2b970951152cf4
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-ack-recovery-windows/evidence-sha256.json
      historical_git_mode: '100644'
      owner: evidence.sdk-ack-recovery-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 0582235e701f0ce68297473a2f93db09f9095dd2139f2d51abf642ea2a80d85d
      owner_fingerprint: 57fab001807d0f7ecd23abad1d986229b26dfafdb96cda448a25ab96b2934de6
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: cf9fce4fc0ce6daf46ecf60bed34827dab47b6703e44903dfdce03ff0d063770
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_9b96b080d63e9625
  evidence.sdk-ack-recovery-verification#source:641b7205e5b43ea6f054886b236c926917ca3dfd08e4344ff83d51be7ac442f5:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 3f804caab3c4a5516165104b3d9da6f254f51ef5dfee9d04451b7315e26a5651
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-ack-recovery-windows/REVIEW-DELTA.md
      historical_git_mode: '100644'
      owner: evidence.sdk-ack-recovery-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: da5b30840e3f836569ac0ea6011b8f3794f537c3e292a85af060b49e08663765
      owner_fingerprint: 57fab001807d0f7ecd23abad1d986229b26dfafdb96cda448a25ab96b2934de6
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 641b7205e5b43ea6f054886b236c926917ca3dfd08e4344ff83d51be7ac442f5
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_0267b9870337ecff
  evidence.sdk-ack-recovery-verification#source:184e9372ef6a6c69d1f56555542fd0a96a1a43b995650ebeb73a3d5dd7aabb12:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 12bb41e2ace37a0b1d4a0ebb4d057def03f3be4f08025e718adab0b447c50272
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-successor-model-reviews/ack-exact-slice.json
      historical_git_mode: '100644'
      owner: evidence.sdk-ack-recovery-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 4965111db32bc7e21605d82e3277ca9e9444c10e2f40381261a9b8ef41f6eb0e
      owner_fingerprint: 57fab001807d0f7ecd23abad1d986229b26dfafdb96cda448a25ab96b2934de6
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 184e9372ef6a6c69d1f56555542fd0a96a1a43b995650ebeb73a3d5dd7aabb12
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_3566e9eafa3ae4bb
  evidence.sdk-ack-recovery-verification#source:9824cc09e892d3f70b2e55494160ee52e52ca24270b07336f9e4f790ea9397ce:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: faa04fa7298699283ce9bf97d8c27a9ee39c4e1fda3ff5c8eca1e25ada774795
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/final-receipt.json
      historical_git_mode: '100644'
      owner: evidence.sdk-ack-recovery-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 6b8b47926a77e63edd460c5d60307b16788abea70048ed171502bb62303d041d
      owner_fingerprint: 57fab001807d0f7ecd23abad1d986229b26dfafdb96cda448a25ab96b2934de6
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 9824cc09e892d3f70b2e55494160ee52e52ca24270b07336f9e4f790ea9397ce
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_8796007fc82a4ac5
  evidence.sdk-acp-agent-run-finite-observation#source:8b89f6d86577e7cd4dc22b0ca20a7642e2e1e6b57ce50b3d4a8c7ce0d8948106:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 3bb60a983084d617e8d747e0d4713d80c40df78393a7e85f37f25f1114fa0dbc
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-acp-agent-run-finite-observation#source:8e3eb3592334cef9dae9bcc6787820528fc706c8b1276387e1e973d727f0eb80:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 7469d7bc10266a2ba90d6ea3a7e3809e63ee5826313b5a1e439a7d100f60ad4a
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-acp-agent-run-finite-observation#source:f9450c38c7cc599f6b3dcd26265aa3953f8dbe6c52c42ef5d12b1c058970dfc9:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 7d10dbcd2f710c7909f0d5e8ed9c319d121cc65d5a80f422f00064452d61881a
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-acp-agent-run-finite-observation#source:1df6f4ee816620a21f01aee52007ad36838f28397678e8cb1520f764e5dbb829:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 7faa1adcf1dae1bb34a696c750290f9996716c0a3a3f1f42bdfc5cdf5e8591e1
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-acp-agent-run-finite-observation#source:e131732ef1f6be04754dbe6291988e4ec052ea8d925a15aeaac442dd1fa4918a:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: fe6036286fa7284dbb41d61103e925216c9fa84281013c614412c509afab9ce1
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-acp-agent-run-finite-observation#source:43f9320d0e552f17116d4e6545c4ea6d4eb39eda636ecc3ad0772779de8d2f54:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 2ff13a51416724264e3cb25b241ff66e427da65faf06a8e70c4c48c68c10fc83
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-acp-agent-run-finite-observation#source:3a26d5b2c89773ada31776c5f23bc208b775c5201662d1ece1543c797f5562d1:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 7478bba719ab02ccfa9dffe235456cca89f245002958496ffd98b04dafdb0193
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-acp-agent-run-finite-observation#source:a203145052e2db2c389afaaf8c02ce3eb8041b9239f42f0388a37d89d36042cc:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: bb36e8ea2420a8a58622d052399003074132da061dec3cebcb4547c2edb5f551
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-acp-agent-run-finite-observation#source:7b8bb77da405dd0af92cdcf7e9fc897907988910a960ec0f70cbe227a2a997e0:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: f1b61ca594e134c63fd96210fd713d6c858faaf4e6198377074c3c21e4cfe000
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-acp-agent-run-finite-observation#source:d1658d4c77694b7f160d2c51874a72a4570cdf649e16a9c212de71d1bb2c0a8e:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: c79caf3300979f367214ff6cd94af1118e8402df6bdca073961d12dde2c6113e
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-acp-agent-run-finite-observation#source:c8b9a96001874d78a4b86b4446219c82e3ead0d7281154ffbbf42702f10b6cac:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: dab66c56ada29388d7b1d56ff215892f7a253058cdef5d37d333b06c932dc863
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-acp-agent-run-finite-observation#source:825677ad5fffe4f5aa13f41437de363a28187eb44c01d7ecd1d70f20985f592d:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 0bb7968f4139f866266799bfc6597a9a4628c82bf5a9d7a89453af5ac8a408fa
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-acp-agent-run-finite-observation#source:7bf353d0f72943801b3794a671c1de8d394da0cf23cbf5b83c9183a7bdc1457b:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 7a20e471b1ccdac25afef11ce71f450f00eb9b438c7b6a31a0352dc3f486ec2b
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-acp-agent-run-finite-observation#source:f83d2c07e89040ce7b97fbefa3a94d23d0f6443a073363ebbc59761de1d09a82:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 83eee560f7849abc363210bf291242e6c17d8d5b365d53c572147956e7676a0f
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-acp-agent-run-finite-observation#source:3c8f9c808703b1ea490361699cfe438fae73d8e5f1c226e13e67647adcb3ffd6:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: dc63cad65440222d5e4e4925d8d565df2543da313f7f163cf14af683b9785f23
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-acp-agent-run-finite-observation#source:8654ec5acd9d30f00275870be52bbfbf0481a1c2a02f7d16bfd75dd5c6774567:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: d4d46357ddb2e9d282fb79112ca6ac6e64923f37821f965a584b67f153460adc
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-acp-agent-run-finite-observation#source:078e338b81c674e4e3bc272f594e8b4b2a9e65dcee899974e07b7d009d5cdf23:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: e3cbc9a3b6880fbc9dc132c5287d199550a2b959f4498d8724ee6fc40d5d122c
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-be5-current-rust14#source:9cfb12ed85896a15f41d59ad944ca2586e6c5f28928162f2c947bdd5f5d31ec6:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: e3142ccf8c727515185fa86e906f8afad7cb2abeed4592f20c148c0e8b1b39b6
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-be5-current-rust14#source:a2f9bcd7158a5c7484d9637e70fa69381b491ee54f0e4fbeee491c5e19057dab:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: a2cd84587a96ce5201ba076035cc3a46b6b6009811f434715cdb9bca8824a6ea
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-be5-current-rust14#source:755880a7b97af7fbb32225303dac333dba26e309256044404f83270675143000:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 3db33604774798a2ac9c01ba6b930b0aee9a2d95067caae19e0c510ff1a2e114
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-be5-current-rust14#source:bacfee37ccb85be2b9150d90ddac3f5145633bf0e3fa552fdec65157c4ec10a2:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 87225538bc45a1189bd1d01b81019366bf2d96638a24141a23a529055baff209
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-be5-dependency-retry-current#source:f6989b858bb48c856646895514bfb7369fe05950d0059aa7520ee0a99727b35b:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 9d05f373e6b0cab725383622a836586996680ead66d256d2bc5bc828ec472908
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-be5-dependency-retry-current#source:a6847465e1c376d23b9b27d4c2a561db89c8f42793f5e57a7cca4c2330db6d0f:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: ebd590233f4a8b52bc8a8de130cf51ab2fd1e60abf5bc38b85753593fea8dc64
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-be5-dependency-retry-current#source:1bd944b2ed88c059f9a549274595d3d8e92ef72aa4a82ef5efbc80b6299433a3:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 67bba3fb32aaa820207d774c06fd99bde71d5f5154b53e407579e1ed7a76321a
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-be5-dependency-retry-current#source:9ddce822ba9e9564caf7be03cbc89e3effa0237ca3dff96802ed69978d020bfb:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 3bab72f4c98b0029d045101a6dca8c34a5d6a2063e44a43f09755c581dccfe8f
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-be5-dependency-retry-current#source:5060d52483123e31eadf11752576901d2a79906db0f1502ddee250a5aa32cb1c:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 948179f154011e68dfc20dcbc4f91fd73e7aada5e86aad413f8ef7e19636ce31
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-be5-dependency-retry-current#source:807e5d241ffdc25c102a722459e125e1ce88ab47e7bdfb1976c715d5ec0eecfa:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: d6d9d8910a8e0c06a39394d3d0deb3bf252ced12ad4bf59714fed35f8b3c3af3
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-be5-dependency-retry-current#source:e0a6b7435471ec33e6c1cce7b6d20c2e0ca592dd45c944e098f19a6503405127:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 4b1525a1fb9a0bcee915c6818dae2f9a2a3b59ff8195fb22d5bb30ede20f7c19
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-be5-dependency-retry-current#source:e322493c219111046d28ed599f4a4512c141f86f8cbb41529f1debbab02e1e7b:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 0b2f182f6aeb504f732948d68cbf46e20a52507969f72cd0cc805758816879ac
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-be5-official-full-language#source:b410e34bb8154ba7d57a4906b9a6f4faedf01424d9d3d23537014fc37e702023:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 6bcfda405669f31428559e67fcff674fb02f91ebc048fc41c47011fdde87f2a5
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-be5-official-full-language#source:a42d7ce654bcf111c6574a217cfce071c5e8e22b5d77e7cce5fac69b95f3dc28:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: f34da5a8e9b71f04c4812b4e52185be19d21838725e469686901c5d924b5f62d
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-be5-official-full-language#source:77f9cf88ab2763e49de136c699a5d350326478eaba3b94e5b0f6068023bcaba9:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 0eee8f1037f6a61352da37a2374672e992ec6662c43dbdb0a751e171ec7ea67b
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-be5-official-full-language#source:32f8af7298606c8cddc28c5edc4d3237241c28f1d336145759c778a6b3c708d5:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: fd756867c3995bb477dacbd28a8506a929c55d63e5534816cb2e3f33f24fde2b
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-be5-official-full-language#source:9263d5e8d6bcf8bd56a905bb0e0e4dc860baeeade99993c26e2a415f2984654e:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 347f5c2825cbb5a49bdc6996358ea97dd46397e16ffc3958a059de104c916068
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-be5-official-full-language#source:07159dd340ec9f648bd84733403894b6b9231b4716fc185fcf64561b3ac617d3:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: f51db562ac42a23e028210d88e56465632f21031f7f35cc2efc3a21285ed809e
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-be5-official-full-language#source:bacfee37ccb85be2b9150d90ddac3f5145633bf0e3fa552fdec65157c4ec10a2:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 87225538bc45a1189bd1d01b81019366bf2d96638a24141a23a529055baff209
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-be5-official-full-language#source:32af764888afca7ee96ee90b232991eda1122dea13683b240541840980983226:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: ecd5b0cdec49b4ae9bba72076f3dfd5991ebab58ea4e88ec32e9a22efb641c2a
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-be5-shared-chart61-ci#source:abd91af72604ad5d8d10f12c4c3c9d1f479d0ae57eb88dc66760f4c62749d06c:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 36e113256f81f5f11fa53ee3c26070f8fb5cb2af008e285717aa9807738e09df
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-be5-shared-chart61-ci#source:b2b86d6e7d59855ffb2e0758804a9f2533559bf62ad38001ca35bd1768a5b595:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: c135265ed8870f29682b0a4404b084dc5908df072e7e8eba1c85de2a37c05ecd
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-be5-shared-chart61-ci#source:7295cd6af46b35a8030881df30a6f71c484820ccd8ab94816b0a649423524874:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 683d475ecdd1af00026259072c6db57ec6644cff46792ad61bcd3dbbab3e44eb
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-be5-shared-chart61-ci#source:fb67dd024e218f13beeb9eb5e6a5e0a7c2415be65c3accd9fe7413af3d8fc007:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 755f1ee2e9cf88b294f2e90f18106d1531a2021daa640157d1972907d43a7cff
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-candidate-behavior-verification#source:bc4d51253476df739969e4803f4cc854315ee335914191f698021791af5fc1b2:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: c41eef949b47e9c8a68dd207fac326487d7ad401006f90380050268e5e3d1f11
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-fc63-historical-20261013/files/evidence.sdk-candidate-behavior-verification.original.txt
      historical_git_mode: '100644'
      owner: evidence.sdk-candidate-behavior-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 00cacb1d332c5b1b39addee512feffe3782b0d8323a76435ed9debee116a50ff
      owner_fingerprint: d407f96a6d7dc5c9566a804a39e82d6813784eff0c3bea0f736c96b9c84a391a
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: bc4d51253476df739969e4803f4cc854315ee335914191f698021791af5fc1b2
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_ed2628e612b250c3
  evidence.sdk-candidate-behavior-verification#source:70a5c4c892a7478d33677a277c037776a25fcb04e2025ce2e32654faf7e198e6:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: e760d9c50106fe421fb6848267087e93e5e7b7d06b8824e2500998d9c3def549
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-fc63-historical-20261013/archive-index.json
      historical_git_mode: '100644'
      owner: evidence.sdk-candidate-behavior-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 2930917f0e0873424894cac47665f16cde8baaeec1d00101ca3fde062b4f0ab2
      owner_fingerprint: d407f96a6d7dc5c9566a804a39e82d6813784eff0c3bea0f736c96b9c84a391a
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 70a5c4c892a7478d33677a277c037776a25fcb04e2025ce2e32654faf7e198e6
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_f1527b8f81b953e3
  evidence.sdk-candidate-behavior-verification#source:b2dce10d84bd4fa8af235b8f6ca4ba465a2197e5ad014e31215f4dec0e0b12c5:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: cba4ecefdb865dc468fbfd3244399cbd71212a007af6782e04824b8f31d7cd19
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-fc63-historical-20261013/comparison.json
      historical_git_mode: '100644'
      owner: evidence.sdk-candidate-behavior-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: b9178d64c0f5b4ff52798eefbca7fc4cbd85ed79172851e771436ed3597153b4
      owner_fingerprint: d407f96a6d7dc5c9566a804a39e82d6813784eff0c3bea0f736c96b9c84a391a
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: b2dce10d84bd4fa8af235b8f6ca4ba465a2197e5ad014e31215f4dec0e0b12c5
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_397dcf04dec104c6
  evidence.sdk-candidate-behavior-verification#source:c68a6a02a1890ffcc39f9504d4721138e9e6bce4c922404fa1b4e58c921c9728:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 0c25dda56bc0304b1811e5f7aacdb1e6aba5d19220878692773b5ef1ce25b64b
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-fc63-historical-20261013/files/sdk-family-candidate-runtime-source-freeze.json
      historical_git_mode: '100644'
      owner: evidence.sdk-candidate-behavior-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: dd5855dbd869c4427a155ab1dac798095e75a742ecec61f2956e12c595adf668
      owner_fingerprint: d407f96a6d7dc5c9566a804a39e82d6813784eff0c3bea0f736c96b9c84a391a
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: c68a6a02a1890ffcc39f9504d4721138e9e6bce4c922404fa1b4e58c921c9728
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_eed18d0737739912
  evidence.sdk-candidate-behavior-verification#source:70e3c3504c5172824e61dd3e296421e14b12d8536a99070813a1772f084cd195:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 133e9ed2932c43e3b09bd275cddcb5519f3059e62c31d7ebaee1a1d8a4e6b4ee
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-fc63-historical-20261013/files/sdk-full-frozen-language-proof.json
      historical_git_mode: '100644'
      owner: evidence.sdk-candidate-behavior-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: d822f51f1a3c2fe018707c1fafcf4e9abe8aa05e35f85b2b6dbc2426c6127410
      owner_fingerprint: d407f96a6d7dc5c9566a804a39e82d6813784eff0c3bea0f736c96b9c84a391a
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 70e3c3504c5172824e61dd3e296421e14b12d8536a99070813a1772f084cd195
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_8d74104be8fa3479
  evidence.sdk-candidate-behavior-verification#source:c4af37edd548db47b65a40378ec710015d724082e9cefb75b271e0b914a39adf:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 724dab121b7b2863e4c9a7f6482c9318ca7ca790ced4fedeb54620b98a3108af
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-fc63-historical-20261013/files/sdk-full-frozen-language-gates-corrected.log
      historical_git_mode: '100644'
      owner: evidence.sdk-candidate-behavior-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 4b4bd71300e3e91bee7d9f2cbe7b3cba259a2f41cad828a35e14b09374016eec
      owner_fingerprint: d407f96a6d7dc5c9566a804a39e82d6813784eff0c3bea0f736c96b9c84a391a
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: c4af37edd548db47b65a40378ec710015d724082e9cefb75b271e0b914a39adf
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_9911744535711d96
  evidence.sdk-candidate-behavior-verification#source:2472dc76d2cdb6fd3c4d48c60834feeda5d90421d41ebd2226e679e11bebc28e:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 4c4b334d290b637fdd0aa125cf66d9e4a126644714d38e0f2e0c480ca3153e86
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-fc63-historical-20261013/files/sdk-full-frozen-language-gates.sh
      historical_git_mode: '100644'
      owner: evidence.sdk-candidate-behavior-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: ce0556ca202d85082b17f74dc4d9e69f7c67d49891935ce6805a5bac07e2b81f
      owner_fingerprint: d407f96a6d7dc5c9566a804a39e82d6813784eff0c3bea0f736c96b9c84a391a
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 2472dc76d2cdb6fd3c4d48c60834feeda5d90421d41ebd2226e679e11bebc28e
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_e7ae812101baf0e3
  evidence.sdk-candidate-behavior-verification#source:9cc8e5d25a399b1d10611548c31a2f4fe6fd6ac96d59b3a79a2ef9f083eb73dd:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: fdb5b18c7b7c6dc4053ccccae1600aeb3353544ffbf55eda5e0454cd3728a919
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-fc63-historical-20261013/files/sdk-final-host-local-candidate.log
      historical_git_mode: '100644'
      owner: evidence.sdk-candidate-behavior-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 01381580d4156d2d7714823bfcc7f92bfc3d1815fd9fb599f45d70c67a9cce3c
      owner_fingerprint: d407f96a6d7dc5c9566a804a39e82d6813784eff0c3bea0f736c96b9c84a391a
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 9cc8e5d25a399b1d10611548c31a2f4fe6fd6ac96d59b3a79a2ef9f083eb73dd
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_49ae01b617233689
  evidence.sdk-candidate-behavior-verification#source:3279a20dc80b7a0b0a06cc9d390b73ba864c45b2b7a8b6f4de152d3c37c1b15e:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 55119660fc8780d41426f8317633f7a17b35567eaed94a24fb9a80562cf34547
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-fc63-historical-20261013/files/sdk-family-handle-host-validation.json
      historical_git_mode: '100644'
      owner: evidence.sdk-candidate-behavior-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 4d450075564ac312e384abd51194dbc2ba9d3f244424ad6f6ab3d91ceb6ef368
      owner_fingerprint: d407f96a6d7dc5c9566a804a39e82d6813784eff0c3bea0f736c96b9c84a391a
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 3279a20dc80b7a0b0a06cc9d390b73ba864c45b2b7a8b6f4de152d3c37c1b15e
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_b7503dbe0fb99bf3
  evidence.sdk-candidate-behavior-verification#source:08c2fb47e33b18c28f30f51186aeac7d9adf674d58869e6f4f97dddd042d3dbd:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 636785a8845b6dc57044e85661790a652e5d7693097a0349b70a27636a0706bc
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-fc63-historical-20261013/files/sdk-lsp-three-language-comparison.json
      historical_git_mode: '100644'
      owner: evidence.sdk-candidate-behavior-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: e00bd98e975f593b187be622e92798996adca8451a18e832a19ff9e95ef637c9
      owner_fingerprint: d407f96a6d7dc5c9566a804a39e82d6813784eff0c3bea0f736c96b9c84a391a
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 08c2fb47e33b18c28f30f51186aeac7d9adf674d58869e6f4f97dddd042d3dbd
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_2348e862fadc5c18
  evidence.sdk-candidate-behavior-verification#source:70926cc265461071a234ee6e0a73b46342d86b30092fb4dc808cdf83ecf704aa:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 7bac7682909f012f052a8ccbe973534b2d00275d364c4b4b349d86a9eef8411e
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-fc63-historical-20261013/files/sdk-lsp-language-proof.json
      historical_git_mode: '100644'
      owner: evidence.sdk-candidate-behavior-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 7e5e52fa8b07bd8ca8f44c85cae4a91eb480ec7a41b4e83a4fa95732da80bd05
      owner_fingerprint: d407f96a6d7dc5c9566a804a39e82d6813784eff0c3bea0f736c96b9c84a391a
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 70926cc265461071a234ee6e0a73b46342d86b30092fb4dc808cdf83ecf704aa
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_8ac5eb380db2e755
  evidence.sdk-candidate-behavior-verification#source:a35a14bf50d739228a649fc670ced738051fb7acf006b87393a90d0ca43a96a5:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 662c69c6a1d9a8eec90c44bb59c10681bc49cb7135492b001e8833e5e7bd3241
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-fc63-historical-20261013/files/sdk-identity-reservation-disabled-red.log
      historical_git_mode: '100644'
      owner: evidence.sdk-candidate-behavior-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 93e7bf4b777af192db1ff96d451304ab8f28387167bc911bec53be06f68b36aa
      owner_fingerprint: d407f96a6d7dc5c9566a804a39e82d6813784eff0c3bea0f736c96b9c84a391a
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: a35a14bf50d739228a649fc670ced738051fb7acf006b87393a90d0ca43a96a5
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_cbc0b4f7f0e4eee2
  evidence.sdk-candidate-behavior-verification#source:1aa6b9d52c398005d03d505471fa7385d606c68a1e2d2c9efcf65502d54f3432:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: f27a313b009b7742eb941e28857803cb48c890628cc4ee5d1d36eeda3ee91378
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-fc63-historical-20261013/files/sdk-identity-reservation-restored-full-core.log
      historical_git_mode: '100644'
      owner: evidence.sdk-candidate-behavior-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 64643bdf1657cd5fde00ffc07020b94d1d6ff432cd4ae4441cbc1e9969129bc2
      owner_fingerprint: d407f96a6d7dc5c9566a804a39e82d6813784eff0c3bea0f736c96b9c84a391a
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 1aa6b9d52c398005d03d505471fa7385d606c68a1e2d2c9efcf65502d54f3432
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_423f6647eb7b6439
  evidence.sdk-channel-cooperative-two#source:594ab84a723a1b513ec1939b99a00a745c698396807110c32475ebab83571c3e:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: c1192c90d5a26671215cfdc0f9c75e9f166b8050c3486235144815227ca6e9d2
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-channel-cooperative-two#source:d9d57f5234c5f6478af52ad529fd9f2ce0bfdfb354580be8de93080597304f72:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 10f4ce04a16726144f9b8d8d7796ac271612a2c53f44cd0dce2d10a050dcfa15
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-channel-cooperative-two#source:d44e9ff012033e928933fa3afd158c8e38267ac29e35c72d7d2d5f1dd0c9d93e:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 0a9931d87a5913a841852c7c90d922407b4a8b3d6856ae601029a9f5b9191d0e
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-channel-cooperative-two#source:911a5dcec2e774c7cbf3cddf80c46b03743b615faa6b10a4387d466d6afbbb8c:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: a8928f3a2377cbfa07a8b5cfd33e55d3aa6cd3e61353fe2c76feb35305e92b5c
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/channel-handler-forward-candidate/cooperative-close/manifest.json
      historical_git_mode: '100644'
      owner: evidence.sdk-channel-cooperative-two
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 6c60bc87ac9969014e2fbf07d5c6fe3815ad162caf4a842d1a8a3c7afe2ea968
      owner_fingerprint: 889b44b300f593aa4460ccb8672af9544e0083616e79efcb0c17a82d20039ed4
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 911a5dcec2e774c7cbf3cddf80c46b03743b615faa6b10a4387d466d6afbbb8c
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_1de2a4bb260b07de
  evidence.sdk-channel-cooperative-two#source:aeaea2dbda490a73eb3a44417e8df951a7d18b41e6020e26f525859737164e23:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 59336d1a6218631c4f34cd84a49985cf9fb8eeeb502268a0acb88873dabf6e7d
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/channel-handler-forward-candidate/cooperative-close/f852-validation/manifest.json
      historical_git_mode: '100644'
      owner: evidence.sdk-channel-cooperative-two
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 4b3168775b4f60fae4e6c6c7835179aa0bb386c1f9c5721391f0b31b54faaeb5
      owner_fingerprint: 889b44b300f593aa4460ccb8672af9544e0083616e79efcb0c17a82d20039ed4
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: aeaea2dbda490a73eb3a44417e8df951a7d18b41e6020e26f525859737164e23
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_f11f2b44502a589b
  evidence.sdk-channel-cooperative-two#source:a7d1238542f93270dbd65d51c4e2a169e00a17a714c5cff87a4a59cafbd016cf:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 59a0f94323d565444b463ab46f2ba70a6a4a1927f6015db07a0f9faf809e5e28
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-successor-model-reviews/channel-cooperative-two.json
      historical_git_mode: '100644'
      owner: evidence.sdk-channel-cooperative-two
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 49385225dc36a1cf890dc7e4f37f8991f387e01ed1344d296219218c987fe4bf
      owner_fingerprint: 889b44b300f593aa4460ccb8672af9544e0083616e79efcb0c17a82d20039ed4
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: a7d1238542f93270dbd65d51c4e2a169e00a17a714c5cff87a4a59cafbd016cf
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_f0b6d59e9e0cfd78
  evidence.sdk-channel-cooperative-two#source:9824cc09e892d3f70b2e55494160ee52e52ca24270b07336f9e4f790ea9397ce:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: faa04fa7298699283ce9bf97d8c27a9ee39c4e1fda3ff5c8eca1e25ada774795
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/final-receipt.json
      historical_git_mode: '100644'
      owner: evidence.sdk-channel-cooperative-two
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 8438b12a85345058b555cd638a51c35d0a51f69e05c219979dceb9b7710ec2dd
      owner_fingerprint: 889b44b300f593aa4460ccb8672af9544e0083616e79efcb0c17a82d20039ed4
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 9824cc09e892d3f70b2e55494160ee52e52ca24270b07336f9e4f790ea9397ce
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_aa0a7a0cd1cc64e5
  evidence.sdk-channel-forward-human-scope#source:e7056c63937197321720299959194b148669b33382128451b59c4333ab2d53e8:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 25a0072c760b898a8b1c263d901ee417cd2e6147a9714aced26069fe79d23221
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  ? evidence.sdk-channel-message-handler-consumer-audit#source:245d9daf8847d8b64fe15cf9800e7a74c7caf6c448da5576cd82161d4fc371b0
  : disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 1ab1d3fb00c1d7dd6620e4dd6f61e9f1b9385efb78f21627f0a12b58a0e85248
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  ? evidence.sdk-channel-message-handler-consumer-audit#source:594ab84a723a1b513ec1939b99a00a745c698396807110c32475ebab83571c3e
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: c1192c90d5a26671215cfdc0f9c75e9f166b8050c3486235144815227ca6e9d2
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  ? evidence.sdk-channel-message-handler-consumer-audit#source:d9d57f5234c5f6478af52ad529fd9f2ce0bfdfb354580be8de93080597304f72
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 10f4ce04a16726144f9b8d8d7796ac271612a2c53f44cd0dce2d10a050dcfa15
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  ? evidence.sdk-channel-message-handler-consumer-audit#source:d44e9ff012033e928933fa3afd158c8e38267ac29e35c72d7d2d5f1dd0c9d93e
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 0a9931d87a5913a841852c7c90d922407b4a8b3d6856ae601029a9f5b9191d0e
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  ? evidence.sdk-channel-message-handler-consumer-audit#source:9824cc09e892d3f70b2e55494160ee52e52ca24270b07336f9e4f790ea9397ce
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: faa04fa7298699283ce9bf97d8c27a9ee39c4e1fda3ff5c8eca1e25ada774795
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/final-receipt.json
      historical_git_mode: '100644'
      owner: evidence.sdk-channel-message-handler-consumer-audit
      source_role: documentation
      observed_revision: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
      issue_id: 32b91abc8c7a6dde09b296ec35910acdb3b6812d777f696f139e355f6339e213
      owner_fingerprint: 9fa21bbc9f1bbeec872789115cba08bc2a348b1e69b560b3450e858a67c30962
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 9824cc09e892d3f70b2e55494160ee52e52ca24270b07336f9e4f790ea9397ce
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_14b703eea5d18879
  ? evidence.sdk-channel-message-handler-consumer-audit#source:5e0f153848725261abf228db5320a840d3b0afef6b72f1b8f076a02cdf6eded3
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: afa85d09a95bf2e8264d8e410d8bb423c21bd3f0b0c94e420e11c074642fbdfa
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/source-build-manifest.json
      historical_git_mode: '100644'
      owner: evidence.sdk-channel-message-handler-consumer-audit
      source_role: documentation
      observed_revision: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
      issue_id: a273c5286d99957fcd276ab13c4f09d03bcde430d05eb6adbf7cc3339aef5392
      owner_fingerprint: 9fa21bbc9f1bbeec872789115cba08bc2a348b1e69b560b3450e858a67c30962
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 5e0f153848725261abf228db5320a840d3b0afef6b72f1b8f076a02cdf6eded3
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_55cd4837738a4e75
  evidence.sdk-channel-resource-admission#source:594ab84a723a1b513ec1939b99a00a745c698396807110c32475ebab83571c3e:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: c1192c90d5a26671215cfdc0f9c75e9f166b8050c3486235144815227ca6e9d2
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-channel-resource-admission#source:d9d57f5234c5f6478af52ad529fd9f2ce0bfdfb354580be8de93080597304f72:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 10f4ce04a16726144f9b8d8d7796ac271612a2c53f44cd0dce2d10a050dcfa15
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-channel-resource-admission#source:d44e9ff012033e928933fa3afd158c8e38267ac29e35c72d7d2d5f1dd0c9d93e:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 0a9931d87a5913a841852c7c90d922407b4a8b3d6856ae601029a9f5b9191d0e
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-channel-resource-admission#source:355964944fb8faf13dd66ba122d7b3eb4037cd1028e2654392a416366968c9dc:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 471ebe8214d4e71f080c9cb78f3e27a4838a067d220f9076a1c3bf7388049aca
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-mcp-channel-20261013/owned-channel/resource-admission-epoch/manifest.json
      historical_git_mode: '100644'
      owner: evidence.sdk-channel-resource-admission
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: e1a8fc2d0fa4f6dde15d57810f80d8e07ecb3b63a662a3bb415a19e3bb836400
      owner_fingerprint: 030e89e620eb0ac75c8c7f66a496f8d630123d412c43cd8af7950d0eddcfe1a3
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 355964944fb8faf13dd66ba122d7b3eb4037cd1028e2654392a416366968c9dc
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_ce7374536e3d5af0
  evidence.sdk-channel-resource-admission#source:ff82fa0da51ac2201aa1d80cd2fd69a6d3343958374b68d5279478f70fb7aec9:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: c7c2e73bcd4d0a39212b21c154ae1f424097740ba28ae6b5b8296a78261b58be
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-successor-model-reviews/channel-resource-admission.json
      historical_git_mode: '100644'
      owner: evidence.sdk-channel-resource-admission
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 59148e9e5c14b111c3a9a7ae459e9b2af827437b071030c1f23a14ff2ebf8f76
      owner_fingerprint: 030e89e620eb0ac75c8c7f66a496f8d630123d412c43cd8af7950d0eddcfe1a3
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: ff82fa0da51ac2201aa1d80cd2fd69a6d3343958374b68d5279478f70fb7aec9
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_64fa88e64571d69e
  evidence.sdk-channel-resource-admission#source:9824cc09e892d3f70b2e55494160ee52e52ca24270b07336f9e4f790ea9397ce:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: faa04fa7298699283ce9bf97d8c27a9ee39c4e1fda3ff5c8eca1e25ada774795
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/final-receipt.json
      historical_git_mode: '100644'
      owner: evidence.sdk-channel-resource-admission
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 0a9d967749266f93bdce1126732ccb26f95ffc0e371c6b9d62c0911724ae1416
      owner_fingerprint: 030e89e620eb0ac75c8c7f66a496f8d630123d412c43cd8af7950d0eddcfe1a3
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 9824cc09e892d3f70b2e55494160ee52e52ca24270b07336f9e4f790ea9397ce
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_1c8b6aaf7748382b
  evidence.sdk-compatibility-gates#source:8793b0382b493e3065829476ed7b7b7bfa6393ce1c610df03aee40f4972c6826:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 25bced96db0a8d8189f009164acbfbada6b9ee02ad985e00dac6a7bff63e7318
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-computation-45-candidate#source:8d80ad2057a12a2ba506cc5626680eb4c1fedb2bcb293bd4a4acf37245a3d9e8:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 6429eb6eba6a0941fb83155f537e155fea022be18fc0f4797da2bb99c387eee0
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-computation-45-candidate/manifest.json
      historical_git_mode: '100644'
      owner: evidence.sdk-computation-45-candidate
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 360b0137603ec45a4bcd52ac3223bf71839231d8cdb07e33a761f847070e6525
      owner_fingerprint: 758f71cc9c4da700d16f9c30a76a130e1f6595c2319e46125340f9b80e4faea8
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 8d80ad2057a12a2ba506cc5626680eb4c1fedb2bcb293bd4a4acf37245a3d9e8
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_de5f90e79becd783
  evidence.sdk-computation-45-candidate#source:581798c7bee4cebe433a2c5ceac4d8da17cacb16ae2ae321e4c564b69bf6f190:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: c7727ca8669dde4df828130bdc30956ce68ad71a7f1751aaadb857446dd91c33
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-computation-45-candidate/native-cross-language-comparison.json
      historical_git_mode: '100644'
      owner: evidence.sdk-computation-45-candidate
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 25b555221013afb6ac811a3c9032c85b4053419c44ecf34481ee0e74b242436e
      owner_fingerprint: 758f71cc9c4da700d16f9c30a76a130e1f6595c2319e46125340f9b80e4faea8
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 581798c7bee4cebe433a2c5ceac4d8da17cacb16ae2ae321e4c564b69bf6f190
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_2a287d6955a12ff4
  evidence.sdk-computation-45-candidate#source:b4edb27e11ad6fb429823ff6de25f2d4b79ffe0de227d4b6dc2454da9b2d1cc8:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 963df3a766f789da475d4ffda29c906add3ae8c5228e1440b103a9aee1029a09
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-computation-45-candidate/ACTUAL-RESULTS.md
      historical_git_mode: '100644'
      owner: evidence.sdk-computation-45-candidate
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 92cb2c40eaf09132efb14c8c74f50ad19e37d3a1d41f9b5e2b619b546bca894a
      owner_fingerprint: 758f71cc9c4da700d16f9c30a76a130e1f6595c2319e46125340f9b80e4faea8
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: b4edb27e11ad6fb429823ff6de25f2d4b79ffe0de227d4b6dc2454da9b2d1cc8
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_326a784167465a05
  evidence.sdk-computation-45-candidate#source:9a9e35686c1eb2c5ff098a678e1703dd2bb9436d1931d517a121699ed95be2b1:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 06c824dbaa8232133785d7d7bd7b67de25c1d16fa2dd4bdc094ff74c1be32c19
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-computation-45-candidate/corpus.py.txt
      historical_git_mode: '100644'
      owner: evidence.sdk-computation-45-candidate
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 7112616af3de51a2918a2b214bddcccc01b0d4b20b805493cf1348086d7a09c8
      owner_fingerprint: 758f71cc9c4da700d16f9c30a76a130e1f6595c2319e46125340f9b80e4faea8
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 9a9e35686c1eb2c5ff098a678e1703dd2bb9436d1931d517a121699ed95be2b1
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_85a5eaec4038fb2d
  evidence.sdk-computation-45-candidate#source:c9ad0ede94233ed0abf3b359c954d9879cafa37f26260b6af9584c59139092bf:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: e65fd9b58781ccee3ce76d59cece30cc3a27445d586f939ad16a578c598d8144
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-computation-45-candidate/inputs.json
      historical_git_mode: '100644'
      owner: evidence.sdk-computation-45-candidate
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 5d7b78c38ef4eb1a722af6f551ab60107563c5acd876e2687ec33787871f679f
      owner_fingerprint: 758f71cc9c4da700d16f9c30a76a130e1f6595c2319e46125340f9b80e4faea8
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: c9ad0ede94233ed0abf3b359c954d9879cafa37f26260b6af9584c59139092bf
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_ae596105a80b4f2e
  evidence.sdk-computation-50#source:77c1d305a55e8320692929efca4f48a6d6dc7a4370cd33c9b403da996dcc15c3:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: c349aa24b4bccfa683fed0fbf9cf4df16db5e4a9ec94cf04f2be111c7b4bc4ff
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-computation-50#source:594ab84a723a1b513ec1939b99a00a745c698396807110c32475ebab83571c3e:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: c1192c90d5a26671215cfdc0f9c75e9f166b8050c3486235144815227ca6e9d2
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-computation-50#source:d9d57f5234c5f6478af52ad529fd9f2ce0bfdfb354580be8de93080597304f72:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 10f4ce04a16726144f9b8d8d7796ac271612a2c53f44cd0dce2d10a050dcfa15
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-computation-50#source:d44e9ff012033e928933fa3afd158c8e38267ac29e35c72d7d2d5f1dd0c9d93e:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 0a9931d87a5913a841852c7c90d922407b4a8b3d6856ae601029a9f5b9191d0e
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-computation-50#source:00a3240a4d37a5f3a4f20cf29f18668833194fa9f1c5c410bd5e22bbeddec8da:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: d3650e6871ca064e5d1e4673b10767da149ea1fcfb7340463c1e3b453bf368b3
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-computation-50#source:3d2fbf48d4f2a76043b29ddb542a6035db678bb19ff1acf1a560c96e8193b887:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 420ac2398c6059b586bf7b47a6630013df546e0502b7528d625681b8382b39e0
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-computation-20261013/receipt.json
      historical_git_mode: '100644'
      owner: evidence.sdk-computation-50
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: d9790d5051d6c35eaeccbffa5173fd04fc5bc60f33d2dda1960490e4ccc89b42
      owner_fingerprint: e543602ce570999e81af1fe2ebfeafd811eb7b9e484dfe1b066e48c0ccdf2a29
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 3d2fbf48d4f2a76043b29ddb542a6035db678bb19ff1acf1a560c96e8193b887
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_c686e5042cf10bd2
  evidence.sdk-computation-50#source:da3ace49e2034a366dfc300afbf5c3ce694b6dbfef63427fdc804d527e7d4825:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 7a5f8c601ccb570402d4550f4415decd994940d49f333d2785895713f21ca36a
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-computation-20261013/comparison.json
      historical_git_mode: '100644'
      owner: evidence.sdk-computation-50
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 1897641b7503f40132db701db72b3dfd54f6dd7aec3dbb563779430e67a02c32
      owner_fingerprint: e543602ce570999e81af1fe2ebfeafd811eb7b9e484dfe1b066e48c0ccdf2a29
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: da3ace49e2034a366dfc300afbf5c3ce694b6dbfef63427fdc804d527e7d4825
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_b345454222ff642d
  evidence.sdk-computation-50#source:0c0e5721de31209a43ff21f7f50637fab2505a8226e654a41ac43c75de4e86ca:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 4911814d6e95890c8fe84604cbde7eb7653e947ef7502fac458be76590cdb889
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-computation-20261013/source-freeze-before.json
      historical_git_mode: '100644'
      owner: evidence.sdk-computation-50
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: ee91611ac225491729a6445b2692653620a1b60b4ac2e9c9472d84a1019799c8
      owner_fingerprint: e543602ce570999e81af1fe2ebfeafd811eb7b9e484dfe1b066e48c0ccdf2a29
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 0c0e5721de31209a43ff21f7f50637fab2505a8226e654a41ac43c75de4e86ca
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_01d1411a8933ab01
  evidence.sdk-computation-50#source:3747fd8178c38a1a1996719c7ca786fce3d306d376be08828aadc6b74af8f465:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 85df5c0b99e2bffc08d523e74104cee02c2713b8fe08d41342cc0ea81029e5b7
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-successor-model-reviews/computation-50.json
      historical_git_mode: '100644'
      owner: evidence.sdk-computation-50
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: f05ff4dbde918aa726820ea38968af2d9210f2698ac47ca3ee606bcf714595c2
      owner_fingerprint: e543602ce570999e81af1fe2ebfeafd811eb7b9e484dfe1b066e48c0ccdf2a29
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 3747fd8178c38a1a1996719c7ca786fce3d306d376be08828aadc6b74af8f465
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_46c3f8c7539144e4
  evidence.sdk-computation-50#source:9824cc09e892d3f70b2e55494160ee52e52ca24270b07336f9e4f790ea9397ce:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: faa04fa7298699283ce9bf97d8c27a9ee39c4e1fda3ff5c8eca1e25ada774795
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/final-receipt.json
      historical_git_mode: '100644'
      owner: evidence.sdk-computation-50
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: e42fedf932432e10dbca4ed38ea2219c0d617f0d2e21118b7aee9223ad2a1033
      owner_fingerprint: e543602ce570999e81af1fe2ebfeafd811eb7b9e484dfe1b066e48c0ccdf2a29
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 9824cc09e892d3f70b2e55494160ee52e52ca24270b07336f9e4f790ea9397ce
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_a0bde4dddf44cf6e
  evidence.sdk-computation-50#source:5e0f153848725261abf228db5320a840d3b0afef6b72f1b8f076a02cdf6eded3:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: afa85d09a95bf2e8264d8e410d8bb423c21bd3f0b0c94e420e11c074642fbdfa
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/source-build-manifest.json
      historical_git_mode: '100644'
      owner: evidence.sdk-computation-50
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 7f0ec61450cb4105cd7cf3ddfa288c4d9ebb4c91e57ed180a47c1144bbccaadd
      owner_fingerprint: e543602ce570999e81af1fe2ebfeafd811eb7b9e484dfe1b066e48c0ccdf2a29
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 5e0f153848725261abf228db5320a840d3b0afef6b72f1b8f076a02cdf6eded3
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_11edaa4fdd42a061
  evidence.sdk-current-reviewed-rust-gates#source:9824cc09e892d3f70b2e55494160ee52e52ca24270b07336f9e4f790ea9397ce:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: faa04fa7298699283ce9bf97d8c27a9ee39c4e1fda3ff5c8eca1e25ada774795
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/final-receipt.json
      historical_git_mode: '100644'
      owner: evidence.sdk-current-reviewed-rust-gates
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: c2bd1da26199582c34ece49bd7ab636bcab054edfc5c9c9400d5d7368d015154
      owner_fingerprint: b45ee28c90cef3812aec9f58f0fa0ec3a6f3ee87eacbb5a9ff7fa56aca1c3b4f
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 9824cc09e892d3f70b2e55494160ee52e52ca24270b07336f9e4f790ea9397ce
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_134fdecb2086def0
  evidence.sdk-current-reviewed-rust-gates#source:5e0f153848725261abf228db5320a840d3b0afef6b72f1b8f076a02cdf6eded3:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: afa85d09a95bf2e8264d8e410d8bb423c21bd3f0b0c94e420e11c074642fbdfa
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/source-build-manifest.json
      historical_git_mode: '100644'
      owner: evidence.sdk-current-reviewed-rust-gates
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: ec5add2996679e5aa5182e09d256a59452d28208226dcb21783ab6555a371a72
      owner_fingerprint: b45ee28c90cef3812aec9f58f0fa0ec3a6f3ee87eacbb5a9ff7fa56aca1c3b4f
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 5e0f153848725261abf228db5320a840d3b0afef6b72f1b8f076a02cdf6eded3
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_727b500f781bd999
  evidence.sdk-current-reviewed-rust-gates#source:67d0f575975cca16a70a1fe8c522034bd219a7bd36215419864830efc2913d0b:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: c1582500ec0c91e20b180463bad13c60152f8c933436d8ec83dca8e6e3a7010d
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/matrix.json
      historical_git_mode: '100644'
      owner: evidence.sdk-current-reviewed-rust-gates
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 105701e9e588d303c7be393b73c461dcc52af1fd40ff9456d0b1b0b2f029a433
      owner_fingerprint: b45ee28c90cef3812aec9f58f0fa0ec3a6f3ee87eacbb5a9ff7fa56aca1c3b4f
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 67d0f575975cca16a70a1fe8c522034bd219a7bd36215419864830efc2913d0b
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_721ffb71e93c8db4
  evidence.sdk-current-reviewed-rust-gates#source:0211396e8dbde80c912c33c6784f30bba7fb8bf7650c881d372d3ebb30b9edb3:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: de33bf3db0aaefceb8cdb92a5e7a62b34b9c265dad5fed9d3276688ce4e75f10
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/run.json
      historical_git_mode: '100644'
      owner: evidence.sdk-current-reviewed-rust-gates
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 674a4288aee755d7023aa0131212cb3352d343b4eab4635b511af7f8d4f9a228
      owner_fingerprint: b45ee28c90cef3812aec9f58f0fa0ec3a6f3ee87eacbb5a9ff7fa56aca1c3b4f
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 0211396e8dbde80c912c33c6784f30bba7fb8bf7650c881d372d3ebb30b9edb3
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_514f3d33a256aba6
  evidence.sdk-current-reviewed-rust-gates#source:ec10f254b5501c728dfc3535509d175e9ce68fa6058a1597a0b208e2f6950f00:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: afd3c7a083e27a17da0f109742c785c29ce7f93a3b57083b289ced6dbcfc6360
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/source-before.json
      historical_git_mode: '100644'
      owner: evidence.sdk-current-reviewed-rust-gates
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 46a426c0f934d2210f634a66ae269b186c4f294ed9c6cf960c90bf47ca92dd47
      owner_fingerprint: b45ee28c90cef3812aec9f58f0fa0ec3a6f3ee87eacbb5a9ff7fa56aca1c3b4f
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: ec10f254b5501c728dfc3535509d175e9ce68fa6058a1597a0b208e2f6950f00
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_402542bf70f5b8f4
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/source-after.json
      historical_git_mode: '100644'
      owner: evidence.sdk-current-reviewed-rust-gates
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: d9e09d4882b766f12cf1e6d18bff525f83432ee524b602ebb212b39daab0e346
      owner_fingerprint: b45ee28c90cef3812aec9f58f0fa0ec3a6f3ee87eacbb5a9ff7fa56aca1c3b4f
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: ec10f254b5501c728dfc3535509d175e9ce68fa6058a1597a0b208e2f6950f00
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_402542bf70f5b8f4
  evidence.sdk-current-reviewed-rust-gates#source:6337e8c957fccaf13d91ca75286b93b88b305731defe6b85c58a7d0d68cf6602:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 89cc5bc1906f2cfb01f153a9b970cd4db68a793537fb116243e07959f448af72
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/workspace-tests.log
      historical_git_mode: '100644'
      owner: evidence.sdk-current-reviewed-rust-gates
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 26f2c0af3b035a95cf290562c2d2fd0e363714eafcf7696fb86dae1feb7665ff
      owner_fingerprint: b45ee28c90cef3812aec9f58f0fa0ec3a6f3ee87eacbb5a9ff7fa56aca1c3b4f
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 6337e8c957fccaf13d91ca75286b93b88b305731defe6b85c58a7d0d68cf6602
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_92c2042c9067a145
  evidence.sdk-current-reviewed-rust-gates#source:6a8faac827278258da59fdb217cc0122d2b44a06b5f6f7abc6094c59d1c5f036:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 9b7558664df50c53b9cdacfb51a05a79d024f63ec8cb10c08add627154c743af
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/workspace-doctests.log
      historical_git_mode: '100644'
      owner: evidence.sdk-current-reviewed-rust-gates
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: c6d02105c328da9d4c436fc7d859b7744481d08aab7e20406de427a1c8b28e77
      owner_fingerprint: b45ee28c90cef3812aec9f58f0fa0ec3a6f3ee87eacbb5a9ff7fa56aca1c3b4f
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 6a8faac827278258da59fdb217cc0122d2b44a06b5f6f7abc6094c59d1c5f036
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_b8ad6c25d7fabdc9
  evidence.sdk-current-reviewed-rust-gates#source:6166375d7ce864bc5005ae8b8a54cead6c37f918a570152f504e4d7933c7b14c:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: fa9b1dda203da5ced1a6ba9173b3ea7038aadf27e269887d9b8f3ae2e7fa0859
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/evidence-sha256.json
      historical_git_mode: '100644'
      owner: evidence.sdk-current-reviewed-rust-gates
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: effdf27dc42f9f1c99d46fd2f262d1b49d55af41aa2c0baf9ac656b3c5690bcf
      owner_fingerprint: b45ee28c90cef3812aec9f58f0fa0ec3a6f3ee87eacbb5a9ff7fa56aca1c3b4f
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 6166375d7ce864bc5005ae8b8a54cead6c37f918a570152f504e4d7933c7b14c
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_e6ccdf7055395c00
  evidence.sdk-current-reviewed-rust-gates#source:64237644b6f72face4c7c993ee1e1560458977a4dbe22d32a04e8ba974c7c040:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 1f682b3c09a92d1342163c5695d0e1cfb18341b1764b54eb2fb430dd1377c1c8
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/feature-wiring-review-manifest.json
      historical_git_mode: '100644'
      owner: evidence.sdk-current-reviewed-rust-gates
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 7be88dcdaebee4b55f6bdee54142dff327a0e5c867325fde3a9de7ae57bf7d3f
      owner_fingerprint: b45ee28c90cef3812aec9f58f0fa0ec3a6f3ee87eacbb5a9ff7fa56aca1c3b4f
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 64237644b6f72face4c7c993ee1e1560458977a4dbe22d32a04e8ba974c7c040
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_1ea92b73c9a8c0d8
  evidence.sdk-current-reviewed-rust-gates#source:f7fb4f69b4e0a1e712df52085f4b46bff93296bb0efa36598ddfe3caaad8e2d4:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 0f6144383308876c8ffaec209a754e565daa3b8e4144f4270549502dc48cf4f5
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-successor-model-reviews/current-reviewed-rust-gates.json
      historical_git_mode: '100644'
      owner: evidence.sdk-current-reviewed-rust-gates
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 6f1d231ff2bc200be3880e3d88ce11ec776f79e1075a839db95fc06f2fec8bac
      owner_fingerprint: b45ee28c90cef3812aec9f58f0fa0ec3a6f3ee87eacbb5a9ff7fa56aca1c3b4f
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: f7fb4f69b4e0a1e712df52085f4b46bff93296bb0efa36598ddfe3caaad8e2d4
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_ec346128ed0ae66d
  evidence.sdk-deferred-product-disposition#source:609667071d210895d3ea9983e7f2805e220027ee59168daf910e36f60c272c3e:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 310a5fa404a12feb13ed7778a8cf7ec0f8d4c1ddb4ea401051bc2e10cf4745fc
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-deferred-product-disposition#source:19fea7868031a34e8b44789d7ab402f177de79dee6e382c0ff8d3188247e5a47:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 76ca187c772cb3ea5b0c8410a7a1fd2b32921b17b3ce3b2d6ae4a1314a36f906
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-deferred-product-disposition#source:59e4b7c35a15744ba697f072c038708f185a6dfb865c2a3fef7deb47e8756ed5:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: aa662b3c0130489c08fe5f600ec9b0eace913508e148a67854873b45ea82e32c
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-dependency-duplicate-human-decision#source:83ae0ba3deaeca9ca2102092c9cd77dc334814775811ec5a8564af5a68b976f6:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 7083616c3e494b1c6231e288e3c98220f2e115e477872ff308336d37d097cd51
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-dependency-duplicate-human-decision#source:1bd944b2ed88c059f9a549274595d3d8e92ef72aa4a82ef5efbc80b6299433a3:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 67bba3fb32aaa820207d774c06fd99bde71d5f5154b53e407579e1ed7a76321a
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-dependency-duplicate-human-decision#source:7fddef67e5dac8a5f393b60fe7d6f9d9d4e176ec785cda1035c3a8dcee4cb6cc:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 94cb3ea6045edae65dd0fd0fa0e1b38f924389d08c3360de432acffef1de792d
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-current-dependency-policy-after-feature-wiring/baseline-comparison.json
      historical_git_mode: '100644'
      owner: evidence.sdk-dependency-duplicate-human-decision
      source_role: documentation
      observed_revision: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
      issue_id: c90d75454c435c02a5a15d04935e80e0d1b641eb5d18d2f074333f97f9bc5fae
      owner_fingerprint: fb71171f938d91b9eb3714ffd1e4f07de851caa2c6bb08c91db51a2eaaef2275
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 7fddef67e5dac8a5f393b60fe7d6f9d9d4e176ec785cda1035c3a8dcee4cb6cc
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_4020bac0593e7401
  evidence.sdk-dependency-duplicate-human-decision#source:402b9eb23ed512e2cae5a1406fd9a447c7ab30225ab9a0d07cdf4b0758c630b0:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 373e550c1ccf70938f5c8292df4ed7613956c2412949d1dc4eec0cc5fc819d67
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-current-dependency-policy-after-feature-wiring/source-before.json
      historical_git_mode: '100644'
      owner: evidence.sdk-dependency-duplicate-human-decision
      source_role: documentation
      observed_revision: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
      issue_id: dd76027d4e3071bb8021ed99c511977969de0bde0fc6ee22d08404c2b498dc9f
      owner_fingerprint: fb71171f938d91b9eb3714ffd1e4f07de851caa2c6bb08c91db51a2eaaef2275
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 402b9eb23ed512e2cae5a1406fd9a447c7ab30225ab9a0d07cdf4b0758c630b0
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_0cf0d35477533c99
  evidence.sdk-dependency-duplicate-human-decision#source:379c050708c2c09cab6470ed3b8807887dceaaa7d01f3624c8ef383b063e5a8e:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 5c01528c5eb3a81e50f4f1716244dd960a052b2a4b759996f27577579cd55640
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-current-dependency-policy-after-feature-wiring/source-after.json
      historical_git_mode: '100644'
      owner: evidence.sdk-dependency-duplicate-human-decision
      source_role: documentation
      observed_revision: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
      issue_id: 7e51166c8225897fb8883ba941d34db0574da19f16c088d205a7ea3150093aa1
      owner_fingerprint: fb71171f938d91b9eb3714ffd1e4f07de851caa2c6bb08c91db51a2eaaef2275
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 379c050708c2c09cab6470ed3b8807887dceaaa7d01f3624c8ef383b063e5a8e
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_7076eea9db864c7d
  ? evidence.sdk-dependency-duplicate-policy-implementation#source:83ae0ba3deaeca9ca2102092c9cd77dc334814775811ec5a8564af5a68b976f6
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 7083616c3e494b1c6231e288e3c98220f2e115e477872ff308336d37d097cd51
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  ? evidence.sdk-dependency-duplicate-policy-implementation#source:1bd944b2ed88c059f9a549274595d3d8e92ef72aa4a82ef5efbc80b6299433a3
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 67bba3fb32aaa820207d774c06fd99bde71d5f5154b53e407579e1ed7a76321a
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  ? evidence.sdk-dependency-duplicate-policy-implementation#source:7fddef67e5dac8a5f393b60fe7d6f9d9d4e176ec785cda1035c3a8dcee4cb6cc
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 94cb3ea6045edae65dd0fd0fa0e1b38f924389d08c3360de432acffef1de792d
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-current-dependency-policy-after-feature-wiring/baseline-comparison.json
      historical_git_mode: '100644'
      owner: evidence.sdk-dependency-duplicate-policy-implementation
      source_role: documentation
      observed_revision: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
      issue_id: 65c5ad520240d819ecbede97885b160547bf770dd230e30e2f1a5b0e2ee96457
      owner_fingerprint: 6e58dea719f319f2b3ea4d0d585df611002491dbc03b3d383b04a1b5af21c173
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 7fddef67e5dac8a5f393b60fe7d6f9d9d4e176ec785cda1035c3a8dcee4cb6cc
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_1fb14b6c9b7b7e55
  ? evidence.sdk-dependency-duplicate-policy-implementation#source:e586545b5e30de78ecb5a2dab07fe3b73a2f87e5328025b83a4426099f3f08fa
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: ccccc2bbbc8dfd5ccc98bc240dce5a02f5ccab30a0e4f58a9dc2948a46688097
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-current-dependency-policy-after-feature-wiring/audit-receipt.json
      historical_git_mode: '100644'
      owner: evidence.sdk-dependency-duplicate-policy-implementation
      source_role: documentation
      observed_revision: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
      issue_id: ab9caad393a731740d55b3692c34244b5761c4d0413c690b2b79212877796337
      owner_fingerprint: 6e58dea719f319f2b3ea4d0d585df611002491dbc03b3d383b04a1b5af21c173
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: e586545b5e30de78ecb5a2dab07fe3b73a2f87e5328025b83a4426099f3f08fa
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_83edc65aaa3edb64
  ? evidence.sdk-dependency-duplicate-policy-implementation#source:82b7f878ac121a0de29337e9dc0d9d39c172101002100c40abac0f102ce8271b
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: a72485e244be77b1665321e0dc32fb2b4091f9e9f8392c7005e9bcd8acc5f154
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-current-dependency-policy-after-feature-wiring/deny-receipt.json
      historical_git_mode: '100644'
      owner: evidence.sdk-dependency-duplicate-policy-implementation
      source_role: documentation
      observed_revision: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
      issue_id: 2f45d6b6f97b98d38d91f6966d601d395f8a56284ae06d2ea58d18fce8dd1085
      owner_fingerprint: 6e58dea719f319f2b3ea4d0d585df611002491dbc03b3d383b04a1b5af21c173
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 82b7f878ac121a0de29337e9dc0d9d39c172101002100c40abac0f102ce8271b
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_e24e1bcc9b01e5d1
  ? evidence.sdk-dependency-duplicate-policy-verification#source:83ae0ba3deaeca9ca2102092c9cd77dc334814775811ec5a8564af5a68b976f6
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 7083616c3e494b1c6231e288e3c98220f2e115e477872ff308336d37d097cd51
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  ? evidence.sdk-dependency-duplicate-policy-verification#source:1bd944b2ed88c059f9a549274595d3d8e92ef72aa4a82ef5efbc80b6299433a3
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 67bba3fb32aaa820207d774c06fd99bde71d5f5154b53e407579e1ed7a76321a
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  ? evidence.sdk-dependency-duplicate-policy-verification#source:7fddef67e5dac8a5f393b60fe7d6f9d9d4e176ec785cda1035c3a8dcee4cb6cc
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 94cb3ea6045edae65dd0fd0fa0e1b38f924389d08c3360de432acffef1de792d
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-current-dependency-policy-after-feature-wiring/baseline-comparison.json
      historical_git_mode: '100644'
      owner: evidence.sdk-dependency-duplicate-policy-verification
      source_role: documentation
      observed_revision: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
      issue_id: ecbffb533d967dfd35eb0b3e64e262757188e58b24e51fb3b182b3fc9b8598a1
      owner_fingerprint: 144efed87fa9dd2c5dd10ba3d0deb823be1f4a2118d8009eb13eb0566e6a248a
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 7fddef67e5dac8a5f393b60fe7d6f9d9d4e176ec785cda1035c3a8dcee4cb6cc
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_be84e5a3c1180b3d
  ? evidence.sdk-dependency-duplicate-policy-verification#source:e586545b5e30de78ecb5a2dab07fe3b73a2f87e5328025b83a4426099f3f08fa
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: ccccc2bbbc8dfd5ccc98bc240dce5a02f5ccab30a0e4f58a9dc2948a46688097
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-current-dependency-policy-after-feature-wiring/audit-receipt.json
      historical_git_mode: '100644'
      owner: evidence.sdk-dependency-duplicate-policy-verification
      source_role: documentation
      observed_revision: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
      issue_id: 57b9ff63d84e629bf2cee9b63ea8c9dec9cfa100089e5609b0024148276a10df
      owner_fingerprint: 144efed87fa9dd2c5dd10ba3d0deb823be1f4a2118d8009eb13eb0566e6a248a
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: e586545b5e30de78ecb5a2dab07fe3b73a2f87e5328025b83a4426099f3f08fa
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_f0a9bbd8abd53f5b
  ? evidence.sdk-dependency-duplicate-policy-verification#source:82b7f878ac121a0de29337e9dc0d9d39c172101002100c40abac0f102ce8271b
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: a72485e244be77b1665321e0dc32fb2b4091f9e9f8392c7005e9bcd8acc5f154
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-current-dependency-policy-after-feature-wiring/deny-receipt.json
      historical_git_mode: '100644'
      owner: evidence.sdk-dependency-duplicate-policy-verification
      source_role: documentation
      observed_revision: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
      issue_id: 5215027f73e4007185ee487bebda2fe38f33689703a5932eb899fbd1175589eb
      owner_fingerprint: 144efed87fa9dd2c5dd10ba3d0deb823be1f4a2118d8009eb13eb0566e6a248a
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 82b7f878ac121a0de29337e9dc0d9d39c172101002100c40abac0f102ce8271b
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_006427d3a288fbee
  evidence.sdk-eval-improve-topology#source:594ab84a723a1b513ec1939b99a00a745c698396807110c32475ebab83571c3e:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: c1192c90d5a26671215cfdc0f9c75e9f166b8050c3486235144815227ca6e9d2
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-eval-improve-topology#source:d9d57f5234c5f6478af52ad529fd9f2ce0bfdfb354580be8de93080597304f72:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 10f4ce04a16726144f9b8d8d7796ac271612a2c53f44cd0dce2d10a050dcfa15
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-eval-improve-topology#source:d44e9ff012033e928933fa3afd158c8e38267ac29e35c72d7d2d5f1dd0c9d93e:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 0a9931d87a5913a841852c7c90d922407b4a8b3d6856ae601029a9f5b9191d0e
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-eval-improve-topology#source:68f8ebc83143ae377cbcd28e8280064ae29f0a358b23c48724047fa3654cb41a:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 8887096cc0453201270c885a078cad8e6626c613040f119db56dd495b9f3227b
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/eval-improve-topology/receipt.json
      historical_git_mode: '100644'
      owner: evidence.sdk-eval-improve-topology
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 873f9fd876da9375285e3f9721cc744dfdf1e8ed95962c573e24ca2f666c49e0
      owner_fingerprint: eb63178e83d62b553fd5b5034b18a9a1cf6b65cf7b6997850bd141291f92dc18
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 68f8ebc83143ae377cbcd28e8280064ae29f0a358b23c48724047fa3654cb41a
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_bb60b36004a3010f
  evidence.sdk-eval-improve-topology#source:91192c147b603d64a5c3140b56266fd3736f1da0d2a7c84c7ae7b54184ce7c61:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 76638198e473a34bd27525f0d201b2904587cabaf2bd23e2d86c6380f014fed0
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/eval-improve-topology/README.md
      historical_git_mode: '100644'
      owner: evidence.sdk-eval-improve-topology
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 21ffd2b6a065b05b121fe95f6ff0e892fd340a270405aa91613bf63ccfdd9dfc
      owner_fingerprint: eb63178e83d62b553fd5b5034b18a9a1cf6b65cf7b6997850bd141291f92dc18
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 91192c147b603d64a5c3140b56266fd3736f1da0d2a7c84c7ae7b54184ce7c61
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_197df202bb121ee3
  evidence.sdk-eval-improve-topology#source:ec5b34536c7e7c222c4684e323fc9bc1a463665bbbdbd2d449dbf35ea6d7e6b9:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 2704095d35ded0dd17cf6969bb9b83a76d25bfb87dad6dd944ea9dff54fb0f15
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/eval-improve-topology/raw/deadline-before-receipt.json
      historical_git_mode: '100644'
      owner: evidence.sdk-eval-improve-topology
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 8e61ff39e0dc8300954803ec612f77e2681271814a4be19ad7df90e90e2b8593
      owner_fingerprint: eb63178e83d62b553fd5b5034b18a9a1cf6b65cf7b6997850bd141291f92dc18
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: ec5b34536c7e7c222c4684e323fc9bc1a463665bbbdbd2d449dbf35ea6d7e6b9
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_40f80061f11337c1
  evidence.sdk-eval-improve-topology#source:ea31c78e6754b9dc7d2d3c9ceca1611577292dc5a965e9d601ca13757397ecea:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: d0a53b70b182dacecc8d3cf8f5fc8d341141d201dd9a5e34c83acc55d9bb718a
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-successor-model-reviews/eval-improve-topology.json
      historical_git_mode: '100644'
      owner: evidence.sdk-eval-improve-topology
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 208d0e21c5c6e0e93fea53b146e48d03c58d47e245e56235e7b7df416a057c84
      owner_fingerprint: eb63178e83d62b553fd5b5034b18a9a1cf6b65cf7b6997850bd141291f92dc18
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: ea31c78e6754b9dc7d2d3c9ceca1611577292dc5a965e9d601ca13757397ecea
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_d953694f65bf82a7
  evidence.sdk-eval-improve-topology#source:9824cc09e892d3f70b2e55494160ee52e52ca24270b07336f9e4f790ea9397ce:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: faa04fa7298699283ce9bf97d8c27a9ee39c4e1fda3ff5c8eca1e25ada774795
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/final-receipt.json
      historical_git_mode: '100644'
      owner: evidence.sdk-eval-improve-topology
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: e41417d4fcbc503f01836a36a19ea11a3087f839cfbee683962220cd15ddc2f2
      owner_fingerprint: eb63178e83d62b553fd5b5034b18a9a1cf6b65cf7b6997850bd141291f92dc18
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 9824cc09e892d3f70b2e55494160ee52e52ca24270b07336f9e4f790ea9397ce
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_7d667123548dba59
  evidence.sdk-events-resources-finite-observation#source:f9ff80c046ac127d2af6dc0aa975015a733dab234302c9363a142c0bb4ef974c:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: b97a4e6e35ff7c8c2b29a7cdef4fbe3dccecbc9eb9c61c25a81af9a797ff9a9e
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-events-resources-finite-observation#source:feba2fcfdac6e34fa665cef7172b3970c35a33b31d0b9a23d1a57dc448347ca1:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: f57526912e49d557dbc984ec87900be64db56738890b345c22f2bcfd3bac3876
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-events-resources-finite-observation#source:21ab3f2569c3f592cc4631df46764312daf8328b4c7ebde3d29bfb007aac7a49:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: d7d50db1b28748e94be8b47fe5198d74e22fe1eca28369ccf5f648bc32ce7ef0
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-events-resources-finite-observation#source:f3b14d81a82d2c6a6f885d7fccf92a34e3ba29adfb9787190ec1bf689c6a3b2f:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: a59877b7aacbe3fc23cf01b25ec7f248acf07eb8992f9673c45b8d289a44ddf7
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-events-resources-finite-observation#source:a7d0f47f9283c885d116fb44b4cc46a59eb8eb68fdfcf78648197c3b86a9d3a4:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: b3cb387909cf1c6809c32d44130d974fe9e16ec26fc91e485b9586bd88bbfd3d
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-events-resources-finite-observation#source:b1c6ed389e99b2ad3d37c87e303e6580c5a38a72d3cb5a98943352cc0188d77c:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 53645ecd4a15d0bfdb416098ea3c5ab0d8bb742d98277cd5cd9844c2255294da
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-events-resources-finite-observation#source:178b87c0dd3caa4f3584253e619d07b009778ee104e33bee88ec548956901a8e:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 4fcaad3c3ec2e5be0a5bbe2f1176fb4075a2e76d9895bbb594351b3499afa73c
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  ? evidence.sdk-extension-teardown-prefix-verification#source:7497f80f7dde08bd2e1b342f0f12a9432f08454291e2ca910f4e905776ae7d49
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 242aaebf3283168dbba63a4f5397d15c540108f76b58e47c94367f6f7246d9bc
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  ? evidence.sdk-extension-teardown-prefix-verification#source:ea22772b5108c7425e0bddb83e0d13f41d107c99bf8c8894f8e1c0cac0f9e0e7
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: dd622382e8f96f31c47b68f007ec4452174871880d07ba306592c26d6d27accd
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  ? evidence.sdk-extension-teardown-prefix-verification#source:9e0e305c46e5d567fa577b984e24cfa2fe8cbdff945ef30b95d7968983369cbb
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 4020c192f2e8bbd4741db6889e15e6d47dfd48bc9302365d94b6e20d33f98fdc
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  ? evidence.sdk-extension-teardown-prefix-verification#source:ccbd1fc83b3bde0f3f25425ce5c0b9f60f43fa520da32932fcfd00a2626a7933
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 5e0033923fd1284e8559f29c2373eb60c72a90598e68a4a1e91233468622814e
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  ? evidence.sdk-extension-teardown-prefix-verification#source:cca0f365a12d9d7bea2dce82a57e71386803f2cfb123077b0d8423fee7052562
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: aa25176d37e96d63e22575d601829c05d272aca816f138fee7e22a580fbd3ba6
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  ? evidence.sdk-extension-teardown-prefix-verification#source:fa6a744e6300c3be3c509b6cfaf88b40301e40a8f8746122c6e0109d074ff2cd
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 8373efe873733ce39d5efcb5b4fa267ef8caf317ec136f855a0b6f2db743aa8a
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  ? evidence.sdk-extension-teardown-prefix-verification#source:845f848458bc55de7a6e8097706f4f9a0e9c8ac226f72cdcfeb8c5eaa6c8d424
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 7aeb2795c6dcc471412eafb9c6cf7cfe332a6113152084a8f4b013bfb4d58347
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  ? evidence.sdk-extension-teardown-prefix-verification#source:daa34a36fc094447cf52b9e99c27db963610e917542c1bc987d5d687a3104739
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: e4a80fa61d9f4201091c8daed8bdc654c713090a576e83bc4459bb3ae6569384
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  ? evidence.sdk-extension-teardown-prefix-verification#source:28ddc7aa385125619f33b01c6ca76a899cc7df084c55e4b8a4e7b6d552f909fb
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 14b6644c5cf2e66406c8e0b8342cae84ba3c9ca7903390c29bc2f9ee956c5656
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  ? evidence.sdk-extension-teardown-prefix-verification#source:594ab84a723a1b513ec1939b99a00a745c698396807110c32475ebab83571c3e
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: c1192c90d5a26671215cfdc0f9c75e9f166b8050c3486235144815227ca6e9d2
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  ? evidence.sdk-extension-teardown-prefix-verification#source:d9d57f5234c5f6478af52ad529fd9f2ce0bfdfb354580be8de93080597304f72
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 10f4ce04a16726144f9b8d8d7796ac271612a2c53f44cd0dce2d10a050dcfa15
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  ? evidence.sdk-extension-teardown-prefix-verification#source:d44e9ff012033e928933fa3afd158c8e38267ac29e35c72d7d2d5f1dd0c9d93e
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 0a9931d87a5913a841852c7c90d922407b4a8b3d6856ae601029a9f5b9191d0e
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  ? evidence.sdk-extension-teardown-prefix-verification#source:551101a5c70fa9273a20511985d25683854405efda45ad2c5b9483c9e677216b
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: cc6a4c1317d55acf203580a0d97c2f56efabf0f48eea033912974c44f624e9e5
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-successor-model-reviews/extension-prefix-final.json
      historical_git_mode: '100644'
      owner: evidence.sdk-extension-teardown-prefix-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 70b40eb0e30f9def23abeaa744716be352068789203ef70572e4e004b3cd36f3
      owner_fingerprint: 3359c56bf04d95cabf3d9cbf448d0c515a2bc0d49514a34d114fa47505648099
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 551101a5c70fa9273a20511985d25683854405efda45ad2c5b9483c9e677216b
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_f0d648f0b5050c5d
  ? evidence.sdk-extension-teardown-prefix-verification#source:9824cc09e892d3f70b2e55494160ee52e52ca24270b07336f9e4f790ea9397ce
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: faa04fa7298699283ce9bf97d8c27a9ee39c4e1fda3ff5c8eca1e25ada774795
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/final-receipt.json
      historical_git_mode: '100644'
      owner: evidence.sdk-extension-teardown-prefix-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: f86ef44fa0558defd105f474750f58b32bfbc29d83e02193ed0de7970f0db723
      owner_fingerprint: 3359c56bf04d95cabf3d9cbf448d0c515a2bc0d49514a34d114fa47505648099
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 9824cc09e892d3f70b2e55494160ee52e52ca24270b07336f9e4f790ea9397ce
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_47120b8379a52ee8
  evidence.sdk-external-timeout-oracle#source:1616dc6d6294157edda228fd260242ec2b83992eb6f294f24d0c9e820d581ec5:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 290e2c5e43d4082d1511b56c172fe8e120ed443cb46e63dcf244811e2c4abc32
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-external-timeout-oracle#source:7770432ecd163c22f9fe0719131c5d0b5ecace0706d97ebb0de2b35097ae826b:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: d4837345967623e8062b86b00c714fbc1604a67fc461e4f160edefb034eebe89
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-external-timeout-oracle#source:57428c630787bab9480e2f1219be6b8248e08398da72ee27598671d562978171:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 540fe5ec9639cafb4b37965cee04739f13436f5a801bf9acdea963942aa9dbc1
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-external-timeout-oracle#source:7eea06eb65d9f5ba522390cc15f716816f48f699729005d2d0e1143ed4650e20:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 320e8483eb7bb0923baa341a78488589bd042aeabab2059a5d98379faf6efeaa
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-external-timeout-oracle#source:77f4e878affc401c207671effff43f40945553da89e03869fcaf81c7b84a0b3a:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 7f5d0faea559fc1336f0108f9e32d65fd63b84dd9c7cd908fafaf8a03f5a659e
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-external-timeout-oracle#source:d133999684363f85d026b2ff4cd2fdc9150927b049ddd812c8ec6b52fdb2435a:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 08167ab5ab9bb42d14cb87640ecbb184ce1733004969714479fe9966433831dd
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-external-timeout-oracle#source:dd1dbb7cd11180bb13b756435c287872ccadffd13faaa006cc5600142d2c5a74:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: fd1bfc308eb2a8742492cb4d071b7481e0daed0f7072cc952404a905b06b8da5
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-external-timeout-oracle#source:32f8af7298606c8cddc28c5edc4d3237241c28f1d336145759c778a6b3c708d5:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: fd756867c3995bb477dacbd28a8506a929c55d63e5534816cb2e3f33f24fde2b
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-factory-child-settlement-repair#source:2595a39b031bff987a2c0a084a5c0194708302db86e344059efa3dce668d9a51:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 12cc4d60aa3389e514849658065b582fd135198783312210f5b2c537c37f5327
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/factory-child-settlement/receipt.json
      historical_git_mode: '100644'
      owner: evidence.sdk-factory-child-settlement-repair
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: f5d42f7f4a74ba865df0c54998bf657eab92289c18829cc658e43d50a11a539c
      owner_fingerprint: 3bec253476714ca0b4e5e8ffdea27b9a1d49b4aa1856da29bb5b37bc2122dfc9
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 2595a39b031bff987a2c0a084a5c0194708302db86e344059efa3dce668d9a51
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_10f04159790fb64a
  evidence.sdk-factory-child-settlement-repair#source:a192e17da25106afb3fe1f5030fe232545749136824c7a765109c672b39c80e5:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 88b43364e8227814220193f2c8b590b9b1eeb6d0299d80efc69f4f4a7d033cdc
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/factory-child-settlement/SHA256SUMS.json
      historical_git_mode: '100644'
      owner: evidence.sdk-factory-child-settlement-repair
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 4b04687a3e6f5fdb530719987f46cb842762d481b51a7b2fb1d96a9ef45c1f32
      owner_fingerprint: 3bec253476714ca0b4e5e8ffdea27b9a1d49b4aa1856da29bb5b37bc2122dfc9
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: a192e17da25106afb3fe1f5030fe232545749136824c7a765109c672b39c80e5
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_30a318575fc3e6d2
  evidence.sdk-factory-child-settlement-repair#source:1fb3761594d8e2df3fdb42c55c31766aaad7ed2840e85637dab06272d81b28eb:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: ca7a0b4dfc57821334399da0809b448413be6aec810aa75253758ce084c91894
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/factory-child-settlement/README.md
      historical_git_mode: '100644'
      owner: evidence.sdk-factory-child-settlement-repair
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 2f88dece8e60ac754706d341ff3e2d4eaed746675722e853a99999b64abdcf35
      owner_fingerprint: 3bec253476714ca0b4e5e8ffdea27b9a1d49b4aa1856da29bb5b37bc2122dfc9
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 1fb3761594d8e2df3fdb42c55c31766aaad7ed2840e85637dab06272d81b28eb
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_da484d759660faa5
  evidence.sdk-factory-child-settlement-repair#source:5dfd8d004152e262f071ee0656536f66daf85e76ab702435fb83967dff8059cc:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 1d18e9e117faff6d609a1a609e116164e277908e1dd051826564ff47ee86b96f
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/factory-child-settlement/scoped-bridge.patch
      historical_git_mode: '100644'
      owner: evidence.sdk-factory-child-settlement-repair
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 8ef6f465c56991d071b95bd7649f703af20854720e6fa0dd5774eb80f7e90bd4
      owner_fingerprint: 3bec253476714ca0b4e5e8ffdea27b9a1d49b4aa1856da29bb5b37bc2122dfc9
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 5dfd8d004152e262f071ee0656536f66daf85e76ab702435fb83967dff8059cc
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_d5fbaf944bf4a5bf
  evidence.sdk-factory-child-settlement-repair#source:95eae872dc5947e0be9cda60631b9bfb26e1889d361a0a3e5e90c0aa0b83814c:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: fa9fc133af2cdcae5ab53e4f28433daf363823d86d47c87aa42c1566908d32b4
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/factory-child-settlement/before-terminal-release-unit.log
      historical_git_mode: '100644'
      owner: evidence.sdk-factory-child-settlement-repair
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 8aed8dfb3d76a06c35c2a9d517e53a27bb7ff47d43775f7d9f49a87d0a2be78b
      owner_fingerprint: 3bec253476714ca0b4e5e8ffdea27b9a1d49b4aa1856da29bb5b37bc2122dfc9
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 95eae872dc5947e0be9cda60631b9bfb26e1889d361a0a3e5e90c0aa0b83814c
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_7c099d2f082c5c76
  evidence.sdk-factory-child-settlement-repair#source:4a2675100d9a84ddd6ce0618369e7f563643a19bdbbd5a88d01d805a117b0b89:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: e9749c25a7ff0c7788e7f4d3ec141ea2b4e2d09376ca52d0f0d3febe1f4c4ff6
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/factory-child-settlement/after-terminal-release-unit.log
      historical_git_mode: '100644'
      owner: evidence.sdk-factory-child-settlement-repair
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: f7b34d9a75979ccbb9b472f3d5c068301f2f1fc2a8d03985d3fc2297bf299f68
      owner_fingerprint: 3bec253476714ca0b4e5e8ffdea27b9a1d49b4aa1856da29bb5b37bc2122dfc9
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 4a2675100d9a84ddd6ce0618369e7f563643a19bdbbd5a88d01d805a117b0b89
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_cae99a9770422972
  evidence.sdk-factory-child-settlement-repair#source:46b40cd0cec0cf94c4a84622cc9ac01d90105f33b13f815d53d2432fa937e135:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 16311102204e07488630ea56f7acccc3cfdd32b2b8be416ee65aead165160244
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/factory-child-settlement/before-close-retry-unit.log
      historical_git_mode: '100644'
      owner: evidence.sdk-factory-child-settlement-repair
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 4172fb4ac94d824880b3119e19243b65d815726e94b28fc9f6fbb30d27e4518a
      owner_fingerprint: 3bec253476714ca0b4e5e8ffdea27b9a1d49b4aa1856da29bb5b37bc2122dfc9
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 46b40cd0cec0cf94c4a84622cc9ac01d90105f33b13f815d53d2432fa937e135
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_c221ad9b44cd941e
  evidence.sdk-factory-child-settlement-repair#source:510a7f090b12155bc78ef6a2d259631feedea658fb70fdc9c69b1e609cf2f523:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 6f07d928660180a7d2edd4027b3912ffe4c6d78aea7c9df83e05654082b81006
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/factory-child-settlement/after-close-retry-unit.log
      historical_git_mode: '100644'
      owner: evidence.sdk-factory-child-settlement-repair
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: cf6540c1f601e8342a6f3d2f8667caa10fce6c0e12f8f3ea57cdb6b294bc9873
      owner_fingerprint: 3bec253476714ca0b4e5e8ffdea27b9a1d49b4aa1856da29bb5b37bc2122dfc9
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 510a7f090b12155bc78ef6a2d259631feedea658fb70fdc9c69b1e609cf2f523
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_7d95d3635eb17878
  evidence.sdk-factory-child-settlement-repair#source:8f0487a7c62cd1a93cf7d7bbe5980a5c0e9cc1cb284e2daadd00e47a2d453581:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 417548308cb5cf5c7cf0acc5af27d4ae7f4ac8360149bb45746285c5a4aee8cd
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/factory-child-settlement/before-factory-e2e.log
      historical_git_mode: '100644'
      owner: evidence.sdk-factory-child-settlement-repair
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 037e8bf979e0c9e2787753f6b3f45e160041eb22fde4e6bd922ad152603fc0ca
      owner_fingerprint: 3bec253476714ca0b4e5e8ffdea27b9a1d49b4aa1856da29bb5b37bc2122dfc9
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 8f0487a7c62cd1a93cf7d7bbe5980a5c0e9cc1cb284e2daadd00e47a2d453581
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_43bb9c0a68433563
  evidence.sdk-factory-child-settlement-repair#source:105a1bd6ab6c07146178f7dcbeb6a0284ba2c405507a490ecb78fc86cfd0171d:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: f32509afd6866611c53356c29e3baa8f859ffcb5df30723db90484f16cefb008
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/factory-child-settlement/after-factory-e2e.log
      historical_git_mode: '100644'
      owner: evidence.sdk-factory-child-settlement-repair
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 623c67a25ae134d0fe3548b6c8560f09d99bf39ccfb0cf784b9b64a5dec44dff
      owner_fingerprint: 3bec253476714ca0b4e5e8ffdea27b9a1d49b4aa1856da29bb5b37bc2122dfc9
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 105a1bd6ab6c07146178f7dcbeb6a0284ba2c405507a490ecb78fc86cfd0171d
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_eb1c9262cb266ba3
  evidence.sdk-factory-child-settlement-repair#source:dfc946b3ad7ee2e68b5a24ad5ed4e6d2633648e74f830ac7e4161f71dc6d200a:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 89e7f82b3cfeafb0981698d3ab785d1d0c248a5a0b20b1a9e80a38a19c420f65
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/factory-child-settlement/final-hostlib-all.log
      historical_git_mode: '100644'
      owner: evidence.sdk-factory-child-settlement-repair
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 4177dc654df24d0a8086bdec7670ee8069463e9397101860345aeaf1d50a2371
      owner_fingerprint: 3bec253476714ca0b4e5e8ffdea27b9a1d49b4aa1856da29bb5b37bc2122dfc9
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: dfc946b3ad7ee2e68b5a24ad5ed4e6d2633648e74f830ac7e4161f71dc6d200a
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_4c6fd0d93211ab2c
  evidence.sdk-factory-child-settlement-repair#source:373bb5e9519b9c37375921d7878750149c8dfd4aae6f2cb1cf375baf2bddf43f:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 3a3089f385db0614132c96a1fcf864d1c47503eabd0506d051c92645ed77619b
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/factory-child-settlement/final-extension-bridge-all.log
      historical_git_mode: '100644'
      owner: evidence.sdk-factory-child-settlement-repair
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: a564525d02d01110463595ca7d4815547ab7da5bdc542a35a04652b4af25a08f
      owner_fingerprint: 3bec253476714ca0b4e5e8ffdea27b9a1d49b4aa1856da29bb5b37bc2122dfc9
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 373bb5e9519b9c37375921d7878750149c8dfd4aae6f2cb1cf375baf2bddf43f
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_34dc9e82fe9697ed
  evidence.sdk-factory-child-settlement-repair#source:9b61071994c178b1e3d1329c8c00704431d90c2e3a68eb73127898bb4f5e04fb:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 09efa3934043b59f368b7aae14c3bea5102dc332c74a9451d019f16f7846e69f
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/factory-child-settlement/clippy-all.log
      historical_git_mode: '100644'
      owner: evidence.sdk-factory-child-settlement-repair
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: d6aa936e7c50974252b5a5e50470c283e2aac9134c254d1352d4455f4584a325
      owner_fingerprint: 3bec253476714ca0b4e5e8ffdea27b9a1d49b4aa1856da29bb5b37bc2122dfc9
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 9b61071994c178b1e3d1329c8c00704431d90c2e3a68eb73127898bb4f5e04fb
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_ac8e885c6fd22ffc
  evidence.sdk-factory-child-settlement-repair#source:4f32c4b801a713225c0985a949ae9b95e306e1c0ee009989d6025a9f29108ee4:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: ab7f15d0b1b8ce9ee244f956d884c7c8379b99ce38b304cce52a582bdb20e87f
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/factory-child-settlement/clippy-production-panic.log
      historical_git_mode: '100644'
      owner: evidence.sdk-factory-child-settlement-repair
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 8d8303de23f501e6bb520b2544fcbceb34f28854eae17f2c29cada6f747b85b3
      owner_fingerprint: 3bec253476714ca0b4e5e8ffdea27b9a1d49b4aa1856da29bb5b37bc2122dfc9
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 4f32c4b801a713225c0985a949ae9b95e306e1c0ee009989d6025a9f29108ee4
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_5829b8fffd31ecd6
  evidence.sdk-factory-child-settlement-repair#source:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 769cb44cf6bc59c752b70f83f2dd6e48f5fc9cbc108aa0c74a9002137bfb3250
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/factory-child-settlement/fmt-check.log
      historical_git_mode: '100644'
      owner: evidence.sdk-factory-child-settlement-repair
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: c0ff6ff18aa457c88c45325d7ff406b82871cec16b31f6dcd34c63f72a6a027d
      owner_fingerprint: 3bec253476714ca0b4e5e8ffdea27b9a1d49b4aa1856da29bb5b37bc2122dfc9
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_6c12fda154657336
  evidence.sdk-factory-child-settlement-repair#source:f2f47097571260a4fd1f7f42a9da11e1d084e7a9774527a4b5219f61eaeeb991:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: f8fc948b21eb111a653c1f9fbd7a50e5c7b6925266db82c1761b50468887ae1a
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-successor-model-reviews/factory-child-settlement.json
      historical_git_mode: '100644'
      owner: evidence.sdk-factory-child-settlement-repair
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: ee13b5cec5409a53f6369e97c630f0a9af0625ca3f383e2e7e19b9ed8b418225
      owner_fingerprint: 3bec253476714ca0b4e5e8ffdea27b9a1d49b4aa1856da29bb5b37bc2122dfc9
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: f2f47097571260a4fd1f7f42a9da11e1d084e7a9774527a4b5219f61eaeeb991
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_ef5a7694aa641930
  evidence.sdk-factory-child-settlement-verification#source:2595a39b031bff987a2c0a084a5c0194708302db86e344059efa3dce668d9a51:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 12cc4d60aa3389e514849658065b582fd135198783312210f5b2c537c37f5327
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/factory-child-settlement/receipt.json
      historical_git_mode: '100644'
      owner: evidence.sdk-factory-child-settlement-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 2f5ef212d1aac57b038d7f2feefc82688fc1ccc416a613705966e5383d037674
      owner_fingerprint: 889e0db906e8d34731abd3b87b7ed703c1f6d41314000b5fac90e0c241e99374
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 2595a39b031bff987a2c0a084a5c0194708302db86e344059efa3dce668d9a51
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_acc52bea9ec2e99e
  evidence.sdk-factory-child-settlement-verification#source:a192e17da25106afb3fe1f5030fe232545749136824c7a765109c672b39c80e5:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 88b43364e8227814220193f2c8b590b9b1eeb6d0299d80efc69f4f4a7d033cdc
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/factory-child-settlement/SHA256SUMS.json
      historical_git_mode: '100644'
      owner: evidence.sdk-factory-child-settlement-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: eea48d6e4f74c3d9d8d40ca244838038afd83aee88886503e1e5a0e416773c26
      owner_fingerprint: 889e0db906e8d34731abd3b87b7ed703c1f6d41314000b5fac90e0c241e99374
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: a192e17da25106afb3fe1f5030fe232545749136824c7a765109c672b39c80e5
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_840d08d8cf83d283
  evidence.sdk-factory-child-settlement-verification#source:1fb3761594d8e2df3fdb42c55c31766aaad7ed2840e85637dab06272d81b28eb:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: ca7a0b4dfc57821334399da0809b448413be6aec810aa75253758ce084c91894
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/factory-child-settlement/README.md
      historical_git_mode: '100644'
      owner: evidence.sdk-factory-child-settlement-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 3b75f15fc83cdf7b715d296e6f90ab9991fa43a96f43a7cdfe244804ce1cbc6a
      owner_fingerprint: 889e0db906e8d34731abd3b87b7ed703c1f6d41314000b5fac90e0c241e99374
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 1fb3761594d8e2df3fdb42c55c31766aaad7ed2840e85637dab06272d81b28eb
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_98fee092eaeaa77c
  evidence.sdk-factory-child-settlement-verification#source:5dfd8d004152e262f071ee0656536f66daf85e76ab702435fb83967dff8059cc:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 1d18e9e117faff6d609a1a609e116164e277908e1dd051826564ff47ee86b96f
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/factory-child-settlement/scoped-bridge.patch
      historical_git_mode: '100644'
      owner: evidence.sdk-factory-child-settlement-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 8c70954945a307f8e51b8a998ed3560f910a3a2dab00c83f05036d31f99220c4
      owner_fingerprint: 889e0db906e8d34731abd3b87b7ed703c1f6d41314000b5fac90e0c241e99374
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 5dfd8d004152e262f071ee0656536f66daf85e76ab702435fb83967dff8059cc
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_587a492b290fc34b
  evidence.sdk-factory-child-settlement-verification#source:95eae872dc5947e0be9cda60631b9bfb26e1889d361a0a3e5e90c0aa0b83814c:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: fa9fc133af2cdcae5ab53e4f28433daf363823d86d47c87aa42c1566908d32b4
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/factory-child-settlement/before-terminal-release-unit.log
      historical_git_mode: '100644'
      owner: evidence.sdk-factory-child-settlement-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 57f8014bfb620130c044f770fee097db02cefe0bbe5a11ce9d3ece10a3489e0e
      owner_fingerprint: 889e0db906e8d34731abd3b87b7ed703c1f6d41314000b5fac90e0c241e99374
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 95eae872dc5947e0be9cda60631b9bfb26e1889d361a0a3e5e90c0aa0b83814c
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_579214c1b4fa5d81
  evidence.sdk-factory-child-settlement-verification#source:4a2675100d9a84ddd6ce0618369e7f563643a19bdbbd5a88d01d805a117b0b89:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: e9749c25a7ff0c7788e7f4d3ec141ea2b4e2d09376ca52d0f0d3febe1f4c4ff6
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/factory-child-settlement/after-terminal-release-unit.log
      historical_git_mode: '100644'
      owner: evidence.sdk-factory-child-settlement-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 4ff4158bd2f11991b42908ad696be5b1db1810c3cd5917dd8b323621b6d79d23
      owner_fingerprint: 889e0db906e8d34731abd3b87b7ed703c1f6d41314000b5fac90e0c241e99374
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 4a2675100d9a84ddd6ce0618369e7f563643a19bdbbd5a88d01d805a117b0b89
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_b82f00bd88307fb8
  evidence.sdk-factory-child-settlement-verification#source:46b40cd0cec0cf94c4a84622cc9ac01d90105f33b13f815d53d2432fa937e135:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 16311102204e07488630ea56f7acccc3cfdd32b2b8be416ee65aead165160244
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/factory-child-settlement/before-close-retry-unit.log
      historical_git_mode: '100644'
      owner: evidence.sdk-factory-child-settlement-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 360b4cf28c3bda1c667e09f103a687adad74a2bd6a3377e4707ae8b15d660580
      owner_fingerprint: 889e0db906e8d34731abd3b87b7ed703c1f6d41314000b5fac90e0c241e99374
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 46b40cd0cec0cf94c4a84622cc9ac01d90105f33b13f815d53d2432fa937e135
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_07b97e4582ca6c88
  evidence.sdk-factory-child-settlement-verification#source:510a7f090b12155bc78ef6a2d259631feedea658fb70fdc9c69b1e609cf2f523:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 6f07d928660180a7d2edd4027b3912ffe4c6d78aea7c9df83e05654082b81006
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/factory-child-settlement/after-close-retry-unit.log
      historical_git_mode: '100644'
      owner: evidence.sdk-factory-child-settlement-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 3d5c8ebc2c7ed3b9cd3fd49ab5b477fae77d569c4e37e9eebcf1c2acc72138c2
      owner_fingerprint: 889e0db906e8d34731abd3b87b7ed703c1f6d41314000b5fac90e0c241e99374
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 510a7f090b12155bc78ef6a2d259631feedea658fb70fdc9c69b1e609cf2f523
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_a8d555f974335504
  evidence.sdk-factory-child-settlement-verification#source:8f0487a7c62cd1a93cf7d7bbe5980a5c0e9cc1cb284e2daadd00e47a2d453581:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 417548308cb5cf5c7cf0acc5af27d4ae7f4ac8360149bb45746285c5a4aee8cd
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/factory-child-settlement/before-factory-e2e.log
      historical_git_mode: '100644'
      owner: evidence.sdk-factory-child-settlement-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: b1e754342ac9bc60dd4cd8cd081f585ce085ee2c9ee32794287b5d0fe8b08e4b
      owner_fingerprint: 889e0db906e8d34731abd3b87b7ed703c1f6d41314000b5fac90e0c241e99374
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 8f0487a7c62cd1a93cf7d7bbe5980a5c0e9cc1cb284e2daadd00e47a2d453581
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_c418708d8b834872
  evidence.sdk-factory-child-settlement-verification#source:105a1bd6ab6c07146178f7dcbeb6a0284ba2c405507a490ecb78fc86cfd0171d:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: f32509afd6866611c53356c29e3baa8f859ffcb5df30723db90484f16cefb008
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/factory-child-settlement/after-factory-e2e.log
      historical_git_mode: '100644'
      owner: evidence.sdk-factory-child-settlement-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: cf0afc5383aceffb3acded7a8d54198db4ed0491a2168b41be29def92770b869
      owner_fingerprint: 889e0db906e8d34731abd3b87b7ed703c1f6d41314000b5fac90e0c241e99374
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 105a1bd6ab6c07146178f7dcbeb6a0284ba2c405507a490ecb78fc86cfd0171d
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_c327d898e6241950
  evidence.sdk-factory-child-settlement-verification#source:dfc946b3ad7ee2e68b5a24ad5ed4e6d2633648e74f830ac7e4161f71dc6d200a:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 89e7f82b3cfeafb0981698d3ab785d1d0c248a5a0b20b1a9e80a38a19c420f65
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/factory-child-settlement/final-hostlib-all.log
      historical_git_mode: '100644'
      owner: evidence.sdk-factory-child-settlement-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: a54c0fe057dbe467fc5312f89a0025055bbd9f83f2bb90055d37d3eccdba4d52
      owner_fingerprint: 889e0db906e8d34731abd3b87b7ed703c1f6d41314000b5fac90e0c241e99374
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: dfc946b3ad7ee2e68b5a24ad5ed4e6d2633648e74f830ac7e4161f71dc6d200a
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_b7d613d176262883
  evidence.sdk-factory-child-settlement-verification#source:373bb5e9519b9c37375921d7878750149c8dfd4aae6f2cb1cf375baf2bddf43f:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 3a3089f385db0614132c96a1fcf864d1c47503eabd0506d051c92645ed77619b
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/factory-child-settlement/final-extension-bridge-all.log
      historical_git_mode: '100644'
      owner: evidence.sdk-factory-child-settlement-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: f7c890a0c0ee440f35d454051aba1bac042a16eafb0bccbb07ce200315e26622
      owner_fingerprint: 889e0db906e8d34731abd3b87b7ed703c1f6d41314000b5fac90e0c241e99374
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 373bb5e9519b9c37375921d7878750149c8dfd4aae6f2cb1cf375baf2bddf43f
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_0ad41a240c131332
  evidence.sdk-factory-child-settlement-verification#source:9b61071994c178b1e3d1329c8c00704431d90c2e3a68eb73127898bb4f5e04fb:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 09efa3934043b59f368b7aae14c3bea5102dc332c74a9451d019f16f7846e69f
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/factory-child-settlement/clippy-all.log
      historical_git_mode: '100644'
      owner: evidence.sdk-factory-child-settlement-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: aed1bfde8af0e322a73b97001da71969b1d7aec6fd67cf4cae99305710ee1bb0
      owner_fingerprint: 889e0db906e8d34731abd3b87b7ed703c1f6d41314000b5fac90e0c241e99374
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 9b61071994c178b1e3d1329c8c00704431d90c2e3a68eb73127898bb4f5e04fb
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_469750415bdce797
  evidence.sdk-factory-child-settlement-verification#source:4f32c4b801a713225c0985a949ae9b95e306e1c0ee009989d6025a9f29108ee4:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: ab7f15d0b1b8ce9ee244f956d884c7c8379b99ce38b304cce52a582bdb20e87f
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/factory-child-settlement/clippy-production-panic.log
      historical_git_mode: '100644'
      owner: evidence.sdk-factory-child-settlement-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: c61c29a3751938c680c3f3df42693ebc02b34a14cfd1f15cc2488adf546844be
      owner_fingerprint: 889e0db906e8d34731abd3b87b7ed703c1f6d41314000b5fac90e0c241e99374
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 4f32c4b801a713225c0985a949ae9b95e306e1c0ee009989d6025a9f29108ee4
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_bcc47314f191e09f
  evidence.sdk-factory-child-settlement-verification#source:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 769cb44cf6bc59c752b70f83f2dd6e48f5fc9cbc108aa0c74a9002137bfb3250
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/factory-child-settlement/fmt-check.log
      historical_git_mode: '100644'
      owner: evidence.sdk-factory-child-settlement-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: a40855c75cfb1a0ce8ae9512e52bed193c14f364a443f09360d5e9c76d28b1ce
      owner_fingerprint: 889e0db906e8d34731abd3b87b7ed703c1f6d41314000b5fac90e0c241e99374
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_ebc28967d0e10610
  evidence.sdk-factory-child-settlement-verification#source:f2f47097571260a4fd1f7f42a9da11e1d084e7a9774527a4b5219f61eaeeb991:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: f8fc948b21eb111a653c1f9fbd7a50e5c7b6925266db82c1761b50468887ae1a
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-successor-model-reviews/factory-child-settlement.json
      historical_git_mode: '100644'
      owner: evidence.sdk-factory-child-settlement-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 02e59a1faae7d488a167366687d6d6b7f59d3ae7648a5a732a1ce5221173167f
      owner_fingerprint: 889e0db906e8d34731abd3b87b7ed703c1f6d41314000b5fac90e0c241e99374
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: f2f47097571260a4fd1f7f42a9da11e1d084e7a9774527a4b5219f61eaeeb991
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_a89304f1ab6934c6
  ? evidence.sdk-family-catalog-method-routing-candidate#source:28b3625e7d7700dc6a9d2eff895c01ccf4e69621a403c27a7987ea88ff2bd1ad
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 84cd65076cf2f581dd0f24b761a0c77045cea85b23ce2b27da2fb70529937f51
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-family-route-20261013/manifest.json
      historical_git_mode: '100644'
      owner: evidence.sdk-family-catalog-method-routing-candidate
      source_role: documentation
      observed_revision: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
      issue_id: 6c3e2be99335b39a29bfd206cd1113abd827dff80894aeaa568d50f14316dc7d
      owner_fingerprint: c58417499e6f161841303be2b266f9e7bf104dbbdc39ca52d3ebf44965833111
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 28b3625e7d7700dc6a9d2eff895c01ccf4e69621a403c27a7987ea88ff2bd1ad
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_e817db3d4d84f4ac
  ? evidence.sdk-family-catalog-method-routing-candidate#source:facee131488751e69bdecaf7cd38910323398d1d2136d3435c9f3ec4d28f0788
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: c346975ad35cde1f000334c39e531a9520b06c787a89d9782f59b77f93887a2c
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-family-route-20261013/content-guard-public-call-route-failure.log
      historical_git_mode: '100644'
      owner: evidence.sdk-family-catalog-method-routing-candidate
      source_role: documentation
      observed_revision: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
      issue_id: 3a4d917ddb19bf3d204ae4fcdd852f47883da00fa6bccd6a22ed072ee09af552
      owner_fingerprint: c58417499e6f161841303be2b266f9e7bf104dbbdc39ca52d3ebf44965833111
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: facee131488751e69bdecaf7cd38910323398d1d2136d3435c9f3ec4d28f0788
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_c247c7bc32f70e93
  ? evidence.sdk-family-catalog-method-routing-candidate#source:59cfd6409515f5c5416d775dd4631be3e8d26bf69241f855ba1ba125f0e77c14
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 718e5e036e4f562c03c914b747b86f7083b7de11407a757d4fcf0cdabd93e9d8
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-family-route-20261013/sdk-family-route-typescript-green.log
      historical_git_mode: '100644'
      owner: evidence.sdk-family-catalog-method-routing-candidate
      source_role: documentation
      observed_revision: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
      issue_id: 6d613362cd155ed4bb0abc7b1b1e577aa7eddeb17924340b98f74f14889371fc
      owner_fingerprint: c58417499e6f161841303be2b266f9e7bf104dbbdc39ca52d3ebf44965833111
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 59cfd6409515f5c5416d775dd4631be3e8d26bf69241f855ba1ba125f0e77c14
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_a05c4a5bc8decca0
  ? evidence.sdk-family-catalog-method-routing-candidate#source:781d8adb85d573445f0a3aa93007fd37217a9bd16cb36bbbb550e5b91756898b
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: e24c8a54e45fd7315779396cbfaa085946baf7c9c93c37bcfd937fe56880a176
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-family-route-20261013/sdk-family-route-python-serial-green.log
      historical_git_mode: '100644'
      owner: evidence.sdk-family-catalog-method-routing-candidate
      source_role: documentation
      observed_revision: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
      issue_id: d173dc99ea438c05ef0630dfb63a59c4923afb9b14987e5dcb0d7ef27efe8af8
      owner_fingerprint: c58417499e6f161841303be2b266f9e7bf104dbbdc39ca52d3ebf44965833111
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 781d8adb85d573445f0a3aa93007fd37217a9bd16cb36bbbb550e5b91756898b
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_e9f99d1abb9b2e21
  ? evidence.sdk-family-catalog-method-routing-candidate#source:2c58b87a0350a39bb1c3b5f4891defeeb9a785e84aa11a466dc9ddb21253bf58
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 8ceb379a335427047dce9e03ef8065af546cf61c3f70626be15ff24a4ad0375a
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-family-route-20261013/sdk-family-route-java-green.log
      historical_git_mode: '100644'
      owner: evidence.sdk-family-catalog-method-routing-candidate
      source_role: documentation
      observed_revision: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
      issue_id: 86c50afaa9b79200b9fbc82097fc2e2a9753b66883e1fa1e764962b1ff89c3fb
      owner_fingerprint: c58417499e6f161841303be2b266f9e7bf104dbbdc39ca52d3ebf44965833111
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 2c58b87a0350a39bb1c3b5f4891defeeb9a785e84aa11a466dc9ddb21253bf58
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_03ebaf08771f3a49
  ? evidence.sdk-family-catalog-method-routing-candidate#source:c3a6e23b4b4c33ff03fa325608e9b576a10739c60a519d2a142e8dd31b4507b7
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 62525006f3b5bd62ce90e49c75cbf2e5e1f3f690358ff6dba28c4b315cc9661a
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    current_reference_replacements:
    - raw_reference: docs/sdk/governance/evidence/sdk-family-route-20261013/TEST-com.echoagent.sdk.FamilyRouteHostTest.xml
      historical_git_mode: '100644'
      owner: evidence.sdk-family-catalog-method-routing-candidate
      source_role: test_consumer
      observed_revision: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
      issue_id: bcae781dccecfa046633145fe56017dc70bc0504066e6fa7221500bdb0c395eb
      owner_fingerprint: c58417499e6f161841303be2b266f9e7bf104dbbdc39ca52d3ebf44965833111
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: c3a6e23b4b4c33ff03fa325608e9b576a10739c60a519d2a142e8dd31b4507b7
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_74f56b740fa5acbc
      replacement_source_ref: sdks/java/src/test/java/com/echoagent/sdk/FamilyRouteHostTest.java
      replacement_content_sha256: c601acd1b875ef8b657f1bf3c446ff64365e4bd94f6b5754f0b3133c5e2fc706
      replacement_git_mode: '100644'
  evidence.sdk-feature-wiring-verification#source:64237644b6f72face4c7c993ee1e1560458977a4dbe22d32a04e8ba974c7c040:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 1f682b3c09a92d1342163c5695d0e1cfb18341b1764b54eb2fb430dd1377c1c8
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/feature-wiring-review-manifest.json
      historical_git_mode: '100644'
      owner: evidence.sdk-feature-wiring-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 8e54d8feb9d0e485559434d9d36620811e16769e3f545707fdfdcf67c7d124d7
      owner_fingerprint: a2b3c925d88164fb5180f628362d5997f66274b47f17b520c17ddea93ed98b75
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 64237644b6f72face4c7c993ee1e1560458977a4dbe22d32a04e8ba974c7c040
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_72f33442d906f5ba
  evidence.sdk-feature-wiring-verification#source:12a266cb8eb1acdd268aa64b7081e6b1c720a15dbd1721c9d47ed683140ea4f0:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: bb3b2f0e87c986e28a6fafe6cc1a7bef42ca89cc80b9f1fe751d42f5340454d8
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/feature-wiring-only.patch
      historical_git_mode: '100644'
      owner: evidence.sdk-feature-wiring-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 3accdd836194169d16933bb6828f77c835524f68681610c99f879930cb01cd04
      owner_fingerprint: a2b3c925d88164fb5180f628362d5997f66274b47f17b520c17ddea93ed98b75
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 12a266cb8eb1acdd268aa64b7081e6b1c720a15dbd1721c9d47ed683140ea4f0
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_1bf0c6c924a71216
  evidence.sdk-feature-wiring-verification#source:1d04d497e278a3ac07934f37c198587480a31b8c25a938c53a12b823b46c6c68:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 486a82dc4016214ce0f051aa8d8283cf802e7ff06b0b10717bc2623962a747d1
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/README.md
      historical_git_mode: '100644'
      owner: evidence.sdk-feature-wiring-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: ae59a299c2355fb158f160917950e327484d526ba7bc6f27853eb2a215c8b981
      owner_fingerprint: a2b3c925d88164fb5180f628362d5997f66274b47f17b520c17ddea93ed98b75
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 1d04d497e278a3ac07934f37c198587480a31b8c25a938c53a12b823b46c6c68
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_d47be92344322b06
  evidence.sdk-feature-wiring-verification#source:4c0b37a27e5d95292912199bc158a034c395f178bdb4d5562a2dadb75a14cdef:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: fdf1092d934bae873843b113bccf19391a0125451870f356fb5613e187026d55
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-successor-model-reviews/feature-wiring.json
      historical_git_mode: '100644'
      owner: evidence.sdk-feature-wiring-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 37b324b5663d851bcc94e0321d494e0abbcdcecd809e042319dee0c23c81b145
      owner_fingerprint: a2b3c925d88164fb5180f628362d5997f66274b47f17b520c17ddea93ed98b75
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 4c0b37a27e5d95292912199bc158a034c395f178bdb4d5562a2dadb75a14cdef
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_5f85944bfacc982c
  evidence.sdk-framework-pin#source:7ff6ae1a29f0d2e156ee1ba7fec78179030911e6c91f808b24aaee4812d5c971:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: ea0bd73785018e3d78947b8431dfbd53f1aa5d915198504bf474f4ceb796bc79
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-framework-pin#source:83ae0ba3deaeca9ca2102092c9cd77dc334814775811ec5a8564af5a68b976f6:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 7083616c3e494b1c6231e288e3c98220f2e115e477872ff308336d37d097cd51
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-framework-pin#source:45ea97df7083eb5404f70cd94bb1426acef9740236d82b109d4c7d1d1cbd77d7:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 9cb18bf8101bb85c095f8520ab30d8693f4fc92f4232008cf3dc02ce3b45031e
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-framework-pin#source:9828351422d02002b941a5f1b60fade021b4cdbe74cdc8ee3b3f8739d254e9db:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 92cdc3dc3bb807e564d4bc408f79f76889b41e393d4cc7fadadc8c53ade7963e
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-framework-pin#source:8793b0382b493e3065829476ed7b7b7bfa6393ce1c610df03aee40f4972c6826:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 25bced96db0a8d8189f009164acbfbada6b9ee02ad985e00dac6a7bff63e7318
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-framework-pin#source:3412033ea7cfe1d4e8a21b3df5c2c48e31e3ca9c96177c6a6e6943c20f77a8f4:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: a84b279763386fa49e2e4af490dbcd58fa543431fe6c79183c5c1d1c1220ceb1
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-framework-pin#source:2c015f9015361febfc67dee6806952d509000e8b9a72f41620a9c18b1d0fc741:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: df41ccf55700f550b01c9194af1d6a6799d32ed16e74de4524c2092e972e4de2
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-framework-pin#source:67de89a42755b692d3164ff45d2276211e3998e0f1588fa285d545073c8fcbe2:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: dae67a3979ddfc5ff76f6b1c6db6fd686ef21a1cd9d7830e157d4eab48a0c0fd
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-framework-pin#source:fb2b49f3981cd463ec32379b19ec7c5796912dce3e45dfe274d4f5931923112f:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 0e12d3277d9f354301110da997aaac17e4a526bf2e3611ec7562b3f4c0ad311c
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-framework-pin#source:11c334ecbd78795d6dab408a284c38c514eb69073b2bc5096721a1811346a913:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 883a62a9c2642f158754441301dff12a9a9f39ef56103591cf2ddfa08abdee07
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-framework-pin#source:85f68063b51d6533146158109e02ad9d3b55019ca85a50e836df22f7969fa0a8:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: b2c7a3e5f0c2f74c4c78df51ae186755849963176b5687859d2076897bf432f9
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-framework-pin#source:3bb0f3a40ef9f7f2ef302d11e172212b245a117fffb089bede154a9966c987c3:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 053904f3070f15f6b1201eb68c41b872d1171264a6e7348fdf62fd774792a07c
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-framework-pin#source:f72cd073939e72b8c1946482bbd1c54d1bce6cdc6c21bb0810797752739e5e15:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 9183acd8fe802d5e6120b685c0d4440c0f7c2fc83b7d12102afc1eec9424cd94
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-framework-pin#source:f8bd249a92171101c097e4a5f242a535445ba438eac3f8f1ed11794026ee87c7:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 7890e1a7d7774b68b10ced537ae5680faa6d84c99f18132f194dd61b553f49ec
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-framework-pin#source:cdb012bd095fcdfc3c1e912f70deb077086fe0b7401e63c4b4cad1b8468f4c89:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 8ba71a821ebad1cb3dc00c18198e20fe0532782d7df83e3a68af92f571df34e1
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-framework-pin#source:fb5a0f62075a98a7b2867fe102dfd04ee3e3756269ff911f8a3197728f227321:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 3d58937876e4186098abc1866efb5aa3e4daae1e0065b3dd9c9473c5722baefd
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-framework-pin#source:7f61c8762f340f9d45f6b7129a9713272a92b9e78c4f13a3a12e582c28a2f777:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: f3ed6919687ccab23e05e1ac3a77a4d7496111aaadf4e14632a938103239c42b
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-framework-pin#source:18767cf395f861b25db6543ca1d4071407a23593a0db2e7695ac87d3a416911e:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 9c87b33eb1108f46753068f270d005d0a9babcac093ac737150d88fd742ffbd3
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-framework-pin#source:ea66311cfc8937d4a887dbe468e1d7ee16a16423fc558ddb88ac6899940f82ca:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 7d593db524e0a7c90ed6042306c54b142183b7a6fc0a91f517484ea95bbf59e8
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-framework-pin#source:c77a21f540cbfbc67d820f6b5b46baf975e37551fed36e7cbb2b868ef23bd2f0:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 8021533e4041834a20a8834ca7f24be5a73773b5f6331418f0b8b081b8cb66b9
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-framework-pin#source:7d5ccad093832b84649a2c841f3e8b164a73f7008b66bcba8408a951187bf61a:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: f24a64fcdd5f2c56a529251437dd9cc81e2568adeee478895e7810ed563b8b63
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-framework-pin#source:d12f742972cdbea79b7494989c35612d93a42a008d28f9954e34db7948169a1b:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: c71267396cc4d2a287fef6e18e317901ebd976c595a8aba9719008906f520ef1
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-framework-pin#source:04657d4f64ef385deb69648b59803667a98e4d8508d13f1b463da8d7259ad7c7:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: b4e0ec345a2bfc132dc0ba849477d219ff56096fd0c61b64d800d1a60d13b398
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-framework-pin#source:879f17c51d6f8f206d4436141a2bb2c034f1bd7ac7501597e2de51adfec136da:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 416fd688a958b0ea5971ceb93352bb229dc7b59a0040c551929a50e536590795
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-framework-pin#source:8bf1e726e9c42cdb3e3cd50722373f28f92e49375a5b481719722471c3d0bb8b:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: e9a5b5baf180393c365b07cce864352774ca5cef07265ae8d30c3a7a59425e7e
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-framework-pin#source:a22527491fb0cd0d46089ae67c7cc3445730fb2820831c460b6ed7eb032025db:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: ba9a7ba970bde59a3a7ed59dd427afad6da69129bde76960b3bbdb9f6f6893cd
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-framework-pin#source:54b0260c1e63f54b7160b1326e40cd231a4148ead7b203af0942153713818eea:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 1c97179489bbba48cdb07074a29b957c9646de1f3fce12579b9ddd671f86078a
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-framework-pin#source:c894613315f49fde9e761502d0afacaed6e97c5dc3112a47365cace1e605dc3f:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: e47bc70f775ec1c16363af3484d71cedeb745ce2eecdde2619824fb5fa72374f
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-gap-watermark-host-verification#source:e4ae808abb8f916a7318faec3643c5a56cd1bbd143e9e88cb447e0cf7127f7de:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: efba8b3c4cace3f64f6f1edd2f71df59dc191bdeb459f45a76503dafe0b63f2c
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-gap-watermark-host-verification#source:594ab84a723a1b513ec1939b99a00a745c698396807110c32475ebab83571c3e:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: c1192c90d5a26671215cfdc0f9c75e9f166b8050c3486235144815227ca6e9d2
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-gap-watermark-host-verification#source:d9d57f5234c5f6478af52ad529fd9f2ce0bfdfb354580be8de93080597304f72:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 10f4ce04a16726144f9b8d8d7796ac271612a2c53f44cd0dce2d10a050dcfa15
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-gap-watermark-host-verification#source:d44e9ff012033e928933fa3afd158c8e38267ac29e35c72d7d2d5f1dd0c9d93e:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 0a9931d87a5913a841852c7c90d922407b4a8b3d6856ae601029a9f5b9191d0e
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-gap-watermark-host-verification#source:cbf90de4b8b415dd26a45b7d5ffd75e5f42fe433e2345e3f076a77c1ed0996f5:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: aa2ea26996075e2c338b8d2489973b73f17153242dbac80c48f7faa0acef31eb
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-gap-watermark-host-verification#source:4dd701525ff692fce5901a0ae55fc1cb871ad3d05c6500f6053c16ef3d87737c:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: d452ec396c503bb4f514d7d76f7977fe58c154dd38754cdaecd05ee1a9fe8cd0
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-gap-watermark-host-red/receipt.json
      historical_git_mode: '100644'
      owner: evidence.sdk-gap-watermark-host-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 72825e5ec0a4ac08917c511cf273e4fd20595e064830afab8201f6f31bf553b3
      owner_fingerprint: 15afc174f202233780de11962fa4b3c6141b59a98609f0b86ec5fa4fe852ae62
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 4dd701525ff692fce5901a0ae55fc1cb871ad3d05c6500f6053c16ef3d87737c
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_4310c9a560cea664
  evidence.sdk-gap-watermark-host-verification#source:7b3c28aeb0c86c7692d5be133f3b3d197b7176284c4ef366aa50b01c9c245d05:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: ca1bc9b00b067b71b1c4a1d50cb40b2edb53e87d8ceb1aaa2445441346cd62d1
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-gap-watermark-host-red/SHA256SUMS.json
      historical_git_mode: '100644'
      owner: evidence.sdk-gap-watermark-host-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: f121eff6161d03d996c4db96ffab22b2e7faab146a6814ac235e022e1256c647
      owner_fingerprint: 15afc174f202233780de11962fa4b3c6141b59a98609f0b86ec5fa4fe852ae62
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 7b3c28aeb0c86c7692d5be133f3b3d197b7176284c4ef366aa50b01c9c245d05
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_72c0ffc066279458
  evidence.sdk-gap-watermark-host-verification#source:013d0434caa5b268888ad07f55e46645f8b3130668e74000a0aa2484ce8af67a:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: b3940b403b1d2034425f6ba8a90044f02e3ce54678bbc769fbc4a21a5768a806
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-gap-watermark-host-red/source-manifest.json
      historical_git_mode: '100644'
      owner: evidence.sdk-gap-watermark-host-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 0a641b6b5c09f450ed704b3b1aa0c940b72fdbc453b6fc7e143f8bb53df026cf
      owner_fingerprint: 15afc174f202233780de11962fa4b3c6141b59a98609f0b86ec5fa4fe852ae62
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 013d0434caa5b268888ad07f55e46645f8b3130668e74000a0aa2484ce8af67a
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_717278727f9f2879
  evidence.sdk-gap-watermark-host-verification#source:2a9b53a262726e2f9d9b1d4c7e679c074a0fa4e92f7bb37bf85e162cfd48cf76:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 93c45ff54e151eb4bc93c1b6c40a738403967bfc3a78c31f6f0b9e3e459938a2
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-gap-watermark-host-red/watermark-only-regression.patch
      historical_git_mode: '100644'
      owner: evidence.sdk-gap-watermark-host-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 907e1570cfeb1f98ecc2425069df640c07a896498a57f24ff2f9cf52ac8b6c83
      owner_fingerprint: 15afc174f202233780de11962fa4b3c6141b59a98609f0b86ec5fa4fe852ae62
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 2a9b53a262726e2f9d9b1d4c7e679c074a0fa4e92f7bb37bf85e162cfd48cf76
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_6679c73943165ef8
  evidence.sdk-gap-watermark-host-verification#source:5eb8e672cac5a58cf53efecf739bc85328e9971d712eaafe998691caef0700ef:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 9d28d94dee91486de1394f8c935edebc2efd6e894f2a66de82f60051ad9004a2
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-gap-watermark-host-red/red-host-receipt.json
      historical_git_mode: '100644'
      owner: evidence.sdk-gap-watermark-host-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 234e27566836f89c23675b937c6906a34e0517591f5aab3fd460e2da27b5fc6e
      owner_fingerprint: 15afc174f202233780de11962fa4b3c6141b59a98609f0b86ec5fa4fe852ae62
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 5eb8e672cac5a58cf53efecf739bc85328e9971d712eaafe998691caef0700ef
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_b12c213719229b6a
  evidence.sdk-gap-watermark-host-verification#source:e50013a266f9813859099d13b5552d99f178f132c35340063d9c45fd2488f8c3:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: dbe2e773d995c6650a3d5ac382999940d39e8f0ed7a3748d33541a6d76be954d
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-gap-watermark-host-red/green-host-receipt.json
      historical_git_mode: '100644'
      owner: evidence.sdk-gap-watermark-host-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: cc6841a313440136e39c5840f8b20a135b46e7333368492caccc851fe9b0600a
      owner_fingerprint: 15afc174f202233780de11962fa4b3c6141b59a98609f0b86ec5fa4fe852ae62
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: e50013a266f9813859099d13b5552d99f178f132c35340063d9c45fd2488f8c3
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_8dd4bd371b1d41d6
  evidence.sdk-gap-watermark-host-verification#source:5aa34750c4ba7e19d3b4362409b2331e8333544899efb550c31ca6205c63af8b:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 31b65ce9e0ea4bedc7d43dcb1553737abf9c7cc3192a49f5f91baa01d0180cb3
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-gap-watermark-host-red/red-host.log
      historical_git_mode: '100644'
      owner: evidence.sdk-gap-watermark-host-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: b88f73130c7aafedc199441c86ad4ee175b07d170479ce7e7423438e124d7c79
      owner_fingerprint: 15afc174f202233780de11962fa4b3c6141b59a98609f0b86ec5fa4fe852ae62
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 5aa34750c4ba7e19d3b4362409b2331e8333544899efb550c31ca6205c63af8b
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_91c1cb09d6a5a5a5
  evidence.sdk-gap-watermark-host-verification#source:6e8b7bbf4188fee43f0bf4427b8dc2bbfa2386cdc2bf7574a429a5e8f5d887d9:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 2f0ea7396fce98af5e70edb9ecd9a0c330670c8f7c47c98a27bf3fa58ed5c685
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-gap-watermark-host-red/green-host.log
      historical_git_mode: '100644'
      owner: evidence.sdk-gap-watermark-host-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 60cbdf09a416b5bc37d1d031e1a0e6d28aaf28c427c3672d9772c2872d2e5ff4
      owner_fingerprint: 15afc174f202233780de11962fa4b3c6141b59a98609f0b86ec5fa4fe852ae62
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 6e8b7bbf4188fee43f0bf4427b8dc2bbfa2386cdc2bf7574a429a5e8f5d887d9
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_77fe95dadca116c2
  evidence.sdk-gap-watermark-host-verification#source:2b372ce338a031cdd39723e98f1716b25a2986ccfb205709b787ad534ea04218:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: d99e0a705c38210207128c1aaecaa23d17a6469a50e1436a38c0982c8df205f1
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-gap-watermark-host-red/events-regressed.rs
      historical_git_mode: '100644'
      owner: evidence.sdk-gap-watermark-host-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 85adcf3dae24c2d72ae0806e0e726312a269a051b1c8dc89a8df6e22f187961d
      owner_fingerprint: 15afc174f202233780de11962fa4b3c6141b59a98609f0b86ec5fa4fe852ae62
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 2b372ce338a031cdd39723e98f1716b25a2986ccfb205709b787ad534ea04218
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_fda77cfa1361122c
  evidence.sdk-gap-watermark-host-verification#source:2e29cbe4e5257b79171adf35b60073566d99e810b375271d10586f13a1bb2d81:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: cd3560584bd8877960c542e0019a689925e5be216962ba2befb9adffdae456ad
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-gap-watermark-host-red/fixture-observed.rs
      historical_git_mode: '100644'
      owner: evidence.sdk-gap-watermark-host-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: d76d28fed0d002f7a398fafb3e085e62b7dbb0e17fd277d3817f5ec8d147f30c
      owner_fingerprint: 15afc174f202233780de11962fa4b3c6141b59a98609f0b86ec5fa4fe852ae62
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 2e29cbe4e5257b79171adf35b60073566d99e810b375271d10586f13a1bb2d81
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_d342e08cd3f217dd
  evidence.sdk-gap-watermark-host-verification#source:9824cc09e892d3f70b2e55494160ee52e52ca24270b07336f9e4f790ea9397ce:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: faa04fa7298699283ce9bf97d8c27a9ee39c4e1fda3ff5c8eca1e25ada774795
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/final-receipt.json
      historical_git_mode: '100644'
      owner: evidence.sdk-gap-watermark-host-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 073641450fd744183bc6a3f695c2aa94afde9fc99c05f30260f7617de4038607
      owner_fingerprint: 15afc174f202233780de11962fa4b3c6141b59a98609f0b86ec5fa4fe852ae62
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 9824cc09e892d3f70b2e55494160ee52e52ca24270b07336f9e4f790ea9397ce
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_b98106b73874ebc1
  evidence.sdk-gap-watermark-host-verification#source:5e0f153848725261abf228db5320a840d3b0afef6b72f1b8f076a02cdf6eded3:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: afa85d09a95bf2e8264d8e410d8bb423c21bd3f0b0c94e420e11c074642fbdfa
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/source-build-manifest.json
      historical_git_mode: '100644'
      owner: evidence.sdk-gap-watermark-host-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 2b3ce8cd7fc17c5a52cf71d55561fbb4e386395956999eee0be2ea788ec91d87
      owner_fingerprint: 15afc174f202233780de11962fa4b3c6141b59a98609f0b86ec5fa4fe852ae62
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 5e0f153848725261abf228db5320a840d3b0afef6b72f1b8f076a02cdf6eded3
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_22560042610fa5ff
  evidence.sdk-git-published-consumer-verification#source:e958cdbb0672c0545247b05a8a97332f9a1a2ce7624066a0cb7b954fe1840fb3:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 0d9d4e5c58c467570f7cd7dad7e866f05b18405300d48307c96d64214a232cbb
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-git-published-consumer-verification#source:a20b93fcb1e9193c7415d455fc442570336c6c0612635afe6a2350ccf62103f9:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 86408e7223f88eeeb192933ed06e9f490e157a19f8557ca31b2916a8fa359f4c
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-git-published-consumer-verification#source:21662715abdcafc67fc12287f7c3816a0db321b3c8cbeaae7c4c07bba0bca126:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: a584a5d4ed4b0fb597bad55a9251fc0683878c79f61cd573a228c93e84a06ce2
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-git-published-consumer-verification#source:1122f76151a7bd01228d3e0cb7cf8cf3046307262c0c6c766080701c30d3fc5c:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 027f953653e877226fe50ea06f701b46cec920230ac35213bbc60d26a3c1a7d3
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-git-published-consumer-verification#source:5bc31a2070658ffb2b23a0866a8cd82d177503b946802bea4a656d1d529fab06:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 3723eabc5266b42d394d9ec6a82d00d8583f1d213892d9d62266994eff758c7f
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-git-published-consumer-verification#source:2c3b22d209bad8b3f7210913e867091cde085e94e93936328939df866faed43a:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 5110be7c6da8aae7fad62bcb56cb630f10b2eff801d9acbad2cd4e336eb58e71
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-git-published-consumer-verification#source:a05f24d52720de48bb1eaa131ddc0eb40d49cc190fc63cf7c2acb711988ecc9d:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: cd29ee93a95481af9b0903ec7a1520f4d1cd3345b153a88b42dc16240c35a04e
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-git-published-consumer-verification#source:babcaf65c079d723d73bffdd268f1b5dd39f635ac0c85490825919843f561c0b:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: a3e1e80c8957b6fc13c6337ea207dcbffc2962e6d13bee7dc5c2ed529cafb626
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-git-published-consumer-verification#source:098dd2f1008fbe1921216b8377dd7ede4bc6fe0104c58d7c79d84680d56fab4f:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: b8322df37310f03809268366e26caee03e222b5982c1b96f9e8f27e0c6f13058
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-git-published-consumer-verification#source:a1a99fce177f6a2d9e5b7145e2b392c701aa2cc24b0d822451648651f5fe639c:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 51bcc4e1c0f570155e18a1206519b595c1a7074ba3a83908b210af4aae934fd9
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-git-published-consumer-verification#source:240fa9be5b5814c75c71a50aaebdf734993a26eefd7364a396cab1458b9e8fe7:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 34ea57586a5a2949eb6ae89cb60641f9db3a2bf038c29b25a221701debd0fb69
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-git-published-consumer-verification#source:da15e969e1907ca6536f21193320809ea4196d30e5e4b4c7fa504c5063714469:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: c7d00eba969483c4d15c21b1032f58bc0fbb2350b198223b1bfbc9b3c614b643
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-host-pid-publication-oracle#source:a48d8252aafd14101214bde7cead4af8056a7af28fa7ab0b8fe98a12d50dfd2d:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 9726ef930f46fa790d9795ec7a5384c666f27df6e94e33c2e39ac2cb0bb424d7
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-host-pid-publication-oracle#source:97eb54f418c15d60458c31e89a316d2869879c861176a93a2f9000280023e5a5:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 5422f77c1c012a923473e954f38ac0f19fefe5c221a61fed5d555299e7c3dedb
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-host-pid-publication-oracle#source:bacfee37ccb85be2b9150d90ddac3f5145633bf0e3fa552fdec65157c4ec10a2:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 87225538bc45a1189bd1d01b81019366bf2d96638a24141a23a529055baff209
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-host-pid-publication-oracle#source:32f8af7298606c8cddc28c5edc4d3237241c28f1d336145759c778a6b3c708d5:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: fd756867c3995bb477dacbd28a8506a929c55d63e5534816cb2e3f33f24fde2b
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-integrations-finite-observation#source:2875aafb87d6543a43135c635e065899f8b58c6325b6a9497e5d07903e1cf8e9:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: ba5583d474fbf20815baa185f38800c3002c4beaa6c063b7ffff47d8ed006a25
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-integrations-finite-observation#source:21ce0c4f56c85691aa461d22a3e2b6837a0e335073c8a136bda916597201c728:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: bea76a0a8e0d4d8965f6d5062110083a1f475e4cb0fd9e2bdefb0af1f11edafd
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-integrations-finite-observation#source:2df1bbe8ff3221c690d016379030dee6f76afa8f1893f8ccadb1791d14769649:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 5b5f99514d15263864e388a5f6217d7f668628f8e12f011253cc57f1647a3a8e
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-integrations-finite-observation#source:95c3b306c857efd65bd315029dab7a5f346f0c7926201778ed70043042d26628:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 51d2bd97f6acb8c8914540fb3a4aa3807921a1c615cda5a7cd486d63fb4dcf70
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-integrations-finite-observation#source:31a2ea7ca1f3d519ce28fc15eca3c7ff6b5ddecec436a079c2c922a4383a79cf:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: aa5be491b7bf79a691066231bec85672867d130dfae912dc635304e558ad22ee
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-integrations-finite-observation#source:47d0ead7af39fedcccb99b66a36a62fc63e2a1d228338a032def71db3c8d1dc4:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 4f518187fd3cb368b629671243a700ab4f83f1a788cbaba80704fec1efe4a755
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-integrations-finite-observation#source:9a03aad737de6524b2139560772801f486fd28f941476350c522d65d02f5de19:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: abbf9899f636353b809e97137ef731bb85b3d51f9c2a8808db08d35dedff6ed0
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-integrations-finite-observation#source:4de9dd615c62f11b5f0278aca3ae90a39abcf0afdd0ed269657437771d69c8a9:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: f516c54f90912c339ac163744beb3e61590039ca325aad98e340fc0f6f5a5281
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-integrations-finite-observation#source:b1c6ed389e99b2ad3d37c87e303e6580c5a38a72d3cb5a98943352cc0188d77c:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 53645ecd4a15d0bfdb416098ea3c5ab0d8bb742d98277cd5cd9844c2255294da
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-integrations-finite-observation#source:178b87c0dd3caa4f3584253e619d07b009778ee104e33bee88ec548956901a8e:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 4fcaad3c3ec2e5be0a5bbe2f1176fb4075a2e76d9895bbb594351b3499afa73c
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-integrations-finite-observation#source:d9cee6cb3e27dbf9ebb868320ffd7bf2bdab4cfda83e2b556ddea9349dfa7bbf:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 12a275c6186a08ef85a187049708837a533d7fe780fb815307c84d8f7fb47121
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-integrations-finite-observation#source:3a26d5b2c89773ada31776c5f23bc208b775c5201662d1ece1543c797f5562d1:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 7478bba719ab02ccfa9dffe235456cca89f245002958496ffd98b04dafdb0193
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-integrations-finite-observation#source:3e91e2836c450b6eddb30115ec38e58f1859faaeb4ff61eccabaa685ab40dd8a:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: adda9a718d917d2c556c9952038dba2ede245874ebb249b5a35845f423d37fe2
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-integrations-finite-observation#source:df13c0187206340d2e0fb8a1553a13c78b5da7a1dfaaccd3d8857095c7987cbc:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 2b2def3acf3c220e8d5fb63d9fb8b8256fd2394327628788691fafcdd80daa8c
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-integrations-finite-observation#source:875b938aacd0886b84e74db7cde848f22b3d4fb28bdda61fd744d8beff1ce344:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: edbad2fa0f40317979b248fbc6a3b801d35b9ed10222c5b2a544c6a1b26692c6
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-integrations-finite-observation#source:e7056c63937197321720299959194b148669b33382128451b59c4333ab2d53e8:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 25a0072c760b898a8b1c263d901ee417cd2e6147a9714aced26069fe79d23221
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-integrations-finite-observation#source:3c8f9c808703b1ea490361699cfe438fae73d8e5f1c226e13e67647adcb3ffd6:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: dc63cad65440222d5e4e4925d8d565df2543da313f7f163cf14af683b9785f23
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-integrations-finite-observation#source:8654ec5acd9d30f00275870be52bbfbf0481a1c2a02f7d16bfd75dd5c6774567:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: d4d46357ddb2e9d282fb79112ca6ac6e64923f37821f965a584b67f153460adc
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-integrations-finite-observation#source:078e338b81c674e4e3bc272f594e8b4b2a9e65dcee899974e07b7d009d5cdf23:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: e3cbc9a3b6880fbc9dc132c5287d199550a2b959f4498d8724ee6fc40d5d122c
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-integrations-finite-observation#source:30ca4fb5f2a008160290fe3664d16bb70b2d168d57b693f23bab6f2a9628e38e:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: e0f0216b6c434f61b7371e59ae25dd98dcb335f22c65ca56503cf50119075662
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-integrations-finite-observation#source:197e191b6bc2e5275919bf22e0fa023055bd26ad7a6e2a0a08b58779801b009b:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: e20111b2875a3e9db27d41c2d899be29d1db475b77ce326cff7768fbb0f88d49
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-integrations-finite-observation#source:beaacc87a18188d89681b6ff2712dcb890f2eed93273071eb244fd0d6f178bbf:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 5f459fef2d52552f278879ac969eda2093b239344b74922f8ca2bcf2c2bf427a
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-java-host-early-exit-verification#source:0e3181817d84578c51efb60ce4fafbe8be9e599396ab679dd8a9bacc2b06ef28:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 2b1dfda02195dfa3f4b025cb9928c71ddab482b49c36238a438355a8c28468f5
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-java-host-early-exit-verification#source:594ab84a723a1b513ec1939b99a00a745c698396807110c32475ebab83571c3e:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: c1192c90d5a26671215cfdc0f9c75e9f166b8050c3486235144815227ca6e9d2
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-java-host-early-exit-verification#source:d9d57f5234c5f6478af52ad529fd9f2ce0bfdfb354580be8de93080597304f72:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 10f4ce04a16726144f9b8d8d7796ac271612a2c53f44cd0dce2d10a050dcfa15
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-java-host-early-exit-verification#source:d44e9ff012033e928933fa3afd158c8e38267ac29e35c72d7d2d5f1dd0c9d93e:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 0a9931d87a5913a841852c7c90d922407b4a8b3d6856ae601029a9f5b9191d0e
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-java-host-early-exit-verification#source:cbf90de4b8b415dd26a45b7d5ffd75e5f42fe433e2345e3f076a77c1ed0996f5:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: aa2ea26996075e2c338b8d2489973b73f17153242dbac80c48f7faa0acef31eb
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-java-host-early-exit-verification#source:ccb6f21586a20cacb69b2d90fe0dbaf4d534891fe8e94adb723a4e65d37ec706:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: e176748e46308708be919592818e1bbb84471a8762746cbb47dad0c6c1d382ae
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-java-host-early-exit/README.md
      historical_git_mode: '100644'
      owner: evidence.sdk-java-host-early-exit-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: fef0e0a6ef4c5b6927d9574f84432c99064c51e288f311fa1a9d737d779f1938
      owner_fingerprint: 894671c7b6a1ae862d34cf5630dc8f11da3610e6d16d9e3521bcc263c4a2ede2
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: ccb6f21586a20cacb69b2d90fe0dbaf4d534891fe8e94adb723a4e65d37ec706
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_fee8fb96eee52caf
  evidence.sdk-java-host-early-exit-verification#source:a9e305568ce77fa7b5b4f9a6fff035f7bf94903b0d8de3a98247f9b099007265:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: de665acd930d372fd9337d033584ce45011952355a76c09eb818360c90cf75e0
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-java-host-early-exit/evidence-sha256.json
      historical_git_mode: '100644'
      owner: evidence.sdk-java-host-early-exit-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 6d45ba62aed55b5040599275fb25d9a6cadd0640b070535d4ab318a888847549
      owner_fingerprint: 894671c7b6a1ae862d34cf5630dc8f11da3610e6d16d9e3521bcc263c4a2ede2
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: a9e305568ce77fa7b5b4f9a6fff035f7bf94903b0d8de3a98247f9b099007265
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_b86f96d7203a9473
  evidence.sdk-java-host-early-exit-verification#source:fe2533d7a252f4b0ecc133c0dfde13189107ae5dac8a887bc1d1ca1778733d2e:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: a7760aa51f6dba82cfa7804c77aa55ab66f623adc8c34f2e32241affe04c730b
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-java-host-early-exit/closing-final-source-class-manifest.json
      historical_git_mode: '100644'
      owner: evidence.sdk-java-host-early-exit-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: afa3ae0b2e58732b095754b4889c29092e5740d7ff2aa93a10357462a2e7c3d7
      owner_fingerprint: 894671c7b6a1ae862d34cf5630dc8f11da3610e6d16d9e3521bcc263c4a2ede2
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: fe2533d7a252f4b0ecc133c0dfde13189107ae5dac8a887bc1d1ca1778733d2e
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_3ae5dc89b421408c
  evidence.sdk-java-host-early-exit-verification#source:aade9670f0fd2c6928911f6b5cc64e5c33f0f470cfb9f1cf56818fbedcf8639f:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 0601b477e1c8b6bbe9f054815b02d3b069bfb91f395b762aae93d0233b076471
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-java-host-early-exit/real-child-before.log
      historical_git_mode: '100644'
      owner: evidence.sdk-java-host-early-exit-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 450dbc184655757c0c4ab9ca2a90676ec80a969c39e55b581a8ee586bbb8fa84
      owner_fingerprint: 894671c7b6a1ae862d34cf5630dc8f11da3610e6d16d9e3521bcc263c4a2ede2
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: aade9670f0fd2c6928911f6b5cc64e5c33f0f470cfb9f1cf56818fbedcf8639f
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_e6732af4891a7033
  evidence.sdk-java-host-early-exit-verification#source:2c58b87a0350a39bb1c3b5f4891defeeb9a785e84aa11a466dc9ddb21253bf58:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 8ceb379a335427047dce9e03ef8065af546cf61c3f70626be15ff24a4ad0375a
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-java-host-early-exit/java-lifecycle-focused-closing-final.log
      historical_git_mode: '100644'
      owner: evidence.sdk-java-host-early-exit-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 495ff464c70a0f6e8421d53524ee3a15690b3ec99366d25c44e6e59105853f47
      owner_fingerprint: 894671c7b6a1ae862d34cf5630dc8f11da3610e6d16d9e3521bcc263c4a2ede2
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 2c58b87a0350a39bb1c3b5f4891defeeb9a785e84aa11a466dc9ddb21253bf58
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_30c5662d6a3c7052
  evidence.sdk-java-host-early-exit-verification#source:7de6d1671a76fd69a191e4004f1c54b86d229c11fc38982ab68e464152e687b9:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: c835efa3d416a07e1ef085fd72a380d2edb8a07d840797b990a96086394064cc
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-successor-model-reviews/java-original-host-exit.json
      historical_git_mode: '100644'
      owner: evidence.sdk-java-host-early-exit-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 6e1666351baa24359668e18b7a3894931781a6e61ee28d9690e175dcad06ab03
      owner_fingerprint: 894671c7b6a1ae862d34cf5630dc8f11da3610e6d16d9e3521bcc263c4a2ede2
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 7de6d1671a76fd69a191e4004f1c54b86d229c11fc38982ab68e464152e687b9
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_e56817798089623f
  evidence.sdk-java-host-early-exit-verification#source:b65dea36e455ea74a9c3926b64e456dea037771c9d1125ad158e34cbf73db14e:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 5cb0501fb7bfd84f04595b311e293c6763286b8ff8b38f22ee58d58ce3103dd9
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    current_reference_replacements:
    - raw_reference: sdks/java/src/main/java/com/echoagent/sdk/EchoAgentClient.java#awaitForExit
      historical_git_mode: '100644'
      owner: evidence.sdk-java-host-early-exit-verification
      source_role: production_source
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 465e7157a3fd94f569918b7907909f35b5e182843abddf63e970ab368a354754
      owner_fingerprint: 894671c7b6a1ae862d34cf5630dc8f11da3610e6d16d9e3521bcc263c4a2ede2
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: b65dea36e455ea74a9c3926b64e456dea037771c9d1125ad158e34cbf73db14e
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_15d5b6e89518a344
      replacement_source_ref: sdks/java/src/main/java/com/echoagent/sdk/EchoAgentClient.java
      replacement_content_sha256: 0e3181817d84578c51efb60ce4fafbe8be9e599396ab679dd8a9bacc2b06ef28
      replacement_git_mode: '100644'
  evidence.sdk-java-host-early-exit-verification#source:9824cc09e892d3f70b2e55494160ee52e52ca24270b07336f9e4f790ea9397ce:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: faa04fa7298699283ce9bf97d8c27a9ee39c4e1fda3ff5c8eca1e25ada774795
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/final-receipt.json
      historical_git_mode: '100644'
      owner: evidence.sdk-java-host-early-exit-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 2666afd1771af916a054e935949668aa298efa87689fc7b5d920a93b46fbf281
      owner_fingerprint: 894671c7b6a1ae862d34cf5630dc8f11da3610e6d16d9e3521bcc263c4a2ede2
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 9824cc09e892d3f70b2e55494160ee52e52ca24270b07336f9e4f790ea9397ce
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_868f1aab1dc3f56e
  evidence.sdk-java-host-early-exit-verification#source:5e0f153848725261abf228db5320a840d3b0afef6b72f1b8f076a02cdf6eded3:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: afa85d09a95bf2e8264d8e410d8bb423c21bd3f0b0c94e420e11c074642fbdfa
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/source-build-manifest.json
      historical_git_mode: '100644'
      owner: evidence.sdk-java-host-early-exit-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 3bb2b1e9fdeb52b02889e0d0c10e5b6ccde9f724ddf4c7919a69c99a1826241d
      owner_fingerprint: 894671c7b6a1ae862d34cf5630dc8f11da3610e6d16d9e3521bcc263c4a2ede2
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 5e0f153848725261abf228db5320a840d3b0afef6b72f1b8f076a02cdf6eded3
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_17044e4ca2720d04
  evidence.sdk-java-stdio-close-specific-closure#source:bc4369fe45f9f762a744b50a2a0fa4a46feea8cda1a90a35ea8192542b4c1f79:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 86d48dc404b342b31b5c061b4bb5e7b492b0170290f0046cfbb17701429c5b9d
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-java-stdio-close-specific-closure#source:df705c7520e5d2faffd7b9ed849f0f057345b29b0af6b208e50cba4e167a0c7d:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 7fbd39bfdd553a1507bbb5ac7950dbf36cf6264fafccfcbac153be22aed864e4
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-java-stdio-close-specific-closure#source:da2a353c0eb5efef24a0ffc8f9aeb33aa954719bb4daeefe08d0ff9ac4decdd9:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 65cbefb6f97ee7a045cbb5abd3fd1299f3e79ec266e3eed7e3f5b6dc4828ee71
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-java-stdio-close-specific-closure#source:17a0932d5a87cb64ececac6a1e6d3118f99326ce157896e27ae6d1af9e4f288b:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 2a2a0dc6cca69d5f91083cd862de55d905c327461325e7a53b7afdb50fcf4251
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-java-stdio-close-specific-closure#source:b1c6ed389e99b2ad3d37c87e303e6580c5a38a72d3cb5a98943352cc0188d77c:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 53645ecd4a15d0bfdb416098ea3c5ab0d8bb742d98277cd5cd9844c2255294da
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-java-stdio-close-specific-closure#source:c5067422bba892564c813fbd8af4dd3df5b8928296cae7eb3849104c9201e117:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: aadf20a8d728cd550ac736a3a0199124e1c8ae7b1bcbc4f45e28d15cb56c1e86
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-java-stdio-close-specific-closure#source:178b87c0dd3caa4f3584253e619d07b009778ee104e33bee88ec548956901a8e:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 4fcaad3c3ec2e5be0a5bbe2f1176fb4075a2e76d9895bbb594351b3499afa73c
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-java-stdio-close-specific-closure#source:d9cee6cb3e27dbf9ebb868320ffd7bf2bdab4cfda83e2b556ddea9349dfa7bbf:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 12a275c6186a08ef85a187049708837a533d7fe780fb815307c84d8f7fb47121
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-java-stdio-close-specific-closure#source:ad102b7792b6160fc32e80e8509e222723bdeafae26ba2cacda586bc9a2652eb:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 7b732b1c07bbd84a001ded86f10673cda59c6da0b0b06f85312a9f9e4b8eb036
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-language-client-lifecycle-current#source:28ee6e9e2555858ceda3959c32d3d02aa6a604ebcdf1a0c432e491d713dce01f:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 4cb06d229a9e80cdbe21da8515cd92d36cc9da2600ea80deaa8a54375ecfe20d
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-language-client-lifecycle-current#source:d6bc54be4f37be767b16cce4beecab2145d47c8ad9237ba6601832606fd0c0a6:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 89ab9bb8959586131f3bb5dffafc6a5d64b57e9ecaf0d5546e6a561882891525
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-language-client-lifecycle-current#source:f5220ea6af0d61db753a7636ad688c2cd3b15f1b39bf1397b1b6599d3ef37046:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 0a188b5f12712293767cca9c86c8655c9224b41b2857bc6ebad3bb0d79be6be1
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-language-client-lifecycle-current#source:7064a8d6be6db3e0fc84d659f056e299f9dd13942674e4f82b36af1b4c994c70:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: a7f506ad64fa7c17bbacef28e4b1783d62cbe707b33bd29607bdd2ef6593fea7
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-language-client-lifecycle-current#source:df705c7520e5d2faffd7b9ed849f0f057345b29b0af6b208e50cba4e167a0c7d:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 7fbd39bfdd553a1507bbb5ac7950dbf36cf6264fafccfcbac153be22aed864e4
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-language-client-lifecycle-current#source:d7f7de5799d8613a57097c6dbb39a8a72f5b926aec85c690664c391945d62a7d:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: caac865d1060ee1db574f5b8efa5c62420da8353417190d8aeefeb6f88e592f3
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-language-client-lifecycle-current#source:ad102b7792b6160fc32e80e8509e222723bdeafae26ba2cacda586bc9a2652eb:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 7b732b1c07bbd84a001ded86f10673cda59c6da0b0b06f85312a9f9e4b8eb036
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-language-client-lifecycle-current#source:b1c6ed389e99b2ad3d37c87e303e6580c5a38a72d3cb5a98943352cc0188d77c:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 53645ecd4a15d0bfdb416098ea3c5ab0d8bb742d98277cd5cd9844c2255294da
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-language-client-lifecycle-current#source:c5067422bba892564c813fbd8af4dd3df5b8928296cae7eb3849104c9201e117:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: aadf20a8d728cd550ac736a3a0199124e1c8ae7b1bcbc4f45e28d15cb56c1e86
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-language-client-lifecycle-current#source:178b87c0dd3caa4f3584253e619d07b009778ee104e33bee88ec548956901a8e:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 4fcaad3c3ec2e5be0a5bbe2f1176fb4075a2e76d9895bbb594351b3499afa73c
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-language-client-lifecycle-current#source:d9cee6cb3e27dbf9ebb868320ffd7bf2bdab4cfda83e2b556ddea9349dfa7bbf:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 12a275c6186a08ef85a187049708837a533d7fe780fb815307c84d8f7fb47121
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-language-client-lifecycle-current#source:bca8e3c5d596c557a40e0398fc22b9b153d5068220fbca53cfa95d90d12fdfb4:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: cdb70d820219b718a81ef9ce1f6508cc82c906b473a2dc0179202ac6c1833c9b
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-language-client-lifecycle-current#source:1b16931c04188a2f9e152e0f386f0628cf2fcad82b01fdf6a2ec420340908c31:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: b558ffc98416ed7185f330a785cfe06db75446848b5365f391d574e85b46b832
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-language-client-lifecycle-current#source:94939365f49db602abaea27dff1bd5249080dc1abfe76ddec307b1d72c127088:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 06b4d2a9ef1de206994e3a87f967526de1a49288180bc6d2239093e1fdafe1fd
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-language-client-lifecycle-current#source:9366b307c8961459768da7571d575ce9fd6af8be250eba5a256400886efd0064:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: e3c0459ef99ff475dcda716546dc69b9f4ff5b1fd7ae922201d18f2d60048f30
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-language-client-lifecycle-current#source:26ae1e9d32236594e98dfca68208df26271aa9e4ae1340578eb8b185dbf03368:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 4f4b49eaefbcb7f80a90c5538891940634f49e255b679cd0abcb6125b6c7be98
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-language-client-lifecycle-current#source:874f4d9b104c28558cc5429918276e93729c0555007686fd799410cca10dc47d:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: fd0e3569cadd9266a0f16c0361bc99efab61d7649fe5b7b7cc8931b1904a8727
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-language-client-lifecycle-current#source:7fd1773ff07d35ac3eb664f44ec35cd228c88b3fc22d1b23ff5dd3a46906550c:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: e7a104598618669482f5664d8f65269bcd0b137b120c38dd9ad4b23e6d10f785
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-language-client-lifecycle-current#source:30ca4fb5f2a008160290fe3664d16bb70b2d168d57b693f23bab6f2a9628e38e:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: e0f0216b6c434f61b7371e59ae25dd98dcb335f22c65ca56503cf50119075662
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-language-current-first-gate-failure#source:469ccce7bf30b6b2f5934588cc8b5d9dd5fca1738d3048761ff08953c7de8bc1:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: a8503010fa5bba8f1da5e6a032622304c1b2cf80efc515bd2933196f687bab77
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-language-current-first-gate/scope.json
      historical_git_mode: '100644'
      owner: evidence.sdk-language-current-first-gate-failure
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 052066a7ae926572ab9cd8b2c4e3a753f5743e6d9e27651e7dcdcefb38390c1a
      owner_fingerprint: 2079a5626ac5d7b1d342751aeb7ee69d7c007783b4d96f2d3c009923d57769a6
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 469ccce7bf30b6b2f5934588cc8b5d9dd5fca1738d3048761ff08953c7de8bc1
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_2a36449a82639a5f
  evidence.sdk-language-current-first-gate-failure#source:6b34da6e068b6a8d3829237c3df8ccdce86c228a7d176405556a64509f189482:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: e4fb9cae112fe9344287200532c37c647ba5571a1989453681bcf88c9d26fb26
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-language-current-first-gate/receipt.json
      historical_git_mode: '100644'
      owner: evidence.sdk-language-current-first-gate-failure
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 0c673f7355c260ed3527f5bea07da93e33d79135e36af986e8486f6f2453329a
      owner_fingerprint: 2079a5626ac5d7b1d342751aeb7ee69d7c007783b4d96f2d3c009923d57769a6
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 6b34da6e068b6a8d3829237c3df8ccdce86c228a7d176405556a64509f189482
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_34daa7bfc446e401
  evidence.sdk-language-current-first-gate-failure#source:7b95dac6b5968fe99fb4d51505eca940425068da96a6f3cfa729e50b0aadafa5:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 2642b403f5d887547a1c00f5a77210124268eafff0b2b64014c8c32b57681321
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-language-current-first-gate/full-language.log
      historical_git_mode: '100644'
      owner: evidence.sdk-language-current-first-gate-failure
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 2656e70c3b36ee7b05116001cb0b06ce2e1b24519d24ed6f3c773e01c293d6a3
      owner_fingerprint: 2079a5626ac5d7b1d342751aeb7ee69d7c007783b4d96f2d3c009923d57769a6
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 7b95dac6b5968fe99fb4d51505eca940425068da96a6f3cfa729e50b0aadafa5
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_512591c734de8904
  evidence.sdk-language-current-first-gate-failure#source:ec9d02dba464115cfdede2bcbca4ec108246e12b2a2cc3794c166040aca29898:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 7e382fb5e4027c53e6a6515d774fdd264872106916480dc380e709446391aea5
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-language-current-first-gate/task-store.json
      historical_git_mode: '100644'
      owner: evidence.sdk-language-current-first-gate-failure
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 3213c78f7076b812bb541ba4c271b97ac7fc6377e987f765f3ba08efd3e5c5a6
      owner_fingerprint: 2079a5626ac5d7b1d342751aeb7ee69d7c007783b4d96f2d3c009923d57769a6
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: ec9d02dba464115cfdede2bcbca4ec108246e12b2a2cc3794c166040aca29898
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_099fc04dea1357ac
  evidence.sdk-language-current-first-gate-failure#source:cad0b0801af06434db84b521caf98183b99ee1193c00f60ba54a3a2b8ef34d9c:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 231c2c821de7222e9359c3606a283705ff0a5dc8d06a1bfd2e0580854e8285bd
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-language-current-first-gate/task-host.json
      historical_git_mode: '100644'
      owner: evidence.sdk-language-current-first-gate-failure
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: e3dc9678c9b436412b1ea0bf0302853959e43c73ffd451c7179261b208f6a42e
      owner_fingerprint: 2079a5626ac5d7b1d342751aeb7ee69d7c007783b4d96f2d3c009923d57769a6
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: cad0b0801af06434db84b521caf98183b99ee1193c00f60ba54a3a2b8ef34d9c
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_a20d2ec69661782e
  evidence.sdk-language-current-first-gate-failure#source:76c3fba61f303c6ccb6f5c74a2bd27f8f859ba4db991dcf2feb94bd1c3d9c577:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: d1aab3350efd6f7eaf096a927459cccd15046ecc60505cd201873ccfa024288d
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-language-current-first-gate/task-fixture.json
      historical_git_mode: '100644'
      owner: evidence.sdk-language-current-first-gate-failure
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 131633d0f08d1ee0967748f54cea206d6a14d46252d3ef50f6c00114901c5a46
      owner_fingerprint: 2079a5626ac5d7b1d342751aeb7ee69d7c007783b4d96f2d3c009923d57769a6
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 76c3fba61f303c6ccb6f5c74a2bd27f8f859ba4db991dcf2feb94bd1c3d9c577
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_1fedc4dd3f13963e
  evidence.sdk-llm-workflow-stream#source:594ab84a723a1b513ec1939b99a00a745c698396807110c32475ebab83571c3e:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: c1192c90d5a26671215cfdc0f9c75e9f166b8050c3486235144815227ca6e9d2
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-llm-workflow-stream#source:d9d57f5234c5f6478af52ad529fd9f2ce0bfdfb354580be8de93080597304f72:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 10f4ce04a16726144f9b8d8d7796ac271612a2c53f44cd0dce2d10a050dcfa15
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-llm-workflow-stream#source:d44e9ff012033e928933fa3afd158c8e38267ac29e35c72d7d2d5f1dd0c9d93e:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 0a9931d87a5913a841852c7c90d922407b4a8b3d6856ae601029a9f5b9191d0e
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-llm-workflow-stream#source:f4ab18a9c1869e4ecc2ae91c4ed48cfeb40e8517e93fa1e56e802e433ed247fd:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 09ede4e6825f0d6918f52f3f2e4986bd9a523c185cba6d0faab89524f7a7b865
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/llm-workflow-stream/raw/receipt.json
      historical_git_mode: '100644'
      owner: evidence.sdk-llm-workflow-stream
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 58e97d9406d95d60006fc67406cf5e4bcbccc9109c04693c56f1099cd9f4adf8
      owner_fingerprint: b7707fddc019e7b328833bc5a2d8241d1a0dc5e36ac9fc839fcb9f7d1de6eadb
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: f4ab18a9c1869e4ecc2ae91c4ed48cfeb40e8517e93fa1e56e802e433ed247fd
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_6ddb35c85b430a73
  evidence.sdk-llm-workflow-stream#source:e7a8383715a79bd1e84a3d0a111336cf30333f25fb00e12044ecd24ac4e0863c:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 7f800879f7b4e22a5558c19c305e276d6904cdc6abc08ab6db54d1a95ea66d7e
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/llm-workflow-stream/README.md
      historical_git_mode: '100644'
      owner: evidence.sdk-llm-workflow-stream
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 03df4bd3ca3c40c2ddc09585dd0014ff660a34de2e95aad4ef2309f2f37a4ac9
      owner_fingerprint: b7707fddc019e7b328833bc5a2d8241d1a0dc5e36ac9fc839fcb9f7d1de6eadb
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: e7a8383715a79bd1e84a3d0a111336cf30333f25fb00e12044ecd24ac4e0863c
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_83775c6096ded148
  evidence.sdk-llm-workflow-stream#source:de443e5a5f1fa2ba47dd16e439f88e14c2da84845b057f6d3fed09ea08843e32:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 76d57e4425ac5930c983a5047dbc624909022cd6752271ccda68b3d8f752031f
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-successor-model-reviews/llm-workflow-stream.json
      historical_git_mode: '100644'
      owner: evidence.sdk-llm-workflow-stream
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 51987b719e327172101321c651075aa53cb47d4e0845e24894369f66cae6f339
      owner_fingerprint: b7707fddc019e7b328833bc5a2d8241d1a0dc5e36ac9fc839fcb9f7d1de6eadb
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: de443e5a5f1fa2ba47dd16e439f88e14c2da84845b057f6d3fed09ea08843e32
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_88bf2e260ac93042
  evidence.sdk-llm-workflow-stream#source:9824cc09e892d3f70b2e55494160ee52e52ca24270b07336f9e4f790ea9397ce:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: faa04fa7298699283ce9bf97d8c27a9ee39c4e1fda3ff5c8eca1e25ada774795
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/final-receipt.json
      historical_git_mode: '100644'
      owner: evidence.sdk-llm-workflow-stream
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 9a27fdbe82d7b6488ac9ac698e1f2e93e06b4c038d5144306079bdb35e284bee
      owner_fingerprint: b7707fddc019e7b328833bc5a2d8241d1a0dc5e36ac9fc839fcb9f7d1de6eadb
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 9824cc09e892d3f70b2e55494160ee52e52ca24270b07336f9e4f790ea9397ce
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_935d147da621a6f2
  evidence.sdk-managed-retirement#source:594ab84a723a1b513ec1939b99a00a745c698396807110c32475ebab83571c3e:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: c1192c90d5a26671215cfdc0f9c75e9f166b8050c3486235144815227ca6e9d2
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-managed-retirement#source:d9d57f5234c5f6478af52ad529fd9f2ce0bfdfb354580be8de93080597304f72:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 10f4ce04a16726144f9b8d8d7796ac271612a2c53f44cd0dce2d10a050dcfa15
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-managed-retirement#source:d44e9ff012033e928933fa3afd158c8e38267ac29e35c72d7d2d5f1dd0c9d93e:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 0a9931d87a5913a841852c7c90d922407b4a8b3d6856ae601029a9f5b9191d0e
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-managed-retirement#source:0e5a68a9024fe9f257439adf51412b231877212bbbb7ed15bd4103bf8baa32f7:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 599f0900032c260859772317e09db1ab6763ea74ca2c5b377152edae57e6edc1
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/managed-retirement-evidence/raw/receipt.json
      historical_git_mode: '100644'
      owner: evidence.sdk-managed-retirement
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 26f3ab5d735c34c5e93ec4ad29b729adef374f417419e8697f9ee2d664e7c532
      owner_fingerprint: 0c698f45a089bc1691d4a367688cb7ee402bf91b9415ba5d2081045b3054e784
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 0e5a68a9024fe9f257439adf51412b231877212bbbb7ed15bd4103bf8baa32f7
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_344f5d1c306040a8
  evidence.sdk-managed-retirement#source:9c794988b68140d3a0e87a5f43b8a2f9dd2310aa5c70822d6ffa98e8f26f3f46:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 292c5e77515feef8a43732ba54987e7c377f416142df5b0df517541e0d6b14d0
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/managed-retirement-evidence/raw/source-freeze.json
      historical_git_mode: '100644'
      owner: evidence.sdk-managed-retirement
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 7caea7c40025ad80285dd2927d8c5bbae57a2849e998a8d59c81f5b6023ce00a
      owner_fingerprint: 0c698f45a089bc1691d4a367688cb7ee402bf91b9415ba5d2081045b3054e784
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 9c794988b68140d3a0e87a5f43b8a2f9dd2310aa5c70822d6ffa98e8f26f3f46
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_436d549e8da7451f
  evidence.sdk-managed-retirement#source:61b3fe7ec715a9627125998832157b6de800e1ec8d8df6cf1751109a0e0fd424:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: c55e0a2c0dc2bec396f89aa16a8b8598505fe009b41ecc828ce42d9879231e5d
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/managed-retirement-evidence/README.md
      historical_git_mode: '100644'
      owner: evidence.sdk-managed-retirement
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 08c69ef68af950166161f9c668c81f45dc7d01516e499aa3b7527d4881b2fa95
      owner_fingerprint: 0c698f45a089bc1691d4a367688cb7ee402bf91b9415ba5d2081045b3054e784
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 61b3fe7ec715a9627125998832157b6de800e1ec8d8df6cf1751109a0e0fd424
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_3ba2e9f46b3f7c4b
  evidence.sdk-managed-retirement#source:b7a0eafcc8e7e4f206495718c8d27f97b0c09e8c4aa91d92871dc794116abcad:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 3277e3aa1950eb410bd592b7dec55c776fe9ba9b0ec9f8afa735f585c00e268c
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-successor-model-reviews/managed-retirement.json
      historical_git_mode: '100644'
      owner: evidence.sdk-managed-retirement
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: e3cd57d1ded3774aaa27275221703c4ad5f36ac3cdbed3efbb4f32abeca0baad
      owner_fingerprint: 0c698f45a089bc1691d4a367688cb7ee402bf91b9415ba5d2081045b3054e784
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: b7a0eafcc8e7e4f206495718c8d27f97b0c09e8c4aa91d92871dc794116abcad
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_974cc641a8ca0e99
  evidence.sdk-managed-retirement#source:9824cc09e892d3f70b2e55494160ee52e52ca24270b07336f9e4f790ea9397ce:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: faa04fa7298699283ce9bf97d8c27a9ee39c4e1fda3ff5c8eca1e25ada774795
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/final-receipt.json
      historical_git_mode: '100644'
      owner: evidence.sdk-managed-retirement
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: f4d2e988c8006cc02f389c884180b1c45835966ae8c30569ba243779c5f27279
      owner_fingerprint: 0c698f45a089bc1691d4a367688cb7ee402bf91b9415ba5d2081045b3054e784
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 9824cc09e892d3f70b2e55494160ee52e52ca24270b07336f9e4f790ea9397ce
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_685bee0ed9e261c8
  evidence.sdk-managed-supporting-operation-boundary#source:3179ad54da4e06914a532759f15be7bc280bfbcff9e894f3b5db812532d55921:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: c177fc5ca8ac4e68736ab97a4fcde410f1e00af85c330d5faa9496c5d6835ba2
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-managed-supporting-operation-boundary#source:f3bce075cd244d691ef03619ec9e1d5e86da5399a6d159c00e14f3b522b6590e:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: bf53d2a9a90c363f18b34c627c51a0ac59a0a1e3da69fe3c682c36c2289fd40c
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-managed-supporting-operation-boundary#source:505b5dbe29f4756a8b9c938ab01aeb758199d2fe48840a4026c8b98561605150:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: bb2be729300a2d70667507e643f18925374eb3a4168c02ae541124299c39e3b8
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-managed-supporting-operation-boundary#source:13c2cb112903c1d980955db527084cd8a00d9265a760d30e864b92ac6e7ab9d9:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 2f9799e51ce18912fb5de9d2ea7ec5240825b1c490add94a70c63adb7604ed0b
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-mcp-channel-owned-candidate#source:594ab84a723a1b513ec1939b99a00a745c698396807110c32475ebab83571c3e:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: c1192c90d5a26671215cfdc0f9c75e9f166b8050c3486235144815227ca6e9d2
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-mcp-channel-owned-candidate#source:d9d57f5234c5f6478af52ad529fd9f2ce0bfdfb354580be8de93080597304f72:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 10f4ce04a16726144f9b8d8d7796ac271612a2c53f44cd0dce2d10a050dcfa15
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-mcp-channel-owned-candidate#source:d44e9ff012033e928933fa3afd158c8e38267ac29e35c72d7d2d5f1dd0c9d93e:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 0a9931d87a5913a841852c7c90d922407b4a8b3d6856ae601029a9f5b9191d0e
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-mcp-channel-owned-candidate#source:145cb11d1c5c7f0d366b4196841c21d8541fe8736dd7099037804c12043e84e3:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 37c2ff5033584d6ee57025f14438c0eed5ce675a497d01b1e60801237abc0a4a
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-mcp-channel-20261013/owned-channel/manifest.json
      historical_git_mode: '100644'
      owner: evidence.sdk-mcp-channel-owned-candidate
      source_role: documentation
      observed_revision: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
      issue_id: 0fe99da6d1a0f5144a5cff4a142dc43b58a2bdaa6467648f61cb7c9e271a4c3a
      owner_fingerprint: acd704c9a0570630049f375313226a2e6bb7ddfb2169c10c44d3ff95b398c647
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 145cb11d1c5c7f0d366b4196841c21d8541fe8736dd7099037804c12043e84e3
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_d0cd3a8049c70a83
  evidence.sdk-mcp-channel-owned-candidate#source:1ff365a8b950870ae9b75ffdc450224334f3681a7d4a7608c631ce0ded0b770a:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 384351793dfaad6d60f70d5530c9f87422141552cc47839b2ef4fc0d8b6716d7
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-mcp-channel-20261013/owned-channel/successor-native/typescript/run.log
      historical_git_mode: '100644'
      owner: evidence.sdk-mcp-channel-owned-candidate
      source_role: documentation
      observed_revision: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
      issue_id: c0e05539550aa544be6206c0a0c733a237a9ad4b100c2748a6f25a6f1de15027
      owner_fingerprint: acd704c9a0570630049f375313226a2e6bb7ddfb2169c10c44d3ff95b398c647
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 1ff365a8b950870ae9b75ffdc450224334f3681a7d4a7608c631ce0ded0b770a
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_00cfbebaf92231ea
  evidence.sdk-mcp-channel-owned-candidate#source:59c7eea338a6d09f910b7f650b85bc7b3b3262a5549b3be1d11547165c0a90d4:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 425aec4a0e1cdf2b6cfb5b9b11178703d031149feffff4b6954db018d44ed156
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-mcp-channel-20261013/owned-channel/successor-native/python/run.log
      historical_git_mode: '100644'
      owner: evidence.sdk-mcp-channel-owned-candidate
      source_role: documentation
      observed_revision: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
      issue_id: 746e41518d360622fccc967914ec7df4b735d8badbdcc5d92b7b7c4dc64fd55e
      owner_fingerprint: acd704c9a0570630049f375313226a2e6bb7ddfb2169c10c44d3ff95b398c647
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 59c7eea338a6d09f910b7f650b85bc7b3b3262a5549b3be1d11547165c0a90d4
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_f119fef9e40e75ca
  evidence.sdk-mcp-channel-owned-candidate#source:fb422fbd1a189ba13d2a1756e366176ac111426d138de5f1c9c14827f9952b11:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 8cd30af8f957b19be88dd92ffe5eb33af0509255d5dd649fa7bf4e4972ff204f
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-mcp-channel-20261013/owned-channel/successor-native/java/run.log
      historical_git_mode: '100644'
      owner: evidence.sdk-mcp-channel-owned-candidate
      source_role: documentation
      observed_revision: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
      issue_id: 80d005ad882f3fcc8043789c04b2ce430584c309eaa17e18a93198377c961b45
      owner_fingerprint: acd704c9a0570630049f375313226a2e6bb7ddfb2169c10c44d3ff95b398c647
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: fb422fbd1a189ba13d2a1756e366176ac111426d138de5f1c9c14827f9952b11
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_f826f0a770744312
  evidence.sdk-mcp-channel-owned-candidate#source:4bf65a5561508a8fb8c5aa141a5b3e1a83bcca12abed237ba14355178535fd2f:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 147ad39084a0fbf001e177536a28d1f6068ae01e00588478c48c90aaee92c2f5
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    current_reference_replacements:
    - raw_reference: docs/sdk/governance/evidence/sdk-mcp-channel-20261013/owned-channel/successor-native/TEST-com.echoagent.sdk.McpChannelHostTest.xml
      historical_git_mode: '100644'
      owner: evidence.sdk-mcp-channel-owned-candidate
      source_role: test_consumer
      observed_revision: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
      issue_id: ad6f5492b7cdbe8d0c808b3da65d85099215800a2c52d53578c8119ece541ed9
      owner_fingerprint: acd704c9a0570630049f375313226a2e6bb7ddfb2169c10c44d3ff95b398c647
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 4bf65a5561508a8fb8c5aa141a5b3e1a83bcca12abed237ba14355178535fd2f
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_96830035120dc7db
      replacement_source_ref: sdks/java/src/test/java/com/echoagent/sdk/McpChannelHostTest.java
      replacement_content_sha256: 90033f01a7c26af4e4bd2c9e447beca1176337c57a612a8a7dd13485b1723e6c
      replacement_git_mode: '100644'
  evidence.sdk-mcp-channel-owned-candidate#source:9824cc09e892d3f70b2e55494160ee52e52ca24270b07336f9e4f790ea9397ce:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: faa04fa7298699283ce9bf97d8c27a9ee39c4e1fda3ff5c8eca1e25ada774795
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/final-receipt.json
      historical_git_mode: '100644'
      owner: evidence.sdk-mcp-channel-owned-candidate
      source_role: documentation
      observed_revision: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
      issue_id: 65590f484cac9cfba4a652132bc3c910aa1fa72940e2ccc740ec20c3af77faa4
      owner_fingerprint: acd704c9a0570630049f375313226a2e6bb7ddfb2169c10c44d3ff95b398c647
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 9824cc09e892d3f70b2e55494160ee52e52ca24270b07336f9e4f790ea9397ce
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_ce8e35e13013cc8c
  evidence.sdk-mcp-channel-owned-candidate#source:5e0f153848725261abf228db5320a840d3b0afef6b72f1b8f076a02cdf6eded3:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: afa85d09a95bf2e8264d8e410d8bb423c21bd3f0b0c94e420e11c074642fbdfa
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/source-build-manifest.json
      historical_git_mode: '100644'
      owner: evidence.sdk-mcp-channel-owned-candidate
      source_role: documentation
      observed_revision: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
      issue_id: 63a8b14029e08bcebbd76c80b0cfacb11fbf4b7a40089cc13d010141e8d2c234
      owner_fingerprint: acd704c9a0570630049f375313226a2e6bb7ddfb2169c10c44d3ff95b398c647
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 5e0f153848725261abf228db5320a840d3b0afef6b72f1b8f076a02cdf6eded3
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_30d12f07f546b569
  evidence.sdk-memory-permission-candidate#source:5a8661153b387d16aa6aa795296fb1f4102930a8032f228e00bb2e533b8f4fac:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 5e18a22d644bf1f5825c59856caeef67845def7436237b837455a5f9fdd6e6e2
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-memory-permission-candidate#source:7eb9ba4c44817b14bf43dbe55ed2ab3c9b3eb1ec920e4f83d8a603793b152212:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 0c120e567c46f052f4ca5037df84402a5ca07e6acc580faba8d8544f7a8ab1a7
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-memory-permission-candidate#source:9cdfc8a355648bdccf35110d413fcc5966d8f523e9f9c549ab57957c2fe1a426:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: a06f7742028c992226a6d07735ef1cab0eec10b679125ecb17cfa1de31fdf769
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-memory-permission-candidate#source:594ab84a723a1b513ec1939b99a00a745c698396807110c32475ebab83571c3e:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: c1192c90d5a26671215cfdc0f9c75e9f166b8050c3486235144815227ca6e9d2
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-memory-permission-candidate#source:d9d57f5234c5f6478af52ad529fd9f2ce0bfdfb354580be8de93080597304f72:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 10f4ce04a16726144f9b8d8d7796ac271612a2c53f44cd0dce2d10a050dcfa15
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-memory-permission-candidate#source:d44e9ff012033e928933fa3afd158c8e38267ac29e35c72d7d2d5f1dd0c9d93e:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 0a9931d87a5913a841852c7c90d922407b4a8b3d6856ae601029a9f5b9191d0e
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-memory-permission-candidate#source:e30ad679d2ba74c3a8d9d63154449c409a401f042a317373b8f291219d09cdff:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 5b7bd1366039e322e9b56fac3b6b1732f8aa6d7407e1f3378afc8945822ae2f6
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-memory-permission-20261013/formal-manifest.json
      historical_git_mode: '100644'
      owner: evidence.sdk-memory-permission-candidate
      source_role: documentation
      observed_revision: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
      issue_id: 351df218f2a4b99d33ea07aa140de847e21d78d0595c555c2b292c4b6226e8fd
      owner_fingerprint: ca27978a3e7803e0de19ad4c5e2ae5975452d640573805559df169ffcce89422
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: e30ad679d2ba74c3a8d9d63154449c409a401f042a317373b8f291219d09cdff
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_02cfe0445b55c180
  evidence.sdk-memory-permission-candidate#source:07df10f13df041c6d39ccae32718e4ddae8cf669856773a2d1be953c97d54bd5:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: f574565775437cdf4c2458be645503d5c40ed4bb051db05b279cecb0293c52c0
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    current_reference_replacements:
    - raw_reference: docs/sdk/governance/evidence/sdk-memory-permission-20261013/formal/TEST-com.echoagent.sdk.MemoryPermissionHostTest.xml
      historical_git_mode: '100644'
      owner: evidence.sdk-memory-permission-candidate
      source_role: test_consumer
      observed_revision: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
      issue_id: c103ed77282a69dfb5fb28d93a883cb09636e7fb2a4ebb6829a09de3ae01fd80
      owner_fingerprint: ca27978a3e7803e0de19ad4c5e2ae5975452d640573805559df169ffcce89422
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 07df10f13df041c6d39ccae32718e4ddae8cf669856773a2d1be953c97d54bd5
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_35453088cbce6c56
      replacement_source_ref: sdks/java/src/test/java/com/echoagent/sdk/MemoryPermissionHostTest.java
      replacement_content_sha256: b93576185fc4b94013ce8b60147f3beb6929dd80930b42877af57a7d9cdf4366
      replacement_git_mode: '100644'
  evidence.sdk-memory-permission-candidate#source:d4d3fb8516a0855fc5df0972ec773364b12c6e41b32503ff63a85524555dd9f8:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: b0b7364aaf642e0986624f4ef00df4fb0a24d3d230d1564b5c9c675e4c22ba24
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-memory-permission-20261013/formal/cross-language-comparison.json
      historical_git_mode: '100644'
      owner: evidence.sdk-memory-permission-candidate
      source_role: documentation
      observed_revision: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
      issue_id: dcab0e109e2b594334539c96c6da91e26aacfdb228f057a1387ba7ef5997f3cd
      owner_fingerprint: ca27978a3e7803e0de19ad4c5e2ae5975452d640573805559df169ffcce89422
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: d4d3fb8516a0855fc5df0972ec773364b12c6e41b32503ff63a85524555dd9f8
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_cfa3afcb723a1c95
  evidence.sdk-memory-permission-candidate#source:cc094d09df6f43dd71e1589dba864b8e51d8a11f59c77cb7562d17ed4a327625:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 7857cd92c921590940187f3c0bc5e84aa333c51f3c1830cd5840240182aa1841
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-memory-permission-20261013/formal/sdk-memory-permission-formal-java.log
      historical_git_mode: '100644'
      owner: evidence.sdk-memory-permission-candidate
      source_role: documentation
      observed_revision: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
      issue_id: f0c6bfd7360b77aa9a1555c8645b52b98311bd461ed6bf29a213db49a3a773c4
      owner_fingerprint: ca27978a3e7803e0de19ad4c5e2ae5975452d640573805559df169ffcce89422
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: cc094d09df6f43dd71e1589dba864b8e51d8a11f59c77cb7562d17ed4a327625
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_c3dcd9e6b547ddfe
  evidence.sdk-memory-permission-candidate#source:9824cc09e892d3f70b2e55494160ee52e52ca24270b07336f9e4f790ea9397ce:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: faa04fa7298699283ce9bf97d8c27a9ee39c4e1fda3ff5c8eca1e25ada774795
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/final-receipt.json
      historical_git_mode: '100644'
      owner: evidence.sdk-memory-permission-candidate
      source_role: documentation
      observed_revision: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
      issue_id: d94b6a1ae63b351499c1c50f9985f4408e0ae5ee03acc0cb91773e7ec56b4b43
      owner_fingerprint: ca27978a3e7803e0de19ad4c5e2ae5975452d640573805559df169ffcce89422
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 9824cc09e892d3f70b2e55494160ee52e52ca24270b07336f9e4f790ea9397ce
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_36180e4d8873dd6c
  evidence.sdk-memory-permission-candidate#source:5e0f153848725261abf228db5320a840d3b0afef6b72f1b8f076a02cdf6eded3:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: afa85d09a95bf2e8264d8e410d8bb423c21bd3f0b0c94e420e11c074642fbdfa
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/source-build-manifest.json
      historical_git_mode: '100644'
      owner: evidence.sdk-memory-permission-candidate
      source_role: documentation
      observed_revision: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
      issue_id: cbb98093d46b9fe6455aeefe7dac89b6f6c0f3367ce15e3b0846872055b95764
      owner_fingerprint: ca27978a3e7803e0de19ad4c5e2ae5975452d640573805559df169ffcce89422
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 5e0f153848725261abf228db5320a840d3b0afef6b72f1b8f076a02cdf6eded3
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_604e9893f012faa8
  evidence.sdk-official-current-language-gate#source:594ab84a723a1b513ec1939b99a00a745c698396807110c32475ebab83571c3e:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: c1192c90d5a26671215cfdc0f9c75e9f166b8050c3486235144815227ca6e9d2
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-official-current-language-gate#source:ea0a0f1a7d027c304ea8d146970cd424d0842269ea27842ad4451b8ac5bd3226:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: c49462a4366a28ecda4279f3d18ec3fdf67ba5e2673cd74a0df0df78b129e22b
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-official-current-language-gate#source:d44e9ff012033e928933fa3afd158c8e38267ac29e35c72d7d2d5f1dd0c9d93e:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 0a9931d87a5913a841852c7c90d922407b4a8b3d6856ae601029a9f5b9191d0e
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-official-current-language-gate#source:5e0f153848725261abf228db5320a840d3b0afef6b72f1b8f076a02cdf6eded3:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: afa85d09a95bf2e8264d8e410d8bb423c21bd3f0b0c94e420e11c074642fbdfa
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-official-current-language-gate#source:cbf90de4b8b415dd26a45b7d5ffd75e5f42fe433e2345e3f076a77c1ed0996f5:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: aa2ea26996075e2c338b8d2489973b73f17153242dbac80c48f7faa0acef31eb
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-official-current-language-gate#source:ca0f1cc09da0fafe172b45d2679acbf0ce7ee2f7c2f876fc5acae3ba99f5eee0:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 730a393c62e619c4ea5057d91143374cf3305cc47e9b0feeabdd8365d8a11c6c
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-language-test-nop-complete/receipt.json
      historical_git_mode: '100644'
      owner: evidence.sdk-official-current-language-gate
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 7c7ce7ad65187aeb742f572c5cd28eb6e8ba0c09951127bc32ef770b48f0ed9b
      owner_fingerprint: 7a20a353de86b5abe97e9b08d6fd171740c9cfe5be9aa6c07d37da0b486305cb
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: ca0f1cc09da0fafe172b45d2679acbf0ce7ee2f7c2f876fc5acae3ba99f5eee0
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_85a5282044e78be7
  evidence.sdk-official-current-language-gate#source:9478cd0183aa6a57e6e0d788377cf5e9b8f6ef34e9ebee9c44b6d3d183560b39:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 5e9adef8ecbaabe79b718f8695bfd66c060970df59b97b8c572d685e41201d27
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-language-test-nop-complete/run.json
      historical_git_mode: '100644'
      owner: evidence.sdk-official-current-language-gate
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 3fcac97056aeeace3aca532ce5aba5b18a26a522f97d4736a0c14bf527efe1af
      owner_fingerprint: 7a20a353de86b5abe97e9b08d6fd171740c9cfe5be9aa6c07d37da0b486305cb
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 9478cd0183aa6a57e6e0d788377cf5e9b8f6ef34e9ebee9c44b6d3d183560b39
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_43b5cedd0f254ba7
  evidence.sdk-official-current-language-gate#source:42332e05bb429e5539938b393adc2caeb320991be8f9bfb021d175e05ec684ef:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 8021d8ff0c3cae9d9f06fc5565bbfeb5a66af6344e3679699b5df78e0d7ce652
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-language-test-nop-complete/full-language.log
      historical_git_mode: '100644'
      owner: evidence.sdk-official-current-language-gate
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 6496c60f6a0bc8ce83bd54e60940988db9e02dc499e54a208b44a8225b7ffcec
      owner_fingerprint: 7a20a353de86b5abe97e9b08d6fd171740c9cfe5be9aa6c07d37da0b486305cb
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 42332e05bb429e5539938b393adc2caeb320991be8f9bfb021d175e05ec684ef
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_914e507affb06f24
  evidence.sdk-official-current-language-gate#source:8fe8eca1376dcc597d597e01322fa0a9747f85f35f7c662c000b69089a3dde4b:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 8921e3656b8ee011631d8737563f5b9363f394791cd0baef3b61ed61bf8255dc
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-language-test-nop-complete/summary.json
      historical_git_mode: '100644'
      owner: evidence.sdk-official-current-language-gate
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 790a1a5a61ecf64457fabdc3dd0dd5439e447b083ed825646940c89f6e18e510
      owner_fingerprint: 7a20a353de86b5abe97e9b08d6fd171740c9cfe5be9aa6c07d37da0b486305cb
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 8fe8eca1376dcc597d597e01322fa0a9747f85f35f7c662c000b69089a3dde4b
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_96dc60bedb36a132
  evidence.sdk-official-current-language-gate#source:3e0e875d01cbaff197f211376b2dba4780ec0a90c0492a1a36bf54c84b1c55d1:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: cfb6c07c171346cb2c9d2009bbbf665e3adae2909200eccba8b70113858f83e4
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-language-test-nop-complete/java-summary.json
      historical_git_mode: '100644'
      owner: evidence.sdk-official-current-language-gate
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 5078eac3c2d8c1cca26053ef5caf8c560c1d0140fb13afe6eebbe22730fd1b80
      owner_fingerprint: 7a20a353de86b5abe97e9b08d6fd171740c9cfe5be9aa6c07d37da0b486305cb
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 3e0e875d01cbaff197f211376b2dba4780ec0a90c0492a1a36bf54c84b1c55d1
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_88e1d8dad9b52f54
  evidence.sdk-official-current-language-gate#source:aec487c70d358f03591222486c350330893b47549e98eff0e599f9ab1351ead8:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 4f65afee0d5a2748da87b74bec687f0eef996324e0d8b919868f7642ae885e5a
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-language-test-nop-complete/source-before.json
      historical_git_mode: '100644'
      owner: evidence.sdk-official-current-language-gate
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 8db42da5961932703bc07bb1e2ca07dcd9d9c37d8d81e29077ed69bb5d860393
      owner_fingerprint: 7a20a353de86b5abe97e9b08d6fd171740c9cfe5be9aa6c07d37da0b486305cb
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: aec487c70d358f03591222486c350330893b47549e98eff0e599f9ab1351ead8
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_a12d23136032dbe0
  evidence.sdk-official-current-language-gate#source:9acce8b4d039309c7d70b00edb83dc6b73484bfbd9cde946e1a4dbf518ebbf35:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 5c85319575d9fee878ea40b315a9108b1a27a7748ecd6839d27f02cba828aea2
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-language-test-nop-complete/source-after.json
      historical_git_mode: '100644'
      owner: evidence.sdk-official-current-language-gate
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: ffb86f4e0d461a19f220ae839f1ee9c87ce44b0fd72296c902478e990a3fbf5f
      owner_fingerprint: 7a20a353de86b5abe97e9b08d6fd171740c9cfe5be9aa6c07d37da0b486305cb
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 9acce8b4d039309c7d70b00edb83dc6b73484bfbd9cde946e1a4dbf518ebbf35
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_dae757b0313aca31
  evidence.sdk-official-current-language-gate#source:d78215187166eebedba51642b9bc18ce51ccb197f139ecba5b5ab785e6a737fd:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 5e1fe59364e24d0196ae02cf703ae7633b8825833b56f7aef44bb8d47f45d620
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-language-test-nop-complete/ARCHIVE-SHA256SUMS.json
      historical_git_mode: '100644'
      owner: evidence.sdk-official-current-language-gate
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 7fd5347ee76779b43aa4b6f4585b8c953c0bdbeddd4afaa0c6d8cc68045c97ed
      owner_fingerprint: 7a20a353de86b5abe97e9b08d6fd171740c9cfe5be9aa6c07d37da0b486305cb
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: d78215187166eebedba51642b9bc18ce51ccb197f139ecba5b5ab785e6a737fd
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_0bfa4ecabd35170f
  evidence.sdk-official-current-language-gate#source:5f45fccd35cee927c4102cc4253f8a5febd1b01388baff3dd3b307c29d2a42aa:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 56979f5c18ca5c4c2adcccefc3c95b781c316b44fe2939a24c85e9783c59631c
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-language-test-nop-complete/warning-classification.json
      historical_git_mode: '100644'
      owner: evidence.sdk-official-current-language-gate
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 31af472f04cbaf5ac187ebfa33cfc224cc43569e903675b8c2aa1880b21ff50f
      owner_fingerprint: 7a20a353de86b5abe97e9b08d6fd171740c9cfe5be9aa6c07d37da0b486305cb
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 5f45fccd35cee927c4102cc4253f8a5febd1b01388baff3dd3b307c29d2a42aa
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_3facabd2b1308c3c
  evidence.sdk-official-current-language-gate#source:ccb138fb61fc5901a9265734ea6d4d0a46c70604d1640e3bd730aba4da9d5c92:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 074702a79b08968df0b9cea89560b936ff033d885884ed1c041c1a9c05f204dc
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-language-test-nop-complete/README.md
      historical_git_mode: '100644'
      owner: evidence.sdk-official-current-language-gate
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: f9725b49f50016aa92ea37bf96c97d6fb36a49e1ed2e9ce2c89a8e0d285ff069
      owner_fingerprint: 7a20a353de86b5abe97e9b08d6fd171740c9cfe5be9aa6c07d37da0b486305cb
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: ccb138fb61fc5901a9265734ea6d4d0a46c70604d1640e3bd730aba4da9d5c92
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_47843cf7892ac8df
  evidence.sdk-official-current-language-gate#source:e1fed55826e7ceda63dc664aab21a1d57e8ca43a291d678f93947e8e97fec796:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 013bbe6933b99021ae2b8653d19cfd822b5e653a776fb84f78e36370cf64e6f5
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-successor-model-reviews/current-official-language-gate.json
      historical_git_mode: '100644'
      owner: evidence.sdk-official-current-language-gate
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 6c3ba770ec6a6e6d7c273163708e1861760d45828f02b0f956e8304b7710b4b5
      owner_fingerprint: 7a20a353de86b5abe97e9b08d6fd171740c9cfe5be9aa6c07d37da0b486305cb
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: e1fed55826e7ceda63dc664aab21a1d57e8ca43a291d678f93947e8e97fec796
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_8ea8725ec2e4a024
  evidence.sdk-official-factory-final-language-gate#source:3419c7b6dfe8b0b138a8c540bc7da45f71beb7bb9a464ed5a28b333e9da8f5f0:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 3e81047ee4c56a09dc511f7f14cd978ce9c9f5b420ba8e4742bc82d64b714d3b
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-official-factory-final-language-gate#source:594ab84a723a1b513ec1939b99a00a745c698396807110c32475ebab83571c3e:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: c1192c90d5a26671215cfdc0f9c75e9f166b8050c3486235144815227ca6e9d2
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-official-factory-final-language-gate#source:45f54d0217bf681d8ad218534e8f6a00a9d2c528f57564cb771d09c098c224fb:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: e4ef2f69ed6f0f0f1efc6a83fd95c674c409229be17654eef76687eebeb2217a
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-official-factory-final-language-gate#source:74246c3ccfe8c6a598c1a7fae8e83470e4c27e130e139ff9ec4a7d398a277977:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: c5932795c787db10d4ebd874e83415385569c32859ca5f457612464cdd47b765
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-official-factory-final-language-gate#source:ea0a0f1a7d027c304ea8d146970cd424d0842269ea27842ad4451b8ac5bd3226:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: c49462a4366a28ecda4279f3d18ec3fdf67ba5e2673cd74a0df0df78b129e22b
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-official-factory-final-language-gate#source:d9d57f5234c5f6478af52ad529fd9f2ce0bfdfb354580be8de93080597304f72:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 10f4ce04a16726144f9b8d8d7796ac271612a2c53f44cd0dce2d10a050dcfa15
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-official-factory-final-language-gate#source:d44e9ff012033e928933fa3afd158c8e38267ac29e35c72d7d2d5f1dd0c9d93e:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 0a9931d87a5913a841852c7c90d922407b4a8b3d6856ae601029a9f5b9191d0e
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-official-factory-final-language-gate#source:2aab3df077dd14f3019548f43215abe554f2b4d1f13b526883ed4cf43673d1a4:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 8c486b5739a54e97ed7f13f1923d1ce48d7f49dbc2e3ca3daf160638a0c595d0
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-official-factory-final-language-gate#source:5e0f153848725261abf228db5320a840d3b0afef6b72f1b8f076a02cdf6eded3:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: afa85d09a95bf2e8264d8e410d8bb423c21bd3f0b0c94e420e11c074642fbdfa
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-official-factory-final-language-gate#source:cbf90de4b8b415dd26a45b7d5ffd75e5f42fe433e2345e3f076a77c1ed0996f5:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: aa2ea26996075e2c338b8d2489973b73f17153242dbac80c48f7faa0acef31eb
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-official-factory-final-language-gate#source:8b734f9472ecd337be6c2fde442ef066c42ab748e844808e314bcdc0cd515948:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 0d514bd6faeb2da0cfc03fe2d4c0148f3908f5c2278dab3421dd93536f4741f4
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-successor-model-reviews/factory-cut-executed-packets.json
      historical_git_mode: '100644'
      owner: evidence.sdk-official-factory-final-language-gate
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: dc161e8863ac11feddf467b4c8554bebdfc6592dc76a43e282ac916da1c13050
      owner_fingerprint: ad0cf08caae5cb7ac9bcd0f7cbff9a0f7e647e96be45a7678d9e643655b36b3c
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 8b734f9472ecd337be6c2fde442ef066c42ab748e844808e314bcdc0cd515948
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_13bf502044445743
  evidence.sdk-ordinary-tools-b909-verification#source:d73ec8f087a19271049ae21f23f48fe5e7fcf2da8405fbfa0e71764b5c55d65d:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 207c9da21813991497e4a4a825851cd0eea0d17beb25a29a842838b28caea2f8
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-ordinary-tools-b909-verification#source:739e09d558e35fb279369c3e4e7b7b8834620ec6f251371cc2fd86ab076e3941:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 016e6a7143e93a41900957efe350fe8c779f23f980f57a9e2d8cbb41c7cd56b2
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-ordinary-tools-b909-verification#source:3f60de6567b93259e9fdcd024175cbc2bd10979f76c089f78d4bb60fd41924a6:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 2e906f9369d6a789e8efee76cc0b396163b951424b7bdf267be874591fea4b36
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-ordinary-tools-b909-verification#source:68b6c4a2b396faecf500fd1c68c9ef28078e9faef20c4ff059ad789205912a38:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: b73b90148b78ef80c6d30d880a25c814e691ddbcdfd5831b0929b5e951d25f05
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-ordinary-tools-b909-verification#source:f161f5f1bdcf932b6335267627505797d4b31b720e4a42179864705ae0ed95a9:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 43419634a289b34363f7f07adcb46eb04a7327ae166774a4b29d3cb2df2e3e11
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-ordinary-tools-b909-verification#source:77c1d305a55e8320692929efca4f48a6d6dc7a4370cd33c9b403da996dcc15c3:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 8c6525a46c313efac3364a140f1fe2ec36e68e14d3ad6a092815883d74c4e7a6
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-ordinary-tools-b909-verification#source:00a3240a4d37a5f3a4f20cf29f18668833194fa9f1c5c410bd5e22bbeddec8da:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: d3650e6871ca064e5d1e4673b10767da149ea1fcfb7340463c1e3b453bf368b3
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-ordinary-tools-b909-verification#source:594ab84a723a1b513ec1939b99a00a745c698396807110c32475ebab83571c3e:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: c1192c90d5a26671215cfdc0f9c75e9f166b8050c3486235144815227ca6e9d2
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-ordinary-tools-b909-verification#source:d9d57f5234c5f6478af52ad529fd9f2ce0bfdfb354580be8de93080597304f72:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 10f4ce04a16726144f9b8d8d7796ac271612a2c53f44cd0dce2d10a050dcfa15
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-ordinary-tools-b909-verification#source:d44e9ff012033e928933fa3afd158c8e38267ac29e35c72d7d2d5f1dd0c9d93e:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 0a9931d87a5913a841852c7c90d922407b4a8b3d6856ae601029a9f5b9191d0e
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-ordinary-tools-b909-verification#source:cbf90de4b8b415dd26a45b7d5ffd75e5f42fe433e2345e3f076a77c1ed0996f5:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: aa2ea26996075e2c338b8d2489973b73f17153242dbac80c48f7faa0acef31eb
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-ordinary-tools-b909-verification#source:36f777a8b161a2cad6e7e4ab485ad9ed4c0bd041c15c3946bcaff89e30b40d2b:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 2083fd3a9cbe3dfb277221f5e131360e6f5a1a7cd8ca51d7de885a177a87b03a
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-successor-model-reviews/ordinary-tools-b909.json
      historical_git_mode: '100644'
      owner: evidence.sdk-ordinary-tools-b909-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 4700784cb8f0679a452eccb90e4dea6a8b81b59d1297be3ec7802baed7ab571e
      owner_fingerprint: 84d1d031a2dfdc6da2646633eb860e89f52d222f87d56f13bb8379b3a8fed22d
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 36f777a8b161a2cad6e7e4ab485ad9ed4c0bd041c15c3946bcaff89e30b40d2b
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_460ca09d307074e3
  evidence.sdk-ordinary-tools-b909-verification#source:475d6480df8e791bb67b7739d427f46c4937e5e6a3cad6d56a08fa959d1402e6:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 8259f7398e88fdf0f0f75b7159ff4d0efb5caa1d22cbdb7cb138f900e8a3a65a
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-successor-model-reviews/framework-database-rag-final-gates.json
      historical_git_mode: '100644'
      owner: evidence.sdk-ordinary-tools-b909-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 5e88577f9bfaa37aeb8111d3f2a512f98f2f1e5a3d60a371c2872a5f13c5bdbd
      owner_fingerprint: 84d1d031a2dfdc6da2646633eb860e89f52d222f87d56f13bb8379b3a8fed22d
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 475d6480df8e791bb67b7739d427f46c4937e5e6a3cad6d56a08fa959d1402e6
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_92ce92f72dba9df0
  evidence.sdk-ordinary-tools-b909-verification#source:9824cc09e892d3f70b2e55494160ee52e52ca24270b07336f9e4f790ea9397ce:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: faa04fa7298699283ce9bf97d8c27a9ee39c4e1fda3ff5c8eca1e25ada774795
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/final-receipt.json
      historical_git_mode: '100644'
      owner: evidence.sdk-ordinary-tools-b909-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 7cf7ff36dfaf9d76770df3f67c60f68233ffc84762d1598b3af5fc93b7766628
      owner_fingerprint: 84d1d031a2dfdc6da2646633eb860e89f52d222f87d56f13bb8379b3a8fed22d
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 9824cc09e892d3f70b2e55494160ee52e52ca24270b07336f9e4f790ea9397ce
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_0e7a518e3e5304b3
  evidence.sdk-ordinary-tools-b909-verification#source:5e0f153848725261abf228db5320a840d3b0afef6b72f1b8f076a02cdf6eded3:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: afa85d09a95bf2e8264d8e410d8bb423c21bd3f0b0c94e420e11c074642fbdfa
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/source-build-manifest.json
      historical_git_mode: '100644'
      owner: evidence.sdk-ordinary-tools-b909-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 3fe819793117a278a3cd5130995b1afa2788f82312bddf29fbcf37d7ba161e19
      owner_fingerprint: 84d1d031a2dfdc6da2646633eb860e89f52d222f87d56f13bb8379b3a8fed22d
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 5e0f153848725261abf228db5320a840d3b0afef6b72f1b8f076a02cdf6eded3
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_bda5b9eb0ea16e57
  evidence.sdk-parent-driver-candidate-verification#source:ce6ff4c33cac7d3e58069b2b234754454395261fc5d20a234acc421e30677349:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 910dc30c57d7f6e7811aecbd0b71be37592ae225d324b58abc73062eb6ae7a9f
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-parent-driver-candidate-verification#source:2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 661b8225f09b8ae7010c6089e5934bbdc59663558165b04b9d95e557a77a5e58
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-parent-driver-candidate-verification#source:875b938aacd0886b84e74db7cde848f22b3d4fb28bdda61fd744d8beff1ce344:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: eca8d04a9eb67dd0cdd76e5cb606c3780203e4191fc0f551d3061a0620e93296
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-parent-driver-candidate-verification#source:be870643edf05900800a083e09a678b367749060138c1295f2aecd4f46faf5fd:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 7b68c18e3bf1b6f334aff5f8fc4bb34fa9eefc33dc001267e290cd48024f20b2
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-parent-driver-candidate-verification#source:4d47c7c9e9d541087e0ef2e48cb9d6a495b357216ee92276ab6a6e267195be37:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: d42e2d8b8c82b56c0ab34ec67abaca6a27ba50ce0ef398567ebc3d6a67ac9328
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-parent-driver-candidate-verification#source:1532ab8b88dc82af92f15ea334ea1cbe64cf7c8aa7248f9315b3387eeda4fa0f:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 78659a201df270f3870f7a0b4f1d414166c5cef89ee75468ad1945e193773cca
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-parent-driver-candidate-verification#source:77c1d305a55e8320692929efca4f48a6d6dc7a4370cd33c9b403da996dcc15c3:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 37a812ee0f27012fdc7f062fcc174250d1166ca9460103abf7b65ab98930e615
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-parent-driver-candidate-verification#source:e4ae808abb8f916a7318faec3643c5a56cd1bbd143e9e88cb447e0cf7127f7de:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: e5b9b782276f244b2dd732c13876ae1ca5d8cb7b727da2d95ee4e6f0fe357300
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-parent-driver-candidate-verification#source:594ab84a723a1b513ec1939b99a00a745c698396807110c32475ebab83571c3e:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: c1192c90d5a26671215cfdc0f9c75e9f166b8050c3486235144815227ca6e9d2
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-parent-driver-candidate-verification#source:d9d57f5234c5f6478af52ad529fd9f2ce0bfdfb354580be8de93080597304f72:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 10f4ce04a16726144f9b8d8d7796ac271612a2c53f44cd0dce2d10a050dcfa15
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-parent-driver-candidate-verification#source:d44e9ff012033e928933fa3afd158c8e38267ac29e35c72d7d2d5f1dd0c9d93e:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 0a9931d87a5913a841852c7c90d922407b4a8b3d6856ae601029a9f5b9191d0e
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-parent-driver-candidate-verification#source:ba5be59d04a790c4323d11a3be24b49e35c8cf47634bc29181d33cc9b25efa59:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 947b24e2c363bc032306e031fd226c1275886f1cb7387ca7cc9c72dbd468c77f
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-parent-driver-20261013/sdk-parent-driver-local-proof.json
      historical_git_mode: '100644'
      owner: evidence.sdk-parent-driver-candidate-verification
      source_role: documentation
      observed_revision: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
      issue_id: 05f15e1c14997fbcb5b488d936d80089aa6d1bc61a12f50f988135b48a9584b0
      owner_fingerprint: d6ce47c3c9b727bc5d7ac8a570ecdfd417d5f856702f8a2fe32eddf125d2714f
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: ba5be59d04a790c4323d11a3be24b49e35c8cf47634bc29181d33cc9b25efa59
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_627e075eda37ed24
  evidence.sdk-parent-driver-candidate-verification#source:f60347ffb46458d76074960f9db8fa060e381b32c385c776f0093bd0445aad60:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 456ae0e0edad7199aac51905e5642af9d0040c30bd0067a1949b058d901f5206
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-parent-driver-20261013/sdk-full-parent-driver-language-proof.json
      historical_git_mode: '100644'
      owner: evidence.sdk-parent-driver-candidate-verification
      source_role: documentation
      observed_revision: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
      issue_id: e3b09300fbb517762f3ed8cd71eba19d7d939745a57e851105be6386468ab15b
      owner_fingerprint: d6ce47c3c9b727bc5d7ac8a570ecdfd417d5f856702f8a2fe32eddf125d2714f
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: f60347ffb46458d76074960f9db8fa060e381b32c385c776f0093bd0445aad60
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_061ff571165cacf0
  evidence.sdk-parent-driver-candidate-verification#source:558c509ff968ca6bdaaae6c8e736e9de318aae4e4cbd5cac983e4dcaeb82efa4:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: fe3c0e44ed758ffbcc99b7dbef1a4a280fa796272f4d3e4dfeb26eeb0d1e47c2
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-parent-driver-20261013/sdk-full-parent-driver-language-gates.log
      historical_git_mode: '100644'
      owner: evidence.sdk-parent-driver-candidate-verification
      source_role: documentation
      observed_revision: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
      issue_id: 9e467cc92d9a52084d5e6729e29f538b8ef1219ef2cd84ed0aee535cccff21dd
      owner_fingerprint: d6ce47c3c9b727bc5d7ac8a570ecdfd417d5f856702f8a2fe32eddf125d2714f
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 558c509ff968ca6bdaaae6c8e736e9de318aae4e4cbd5cac983e4dcaeb82efa4
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_ba4576f22af5a997
  evidence.sdk-parent-driver-candidate-verification#source:7959ef1bf8ffedb10f9b38b1ee232669acf385ffb7b58fa49cc8c62807d71c7c:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: ca28758c96feb2519e6af2df22f76dbb522dc2c64ab216f0cd0b06a1b17dd8ee
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-parent-driver-20261013/current-source-comparison.json
      historical_git_mode: '100644'
      owner: evidence.sdk-parent-driver-candidate-verification
      source_role: documentation
      observed_revision: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
      issue_id: 3110aec1e3337f7a2c5fd81c3e9d9dd662f9eeee2da2ca55e460b5b1dcd61e8e
      owner_fingerprint: d6ce47c3c9b727bc5d7ac8a570ecdfd417d5f856702f8a2fe32eddf125d2714f
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 7959ef1bf8ffedb10f9b38b1ee232669acf385ffb7b58fa49cc8c62807d71c7c
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_378fd6844ffeabcf
  evidence.sdk-parent-driver-candidate-verification#source:9824cc09e892d3f70b2e55494160ee52e52ca24270b07336f9e4f790ea9397ce:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: faa04fa7298699283ce9bf97d8c27a9ee39c4e1fda3ff5c8eca1e25ada774795
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/final-receipt.json
      historical_git_mode: '100644'
      owner: evidence.sdk-parent-driver-candidate-verification
      source_role: documentation
      observed_revision: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
      issue_id: 72a239fad6148f67160344f608e0c174098e72b6e8fd632f0eaa3573cdc4f2fc
      owner_fingerprint: d6ce47c3c9b727bc5d7ac8a570ecdfd417d5f856702f8a2fe32eddf125d2714f
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 9824cc09e892d3f70b2e55494160ee52e52ca24270b07336f9e4f790ea9397ce
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_24bebc0243550b36
  evidence.sdk-parent-driver-candidate-verification#source:5e0f153848725261abf228db5320a840d3b0afef6b72f1b8f076a02cdf6eded3:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: afa85d09a95bf2e8264d8e410d8bb423c21bd3f0b0c94e420e11c074642fbdfa
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/source-build-manifest.json
      historical_git_mode: '100644'
      owner: evidence.sdk-parent-driver-candidate-verification
      source_role: documentation
      observed_revision: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
      issue_id: 0fec9885f27e50197cb5cf331b060798312eae4992eaaddfb012aba9590c9bad
      owner_fingerprint: d6ce47c3c9b727bc5d7ac8a570ecdfd417d5f856702f8a2fe32eddf125d2714f
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 5e0f153848725261abf228db5320a840d3b0afef6b72f1b8f076a02cdf6eded3
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_233c9c561127ee07
  evidence.sdk-protocol-finite-observation#source:c683f51728c241be24d1d3e4bc0d4448b7dd3fd10541f63a50238d753c8ce995:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: bbaef1aed143e647998db496ac7e6d27b03fcb8734d10488969e6416f451dd78
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-protocol-finite-observation#source:8332f6ccc7be4a2ce1a0143c31bfcf9df4b89e015dc8cf64cc2f1927bca9b92b:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 5cf181e287b1b2ec7d9a0b4b56ac7225bcbf44ecc644e518587723cfd19c6535
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-protocol-finite-observation#source:67198f101ec8c4afeb9129d4663eaa03a97fc2182a5728557c66c9b786224ec0:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 8cb6ac1499c6a8c4c83235d1eff7c0f6d0e7cec175d30fd66e62ca7635b6dfb4
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-protocol-finite-observation#source:3a61455bd4fc7d25b15c57250b785a4588ecc9ef78797653060ba19b7182bf85:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: e5fd3f291ca08a17caef0dcecf8b7d9cbca9882cb9ec398531b67592ebf0f2f7
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-protocol-finite-observation#source:aed53c74d9708ca79a05831f33d3226f4630ded85ffb6042bb2455dbd2fa42f8:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 5ff0a18e43318cbc776c19ba8595eedc1423110d464aa2bca25c8bb301e69568
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-protocol-finite-observation#source:34d1d0db0d4bdd7e1f203018a01d0b02d77543fd5d19d571c51d5a30db4a57a5:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: a31f597f87551bfbffaaed8dbe7736887678adb8ebb82ef4bce30d1de9745d6a
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-protocol-finite-observation#source:1d04828d71ca713869184c17cb32318c3cd80a23e9223c2b1cc54a4c586098f9:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: e56206dc2a78477fd48e9382a69222b8b446e0478ddee06d5fc2ab99e755e690
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-protocol-finite-observation#source:c8d3dbca61728dc025e90c3c56e8fd6f9d9b1609afbb586fe3a4ae96cea62296:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: ce40e9c637d986d6d1e83ec9cb0eab3af567940eb87c5792733fa58f0d353a7f
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-protocol-finite-observation#source:96a563f62101632cd881013bbeba0b352647445bc6ac467f53fccead3b3b1794:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 4d0e303fc452e717b4eb977c84248ac2117314e12423a39ba8be10f17577f82b
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-protocol-finite-observation#source:d8569e3f412735b92860435ed81a37841df0f6b866e917930963d3e8939d2a20:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 87801edd52a520fc1032320203c85decb49a461b5a33acce4809bc279d34747d
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-protocol-finite-observation#source:f787ef56997ba17e0015b413ff1209d9da38d0836c3c90034d6e22619d87cdf1:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 4fc60e6c49a09cf805baafa13c13eb0ba0c748a6c2601becd12a9fec69e6a1c7
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-protocol-finite-observation#source:16dbbe30e1f13a607852e8df6f3fbfcbce25d9d6d62908b4808ad8a990ac54a9:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: f927fda6d5d796a6db69b11b26a94e3e97f7b7731db8917f9a2103cd9e6cf554
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-protocol-finite-observation#source:c4b71353f5c817dacb4ac74f889cd1593f4904adbce3d01dcc7d8bc637d3ae83:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: a8ab4bbc22acf5cf097211d8f492064d61f4e8712be40ab28a8061e12fc8578f
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-protocol-finite-observation#source:a7d0f47f9283c885d116fb44b4cc46a59eb8eb68fdfcf78648197c3b86a9d3a4:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: b3cb387909cf1c6809c32d44130d974fe9e16ec26fc91e485b9586bd88bbfd3d
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-protocol-finite-observation#source:a203145052e2db2c389afaaf8c02ce3eb8041b9239f42f0388a37d89d36042cc:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: bb36e8ea2420a8a58622d052399003074132da061dec3cebcb4547c2edb5f551
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-protocol-finite-observation#source:7fd1773ff07d35ac3eb664f44ec35cd228c88b3fc22d1b23ff5dd3a46906550c:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: e7a104598618669482f5664d8f65269bcd0b137b120c38dd9ad4b23e6d10f785
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-protocol-finite-observation#source:3e8eaabc720704de6dc28b51550369fdfaa8387a5d238a27a19d220d00f43ad2:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 3aa14c65a81a6cd762cd486aabf01cf4ab5e4aa2505a345d00ca3eda5c41f334
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-protocol-finite-observation#source:569ea658c267d505d0c20369dbdb384eb2022ea12e0bb6fabb70c398cc13ca0c:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 5f14bf142b5824e613244808415d9a20f5ab04faa81b24ae71df3ece21c0dc5d
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-protocol-finite-observation#source:3e91e2836c450b6eddb30115ec38e58f1859faaeb4ff61eccabaa685ab40dd8a:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: adda9a718d917d2c556c9952038dba2ede245874ebb249b5a35845f423d37fe2
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-protocol-finite-observation#source:0b6555b881edfa670d9fa6129b2c0a8205da65d6a21a5ea27221cf72db5c1143:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: bc21fce308a24ae19bacc595df688ed54453ff34255eb449dccb6da30e738d58
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-protocol-finite-observation#source:3a26d5b2c89773ada31776c5f23bc208b775c5201662d1ece1543c797f5562d1:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 7478bba719ab02ccfa9dffe235456cca89f245002958496ffd98b04dafdb0193
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-protocol-finite-observation#source:30ca4fb5f2a008160290fe3664d16bb70b2d168d57b693f23bab6f2a9628e38e:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: e0f0216b6c434f61b7371e59ae25dd98dcb335f22c65ca56503cf50119075662
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-protocol-finite-observation#source:fc843caf105c9e755f8976c4307533a560da98607965f63a3c5d5d21446924a9:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: d661d5111dbe51b0f64a1e850645a79e2ccec23ad9da783f87e0572d6bd3dee6
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  ? evidence.sdk-python-typed-component-stream-verification#source:594ab84a723a1b513ec1939b99a00a745c698396807110c32475ebab83571c3e
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: c1192c90d5a26671215cfdc0f9c75e9f166b8050c3486235144815227ca6e9d2
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  ? evidence.sdk-python-typed-component-stream-verification#source:d9d57f5234c5f6478af52ad529fd9f2ce0bfdfb354580be8de93080597304f72
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 10f4ce04a16726144f9b8d8d7796ac271612a2c53f44cd0dce2d10a050dcfa15
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  ? evidence.sdk-python-typed-component-stream-verification#source:d44e9ff012033e928933fa3afd158c8e38267ac29e35c72d7d2d5f1dd0c9d93e
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 0a9931d87a5913a841852c7c90d922407b4a8b3d6856ae601029a9f5b9191d0e
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  ? evidence.sdk-python-typed-component-stream-verification#source:7addd8df6f3e9a16829eb5d8f37e49332a633f3c2d6712a09a344e833b6524ff
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: cf5aa131ed8660d86c8998ef571f15e7534ca9ae26e47bc538f7dd6935bf75a2
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/llm-workflow-stream/typed-python-delta/receipt.json
      historical_git_mode: '100644'
      owner: evidence.sdk-python-typed-component-stream-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 6be390dbcde883205e182167b2b3466dd3a7f6fe4a08f005a831cc4e6900275b
      owner_fingerprint: 72964389d72390f98ec1ad884cca153aaf6c8063c99e058431f8a011a3744bf8
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 7addd8df6f3e9a16829eb5d8f37e49332a633f3c2d6712a09a344e833b6524ff
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_e3dda1373ad56856
  ? evidence.sdk-python-typed-component-stream-verification#source:34f55514fc758d3d46c2ba3e443b676131be52626ca53a8437b57a567e78d44a
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 5fdeaaca0c3106e7ccf41895fb2d7189e0992b3692ce288018e0d3b8fee2315a
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/llm-workflow-stream/typed-python-delta/before-source.json
      historical_git_mode: '100644'
      owner: evidence.sdk-python-typed-component-stream-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: a04c546e6261f3704d4d9b4c30e6bf1220c17128a410ddab34957a8a2fcd10a3
      owner_fingerprint: 72964389d72390f98ec1ad884cca153aaf6c8063c99e058431f8a011a3744bf8
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 34f55514fc758d3d46c2ba3e443b676131be52626ca53a8437b57a567e78d44a
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_30696c0a3712f136
  ? evidence.sdk-python-typed-component-stream-verification#source:ca2cc9d2a8bac5fb218379fb311213a8cd54544acb334899b18569439b6f90bc
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 707b0beccd1fcd9745593eb5aa0b3b1ad8842f98c2a10fe51a00fde839d7a177
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/llm-workflow-stream/typed-python-delta/after-source.json
      historical_git_mode: '100644'
      owner: evidence.sdk-python-typed-component-stream-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 45e6024dd4c7c682af4f147c643c0cd9e13f7bb5574c48bf954a8f50cd79ed11
      owner_fingerprint: 72964389d72390f98ec1ad884cca153aaf6c8063c99e058431f8a011a3744bf8
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: ca2cc9d2a8bac5fb218379fb311213a8cd54544acb334899b18569439b6f90bc
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_cd43b65d3bb02857
  ? evidence.sdk-python-typed-component-stream-verification#source:d9b337bfbb8e7b0c02cbb4ac0fef5f71081e9c94a7ecb75e6867797fba2b4485
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 3ce787f7b7a643b531d0479c4dc3506057606fc7e6c237228951054170f63d46
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/llm-workflow-stream/typed-python-delta/before-AgentComponentCall.py.txt
      historical_git_mode: '100644'
      owner: evidence.sdk-python-typed-component-stream-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 13d0a53f59764228a2ae9f7840071bc3a8e04a9485a4230605f27aae741f1b15
      owner_fingerprint: 72964389d72390f98ec1ad884cca153aaf6c8063c99e058431f8a011a3744bf8
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: d9b337bfbb8e7b0c02cbb4ac0fef5f71081e9c94a7ecb75e6867797fba2b4485
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_43958ca13a3c2bdb
  ? evidence.sdk-python-typed-component-stream-verification#source:c6e9e868daa869415155a4e80065c341bf953c0735302776f5447e23e172cd46
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: ea948676851993af355b40e981ee21b908e3d99ea329aaaea7578d1d94bb1f1a
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/llm-workflow-stream/typed-python-delta/after-AgentComponentCall.py.txt
      historical_git_mode: '100644'
      owner: evidence.sdk-python-typed-component-stream-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 23caad684bc0c7c3abff5eba63d2ce64e26aea0c15e3d7cc5a2e2560b7dd8d88
      owner_fingerprint: 72964389d72390f98ec1ad884cca153aaf6c8063c99e058431f8a011a3744bf8
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: c6e9e868daa869415155a4e80065c341bf953c0735302776f5447e23e172cd46
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_b63953abceebbf93
  ? evidence.sdk-python-typed-component-stream-verification#source:c72ba7205c22480deffe6d8cf843f8ab416bfcf492f98d31d216c98860084fff
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: ebcca088ab6ceb59386b2e91c8999068777db19229519268a401cec78457912d
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-successor-model-reviews/python-typed-component-stream.json
      historical_git_mode: '100644'
      owner: evidence.sdk-python-typed-component-stream-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: e3efe35c2019bff157a9d6d6f6e3480efb952f81c7289bef81d90d880bfa9007
      owner_fingerprint: 72964389d72390f98ec1ad884cca153aaf6c8063c99e058431f8a011a3744bf8
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: c72ba7205c22480deffe6d8cf843f8ab416bfcf492f98d31d216c98860084fff
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_00341ad0c5170e9f
  ? evidence.sdk-python-typed-component-stream-verification#source:9824cc09e892d3f70b2e55494160ee52e52ca24270b07336f9e4f790ea9397ce
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: faa04fa7298699283ce9bf97d8c27a9ee39c4e1fda3ff5c8eca1e25ada774795
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/final-receipt.json
      historical_git_mode: '100644'
      owner: evidence.sdk-python-typed-component-stream-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 5b7af86bf96eadd54f89a6d43d0d2b7e73735f1da7f6510c024e1480ceb1867c
      owner_fingerprint: 72964389d72390f98ec1ad884cca153aaf6c8063c99e058431f8a011a3744bf8
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 9824cc09e892d3f70b2e55494160ee52e52ca24270b07336f9e4f790ea9397ce
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_85733df2d77e9ad6
  evidence.sdk-rag-project-rules#source:14278d367b008659fd39caae78fd26eae7a9891ac7e040eb642573a65648dc98:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 99baeb2b34b6c807781b8d967ed155416083de1d23499bd78d897f469b73e014
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-rag-project-rules#source:cc8ff05fc4f5c5e3c34d25f71227d19481374cdc879e61a3c9df442305b4aaeb:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 508dd4550f584c68a8675636f9c9c2082934f184a74712a445799af1fb387abe
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-rag-project-rules#source:98e59b92039bda95393766cb85b7047f25d7974c1355e35ed2c0896d3b261c06:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 3e61a68df68d6e7b9e894a06eaca9a287f722277e43bbb69593af6e6c95c2c16
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-rag-project-rules#source:24e962f63628c9d4c48cc9b4a86380e0f69fdb4346ac3b8e42760e988c59ac5a:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: c8eb397ae0703210a3a7bd540f63ae4f82135191982b129067308350837a2f91
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-rag-project-rules#source:1b39e8949179645e3b1b6254291de7a153b03ed563c2accb67db47c4609db1cb:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 00e633fb8d9dcfea531d36261fe4d333d7d2d3f254964cff9a2666667003cbd0
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-successor-model-reviews/rag-project-rules.json
      historical_git_mode: '100644'
      owner: evidence.sdk-rag-project-rules
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 77a34d3c1e58e0d8e6d3a7350b4f4d45722588ec56765fc8d65d9db6ef550510
      owner_fingerprint: ca58b35ffcbdbbc63490e7e8c825a32a250c0d4b429e33318e3c62c47f4f9d53
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 1b39e8949179645e3b1b6254291de7a153b03ed563c2accb67db47c4609db1cb
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_c379c976c7f2160d
  evidence.sdk-rag-scalar-verification#source:ed24e4c64460af77d3e566f3c9a5a3ce8e19f5ecdb8c0bbbbb5e5649e503edb9:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 178638f4359a169b4e0aab62231bf7fea76ace351887fdaed290f505188bf3b3
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-rag-scalar-verification#source:ceb7bb57e428ad892fd7fdd28dd6ec77a487dc7112ef49700186aeb2d86c60b8:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 4f64bafdd87786fc136048e37fc5afdc1af7bf886b82e5729e90e4d68d3a6476
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-rag-scalar-verification#source:5d6af41ae1b45ad88de413eb5b8df7e0a5d700cbd158d89a6063186920529ab6:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: d59fccf943b872e10f4feb28a1e9e1370a0edcb7c2a0b0af8f5016e02e80875a
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-rag-scalar-verification#source:71e51c5f76a764ff3cbfe64db101f902605723c8f2a97df182948301fb805879:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: eb18ae535b4f6975214a9d88fb901ac2997166319af7294524a01429628e280d
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-rag-scalar-verification#source:a1704ebe956aa497da8a407d4a306653a5c15d42aac995ac3f4f388e7cee0682:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: e876d6817c6163f74820e9b91e8fa06c21810ed1a731abca2d0b696c018d50f4
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-rag-scalar-verification#source:d7dacc2133e935b982237807cbb13b7b3de7d11cbdb85d4662793f357bd77f6f:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 631c35d341390acda4a407979d9bd1911bb59fe29d9a98c9e4e7e4d174fb949c
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-rag-scalar-verification#source:594ab84a723a1b513ec1939b99a00a745c698396807110c32475ebab83571c3e:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: c1192c90d5a26671215cfdc0f9c75e9f166b8050c3486235144815227ca6e9d2
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-rag-scalar-verification#source:d9d57f5234c5f6478af52ad529fd9f2ce0bfdfb354580be8de93080597304f72:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 10f4ce04a16726144f9b8d8d7796ac271612a2c53f44cd0dce2d10a050dcfa15
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-rag-scalar-verification#source:d44e9ff012033e928933fa3afd158c8e38267ac29e35c72d7d2d5f1dd0c9d93e:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 0a9931d87a5913a841852c7c90d922407b4a8b3d6856ae601029a9f5b9191d0e
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-rag-scalar-verification#source:cbf90de4b8b415dd26a45b7d5ffd75e5f42fe433e2345e3f076a77c1ed0996f5:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: aa2ea26996075e2c338b8d2489973b73f17153242dbac80c48f7faa0acef31eb
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-rag-scalar-verification#source:7091a410607de11b1b1b997e919552f25a56e4ffd5a01f70ba82845f7d6fdbce:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 00b4d548539ea2b39c8049a193ec644dbbe2ddc9c20d7d75b6e28a26455c8bbd
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-successor-model-reviews/rag-scalar-b909.json
      historical_git_mode: '100644'
      owner: evidence.sdk-rag-scalar-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: b443cda8ba68697b70f0338dd1c2af2e0d3c03e9803a6b356211199df763a3eb
      owner_fingerprint: f3726189acdb55e5f4d9c27da8187ee3f4b2961493710f7b92986a390088af53
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 7091a410607de11b1b1b997e919552f25a56e4ffd5a01f70ba82845f7d6fdbce
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_b91ffb1364affb8d
  evidence.sdk-rag-scalar-verification#source:9824cc09e892d3f70b2e55494160ee52e52ca24270b07336f9e4f790ea9397ce:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: faa04fa7298699283ce9bf97d8c27a9ee39c4e1fda3ff5c8eca1e25ada774795
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/final-receipt.json
      historical_git_mode: '100644'
      owner: evidence.sdk-rag-scalar-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 461604a1e01f7590ff61af36b7c5edcc7aa546f07b08e40c1b11ea5d630b1774
      owner_fingerprint: f3726189acdb55e5f4d9c27da8187ee3f4b2961493710f7b92986a390088af53
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 9824cc09e892d3f70b2e55494160ee52e52ca24270b07336f9e4f790ea9397ce
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_2cae9b3a389f0f15
  evidence.sdk-rag-scalar-verification#source:5e0f153848725261abf228db5320a840d3b0afef6b72f1b8f076a02cdf6eded3:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: afa85d09a95bf2e8264d8e410d8bb423c21bd3f0b0c94e420e11c074642fbdfa
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/source-build-manifest.json
      historical_git_mode: '100644'
      owner: evidence.sdk-rag-scalar-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: b290203ac1625d202d1eda3c78bbe419c744799db410c2f475a16c9277af0258
      owner_fingerprint: f3726189acdb55e5f4d9c27da8187ee3f4b2961493710f7b92986a390088af53
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 5e0f153848725261abf228db5320a840d3b0afef6b72f1b8f076a02cdf6eded3
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_60da0495d498c911
  evidence.sdk-real-permission-default-ttl#source:1773d11ea9e760b0ebae6f510973e8ffe882cc9e1e85a0bf939c1b6957739445:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: f26504e3bc1f7f80259e0e10ff3028136c6b1e510a0cc34cf4b66b55dde46eec
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-real-permission-default-ttl#source:facdafc7b2b1e120bc0e80f324b0f1ad123912bd03cf9765f905337304beac3e:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: d54179c6280aa04c2336ae9913aa7641dda3fa4a098fc570fb323949f4f1b257
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-real-permission-default-ttl#source:33279f6b35a99e566350cbb7c647fbc3bbd8e32266a6cbdbb7ac0a0bf816ddda:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: ae55416e9691e9583ed7c9d2f6d71681ec410ba7f9fce94104802f2936f61473
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-real-permission-default-ttl#source:9b7ba2817272eb3e8b0e76bd15798e541afe313e3c2fc756a272884c82c10296:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 17575b4c04d636e98f2446f7dec3dc1af8a7ca89b9bf262658b8b1945c08ce64
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-real-permission-default-ttl#source:b3ee4798d0de430138fd38113787c8234acc1b39d0671ff6210caa846a2933a2:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 319ae97defb484dc68b857457d6385c6a8804c013d834b92c4ce08e55886eb32
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-real-permission-default-ttl#source:f5252364efcbf10be53e956dccdaca8e3546315edd5825d56000f8e85d31c5ee:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: d73bf733af995a1033952967be432887ec35559dce55f5a9caff6007ea612bca
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-real-permission-default-ttl#source:933aa44b5a61de85d4768e49da0002d165c7c9fbd6cc576701ee257e5ebfbfdd:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: b6cc005b406903f8885cce435838d13b2470f0e1712af0a46e75d4da37ab6e29
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-real-permission-default-ttl#source:8ceb700533532ebe92af6cfa4ccd23066a186df5d6758d31e49fab581908285b:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: d9d2106f1457bb6a39a377b3dcfaa975a77840987748ae164f759adbbe288bac
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-real-permission-default-ttl#source:51abe5fcc2c0959c365d291bd34770ded80e39e80ef1fdefc2aa160b7ada3532:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 1e941a373976b2b38b981daaab03274f810af6e42796ba1486d466207f13a7a4
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-real-permission-default-ttl#source:5e0f153848725261abf228db5320a840d3b0afef6b72f1b8f076a02cdf6eded3:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: afa85d09a95bf2e8264d8e410d8bb423c21bd3f0b0c94e420e11c074642fbdfa
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-real-permission-default-ttl#source:aa69f486583953fbefa81baeb838a82a9153dd9b85debdd75a0ec8a3533f3d95:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 98021415512bd107cb74adf06662c18455fca6513447025653af40c4a1101634
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-real-permission-default-ttl#source:fbcd60b86b9585f0d93d8e6b9dd384e67806a958b6f6994fd5e0e7e075c68b54:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 14b1aff50811f74c17f20cac49dad95f8f0100d2f49fe37a4a5fa89376d80876
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-real-permission-default-ttl#source:26246039900a075f39bd2e8e940d8a3e28f684e77f971d8d0f7bb2986a822840:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 90a6e93c9bb6d2b1b35d956bb35c1448f5d29cca53b78d5f6dccf2e9fc7d89ea
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-real-permission-default-ttl#source:33bb395c0c0a1ddf0fdb5d23cd8b01424826559908ed5b78b64ef1625216d158:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 9e12ef31b6b29d0bb086c1fc1841fa92ac532695515cb2782a781b2e6ed0ad78
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-real-permission-default-ttl#source:248a34bd933c9cc99fc5e9cef5df8cd7e21b055dc364ca4de86ca24db19d856f:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 78827b047aa54cc99304d88e15c65a693acbb5adf0b0fbef3d2bd950b2464689
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-real-permission-default-ttl#source:01ba4719c80b6fe911b091a7c05124b64eeece964e09c058ef8f9805daca546b:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 952179006d35c2748044edf7c71f90b9cd6f0b7830f59d445b7f905673e8e9ca
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-real-permission-default-ttl#source:9658c4814fb3b39e352d6cea74e50d33d350a24654a776cd22b2fce80b69eb9f:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 115071fd5505310ae7929d5160236bcd0d59fc28540df2bf4f0442f9a1f6d1f2
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-real-permission-default-ttl#source:5f121a7bb208dd51ad97e78255c9f0f29e1ac63f5a31219eb0b58494d9773478:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: c4d6240df51bbf27bf0b250718cf77cb0f2de9263317f50efb494db045378b95
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-real-permission-default-ttl#source:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 769cb44cf6bc59c752b70f83f2dd6e48f5fc9cbc108aa0c74a9002137bfb3250
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-real-permission-default-ttl#source:9d556c85c2000073bb6f66344a6f09823a0009a8c587cc3c8bd0ee29b7cc9916:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 4d51b1c85e2306c4fd3075848d9075052a84de8784dd324602147bc07c7d1f21
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-real-permission-default-ttl#source:8f4882db3876e9c37de4a0a4da5d68963acf52fe4c0f512de8a9c948e26f546d:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: faa3a3d13d9964494e1fed9730490b65122ab54949366cb22f52da3402c95e4e
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-real-permission-default-ttl#source:83e0aa53342968f8e95b613d9ea4484d25fcb9a0be0b2a1c426b247c7cd37ba6:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 0973d77ace942b05af16838cf40326fa994207158553bb7021b0cb7d1da747d2
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-real-permission-default-ttl#source:78699fc865591fa2edac85067ad726f2adf54562c212f7cd0dfd46a07d0cd1de:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: c22199338f6af21cfe767b6bb989a591c527ba7f872d0717f81f6f67361d8d59
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-real-permission-default-ttl#source:90c5aed3cb514e8eb6b2098e0e70a9fd953b0c6c2df6521e90dfee7fed64803d:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: e9ea15955b31fc049030756e1b79f5ae2402a3703530a567905938c33637f62d
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-real-permission-default-ttl#source:1d368671024232773a915fb73be08b9c849d7a5f61334362dd3f153154c6dc25:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 58288c105a0efd976195b74e301867d60af1f915ac6a6445030bdeb0258b045a
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-real-permission-default-ttl#source:1e030c1ed5eded3bab3c34eb58998dcf6a588c3aa29ba70e559d38098beb5cb4:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 1d6b5a362ba5ef1900188d5c9f9caad0ba69d21bb42f513e18fd7b653429d5f9
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-real-permission-default-ttl#source:6b86b273ff34fce19d6b804eff5a3f5747ada4eaa22f1d49c01e52ddb7875b4b:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: aaf51bdfdc77f046041be9007fadd7d358b3de89a9322ed0d7c0177f59fb6e6d
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-real-permission-default-ttl#source:5926799787a3ad3debd243558c3ec2c85556d094bc23d1d7b6aad511463dfd5d:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 3acb5f4ed982efa8ace9f6cc3985a50c1538268557e44fb35b6016b77bedf52a
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-real-permission-default-ttl#source:25d87c82db5dfcf658492ab329095097c88b062becc7c66cbab0b18bac25e558:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 2ba55e911e39d8b3ab075bf42f381a2b6dd2afef205548cdea301c7aeff32052
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-real-permission-default-ttl#source:569c184c85a427085a9e05003de82771f4dae938098b1097b3032b70471e18ce:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 2e8bd6fa4c2c4a8ddc21ecd48e91db21f913ab58975ca20b29dc8f89e7224b47
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-real-permission-default-ttl#source:2c22e7941f9dc7e6518b442363bcdd429a7692fd89b4b449f2a7293f200e09a7:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 201af38b143461ab4b2c06a1a7e5732ef1e5b2ba57ad54f5676bf3414e011a3d
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-real-permission-default-ttl#source:74da96985c6b7e9be9eaeaf0708af598bb104d798884435cdaa2ba81524fa67e:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 63602f7b604799dd9c4e1da2ae5459509581acc87b941501cf28fc3358a5b2f0
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-real-permission-default-ttl#source:30ca4fb5f2a008160290fe3664d16bb70b2d168d57b693f23bab6f2a9628e38e:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: e0f0216b6c434f61b7371e59ae25dd98dcb335f22c65ca56503cf50119075662
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-real-permission-default-ttl#source:50d132ec93ebc09110ed52e549335ff6c789bf198a1cfc94253023d75966c374:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 7bab30e4f37d1f48bf65ce78be06d71a2c2b511a6fcb8063167213b304e2b4ff
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-real-permission-default-ttl#source:c2f645237df25c3154947e2162c48efcb097de522a12271ae1bc2f7f82cdde83:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 06258308f03d45c295bf8629de7d76fe3972367e6e4bf5e3c4975776246382d1
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-real-permission-default-ttl#source:fa456244acef9e77038ca1a855900b1d7be2dcb8d4f2e931869b8d27d45367d8:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 5b941986a96ec0b2a8a56a766a55dc4f30751ea6b34d412e354577e6175ef860
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-real-permission-default-ttl#source:a3dad4e0d6d489af7d14c427d1e8b0b6e7e5a18a0e13b1db491ee6a13e771404:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 676e492cc0660e089f982daf23509232afdae9932487e62ed19523153c89a476
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-real-permission-default-ttl#source:f5566cc584a59e158fb47cccf86deaddb43d5a3079cdf80569847a08dc2afc66:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: a9654ef8922bbf5e28802b03aa2a09b228acbd6df8cecc6f1ded325910871369
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-real-permission-default-ttl#source:fb19aada11141dbbc139539ca3c18523771729fd2b10808ed01e871cc9764d5f:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 7ee7d4bc385c2d1675ee9ce3fb6ff4ca8ffddc7ca59f26b2d107ebb67689f1d5
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-real-permission-default-ttl#source:b78d63d156926a474b800cb0bf35a5627cb471ccf87a9789dcf6d5863b036fd4:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 026e118df2fc37ab001b7b0e8a3e94a13f9fac7920abfe58af9eef9b19246a98
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-real-permission-default-ttl#source:84d12f7862d3aa68b0a98c26489af3b659009b6edfeda11337a4f43479c1d2c9:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 877c14ead2b99c602505f270d458038f2eb7d8a0f12f0bc12625e4c6920d5a08
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-real-permission-default-ttl#source:c0775da031b2c1a451b4123ab8c6ceb40c14cb13931b7dd3d151875658d00d38:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 397763e09caa6afa427ad9f3e1a6130daaacf1c07b3fd1be34e72901d7df1653
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-real-permission-default-ttl#source:35ded3a05feaa44e64355389a0dbadd2dc0089a190e37d586d11e5399629a8dd:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 02ffe17e5ae4223da15c12c8aff655ad1ec97da224afd608d6c3de71fe287db4
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-rust-workspace-factory-gate-failure#source:1d04d497e278a3ac07934f37c198587480a31b8c25a938c53a12b823b46c6c68:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 486a82dc4016214ce0f051aa8d8283cf802e7ff06b0b10717bc2623962a747d1
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/README.md
      historical_git_mode: '100644'
      owner: evidence.sdk-rust-workspace-factory-gate-failure
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: b5308267c710ed89e74fbbbb2ce7476f6339a16e3fc362b6b56e91d7a3b2d322
      owner_fingerprint: d47093317d45719a134830239fdd6308f5fa7fe7c2d0be06bacc1393a88bf3e2
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 1d04d497e278a3ac07934f37c198587480a31b8c25a938c53a12b823b46c6c68
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_30e7f23d15cedd26
  evidence.sdk-rust-workspace-factory-gate-failure#source:8102d6e752f330b05608051224f54a47f3687967f079f1e48afd50bcbe24988d:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: bfbf92ba77057f52213724ccea008ee76f5192879ea6d4af3bd26884b9214b81
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/focused-source-manifest.json
      historical_git_mode: '100644'
      owner: evidence.sdk-rust-workspace-factory-gate-failure
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 43a0d03b9267425ca90e833da0d715db4d102f698e9153efcde68c6450d9efa3
      owner_fingerprint: d47093317d45719a134830239fdd6308f5fa7fe7c2d0be06bacc1393a88bf3e2
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 8102d6e752f330b05608051224f54a47f3687967f079f1e48afd50bcbe24988d
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_5a8146ac68d24efb
  evidence.sdk-rust-workspace-factory-gate-failure#source:4bb8082aa73bf785954586eeb4407dbb02edaff195429ca070fef5c9662942e9:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 4c99066780a6448143bcf9c818c088482cd00de846b1875db60851844944ca17
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/focused-active-claim.log
      historical_git_mode: '100644'
      owner: evidence.sdk-rust-workspace-factory-gate-failure
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 59e5c7cdaf3088ef7f7f9a2d53ea6b9c3764e5c0f5286100120aa507c395369b
      owner_fingerprint: d47093317d45719a134830239fdd6308f5fa7fe7c2d0be06bacc1393a88bf3e2
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 4bb8082aa73bf785954586eeb4407dbb02edaff195429ca070fef5c9662942e9
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_93ccafa3b7bdb433
  evidence.sdk-rust-workspace-factory-gate-failure#source:f80e187a26fedac5c8e2341f85c9970f869bf4e3c261c9485cc472826ce683fd:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: da9e4e4687db27c0f6314489d16a12c26202f1a7d1f676c451436d259c3789da
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/rust-gates/matrix.json
      historical_git_mode: '100644'
      owner: evidence.sdk-rust-workspace-factory-gate-failure
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 1dd1d90820f4f94d2bdb449cfd4e85018bd9f95a01639595f83dca52cc45c637
      owner_fingerprint: d47093317d45719a134830239fdd6308f5fa7fe7c2d0be06bacc1393a88bf3e2
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: f80e187a26fedac5c8e2341f85c9970f869bf4e3c261c9485cc472826ce683fd
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_7b2e475e40c524cc
  evidence.sdk-rust-workspace-factory-gate-failure#source:1a6f4311c544fb880dec833fcfc8aeec549aa57a4d772e108718212779cf8b40:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 4b83e4a17c5d82aeab7223252dac09b24e4fd995732e96ea693a02b238127b92
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/rust-gates/run.json
      historical_git_mode: '100644'
      owner: evidence.sdk-rust-workspace-factory-gate-failure
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 38c6e680f590ecbf696407879841ef914edab9523efd73c228a5d5261fcd94d8
      owner_fingerprint: d47093317d45719a134830239fdd6308f5fa7fe7c2d0be06bacc1393a88bf3e2
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 1a6f4311c544fb880dec833fcfc8aeec549aa57a4d772e108718212779cf8b40
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_9c808988a6e4ece4
  evidence.sdk-rust-workspace-factory-gate-failure#source:b94843cdb61c82508e98c82d9655dee5a2eca332d969d6d7b5c774a70b359da0:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: e53c8cbdb22d89a8745010c2dd44ba610e11230738fda515040345f305b10866
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/rust-gates/workspace-tests.log
      historical_git_mode: '100644'
      owner: evidence.sdk-rust-workspace-factory-gate-failure
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 5808ffff6d9ca0ef32f546b33bc810ba4ad4de0c52954b65f1857528354357cd
      owner_fingerprint: d47093317d45719a134830239fdd6308f5fa7fe7c2d0be06bacc1393a88bf3e2
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: b94843cdb61c82508e98c82d9655dee5a2eca332d969d6d7b5c774a70b359da0
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_13bda9c353dbfe8b
  evidence.sdk-sandbox-natural-eof-verification#source:95c3b306c857efd65bd315029dab7a5f346f0c7926201778ed70043042d26628:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: a965b03045e3ff9dadd0a414a6e0925824467f21e1f536b2cbcd790bd9bd5465
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-sandbox-natural-eof-verification#source:594ab84a723a1b513ec1939b99a00a745c698396807110c32475ebab83571c3e:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: c1192c90d5a26671215cfdc0f9c75e9f166b8050c3486235144815227ca6e9d2
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-sandbox-natural-eof-verification#source:d9d57f5234c5f6478af52ad529fd9f2ce0bfdfb354580be8de93080597304f72:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 10f4ce04a16726144f9b8d8d7796ac271612a2c53f44cd0dce2d10a050dcfa15
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-sandbox-natural-eof-verification#source:d44e9ff012033e928933fa3afd158c8e38267ac29e35c72d7d2d5f1dd0c9d93e:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 0a9931d87a5913a841852c7c90d922407b4a8b3d6856ae601029a9f5b9191d0e
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-sandbox-natural-eof-verification#source:cbf90de4b8b415dd26a45b7d5ffd75e5f42fe433e2345e3f076a77c1ed0996f5:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: aa2ea26996075e2c338b8d2489973b73f17153242dbac80c48f7faa0acef31eb
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-sandbox-natural-eof-verification#source:892bd02dde6a42286d792c4683a64fef67f257689a611dd27b0a3ecf9c7f51d0:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: a89d3cfdb4b40449dfa614cb947a13928e06535a7d7541879402a7275982ca87
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sandbox-natural-eof/manifest.json
      historical_git_mode: '100644'
      owner: evidence.sdk-sandbox-natural-eof-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 63af174cd697e8e86f8fba6313cd71ea2aeeac70818fa72e600dca74e4237dd6
      owner_fingerprint: 988d0696d0a1faab8152711e70f51250c55442971d974145d8dd1f67743da586
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 892bd02dde6a42286d792c4683a64fef67f257689a611dd27b0a3ecf9c7f51d0
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_c2d1c1a69d665230
  evidence.sdk-sandbox-natural-eof-verification#source:b26afbdf9163de470268d5bc13179fba348e198a87e7c656a4880e08d5498cb1:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: b3689a727cf3e158dacf58bc3ce0ae90b2bd7fce636a1bd6b2bddf866918646b
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sandbox-natural-eof/sandbox-natural-eof-before.log
      historical_git_mode: '100644'
      owner: evidence.sdk-sandbox-natural-eof-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 91e6dbb95f922f280192ef7ee83d5696af12d7b38edf8e2a6d55e222fa3ff61e
      owner_fingerprint: 988d0696d0a1faab8152711e70f51250c55442971d974145d8dd1f67743da586
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: b26afbdf9163de470268d5bc13179fba348e198a87e7c656a4880e08d5498cb1
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_daf36826cce1b793
  evidence.sdk-sandbox-natural-eof-verification#source:a93f8696fb78a89041a02b4cbba612d1cffaa3373ba9816e2c5ccf77eaa27783:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 9048f8e0bcc00d8538b9d6e58eed1d24409518d589e204febd7c6e4dd982ce89
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sandbox-natural-eof/sandbox-natural-eof-after.log
      historical_git_mode: '100644'
      owner: evidence.sdk-sandbox-natural-eof-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 7efeb00624d67ce50a0b6a2943ea1744bc71f363f2f0dfa741b57bc62d9ecfc0
      owner_fingerprint: 988d0696d0a1faab8152711e70f51250c55442971d974145d8dd1f67743da586
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: a93f8696fb78a89041a02b4cbba612d1cffaa3373ba9816e2c5ccf77eaa27783
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_501f771b63931550
  evidence.sdk-sandbox-natural-eof-verification#source:7f1ccb061ac1f2b937f0f12a94a51da13757cc31c96f1ee3dd86c5db0456184c:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 09c60065d2c5533cf5149cd1fda997ebd6af7238cb615c22c4cd25814d313c14
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sandbox-natural-eof/mod-after.rs
      historical_git_mode: '100644'
      owner: evidence.sdk-sandbox-natural-eof-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: a6c3c315359468e4330ea814b5114f0ff3effb1e846453122dd5140e52b0435a
      owner_fingerprint: 988d0696d0a1faab8152711e70f51250c55442971d974145d8dd1f67743da586
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 7f1ccb061ac1f2b937f0f12a94a51da13757cc31c96f1ee3dd86c5db0456184c
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_9debc749484993f1
  evidence.sdk-sandbox-natural-eof-verification#source:2b631efaa331eb85281a54edcebbcc32170a877ae4829525ba62072b067c5e3c:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 38d4e595c7380a8147bf4246e6ed7a8b430b6e491381c241c05699338e1e33ef
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/tool-value-effects/b909-natural-eof-after/manifest.json
      historical_git_mode: '100644'
      owner: evidence.sdk-sandbox-natural-eof-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 257396e05ea074f2ac9369c453e4d7fb64a7e57ec8f21cdcffeb36ded482e785
      owner_fingerprint: 988d0696d0a1faab8152711e70f51250c55442971d974145d8dd1f67743da586
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 2b631efaa331eb85281a54edcebbcc32170a877ae4829525ba62072b067c5e3c
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_cea91f62674a4731
  evidence.sdk-sandbox-natural-eof-verification#source:2415f6c17c7ba7de8efd76a56d5443b34a346fd5503832e45647d83fca278648:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: e5e051fe5549ad964bbd9d12b0fa95e7e53c52f806fe53e03882baa24d420ad0
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/tool-value-effects/b909-natural-eof-after/SHA256SUMS.json
      historical_git_mode: '100644'
      owner: evidence.sdk-sandbox-natural-eof-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: a72bd1f442e3ccd666ebcfec2b87eca114a4d3de160b2e358dd4981320863318
      owner_fingerprint: 988d0696d0a1faab8152711e70f51250c55442971d974145d8dd1f67743da586
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 2415f6c17c7ba7de8efd76a56d5443b34a346fd5503832e45647d83fca278648
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_6599f77ac3aa47e5
  evidence.sdk-sandbox-natural-eof-verification#source:31278280cf0a931146933a395b64eb84690f589e1aaea003521a8e011f6b4eb7:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 1b49cb5cfd10dbc900a5d3bc1883ccfe7fb89b51675bc27ddb92384d151b6450
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-successor-model-reviews/sandbox-natural-eof-final.json
      historical_git_mode: '100644'
      owner: evidence.sdk-sandbox-natural-eof-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 4d7083cb2b48c7ad1a9734cf9e0ed7e16f5d3a14f3560c728a7141e9f1a5f6db
      owner_fingerprint: 988d0696d0a1faab8152711e70f51250c55442971d974145d8dd1f67743da586
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 31278280cf0a931146933a395b64eb84690f589e1aaea003521a8e011f6b4eb7
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_ed11ca15a433541c
  evidence.sdk-sandbox-natural-eof-verification#source:9824cc09e892d3f70b2e55494160ee52e52ca24270b07336f9e4f790ea9397ce:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: faa04fa7298699283ce9bf97d8c27a9ee39c4e1fda3ff5c8eca1e25ada774795
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/final-receipt.json
      historical_git_mode: '100644'
      owner: evidence.sdk-sandbox-natural-eof-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 8a611ee9c29bc43286785e32f07af7aa784cba07bcf9cab1e9a5110dfb288628
      owner_fingerprint: 988d0696d0a1faab8152711e70f51250c55442971d974145d8dd1f67743da586
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 9824cc09e892d3f70b2e55494160ee52e52ca24270b07336f9e4f790ea9397ce
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_94ca9968c3c95670
  evidence.sdk-sandbox-natural-eof-verification#source:5e0f153848725261abf228db5320a840d3b0afef6b72f1b8f076a02cdf6eded3:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: afa85d09a95bf2e8264d8e410d8bb423c21bd3f0b0c94e420e11c074642fbdfa
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/source-build-manifest.json
      historical_git_mode: '100644'
      owner: evidence.sdk-sandbox-natural-eof-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: c3061bab093613962f988acb0e64c1ca2f1651e9001d5a0908aff379c0db3c61
      owner_fingerprint: 988d0696d0a1faab8152711e70f51250c55442971d974145d8dd1f67743da586
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 5e0f153848725261abf228db5320a840d3b0afef6b72f1b8f076a02cdf6eded3
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_8d0859b7f7ddbbdc
  evidence.sdk-search-sandbox-06bb#source:b7ba118f46ea2a709da0d35b69e7ecc4d5ec3abbf84f79bc47b4c3ec30b002f7:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 869ad9a9fce118dc2aba7ec0322d1e4c2d3cadccdeb6492318b23fda0f185c65
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-search-sandbox-06bb#source:f088b14b8967d4d69eda8319811cff84f677af784820ed47ba5e76fb00d08ce5:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 2c91b152a0f0791bbe5c80f84badf69bca6d5affefc424f667d702bbd746bf61
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-search-sandbox-06bb#source:d050e7543c794d31325bea3c51a767df2ab73c58c5eb67227e2a0e216dd7e39f:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 3f404973034e6aff5ea4ba57ce04f8dd6554aa4cef4a806d51e6533caea740e6
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-search-sandbox-06bb#source:cfd150095820cd84a4bfd2053b27cc06093c19b70e2db1050c5a37bf9234d914:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: ec296eddc9ff6b4b75c3239f7f9939e8bd8de7eb5d3ba425eaeeb05ac7749e91
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-search-sandbox-06bb#source:594ab84a723a1b513ec1939b99a00a745c698396807110c32475ebab83571c3e:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: c1192c90d5a26671215cfdc0f9c75e9f166b8050c3486235144815227ca6e9d2
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-search-sandbox-06bb#source:d9d57f5234c5f6478af52ad529fd9f2ce0bfdfb354580be8de93080597304f72:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 10f4ce04a16726144f9b8d8d7796ac271612a2c53f44cd0dce2d10a050dcfa15
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-search-sandbox-06bb#source:d44e9ff012033e928933fa3afd158c8e38267ac29e35c72d7d2d5f1dd0c9d93e:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 0a9931d87a5913a841852c7c90d922407b4a8b3d6856ae601029a9f5b9191d0e
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-search-sandbox-06bb#source:0a6465e01c3867066f2cea3107cbf20ff408610cec6bcd743d15c8c5070d8902:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: c2aa1164fccc6e63c5324c0d51b9117afd9b09492931dfe5b7ce9e4e14aa0ef9
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-successor-model-reviews/search-sandbox-06bb.json
      historical_git_mode: '100644'
      owner: evidence.sdk-search-sandbox-06bb
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 1ceaa0c6ed2f395d71feeebbe7937a6a6bb8f95bfedd2afb4b7dfc5b977ba4cd
      owner_fingerprint: afe0e039e619d0b106d70f503fb0d4d36d3229d55d17e58a2996d4812dd23882
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 0a6465e01c3867066f2cea3107cbf20ff408610cec6bcd743d15c8c5070d8902
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_6b5852050dd03793
  evidence.sdk-search-sandbox-06bb#source:9824cc09e892d3f70b2e55494160ee52e52ca24270b07336f9e4f790ea9397ce:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: faa04fa7298699283ce9bf97d8c27a9ee39c4e1fda3ff5c8eca1e25ada774795
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/final-receipt.json
      historical_git_mode: '100644'
      owner: evidence.sdk-search-sandbox-06bb
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: a17a354d27dfe06f0fb0d46cf5d26e66fdcd4f40f9c5a9a4d0362ba39ce3757f
      owner_fingerprint: afe0e039e619d0b106d70f503fb0d4d36d3229d55d17e58a2996d4812dd23882
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 9824cc09e892d3f70b2e55494160ee52e52ca24270b07336f9e4f790ea9397ce
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_d356c04d0f2b4863
  evidence.sdk-source-continuity#source:45ea97df7083eb5404f70cd94bb1426acef9740236d82b109d4c7d1d1cbd77d7:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 9cb18bf8101bb85c095f8520ab30d8693f4fc92f4232008cf3dc02ce3b45031e
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-source-import#source:45ea97df7083eb5404f70cd94bb1426acef9740236d82b109d4c7d1d1cbd77d7:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 9cb18bf8101bb85c095f8520ab30d8693f4fc92f4232008cf3dc02ce3b45031e
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-state-trace-delivery-candidate#source:89425a805d469a699f13a8db50bc111866d78f10c2ba02d6c0a896c76f2a280d:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: df9dac063b7a6e14da33e24fb5beb3230d60acc40b0656ad3e576eb196336c11
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-state-trace-delivery-candidate#source:476800d1d11e2bf6e5943f1e2fc8758dbb24a577753b8433c883fee998314095:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: b6db38ba32313296ebec777fb310e070d8aa8af4cc358ee55ea9f873b19c2c5c
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-state-trace-delivery-candidate#source:8a1c78db297e7e2e81b9a6d7aad063d2ab0aeb7419fb5df0520c2dcb8716c59c:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: e657e6766fed2bcafeb80658b5c38749d4c0ca5cd1f7aaa9661f97e43c2d0d6e
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-state-trace-delivery-candidate#source:1f26d96802f5f1acf677abba02196126226074c141b1820fb2cb4935beec3f25:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 5516e15fa6a728421624eb7a296a9e77e3134ddafe9c3f78bbc8b1c6fe4bfe01
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-state-trace-delivery-candidate#source:5183d66485219c95bd5019e2a92098922cdb6b050b867204499b12d3a253f01a:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 83c866bd0c86be24df0f2c8c491a6aae9040d8f7b2beef08a093f702cb94f620
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-state-trace-delivery-candidate#source:5560683667a48f238c337a372d7ba15a1e70bc62b539df92d48f742b5dd3bc70:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: f7beba62afa3f4933111451ecd8506a7fe84d732a30e5ab1f4384dbc1d7c2b0f
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-state-trace-delivery-candidate#source:796683b4986c97e4c6c4be4ab482aa8be1d5cd61a309ef1def4900b81df05036:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: e80f7d2656deb57d979583b773bb9b221933c9070ce2b896e67f0a55110a6af6
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-state-trace-delivery-candidate#source:cc2821f2b803443c9e49f98287bff2497afd86cb92d52d0413f71051e58b98cd:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 9d2e0ce07abd9f980332bb9466d6eae73a1682bad407fbad6d42e03aa4053a46
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-state-trace-delivery-candidate#source:c3ed9b3a468a941dd1b8d13c6993822b44cc892d2c88ef10516af91d5a2212c8:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: c5c05d76c11d080fd9a6114a6e80ca105b9100211e8093b7940a934cf552fc92
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-state-trace-delivery-candidate#source:594ab84a723a1b513ec1939b99a00a745c698396807110c32475ebab83571c3e:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: c1192c90d5a26671215cfdc0f9c75e9f166b8050c3486235144815227ca6e9d2
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-state-trace-delivery-candidate#source:d9d57f5234c5f6478af52ad529fd9f2ce0bfdfb354580be8de93080597304f72:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 10f4ce04a16726144f9b8d8d7796ac271612a2c53f44cd0dce2d10a050dcfa15
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-state-trace-delivery-candidate#source:d44e9ff012033e928933fa3afd158c8e38267ac29e35c72d7d2d5f1dd0c9d93e:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 0a9931d87a5913a841852c7c90d922407b4a8b3d6856ae601029a9f5b9191d0e
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-state-trace-delivery-candidate#source:0c777ad1422e96d8d88c7316bf0a1fc22ae85aa9ea8a9a74012c08133461d084:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 2a55b55d8e4fa7ebeff7bab27de1024c63738538ceb075243e923c0156c44807
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-state-trace-delivery-20261013/current-test-source-comparison.json
      historical_git_mode: '100644'
      owner: evidence.sdk-state-trace-delivery-candidate
      source_role: documentation
      observed_revision: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
      issue_id: 7e1f7b6fced7327c5981e3af2829b198ecfe5768c501180b8bc8d0cc499a809f
      owner_fingerprint: 50fe94a21cb83f2f18560298f4c5904e8a915c574aaa86f4151ba3b61c95eafe
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 0c777ad1422e96d8d88c7316bf0a1fc22ae85aa9ea8a9a74012c08133461d084
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_7c18318b337511c1
  evidence.sdk-state-trace-delivery-candidate#source:2b4cae8147e358bcb6a7988dc7962521c7666843bbd0a31850959fa996911771:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 4e89516bbc4d852469c0142a0e20416bcbf7b3c48f69d406c42de57542a9f99a
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-state-trace-delivery-20261013/formal-adoption-receipt.json
      historical_git_mode: '100644'
      owner: evidence.sdk-state-trace-delivery-candidate
      source_role: documentation
      observed_revision: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
      issue_id: 05b67bdedf2be81da2b7ce253d8767627e60f869dff0c4735bfb4fea344d797d
      owner_fingerprint: 50fe94a21cb83f2f18560298f4c5904e8a915c574aaa86f4151ba3b61c95eafe
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 2b4cae8147e358bcb6a7988dc7962521c7666843bbd0a31850959fa996911771
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_7be4799f0ac19faf
  evidence.sdk-state-trace-delivery-candidate#source:9824cc09e892d3f70b2e55494160ee52e52ca24270b07336f9e4f790ea9397ce:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: faa04fa7298699283ce9bf97d8c27a9ee39c4e1fda3ff5c8eca1e25ada774795
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/final-receipt.json
      historical_git_mode: '100644'
      owner: evidence.sdk-state-trace-delivery-candidate
      source_role: documentation
      observed_revision: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
      issue_id: 0e90248444238f9d6691d473f7b9f7c8b6edee42658e2bb91483e8cca5b4356c
      owner_fingerprint: 50fe94a21cb83f2f18560298f4c5904e8a915c574aaa86f4151ba3b61c95eafe
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 9824cc09e892d3f70b2e55494160ee52e52ca24270b07336f9e4f790ea9397ce
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_b521393fc284c1d2
  evidence.sdk-state-trace-delivery-candidate#source:5e0f153848725261abf228db5320a840d3b0afef6b72f1b8f076a02cdf6eded3:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: afa85d09a95bf2e8264d8e410d8bb423c21bd3f0b0c94e420e11c074642fbdfa
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/source-build-manifest.json
      historical_git_mode: '100644'
      owner: evidence.sdk-state-trace-delivery-candidate
      source_role: documentation
      observed_revision: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
      issue_id: 10bb0401b0598093be47412674ca3efb300d409b356493d0e2d196df02d3aa73
      owner_fingerprint: 50fe94a21cb83f2f18560298f4c5904e8a915c574aaa86f4151ba3b61c95eafe
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 5e0f153848725261abf228db5320a840d3b0afef6b72f1b8f076a02cdf6eded3
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_fc829446d1c564e7
  evidence.sdk-stream-cancellation-routing-repair#source:1a1039514602a442a761482e0db0a8643b0fd1113f7c40d6f686c3d2c56c696b:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: eb2c1046973ea8b90e84206c087043cb3394647360e3e1eedf65213d38b4e919
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-stream-cancellation-routing-repair#source:43c6e23118958dffe4f40d286df09ffb4b77f17fcd554826e71563af69f86329:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 19175c3675fd86f6ef53aa6c3284c860348468f0cc459a92d726ee70efdfad12
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-stream-cancellation-routing-repair#source:a6e1e56ea4ebbe8ff76ee41964102b2ccd60541331ca9564cbfbb94e50ba0cc8:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: e4ed5d84f43a02418277a83a8fb84b347609d0160934cc3bdbfe39005932201a
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-stream-cancellation-routing-repair#source:df97421700e8fe379238cf0a595c9d1cb510774aa418222d40a27eee0373b218:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 3430907f833f1840b35773701b6cfb5825d768c0121eff1e4958625aeaa28dfc
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-stream-cancellation-routing-repair#source:9a1fe6ce00aaf625c296a607c23dc0b650cfe01c3a5182d012124a41b2f48951:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 63489a3d1e7d20e60c66bf87f1bb8253e4fa6e21fbc7fa88e30d3176bdef9f61
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-stream-cancellation-routing-repair#source:594ab84a723a1b513ec1939b99a00a745c698396807110c32475ebab83571c3e:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: c1192c90d5a26671215cfdc0f9c75e9f166b8050c3486235144815227ca6e9d2
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-stream-cancellation-routing-repair#source:d9d57f5234c5f6478af52ad529fd9f2ce0bfdfb354580be8de93080597304f72:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 10f4ce04a16726144f9b8d8d7796ac271612a2c53f44cd0dce2d10a050dcfa15
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-stream-cancellation-routing-repair#source:d44e9ff012033e928933fa3afd158c8e38267ac29e35c72d7d2d5f1dd0c9d93e:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 0a9931d87a5913a841852c7c90d922407b4a8b3d6856ae601029a9f5b9191d0e
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-stream-cancellation-routing-repair#source:36641db314d83d5cde3fac4099c3e514b97f317f6c6de000c6765f4ab17422e6:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 834ff86aedc9db898beb355afa4bb7904489db53e87bbf9f403f103b9725e85d
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-successor-model-reviews/stream-cancellation-routing.json
      historical_git_mode: '100644'
      owner: evidence.sdk-stream-cancellation-routing-repair
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 4e05ed04a7cb3cb5e81f628d84d115a98033813e0518d2224ae956a9d587a298
      owner_fingerprint: 7981df866366c6de8e8eb2c5a2ab304482e2222d5ed4315396348c5ca44e5746
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 36641db314d83d5cde3fac4099c3e514b97f317f6c6de000c6765f4ab17422e6
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_1d510fc10023b0d1
  evidence.sdk-stream-cancellation-routing-repair#source:9824cc09e892d3f70b2e55494160ee52e52ca24270b07336f9e4f790ea9397ce:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: faa04fa7298699283ce9bf97d8c27a9ee39c4e1fda3ff5c8eca1e25ada774795
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/final-receipt.json
      historical_git_mode: '100644'
      owner: evidence.sdk-stream-cancellation-routing-repair
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: e07e9123b697091d5142092c399b602894c6fa625ed63fb97326fadffeda72d4
      owner_fingerprint: 7981df866366c6de8e8eb2c5a2ab304482e2222d5ed4315396348c5ca44e5746
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 9824cc09e892d3f70b2e55494160ee52e52ca24270b07336f9e4f790ea9397ce
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_cf39dfcbbb220885
  ? evidence.sdk-stream-cancellation-routing-verification#source:1a1039514602a442a761482e0db0a8643b0fd1113f7c40d6f686c3d2c56c696b
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: eb2c1046973ea8b90e84206c087043cb3394647360e3e1eedf65213d38b4e919
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  ? evidence.sdk-stream-cancellation-routing-verification#source:43c6e23118958dffe4f40d286df09ffb4b77f17fcd554826e71563af69f86329
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 19175c3675fd86f6ef53aa6c3284c860348468f0cc459a92d726ee70efdfad12
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  ? evidence.sdk-stream-cancellation-routing-verification#source:a6e1e56ea4ebbe8ff76ee41964102b2ccd60541331ca9564cbfbb94e50ba0cc8
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: e4ed5d84f43a02418277a83a8fb84b347609d0160934cc3bdbfe39005932201a
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  ? evidence.sdk-stream-cancellation-routing-verification#source:df97421700e8fe379238cf0a595c9d1cb510774aa418222d40a27eee0373b218
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 3430907f833f1840b35773701b6cfb5825d768c0121eff1e4958625aeaa28dfc
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  ? evidence.sdk-stream-cancellation-routing-verification#source:9a1fe6ce00aaf625c296a607c23dc0b650cfe01c3a5182d012124a41b2f48951
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 63489a3d1e7d20e60c66bf87f1bb8253e4fa6e21fbc7fa88e30d3176bdef9f61
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  ? evidence.sdk-stream-cancellation-routing-verification#source:594ab84a723a1b513ec1939b99a00a745c698396807110c32475ebab83571c3e
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: c1192c90d5a26671215cfdc0f9c75e9f166b8050c3486235144815227ca6e9d2
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  ? evidence.sdk-stream-cancellation-routing-verification#source:d9d57f5234c5f6478af52ad529fd9f2ce0bfdfb354580be8de93080597304f72
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 10f4ce04a16726144f9b8d8d7796ac271612a2c53f44cd0dce2d10a050dcfa15
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  ? evidence.sdk-stream-cancellation-routing-verification#source:d44e9ff012033e928933fa3afd158c8e38267ac29e35c72d7d2d5f1dd0c9d93e
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 0a9931d87a5913a841852c7c90d922407b4a8b3d6856ae601029a9f5b9191d0e
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  ? evidence.sdk-stream-cancellation-routing-verification#source:36641db314d83d5cde3fac4099c3e514b97f317f6c6de000c6765f4ab17422e6
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 834ff86aedc9db898beb355afa4bb7904489db53e87bbf9f403f103b9725e85d
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-successor-model-reviews/stream-cancellation-routing.json
      historical_git_mode: '100644'
      owner: evidence.sdk-stream-cancellation-routing-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: adcac6df81ea037756ebf1eaa5d70b320dd8335f0c6ebfaea8b4b2fa3a846e11
      owner_fingerprint: 04b1b27b1eedfe814fb0e5a723aeac2e275ce9212ddf9466653938f89a79b5e2
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 36641db314d83d5cde3fac4099c3e514b97f317f6c6de000c6765f4ab17422e6
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_a02bc3ab780ba312
  ? evidence.sdk-stream-cancellation-routing-verification#source:9824cc09e892d3f70b2e55494160ee52e52ca24270b07336f9e4f790ea9397ce
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: faa04fa7298699283ce9bf97d8c27a9ee39c4e1fda3ff5c8eca1e25ada774795
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/final-receipt.json
      historical_git_mode: '100644'
      owner: evidence.sdk-stream-cancellation-routing-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 142eded18bb4eff176e24c04ebc59d6b3bbf006ee254aa706defbb4747140ecd
      owner_fingerprint: 04b1b27b1eedfe814fb0e5a723aeac2e275ce9212ddf9466653938f89a79b5e2
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 9824cc09e892d3f70b2e55494160ee52e52ca24270b07336f9e4f790ea9397ce
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_4d0cfd80f935e617
  evidence.sdk-stream-producer-verification#source:594ab84a723a1b513ec1939b99a00a745c698396807110c32475ebab83571c3e:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: c1192c90d5a26671215cfdc0f9c75e9f166b8050c3486235144815227ca6e9d2
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-stream-producer-verification#source:d9d57f5234c5f6478af52ad529fd9f2ce0bfdfb354580be8de93080597304f72:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 10f4ce04a16726144f9b8d8d7796ac271612a2c53f44cd0dce2d10a050dcfa15
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-stream-producer-verification#source:d44e9ff012033e928933fa3afd158c8e38267ac29e35c72d7d2d5f1dd0c9d93e:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 0a9931d87a5913a841852c7c90d922407b4a8b3d6856ae601029a9f5b9191d0e
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-stream-producer-verification#source:728addb408f60de808ad225e379413ec0979a2f037158ace81c1d70fd8e61779:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: c50a65718e9616af2dd07f4e34f4b4be9f899224f0c59c86cfc5d43ba9a8d202
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-stream-local-closure/sdk-stream-producer-proof.json
      historical_git_mode: '100644'
      owner: evidence.sdk-stream-producer-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 7bf0c7f6cb2965c886322034045fd60218ece3ced9b66da26a58588d7b06fe4a
      owner_fingerprint: ee7e4dabe0d35e2aa60593ded9b937602a5a9b42475b6d830ad277099fcaff65
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 728addb408f60de808ad225e379413ec0979a2f037158ace81c1d70fd8e61779
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_c8c8e2dbba5b5026
  evidence.sdk-stream-producer-verification#source:e73c2739bd3ded6764f1ab5b4f605652eafe663b480f393b21f888a1be3389ca:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 759a838e73f0488ae9cb84fe3865fb1e38e9a4faf224d5f8020c79810d9211cb
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-stream-local-closure/sdk-stream-next-safety-proof.json
      historical_git_mode: '100644'
      owner: evidence.sdk-stream-producer-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: f38449e94ca4a84cde965c847c280f191e81c03813d2eb4c5c10fd1f8c8c097c
      owner_fingerprint: ee7e4dabe0d35e2aa60593ded9b937602a5a9b42475b6d830ad277099fcaff65
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: e73c2739bd3ded6764f1ab5b4f605652eafe663b480f393b21f888a1be3389ca
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_e6e8602ff888c7ae
  evidence.sdk-stream-producer-verification#source:3b9ec485ca7c7fd4ea05e348bb910079f7a7d7e06311d0d000c9524eb59ea1ea:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: c1d4e99e815df800a3d79e7734f31878684f7ab158b4efda1bd5b567a0f2d4c6
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-stream-local-closure/sdk-stream-atomic-cancel-proof.json
      historical_git_mode: '100644'
      owner: evidence.sdk-stream-producer-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: f3de873b151856f41a2a96ddb9903ce534c0b1c1d0cb82aec595621f3ff98d44
      owner_fingerprint: ee7e4dabe0d35e2aa60593ded9b937602a5a9b42475b6d830ad277099fcaff65
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 3b9ec485ca7c7fd4ea05e348bb910079f7a7d7e06311d0d000c9524eb59ea1ea
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_e4780d62b7007d2e
  evidence.sdk-stream-producer-verification#source:45e8896e48c9a53a8e9250cb5dbb0bc1319a5f5d9a6edf54a427f4a3ebb7ade8:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 4202ee9e3fb184e3c709466af10d85534914487fc647d2b575e448de9c7f337f
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-stream-local-closure/sdk-convenience-retry-evidence.json
      historical_git_mode: '100644'
      owner: evidence.sdk-stream-producer-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 8ed75bb7c7232fc6f74be8588294dfc5495f4ab23bd995931b369db7bdeebff2
      owner_fingerprint: ee7e4dabe0d35e2aa60593ded9b937602a5a9b42475b6d830ad277099fcaff65
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 45e8896e48c9a53a8e9250cb5dbb0bc1319a5f5d9a6edf54a427f4a3ebb7ade8
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_033be81373aeee49
  evidence.sdk-stream-producer-verification#source:39772a593e8ea07681666744490b47b1ecd415ab64d3cfcb8f87031c3096e7d2:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 6d7c9ec6dbb2f98b5916ad529e63f7fc5abca6400c5239d0ff341f99e4da66d3
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-stream-local-closure/independent-review.json
      historical_git_mode: '100644'
      owner: evidence.sdk-stream-producer-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: ecd2a7f655840c9313205f21ec813123d06191bc2f8b61740b4c587b305f5500
      owner_fingerprint: ee7e4dabe0d35e2aa60593ded9b937602a5a9b42475b6d830ad277099fcaff65
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 39772a593e8ea07681666744490b47b1ecd415ab64d3cfcb8f87031c3096e7d2
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_027988dc2ce95386
  evidence.sdk-stream-producer-verification#source:9824cc09e892d3f70b2e55494160ee52e52ca24270b07336f9e4f790ea9397ce:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: faa04fa7298699283ce9bf97d8c27a9ee39c4e1fda3ff5c8eca1e25ada774795
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/final-receipt.json
      historical_git_mode: '100644'
      owner: evidence.sdk-stream-producer-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 2a1cec618418d4e9e8fb82973605a41743a77994c3c7ab69fa509c1c091c22a0
      owner_fingerprint: ee7e4dabe0d35e2aa60593ded9b937602a5a9b42475b6d830ad277099fcaff65
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 9824cc09e892d3f70b2e55494160ee52e52ca24270b07336f9e4f790ea9397ce
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_319a0eeea022489c
  ? evidence.sdk-task-context-direct-getter-consumption#source:1d04d497e278a3ac07934f37c198587480a31b8c25a938c53a12b823b46c6c68
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 486a82dc4016214ce0f051aa8d8283cf802e7ff06b0b10717bc2623962a747d1
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/README.md
      historical_git_mode: '100644'
      owner: evidence.sdk-task-context-direct-getter-consumption
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: da7f192d0adcbd5f7daf0a954aaef67be7ece207ed256163cb97a4637e753433
      owner_fingerprint: d8ebbb2ed3b84c77eb81692a98a4dd0f27b25cf70a37b2923f9fb7e0cdf7b7ae
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 1d04d497e278a3ac07934f37c198587480a31b8c25a938c53a12b823b46c6c68
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_050c30cc27494573
  ? evidence.sdk-task-context-direct-getter-consumption#source:8102d6e752f330b05608051224f54a47f3687967f079f1e48afd50bcbe24988d
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: bfbf92ba77057f52213724ccea008ee76f5192879ea6d4af3bd26884b9214b81
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/focused-source-manifest.json
      historical_git_mode: '100644'
      owner: evidence.sdk-task-context-direct-getter-consumption
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 934742cfed7a65bb922e1b3c18b3b69f9483070b3524ed74f9673d7dbef0c17d
      owner_fingerprint: d8ebbb2ed3b84c77eb81692a98a4dd0f27b25cf70a37b2923f9fb7e0cdf7b7ae
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 8102d6e752f330b05608051224f54a47f3687967f079f1e48afd50bcbe24988d
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_0fa1d79ff6b8a0ad
  ? evidence.sdk-task-context-direct-getter-consumption#source:4bb8082aa73bf785954586eeb4407dbb02edaff195429ca070fef5c9662942e9
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 4c99066780a6448143bcf9c818c088482cd00de846b1875db60851844944ca17
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/focused-active-claim.log
      historical_git_mode: '100644'
      owner: evidence.sdk-task-context-direct-getter-consumption
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 46eea9b5d937df1c8a03bd883ec41f3b191e8f61925bd945beb8643c27d73e27
      owner_fingerprint: d8ebbb2ed3b84c77eb81692a98a4dd0f27b25cf70a37b2923f9fb7e0cdf7b7ae
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 4bb8082aa73bf785954586eeb4407dbb02edaff195429ca070fef5c9662942e9
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_a78c35d968e2d253
  ? evidence.sdk-task-context-direct-getter-consumption#source:6d48f1c923cb64149404487f246e267ed458349873e70d8e55860dab52552327
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: d5dff21a9db00bf376a2204df529046fad8543f8c9e9041c78ac133634ed863b
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/task-context-test-only.patch
      historical_git_mode: '100644'
      owner: evidence.sdk-task-context-direct-getter-consumption
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 62ccab4c7b74799635c7e641ba31ac28306ae4aaa51629b80dd6b08d2b6d2bdc
      owner_fingerprint: d8ebbb2ed3b84c77eb81692a98a4dd0f27b25cf70a37b2923f9fb7e0cdf7b7ae
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 6d48f1c923cb64149404487f246e267ed458349873e70d8e55860dab52552327
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_dbc3b26b1205bcab
  ? evidence.sdk-task-memory-workflow-finite-observation#source:e46e619a603b599db1749e785f3460d27dec74f6ee7df6ef0209d86ab0f3b105
  : disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: fd77fbb8369de7c108d08d09dcf76d3e28d27cca336cbe666172fbbcd2601b0c
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  ? evidence.sdk-task-memory-workflow-finite-observation#source:7c7a9d698eb9e1ecfc41cd3e78dac27b0a3f02b5650f45b9464ea596f2b55b5d
  : disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 107f43c9911d45f0cef18e2347d5175a3fafefe66bf0c44a300a17f4f2eb52ba
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  ? evidence.sdk-task-memory-workflow-finite-observation#source:1532ab8b88dc82af92f15ea334ea1cbe64cf7c8aa7248f9315b3387eeda4fa0f
  : disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 48b1512b2058de4026fcd298ce46b268b2fd57ccf3335fdfb8622ddb76d656e9
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  ? evidence.sdk-task-memory-workflow-finite-observation#source:d516d4af66328d7c7dbb6d1ec6c940b14cc8364c306228aa30d96f1472aa3825
  : disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 72aa4e751f43bff2d593a2f08986a805b37549f6fc524dca2ea4dc3189994e80
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  ? evidence.sdk-task-memory-workflow-finite-observation#source:2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c
  : disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 7bbb37c5c643eb138ffdeaf9ab92f82f729efd20ab59c14a2974ad10527490ef
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  ? evidence.sdk-task-memory-workflow-finite-observation#source:b1c6ed389e99b2ad3d37c87e303e6580c5a38a72d3cb5a98943352cc0188d77c
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 53645ecd4a15d0bfdb416098ea3c5ab0d8bb742d98277cd5cd9844c2255294da
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  ? evidence.sdk-task-memory-workflow-finite-observation#source:178b87c0dd3caa4f3584253e619d07b009778ee104e33bee88ec548956901a8e
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 4fcaad3c3ec2e5be0a5bbe2f1176fb4075a2e76d9895bbb594351b3499afa73c
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  ? evidence.sdk-task-memory-workflow-finite-observation#source:e2f0f758a330e6733f46f2faa61a51ea2fca4e5ab38e6a3eef465486fbdcadfb
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: d0d8e3208b0db4649f9a4390e3b53d0e421e7318476dbe79f1e2154b76fa766a
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  ? evidence.sdk-task-memory-workflow-finite-observation#source:b3ef4d04ba8326acb5b63ac9aea35a1922db5fecd1c53ae3cc0c27eb23a059d8
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 49a8807f2c244dc0a47f553e738f3bb1572fe809036bef712990723008c4721a
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  ? evidence.sdk-task-memory-workflow-finite-observation#source:92d04f11a95f5784971d384976963559d0043dca035959879f3035257b4c9f5a
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: aaf79f61f9ef29e30e5194ee0a644a4d8a2e90d4de240e4781191cf7ecb56483
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  ? evidence.sdk-task-memory-workflow-finite-observation#source:740ca2f0b24d1b8261becacb4152e3975ae955c886b2c072a24119b64c406944
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 28e6ae98edfdf7f8e232b94de888a75c39018fbdc823e0fc7c21799b271af96c
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  ? evidence.sdk-task-memory-workflow-finite-observation#source:3e91e2836c450b6eddb30115ec38e58f1859faaeb4ff61eccabaa685ab40dd8a
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: adda9a718d917d2c556c9952038dba2ede245874ebb249b5a35845f423d37fe2
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  ? evidence.sdk-task-memory-workflow-finite-observation#source:5bde6ae445aba3da6a8a60adfbbb621c06c281b7bf3fae0ab3d2eea969bb0bab
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: c76266a6fd14d878c079e8d01ca8bc8924c39ad90097f3e3fc29707b26880c94
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  ? evidence.sdk-task-memory-workflow-finite-observation#source:3a26d5b2c89773ada31776c5f23bc208b775c5201662d1ece1543c797f5562d1
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 7478bba719ab02ccfa9dffe235456cca89f245002958496ffd98b04dafdb0193
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  ? evidence.sdk-task-memory-workflow-finite-observation#source:33ea0aadb26ecea7ab429d244b5cd7a9f6b533a7398a2a0d2f4ed7ce3df0c2ef
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 29e56a237c1b3eeb77bb532246f848c2651c4b21681392bc543ca8e28507d9f7
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  ? evidence.sdk-task-memory-workflow-finite-observation#source:e5b0756444a3d3901ded91abb4c83196c8d74a83d56234d8c6d092140a133702
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 4dadcd0bd55c0f146ab3293460505ca405862a09e10972ce12956801c4fd4744
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  ? evidence.sdk-task-memory-workflow-finite-observation#source:e49dcb7fd702c1ba4c3fadc59bdf62d93af8625ba2cdb9d41e60e6e67c44e034
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 6ead73a9a7824409e4123b81eedc294d1d3fff399f4de5742f42ac32bbf5181a
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  ? evidence.sdk-task-memory-workflow-finite-observation#source:97235eb49f2cb2a1a51e717549029643cd446d7e498e25647814227ab9f4acb1
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: d47a7d667f59646a92e79ad9b571fd0dc9c7eb6e25a811023edb2cb2ff785925
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  ? evidence.sdk-task-memory-workflow-finite-observation#source:d1deddfa3e89f23988e2ace05ccedb0790a6fe6be678a854bf84b2d0e9d7e0dc
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 9e0b846941dff5c4810113c4d2ebb08d7ab292279f6db7a1da1f9ef073b30c66
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  ? evidence.sdk-task-memory-workflow-finite-observation#source:0972d0a8d569b4e3671e580f616756fb75d6f9705dc3eb801a78308db7508d99
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 8dedf9a5719434dc2c556f22b6aa8ce5536dd035d2652afa4aa0a385120bdb29
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  ? evidence.sdk-task-memory-workflow-finite-observation#source:4c9b49122dfe24154be9eacb96c8d054bf62fe04d1dce4d9ac4d513db8b42f60
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 530285d6799e43477371805c2d793dc424d403ec908f07176fd64f763d1cdec9
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-task-subagent-candidate#source:021defd2133ad6df64390f0a234ad2ec76c250ff34409d2acf3e7fdc96c944fc:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 36b20427b56a63e591f83a29d824bcf665bb4dca8c25daa067a7c6a1ceaab0e3
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-task-subagent-candidate#source:ec752440a607a4d187967378bea4792ac6702479ffb1af1f1451322944d64653:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 59a6059fba7083088f8e3cc554bf49a28e16a41600aa0fd513d66498f09290ea
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-task-subagent-candidate#source:a575708c8f0948efd0f66686e15ef452d9e9718b77c45948295a0bd95c04d153:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: f9e9397c53942ab66bec18a809434a0615b21407bff40c8f33688e2a5b9c724f
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-task-subagent-candidate#source:18b9826f0aeb35de8171793e6ef955ad49b8cb2dc8fae00aab2ba39eee2ceac2:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: efb7711218d2bec39e29a4ac16658ec92389f72314645928802f7156423df2b0
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-task-subagent-candidate#source:5e645ffe834a4da96b3cef3a730bfa7007e7d496a760beb1060803d4912afa4a:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: d37f690ad89d6b2419cfab27e30936d2014b65194d08c14621175c3ee85d200c
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-task-subagent-candidate#source:e85d1c715ddfcabb2702b88f49b200053300c7f9581bbeb31a60e5ad6b896668:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 9e89d67083390dd0e15699555cac50fe05b1b32854743bb00228f164d25191ea
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-task-subagent-candidate#source:594ab84a723a1b513ec1939b99a00a745c698396807110c32475ebab83571c3e:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: c1192c90d5a26671215cfdc0f9c75e9f166b8050c3486235144815227ca6e9d2
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-task-subagent-candidate#source:d9d57f5234c5f6478af52ad529fd9f2ce0bfdfb354580be8de93080597304f72:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 10f4ce04a16726144f9b8d8d7796ac271612a2c53f44cd0dce2d10a050dcfa15
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-task-subagent-candidate#source:d44e9ff012033e928933fa3afd158c8e38267ac29e35c72d7d2d5f1dd0c9d93e:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 0a9931d87a5913a841852c7c90d922407b4a8b3d6856ae601029a9f5b9191d0e
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-task-subagent-candidate#source:765c4c97f57aa224e9771591a6dbb8ec54759d89669a2934f2b8b6341a0b56fc:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: c7164743702bcab461848a8b4d44ddffe66f498ecdd9b8c79638755415aee1a8
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-task-subagent-candidate-20261013/proof.json
      historical_git_mode: '100644'
      owner: evidence.sdk-task-subagent-candidate
      source_role: documentation
      observed_revision: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
      issue_id: f5db56b3a7a38c7d8e13ce633640d336cc619a1984a7b92f12290a85ed2b5f61
      owner_fingerprint: 3e59ba66ee6f098e99e788c6c0dce607df267f075a9be9e31ab81a16c8f8bc53
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 765c4c97f57aa224e9771591a6dbb8ec54759d89669a2934f2b8b6341a0b56fc
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_bf18815a8e70603d
  evidence.sdk-task-subagent-candidate#source:d23a8d2f09e94fc0c2caa186a10d53207e1eeff7196db0ad0e2a9fc2220ff696:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 01eef6f93611c7c811fd678fe55fa29ed2050ad349dd5515e150a0ed8adb29da
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-task-subagent-candidate-20261013/archive-index.json
      historical_git_mode: '100644'
      owner: evidence.sdk-task-subagent-candidate
      source_role: documentation
      observed_revision: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
      issue_id: e4987cb1d7b4f5bfd440ddd1d38af381e6d3b46e597a826923f73f48ca395090
      owner_fingerprint: 3e59ba66ee6f098e99e788c6c0dce607df267f075a9be9e31ab81a16c8f8bc53
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: d23a8d2f09e94fc0c2caa186a10d53207e1eeff7196db0ad0e2a9fc2220ff696
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_d930bd03f5532e24
  evidence.sdk-task-subagent-candidate#source:381503435c515b7609008809f9d7f98d0e00fee6a0f5c2700472ee7a6c6b8e16:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: ffc69b66a431f082223b658eacb091f3f6e0eeab40496c39e2dbc923e4d349bf
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-task-subagent-candidate-20261013/files/task_subagent_fixture.py.txt
      historical_git_mode: '100644'
      owner: evidence.sdk-task-subagent-candidate
      source_role: documentation
      observed_revision: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
      issue_id: e9bfadaa9fbba39fdd387c22384592c4f959ad81a99134277495fbe447186660
      owner_fingerprint: 3e59ba66ee6f098e99e788c6c0dce607df267f075a9be9e31ab81a16c8f8bc53
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 381503435c515b7609008809f9d7f98d0e00fee6a0f5c2700472ee7a6c6b8e16
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_84a9848c8b9a40ba
  evidence.sdk-task-subagent-candidate#source:d216108adda823b93604aad5646960e905413c5d0ef2689f7ab30e88f8f36a4f:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 337ed32b1cede01301d619903f33acd5265215f60e821e9fe1ad73005dd5e45f
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    current_reference_replacements:
    - raw_reference: docs/sdk/governance/evidence/sdk-task-subagent-candidate-20261013/files/task-subagent-host.test.js.txt
      historical_git_mode: '100644'
      owner: evidence.sdk-task-subagent-candidate
      source_role: test_consumer
      observed_revision: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
      issue_id: e3353ea8c60de4f4b39327c115f10afb757b688ba54904982071cab4e822a071
      owner_fingerprint: 3e59ba66ee6f098e99e788c6c0dce607df267f075a9be9e31ab81a16c8f8bc53
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: d216108adda823b93604aad5646960e905413c5d0ef2689f7ab30e88f8f36a4f
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_a7efaeb2e8646da4
      replacement_source_ref: sdks/typescript/test/task-subagent-host.test.js
      replacement_content_sha256: 104a5729e2f603cbda7836572155294073e9dbe2d9dde8e89774f66b32d8a930
      replacement_git_mode: '100644'
  evidence.sdk-task-subagent-candidate#source:4f1c4d4be47727863644698eb10977e744c8e550a60893bd14afbe0e2c8be837:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: b0a882028c5a8937854af3dd9e0773fcd322cf6c219c048eab86658e72c39fc6
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    current_reference_replacements:
    - raw_reference: docs/sdk/governance/evidence/sdk-task-subagent-candidate-20261013/files/test_task_subagent_host.py.txt
      historical_git_mode: '100644'
      owner: evidence.sdk-task-subagent-candidate
      source_role: test_consumer
      observed_revision: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
      issue_id: 91781ae46e3422c16b6587126604b601426420ac20a15e639ee21f62c726e466
      owner_fingerprint: 3e59ba66ee6f098e99e788c6c0dce607df267f075a9be9e31ab81a16c8f8bc53
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 4f1c4d4be47727863644698eb10977e744c8e550a60893bd14afbe0e2c8be837
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_e44ebbb898afe8e2
      replacement_source_ref: sdks/python/tests/test_task_subagent_host.py
      replacement_content_sha256: a69cfaa5f175c84dadf74a7176502648c96f86993bc4ae33b43361ef99727a38
      replacement_git_mode: '100644'
  evidence.sdk-task-subagent-candidate#source:6f680de1db27db451259d94594bc7a1650757344b5984ca717bdcd6c6848d833:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 266e5bd6358e15fc508838138dd71a66af2b3870a4ea406710dad1a8759943a8
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-task-subagent-candidate-20261013/files/TaskSubagentCandidate.java.txt
      historical_git_mode: '100644'
      owner: evidence.sdk-task-subagent-candidate
      source_role: documentation
      observed_revision: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
      issue_id: ae55fd473235a0d904163355d6be51c2e1529c77cb46cdad2c4e6113ef7c4c0f
      owner_fingerprint: 3e59ba66ee6f098e99e788c6c0dce607df267f075a9be9e31ab81a16c8f8bc53
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 6f680de1db27db451259d94594bc7a1650757344b5984ca717bdcd6c6848d833
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_5cdb7dc3b75b785b
  evidence.sdk-task-subagent-candidate#source:45ce26a5a04d9e4bb0bb76d7b4253c347a38085cf0253420571ca461f4d4c546:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 3f81b14188a5ad354fcf6f2cec0f13698a319b6a549e77a3c59cb08acf2e71e3
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-task-subagent-candidate-20261013/files/scenario-plan.md.txt
      historical_git_mode: '100644'
      owner: evidence.sdk-task-subagent-candidate
      source_role: documentation
      observed_revision: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
      issue_id: 45c75298eae3e96c3d188b571ae27a0193df7c0cc9bbe8cdfa1ba5a6caaed846
      owner_fingerprint: 3e59ba66ee6f098e99e788c6c0dce607df267f075a9be9e31ab81a16c8f8bc53
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 45ce26a5a04d9e4bb0bb76d7b4253c347a38085cf0253420571ca461f4d4c546
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_3051bddb8ba3f7bd
  evidence.sdk-task-subagent-candidate#source:51d91e90e3d3d6796cb6342fb6ca13e7e89198d2ca3f1637828dea02e65a39ca:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 60a7f43dd584e283917438fe0523947bc1aebc2fd9749e57b7991151b3165d5e
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-task-subagent-candidate-20261013/files/README.md.txt
      historical_git_mode: '100644'
      owner: evidence.sdk-task-subagent-candidate
      source_role: documentation
      observed_revision: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
      issue_id: e2a0391f08d2fe982145328f2ba86a38ff998d1af03e237031530e9371ae1b5f
      owner_fingerprint: 3e59ba66ee6f098e99e788c6c0dce607df267f075a9be9e31ab81a16c8f8bc53
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 51d91e90e3d3d6796cb6342fb6ca13e7e89198d2ca3f1637828dea02e65a39ca
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_db62cf970c74ce4d
  evidence.sdk-task-subagent-candidate#source:9824cc09e892d3f70b2e55494160ee52e52ca24270b07336f9e4f790ea9397ce:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: faa04fa7298699283ce9bf97d8c27a9ee39c4e1fda3ff5c8eca1e25ada774795
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/final-receipt.json
      historical_git_mode: '100644'
      owner: evidence.sdk-task-subagent-candidate
      source_role: documentation
      observed_revision: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
      issue_id: 2c0372f0e38cdb37a6401c8835685c92edada034d1968c05a1e6262517685c1c
      owner_fingerprint: 3e59ba66ee6f098e99e788c6c0dce607df267f075a9be9e31ab81a16c8f8bc53
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 9824cc09e892d3f70b2e55494160ee52e52ca24270b07336f9e4f790ea9397ce
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_bf349d00d0202359
  evidence.sdk-task-subagent-candidate#source:5e0f153848725261abf228db5320a840d3b0afef6b72f1b8f076a02cdf6eded3:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: afa85d09a95bf2e8264d8e410d8bb423c21bd3f0b0c94e420e11c074642fbdfa
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/source-build-manifest.json
      historical_git_mode: '100644'
      owner: evidence.sdk-task-subagent-candidate
      source_role: documentation
      observed_revision: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
      issue_id: 13c4820d012903f74b98747459d7fcad8d50ec58ef96f18b23f948414a8a1348
      owner_fingerprint: 3e59ba66ee6f098e99e788c6c0dce607df267f075a9be9e31ab81a16c8f8bc53
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 5e0f153848725261abf228db5320a840d3b0afef6b72f1b8f076a02cdf6eded3
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_81249dec4d319945
  evidence.sdk-testing-value-candidate#source:594ab84a723a1b513ec1939b99a00a745c698396807110c32475ebab83571c3e:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: c1192c90d5a26671215cfdc0f9c75e9f166b8050c3486235144815227ca6e9d2
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-testing-value-candidate#source:d9d57f5234c5f6478af52ad529fd9f2ce0bfdfb354580be8de93080597304f72:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 10f4ce04a16726144f9b8d8d7796ac271612a2c53f44cd0dce2d10a050dcfa15
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-testing-value-candidate#source:d44e9ff012033e928933fa3afd158c8e38267ac29e35c72d7d2d5f1dd0c9d93e:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 0a9931d87a5913a841852c7c90d922407b4a8b3d6856ae601029a9f5b9191d0e
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-testing-value-candidate#source:9824cc09e892d3f70b2e55494160ee52e52ca24270b07336f9e4f790ea9397ce:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: faa04fa7298699283ce9bf97d8c27a9ee39c4e1fda3ff5c8eca1e25ada774795
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/final-receipt.json
      historical_git_mode: '100644'
      owner: evidence.sdk-testing-value-candidate
      source_role: documentation
      observed_revision: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
      issue_id: 7ab16865b1c8db46969a69995ecff8c081f2edc72913b9e72cc98b0f9c730c02
      owner_fingerprint: 871ba0e7ce9033e1c5705ab7e43bae181ab2c12b541373934441337609681753
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 9824cc09e892d3f70b2e55494160ee52e52ca24270b07336f9e4f790ea9397ce
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_af9e2a3b37518be5
  evidence.sdk-testing-value-candidate#source:5e0f153848725261abf228db5320a840d3b0afef6b72f1b8f076a02cdf6eded3:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: afa85d09a95bf2e8264d8e410d8bb423c21bd3f0b0c94e420e11c074642fbdfa
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/source-build-manifest.json
      historical_git_mode: '100644'
      owner: evidence.sdk-testing-value-candidate
      source_role: documentation
      observed_revision: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
      issue_id: fbc74c639fd9c546c4be512cf1bd48ea85ddda4c1b24e7060136c25de3e6503c
      owner_fingerprint: 871ba0e7ce9033e1c5705ab7e43bae181ab2c12b541373934441337609681753
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 5e0f153848725261abf228db5320a840d3b0afef6b72f1b8f076a02cdf6eded3
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_8f4bc5525900c050
  evidence.sdk-tool-behavior-candidate#source:594ab84a723a1b513ec1939b99a00a745c698396807110c32475ebab83571c3e:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: c1192c90d5a26671215cfdc0f9c75e9f166b8050c3486235144815227ca6e9d2
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-tool-behavior-candidate#source:d9d57f5234c5f6478af52ad529fd9f2ce0bfdfb354580be8de93080597304f72:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 10f4ce04a16726144f9b8d8d7796ac271612a2c53f44cd0dce2d10a050dcfa15
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-tool-behavior-candidate#source:d44e9ff012033e928933fa3afd158c8e38267ac29e35c72d7d2d5f1dd0c9d93e:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 0a9931d87a5913a841852c7c90d922407b4a8b3d6856ae601029a9f5b9191d0e
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-tool-behavior-candidate#source:9824cc09e892d3f70b2e55494160ee52e52ca24270b07336f9e4f790ea9397ce:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: faa04fa7298699283ce9bf97d8c27a9ee39c4e1fda3ff5c8eca1e25ada774795
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/final-receipt.json
      historical_git_mode: '100644'
      owner: evidence.sdk-tool-behavior-candidate
      source_role: documentation
      observed_revision: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
      issue_id: fabc1e2edabd8f642ecf99af3ba2e223c2615a3b85ccdabab81cd31306f950fa
      owner_fingerprint: 5e237087df6ad9f6775c4fdb3c269692bc187b1b2d076f2330ea678c49614536
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 9824cc09e892d3f70b2e55494160ee52e52ca24270b07336f9e4f790ea9397ce
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_f56e5b1c5d146e07
  evidence.sdk-tool-behavior-candidate#source:5e0f153848725261abf228db5320a840d3b0afef6b72f1b8f076a02cdf6eded3:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: afa85d09a95bf2e8264d8e410d8bb423c21bd3f0b0c94e420e11c074642fbdfa
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/source-build-manifest.json
      historical_git_mode: '100644'
      owner: evidence.sdk-tool-behavior-candidate
      source_role: documentation
      observed_revision: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
      issue_id: 2620b7239baa070401c44d11e8ae3eb20b888e624763500949f4a75f4dcd87f9
      owner_fingerprint: 5e237087df6ad9f6775c4fdb3c269692bc187b1b2d076f2330ea678c49614536
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 5e0f153848725261abf228db5320a840d3b0afef6b72f1b8f076a02cdf6eded3
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_84f652c10c30c76f
  evidence.sdk-tool-input-validation-candidate#source:594ab84a723a1b513ec1939b99a00a745c698396807110c32475ebab83571c3e:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: c1192c90d5a26671215cfdc0f9c75e9f166b8050c3486235144815227ca6e9d2
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-tool-input-validation-candidate#source:d9d57f5234c5f6478af52ad529fd9f2ce0bfdfb354580be8de93080597304f72:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 10f4ce04a16726144f9b8d8d7796ac271612a2c53f44cd0dce2d10a050dcfa15
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-tool-input-validation-candidate#source:d44e9ff012033e928933fa3afd158c8e38267ac29e35c72d7d2d5f1dd0c9d93e:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 0a9931d87a5913a841852c7c90d922407b4a8b3d6856ae601029a9f5b9191d0e
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-tool-input-validation-candidate#source:1c6e2511db43e3a7b7cd008410b496f7661eef7e546b6dbe6287bff45e458062:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 2646f2da14ccb0398ca2f5055128461fbed54eeccf990c73b71a2c30747a5e29
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-tool-validator-20261013/sdk-tool-validation-host-proof.json
      historical_git_mode: '100644'
      owner: evidence.sdk-tool-input-validation-candidate
      source_role: documentation
      observed_revision: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
      issue_id: c91074faaa28ae697d42f4d01943a1786ceccf9eebe25c2368d1763b4030ee3e
      owner_fingerprint: 4a55f5b55ae8b364df8314d40293c63620e75fd885df13c120af78e98426faf3
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 1c6e2511db43e3a7b7cd008410b496f7661eef7e546b6dbe6287bff45e458062
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_c519c4f18ddc16fe
  evidence.sdk-tool-input-validation-candidate#source:0ade84fc63876ecc3df9d43078da7a858a93d68a22cacc36e33ff83045f778c2:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 2992585c251fd722bd513a9167adf3e3f3a2b8864c0ca818dacd555745f4e6e0
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-tool-validator-20261013/sdk-chart-input-probe.log
      historical_git_mode: '100644'
      owner: evidence.sdk-tool-input-validation-candidate
      source_role: documentation
      observed_revision: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
      issue_id: eb8b6e270f28181e85a6ba3eb5bb6147f61af478674d778ce9698276f66aa529
      owner_fingerprint: 4a55f5b55ae8b364df8314d40293c63620e75fd885df13c120af78e98426faf3
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 0ade84fc63876ecc3df9d43078da7a858a93d68a22cacc36e33ff83045f778c2
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_402441b5134d5872
  evidence.sdk-tool-input-validation-candidate#source:5cf8e266111cd96234832a85f5056c77997d9b02da560842b35826ed7c451d56:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: cdce15a43498f1034316bb018194be58d9915522ea3957ebafc7086c269e4cf6
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-tool-validator-20261013/sdk-chart-input-probe-green.log
      historical_git_mode: '100644'
      owner: evidence.sdk-tool-input-validation-candidate
      source_role: documentation
      observed_revision: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
      issue_id: 68e7b8b8d3f539f8035f6657555aef5478cf1856bdfb29000d214b68592c5bfb
      owner_fingerprint: 4a55f5b55ae8b364df8314d40293c63620e75fd885df13c120af78e98426faf3
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 5cf8e266111cd96234832a85f5056c77997d9b02da560842b35826ed7c451d56
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_49fd2b8a84d26c86
  evidence.sdk-tool-input-validation-candidate#source:f47fb28eeba28db8aea99803e874dd8d2fd3035cf7f1be9dc5153d08b9b81b07:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 392b10ae43037f1bf4d31613937212fd2ea2be4164eb9b0d0180cf95807728e5
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-tool-validator-20261013/archive-index.json
      historical_git_mode: '100644'
      owner: evidence.sdk-tool-input-validation-candidate
      source_role: documentation
      observed_revision: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
      issue_id: 21a5ecdf3ccac2348b4de894fa1df5d765a6973d7c880eeca402ad27dd9ac09f
      owner_fingerprint: 4a55f5b55ae8b364df8314d40293c63620e75fd885df13c120af78e98426faf3
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: f47fb28eeba28db8aea99803e874dd8d2fd3035cf7f1be9dc5153d08b9b81b07
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_cf455fbeb0d95fe6
  evidence.sdk-tool-input-validation-candidate#source:9824cc09e892d3f70b2e55494160ee52e52ca24270b07336f9e4f790ea9397ce:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: faa04fa7298699283ce9bf97d8c27a9ee39c4e1fda3ff5c8eca1e25ada774795
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/final-receipt.json
      historical_git_mode: '100644'
      owner: evidence.sdk-tool-input-validation-candidate
      source_role: documentation
      observed_revision: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
      issue_id: a1c9647c835527513375d7746ef753b70d474acb45baa0ab373bb2e17fb391f3
      owner_fingerprint: 4a55f5b55ae8b364df8314d40293c63620e75fd885df13c120af78e98426faf3
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 9824cc09e892d3f70b2e55494160ee52e52ca24270b07336f9e4f790ea9397ce
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_c186a785a353e017
  evidence.sdk-tool-input-validation-candidate#source:5e0f153848725261abf228db5320a840d3b0afef6b72f1b8f076a02cdf6eded3:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: afa85d09a95bf2e8264d8e410d8bb423c21bd3f0b0c94e420e11c074642fbdfa
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/source-build-manifest.json
      historical_git_mode: '100644'
      owner: evidence.sdk-tool-input-validation-candidate
      source_role: documentation
      observed_revision: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
      issue_id: bb575ce4662fd7bd4d1073eeb424030daa3c15de832d1b19fd38e6ec401321f1
      owner_fingerprint: 4a55f5b55ae8b364df8314d40293c63620e75fd885df13c120af78e98426faf3
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 5e0f153848725261abf228db5320a840d3b0afef6b72f1b8f076a02cdf6eded3
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_533d069853ab73f6
  ? evidence.sdk-tool-value-natural-eof-after-candidate#source:2b631efaa331eb85281a54edcebbcc32170a877ae4829525ba62072b067c5e3c
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 38d4e595c7380a8147bf4246e6ed7a8b430b6e491381c241c05699338e1e33ef
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/tool-value-effects/b909-natural-eof-after/manifest.json
      historical_git_mode: '100644'
      owner: evidence.sdk-tool-value-natural-eof-after-candidate
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 93671a0815dee6ccf9e0b6558bb4ce7f30a523644e27a5e00f49a2f2852f91c7
      owner_fingerprint: c7d2fa760b0120b3a468aa8f7f302e34e211f53ba0adc49f61628115fa23321e
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 2b631efaa331eb85281a54edcebbcc32170a877ae4829525ba62072b067c5e3c
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_cc7a45732d1ee2da
  ? evidence.sdk-tool-value-natural-eof-after-candidate#source:2415f6c17c7ba7de8efd76a56d5443b34a346fd5503832e45647d83fca278648
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: e5e051fe5549ad964bbd9d12b0fa95e7e53c52f806fe53e03882baa24d420ad0
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/tool-value-effects/b909-natural-eof-after/SHA256SUMS.json
      historical_git_mode: '100644'
      owner: evidence.sdk-tool-value-natural-eof-after-candidate
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 019afcb3efb9caafd60f30c0f4181a9ad721c3c15813c5328c18e7ad7e7a6379
      owner_fingerprint: c7d2fa760b0120b3a468aa8f7f302e34e211f53ba0adc49f61628115fa23321e
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 2415f6c17c7ba7de8efd76a56d5443b34a346fd5503832e45647d83fca278648
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_1f66b6bc10cbe3f4
  ? evidence.sdk-tool-value-natural-eof-after-candidate#source:3f60de6567b93259e9fdcd024175cbc2bd10979f76c089f78d4bb60fd41924a6
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 2e906f9369d6a789e8efee76cc0b396163b951424b7bdf267be874591fea4b36
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/tool-value-effects/b909-natural-eof-after/tool-comparison.json
      historical_git_mode: '100644'
      owner: evidence.sdk-tool-value-natural-eof-after-candidate
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: ac9c26c3e34a228466ac932bfc9ef8f60d1f852ff9191374904f39dc8f43823a
      owner_fingerprint: c7d2fa760b0120b3a468aa8f7f302e34e211f53ba0adc49f61628115fa23321e
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 3f60de6567b93259e9fdcd024175cbc2bd10979f76c089f78d4bb60fd41924a6
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_02d930e8b5a82348
  evidence.sdk-tools-bridge-finite-observation#source:77c1d305a55e8320692929efca4f48a6d6dc7a4370cd33c9b403da996dcc15c3:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 4a54cfd65a3678bb42de68160c7d380925f44ac9284205ee059a2d384c82681f
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-tools-bridge-finite-observation#source:2df1bbe8ff3221c690d016379030dee6f76afa8f1893f8ccadb1791d14769649:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: bfe247b85537ba880bcabd936800e762a56fed7241cc498491ca622924398d5e
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-tools-bridge-finite-observation#source:245d9daf8847d8b64fe15cf9800e7a74c7caf6c448da5576cd82161d4fc371b0:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: b8f67164014094962879b4dda64f0cf7c9bd0e9967bbacf29919f2df7aea1b36
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-tools-bridge-finite-observation#source:be870643edf05900800a083e09a678b367749060138c1295f2aecd4f46faf5fd:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: fc9f409d49f71f97e258f9ffd969611a5abdc06c55fa7b52c771b3a76fb6bf1b
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-tools-bridge-finite-observation#source:a54fc88cbcdaedc2f9df5c71f8bdd7648f043a43d63eebce2e587c6ae6f028b6:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: d0952ffec34372a8d37af29a0a4d5b7cfcc7c9b3540fe72e01d38f7df1995d13
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-tools-bridge-finite-observation#source:ec681c139c44708be94fbebc08ce73c81d4c976d6a7914def0f0bd39da445e7e:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 2cdeccacd2db8c9eaa4edbd10d437fdbabfd3630cde5f0b52232368c20d067aa
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-tools-bridge-finite-observation#source:3136c2a7c39c3759e75074f0d80fe7b054c8c54f671bc6c40265f85375c3fd34:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 3b546370789d5a878dc54a0c28bfd94b8c2af69e8a04c299bb0af80941bf0558
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-tools-bridge-finite-observation#source:a819a6b9db73e1af3948292e0b79e9c66a96003c34633666f9c94dd1f0eb57b7:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: f68c66e63e0652d4a0b58fe6e0f32fe4d94e4b40928c117772bb5fdc3ef31230
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-tools-bridge-finite-observation#source:49f19a1f66b3baba2dfa1cc9b3b3eb5dee068ed3c15551983cfc43296d486216:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 39b91be59facd79c2cc5b5fa5c4509b2c499ce538c3ed6a2c559c71e1b7dcd27
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-tools-bridge-finite-observation#source:b1c6ed389e99b2ad3d37c87e303e6580c5a38a72d3cb5a98943352cc0188d77c:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 53645ecd4a15d0bfdb416098ea3c5ab0d8bb742d98277cd5cd9844c2255294da
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-tools-bridge-finite-observation#source:178b87c0dd3caa4f3584253e619d07b009778ee104e33bee88ec548956901a8e:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 4fcaad3c3ec2e5be0a5bbe2f1176fb4075a2e76d9895bbb594351b3499afa73c
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-tools-bridge-finite-observation#source:b2b86d6e7d59855ffb2e0758804a9f2533559bf62ad38001ca35bd1768a5b595:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: c135265ed8870f29682b0a4404b084dc5908df072e7e8eba1c85de2a37c05ecd
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-tools-bridge-finite-observation#source:4d47c7c9e9d541087e0ef2e48cb9d6a495b357216ee92276ab6a6e267195be37:
    disposition: resolved_conflict
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: e9e934e19104ffc707b2b10bfbb8e354f01c921cf00a1c6673f323c145a13d65
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-tools-bridge-finite-observation#source:3e91e2836c450b6eddb30115ec38e58f1859faaeb4ff61eccabaa685ab40dd8a:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: adda9a718d917d2c556c9952038dba2ede245874ebb249b5a35845f423d37fe2
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-tools-bridge-finite-observation#source:df13c0187206340d2e0fb8a1553a13c78b5da7a1dfaaccd3d8857095c7987cbc:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 2b2def3acf3c220e8d5fb63d9fb8b8256fd2394327628788691fafcdd80daa8c
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-tools-bridge-finite-observation#source:3a26d5b2c89773ada31776c5f23bc208b775c5201662d1ece1543c797f5562d1:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 7478bba719ab02ccfa9dffe235456cca89f245002958496ffd98b04dafdb0193
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-tools-bridge-finite-observation#source:c8dc3f1263ae7a84190080211fbde2eb6e6d93dade6f7b9d24da85026355f9ac:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 6cc9e1d93eecbd39456c62ef4ee77fa110e955fb2af6052d328ec0b270a5dda8
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-tools-bridge-finite-observation#source:8e7aad2896687394a507f339fb20cba2dbacb93fd42ab38b417534570204e8e9:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: ec567507e53300fa87ea93e3f614d82e0451b7c0d43a0d779e3240f1d79a0ebf
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-tools-bridge-finite-observation#source:1653a22e81c1b2c62fb10de02fa77978e6d062f9e1bda6f530290a8a7b0bb2cc:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 06565614b024eb260b1ba0ba07021db6dd95b682776d7291bab94f3160054bd7
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  ? evidence.sdk-typescript-stdio-close-specific-closure#source:eb3d7eeda52e34cbe4820d9b3c6b72690087485b191f6dcfcab20ca0f054da39
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 71c2c652387c967774e473a9759e92e396528765ee9bfcab5a38497498238be9
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  ? evidence.sdk-typescript-stdio-close-specific-closure#source:28ee6e9e2555858ceda3959c32d3d02aa6a604ebcdf1a0c432e491d713dce01f
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 4cb06d229a9e80cdbe21da8515cd92d36cc9da2600ea80deaa8a54375ecfe20d
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  ? evidence.sdk-typescript-stdio-close-specific-closure#source:dd822f73e5cfeb84a5d082923b255d18357b3480a03602ec6a79fcb45e9aeb3f
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 38b4b9d92c71109c08e0bbec74fbdd2651a76d8235c58aaa18dfc6ed479d411d
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  ? evidence.sdk-typescript-stdio-close-specific-closure#source:2b44c81b00e530d7b6ffe84bd8b71c3afa7f0d39fb727f413dafe331183e24aa
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 8c55413cd8fc5d3a40db23bb1a59cb786d886f5f545df4bbf806d829c2b3ba34
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  ? evidence.sdk-typescript-stdio-close-specific-closure#source:b1c6ed389e99b2ad3d37c87e303e6580c5a38a72d3cb5a98943352cc0188d77c
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 53645ecd4a15d0bfdb416098ea3c5ab0d8bb742d98277cd5cd9844c2255294da
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  ? evidence.sdk-typescript-stdio-close-specific-closure#source:c5067422bba892564c813fbd8af4dd3df5b8928296cae7eb3849104c9201e117
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: aadf20a8d728cd550ac736a3a0199124e1c8ae7b1bcbc4f45e28d15cb56c1e86
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  ? evidence.sdk-typescript-stdio-close-specific-closure#source:178b87c0dd3caa4f3584253e619d07b009778ee104e33bee88ec548956901a8e
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 4fcaad3c3ec2e5be0a5bbe2f1176fb4075a2e76d9895bbb594351b3499afa73c
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  ? evidence.sdk-typescript-stdio-close-specific-closure#source:d9cee6cb3e27dbf9ebb868320ffd7bf2bdab4cfda83e2b556ddea9349dfa7bbf
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 12a275c6186a08ef85a187049708837a533d7fe780fb815307c84d8f7fb47121
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  ? evidence.sdk-typescript-stdio-close-specific-closure#source:f5220ea6af0d61db753a7636ad688c2cd3b15f1b39bf1397b1b6599d3ef37046
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 0a188b5f12712293767cca9c86c8655c9224b41b2857bc6ebad3bb0d79be6be1
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-wave2-cutover-final-verification#source:83ae0ba3deaeca9ca2102092c9cd77dc334814775811ec5a8564af5a68b976f6:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 7083616c3e494b1c6231e288e3c98220f2e115e477872ff308336d37d097cd51
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-wave2-cutover-final-verification#source:45ea97df7083eb5404f70cd94bb1426acef9740236d82b109d4c7d1d1cbd77d7:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 9cb18bf8101bb85c095f8520ab30d8693f4fc92f4232008cf3dc02ce3b45031e
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-wave2-cutover-final-verification#source:9828351422d02002b941a5f1b60fade021b4cdbe74cdc8ee3b3f8739d254e9db:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 92cdc3dc3bb807e564d4bc408f79f76889b41e393d4cc7fadadc8c53ade7963e
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-wave2-cutover-repair#source:83ae0ba3deaeca9ca2102092c9cd77dc334814775811ec5a8564af5a68b976f6:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 7083616c3e494b1c6231e288e3c98220f2e115e477872ff308336d37d097cd51
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-wave2-cutover-repair#source:45ea97df7083eb5404f70cd94bb1426acef9740236d82b109d4c7d1d1cbd77d7:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 9cb18bf8101bb85c095f8520ab30d8693f4fc92f4232008cf3dc02ce3b45031e
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-wave2-cutover-repair#source:9828351422d02002b941a5f1b60fade021b4cdbe74cdc8ee3b3f8739d254e9db:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 92cdc3dc3bb807e564d4bc408f79f76889b41e393d4cc7fadadc8c53ade7963e
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-wave2-cutover-repair#source:8793b0382b493e3065829476ed7b7b7bfa6393ce1c610df03aee40f4972c6826:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 25bced96db0a8d8189f009164acbfbada6b9ee02ad985e00dac6a7bff63e7318
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-workflow-successor-candidate#source:594ab84a723a1b513ec1939b99a00a745c698396807110c32475ebab83571c3e:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: c1192c90d5a26671215cfdc0f9c75e9f166b8050c3486235144815227ca6e9d2
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-workflow-successor-candidate#source:d9d57f5234c5f6478af52ad529fd9f2ce0bfdfb354580be8de93080597304f72:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 10f4ce04a16726144f9b8d8d7796ac271612a2c53f44cd0dce2d10a050dcfa15
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-workflow-successor-candidate#source:d44e9ff012033e928933fa3afd158c8e38267ac29e35c72d7d2d5f1dd0c9d93e:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 0a9931d87a5913a841852c7c90d922407b4a8b3d6856ae601029a9f5b9191d0e
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
  evidence.sdk-workflow-successor-candidate#source:3928a354e522ea597547cd72705c1381d7f7c1b8345d7be376cba01d7a2eae6b:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 6b3a46853b741882886b8dd26d85004a6af7cb460208a1b582baf3e1f9b06164
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-workflow-checkpoint-20261013/sdk-workflow-formal-proof.json
      historical_git_mode: '100644'
      owner: evidence.sdk-workflow-successor-candidate
      source_role: documentation
      observed_revision: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
      issue_id: 37efb000eb5dac985522d2ad07baacb4780adfd9824d1e46f599142368fb599a
      owner_fingerprint: e5663f2a2636cf98939332484afef868789fdff1e0c5422ce8c44cdb62d636df
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 3928a354e522ea597547cd72705c1381d7f7c1b8345d7be376cba01d7a2eae6b
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_df03e3e1a84e9d50
  evidence.sdk-workflow-successor-candidate#source:e73d4378dadda9b78604ceb4c5836f1be9d44623717b137c81470a984957b6e2:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: c283826923d79373ed0d70c6abfb1aa2eedd0bb85d30ffe9a2e7f063b68d6b68
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-workflow-checkpoint-20261013/execution-summary.json
      historical_git_mode: '100644'
      owner: evidence.sdk-workflow-successor-candidate
      source_role: documentation
      observed_revision: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
      issue_id: 9005a87cb8028d17e9001147336fd3c5dca6b210f766cdc7b5e7ec6e556bbd4d
      owner_fingerprint: e5663f2a2636cf98939332484afef868789fdff1e0c5422ce8c44cdb62d636df
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: e73d4378dadda9b78604ceb4c5836f1be9d44623717b137c81470a984957b6e2
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_e275c80d440867fe
  evidence.sdk-workflow-successor-candidate#source:1ab800b406dd8e135405dfd70100ff4734a7bfbe33d2325b856657a8289a2d85:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 00323d5df87697b042a27b67752c7d26d227873fbb549b8485044df2b097a215
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-workflow-checkpoint-20261013/execution-evidence.md
      historical_git_mode: '100644'
      owner: evidence.sdk-workflow-successor-candidate
      source_role: documentation
      observed_revision: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
      issue_id: 0014c422317b097719cdbdbf0390312f8ed50ace725acd470d9c92ac01590478
      owner_fingerprint: e5663f2a2636cf98939332484afef868789fdff1e0c5422ce8c44cdb62d636df
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 1ab800b406dd8e135405dfd70100ff4734a7bfbe33d2325b856657a8289a2d85
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_1989a3e9ffb66138
  evidence.sdk-workflow-successor-candidate#source:9824cc09e892d3f70b2e55494160ee52e52ca24270b07336f9e4f790ea9397ce:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: faa04fa7298699283ce9bf97d8c27a9ee39c4e1fda3ff5c8eca1e25ada774795
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/final-receipt.json
      historical_git_mode: '100644'
      owner: evidence.sdk-workflow-successor-candidate
      source_role: documentation
      observed_revision: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
      issue_id: 24c300761be67fb5ffdfa215863f05590e5146255c3812400fcb720b0dc290f4
      owner_fingerprint: e5663f2a2636cf98939332484afef868789fdff1e0c5422ce8c44cdb62d636df
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 9824cc09e892d3f70b2e55494160ee52e52ca24270b07336f9e4f790ea9397ce
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_a58f869c29bf20ba
  evidence.sdk-workflow-successor-candidate#source:5e0f153848725261abf228db5320a840d3b0afef6b72f1b8f076a02cdf6eded3:
    disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: afa85d09a95bf2e8264d8e410d8bb423c21bd3f0b0c94e420e11c074642fbdfa
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/source-build-manifest.json
      historical_git_mode: '100644'
      owner: evidence.sdk-workflow-successor-candidate
      source_role: documentation
      observed_revision: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
      issue_id: 3976fbead13e3dc1f9b0d001c7d6cb44d60507336ad22692a36cf9bfbd243a54
      owner_fingerprint: e5663f2a2636cf98939332484afef868789fdff1e0c5422ce8c44cdb62d636df
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: 5e0f153848725261abf228db5320a840d3b0afef6b72f1b8f076a02cdf6eded3
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_7abdfdfbce632d99
  ? evidence.sdk-acp-agent-run-finite-observation#source:8b89f6d86577e7cd4dc22b0ca20a7642e2e1e6b57ce50b3d4a8c7ce0d8948106#variant:2d03b67b0502440d897639161990f3b84f98e775e1e8162f494483ae5829099a
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 2d03b67b0502440d897639161990f3b84f98e775e1e8162f494483ae5829099a
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: f97e05998a723971782edccf8186aa8cc3d7717a6b3192478db59967a381480e
      variant_obligation: evidence.sdk-acp-agent-run-finite-observation#source:8b89f6d86577e7cd4dc22b0ca20a7642e2e1e6b57ce50b3d4a8c7ce0d8948106#variant:2d03b67b0502440d897639161990f3b84f98e775e1e8162f494483ae5829099a
      variant_fingerprint: 2d03b67b0502440d897639161990f3b84f98e775e1e8162f494483ae5829099a
      raw_reference: docs/sdk/governance/evidence/session-run-idempotency-implementation/current/source/echo-sdk-host/src/core_profile/handles.rs
      source_role: documentation
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: docs/sdk/governance/semantic-history.md
      replacement_content_sha256: c37cc4ea8183ca53ea1a617bda46e8bf1f5f86617021c71a1506e075b35237b3
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_34bdce8a8e405594
  ? evidence.sdk-acp-agent-run-finite-observation#source:8b89f6d86577e7cd4dc22b0ca20a7642e2e1e6b57ce50b3d4a8c7ce0d8948106#variant:a8c7e976f4c7e6725f84722cf73f2a2688092b9dca4f31c166d66caab2657819
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: a8c7e976f4c7e6725f84722cf73f2a2688092b9dca4f31c166d66caab2657819
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: f97e05998a723971782edccf8186aa8cc3d7717a6b3192478db59967a381480e
      variant_obligation: evidence.sdk-acp-agent-run-finite-observation#source:8b89f6d86577e7cd4dc22b0ca20a7642e2e1e6b57ce50b3d4a8c7ce0d8948106#variant:a8c7e976f4c7e6725f84722cf73f2a2688092b9dca4f31c166d66caab2657819
      variant_fingerprint: a8c7e976f4c7e6725f84722cf73f2a2688092b9dca4f31c166d66caab2657819
      raw_reference: echo-sdk-host/src/core_profile/handles.rs
      source_role: production_source
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/src/core_profile/handles.rs
      replacement_content_sha256: 8b89f6d86577e7cd4dc22b0ca20a7642e2e1e6b57ce50b3d4a8c7ce0d8948106
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_b4e44d4500b7a047
  ? evidence.sdk-acp-agent-run-finite-observation#source:8e3eb3592334cef9dae9bcc6787820528fc706c8b1276387e1e973d727f0eb80#variant:34a48e98359066971347643f81279127e00b5148fc171e4c938fca64e4146179
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 34a48e98359066971347643f81279127e00b5148fc171e4c938fca64e4146179
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: c4fb3cfc3757b62e4e7d3cfddc55e2d44a7d8731dee5749b1500ea15309ad647
      variant_obligation: evidence.sdk-acp-agent-run-finite-observation#source:8e3eb3592334cef9dae9bcc6787820528fc706c8b1276387e1e973d727f0eb80#variant:34a48e98359066971347643f81279127e00b5148fc171e4c938fca64e4146179
      variant_fingerprint: 34a48e98359066971347643f81279127e00b5148fc171e4c938fca64e4146179
      raw_reference: docs/sdk/governance/evidence/session-run-idempotency-implementation/current/source/echo-sdk-host/src/core_profile/state.rs
      source_role: documentation
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: docs/sdk/governance/semantic-history.md
      replacement_content_sha256: c37cc4ea8183ca53ea1a617bda46e8bf1f5f86617021c71a1506e075b35237b3
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_e611226700c92b36
  ? evidence.sdk-acp-agent-run-finite-observation#source:8e3eb3592334cef9dae9bcc6787820528fc706c8b1276387e1e973d727f0eb80#variant:dfb93e8723575365a5b0875c0c6aa814a89024c26c953daf617e38db00992872
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: dfb93e8723575365a5b0875c0c6aa814a89024c26c953daf617e38db00992872
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: c4fb3cfc3757b62e4e7d3cfddc55e2d44a7d8731dee5749b1500ea15309ad647
      variant_obligation: evidence.sdk-acp-agent-run-finite-observation#source:8e3eb3592334cef9dae9bcc6787820528fc706c8b1276387e1e973d727f0eb80#variant:dfb93e8723575365a5b0875c0c6aa814a89024c26c953daf617e38db00992872
      variant_fingerprint: dfb93e8723575365a5b0875c0c6aa814a89024c26c953daf617e38db00992872
      raw_reference: echo-sdk-host/src/core_profile/state.rs
      source_role: production_source
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/src/core_profile/state.rs
      replacement_content_sha256: 8e3eb3592334cef9dae9bcc6787820528fc706c8b1276387e1e973d727f0eb80
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_6afbb0080d435a40
  ? evidence.sdk-channel-message-handler-consumer-audit#source:245d9daf8847d8b64fe15cf9800e7a74c7caf6c448da5576cd82161d4fc371b0#variant:634c306d77724b887b3ef5eb4c9bd91544ceb594d5262c53858bb72e5119fffd
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 634c306d77724b887b3ef5eb4c9bd91544ceb594d5262c53858bb72e5119fffd
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: 8bb436e27ec4d5935925030f11db0d9d0d2534c65f3ab48eb8c6d45efd3cdaf1
      variant_obligation: evidence.sdk-channel-message-handler-consumer-audit#source:245d9daf8847d8b64fe15cf9800e7a74c7caf6c448da5576cd82161d4fc371b0#variant:634c306d77724b887b3ef5eb4c9bd91544ceb594d5262c53858bb72e5119fffd
      variant_fingerprint: 634c306d77724b887b3ef5eb4c9bd91544ceb594d5262c53858bb72e5119fffd
      raw_reference: echo-sdk-host/src/core_profile/extension_bridge.rs#ExtensionChannelMessageHandlerProxy
      source_role: production_source
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/src/core_profile/extension_bridge.rs
      replacement_content_sha256: 245d9daf8847d8b64fe15cf9800e7a74c7caf6c448da5576cd82161d4fc371b0
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_a68d1607cfc2e72a
  ? evidence.sdk-channel-message-handler-consumer-audit#source:245d9daf8847d8b64fe15cf9800e7a74c7caf6c448da5576cd82161d4fc371b0#variant:dc3cfc8a80285b9c1c7a414763db5c17ab6fa8dc01270c0b15e9e57b8833e2b3
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: dc3cfc8a80285b9c1c7a414763db5c17ab6fa8dc01270c0b15e9e57b8833e2b3
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: 8bb436e27ec4d5935925030f11db0d9d0d2534c65f3ab48eb8c6d45efd3cdaf1
      variant_obligation: evidence.sdk-channel-message-handler-consumer-audit#source:245d9daf8847d8b64fe15cf9800e7a74c7caf6c448da5576cd82161d4fc371b0#variant:dc3cfc8a80285b9c1c7a414763db5c17ab6fa8dc01270c0b15e9e57b8833e2b3
      variant_fingerprint: dc3cfc8a80285b9c1c7a414763db5c17ab6fa8dc01270c0b15e9e57b8833e2b3
      raw_reference: echo-sdk-host/src/core_profile/extension_bridge.rs#ExtensionChannelPluginProxy
      source_role: production_source
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/src/core_profile/extension_bridge.rs
      replacement_content_sha256: 245d9daf8847d8b64fe15cf9800e7a74c7caf6c448da5576cd82161d4fc371b0
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_fd93949ec5c18457
  ? evidence.sdk-computation-50#source:77c1d305a55e8320692929efca4f48a6d6dc7a4370cd33c9b403da996dcc15c3#variant:03fa76555b60da8df60afeeb1b330bc886ec369814a5cb9226f1974050420767
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 03fa76555b60da8df60afeeb1b330bc886ec369814a5cb9226f1974050420767
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: cd08d2e0773c93b317512c94680f06740f7badbf7e8d8bbacf0f2225bcab1104
      variant_obligation: evidence.sdk-computation-50#source:77c1d305a55e8320692929efca4f48a6d6dc7a4370cd33c9b403da996dcc15c3#variant:03fa76555b60da8df60afeeb1b330bc886ec369814a5cb9226f1974050420767
      variant_fingerprint: 03fa76555b60da8df60afeeb1b330bc886ec369814a5cb9226f1974050420767
      raw_reference: echo-sdk-host/src/core_profile/facade/tools.rs
      source_role: production_source
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/src/core_profile/facade/tools.rs
      replacement_content_sha256: 77c1d305a55e8320692929efca4f48a6d6dc7a4370cd33c9b403da996dcc15c3
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_e036cb244949b78f
  ? evidence.sdk-computation-50#source:77c1d305a55e8320692929efca4f48a6d6dc7a4370cd33c9b403da996dcc15c3#variant:92f1c8d56c94815483e7f0d290e58cf821b3437d9c207cf48611c1662b24f0ef
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 92f1c8d56c94815483e7f0d290e58cf821b3437d9c207cf48611c1662b24f0ef
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: cd08d2e0773c93b317512c94680f06740f7badbf7e8d8bbacf0f2225bcab1104
      variant_obligation: evidence.sdk-computation-50#source:77c1d305a55e8320692929efca4f48a6d6dc7a4370cd33c9b403da996dcc15c3#variant:92f1c8d56c94815483e7f0d290e58cf821b3437d9c207cf48611c1662b24f0ef
      variant_fingerprint: 92f1c8d56c94815483e7f0d290e58cf821b3437d9c207cf48611c1662b24f0ef
      raw_reference: echo-sdk-host/src/core_profile/facade/tools.rs#run_bound_tool
      source_role: production_source
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/src/core_profile/facade/tools.rs
      replacement_content_sha256: 77c1d305a55e8320692929efca4f48a6d6dc7a4370cd33c9b403da996dcc15c3
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_350309e67f236620
  ? evidence.sdk-gap-watermark-host-verification#source:e4ae808abb8f916a7318faec3643c5a56cd1bbd143e9e88cb447e0cf7127f7de#variant:6d6ff1fe333d037399d5809ee1cf44aa96d9aeb492f6015f030af8030e99947a
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 6d6ff1fe333d037399d5809ee1cf44aa96d9aeb492f6015f030af8030e99947a
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    historical_reference_migrations:
    - raw_reference: docs/sdk/governance/evidence/sdk-gap-watermark-host-red/events-fixed.rs
      historical_git_mode: '100644'
      owner: evidence.sdk-gap-watermark-host-verification
      source_role: documentation
      observed_revision: 56929154c1851473e9e4837901b610cecf001b2d
      issue_id: 3206578581ab59f1bb0320da9edc46aed0588a9bdb2e22c3668acfe92c3050aa
      owner_fingerprint: 15afc174f202233780de11962fa4b3c6141b59a98609f0b86ec5fa4fe852ae62
      snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      historical_content_sha256: e4ae808abb8f916a7318faec3643c5a56cd1bbd143e9e88cb447e0cf7127f7de
      replacement_ref: asset.sdk-historical-proof-location
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_9da50bd689e632f5
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: 734f4d19651a24a68bb3f064c4843f4377b2cad276f91c9bcce61043a84406d6
      variant_obligation: evidence.sdk-gap-watermark-host-verification#source:e4ae808abb8f916a7318faec3643c5a56cd1bbd143e9e88cb447e0cf7127f7de#variant:6d6ff1fe333d037399d5809ee1cf44aa96d9aeb492f6015f030af8030e99947a
      variant_fingerprint: 6d6ff1fe333d037399d5809ee1cf44aa96d9aeb492f6015f030af8030e99947a
      raw_reference: docs/sdk/governance/evidence/sdk-gap-watermark-host-red/events-fixed.rs
      source_role: documentation
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: docs/sdk/governance/semantic-history.md
      replacement_content_sha256: c37cc4ea8183ca53ea1a617bda46e8bf1f5f86617021c71a1506e075b35237b3
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_9da50bd689e632f5
  ? evidence.sdk-gap-watermark-host-verification#source:e4ae808abb8f916a7318faec3643c5a56cd1bbd143e9e88cb447e0cf7127f7de#variant:b5eee78e8c71980154e987678aa90fd93b582e2de0e6da35e54a36de8a69a5db
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: b5eee78e8c71980154e987678aa90fd93b582e2de0e6da35e54a36de8a69a5db
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: 734f4d19651a24a68bb3f064c4843f4377b2cad276f91c9bcce61043a84406d6
      variant_obligation: evidence.sdk-gap-watermark-host-verification#source:e4ae808abb8f916a7318faec3643c5a56cd1bbd143e9e88cb447e0cf7127f7de#variant:b5eee78e8c71980154e987678aa90fd93b582e2de0e6da35e54a36de8a69a5db
      variant_fingerprint: b5eee78e8c71980154e987678aa90fd93b582e2de0e6da35e54a36de8a69a5db
      raw_reference: echo-sdk-host/src/core_profile/events.rs#fn acknowledge
      source_role: production_source
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/src/core_profile/events.rs
      replacement_content_sha256: e4ae808abb8f916a7318faec3643c5a56cd1bbd143e9e88cb447e0cf7127f7de
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_93917c2df039f72d
  ? evidence.sdk-gap-watermark-host-verification#source:e4ae808abb8f916a7318faec3643c5a56cd1bbd143e9e88cb447e0cf7127f7de#variant:c4b9e0df984af7125f02267ede9ce787b6b10813d64dd11f6e9d4277fd88314f
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: c4b9e0df984af7125f02267ede9ce787b6b10813d64dd11f6e9d4277fd88314f
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: 734f4d19651a24a68bb3f064c4843f4377b2cad276f91c9bcce61043a84406d6
      variant_obligation: evidence.sdk-gap-watermark-host-verification#source:e4ae808abb8f916a7318faec3643c5a56cd1bbd143e9e88cb447e0cf7127f7de#variant:c4b9e0df984af7125f02267ede9ce787b6b10813d64dd11f6e9d4277fd88314f
      variant_fingerprint: c4b9e0df984af7125f02267ede9ce787b6b10813d64dd11f6e9d4277fd88314f
      raw_reference: echo-sdk-host/src/core_profile/events.rs#fn resume_after
      source_role: production_source
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/src/core_profile/events.rs
      replacement_content_sha256: e4ae808abb8f916a7318faec3643c5a56cd1bbd143e9e88cb447e0cf7127f7de
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_6e5cb69b44c74d64
  ? evidence.sdk-gap-watermark-host-verification#source:e4ae808abb8f916a7318faec3643c5a56cd1bbd143e9e88cb447e0cf7127f7de#variant:feddb5ac172a6fb507397be578d450a37845f6a10e97c15c52302a93cabfefab
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: feddb5ac172a6fb507397be578d450a37845f6a10e97c15c52302a93cabfefab
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: 734f4d19651a24a68bb3f064c4843f4377b2cad276f91c9bcce61043a84406d6
      variant_obligation: evidence.sdk-gap-watermark-host-verification#source:e4ae808abb8f916a7318faec3643c5a56cd1bbd143e9e88cb447e0cf7127f7de#variant:feddb5ac172a6fb507397be578d450a37845f6a10e97c15c52302a93cabfefab
      variant_fingerprint: feddb5ac172a6fb507397be578d450a37845f6a10e97c15c52302a93cabfefab
      raw_reference: echo-sdk-host/src/core_profile/events.rs#fn publish_replay
      source_role: production_source
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/src/core_profile/events.rs
      replacement_content_sha256: e4ae808abb8f916a7318faec3643c5a56cd1bbd143e9e88cb447e0cf7127f7de
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_47657ad2d4aee6d8
  ? evidence.sdk-integrations-finite-observation#source:21ce0c4f56c85691aa461d22a3e2b6837a0e335073c8a136bda916597201c728#variant:ace2ec8b5e0a4d0ea70e3220e1a3ff2bca7437036605b722ec34ab37721f2571
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: ace2ec8b5e0a4d0ea70e3220e1a3ff2bca7437036605b722ec34ab37721f2571
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: 9650447590997d5d41e60816570b71155d35848c43895fffa74972fa3e9eb999
      variant_obligation: evidence.sdk-integrations-finite-observation#source:21ce0c4f56c85691aa461d22a3e2b6837a0e335073c8a136bda916597201c728#variant:ace2ec8b5e0a4d0ea70e3220e1a3ff2bca7437036605b722ec34ab37721f2571
      variant_fingerprint: ace2ec8b5e0a4d0ea70e3220e1a3ff2bca7437036605b722ec34ab37721f2571
      raw_reference: echo-sdk-host/src/core_profile/facade/state_delivery.rs#exact_claim_roundtrip_rejects_delayed_attempt_and_id_only
      source_role: production_source
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/src/core_profile/facade/state_delivery.rs
      replacement_content_sha256: 21ce0c4f56c85691aa461d22a3e2b6837a0e335073c8a136bda916597201c728
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_01c885962e7fcd99
  ? evidence.sdk-integrations-finite-observation#source:21ce0c4f56c85691aa461d22a3e2b6837a0e335073c8a136bda916597201c728#variant:c8badfec113facb5f85de711a2de0e27c369febf0bce5cc2cf758f4b93823e02
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: c8badfec113facb5f85de711a2de0e27c369febf0bce5cc2cf758f4b93823e02
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: 9650447590997d5d41e60816570b71155d35848c43895fffa74972fa3e9eb999
      variant_obligation: evidence.sdk-integrations-finite-observation#source:21ce0c4f56c85691aa461d22a3e2b6837a0e335073c8a136bda916597201c728#variant:c8badfec113facb5f85de711a2de0e27c369febf0bce5cc2cf758f4b93823e02
      variant_fingerprint: c8badfec113facb5f85de711a2de0e27c369febf0bce5cc2cf758f4b93823e02
      raw_reference: echo-sdk-host/src/core_profile/facade/state_delivery.rs
      source_role: production_source
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/src/core_profile/facade/state_delivery.rs
      replacement_content_sha256: 21ce0c4f56c85691aa461d22a3e2b6837a0e335073c8a136bda916597201c728
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_b47084df5167c8f5
  ? evidence.sdk-integrations-finite-observation#source:2875aafb87d6543a43135c635e065899f8b58c6325b6a9497e5d07903e1cf8e9#variant:21430e5ffa936c7c66f65ef7722093b1d65d52abfb272cbf40d3b82fa0464afa
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 21430e5ffa936c7c66f65ef7722093b1d65d52abfb272cbf40d3b82fa0464afa
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: 51db358cc79aee92c4cdde925fd81619f6d45a1cc5213b0f20a39123056df837
      variant_obligation: evidence.sdk-integrations-finite-observation#source:2875aafb87d6543a43135c635e065899f8b58c6325b6a9497e5d07903e1cf8e9#variant:21430e5ffa936c7c66f65ef7722093b1d65d52abfb272cbf40d3b82fa0464afa
      variant_fingerprint: 21430e5ffa936c7c66f65ef7722093b1d65d52abfb272cbf40d3b82fa0464afa
      raw_reference: echo-sdk-host/src/core_profile/facade/observability.rs#failed_trace_open_does_not_consume_the_only_resource_slot
      source_role: production_source
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/src/core_profile/facade/observability.rs
      replacement_content_sha256: 2875aafb87d6543a43135c635e065899f8b58c6325b6a9497e5d07903e1cf8e9
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_46af90cadd206268
  ? evidence.sdk-integrations-finite-observation#source:2875aafb87d6543a43135c635e065899f8b58c6325b6a9497e5d07903e1cf8e9#variant:ea6c934054311646c70a475750e4867c06f6634d6a76d2686c7e78a05953234e
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: ea6c934054311646c70a475750e4867c06f6634d6a76d2686c7e78a05953234e
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: 51db358cc79aee92c4cdde925fd81619f6d45a1cc5213b0f20a39123056df837
      variant_obligation: evidence.sdk-integrations-finite-observation#source:2875aafb87d6543a43135c635e065899f8b58c6325b6a9497e5d07903e1cf8e9#variant:ea6c934054311646c70a475750e4867c06f6634d6a76d2686c7e78a05953234e
      variant_fingerprint: ea6c934054311646c70a475750e4867c06f6634d6a76d2686c7e78a05953234e
      raw_reference: echo-sdk-host/src/core_profile/facade/observability.rs
      source_role: production_source
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/src/core_profile/facade/observability.rs
      replacement_content_sha256: 2875aafb87d6543a43135c635e065899f8b58c6325b6a9497e5d07903e1cf8e9
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_5eb11508c0f7e742
  ? evidence.sdk-integrations-finite-observation#source:2df1bbe8ff3221c690d016379030dee6f76afa8f1893f8ccadb1791d14769649#variant:2172623585f7149d8e946c75d710f149aef2e542ebeea4f953be5701ebeb4594
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 2172623585f7149d8e946c75d710f149aef2e542ebeea4f953be5701ebeb4594
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: 555e44dcb08ce01ec7ab4805f0ade55bc21ba7f3b746e36816db8e2724f6c53e
      variant_obligation: evidence.sdk-integrations-finite-observation#source:2df1bbe8ff3221c690d016379030dee6f76afa8f1893f8ccadb1791d14769649#variant:2172623585f7149d8e946c75d710f149aef2e542ebeea4f953be5701ebeb4594
      variant_fingerprint: 2172623585f7149d8e946c75d710f149aef2e542ebeea4f953be5701ebeb4594
      raw_reference: echo-sdk-host/src/core_profile/facade/source_operations.rs
      source_role: production_source
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/src/core_profile/facade/source_operations.rs
      replacement_content_sha256: 2df1bbe8ff3221c690d016379030dee6f76afa8f1893f8ccadb1791d14769649
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_aae82cfcf922e99e
  ? evidence.sdk-integrations-finite-observation#source:2df1bbe8ff3221c690d016379030dee6f76afa8f1893f8ccadb1791d14769649#variant:afb45c2b129aa2c3cd1b6e9a034bd5e95aaa2c32f45066ed825c29cf001a3504
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: afb45c2b129aa2c3cd1b6e9a034bd5e95aaa2c32f45066ed825c29cf001a3504
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: 555e44dcb08ce01ec7ab4805f0ade55bc21ba7f3b746e36816db8e2724f6c53e
      variant_obligation: evidence.sdk-integrations-finite-observation#source:2df1bbe8ff3221c690d016379030dee6f76afa8f1893f8ccadb1791d14769649#variant:afb45c2b129aa2c3cd1b6e9a034bd5e95aaa2c32f45066ed825c29cf001a3504
      variant_fingerprint: afb45c2b129aa2c3cd1b6e9a034bd5e95aaa2c32f45066ed825c29cf001a3504
      raw_reference: echo-sdk-host/src/core_profile/facade/source_operations.rs#register_mcp_client_resources
      source_role: production_source
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/src/core_profile/facade/source_operations.rs
      replacement_content_sha256: 2df1bbe8ff3221c690d016379030dee6f76afa8f1893f8ccadb1791d14769649
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_85947ddfa7374030
  ? evidence.sdk-integrations-finite-observation#source:875b938aacd0886b84e74db7cde848f22b3d4fb28bdda61fd744d8beff1ce344#variant:0f94dc5fa830e2015de2a0560fc5f904b08fc86962a1f6ff76e6744ec13edf9b
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 0f94dc5fa830e2015de2a0560fc5f904b08fc86962a1f6ff76e6744ec13edf9b
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: 810a559d1f81abb2565f070c772b6b5f67c69b3b68c5669266067e38086bb8ef
      variant_obligation: evidence.sdk-integrations-finite-observation#source:875b938aacd0886b84e74db7cde848f22b3d4fb28bdda61fd744d8beff1ce344#variant:0f94dc5fa830e2015de2a0560fc5f904b08fc86962a1f6ff76e6744ec13edf9b
      variant_fingerprint: 0f94dc5fa830e2015de2a0560fc5f904b08fc86962a1f6ff76e6744ec13edf9b
      raw_reference: echo-sdk-host/tests/facade_feature_adapters_e2e.rs#agent_cascade_preserves_same_named_other_manager_and_releases_resources
      source_role: test_consumer
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/tests/facade_feature_adapters_e2e.rs
      replacement_content_sha256: 875b938aacd0886b84e74db7cde848f22b3d4fb28bdda61fd744d8beff1ce344
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_8c516209b9ae4658
  ? evidence.sdk-integrations-finite-observation#source:875b938aacd0886b84e74db7cde848f22b3d4fb28bdda61fd744d8beff1ce344#variant:56fd775ed3d8c7228d97ad3b82ad4f050be5c0e6936d5175ffa6f0223e7fb4ca
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 56fd775ed3d8c7228d97ad3b82ad4f050be5c0e6936d5175ffa6f0223e7fb4ca
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: 810a559d1f81abb2565f070c772b6b5f67c69b3b68c5669266067e38086bb8ef
      variant_obligation: evidence.sdk-integrations-finite-observation#source:875b938aacd0886b84e74db7cde848f22b3d4fb28bdda61fd744d8beff1ce344#variant:56fd775ed3d8c7228d97ad3b82ad4f050be5c0e6936d5175ffa6f0223e7fb4ca
      variant_fingerprint: 56fd775ed3d8c7228d97ad3b82ad4f050be5c0e6936d5175ffa6f0223e7fb4ca
      raw_reference: echo-sdk-host/tests/facade_feature_adapters_e2e.rs#mcp_file_batch_quota_failure_does_not_publish_unreturned_prefix
      source_role: test_consumer
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/tests/facade_feature_adapters_e2e.rs
      replacement_content_sha256: 875b938aacd0886b84e74db7cde848f22b3d4fb28bdda61fd744d8beff1ce344
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_09e6cff212885587
  ? evidence.sdk-integrations-finite-observation#source:875b938aacd0886b84e74db7cde848f22b3d4fb28bdda61fd744d8beff1ce344#variant:6e3b96c790f1fe78ee68e8bc28aa1341abeafb80cd750d71b52d8e9310f5bd50
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 6e3b96c790f1fe78ee68e8bc28aa1341abeafb80cd750d71b52d8e9310f5bd50
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: 810a559d1f81abb2565f070c772b6b5f67c69b3b68c5669266067e38086bb8ef
      variant_obligation: evidence.sdk-integrations-finite-observation#source:875b938aacd0886b84e74db7cde848f22b3d4fb28bdda61fd744d8beff1ce344#variant:6e3b96c790f1fe78ee68e8bc28aa1341abeafb80cd750d71b52d8e9310f5bd50
      variant_fingerprint: 6e3b96c790f1fe78ee68e8bc28aa1341abeafb80cd750d71b52d8e9310f5bd50
      raw_reference: echo-sdk-host/tests/facade_feature_adapters_e2e.rs#mcp_config_batch_quota_failure_does_not_publish_unreturned_prefix
      source_role: test_consumer
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/tests/facade_feature_adapters_e2e.rs
      replacement_content_sha256: 875b938aacd0886b84e74db7cde848f22b3d4fb28bdda61fd744d8beff1ce344
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_bb083d917d365280
  ? evidence.sdk-integrations-finite-observation#source:875b938aacd0886b84e74db7cde848f22b3d4fb28bdda61fd744d8beff1ce344#variant:a1230886222d2b6ba5b197b3877a7fb02d9bf46eb5533c98026add7f1180a3c2
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: a1230886222d2b6ba5b197b3877a7fb02d9bf46eb5533c98026add7f1180a3c2
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: 810a559d1f81abb2565f070c772b6b5f67c69b3b68c5669266067e38086bb8ef
      variant_obligation: evidence.sdk-integrations-finite-observation#source:875b938aacd0886b84e74db7cde848f22b3d4fb28bdda61fd744d8beff1ce344#variant:a1230886222d2b6ba5b197b3877a7fb02d9bf46eb5533c98026add7f1180a3c2
      variant_fingerprint: a1230886222d2b6ba5b197b3877a7fb02d9bf46eb5533c98026add7f1180a3c2
      raw_reference: echo-sdk-host/tests/facade_feature_adapters_e2e.rs
      source_role: test_consumer
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/tests/facade_feature_adapters_e2e.rs
      replacement_content_sha256: 875b938aacd0886b84e74db7cde848f22b3d4fb28bdda61fd744d8beff1ce344
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_3b0ea45fb1459ed0
  ? evidence.sdk-integrations-finite-observation#source:95c3b306c857efd65bd315029dab7a5f346f0c7926201778ed70043042d26628#variant:03cddd401ab37fa101742900064c5faba76b2a59635dd166ab559e7ca1b3f1bc
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 03cddd401ab37fa101742900064c5faba76b2a59635dd166ab559e7ca1b3f1bc
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: da45603ef1f054a8a26b85ba956eb1ab375fcc8d0e66971d9567d2876a3ffed6
      variant_obligation: evidence.sdk-integrations-finite-observation#source:95c3b306c857efd65bd315029dab7a5f346f0c7926201778ed70043042d26628#variant:03cddd401ab37fa101742900064c5faba76b2a59635dd166ab559e7ca1b3f1bc
      variant_fingerprint: 03cddd401ab37fa101742900064c5faba76b2a59635dd166ab559e7ca1b3f1bc
      raw_reference: echo-sdk-host/src/core_profile/facade/mod.rs
      source_role: production_source
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/src/core_profile/facade/mod.rs
      replacement_content_sha256: 95c3b306c857efd65bd315029dab7a5f346f0c7926201778ed70043042d26628
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_885f0be3b4329bb5
  ? evidence.sdk-integrations-finite-observation#source:95c3b306c857efd65bd315029dab7a5f346f0c7926201778ed70043042d26628#variant:8e224184bbe17e72ed757e9d839997d35fc4e396b5edeaf64fbd0ab04c145d49
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 8e224184bbe17e72ed757e9d839997d35fc4e396b5edeaf64fbd0ab04c145d49
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: da45603ef1f054a8a26b85ba956eb1ab375fcc8d0e66971d9567d2876a3ffed6
      variant_obligation: evidence.sdk-integrations-finite-observation#source:95c3b306c857efd65bd315029dab7a5f346f0c7926201778ed70043042d26628#variant:8e224184bbe17e72ed757e9d839997d35fc4e396b5edeaf64fbd0ab04c145d49
      variant_fingerprint: 8e224184bbe17e72ed757e9d839997d35fc4e396b5edeaf64fbd0ab04c145d49
      raw_reference: echo-sdk-host/src/core_profile/facade/mod.rs#family_handle_argument_retains_opaque_resource_address
      source_role: production_source
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/src/core_profile/facade/mod.rs
      replacement_content_sha256: 95c3b306c857efd65bd315029dab7a5f346f0c7926201778ed70043042d26628
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_63396be640a4a36b
  ? evidence.sdk-integrations-finite-observation#source:95c3b306c857efd65bd315029dab7a5f346f0c7926201778ed70043042d26628#variant:977f78c5893f5f6eb0af1a82a32961269fde1614aeb6a2944fdf4c9c04fda01d
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 977f78c5893f5f6eb0af1a82a32961269fde1614aeb6a2944fdf4c9c04fda01d
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: da45603ef1f054a8a26b85ba956eb1ab375fcc8d0e66971d9567d2876a3ffed6
      variant_obligation: evidence.sdk-integrations-finite-observation#source:95c3b306c857efd65bd315029dab7a5f346f0c7926201778ed70043042d26628#variant:977f78c5893f5f6eb0af1a82a32961269fde1614aeb6a2944fdf4c9c04fda01d
      variant_fingerprint: 977f78c5893f5f6eb0af1a82a32961269fde1614aeb6a2944fdf4c9c04fda01d
      raw_reference: echo-sdk-host/src/core_profile/facade/mod.rs#family_argument_json
      source_role: production_source
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/src/core_profile/facade/mod.rs
      replacement_content_sha256: 95c3b306c857efd65bd315029dab7a5f346f0c7926201778ed70043042d26628
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_7ba9f077ec77cde7
  ? evidence.sdk-java-host-early-exit-verification#source:0e3181817d84578c51efb60ce4fafbe8be9e599396ab679dd8a9bacc2b06ef28#variant:5eb72c6a1ce52aee1e8c4c56afb2f3451063b8dd2d75e51121ec4213c77f4246
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 5eb72c6a1ce52aee1e8c4c56afb2f3451063b8dd2d75e51121ec4213c77f4246
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: 22fb8a1e9253cc1093a938f734da3a39bab3c4279ba395e8d6aa33429704a85a
      variant_obligation: evidence.sdk-java-host-early-exit-verification#source:0e3181817d84578c51efb60ce4fafbe8be9e599396ab679dd8a9bacc2b06ef28#variant:5eb72c6a1ce52aee1e8c4c56afb2f3451063b8dd2d75e51121ec4213c77f4246
      variant_fingerprint: 5eb72c6a1ce52aee1e8c4c56afb2f3451063b8dd2d75e51121ec4213c77f4246
      raw_reference: sdks/java/src/main/java/com/echoagent/sdk/EchoAgentClient.java#awaitForExit
      source_role: production_source
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: sdks/java/src/main/java/com/echoagent/sdk/EchoAgentClient.java
      replacement_content_sha256: 0e3181817d84578c51efb60ce4fafbe8be9e599396ab679dd8a9bacc2b06ef28
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_3b11f64d68818291
  ? evidence.sdk-java-host-early-exit-verification#source:0e3181817d84578c51efb60ce4fafbe8be9e599396ab679dd8a9bacc2b06ef28#variant:779e7d3f3c29bec85941d005bb6fafffb3d7e531566ea550e7c26b21d532e359
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 779e7d3f3c29bec85941d005bb6fafffb3d7e531566ea550e7c26b21d532e359
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: 22fb8a1e9253cc1093a938f734da3a39bab3c4279ba395e8d6aa33429704a85a
      variant_obligation: evidence.sdk-java-host-early-exit-verification#source:0e3181817d84578c51efb60ce4fafbe8be9e599396ab679dd8a9bacc2b06ef28#variant:779e7d3f3c29bec85941d005bb6fafffb3d7e531566ea550e7c26b21d532e359
      variant_fingerprint: 779e7d3f3c29bec85941d005bb6fafffb3d7e531566ea550e7c26b21d532e359
      raw_reference: sdks/java/src/main/java/com/echoagent/sdk/EchoAgentClient.java#class OwnedStdioTransport
      source_role: production_source
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: sdks/java/src/main/java/com/echoagent/sdk/EchoAgentClient.java
      replacement_content_sha256: 0e3181817d84578c51efb60ce4fafbe8be9e599396ab679dd8a9bacc2b06ef28
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_bf817830635afe49
  ? evidence.sdk-ordinary-tools-b909-verification#source:77c1d305a55e8320692929efca4f48a6d6dc7a4370cd33c9b403da996dcc15c3#variant:03fa76555b60da8df60afeeb1b330bc886ec369814a5cb9226f1974050420767
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 03fa76555b60da8df60afeeb1b330bc886ec369814a5cb9226f1974050420767
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: 6695335e57a0adf22e8a53f4f9c9e16a24b3147169223b7163138bf3ec299f94
      variant_obligation: evidence.sdk-ordinary-tools-b909-verification#source:77c1d305a55e8320692929efca4f48a6d6dc7a4370cd33c9b403da996dcc15c3#variant:03fa76555b60da8df60afeeb1b330bc886ec369814a5cb9226f1974050420767
      variant_fingerprint: 03fa76555b60da8df60afeeb1b330bc886ec369814a5cb9226f1974050420767
      raw_reference: echo-sdk-host/src/core_profile/facade/tools.rs
      source_role: production_source
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/src/core_profile/facade/tools.rs
      replacement_content_sha256: 77c1d305a55e8320692929efca4f48a6d6dc7a4370cd33c9b403da996dcc15c3
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_18b3b77ffa27aa45
  ? evidence.sdk-ordinary-tools-b909-verification#source:77c1d305a55e8320692929efca4f48a6d6dc7a4370cd33c9b403da996dcc15c3#variant:9781ef2c2cb9892b0da59759a4f6373bc500a3ad611e33260d4b497e849dabaa
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 9781ef2c2cb9892b0da59759a4f6373bc500a3ad611e33260d4b497e849dabaa
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: 6695335e57a0adf22e8a53f4f9c9e16a24b3147169223b7163138bf3ec299f94
      variant_obligation: evidence.sdk-ordinary-tools-b909-verification#source:77c1d305a55e8320692929efca4f48a6d6dc7a4370cd33c9b403da996dcc15c3#variant:9781ef2c2cb9892b0da59759a4f6373bc500a3ad611e33260d4b497e849dabaa
      variant_fingerprint: 9781ef2c2cb9892b0da59759a4f6373bc500a3ad611e33260d4b497e849dabaa
      raw_reference: echo-sdk-host/src/core_profile/facade/tools.rs#ClinicalTrialsSearchTool
      source_role: production_source
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/src/core_profile/facade/tools.rs
      replacement_content_sha256: 77c1d305a55e8320692929efca4f48a6d6dc7a4370cd33c9b403da996dcc15c3
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_998ef94d74558af0
  ? evidence.sdk-parent-driver-candidate-verification#source:1532ab8b88dc82af92f15ea334ea1cbe64cf7c8aa7248f9315b3387eeda4fa0f#variant:56aaac8b27eac67adc82206822e1fe836f95f4e928080ccd7302e3a15ea9d04b
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 56aaac8b27eac67adc82206822e1fe836f95f4e928080ccd7302e3a15ea9d04b
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: 1337d5db7d5e6156e61e74c131f4eb212bc6f1e0f718f5e1fab1b239ca60ece6
      variant_obligation: evidence.sdk-parent-driver-candidate-verification#source:1532ab8b88dc82af92f15ea334ea1cbe64cf7c8aa7248f9315b3387eeda4fa0f#variant:56aaac8b27eac67adc82206822e1fe836f95f4e928080ccd7302e3a15ea9d04b
      variant_fingerprint: 56aaac8b27eac67adc82206822e1fe836f95f4e928080ccd7302e3a15ea9d04b
      raw_reference: echo-sdk-host/src/core_profile/facade/memory.rs#unsupported_store_operation_keeps_memory_cause_without_mutation
      source_role: production_source
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/src/core_profile/facade/memory.rs
      replacement_content_sha256: 1532ab8b88dc82af92f15ea334ea1cbe64cf7c8aa7248f9315b3387eeda4fa0f
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_ab93dcd0cd6d87d7
  ? evidence.sdk-parent-driver-candidate-verification#source:1532ab8b88dc82af92f15ea334ea1cbe64cf7c8aa7248f9315b3387eeda4fa0f#variant:5d13663fc21caafa84581d3978588a95ff9a9cc831b9f5765264b0c18da5684c
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 5d13663fc21caafa84581d3978588a95ff9a9cc831b9f5765264b0c18da5684c
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: 1337d5db7d5e6156e61e74c131f4eb212bc6f1e0f718f5e1fab1b239ca60ece6
      variant_obligation: evidence.sdk-parent-driver-candidate-verification#source:1532ab8b88dc82af92f15ea334ea1cbe64cf7c8aa7248f9315b3387eeda4fa0f#variant:5d13663fc21caafa84581d3978588a95ff9a9cc831b9f5765264b0c18da5684c
      variant_fingerprint: 5d13663fc21caafa84581d3978588a95ff9a9cc831b9f5765264b0c18da5684c
      raw_reference: echo-sdk-host/src/core_profile/facade/memory.rs#malformed_namespace_never_writes_and_replacement_store_is_live_authority
      source_role: production_source
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/src/core_profile/facade/memory.rs
      replacement_content_sha256: 1532ab8b88dc82af92f15ea334ea1cbe64cf7c8aa7248f9315b3387eeda4fa0f
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_9018ed5726fd6975
  ? evidence.sdk-parent-driver-candidate-verification#source:2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c#variant:02573fc01a01d00f85d8bc64b16a6d9aa3335c087a2d46738808ceeebd6ebc64
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 02573fc01a01d00f85d8bc64b16a6d9aa3335c087a2d46738808ceeebd6ebc64
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: df5cbb4aa9fd8788877720b0c5ec55a002fc513f38fc865edd451b8303aaa5cb
      variant_obligation: evidence.sdk-parent-driver-candidate-verification#source:2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c#variant:02573fc01a01d00f85d8bc64b16a6d9aa3335c087a2d46738808ceeebd6ebc64
      variant_fingerprint: 02573fc01a01d00f85d8bc64b16a6d9aa3335c087a2d46738808ceeebd6ebc64
      raw_reference: echo-sdk-host/tests/core_profile_e2e.rs#workflow_family_runs_declarative_graphs_over_the_framework_engine
      source_role: test_consumer
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/tests/core_profile_e2e.rs
      replacement_content_sha256: 2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_ce65beaf1a2f508c
  ? evidence.sdk-parent-driver-candidate-verification#source:2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c#variant:032130a2e78b1ab0ace116c6b07c74abfb0320b27d163e67badca43ef4ab4b23
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 032130a2e78b1ab0ace116c6b07c74abfb0320b27d163e67badca43ef4ab4b23
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: df5cbb4aa9fd8788877720b0c5ec55a002fc513f38fc865edd451b8303aaa5cb
      variant_obligation: evidence.sdk-parent-driver-candidate-verification#source:2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c#variant:032130a2e78b1ab0ace116c6b07c74abfb0320b27d163e67badca43ef4ab4b23
      variant_fingerprint: 032130a2e78b1ab0ace116c6b07c74abfb0320b27d163e67badca43ef4ab4b23
      raw_reference: echo-sdk-host/tests/core_profile_e2e.rs#consumed_single_replay_page_ack_resumes_live_after_page_cursor
      source_role: test_consumer
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/tests/core_profile_e2e.rs
      replacement_content_sha256: 2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_dba76d55c3f0f31c
  ? evidence.sdk-parent-driver-candidate-verification#source:2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c#variant:09b097605189361490499d522d9e40d584dc849a509ae68bc074a19e64f90610
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 09b097605189361490499d522d9e40d584dc849a509ae68bc074a19e64f90610
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: df5cbb4aa9fd8788877720b0c5ec55a002fc513f38fc865edd451b8303aaa5cb
      variant_obligation: evidence.sdk-parent-driver-candidate-verification#source:2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c#variant:09b097605189361490499d522d9e40d584dc849a509ae68bc074a19e64f90610
      variant_fingerprint: 09b097605189361490499d522d9e40d584dc849a509ae68bc074a19e64f90610
      raw_reference: echo-sdk-host/tests/core_profile_e2e.rs#killed_active_run_is_interrupted_never_completed
      source_role: test_consumer
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/tests/core_profile_e2e.rs
      replacement_content_sha256: 2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_ec39050627ccd156
  ? evidence.sdk-parent-driver-candidate-verification#source:2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c#variant:0ae91a6b0278815e1d61f39ac73e1f437ad742aa59739413e889dc0b5967b299
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 0ae91a6b0278815e1d61f39ac73e1f437ad742aa59739413e889dc0b5967b299
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: df5cbb4aa9fd8788877720b0c5ec55a002fc513f38fc865edd451b8303aaa5cb
      variant_obligation: evidence.sdk-parent-driver-candidate-verification#source:2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c#variant:0ae91a6b0278815e1d61f39ac73e1f437ad742aa59739413e889dc0b5967b299
      variant_fingerprint: 0ae91a6b0278815e1d61f39ac73e1f437ad742aa59739413e889dc0b5967b299
      raw_reference: echo-sdk-host/tests/core_profile_e2e.rs#task_pause_is_exact_and_resume_reuses_canonical_dependency_result
      source_role: test_consumer
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/tests/core_profile_e2e.rs
      replacement_content_sha256: 2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_62a1cd9194e0fa6b
  ? evidence.sdk-parent-driver-candidate-verification#source:2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c#variant:1bdb7a59a30ddc489217c06fd806657c5fb33eba5f8fe2a8484ea82a53c32be0
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 1bdb7a59a30ddc489217c06fd806657c5fb33eba5f8fe2a8484ea82a53c32be0
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: df5cbb4aa9fd8788877720b0c5ec55a002fc513f38fc865edd451b8303aaa5cb
      variant_obligation: evidence.sdk-parent-driver-candidate-verification#source:2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c#variant:1bdb7a59a30ddc489217c06fd806657c5fb33eba5f8fe2a8484ea82a53c32be0
      variant_fingerprint: 1bdb7a59a30ddc489217c06fd806657c5fb33eba5f8fe2a8484ea82a53c32be0
      raw_reference: echo-sdk-host/tests/core_profile_e2e.rs#negotiated_facade_admission_fails_closed
      source_role: test_consumer
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/tests/core_profile_e2e.rs
      replacement_content_sha256: 2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_66ea62c681d3852c
  ? evidence.sdk-parent-driver-candidate-verification#source:2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c#variant:1dfb4071365dbf229092c8c092cdfde8a3251c947a194044f21e5e469135730f
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 1dfb4071365dbf229092c8c092cdfde8a3251c947a194044f21e5e469135730f
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: df5cbb4aa9fd8788877720b0c5ec55a002fc513f38fc865edd451b8303aaa5cb
      variant_obligation: evidence.sdk-parent-driver-candidate-verification#source:2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c#variant:1dfb4071365dbf229092c8c092cdfde8a3251c947a194044f21e5e469135730f
      variant_fingerprint: 1dfb4071365dbf229092c8c092cdfde8a3251c947a194044f21e5e469135730f
      raw_reference: echo-sdk-host/tests/core_profile_e2e.rs#integration_families_use_framework_managers_and_clients
      source_role: test_consumer
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/tests/core_profile_e2e.rs
      replacement_content_sha256: 2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_5f5f5fc164e00683
  ? evidence.sdk-parent-driver-candidate-verification#source:2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c#variant:2a12d9ed67d58da9e25eac8726797042c352c42063fd0abdeaa4a2aec90c6bcb
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 2a12d9ed67d58da9e25eac8726797042c352c42063fd0abdeaa4a2aec90c6bcb
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: df5cbb4aa9fd8788877720b0c5ec55a002fc513f38fc865edd451b8303aaa5cb
      variant_obligation: evidence.sdk-parent-driver-candidate-verification#source:2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c#variant:2a12d9ed67d58da9e25eac8726797042c352c42063fd0abdeaa4a2aec90c6bcb
      variant_fingerprint: 2a12d9ed67d58da9e25eac8726797042c352c42063fd0abdeaa4a2aec90c6bcb
      raw_reference: echo-sdk-host/tests/core_profile_e2e.rs#task_cancel_is_exact_and_pending_sibling_continues
      source_role: test_consumer
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/tests/core_profile_e2e.rs
      replacement_content_sha256: 2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_6e0c5d7c937ec3c5
  ? evidence.sdk-parent-driver-candidate-verification#source:2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c#variant:3ec77053fcb62122a0fa06019d4b050588cd125eb78800c6687f0fd1d1026c3e
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 3ec77053fcb62122a0fa06019d4b050588cd125eb78800c6687f0fd1d1026c3e
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: df5cbb4aa9fd8788877720b0c5ec55a002fc513f38fc865edd451b8303aaa5cb
      variant_obligation: evidence.sdk-parent-driver-candidate-verification#source:2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c#variant:3ec77053fcb62122a0fa06019d4b050588cd125eb78800c6687f0fd1d1026c3e
      variant_fingerprint: 3ec77053fcb62122a0fa06019d4b050588cd125eb78800c6687f0fd1d1026c3e
      raw_reference: echo-sdk-host/tests/core_profile_e2e.rs#task_rpc_shares_the_session_task_authority
      source_role: test_consumer
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/tests/core_profile_e2e.rs
      replacement_content_sha256: 2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_bc22ee2563ba17c6
  ? evidence.sdk-parent-driver-candidate-verification#source:2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c#variant:4b655bc4f5b2d1a5699cdbf8e934e20fa92c4e71422a1f940fe463c043d290de
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 4b655bc4f5b2d1a5699cdbf8e934e20fa92c4e71422a1f940fe463c043d290de
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: df5cbb4aa9fd8788877720b0c5ec55a002fc513f38fc865edd451b8303aaa5cb
      variant_obligation: evidence.sdk-parent-driver-candidate-verification#source:2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c#variant:4b655bc4f5b2d1a5699cdbf8e934e20fa92c4e71422a1f940fe463c043d290de
      variant_fingerprint: 4b655bc4f5b2d1a5699cdbf8e934e20fa92c4e71422a1f940fe463c043d290de
      raw_reference: echo-sdk-host/tests/core_profile_e2e.rs#explicit_session_create_reserves_persisted_turn_identity_before_new_run
      source_role: test_consumer
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/tests/core_profile_e2e.rs
      replacement_content_sha256: 2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_5f79852399f72276
  ? evidence.sdk-parent-driver-candidate-verification#source:2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c#variant:514a9c9b2df8d888473bccdf3cb0e51804719c79306f7a972ae3cad15d3d1313
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 514a9c9b2df8d888473bccdf3cb0e51804719c79306f7a972ae3cad15d3d1313
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: df5cbb4aa9fd8788877720b0c5ec55a002fc513f38fc865edd451b8303aaa5cb
      variant_obligation: evidence.sdk-parent-driver-candidate-verification#source:2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c#variant:514a9c9b2df8d888473bccdf3cb0e51804719c79306f7a972ae3cad15d3d1313
      variant_fingerprint: 514a9c9b2df8d888473bccdf3cb0e51804719c79306f7a972ae3cad15d3d1313
      raw_reference: echo-sdk-host/tests/core_profile_e2e.rs#extended_standard_prompt_bridges_shared_core_handles
      source_role: test_consumer
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/tests/core_profile_e2e.rs
      replacement_content_sha256: 2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_b6e38f8dd82a291e
  ? evidence.sdk-parent-driver-candidate-verification#source:2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c#variant:6bc811b95efe9ebdeb88e32f5420e7ee16e8f16d5bc370e149ba976e7667f56e
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 6bc811b95efe9ebdeb88e32f5420e7ee16e8f16d5bc370e149ba976e7667f56e
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: df5cbb4aa9fd8788877720b0c5ec55a002fc513f38fc865edd451b8303aaa5cb
      variant_obligation: evidence.sdk-parent-driver-candidate-verification#source:2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c#variant:6bc811b95efe9ebdeb88e32f5420e7ee16e8f16d5bc370e149ba976e7667f56e
      variant_fingerprint: 6bc811b95efe9ebdeb88e32f5420e7ee16e8f16d5bc370e149ba976e7667f56e
      raw_reference: echo-sdk-host/tests/core_profile_e2e.rs#state_delivery_and_trace_families_use_framework_services
      source_role: test_consumer
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/tests/core_profile_e2e.rs
      replacement_content_sha256: 2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_d1f06fb58d1ec313
  ? evidence.sdk-parent-driver-candidate-verification#source:2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c#variant:74d1ae6b4bd7a2e1dd5b284006c0d5e76d0d142dacbc1b2abbe4591c2c8d7063
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 74d1ae6b4bd7a2e1dd5b284006c0d5e76d0d142dacbc1b2abbe4591c2c8d7063
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: df5cbb4aa9fd8788877720b0c5ec55a002fc513f38fc865edd451b8303aaa5cb
      variant_obligation: evidence.sdk-parent-driver-candidate-verification#source:2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c#variant:74d1ae6b4bd7a2e1dd5b284006c0d5e76d0d142dacbc1b2abbe4591c2c8d7063
      variant_fingerprint: 74d1ae6b4bd7a2e1dd5b284006c0d5e76d0d142dacbc1b2abbe4591c2c8d7063
      raw_reference: echo-sdk-host/tests/core_profile_e2e.rs#consumed_multiple_replay_page_ack_resumes_live_after_page_cursor
      source_role: test_consumer
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/tests/core_profile_e2e.rs
      replacement_content_sha256: 2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_82f5871d54fa8b08
  ? evidence.sdk-parent-driver-candidate-verification#source:2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c#variant:787058b1b5c0e693d85c4fa553d241cc82bd310c0616ae90f5a92938f8bf7551
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 787058b1b5c0e693d85c4fa553d241cc82bd310c0616ae90f5a92938f8bf7551
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: df5cbb4aa9fd8788877720b0c5ec55a002fc513f38fc865edd451b8303aaa5cb
      variant_obligation: evidence.sdk-parent-driver-candidate-verification#source:2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c#variant:787058b1b5c0e693d85c4fa553d241cc82bd310c0616ae90f5a92938f8bf7551
      variant_fingerprint: 787058b1b5c0e693d85c4fa553d241cc82bd310c0616ae90f5a92938f8bf7551
      raw_reference: echo-sdk-host/tests/core_profile_e2e.rs#tool_families_execute_framework_tools_with_the_session_cwd
      source_role: test_consumer
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/tests/core_profile_e2e.rs
      replacement_content_sha256: 2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_4ab9de27e8bfd3f5
  ? evidence.sdk-parent-driver-candidate-verification#source:2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c#variant:7e22f299bcc3ea50dc51a86569bbad0dc367c9a1d0a18ba79374262e8b5c0bf1
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 7e22f299bcc3ea50dc51a86569bbad0dc367c9a1d0a18ba79374262e8b5c0bf1
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: df5cbb4aa9fd8788877720b0c5ec55a002fc513f38fc865edd451b8303aaa5cb
      variant_obligation: evidence.sdk-parent-driver-candidate-verification#source:2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c#variant:7e22f299bcc3ea50dc51a86569bbad0dc367c9a1d0a18ba79374262e8b5c0bf1
      variant_fingerprint: 7e22f299bcc3ea50dc51a86569bbad0dc367c9a1d0a18ba79374262e8b5c0bf1
      raw_reference: echo-sdk-host/tests/core_profile_e2e.rs#eval_and_improve_families_use_the_framework_analyzers
      source_role: test_consumer
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/tests/core_profile_e2e.rs
      replacement_content_sha256: 2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_d7ce5ea9acb946a1
  ? evidence.sdk-parent-driver-candidate-verification#source:2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c#variant:882991d870a6a9315b6be77a38fa9c44a2e376429b7e02534bd7c6ce53074052
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 882991d870a6a9315b6be77a38fa9c44a2e376429b7e02534bd7c6ce53074052
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: df5cbb4aa9fd8788877720b0c5ec55a002fc513f38fc865edd451b8303aaa5cb
      variant_obligation: evidence.sdk-parent-driver-candidate-verification#source:2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c#variant:882991d870a6a9315b6be77a38fa9c44a2e376429b7e02534bd7c6ce53074052
      variant_fingerprint: 882991d870a6a9315b6be77a38fa9c44a2e376429b7e02534bd7c6ce53074052
      raw_reference: echo-sdk-host/tests/core_profile_e2e.rs#stateful_source_resources_preserve_owner_state_and_close
      source_role: test_consumer
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/tests/core_profile_e2e.rs
      replacement_content_sha256: 2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_8306b9bcd0ab9729
  ? evidence.sdk-parent-driver-candidate-verification#source:2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c#variant:948001ad174e4fb3ef281d157aedddd83e1dfee93c5a8d0097e121d1c150881a
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 948001ad174e4fb3ef281d157aedddd83e1dfee93c5a8d0097e121d1c150881a
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: df5cbb4aa9fd8788877720b0c5ec55a002fc513f38fc865edd451b8303aaa5cb
      variant_obligation: evidence.sdk-parent-driver-candidate-verification#source:2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c#variant:948001ad174e4fb3ef281d157aedddd83e1dfee93c5a8d0097e121d1c150881a
      variant_fingerprint: 948001ad174e4fb3ef281d157aedddd83e1dfee93c5a8d0097e121d1c150881a
      raw_reference: echo-sdk-host/tests/core_profile_e2e.rs#remaining_tool_families_execute_framework_tools_locally
      source_role: test_consumer
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/tests/core_profile_e2e.rs
      replacement_content_sha256: 2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_529ff557b0ecb78a
  ? evidence.sdk-parent-driver-candidate-verification#source:2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c#variant:9aa9d4545673764cc9a519bb071e50fa013e545dd48d7bd2cfbf7cffdf27d9b6
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 9aa9d4545673764cc9a519bb071e50fa013e545dd48d7bd2cfbf7cffdf27d9b6
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: df5cbb4aa9fd8788877720b0c5ec55a002fc513f38fc865edd451b8303aaa5cb
      variant_obligation: evidence.sdk-parent-driver-candidate-verification#source:2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c#variant:9aa9d4545673764cc9a519bb071e50fa013e545dd48d7bd2cfbf7cffdf27d9b6
      variant_fingerprint: 9aa9d4545673764cc9a519bb071e50fa013e545dd48d7bd2cfbf7cffdf27d9b6
      raw_reference: echo-sdk-host/tests/core_profile_e2e.rs#source_files_and_skill_registry_preserve_rust_authority
      source_role: test_consumer
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/tests/core_profile_e2e.rs
      replacement_content_sha256: 2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_fc51a6260e3ccf6f
  ? evidence.sdk-parent-driver-candidate-verification#source:2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c#variant:9df83e3252394522936e5f0e5860c991650ee7bf5566e261cb6caf839ad208d3
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 9df83e3252394522936e5f0e5860c991650ee7bf5566e261cb6caf839ad208d3
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: df5cbb4aa9fd8788877720b0c5ec55a002fc513f38fc865edd451b8303aaa5cb
      variant_obligation: evidence.sdk-parent-driver-candidate-verification#source:2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c#variant:9df83e3252394522936e5f0e5860c991650ee7bf5566e261cb6caf839ad208d3
      variant_fingerprint: 9df83e3252394522936e5f0e5860c991650ee7bf5566e261cb6caf839ad208d3
      raw_reference: echo-sdk-host/tests/core_profile_e2e.rs#subagent_short_await_preserves_result_and_dispatch_receipt
      source_role: test_consumer
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/tests/core_profile_e2e.rs
      replacement_content_sha256: 2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_722509eb5ea340b2
  ? evidence.sdk-parent-driver-candidate-verification#source:2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c#variant:a701e61616bf8a74ea8800d47af585c18202c46e73f2dc6f27310a84ad242953
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: a701e61616bf8a74ea8800d47af585c18202c46e73f2dc6f27310a84ad242953
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: df5cbb4aa9fd8788877720b0c5ec55a002fc513f38fc865edd451b8303aaa5cb
      variant_obligation: evidence.sdk-parent-driver-candidate-verification#source:2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c#variant:a701e61616bf8a74ea8800d47af585c18202c46e73f2dc6f27310a84ad242953
      variant_fingerprint: a701e61616bf8a74ea8800d47af585c18202c46e73f2dc6f27310a84ad242953
      raw_reference: echo-sdk-host/tests/core_profile_e2e.rs#session_load_reserves_persisted_turn_identity_before_new_run
      source_role: test_consumer
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/tests/core_profile_e2e.rs
      replacement_content_sha256: 2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_eafbf3d85f7971d0
  ? evidence.sdk-parent-driver-candidate-verification#source:2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c#variant:ba357cd2d61fc70e5dabfac0387ad1ece846fae375d8826dc8770f629a2ce9c5
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: ba357cd2d61fc70e5dabfac0387ad1ece846fae375d8826dc8770f629a2ce9c5
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: df5cbb4aa9fd8788877720b0c5ec55a002fc513f38fc865edd451b8303aaa5cb
      variant_obligation: evidence.sdk-parent-driver-candidate-verification#source:2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c#variant:ba357cd2d61fc70e5dabfac0387ad1ece846fae375d8826dc8770f629a2ce9c5
      variant_fingerprint: ba357cd2d61fc70e5dabfac0387ad1ece846fae375d8826dc8770f629a2ce9c5
      raw_reference: echo-sdk-host/tests/core_profile_e2e.rs#ack_window_snapshot_ack_resumes_strictly_after_watermark
      source_role: test_consumer
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/tests/core_profile_e2e.rs
      replacement_content_sha256: 2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_a5e49605f4b02f7f
  ? evidence.sdk-parent-driver-candidate-verification#source:2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c#variant:d56f215dfe91a4a8ddcfc4f2ea3e9707aa7adcc8150d744ef2c345d40c1dc853
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: d56f215dfe91a4a8ddcfc4f2ea3e9707aa7adcc8150d744ef2c345d40c1dc853
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: df5cbb4aa9fd8788877720b0c5ec55a002fc513f38fc865edd451b8303aaa5cb
      variant_obligation: evidence.sdk-parent-driver-candidate-verification#source:2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c#variant:d56f215dfe91a4a8ddcfc4f2ea3e9707aa7adcc8150d744ef2c345d40c1dc853
      variant_fingerprint: d56f215dfe91a4a8ddcfc4f2ea3e9707aa7adcc8150d744ef2c345d40c1dc853
      raw_reference: echo-sdk-host/tests/core_profile_e2e.rs#subagent_family_dispatches_and_controls_live_attempts
      source_role: test_consumer
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/tests/core_profile_e2e.rs
      replacement_content_sha256: 2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_61efb25243ad751c
  ? evidence.sdk-parent-driver-candidate-verification#source:2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c#variant:ddeb0810fe827d274ced69beac24171eb361118d80f3121d15b43ce4eb914f92
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: ddeb0810fe827d274ced69beac24171eb361118d80f3121d15b43ce4eb914f92
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: df5cbb4aa9fd8788877720b0c5ec55a002fc513f38fc865edd451b8303aaa5cb
      variant_obligation: evidence.sdk-parent-driver-candidate-verification#source:2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c#variant:ddeb0810fe827d274ced69beac24171eb361118d80f3121d15b43ce4eb914f92
      variant_fingerprint: ddeb0810fe827d274ced69beac24171eb361118d80f3121d15b43ce4eb914f92
      raw_reference: echo-sdk-host/tests/core_profile_e2e.rs#malformed_recovered_identity_rejects_actual_method_without_handle_publication
      source_role: test_consumer
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/tests/core_profile_e2e.rs
      replacement_content_sha256: 2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_8cb138d8f17935f8
  ? evidence.sdk-parent-driver-candidate-verification#source:2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c#variant:e2a1e6abde95d43a8cc18052b86db3725c47ea5903f0140cceff34afb1b157e7
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: e2a1e6abde95d43a8cc18052b86db3725c47ea5903f0140cceff34afb1b157e7
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: df5cbb4aa9fd8788877720b0c5ec55a002fc513f38fc865edd451b8303aaa5cb
      variant_obligation: evidence.sdk-parent-driver-candidate-verification#source:2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c#variant:e2a1e6abde95d43a8cc18052b86db3725c47ea5903f0140cceff34afb1b157e7
      variant_fingerprint: e2a1e6abde95d43a8cc18052b86db3725c47ea5903f0140cceff34afb1b157e7
      raw_reference: echo-sdk-host/tests/core_profile_e2e.rs#turn_receipt_source_operations_project_live_and_recovered
      source_role: test_consumer
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/tests/core_profile_e2e.rs
      replacement_content_sha256: 2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_bbe8ca97266378a4
  ? evidence.sdk-parent-driver-candidate-verification#source:2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c#variant:ef50716c98a37b94f0ac8d99b9aab7b1256a39390a48b8090be36e1eb6387d69
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: ef50716c98a37b94f0ac8d99b9aab7b1256a39390a48b8090be36e1eb6387d69
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: df5cbb4aa9fd8788877720b0c5ec55a002fc513f38fc865edd451b8303aaa5cb
      variant_obligation: evidence.sdk-parent-driver-candidate-verification#source:2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c#variant:ef50716c98a37b94f0ac8d99b9aab7b1256a39390a48b8090be36e1eb6387d69
      variant_fingerprint: ef50716c98a37b94f0ac8d99b9aab7b1256a39390a48b8090be36e1eb6387d69
      raw_reference: echo-sdk-host/tests/core_profile_e2e.rs#valid_hello_completes_full_core_lifecycle
      source_role: test_consumer
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/tests/core_profile_e2e.rs
      replacement_content_sha256: 2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_0b5b73a51a20213b
  ? evidence.sdk-parent-driver-candidate-verification#source:4d47c7c9e9d541087e0ef2e48cb9d6a495b357216ee92276ab6a6e267195be37#variant:92c34aca25dece935f8ec879fc206646638483e77fbcde72e84b9f304d3e332a
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 92c34aca25dece935f8ec879fc206646638483e77fbcde72e84b9f304d3e332a
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: 9763cbe65ca57d84e85e63d7c7d0d7badaecd6b1c704aff54cb7040ea955021c
      variant_obligation: evidence.sdk-parent-driver-candidate-verification#source:4d47c7c9e9d541087e0ef2e48cb9d6a495b357216ee92276ab6a6e267195be37#variant:92c34aca25dece935f8ec879fc206646638483e77fbcde72e84b9f304d3e332a
      variant_fingerprint: 92c34aca25dece935f8ec879fc206646638483e77fbcde72e84b9f304d3e332a
      raw_reference: echo-sdk-host/src/core_profile/wire.rs#managed_checkpoint_and_receipts_preserve_large_revisions_and_subseconds
      source_role: production_source
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/src/core_profile/wire.rs
      replacement_content_sha256: 4d47c7c9e9d541087e0ef2e48cb9d6a495b357216ee92276ab6a6e267195be37
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_9a5071ca9f80d588
  ? evidence.sdk-parent-driver-candidate-verification#source:4d47c7c9e9d541087e0ef2e48cb9d6a495b357216ee92276ab6a6e267195be37#variant:c30f9bc75da1fc60bcba57ff8605b9d8d5214d672713077643f6c6aee6af057e
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: c30f9bc75da1fc60bcba57ff8605b9d8d5214d672713077643f6c6aee6af057e
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: 9763cbe65ca57d84e85e63d7c7d0d7badaecd6b1c704aff54cb7040ea955021c
      variant_obligation: evidence.sdk-parent-driver-candidate-verification#source:4d47c7c9e9d541087e0ef2e48cb9d6a495b357216ee92276ab6a6e267195be37#variant:c30f9bc75da1fc60bcba57ff8605b9d8d5214d672713077643f6c6aee6af057e
      variant_fingerprint: c30f9bc75da1fc60bcba57ff8605b9d8d5214d672713077643f6c6aee6af057e
      raw_reference: echo-sdk-host/src/core_profile/wire.rs#unix_non_utf8_paths_round_trip_without_loss
      source_role: production_source
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/src/core_profile/wire.rs
      replacement_content_sha256: 4d47c7c9e9d541087e0ef2e48cb9d6a495b357216ee92276ab6a6e267195be37
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_03bd5971ea610f2e
  ? evidence.sdk-parent-driver-candidate-verification#source:77c1d305a55e8320692929efca4f48a6d6dc7a4370cd33c9b403da996dcc15c3#variant:6e9d6982740ed9829bac701f69d620a8eeff9af85653de2777e955bc290a5ea1
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 6e9d6982740ed9829bac701f69d620a8eeff9af85653de2777e955bc290a5ea1
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: 68763dac2c8ca0fb22b897e6d03116da72df9f32e1960bcc72189ef954a785f5
      variant_obligation: evidence.sdk-parent-driver-candidate-verification#source:77c1d305a55e8320692929efca4f48a6d6dc7a4370cd33c9b403da996dcc15c3#variant:6e9d6982740ed9829bac701f69d620a8eeff9af85653de2777e955bc290a5ea1
      variant_fingerprint: 6e9d6982740ed9829bac701f69d620a8eeff9af85653de2777e955bc290a5ea1
      raw_reference: echo-sdk-host/src/core_profile/facade/tools.rs#rag_index_and_search_share_one_session_store
      source_role: production_source
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/src/core_profile/facade/tools.rs
      replacement_content_sha256: 77c1d305a55e8320692929efca4f48a6d6dc7a4370cd33c9b403da996dcc15c3
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_fe162033aff437e9
  ? evidence.sdk-parent-driver-candidate-verification#source:77c1d305a55e8320692929efca4f48a6d6dc7a4370cd33c9b403da996dcc15c3#variant:e408837a4e3098c5efcc59643b408cd4bcfdc8117daced4bd801b8691be0d021
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: e408837a4e3098c5efcc59643b408cd4bcfdc8117daced4bd801b8691be0d021
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: 68763dac2c8ca0fb22b897e6d03116da72df9f32e1960bcc72189ef954a785f5
      variant_obligation: evidence.sdk-parent-driver-candidate-verification#source:77c1d305a55e8320692929efca4f48a6d6dc7a4370cd33c9b403da996dcc15c3#variant:e408837a4e3098c5efcc59643b408cd4bcfdc8117daced4bd801b8691be0d021
      variant_fingerprint: e408837a4e3098c5efcc59643b408cd4bcfdc8117daced4bd801b8691be0d021
      raw_reference: echo-sdk-host/src/core_profile/facade/tools.rs#tool_family_projection_preserves_partial_effects_and_artifact
      source_role: production_source
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/src/core_profile/facade/tools.rs
      replacement_content_sha256: 77c1d305a55e8320692929efca4f48a6d6dc7a4370cd33c9b403da996dcc15c3
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_acead20d2b0daa2a
  ? evidence.sdk-parent-driver-candidate-verification#source:875b938aacd0886b84e74db7cde848f22b3d4fb28bdda61fd744d8beff1ce344#variant:0cc4445cfeaa6a2410808cc0069a3de14d0862bd15ef285e26e2d7ddf4fb3194
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 0cc4445cfeaa6a2410808cc0069a3de14d0862bd15ef285e26e2d7ddf4fb3194
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: 0578b3c92581c86030e5ae82c82cb3cb28b297e8189222a39b76d1a5e8c9064d
      variant_obligation: evidence.sdk-parent-driver-candidate-verification#source:875b938aacd0886b84e74db7cde848f22b3d4fb28bdda61fd744d8beff1ce344#variant:0cc4445cfeaa6a2410808cc0069a3de14d0862bd15ef285e26e2d7ddf4fb3194
      variant_fingerprint: 0cc4445cfeaa6a2410808cc0069a3de14d0862bd15ef285e26e2d7ddf4fb3194
      raw_reference: echo-sdk-host/tests/facade_feature_adapters_e2e.rs#a2a_sse_stream_delivers_items_then_terminal
      source_role: test_consumer
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/tests/facade_feature_adapters_e2e.rs
      replacement_content_sha256: 875b938aacd0886b84e74db7cde848f22b3d4fb28bdda61fd744d8beff1ce344
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_67e01f190c33f59c
  ? evidence.sdk-parent-driver-candidate-verification#source:875b938aacd0886b84e74db7cde848f22b3d4fb28bdda61fd744d8beff1ce344#variant:0f94dc5fa830e2015de2a0560fc5f904b08fc86962a1f6ff76e6744ec13edf9b
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 0f94dc5fa830e2015de2a0560fc5f904b08fc86962a1f6ff76e6744ec13edf9b
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: 0578b3c92581c86030e5ae82c82cb3cb28b297e8189222a39b76d1a5e8c9064d
      variant_obligation: evidence.sdk-parent-driver-candidate-verification#source:875b938aacd0886b84e74db7cde848f22b3d4fb28bdda61fd744d8beff1ce344#variant:0f94dc5fa830e2015de2a0560fc5f904b08fc86962a1f6ff76e6744ec13edf9b
      variant_fingerprint: 0f94dc5fa830e2015de2a0560fc5f904b08fc86962a1f6ff76e6744ec13edf9b
      raw_reference: echo-sdk-host/tests/facade_feature_adapters_e2e.rs#agent_cascade_preserves_same_named_other_manager_and_releases_resources
      source_role: test_consumer
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/tests/facade_feature_adapters_e2e.rs
      replacement_content_sha256: 875b938aacd0886b84e74db7cde848f22b3d4fb28bdda61fd744d8beff1ce344
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_462591ab0055698f
  ? evidence.sdk-parent-driver-candidate-verification#source:875b938aacd0886b84e74db7cde848f22b3d4fb28bdda61fd744d8beff1ce344#variant:52e1cb736c7ef01506ac55ff26023ce1af659ca5a98f072aeb76f30c8577e018
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 52e1cb736c7ef01506ac55ff26023ce1af659ca5a98f072aeb76f30c8577e018
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: 0578b3c92581c86030e5ae82c82cb3cb28b297e8189222a39b76d1a5e8c9064d
      variant_obligation: evidence.sdk-parent-driver-candidate-verification#source:875b938aacd0886b84e74db7cde848f22b3d4fb28bdda61fd744d8beff1ce344#variant:52e1cb736c7ef01506ac55ff26023ce1af659ca5a98f072aeb76f30c8577e018
      variant_fingerprint: 52e1cb736c7ef01506ac55ff26023ce1af659ca5a98f072aeb76f30c8577e018
      raw_reference: echo-sdk-host/tests/facade_feature_adapters_e2e.rs#facade_resource_limits_match_the_advertised_bound
      source_role: test_consumer
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/tests/facade_feature_adapters_e2e.rs
      replacement_content_sha256: 875b938aacd0886b84e74db7cde848f22b3d4fb28bdda61fd744d8beff1ce344
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_9a7d81738c030930
  ? evidence.sdk-parent-driver-candidate-verification#source:875b938aacd0886b84e74db7cde848f22b3d4fb28bdda61fd744d8beff1ce344#variant:56fd775ed3d8c7228d97ad3b82ad4f050be5c0e6936d5175ffa6f0223e7fb4ca
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 56fd775ed3d8c7228d97ad3b82ad4f050be5c0e6936d5175ffa6f0223e7fb4ca
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: 0578b3c92581c86030e5ae82c82cb3cb28b297e8189222a39b76d1a5e8c9064d
      variant_obligation: evidence.sdk-parent-driver-candidate-verification#source:875b938aacd0886b84e74db7cde848f22b3d4fb28bdda61fd744d8beff1ce344#variant:56fd775ed3d8c7228d97ad3b82ad4f050be5c0e6936d5175ffa6f0223e7fb4ca
      variant_fingerprint: 56fd775ed3d8c7228d97ad3b82ad4f050be5c0e6936d5175ffa6f0223e7fb4ca
      raw_reference: echo-sdk-host/tests/facade_feature_adapters_e2e.rs#mcp_file_batch_quota_failure_does_not_publish_unreturned_prefix
      source_role: test_consumer
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/tests/facade_feature_adapters_e2e.rs
      replacement_content_sha256: 875b938aacd0886b84e74db7cde848f22b3d4fb28bdda61fd744d8beff1ce344
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_23dbad40f5238e16
  ? evidence.sdk-parent-driver-candidate-verification#source:875b938aacd0886b84e74db7cde848f22b3d4fb28bdda61fd744d8beff1ce344#variant:682eb4366716d0a89e8407f55de19481c8d30db14f63bf3b75d2feac606ceb4a
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 682eb4366716d0a89e8407f55de19481c8d30db14f63bf3b75d2feac606ceb4a
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: 0578b3c92581c86030e5ae82c82cb3cb28b297e8189222a39b76d1a5e8c9064d
      variant_obligation: evidence.sdk-parent-driver-candidate-verification#source:875b938aacd0886b84e74db7cde848f22b3d4fb28bdda61fd744d8beff1ce344#variant:682eb4366716d0a89e8407f55de19481c8d30db14f63bf3b75d2feac606ceb4a
      variant_fingerprint: 682eb4366716d0a89e8407f55de19481c8d30db14f63bf3b75d2feac606ceb4a
      raw_reference: echo-sdk-host/tests/facade_feature_adapters_e2e.rs#workflow_cancel_preserves_canonical_failure_in_rpc_and_stream_terminal
      source_role: test_consumer
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/tests/facade_feature_adapters_e2e.rs
      replacement_content_sha256: 875b938aacd0886b84e74db7cde848f22b3d4fb28bdda61fd744d8beff1ce344
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_6b4eeda0b9a382e2
  ? evidence.sdk-parent-driver-candidate-verification#source:875b938aacd0886b84e74db7cde848f22b3d4fb28bdda61fd744d8beff1ce344#variant:6e3b96c790f1fe78ee68e8bc28aa1341abeafb80cd750d71b52d8e9310f5bd50
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 6e3b96c790f1fe78ee68e8bc28aa1341abeafb80cd750d71b52d8e9310f5bd50
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: 0578b3c92581c86030e5ae82c82cb3cb28b297e8189222a39b76d1a5e8c9064d
      variant_obligation: evidence.sdk-parent-driver-candidate-verification#source:875b938aacd0886b84e74db7cde848f22b3d4fb28bdda61fd744d8beff1ce344#variant:6e3b96c790f1fe78ee68e8bc28aa1341abeafb80cd750d71b52d8e9310f5bd50
      variant_fingerprint: 6e3b96c790f1fe78ee68e8bc28aa1341abeafb80cd750d71b52d8e9310f5bd50
      raw_reference: echo-sdk-host/tests/facade_feature_adapters_e2e.rs#mcp_config_batch_quota_failure_does_not_publish_unreturned_prefix
      source_role: test_consumer
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/tests/facade_feature_adapters_e2e.rs
      replacement_content_sha256: 875b938aacd0886b84e74db7cde848f22b3d4fb28bdda61fd744d8beff1ce344
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_c689923ed7e0d976
  ? evidence.sdk-parent-driver-candidate-verification#source:875b938aacd0886b84e74db7cde848f22b3d4fb28bdda61fd744d8beff1ce344#variant:8082480f03f1a8a8ffd737e81ead5a476c23c62622d61a36989bd3bf9ca64811
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 8082480f03f1a8a8ffd737e81ead5a476c23c62622d61a36989bd3bf9ca64811
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: 0578b3c92581c86030e5ae82c82cb3cb28b297e8189222a39b76d1a5e8c9064d
      variant_obligation: evidence.sdk-parent-driver-candidate-verification#source:875b938aacd0886b84e74db7cde848f22b3d4fb28bdda61fd744d8beff1ce344#variant:8082480f03f1a8a8ffd737e81ead5a476c23c62622d61a36989bd3bf9ca64811
      variant_fingerprint: 8082480f03f1a8a8ffd737e81ead5a476c23c62622d61a36989bd3bf9ca64811
      raw_reference: echo-sdk-host/tests/facade_feature_adapters_e2e.rs#mcp_config_batch_preserves_best_effort_and_existing_alias
      source_role: test_consumer
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/tests/facade_feature_adapters_e2e.rs
      replacement_content_sha256: 875b938aacd0886b84e74db7cde848f22b3d4fb28bdda61fd744d8beff1ce344
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_857111e400a3d39e
  ? evidence.sdk-parent-driver-candidate-verification#source:875b938aacd0886b84e74db7cde848f22b3d4fb28bdda61fd744d8beff1ce344#variant:b6a24a52097b807c7336a96b459a55cdead304524958b6083897277a38980356
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: b6a24a52097b807c7336a96b459a55cdead304524958b6083897277a38980356
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: 0578b3c92581c86030e5ae82c82cb3cb28b297e8189222a39b76d1a5e8c9064d
      variant_obligation: evidence.sdk-parent-driver-candidate-verification#source:875b938aacd0886b84e74db7cde848f22b3d4fb28bdda61fd744d8beff1ce344#variant:b6a24a52097b807c7336a96b459a55cdead304524958b6083897277a38980356
      variant_fingerprint: b6a24a52097b807c7336a96b459a55cdead304524958b6083897277a38980356
      raw_reference: echo-sdk-host/tests/facade_feature_adapters_e2e.rs#workflow_stream_uses_unified_handle_backpressure_and_close
      source_role: test_consumer
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/tests/facade_feature_adapters_e2e.rs
      replacement_content_sha256: 875b938aacd0886b84e74db7cde848f22b3d4fb28bdda61fd744d8beff1ce344
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_209e4049f017f426
  ? evidence.sdk-parent-driver-candidate-verification#source:be870643edf05900800a083e09a678b367749060138c1295f2aecd4f46faf5fd#variant:1109eb7f4f851b3ff826dfc37f5d17a67ff5507c4c7bd385d29297775b1466e2
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 1109eb7f4f851b3ff826dfc37f5d17a67ff5507c4c7bd385d29297775b1466e2
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: dec849c54ecc04c6dc338704e1f7ef8ac85ac3c7f7fe580f4820906014cdd86f
      variant_obligation: evidence.sdk-parent-driver-candidate-verification#source:be870643edf05900800a083e09a678b367749060138c1295f2aecd4f46faf5fd#variant:1109eb7f4f851b3ff826dfc37f5d17a67ff5507c4c7bd385d29297775b1466e2
      variant_fingerprint: 1109eb7f4f851b3ff826dfc37f5d17a67ff5507c4c7bd385d29297775b1466e2
      raw_reference: echo-sdk-host/tests/extension_bridge_e2e.rs#channel_plugin_facade_round_trip
      source_role: test_consumer
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/tests/extension_bridge_e2e.rs
      replacement_content_sha256: be870643edf05900800a083e09a678b367749060138c1295f2aecd4f46faf5fd
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_fccab43ef21f91b4
  ? evidence.sdk-parent-driver-candidate-verification#source:be870643edf05900800a083e09a678b367749060138c1295f2aecd4f46faf5fd#variant:308bcbb5233a56cf88cbc038a438687cecd807994c9822a0d31a4084f63b3bac
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 308bcbb5233a56cf88cbc038a438687cecd807994c9822a0d31a4084f63b3bac
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: dec849c54ecc04c6dc338704e1f7ef8ac85ac3c7f7fe580f4820906014cdd86f
      variant_obligation: evidence.sdk-parent-driver-candidate-verification#source:be870643edf05900800a083e09a678b367749060138c1295f2aecd4f46faf5fd#variant:308bcbb5233a56cf88cbc038a438687cecd807994c9822a0d31a4084f63b3bac
      variant_fingerprint: 308bcbb5233a56cf88cbc038a438687cecd807994c9822a0d31a4084f63b3bac
      raw_reference: echo-sdk-host/tests/extension_bridge_e2e.rs#managed_persistence_deadline_and_lost_ack_preserve_commit_authority
      source_role: test_consumer
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/tests/extension_bridge_e2e.rs
      replacement_content_sha256: be870643edf05900800a083e09a678b367749060138c1295f2aecd4f46faf5fd
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_e7d864b86813ca15
  ? evidence.sdk-parent-driver-candidate-verification#source:be870643edf05900800a083e09a678b367749060138c1295f2aecd4f46faf5fd#variant:506c9cf44dde9ab5ac773ba639ac861bb5252edbe4fc3fcb6a35d8b476b19193
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 506c9cf44dde9ab5ac773ba639ac861bb5252edbe4fc3fcb6a35d8b476b19193
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: dec849c54ecc04c6dc338704e1f7ef8ac85ac3c7f7fe580f4820906014cdd86f
      variant_obligation: evidence.sdk-parent-driver-candidate-verification#source:be870643edf05900800a083e09a678b367749060138c1295f2aecd4f46faf5fd#variant:506c9cf44dde9ab5ac773ba639ac861bb5252edbe4fc3fcb6a35d8b476b19193
      variant_fingerprint: 506c9cf44dde9ab5ac773ba639ac861bb5252edbe4fc3fcb6a35d8b476b19193
      raw_reference: echo-sdk-host/tests/extension_bridge_e2e.rs#tool_bridge_round_trip_with_callbacks_intervention_and_hook
      source_role: test_consumer
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/tests/extension_bridge_e2e.rs
      replacement_content_sha256: be870643edf05900800a083e09a678b367749060138c1295f2aecd4f46faf5fd
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_aecee2c7cfc9dfc5
  ? evidence.sdk-parent-driver-candidate-verification#source:be870643edf05900800a083e09a678b367749060138c1295f2aecd4f46faf5fd#variant:5efbf96ee15b4c3f70a71a62191e9b1b4a001808bc57fdeeb4b660c6428aca40
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 5efbf96ee15b4c3f70a71a62191e9b1b4a001808bc57fdeeb4b660c6428aca40
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: dec849c54ecc04c6dc338704e1f7ef8ac85ac3c7f7fe580f4820906014cdd86f
      variant_obligation: evidence.sdk-parent-driver-candidate-verification#source:be870643edf05900800a083e09a678b367749060138c1295f2aecd4f46faf5fd#variant:5efbf96ee15b4c3f70a71a62191e9b1b4a001808bc57fdeeb4b660c6428aca40
      variant_fingerprint: 5efbf96ee15b4c3f70a71a62191e9b1b4a001808bc57fdeeb4b660c6428aca40
      raw_reference: echo-sdk-host/tests/extension_bridge_e2e.rs#same_factory_constructs_independent_instances_across_concurrent_sessions
      source_role: test_consumer
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/tests/extension_bridge_e2e.rs
      replacement_content_sha256: be870643edf05900800a083e09a678b367749060138c1295f2aecd4f46faf5fd
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_cdb727270d2ae7e2
  ? evidence.sdk-parent-driver-candidate-verification#source:be870643edf05900800a083e09a678b367749060138c1295f2aecd4f46faf5fd#variant:6d56b26512781e6476ee1b6ce353b74c74bce98c778b9c13b088bbe9ffe75c80
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 6d56b26512781e6476ee1b6ce353b74c74bce98c778b9c13b088bbe9ffe75c80
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: dec849c54ecc04c6dc338704e1f7ef8ac85ac3c7f7fe580f4820906014cdd86f
      variant_obligation: evidence.sdk-parent-driver-candidate-verification#source:be870643edf05900800a083e09a678b367749060138c1295f2aecd4f46faf5fd#variant:6d56b26512781e6476ee1b6ce353b74c74bce98c778b9c13b088bbe9ffe75c80
      variant_fingerprint: 6d56b26512781e6476ee1b6ce353b74c74bce98c778b9c13b088bbe9ffe75c80
      raw_reference: echo-sdk-host/tests/extension_bridge_e2e.rs#workflow_agent_component_stream_flows_through_facade_pull_stream
      source_role: test_consumer
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/tests/extension_bridge_e2e.rs
      replacement_content_sha256: be870643edf05900800a083e09a678b367749060138c1295f2aecd4f46faf5fd
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_9ca4c4215abe0743
  ? evidence.sdk-parent-driver-candidate-verification#source:be870643edf05900800a083e09a678b367749060138c1295f2aecd4f46faf5fd#variant:72aa13f7e5e71218b2cc2dc03f4dc161720aa546395e81d4a177b219104d9d43
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 72aa13f7e5e71218b2cc2dc03f4dc161720aa546395e81d4a177b219104d9d43
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: dec849c54ecc04c6dc338704e1f7ef8ac85ac3c7f7fe580f4820906014cdd86f
      variant_obligation: evidence.sdk-parent-driver-candidate-verification#source:be870643edf05900800a083e09a678b367749060138c1295f2aecd4f46faf5fd#variant:72aa13f7e5e71218b2cc2dc03f4dc161720aa546395e81d4a177b219104d9d43
      variant_fingerprint: 72aa13f7e5e71218b2cc2dc03f4dc161720aa546395e81d4a177b219104d9d43
      raw_reference: echo-sdk-host/tests/extension_bridge_e2e.rs#store_and_human_loop_extensions_round_trip
      source_role: test_consumer
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/tests/extension_bridge_e2e.rs
      replacement_content_sha256: be870643edf05900800a083e09a678b367749060138c1295f2aecd4f46faf5fd
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_df6498f4ae68ec43
  ? evidence.sdk-parent-driver-candidate-verification#source:be870643edf05900800a083e09a678b367749060138c1295f2aecd4f46faf5fd#variant:89a7ba1b444c5a2cf32725dccd2dfb2828cc1bfe4c9555dbf0491bc8f83aefc3
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 89a7ba1b444c5a2cf32725dccd2dfb2828cc1bfe4c9555dbf0491bc8f83aefc3
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: dec849c54ecc04c6dc338704e1f7ef8ac85ac3c7f7fe580f4820906014cdd86f
      variant_obligation: evidence.sdk-parent-driver-candidate-verification#source:be870643edf05900800a083e09a678b367749060138c1295f2aecd4f46faf5fd#variant:89a7ba1b444c5a2cf32725dccd2dfb2828cc1bfe4c9555dbf0491bc8f83aefc3
      variant_fingerprint: 89a7ba1b444c5a2cf32725dccd2dfb2828cc1bfe4c9555dbf0491bc8f83aefc3
      raw_reference: echo-sdk-host/tests/extension_bridge_e2e.rs#eval_and_improve_source_adapters_invoke_agent_factory_lazily
      source_role: test_consumer
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/tests/extension_bridge_e2e.rs
      replacement_content_sha256: be870643edf05900800a083e09a678b367749060138c1295f2aecd4f46faf5fd
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_00ccc58015381336
  ? evidence.sdk-parent-driver-candidate-verification#source:be870643edf05900800a083e09a678b367749060138c1295f2aecd4f46faf5fd#variant:b69a43f893bb4fb71b07b7a0c1ff308a95f99c77189526a8c8fb991fe4ff6490
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: b69a43f893bb4fb71b07b7a0c1ff308a95f99c77189526a8c8fb991fe4ff6490
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: dec849c54ecc04c6dc338704e1f7ef8ac85ac3c7f7fe580f4820906014cdd86f
      variant_obligation: evidence.sdk-parent-driver-candidate-verification#source:be870643edf05900800a083e09a678b367749060138c1295f2aecd4f46faf5fd#variant:b69a43f893bb4fb71b07b7a0c1ff308a95f99c77189526a8c8fb991fe4ff6490
      variant_fingerprint: b69a43f893bb4fb71b07b7a0c1ff308a95f99c77189526a8c8fb991fe4ff6490
      raw_reference: echo-sdk-host/tests/extension_bridge_e2e.rs#managed_commit_disconnect_restart_replays_same_receipt
      source_role: test_consumer
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/tests/extension_bridge_e2e.rs
      replacement_content_sha256: be870643edf05900800a083e09a678b367749060138c1295f2aecd4f46faf5fd
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_f6581f7c6ef72e84
  ? evidence.sdk-parent-driver-candidate-verification#source:be870643edf05900800a083e09a678b367749060138c1295f2aecd4f46faf5fd#variant:cabbf9b61a8ffdf5deed5158c192497184fc0254c5625e5bda5dcdd951c5ebae
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: cabbf9b61a8ffdf5deed5158c192497184fc0254c5625e5bda5dcdd951c5ebae
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: dec849c54ecc04c6dc338704e1f7ef8ac85ac3c7f7fe580f4820906014cdd86f
      variant_obligation: evidence.sdk-parent-driver-candidate-verification#source:be870643edf05900800a083e09a678b367749060138c1295f2aecd4f46faf5fd#variant:cabbf9b61a8ffdf5deed5158c192497184fc0254c5625e5bda5dcdd951c5ebae
      variant_fingerprint: cabbf9b61a8ffdf5deed5158c192497184fc0254c5625e5bda5dcdd951c5ebae
      raw_reference: echo-sdk-host/tests/extension_bridge_e2e.rs#sandbox_cancel_bridge_waits_for_cleanup_and_preserves_cancelled_terminal
      source_role: test_consumer
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/tests/extension_bridge_e2e.rs
      replacement_content_sha256: be870643edf05900800a083e09a678b367749060138c1295f2aecd4f46faf5fd
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_7bc2017dc626e9f3
  ? evidence.sdk-parent-driver-candidate-verification#source:be870643edf05900800a083e09a678b367749060138c1295f2aecd4f46faf5fd#variant:dda386bfbffa1c4e8c52e84a31cce4dcb69c42546a51c387a8f7848eabe2c6a5
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: dda386bfbffa1c4e8c52e84a31cce4dcb69c42546a51c387a8f7848eabe2c6a5
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: dec849c54ecc04c6dc338704e1f7ef8ac85ac3c7f7fe580f4820906014cdd86f
      variant_obligation: evidence.sdk-parent-driver-candidate-verification#source:be870643edf05900800a083e09a678b367749060138c1295f2aecd4f46faf5fd#variant:dda386bfbffa1c4e8c52e84a31cce4dcb69c42546a51c387a8f7848eabe2c6a5
      variant_fingerprint: dda386bfbffa1c4e8c52e84a31cce4dcb69c42546a51c387a8f7848eabe2c6a5
      raw_reference: echo-sdk-host/tests/extension_bridge_e2e.rs#revisioned_task_extension_is_shared_by_crud_and_execution
      source_role: test_consumer
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/tests/extension_bridge_e2e.rs
      replacement_content_sha256: be870643edf05900800a083e09a678b367749060138c1295f2aecd4f46faf5fd
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_7c7f4efcd7af173c
  ? evidence.sdk-parent-driver-candidate-verification#source:be870643edf05900800a083e09a678b367749060138c1295f2aecd4f46faf5fd#variant:f4fbc6eb96d66308e4df370195194bdebcf803632a13ddd0e8e94e832511175c
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: f4fbc6eb96d66308e4df370195194bdebcf803632a13ddd0e8e94e832511175c
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: dec849c54ecc04c6dc338704e1f7ef8ac85ac3c7f7fe580f4820906014cdd86f
      variant_obligation: evidence.sdk-parent-driver-candidate-verification#source:be870643edf05900800a083e09a678b367749060138c1295f2aecd4f46faf5fd#variant:f4fbc6eb96d66308e4df370195194bdebcf803632a13ddd0e8e94e832511175c
      variant_fingerprint: f4fbc6eb96d66308e4df370195194bdebcf803632a13ddd0e8e94e832511175c
      raw_reference: echo-sdk-host/tests/extension_bridge_e2e.rs#crud_only_task_store_rejects_execute_before_runtime_side_effects
      source_role: test_consumer
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/tests/extension_bridge_e2e.rs
      replacement_content_sha256: be870643edf05900800a083e09a678b367749060138c1295f2aecd4f46faf5fd
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_cd6558db4215c96c
  ? evidence.sdk-parent-driver-candidate-verification#source:be870643edf05900800a083e09a678b367749060138c1295f2aecd4f46faf5fd#variant:f8a4176c6651f97bff7dc76af86bcc388d54886fdce8d61cf1e204323572da4f
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: f8a4176c6651f97bff7dc76af86bcc388d54886fdce8d61cf1e204323572da4f
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: dec849c54ecc04c6dc338704e1f7ef8ac85ac3c7f7fe580f4820906014cdd86f
      variant_obligation: evidence.sdk-parent-driver-candidate-verification#source:be870643edf05900800a083e09a678b367749060138c1295f2aecd4f46faf5fd#variant:f8a4176c6651f97bff7dc76af86bcc388d54886fdce8d61cf1e204323572da4f
      variant_fingerprint: f8a4176c6651f97bff7dc76af86bcc388d54886fdce8d61cf1e204323572da4f
      raw_reference: echo-sdk-host/tests/extension_bridge_e2e.rs#cancelled_initial_stream_response_releases_session_callback_owner
      source_role: test_consumer
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/tests/extension_bridge_e2e.rs
      replacement_content_sha256: be870643edf05900800a083e09a678b367749060138c1295f2aecd4f46faf5fd
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_255b426f31cd441e
  ? evidence.sdk-parent-driver-candidate-verification#source:e4ae808abb8f916a7318faec3643c5a56cd1bbd143e9e88cb447e0cf7127f7de#variant:0a7c55bec37e4953778476f332b5fd1243f27be25a718b99506bc17a8eff1c7e
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 0a7c55bec37e4953778476f332b5fd1243f27be25a718b99506bc17a8eff1c7e
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: 957c0ccb4346784246db4541513816ad080c7ace0055afe430d93c01407623a3
      variant_obligation: evidence.sdk-parent-driver-candidate-verification#source:e4ae808abb8f916a7318faec3643c5a56cd1bbd143e9e88cb447e0cf7127f7de#variant:0a7c55bec37e4953778476f332b5fd1243f27be25a718b99506bc17a8eff1c7e
      variant_fingerprint: 0a7c55bec37e4953778476f332b5fd1243f27be25a718b99506bc17a8eff1c7e
      raw_reference: echo-sdk-host/src/core_profile/events.rs#real_journal_retention_page_authorizes_only_its_returned_watermark
      source_role: production_source
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/src/core_profile/events.rs
      replacement_content_sha256: e4ae808abb8f916a7318faec3643c5a56cd1bbd143e9e88cb447e0cf7127f7de
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_995f9c10ecb1e456
  ? evidence.sdk-parent-driver-candidate-verification#source:e4ae808abb8f916a7318faec3643c5a56cd1bbd143e9e88cb447e0cf7127f7de#variant:6754ad9cdc4c7cbbe2982fa566d27dcdc3a999f73bd9e5120178875d90aaa9d2
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 6754ad9cdc4c7cbbe2982fa566d27dcdc3a999f73bd9e5120178875d90aaa9d2
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: 957c0ccb4346784246db4541513816ad080c7ace0055afe430d93c01407623a3
      variant_obligation: evidence.sdk-parent-driver-candidate-verification#source:e4ae808abb8f916a7318faec3643c5a56cd1bbd143e9e88cb447e0cf7127f7de#variant:6754ad9cdc4c7cbbe2982fa566d27dcdc3a999f73bd9e5120178875d90aaa9d2
      variant_fingerprint: 6754ad9cdc4c7cbbe2982fa566d27dcdc3a999f73bd9e5120178875d90aaa9d2
      raw_reference: echo-sdk-host/src/core_profile/events.rs#replay_publication_failure_and_empty_future_page_grant_no_ack_coverage
      source_role: production_source
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/src/core_profile/events.rs
      replacement_content_sha256: e4ae808abb8f916a7318faec3643c5a56cd1bbd143e9e88cb447e0cf7127f7de
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_a083c58c718849f3
  ? evidence.sdk-protocol-finite-observation#source:3e8eaabc720704de6dc28b51550369fdfaa8387a5d238a27a19d220d00f43ad2#variant:9f7dea2077ca2e745f2647041ec74756e56ce3799d15c03312d5bd2ebf3c095c
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 9f7dea2077ca2e745f2647041ec74756e56ce3799d15c03312d5bd2ebf3c095c
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: 88683040667493e72ff3e1d91cc0dd7d14ed23ea6e4f70f27907bb140241429b
      variant_obligation: evidence.sdk-protocol-finite-observation#source:3e8eaabc720704de6dc28b51550369fdfaa8387a5d238a27a19d220d00f43ad2#variant:9f7dea2077ca2e745f2647041ec74756e56ce3799d15c03312d5bd2ebf3c095c
      variant_fingerprint: 9f7dea2077ca2e745f2647041ec74756e56ce3799d15c03312d5bd2ebf3c095c
      raw_reference: echo-sdk-protocol/src/inventory.rs#new_public_invoke_is_inventory_not_an_accepted_language_promise
      source_role: production_source
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-protocol/src/inventory.rs
      replacement_content_sha256: 3e8eaabc720704de6dc28b51550369fdfaa8387a5d238a27a19d220d00f43ad2
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_c97ee0b681ffadf5
  ? evidence.sdk-protocol-finite-observation#source:3e8eaabc720704de6dc28b51550369fdfaa8387a5d238a27a19d220d00f43ad2#variant:ff33b992553ee71b288d3165ab42a80ca50ea738f0df7f2b22796c747f865364
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: ff33b992553ee71b288d3165ab42a80ca50ea738f0df7f2b22796c747f865364
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: 88683040667493e72ff3e1d91cc0dd7d14ed23ea6e4f70f27907bb140241429b
      variant_obligation: evidence.sdk-protocol-finite-observation#source:3e8eaabc720704de6dc28b51550369fdfaa8387a5d238a27a19d220d00f43ad2#variant:ff33b992553ee71b288d3165ab42a80ca50ea738f0df7f2b22796c747f865364
      variant_fingerprint: ff33b992553ee71b288d3165ab42a80ca50ea738f0df7f2b22796c747f865364
      raw_reference: echo-sdk-protocol/src/inventory.rs
      source_role: production_source
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-protocol/src/inventory.rs
      replacement_content_sha256: 3e8eaabc720704de6dc28b51550369fdfaa8387a5d238a27a19d220d00f43ad2
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_02934c5be4bc81a2
  ? evidence.sdk-protocol-finite-observation#source:569ea658c267d505d0c20369dbdb384eb2022ea12e0bb6fabb70c398cc13ca0c#variant:2eb5dc5afe91eb697b015f763409e3fabd345f21e428f32d724096156c6f49ac
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 2eb5dc5afe91eb697b015f763409e3fabd345f21e428f32d724096156c6f49ac
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: c54b3a4ab67cfe9b5e5c019f6d4a8bd730ef3463966e1405f8c051944eb73958
      variant_obligation: evidence.sdk-protocol-finite-observation#source:569ea658c267d505d0c20369dbdb384eb2022ea12e0bb6fabb70c398cc13ca0c#variant:2eb5dc5afe91eb697b015f763409e3fabd345f21e428f32d724096156c6f49ac
      variant_fingerprint: 2eb5dc5afe91eb697b015f763409e3fabd345f21e428f32d724096156c6f49ac
      raw_reference: echo-sdk-protocol/src/bin/export_schema.rs#acceptance_dispositions_reject_draft_wrong_action_and_identity_substrings
      source_role: production_source
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-protocol/src/bin/export_schema.rs
      replacement_content_sha256: 569ea658c267d505d0c20369dbdb384eb2022ea12e0bb6fabb70c398cc13ca0c
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_d7c8f1b0b2454e51
  ? evidence.sdk-protocol-finite-observation#source:569ea658c267d505d0c20369dbdb384eb2022ea12e0bb6fabb70c398cc13ca0c#variant:39f2f4ed7d68b86f1bf297355489df9022efeb432bdecb895cccf8df17bc10c2
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 39f2f4ed7d68b86f1bf297355489df9022efeb432bdecb895cccf8df17bc10c2
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: c54b3a4ab67cfe9b5e5c019f6d4a8bd730ef3463966e1405f8c051944eb73958
      variant_obligation: evidence.sdk-protocol-finite-observation#source:569ea658c267d505d0c20369dbdb384eb2022ea12e0bb6fabb70c398cc13ca0c#variant:39f2f4ed7d68b86f1bf297355489df9022efeb432bdecb895cccf8df17bc10c2
      variant_fingerprint: 39f2f4ed7d68b86f1bf297355489df9022efeb432bdecb895cccf8df17bc10c2
      raw_reference: echo-sdk-protocol/src/bin/export_schema.rs#accepted_catalog_ignores_reexport_alias_inventory_growth
      source_role: production_source
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-protocol/src/bin/export_schema.rs
      replacement_content_sha256: 569ea658c267d505d0c20369dbdb384eb2022ea12e0bb6fabb70c398cc13ca0c
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_c186c2ec854cc5cb
  ? evidence.sdk-protocol-finite-observation#source:569ea658c267d505d0c20369dbdb384eb2022ea12e0bb6fabb70c398cc13ca0c#variant:e35ea8dcfdad5bda81d4e8d127ff0327292044d81a0ee516c8df1ec1698253fb
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: e35ea8dcfdad5bda81d4e8d127ff0327292044d81a0ee516c8df1ec1698253fb
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: c54b3a4ab67cfe9b5e5c019f6d4a8bd730ef3463966e1405f8c051944eb73958
      variant_obligation: evidence.sdk-protocol-finite-observation#source:569ea658c267d505d0c20369dbdb384eb2022ea12e0bb6fabb70c398cc13ca0c#variant:e35ea8dcfdad5bda81d4e8d127ff0327292044d81a0ee516c8df1ec1698253fb
      variant_fingerprint: e35ea8dcfdad5bda81d4e8d127ff0327292044d81a0ee516c8df1ec1698253fb
      raw_reference: echo-sdk-protocol/src/bin/export_schema.rs
      source_role: production_source
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-protocol/src/bin/export_schema.rs
      replacement_content_sha256: 569ea658c267d505d0c20369dbdb384eb2022ea12e0bb6fabb70c398cc13ca0c
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_ddd9169ce90ef851
  ? evidence.sdk-protocol-finite-observation#source:569ea658c267d505d0c20369dbdb384eb2022ea12e0bb6fabb70c398cc13ca0c#variant:eed5ddac93dae11ec1035eb6e31afc97eacbbc72aadcbe69ddfbe57783e727fd
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: eed5ddac93dae11ec1035eb6e31afc97eacbbc72aadcbe69ddfbe57783e727fd
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: c54b3a4ab67cfe9b5e5c019f6d4a8bd730ef3463966e1405f8c051944eb73958
      variant_obligation: evidence.sdk-protocol-finite-observation#source:569ea658c267d505d0c20369dbdb384eb2022ea12e0bb6fabb70c398cc13ca0c#variant:eed5ddac93dae11ec1035eb6e31afc97eacbbc72aadcbe69ddfbe57783e727fd
      variant_fingerprint: eed5ddac93dae11ec1035eb6e31afc97eacbbc72aadcbe69ddfbe57783e727fd
      raw_reference: echo-sdk-protocol/src/bin/export_schema.rs#new_source_inventory_route_does_not_change_accepted_catalog_or_digest
      source_role: production_source
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-protocol/src/bin/export_schema.rs
      replacement_content_sha256: 569ea658c267d505d0c20369dbdb384eb2022ea12e0bb6fabb70c398cc13ca0c
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_e34b8314e21c0086
  ? evidence.sdk-protocol-finite-observation#source:8332f6ccc7be4a2ce1a0143c31bfcf9df4b89e015dc8cf64cc2f1927bca9b92b#variant:72a6c7785a5d24a23f7727f7e912c7d71310fe6455761acd7b6933df3d93db24
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 72a6c7785a5d24a23f7727f7e912c7d71310fe6455761acd7b6933df3d93db24
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: c66a65037a863e28b7aa7e03b9d18b81bfd6944b092b10a054a96a9d9d38d83e
      variant_obligation: evidence.sdk-protocol-finite-observation#source:8332f6ccc7be4a2ce1a0143c31bfcf9df4b89e015dc8cf64cc2f1927bca9b92b#variant:72a6c7785a5d24a23f7727f7e912c7d71310fe6455761acd7b6933df3d93db24
      variant_fingerprint: 72a6c7785a5d24a23f7727f7e912c7d71310fe6455761acd7b6933df3d93db24
      raw_reference: echo-sdk-protocol/src/scalar.rs
      source_role: production_source
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-protocol/src/scalar.rs
      replacement_content_sha256: 8332f6ccc7be4a2ce1a0143c31bfcf9df4b89e015dc8cf64cc2f1927bca9b92b
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_e83fd92acdd25398
  ? evidence.sdk-protocol-finite-observation#source:8332f6ccc7be4a2ce1a0143c31bfcf9df4b89e015dc8cf64cc2f1927bca9b92b#variant:94900c6f5ec7e7b7ad7c713f9452b03ad924b495a1c196202dc8e2e9d04f128c
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 94900c6f5ec7e7b7ad7c713f9452b03ad924b495a1c196202dc8e2e9d04f128c
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: c66a65037a863e28b7aa7e03b9d18b81bfd6944b092b10a054a96a9d9d38d83e
      variant_obligation: evidence.sdk-protocol-finite-observation#source:8332f6ccc7be4a2ce1a0143c31bfcf9df4b89e015dc8cf64cc2f1927bca9b92b#variant:94900c6f5ec7e7b7ad7c713f9452b03ad924b495a1c196202dc8e2e9d04f128c
      variant_fingerprint: 94900c6f5ec7e7b7ad7c713f9452b03ad924b495a1c196202dc8e2e9d04f128c
      raw_reference: docs/sdk/governance/evidence/negotiation-response-validation/contract-basis/echo-sdk-protocol/src/scalar.rs
      source_role: documentation
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: docs/sdk/governance/semantic-history.md
      replacement_content_sha256: c37cc4ea8183ca53ea1a617bda46e8bf1f5f86617021c71a1506e075b35237b3
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_812a1ff5a8430c59
  ? evidence.sdk-protocol-finite-observation#source:c683f51728c241be24d1d3e4bc0d4448b7dd3fd10541f63a50238d753c8ce995#variant:b8c5d0ce6ae6ce844741ad1c377835938a83a97c0c5836d8b044ef409b7edea7
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: b8c5d0ce6ae6ce844741ad1c377835938a83a97c0c5836d8b044ef409b7edea7
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: eab6133c988cbe7560503cbdeb259b2c6eb6dc8a77e01f95b4f4178691b10a95
      variant_obligation: evidence.sdk-protocol-finite-observation#source:c683f51728c241be24d1d3e4bc0d4448b7dd3fd10541f63a50238d753c8ce995#variant:b8c5d0ce6ae6ce844741ad1c377835938a83a97c0c5836d8b044ef409b7edea7
      variant_fingerprint: b8c5d0ce6ae6ce844741ad1c377835938a83a97c0c5836d8b044ef409b7edea7
      raw_reference: echo-sdk-protocol/src/capability.rs
      source_role: production_source
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-protocol/src/capability.rs
      replacement_content_sha256: c683f51728c241be24d1d3e4bc0d4448b7dd3fd10541f63a50238d753c8ce995
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_9530e1c2b1c1085f
  ? evidence.sdk-protocol-finite-observation#source:c683f51728c241be24d1d3e4bc0d4448b7dd3fd10541f63a50238d753c8ce995#variant:df31ac2a9f3e14b35114cc3a1aafe235a9d259f5d34b8e49905e7802c64128b3
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: df31ac2a9f3e14b35114cc3a1aafe235a9d259f5d34b8e49905e7802c64128b3
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: eab6133c988cbe7560503cbdeb259b2c6eb6dc8a77e01f95b4f4178691b10a95
      variant_obligation: evidence.sdk-protocol-finite-observation#source:c683f51728c241be24d1d3e4bc0d4448b7dd3fd10541f63a50238d753c8ce995#variant:df31ac2a9f3e14b35114cc3a1aafe235a9d259f5d34b8e49905e7802c64128b3
      variant_fingerprint: df31ac2a9f3e14b35114cc3a1aafe235a9d259f5d34b8e49905e7802c64128b3
      raw_reference: docs/sdk/governance/evidence/negotiation-response-validation/contract-basis/echo-sdk-protocol/src/capability.rs
      source_role: documentation
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: docs/sdk/governance/semantic-history.md
      replacement_content_sha256: c37cc4ea8183ca53ea1a617bda46e8bf1f5f86617021c71a1506e075b35237b3
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_fb0841d4779b703e
  ? evidence.sdk-sandbox-natural-eof-verification#source:95c3b306c857efd65bd315029dab7a5f346f0c7926201778ed70043042d26628#variant:33d3fdf18ade843dd44ca57875af862357168fdc7efe14a441e6153a5dda4e3b
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 33d3fdf18ade843dd44ca57875af862357168fdc7efe14a441e6153a5dda4e3b
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: 1331676438bc771a711a72fd6c4a814dd21cde00c6f9d1ed6b5d63a0479efc99
      variant_obligation: evidence.sdk-sandbox-natural-eof-verification#source:95c3b306c857efd65bd315029dab7a5f346f0c7926201778ed70043042d26628#variant:33d3fdf18ade843dd44ca57875af862357168fdc7efe14a441e6153a5dda4e3b
      variant_fingerprint: 33d3fdf18ade843dd44ca57875af862357168fdc7efe14a441e6153a5dda4e3b
      raw_reference: echo-sdk-host/src/core_profile/facade/mod.rs#async fn drive_sandbox_facade_source
      source_role: production_source
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/src/core_profile/facade/mod.rs
      replacement_content_sha256: 95c3b306c857efd65bd315029dab7a5f346f0c7926201778ed70043042d26628
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_78ffedb156754623
  ? evidence.sdk-sandbox-natural-eof-verification#source:95c3b306c857efd65bd315029dab7a5f346f0c7926201778ed70043042d26628#variant:999cee6191444081fce3b676662cf0e55689474c146fc0483d4a54cdc0d51559
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 999cee6191444081fce3b676662cf0e55689474c146fc0483d4a54cdc0d51559
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: 1331676438bc771a711a72fd6c4a814dd21cde00c6f9d1ed6b5d63a0479efc99
      variant_obligation: evidence.sdk-sandbox-natural-eof-verification#source:95c3b306c857efd65bd315029dab7a5f346f0c7926201778ed70043042d26628#variant:999cee6191444081fce3b676662cf0e55689474c146fc0483d4a54cdc0d51559
      variant_fingerprint: 999cee6191444081fce3b676662cf0e55689474c146fc0483d4a54cdc0d51559
      raw_reference: echo-sdk-host/src/core_profile/facade/mod.rs#sandbox_natural_eof_preserves_queued_complete_without_cleanup_cancellation
      source_role: production_source
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/src/core_profile/facade/mod.rs
      replacement_content_sha256: 95c3b306c857efd65bd315029dab7a5f346f0c7926201778ed70043042d26628
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_f5766f2b634281f2
  ? evidence.sdk-task-memory-workflow-finite-observation#source:1532ab8b88dc82af92f15ea334ea1cbe64cf7c8aa7248f9315b3387eeda4fa0f#variant:085b018de03c562d7e63dd15dcadcb7f05aeb4faabc42299fec63dc48e69741c
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 085b018de03c562d7e63dd15dcadcb7f05aeb4faabc42299fec63dc48e69741c
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: 405078bb70b2930a0a96b97a4e7af335a1e622ecda892b90477e98abc492dd9b
      variant_obligation: evidence.sdk-task-memory-workflow-finite-observation#source:1532ab8b88dc82af92f15ea334ea1cbe64cf7c8aa7248f9315b3387eeda4fa0f#variant:085b018de03c562d7e63dd15dcadcb7f05aeb4faabc42299fec63dc48e69741c
      variant_fingerprint: 085b018de03c562d7e63dd15dcadcb7f05aeb4faabc42299fec63dc48e69741c
      raw_reference: echo-sdk-host/src/core_profile/facade/memory.rs#explicit_invalid_limits_are_not_defaulted
      source_role: production_source
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/src/core_profile/facade/memory.rs
      replacement_content_sha256: 1532ab8b88dc82af92f15ea334ea1cbe64cf7c8aa7248f9315b3387eeda4fa0f
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_c18a93e5d9578871
  ? evidence.sdk-task-memory-workflow-finite-observation#source:1532ab8b88dc82af92f15ea334ea1cbe64cf7c8aa7248f9315b3387eeda4fa0f#variant:5d13663fc21caafa84581d3978588a95ff9a9cc831b9f5765264b0c18da5684c
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 5d13663fc21caafa84581d3978588a95ff9a9cc831b9f5765264b0c18da5684c
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: 405078bb70b2930a0a96b97a4e7af335a1e622ecda892b90477e98abc492dd9b
      variant_obligation: evidence.sdk-task-memory-workflow-finite-observation#source:1532ab8b88dc82af92f15ea334ea1cbe64cf7c8aa7248f9315b3387eeda4fa0f#variant:5d13663fc21caafa84581d3978588a95ff9a9cc831b9f5765264b0c18da5684c
      variant_fingerprint: 5d13663fc21caafa84581d3978588a95ff9a9cc831b9f5765264b0c18da5684c
      raw_reference: echo-sdk-host/src/core_profile/facade/memory.rs#malformed_namespace_never_writes_and_replacement_store_is_live_authority
      source_role: production_source
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/src/core_profile/facade/memory.rs
      replacement_content_sha256: 1532ab8b88dc82af92f15ea334ea1cbe64cf7c8aa7248f9315b3387eeda4fa0f
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_1754e5cfae7c1382
  ? evidence.sdk-task-memory-workflow-finite-observation#source:1532ab8b88dc82af92f15ea334ea1cbe64cf7c8aa7248f9315b3387eeda4fa0f#variant:9dde33618b4641a89d0505bbcc9df895b895a5fe298e93d5d714a5728eb3fc44
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 9dde33618b4641a89d0505bbcc9df895b895a5fe298e93d5d714a5728eb3fc44
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: 405078bb70b2930a0a96b97a4e7af335a1e622ecda892b90477e98abc492dd9b
      variant_obligation: evidence.sdk-task-memory-workflow-finite-observation#source:1532ab8b88dc82af92f15ea334ea1cbe64cf7c8aa7248f9315b3387eeda4fa0f#variant:9dde33618b4641a89d0505bbcc9df895b895a5fe298e93d5d714a5728eb3fc44
      variant_fingerprint: 9dde33618b4641a89d0505bbcc9df895b895a5fe298e93d5d714a5728eb3fc44
      raw_reference: echo-sdk-host/src/core_profile/facade/memory.rs
      source_role: production_source
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/src/core_profile/facade/memory.rs
      replacement_content_sha256: 1532ab8b88dc82af92f15ea334ea1cbe64cf7c8aa7248f9315b3387eeda4fa0f
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_eb0746f502750ba0
  ? evidence.sdk-task-memory-workflow-finite-observation#source:2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c#variant:0ae91a6b0278815e1d61f39ac73e1f437ad742aa59739413e889dc0b5967b299
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 0ae91a6b0278815e1d61f39ac73e1f437ad742aa59739413e889dc0b5967b299
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: a946cd85dd468ea50adb617941caae53fec45b12d70e8f7fa4ca3c61b5b4d665
      variant_obligation: evidence.sdk-task-memory-workflow-finite-observation#source:2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c#variant:0ae91a6b0278815e1d61f39ac73e1f437ad742aa59739413e889dc0b5967b299
      variant_fingerprint: 0ae91a6b0278815e1d61f39ac73e1f437ad742aa59739413e889dc0b5967b299
      raw_reference: echo-sdk-host/tests/core_profile_e2e.rs#task_pause_is_exact_and_resume_reuses_canonical_dependency_result
      source_role: test_consumer
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/tests/core_profile_e2e.rs
      replacement_content_sha256: 2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_42cdb11cfeae106e
  ? evidence.sdk-task-memory-workflow-finite-observation#source:2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c#variant:16ddaec3345c89bc9cb6eba76a91ac8e50640552f2f825055124c9690647b927
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 16ddaec3345c89bc9cb6eba76a91ac8e50640552f2f825055124c9690647b927
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: a946cd85dd468ea50adb617941caae53fec45b12d70e8f7fa4ca3c61b5b4d665
      variant_obligation: evidence.sdk-task-memory-workflow-finite-observation#source:2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c#variant:16ddaec3345c89bc9cb6eba76a91ac8e50640552f2f825055124c9690647b927
      variant_fingerprint: 16ddaec3345c89bc9cb6eba76a91ac8e50640552f2f825055124c9690647b927
      raw_reference: echo-sdk-host/tests/core_profile_e2e.rs#task_execute_and_control_settle_through_the_runtime
      source_role: test_consumer
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/tests/core_profile_e2e.rs
      replacement_content_sha256: 2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_ce9066a7af5fc2c9
  ? evidence.sdk-task-memory-workflow-finite-observation#source:2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c#variant:2a12d9ed67d58da9e25eac8726797042c352c42063fd0abdeaa4a2aec90c6bcb
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 2a12d9ed67d58da9e25eac8726797042c352c42063fd0abdeaa4a2aec90c6bcb
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: a946cd85dd468ea50adb617941caae53fec45b12d70e8f7fa4ca3c61b5b4d665
      variant_obligation: evidence.sdk-task-memory-workflow-finite-observation#source:2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c#variant:2a12d9ed67d58da9e25eac8726797042c352c42063fd0abdeaa4a2aec90c6bcb
      variant_fingerprint: 2a12d9ed67d58da9e25eac8726797042c352c42063fd0abdeaa4a2aec90c6bcb
      raw_reference: echo-sdk-host/tests/core_profile_e2e.rs#task_cancel_is_exact_and_pending_sibling_continues
      source_role: test_consumer
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/tests/core_profile_e2e.rs
      replacement_content_sha256: 2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_0f39e93e92e54f80
  ? evidence.sdk-task-memory-workflow-finite-observation#source:2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c#variant:3ec77053fcb62122a0fa06019d4b050588cd125eb78800c6687f0fd1d1026c3e
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 3ec77053fcb62122a0fa06019d4b050588cd125eb78800c6687f0fd1d1026c3e
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: a946cd85dd468ea50adb617941caae53fec45b12d70e8f7fa4ca3c61b5b4d665
      variant_obligation: evidence.sdk-task-memory-workflow-finite-observation#source:2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c#variant:3ec77053fcb62122a0fa06019d4b050588cd125eb78800c6687f0fd1d1026c3e
      variant_fingerprint: 3ec77053fcb62122a0fa06019d4b050588cd125eb78800c6687f0fd1d1026c3e
      raw_reference: echo-sdk-host/tests/core_profile_e2e.rs#task_rpc_shares_the_session_task_authority
      source_role: test_consumer
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/tests/core_profile_e2e.rs
      replacement_content_sha256: 2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_3dbfe9fb272692d5
  ? evidence.sdk-task-memory-workflow-finite-observation#source:2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c#variant:9df83e3252394522936e5f0e5860c991650ee7bf5566e261cb6caf839ad208d3
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 9df83e3252394522936e5f0e5860c991650ee7bf5566e261cb6caf839ad208d3
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: a946cd85dd468ea50adb617941caae53fec45b12d70e8f7fa4ca3c61b5b4d665
      variant_obligation: evidence.sdk-task-memory-workflow-finite-observation#source:2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c#variant:9df83e3252394522936e5f0e5860c991650ee7bf5566e261cb6caf839ad208d3
      variant_fingerprint: 9df83e3252394522936e5f0e5860c991650ee7bf5566e261cb6caf839ad208d3
      raw_reference: echo-sdk-host/tests/core_profile_e2e.rs#subagent_short_await_preserves_result_and_dispatch_receipt
      source_role: test_consumer
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/tests/core_profile_e2e.rs
      replacement_content_sha256: 2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_3ab50437bfc642d2
  ? evidence.sdk-task-memory-workflow-finite-observation#source:2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c#variant:bf87ecd9b11a0f17b62f10c8487157c13f79be24d6d97802dfc3647cb1a6c9a2
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: bf87ecd9b11a0f17b62f10c8487157c13f79be24d6d97802dfc3647cb1a6c9a2
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: a946cd85dd468ea50adb617941caae53fec45b12d70e8f7fa4ca3c61b5b4d665
      variant_obligation: evidence.sdk-task-memory-workflow-finite-observation#source:2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c#variant:bf87ecd9b11a0f17b62f10c8487157c13f79be24d6d97802dfc3647cb1a6c9a2
      variant_fingerprint: bf87ecd9b11a0f17b62f10c8487157c13f79be24d6d97802dfc3647cb1a6c9a2
      raw_reference: echo-sdk-host/tests/core_profile_e2e.rs
      source_role: test_consumer
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/tests/core_profile_e2e.rs
      replacement_content_sha256: 2689f92761b5ca2388d8c986750b7f2b70b10e5dec1b1ca9d9c500da33abd26c
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_118e4d31e754097e
  ? evidence.sdk-task-memory-workflow-finite-observation#source:7c7a9d698eb9e1ecfc41cd3e78dac27b0a3f02b5650f45b9464ea596f2b55b5d#variant:45e8c0cebfe0a66787f7aba5831e644217ebc247c857e6d03eab0587725f39ab
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 45e8c0cebfe0a66787f7aba5831e644217ebc247c857e6d03eab0587725f39ab
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: 6f7411bd8af8c8c12429346fa83dfc5433ad797e7c8765da9ed30b4204dc0c69
      variant_obligation: evidence.sdk-task-memory-workflow-finite-observation#source:7c7a9d698eb9e1ecfc41cd3e78dac27b0a3f02b5650f45b9464ea596f2b55b5d#variant:45e8c0cebfe0a66787f7aba5831e644217ebc247c857e6d03eab0587725f39ab
      variant_fingerprint: 45e8c0cebfe0a66787f7aba5831e644217ebc247c857e6d03eab0587725f39ab
      raw_reference: echo-sdk-host/src/core_profile/facade/subagent.rs#collect_result
      source_role: production_source
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/src/core_profile/facade/subagent.rs
      replacement_content_sha256: 7c7a9d698eb9e1ecfc41cd3e78dac27b0a3f02b5650f45b9464ea596f2b55b5d
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_00e8ab51e6f7e73d
  ? evidence.sdk-task-memory-workflow-finite-observation#source:7c7a9d698eb9e1ecfc41cd3e78dac27b0a3f02b5650f45b9464ea596f2b55b5d#variant:a8fbf4c9068289322b22b407bba7c28f3465cb271d5be28e465238c81455e2df
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: a8fbf4c9068289322b22b407bba7c28f3465cb271d5be28e465238c81455e2df
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: 6f7411bd8af8c8c12429346fa83dfc5433ad797e7c8765da9ed30b4204dc0c69
      variant_obligation: evidence.sdk-task-memory-workflow-finite-observation#source:7c7a9d698eb9e1ecfc41cd3e78dac27b0a3f02b5650f45b9464ea596f2b55b5d#variant:a8fbf4c9068289322b22b407bba7c28f3465cb271d5be28e465238c81455e2df
      variant_fingerprint: a8fbf4c9068289322b22b407bba7c28f3465cb271d5be28e465238c81455e2df
      raw_reference: echo-sdk-host/src/core_profile/facade/subagent.rs
      source_role: production_source
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/src/core_profile/facade/subagent.rs
      replacement_content_sha256: 7c7a9d698eb9e1ecfc41cd3e78dac27b0a3f02b5650f45b9464ea596f2b55b5d
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_92a9863c06302229
  ? evidence.sdk-task-memory-workflow-finite-observation#source:d516d4af66328d7c7dbb6d1ec6c940b14cc8364c306228aa30d96f1472aa3825#variant:a1501eed249e5a350af3cf35d8ead09e412a207f4ea919b3e362d038e952f916
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: a1501eed249e5a350af3cf35d8ead09e412a207f4ea919b3e362d038e952f916
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: 7b3857b0b714fa68eb3218575321169cf7e3fb2c0b63db54e1fe4a8ed34abf67
      variant_obligation: evidence.sdk-task-memory-workflow-finite-observation#source:d516d4af66328d7c7dbb6d1ec6c940b14cc8364c306228aa30d96f1472aa3825#variant:a1501eed249e5a350af3cf35d8ead09e412a207f4ea919b3e362d038e952f916
      variant_fingerprint: a1501eed249e5a350af3cf35d8ead09e412a207f4ea919b3e362d038e952f916
      raw_reference: echo-sdk-host/src/core_profile/facade/workflow.rs
      source_role: production_source
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/src/core_profile/facade/workflow.rs
      replacement_content_sha256: d516d4af66328d7c7dbb6d1ec6c940b14cc8364c306228aa30d96f1472aa3825
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_ca2525c315b84e49
  ? evidence.sdk-task-memory-workflow-finite-observation#source:d516d4af66328d7c7dbb6d1ec6c940b14cc8364c306228aa30d96f1472aa3825#variant:dbba27a2c8d1cfbfd86da3cce4462923b0d1ad4c2973a67dad14d1c544dd1c45
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: dbba27a2c8d1cfbfd86da3cce4462923b0d1ad4c2973a67dad14d1c544dd1c45
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: 7b3857b0b714fa68eb3218575321169cf7e3fb2c0b63db54e1fe4a8ed34abf67
      variant_obligation: evidence.sdk-task-memory-workflow-finite-observation#source:d516d4af66328d7c7dbb6d1ec6c940b14cc8364c306228aa30d96f1472aa3825#variant:dbba27a2c8d1cfbfd86da3cce4462923b0d1ad4c2973a67dad14d1c544dd1c45
      variant_fingerprint: dbba27a2c8d1cfbfd86da3cce4462923b0d1ad4c2973a67dad14d1c544dd1c45
      raw_reference: echo-sdk-host/src/core_profile/facade/workflow.rs#optional_workflow_fields_reject_malformed_values
      source_role: production_source
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/src/core_profile/facade/workflow.rs
      replacement_content_sha256: d516d4af66328d7c7dbb6d1ec6c940b14cc8364c306228aa30d96f1472aa3825
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_4387335b699584f3
  ? evidence.sdk-task-memory-workflow-finite-observation#source:d516d4af66328d7c7dbb6d1ec6c940b14cc8364c306228aa30d96f1472aa3825#variant:ee1b0c5ad7edee2ec823349ed097aeaee8ab9f432159c144b319d2287a3ad475
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: ee1b0c5ad7edee2ec823349ed097aeaee8ab9f432159c144b319d2287a3ad475
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: 7b3857b0b714fa68eb3218575321169cf7e3fb2c0b63db54e1fe4a8ed34abf67
      variant_obligation: evidence.sdk-task-memory-workflow-finite-observation#source:d516d4af66328d7c7dbb6d1ec6c940b14cc8364c306228aa30d96f1472aa3825#variant:ee1b0c5ad7edee2ec823349ed097aeaee8ab9f432159c144b319d2287a3ad475
      variant_fingerprint: ee1b0c5ad7edee2ec823349ed097aeaee8ab9f432159c144b319d2287a3ad475
      raw_reference: echo-sdk-host/src/core_profile/facade/workflow.rs#agent_node_uses_replaced_live_llm_after_session_services_capture
      source_role: production_source
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/src/core_profile/facade/workflow.rs
      replacement_content_sha256: d516d4af66328d7c7dbb6d1ec6c940b14cc8364c306228aa30d96f1472aa3825
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_29534fa4b455ab71
  ? evidence.sdk-task-memory-workflow-finite-observation#source:e46e619a603b599db1749e785f3460d27dec74f6ee7df6ef0209d86ab0f3b105#variant:dcd2ddf345a8273740e3e7804c452901aa7bc9d94cccc3a1f71fe67b3cd41938
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: dcd2ddf345a8273740e3e7804c452901aa7bc9d94cccc3a1f71fe67b3cd41938
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: 547ff990de794a2611b0f5309126701b02d5eaace7c9e92a8908992bf81652d0
      variant_obligation: evidence.sdk-task-memory-workflow-finite-observation#source:e46e619a603b599db1749e785f3460d27dec74f6ee7df6ef0209d86ab0f3b105#variant:dcd2ddf345a8273740e3e7804c452901aa7bc9d94cccc3a1f71fe67b3cd41938
      variant_fingerprint: dcd2ddf345a8273740e3e7804c452901aa7bc9d94cccc3a1f71fe67b3cd41938
      raw_reference: echo-sdk-host/src/core_profile/facade/task_runtime.rs#request_live_interrupt
      source_role: production_source
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/src/core_profile/facade/task_runtime.rs
      replacement_content_sha256: e46e619a603b599db1749e785f3460d27dec74f6ee7df6ef0209d86ab0f3b105
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_7638795d94b038d7
  ? evidence.sdk-task-memory-workflow-finite-observation#source:e46e619a603b599db1749e785f3460d27dec74f6ee7df6ef0209d86ab0f3b105#variant:f0d74217fbb04e59f4f297bd129ad1cb331f18477a6db87e63e33343bca06603
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: f0d74217fbb04e59f4f297bd129ad1cb331f18477a6db87e63e33343bca06603
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: 547ff990de794a2611b0f5309126701b02d5eaace7c9e92a8908992bf81652d0
      variant_obligation: evidence.sdk-task-memory-workflow-finite-observation#source:e46e619a603b599db1749e785f3460d27dec74f6ee7df6ef0209d86ab0f3b105#variant:f0d74217fbb04e59f4f297bd129ad1cb331f18477a6db87e63e33343bca06603
      variant_fingerprint: f0d74217fbb04e59f4f297bd129ad1cb331f18477a6db87e63e33343bca06603
      raw_reference: echo-sdk-host/src/core_profile/facade/task_runtime.rs
      source_role: production_source
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/src/core_profile/facade/task_runtime.rs
      replacement_content_sha256: e46e619a603b599db1749e785f3460d27dec74f6ee7df6ef0209d86ab0f3b105
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_77500919c3eed2f3
  ? evidence.sdk-tools-bridge-finite-observation#source:245d9daf8847d8b64fe15cf9800e7a74c7caf6c448da5576cd82161d4fc371b0#variant:248aa8d6d909b1ae800eea1db1f9d0b94415f8df1f3043bc55b74d10810f2ccd
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 248aa8d6d909b1ae800eea1db1f9d0b94415f8df1f3043bc55b74d10810f2ccd
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: ba63d244383a90e0a62691778a01b058fd0730177d842de232dd1619715e0243
      variant_obligation: evidence.sdk-tools-bridge-finite-observation#source:245d9daf8847d8b64fe15cf9800e7a74c7caf6c448da5576cd82161d4fc371b0#variant:248aa8d6d909b1ae800eea1db1f9d0b94415f8df1f3043bc55b74d10810f2ccd
      variant_fingerprint: 248aa8d6d909b1ae800eea1db1f9d0b94415f8df1f3043bc55b74d10810f2ccd
      raw_reference: echo-sdk-host/src/core_profile/extension_bridge.rs#dropped_pending_stream_releases_sink_and_session_reentry_guard
      source_role: production_source
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/src/core_profile/extension_bridge.rs
      replacement_content_sha256: 245d9daf8847d8b64fe15cf9800e7a74c7caf6c448da5576cd82161d4fc371b0
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_4b7380cd49f415bd
  ? evidence.sdk-tools-bridge-finite-observation#source:245d9daf8847d8b64fe15cf9800e7a74c7caf6c448da5576cd82161d4fc371b0#variant:457c911f51404276cb611fe418ddb23ace1211a5d3f568ed854320043cede7b5
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 457c911f51404276cb611fe418ddb23ace1211a5d3f568ed854320043cede7b5
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: ba63d244383a90e0a62691778a01b058fd0730177d842de232dd1619715e0243
      variant_obligation: evidence.sdk-tools-bridge-finite-observation#source:245d9daf8847d8b64fe15cf9800e7a74c7caf6c448da5576cd82161d4fc371b0#variant:457c911f51404276cb611fe418ddb23ace1211a5d3f568ed854320043cede7b5
      variant_fingerprint: 457c911f51404276cb611fe418ddb23ace1211a5d3f568ed854320043cede7b5
      raw_reference: echo-sdk-host/src/core_profile/extension_bridge.rs
      source_role: production_source
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/src/core_profile/extension_bridge.rs
      replacement_content_sha256: 245d9daf8847d8b64fe15cf9800e7a74c7caf6c448da5576cd82161d4fc371b0
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_277ca8cfa79bac3c
  ? evidence.sdk-tools-bridge-finite-observation#source:2df1bbe8ff3221c690d016379030dee6f76afa8f1893f8ccadb1791d14769649#variant:14ddb230709e834eed0032424ed0e842368ff1b6ca4135009403eef1db4faa45
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 14ddb230709e834eed0032424ed0e842368ff1b6ca4135009403eef1db4faa45
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: 0e7343467c6d48a904b47f5c3a92f36bea35e610b44b99439a337b357c51278a
      variant_obligation: evidence.sdk-tools-bridge-finite-observation#source:2df1bbe8ff3221c690d016379030dee6f76afa8f1893f8ccadb1791d14769649#variant:14ddb230709e834eed0032424ed0e842368ff1b6ca4135009403eef1db4faa45
      variant_fingerprint: 14ddb230709e834eed0032424ed0e842368ff1b6ca4135009403eef1db4faa45
      raw_reference: echo-sdk-host/src/core_profile/facade/source_operations.rs#factory_failure_retains_canonical_cause_in_real_eval_result
      source_role: production_source
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/src/core_profile/facade/source_operations.rs
      replacement_content_sha256: 2df1bbe8ff3221c690d016379030dee6f76afa8f1893f8ccadb1791d14769649
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_cac5ee20c4b13bf1
  ? evidence.sdk-tools-bridge-finite-observation#source:2df1bbe8ff3221c690d016379030dee6f76afa8f1893f8ccadb1791d14769649#variant:2172623585f7149d8e946c75d710f149aef2e542ebeea4f953be5701ebeb4594
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 2172623585f7149d8e946c75d710f149aef2e542ebeea4f953be5701ebeb4594
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: 0e7343467c6d48a904b47f5c3a92f36bea35e610b44b99439a337b357c51278a
      variant_obligation: evidence.sdk-tools-bridge-finite-observation#source:2df1bbe8ff3221c690d016379030dee6f76afa8f1893f8ccadb1791d14769649#variant:2172623585f7149d8e946c75d710f149aef2e542ebeea4f953be5701ebeb4594
      variant_fingerprint: 2172623585f7149d8e946c75d710f149aef2e542ebeea4f953be5701ebeb4594
      raw_reference: echo-sdk-host/src/core_profile/facade/source_operations.rs
      source_role: production_source
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/src/core_profile/facade/source_operations.rs
      replacement_content_sha256: 2df1bbe8ff3221c690d016379030dee6f76afa8f1893f8ccadb1791d14769649
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_71570dc99c8ecf3a
  ? evidence.sdk-tools-bridge-finite-observation#source:4d47c7c9e9d541087e0ef2e48cb9d6a495b357216ee92276ab6a6e267195be37#variant:7e234f84ccf6df63d92b6697ff7e32e0d9bbbf6530465018718f944c2c827c16
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 7e234f84ccf6df63d92b6697ff7e32e0d9bbbf6530465018718f944c2c827c16
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: 51ce4b942f4beb1efe2fa54d03537767a3a74313fb16823345b22416adacf00c
      variant_obligation: evidence.sdk-tools-bridge-finite-observation#source:4d47c7c9e9d541087e0ef2e48cb9d6a495b357216ee92276ab6a6e267195be37#variant:7e234f84ccf6df63d92b6697ff7e32e0d9bbbf6530465018718f944c2c827c16
      variant_fingerprint: 7e234f84ccf6df63d92b6697ff7e32e0d9bbbf6530465018718f944c2c827c16
      raw_reference: echo-sdk-host/src/core_profile/wire.rs
      source_role: production_source
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/src/core_profile/wire.rs
      replacement_content_sha256: 4d47c7c9e9d541087e0ef2e48cb9d6a495b357216ee92276ab6a6e267195be37
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_0d490ab5707c3c04
  ? evidence.sdk-tools-bridge-finite-observation#source:4d47c7c9e9d541087e0ef2e48cb9d6a495b357216ee92276ab6a6e267195be37#variant:92c34aca25dece935f8ec879fc206646638483e77fbcde72e84b9f304d3e332a
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 92c34aca25dece935f8ec879fc206646638483e77fbcde72e84b9f304d3e332a
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: 51ce4b942f4beb1efe2fa54d03537767a3a74313fb16823345b22416adacf00c
      variant_obligation: evidence.sdk-tools-bridge-finite-observation#source:4d47c7c9e9d541087e0ef2e48cb9d6a495b357216ee92276ab6a6e267195be37#variant:92c34aca25dece935f8ec879fc206646638483e77fbcde72e84b9f304d3e332a
      variant_fingerprint: 92c34aca25dece935f8ec879fc206646638483e77fbcde72e84b9f304d3e332a
      raw_reference: echo-sdk-host/src/core_profile/wire.rs#managed_checkpoint_and_receipts_preserve_large_revisions_and_subseconds
      source_role: production_source
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/src/core_profile/wire.rs
      replacement_content_sha256: 4d47c7c9e9d541087e0ef2e48cb9d6a495b357216ee92276ab6a6e267195be37
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_ffdd36c54f82c7e0
  ? evidence.sdk-tools-bridge-finite-observation#source:77c1d305a55e8320692929efca4f48a6d6dc7a4370cd33c9b403da996dcc15c3#variant:03fa76555b60da8df60afeeb1b330bc886ec369814a5cb9226f1974050420767
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 03fa76555b60da8df60afeeb1b330bc886ec369814a5cb9226f1974050420767
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: 3b2e7e250b1be8f5a31555ec99d084cb2e59dc22bdb712fd57c1aa49b69c3e36
      variant_obligation: evidence.sdk-tools-bridge-finite-observation#source:77c1d305a55e8320692929efca4f48a6d6dc7a4370cd33c9b403da996dcc15c3#variant:03fa76555b60da8df60afeeb1b330bc886ec369814a5cb9226f1974050420767
      variant_fingerprint: 03fa76555b60da8df60afeeb1b330bc886ec369814a5cb9226f1974050420767
      raw_reference: echo-sdk-host/src/core_profile/facade/tools.rs
      source_role: production_source
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/src/core_profile/facade/tools.rs
      replacement_content_sha256: 77c1d305a55e8320692929efca4f48a6d6dc7a4370cd33c9b403da996dcc15c3
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_0789005c3aad549d
  ? evidence.sdk-tools-bridge-finite-observation#source:77c1d305a55e8320692929efca4f48a6d6dc7a4370cd33c9b403da996dcc15c3#variant:337e8c54132e68408674ab9d14d42c1764e28cd593d547f5442e67b8178a2d95
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 337e8c54132e68408674ab9d14d42c1764e28cd593d547f5442e67b8178a2d95
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: 3b2e7e250b1be8f5a31555ec99d084cb2e59dc22bdb712fd57c1aa49b69c3e36
      variant_obligation: evidence.sdk-tools-bridge-finite-observation#source:77c1d305a55e8320692929efca4f48a6d6dc7a4370cd33c9b403da996dcc15c3#variant:337e8c54132e68408674ab9d14d42c1764e28cd593d547f5442e67b8178a2d95
      variant_fingerprint: 337e8c54132e68408674ab9d14d42c1764e28cd593d547f5442e67b8178a2d95
      raw_reference: echo-sdk-host/src/core_profile/facade/tools.rs#validate_tool_input
      source_role: production_source
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/src/core_profile/facade/tools.rs
      replacement_content_sha256: 77c1d305a55e8320692929efca4f48a6d6dc7a4370cd33c9b403da996dcc15c3
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_8c331b10bcf7e72d
  ? evidence.sdk-tools-bridge-finite-observation#source:77c1d305a55e8320692929efca4f48a6d6dc7a4370cd33c9b403da996dcc15c3#variant:777ea0edb4eed8f20d340d7cd14c0167532ddd001be4ab657e62100a04a66e29
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 777ea0edb4eed8f20d340d7cd14c0167532ddd001be4ab657e62100a04a66e29
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: 3b2e7e250b1be8f5a31555ec99d084cb2e59dc22bdb712fd57c1aa49b69c3e36
      variant_obligation: evidence.sdk-tools-bridge-finite-observation#source:77c1d305a55e8320692929efca4f48a6d6dc7a4370cd33c9b403da996dcc15c3#variant:777ea0edb4eed8f20d340d7cd14c0167532ddd001be4ab657e62100a04a66e29
      variant_fingerprint: 777ea0edb4eed8f20d340d7cd14c0167532ddd001be4ab657e62100a04a66e29
      raw_reference: echo-sdk-host/src/core_profile/facade/tools.rs#direct_tool_validation_precedes_custom_rules_and_execution
      source_role: production_source
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/src/core_profile/facade/tools.rs
      replacement_content_sha256: 77c1d305a55e8320692929efca4f48a6d6dc7a4370cd33c9b403da996dcc15c3
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_5e13cb39efa08593
  ? evidence.sdk-tools-bridge-finite-observation#source:be870643edf05900800a083e09a678b367749060138c1295f2aecd4f46faf5fd#variant:308bcbb5233a56cf88cbc038a438687cecd807994c9822a0d31a4084f63b3bac
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 308bcbb5233a56cf88cbc038a438687cecd807994c9822a0d31a4084f63b3bac
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: 332837ed29a2ac883ac3494206c8e6a24bc3b00d442f983ae7b097add2f8e485
      variant_obligation: evidence.sdk-tools-bridge-finite-observation#source:be870643edf05900800a083e09a678b367749060138c1295f2aecd4f46faf5fd#variant:308bcbb5233a56cf88cbc038a438687cecd807994c9822a0d31a4084f63b3bac
      variant_fingerprint: 308bcbb5233a56cf88cbc038a438687cecd807994c9822a0d31a4084f63b3bac
      raw_reference: echo-sdk-host/tests/extension_bridge_e2e.rs#managed_persistence_deadline_and_lost_ack_preserve_commit_authority
      source_role: test_consumer
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/tests/extension_bridge_e2e.rs
      replacement_content_sha256: be870643edf05900800a083e09a678b367749060138c1295f2aecd4f46faf5fd
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_dddfce0b986f41c4
  ? evidence.sdk-tools-bridge-finite-observation#source:be870643edf05900800a083e09a678b367749060138c1295f2aecd4f46faf5fd#variant:396d04fe8f803da01d8baa0ce8386f3a91c5c06579bbddf0a9800524e4db9b98
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 396d04fe8f803da01d8baa0ce8386f3a91c5c06579bbddf0a9800524e4db9b98
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: 332837ed29a2ac883ac3494206c8e6a24bc3b00d442f983ae7b097add2f8e485
      variant_obligation: evidence.sdk-tools-bridge-finite-observation#source:be870643edf05900800a083e09a678b367749060138c1295f2aecd4f46faf5fd#variant:396d04fe8f803da01d8baa0ce8386f3a91c5c06579bbddf0a9800524e4db9b98
      variant_fingerprint: 396d04fe8f803da01d8baa0ce8386f3a91c5c06579bbddf0a9800524e4db9b98
      raw_reference: echo-sdk-host/tests/extension_bridge_e2e.rs
      source_role: test_consumer
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/tests/extension_bridge_e2e.rs
      replacement_content_sha256: be870643edf05900800a083e09a678b367749060138c1295f2aecd4f46faf5fd
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_e1deb9316f541ac3
  ? evidence.sdk-tools-bridge-finite-observation#source:be870643edf05900800a083e09a678b367749060138c1295f2aecd4f46faf5fd#variant:53ed02782d1d25e7ea81522dacadf37df42ff1f5c8e16a1e7290b4b8a24a746c
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 53ed02782d1d25e7ea81522dacadf37df42ff1f5c8e16a1e7290b4b8a24a746c
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: 332837ed29a2ac883ac3494206c8e6a24bc3b00d442f983ae7b097add2f8e485
      variant_obligation: evidence.sdk-tools-bridge-finite-observation#source:be870643edf05900800a083e09a678b367749060138c1295f2aecd4f46faf5fd#variant:53ed02782d1d25e7ea81522dacadf37df42ff1f5c8e16a1e7290b4b8a24a746c
      variant_fingerprint: 53ed02782d1d25e7ea81522dacadf37df42ff1f5c8e16a1e7290b4b8a24a746c
      raw_reference: echo-sdk-host/tests/extension_bridge_e2e.rs#missing_managed_capability_fails_before_store_mutation
      source_role: test_consumer
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/tests/extension_bridge_e2e.rs
      replacement_content_sha256: be870643edf05900800a083e09a678b367749060138c1295f2aecd4f46faf5fd
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_accd6ed324eb962a
  ? evidence.sdk-tools-bridge-finite-observation#source:be870643edf05900800a083e09a678b367749060138c1295f2aecd4f46faf5fd#variant:7fa187bb6716c74c299bfed8a8ad65b33b027700889ae533a31dbfd4beeedf0f
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: 7fa187bb6716c74c299bfed8a8ad65b33b027700889ae533a31dbfd4beeedf0f
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: 332837ed29a2ac883ac3494206c8e6a24bc3b00d442f983ae7b097add2f8e485
      variant_obligation: evidence.sdk-tools-bridge-finite-observation#source:be870643edf05900800a083e09a678b367749060138c1295f2aecd4f46faf5fd#variant:7fa187bb6716c74c299bfed8a8ad65b33b027700889ae533a31dbfd4beeedf0f
      variant_fingerprint: 7fa187bb6716c74c299bfed8a8ad65b33b027700889ae533a31dbfd4beeedf0f
      raw_reference: echo-sdk-host/tests/extension_bridge_e2e.rs#tool_manager_timeout_before_stream_ack_releases_callback_owner
      source_role: test_consumer
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/tests/extension_bridge_e2e.rs
      replacement_content_sha256: be870643edf05900800a083e09a678b367749060138c1295f2aecd4f46faf5fd
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_8ff9a8c54fd94fdc
  ? evidence.sdk-tools-bridge-finite-observation#source:be870643edf05900800a083e09a678b367749060138c1295f2aecd4f46faf5fd#variant:b69a43f893bb4fb71b07b7a0c1ff308a95f99c77189526a8c8fb991fe4ff6490
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: b69a43f893bb4fb71b07b7a0c1ff308a95f99c77189526a8c8fb991fe4ff6490
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: 332837ed29a2ac883ac3494206c8e6a24bc3b00d442f983ae7b097add2f8e485
      variant_obligation: evidence.sdk-tools-bridge-finite-observation#source:be870643edf05900800a083e09a678b367749060138c1295f2aecd4f46faf5fd#variant:b69a43f893bb4fb71b07b7a0c1ff308a95f99c77189526a8c8fb991fe4ff6490
      variant_fingerprint: b69a43f893bb4fb71b07b7a0c1ff308a95f99c77189526a8c8fb991fe4ff6490
      raw_reference: echo-sdk-host/tests/extension_bridge_e2e.rs#managed_commit_disconnect_restart_replays_same_receipt
      source_role: test_consumer
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/tests/extension_bridge_e2e.rs
      replacement_content_sha256: be870643edf05900800a083e09a678b367749060138c1295f2aecd4f46faf5fd
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_758d7ee8f0be6c67
  ? evidence.sdk-tools-bridge-finite-observation#source:be870643edf05900800a083e09a678b367749060138c1295f2aecd4f46faf5fd#variant:f8a4176c6651f97bff7dc76af86bcc388d54886fdce8d61cf1e204323572da4f
  : disposition: replaced
    predecessor_fingerprints:
      7f14a1b9c751b0612ceb42224f83f29d09aea46c: f8a4176c6651f97bff7dc76af86bcc388d54886fdce8d61cf1e204323572da4f
    replacement_ref: asset.sdk-historical-proof-location
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
    decision_authorities: *id001
    compatibility_impact: 原id/业务/status/supports/限制/失败/观察历史均保，当前真实consumer或locator显式替代存储表示
    rollback_ref: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
    source_dependency_variant_replacements:
    - snapshot_revision: 7f14a1b9c751b0612ceb42224f83f29d09aea46c
      collision_id: 332837ed29a2ac883ac3494206c8e6a24bc3b00d442f983ae7b097add2f8e485
      variant_obligation: evidence.sdk-tools-bridge-finite-observation#source:be870643edf05900800a083e09a678b367749060138c1295f2aecd4f46faf5fd#variant:f8a4176c6651f97bff7dc76af86bcc388d54886fdce8d61cf1e204323572da4f
      variant_fingerprint: f8a4176c6651f97bff7dc76af86bcc388d54886fdce8d61cf1e204323572da4f
      raw_reference: echo-sdk-host/tests/extension_bridge_e2e.rs#cancelled_initial_stream_response_releases_session_callback_owner
      source_role: test_consumer
      replacement_ref: asset.sdk-historical-proof-location
      replacement_source_ref: echo-sdk-host/tests/extension_bridge_e2e.rs
      replacement_content_sha256: be870643edf05900800a083e09a678b367749060138c1295f2aecd4f46faf5fd
      replacement_git_mode: '100644'
      equivalence_evidence_ref: evidence.sdk-historical-proof-storage-equivalence
      scenario: preserve_96c21d51286429ac
---
# 历史存储逐义务连续性

## 来源与范围
真实唯一冻结父 `7f14a1b9c751b0612ceb42224f83f29d09aea46c` 的924个改变义务，原508未变义务精同。原330前置错误不能泛忽略；296 typedissues及34真实碰撞各有精确issue/父FP/raw/role/variant mapping。

## 支持的结论
当前canonicalAsset真实引用现存代码/测试/协议；文稿272映射只能使用docs存储规则，23旧测试/1旧production使用强same-role替代。后来从当前待删raw发现的24test/14protocol亦保真实角色；所有124variants/rawrefs全映射。仅实际matched Eq+真实storageADR及afterDG绑定才允许对应前置问题处置；result缺current仍fatal。

## 已知缺口
本材料不是完整gate结果；原S PF/index/源码原样，实际Sapply/merge需Root最终批准。无业务retire/风险接受/状态提升，原四open与剩余main验收不变。
