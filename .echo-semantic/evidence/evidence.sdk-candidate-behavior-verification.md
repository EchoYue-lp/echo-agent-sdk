---
schema_version: 1
id: evidence.sdk-candidate-behavior-verification
kind: evidence
observed_at: 56929154c1851473e9e4837901b610cecf001b2d
source_refs:
- docs/sdk/governance/evidence/sdk-fc63-historical-20261013/files/evidence.sdk-candidate-behavior-verification.original.txt
- docs/sdk/governance/evidence/sdk-fc63-historical-20261013/archive-index.json
- docs/sdk/governance/evidence/sdk-fc63-historical-20261013/comparison.json
- docs/sdk/governance/evidence/sdk-fc63-historical-20261013/files/sdk-family-candidate-runtime-source-freeze.json
- docs/sdk/governance/evidence/sdk-fc63-historical-20261013/files/sdk-full-frozen-language-proof.json
- docs/sdk/governance/evidence/sdk-fc63-historical-20261013/files/sdk-full-frozen-language-gates-corrected.log
- docs/sdk/governance/evidence/sdk-fc63-historical-20261013/files/sdk-full-frozen-language-gates.sh
- docs/sdk/governance/evidence/sdk-fc63-historical-20261013/files/sdk-final-host-local-candidate.log
- docs/sdk/governance/evidence/sdk-fc63-historical-20261013/files/sdk-family-handle-host-validation.json
- docs/sdk/governance/evidence/sdk-fc63-historical-20261013/files/sdk-lsp-three-language-comparison.json
- docs/sdk/governance/evidence/sdk-fc63-historical-20261013/files/sdk-lsp-language-proof.json
- docs/sdk/governance/evidence/sdk-fc63-historical-20261013/files/sdk-identity-reservation-disabled-red.log
- docs/sdk/governance/evidence/sdk-fc63-historical-20261013/files/sdk-identity-reservation-restored-full-core.log
supports: []
limitations:
- Historical fc63 receipts only; source:9e1 observation epoch is not a recoverable
  Git snapshot.
- Original evidence/body/file hashes and raw command logs are byte preserved in the
  archive.
- Historical TS219/Python279+18/Java233 do not prove current Java2bad or Workflow
  successor.
- Current-source partial named assertions require explicit function/hash/run-name
  comparison; no full current closure is inferred.
---

# Historical SDK candidate receipt audit

## 支持的结论

This object locates preserved historical runtime receipts, not a current capability
pass. Original Candidate Evidence is archived byte-for-byte as a text artifact,
including its original observation epoch, 40 partial family claims, 27 file hashes
and selected test-body hashes. Historical Host fc63 and TS219/Python279+18/Java233
are retained without relabeling them as current Java2bad or Workflow runtime proof.
No original source:9e1 Git snapshot is fabricated or claimed recoverable.

## 来源与范围

The repository archive's index records original /tmp source paths and SHA256;
comparison.json separately checks original file/function fingerprints against
currently read inputs. Equal test bodies can support only their named assertions
when the actual frozen command log names them as passed. A source mismatch remains
explicit and cannot be erased by replacing observed_at. The original declaration,
raw logs and all limitations remain available for independent review.

## 已知缺口

The archived source epoch cannot prove current repository state. Whole accepted
scope, final published framework pin, independent SDK build, Workflow successor,
final full language/source checks and precise main still require their own evidence.
Behavior states remain needs_review; this historical object has no current supports.

## 对原保存收据的当前审计观察

本对象observed_at现在指真正当前候选Git `56929154c1851473e9e4837901b610cecf001b2d`，只定位已经保存的收据、输入源码副本与manifest的审计观察。原观测字段为`source:3ea544c47e331b5d29a6624f978f3595619bef39fa5985230351098d38930404`；它若是source摘要或pending，均不被伪装为历史执行当时的可恢复整仓Git。当前观察不宣称原运行发生于Git569、不重跑或升级原runtimeepoch。

全部上列source_refs均从该Gitblob实际读取，与当前archive原字节SHA精确相同，原锚点存在；原日志时间、HostbinarySHA、sourcefreeze、错误/通过计数及正文来源保持。原body的epoch/signature/cause限制与supports均为原Behavior义务的历史有界支持，不能由此宣布当前源码verified；旧3ea/9e没有完整Git时仅有原保存收据可恢复，整仓原epoch仍不可恢复，不补造树。

原保存artifact中的具名元数据（完整值仍见原JSON，不改变）：`host_proof.digests.source_contract_digest=sha256:ce80aa3a7793d4460e082772eb24fa228734d65f98ec997af2087be27b8e015e`；`lsp_comparison.host_sha256=fc63c0815b319424ea2c64a94d3fc6a365a3dff6b70882a23b9edd1266d2f6eb`；`host_sha256=fc63c0815b319424ea2c64a94d3fc6a365a3dff6b70882a23b9edd1266d2f6eb`；`digests.source_contract_digest=sha256:ce80aa3a7793d4460e082772eb24fa228734d65f98ec997af2087be27b8e015e`；`host_sha256=b4625589ecd8d9d356b787d29ae963e5fd467792991a57634cada4fbdd7949d3`；`green_host_sha256=fc63c0815b319424ea2c64a94d3fc6a365a3dff6b70882a23b9edd1266d2f6eb`；`comparison.host_sha256=fc63c0815b319424ea2c64a94d3fc6a365a3dff6b70882a23b9edd1266d2f6eb`。

本次只附加观察说明；原正文/supports/limitations保留，原rawReceipt/ArchiveChecksum/运行时间未写入，任何未决unknown/失败仍不自动resolved。源码业务Audit不是此收据观察，另队列保持原revision与复审义务，不能偷换类型或scope。
