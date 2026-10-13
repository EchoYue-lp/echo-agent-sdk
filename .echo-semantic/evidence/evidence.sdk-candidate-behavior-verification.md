---
{
  "schema_version": 1,
  "id": "evidence.sdk-candidate-behavior-verification",
  "kind": "evidence",
  "observed_at": "source:3ea544c47e331b5d29a6624f978f3595619bef39fa5985230351098d38930404",
  "source_refs": [
    "docs/sdk/governance/evidence/sdk-fc63-historical-20261013/files/evidence.sdk-candidate-behavior-verification.original.txt",
    "docs/sdk/governance/evidence/sdk-fc63-historical-20261013/archive-index.json",
    "docs/sdk/governance/evidence/sdk-fc63-historical-20261013/comparison.json",
    "docs/sdk/governance/evidence/sdk-fc63-historical-20261013/files/sdk-family-candidate-runtime-source-freeze.json",
    "docs/sdk/governance/evidence/sdk-fc63-historical-20261013/files/sdk-full-frozen-language-proof.json",
    "docs/sdk/governance/evidence/sdk-fc63-historical-20261013/files/sdk-full-frozen-language-gates-corrected.log",
    "docs/sdk/governance/evidence/sdk-fc63-historical-20261013/files/sdk-full-frozen-language-gates.sh",
    "docs/sdk/governance/evidence/sdk-fc63-historical-20261013/files/sdk-final-host-local-candidate.log",
    "docs/sdk/governance/evidence/sdk-fc63-historical-20261013/files/sdk-family-handle-host-validation.json",
    "docs/sdk/governance/evidence/sdk-fc63-historical-20261013/files/sdk-lsp-three-language-comparison.json",
    "docs/sdk/governance/evidence/sdk-fc63-historical-20261013/files/sdk-lsp-language-proof.json",
    "docs/sdk/governance/evidence/sdk-fc63-historical-20261013/files/sdk-identity-reservation-disabled-red.log",
    "docs/sdk/governance/evidence/sdk-fc63-historical-20261013/files/sdk-identity-reservation-restored-full-core.log"
  ],
  "supports": [],
  "limitations": [
    "Historical fc63 receipts only; source:9e1 observation epoch is not a recoverable Git snapshot.",
    "Original evidence/body/file hashes and raw command logs are byte preserved in the archive.",
    "Historical TS219/Python279+18/Java233 do not prove current Java2bad or Workflow successor.",
    "Current-source partial named assertions require explicit function/hash/run-name comparison; no full current closure is inferred."
  ]
}
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
