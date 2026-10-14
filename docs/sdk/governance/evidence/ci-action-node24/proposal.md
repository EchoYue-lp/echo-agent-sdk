# Three setup actions: Node24 compatibility maintenance

Status: proposed, not applied; SDK source/preflight/workflow unchanged. Waiting for independent review and Root write authorization. Existing workflow HEAD at capture is in official-provenance.json.

Change exactly the three `language-sdk` `uses` scalars to released full commit SHAs with version comments:

| Action | Release | Exact commit |
| --- | --- | --- |
| actions/setup-node | v6.5.0 | 249970729cb0ef3589644e2896645e5dc5ba9c38 |
| astral-sh/setup-uv | v7.6.0 | 37802adc94f370d6bfd71619e3f0bf239e1f3b78 |
| actions/setup-java | v5.6.0 | 03ad4de0992f5dab5e18fcb136590ce7c4a0ac95 |

`three-uses.patch`, workflow.before.yml and workflow.candidate.yml are the complete candidate. `validate.py` executed successfully using PyYAML BaseLoader: only these three uses AST values change, comments state their release versions, all other job inputs/commands/cache configuration/env/concurrency/permissions/timeouts remain equal. Node test version remains20, Java remains Temurin17; no Python-version input is added, so the original environment remains. No global TZ, skip, NODE_NO_WARNINGS or new timeout is added. `actionlint` is actually absent; no actionlint pass claimed.

## Actual warning and official source

Original immutable log: /Users/ls/MyWork/code/ylp_agent_learn/lp-agent/.worktrees/admin/sdk-pr5-9a-ci-evidence/116126313110.log. It records runner2.337.0, Node20-targeted three actions forced onto Node24, setup-java4 deprecation, DEP0040 punycode and DEP0169 url.parse warnings. This is distinct from the Node20 test interpreter selected by setup-node.

The three action subdirectories contain original official GitHub API response bytes for the precise release and tag commit plus README/action.yml contents at the pinned commit; the decoded entities are exact API base64 bytes with no formatting. official-provenance.json records every URL/status/byte length/SHA. All three exact action.yml files declare node24; input names actually configured here remain valid. Node/Java official READMEs specify runner>=2.327.1, satisfied by the observed2.337.0. setup-uv7 official migration documents say hosted runners support Node24; the same actual runner also meets that runtime prerequisite. No new self-hosted minimum beyond the official Node24 prerequisite is invented.

References: https://github.com/actions/setup-node/releases/tag/v6.5.0 ; https://github.com/actions/setup-java/releases/tag/v5.6.0 ; https://github.com/astral-sh/setup-uv/releases/tag/v7.6.0 .

## Compatibility and bounded tradeoffs

Node4->5 adds package-manager auto-cache; node6 limits auto-cache to npm. Existing explicit cache:npm and dependency path retain this repository's configured cache behavior. v6.5 is selected over older v5 which still has older cache4/http-client2 dependencies, and avoids the additional node7 ESM/auth-token migration.

Java5 moves action runtime to Node24; distribution:temurin/java-version:17 remain supported. Removed/deprecated Adopt and jdkFile paths are not used. v5.6 is a released v5 maintenance backport, so Java6 migration is unnecessary for this warning repair.

uv5->6 makes venv activation explicit, removes pyproject-file/uv-file and changes the default cache-dependency glob; this workflow has none of those old inputs or python-version and does not request activation. The new default glob adds pyproject.toml and may invalidate the cache once. uv7 removes deprecated server-url; absent here. uv7.6 defaults downloads to releases.astral.sh (official release body saved) and respects cache-dir configuration. Root explicitly accepted this ordinary download implementation change. Workflow cache input text remains unchanged; this does not claim upstream defaults are all identical. No uv version or Python interpreter is newly pinned. Current action binaries may choose a uv tool according to the same existing no-version configuration; runtime correctness must be verified by actual CI.

## Verification and limits

ast-validation.json contains actual exit0 and exact three-index delta; it validates candidate input names against each pinned action.yml. Official minimum runner/runtime inspection is static evidence. No new remote CI has executed this candidate, and **warningresolved is not claimed**. After Root writes/commits and CI runs, inspect complete action/test/post-job logs and require zero other warnings; the accepted duplicate50 Cargo-deny exception does not excuse action/runtime warnings.

Web search tool returned404/v1/alpha/search and raw.githubusercontent requests reset; official api.github.com reads succeeded for all precise candidate entities. A later supplementary migration release download encountered403 rate-limit; recorded in additional-migration-fetch-limitation.json, not hidden or repeatedly retried. Existing successful official candidate bytes remain complete.

No source-author-owned test/helpers, business code, SDK API/schema/authority or third tooling repository was modified. No Cargo/Maven/native suite was started. SDK preflight is unchanged until Root authorizes the reviewed exact workflow change.
