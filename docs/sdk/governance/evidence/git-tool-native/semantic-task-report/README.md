# Common-goal task report failure

The verifier was invoked with `--task-scope`, task and session ID `01a12de5-2e2a-7861-9e97-b45cdb727ed3`, base `f7999ef8adf6069dfd693dabc3f568386a6a2578`, and `--require-change-evidence`. The exact command and working directory are in the run receipt.

The result is exit 1 with 483 errors. The complete stderr log is 125,491 bytes and untruncated. The original stdout file is zero bytes: on failure this verifier prints errors to stderr, not JSON. Two premature/incorrect attempts to parse that empty file produced JSONDecodeError in the task tool trace; those read errors are not verification results. The original file remains empty.

Read-only classification by the shared preflight owner found: 1 missing SDK session-policy binding, 14 oversized file evidence entries, 3 obsolete Java anchors, 80 map dependency references, 379 high-risk evidence requirements, and 6 same-change design requirements. These belong to the common governance closure; this packet does not weaken the checks or claim that the failed report passed. The SDK session snapshot was absent. Reusing the task ID as the session ID did not establish that missing binding, and the top-level preserve decision was not fabricated into an SDK receipt.

The separate real Git native evidence passed 28 calls per language. Final SDK main acceptance still requires the applicable complete gates and governance closure.
