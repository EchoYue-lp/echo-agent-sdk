"""Compare full publication responses/errors/index fields with reversible ID maps.

Per-language original observations preserve receipts, timing, callbacks, requests
and every raw notification. This comparison does not equate timing/provider
context strings across different processes and does not remove response fields.
"""

import json
from pathlib import Path

ROOT = Path(__file__).resolve().parent
results = {}
for language in ("typescript", "python", "java"):
    facts = json.loads((ROOT / language / "observed.json").read_text())
    groups = facts["groups"]
    mapping = {}

    def bind(value, name):
        assert value not in mapping or mapping[value] == name
        mapping[value] = name

    for key, owner in (("agent", "$agent-a"), ("other_agent", "$agent-b")):
        bind(facts[key]["id"], owner)
    for response, prefix in ((groups["session"]["first"], "$session-a"), (groups["owner"]["session"], "$session-b")):
        bind(response["session"]["id"], prefix)
        bind(response["task_run"]["id"], prefix + ".task-run")
        bind(response["acp_session_id"], prefix + ".acp")
    for response, name in ((groups["run"]["first"], "$run-a"), (groups["owner"]["run"], "$run-b"), (groups["busy"]["original"], "$busy-original"), (groups["busy"]["retry"], "$busy-retry")):
        bind(response["run"]["id"], name)
        bind(response["stream"]["id"], name + ".stream")
    for row in facts["session_index"]:
        bind(row["cwd"]["path"], "$language-process-cwd")

    def normalize(value):
        if isinstance(value, str):
            return mapping.get(value, value)
        if isinstance(value, list):
            return [normalize(item) for item in value]
        if isinstance(value, dict):
            return {key: normalize(item) for key, item in value.items()}
        return value

    core = {
        "session_responses": {key: groups["session"][key] for key in ("first", "duplicate", "concurrent")},
        "run_responses": {key: groups["run"][key] for key in ("first", "concurrent", "completed", "after_local_close")},
        "independent_owner_session_response": groups["owner"]["session"],
        "independent_owner_run_response": groups["owner"]["run"],
        "busy_original_response": groups["busy"]["original"],
        "busy_retry_response": groups["busy"]["retry"],
        "errors": {
            "session_changed": groups["session"]["changed"],
            "run_changed": groups["run"]["changed"],
            "busy": groups["busy"]["rejection"],
            "session_closed": groups["closed"]["session"],
            "run_closed": groups["closed"]["run"],
        },
        "original_session_index": facts["session_index"],
        "counts": {
            "session_after_duplicate": groups["session"]["creations"],
            "first_run_provider_calls": groups["run"]["provider_calls"],
            "owner_run_provider_calls": groups["owner"]["provider_calls"],
            "busy_rejected_provider_calls": groups["busy"]["provider_calls_while_rejected"],
            "busy_retry_provider_calls": groups["busy"]["provider_calls_after_retry"],
            "total_provider_calls": facts["provider_calls"],
            "total_provider_finally": facts["provider_finally"],
            "session_index_count": facts["session_index_count"],
            "run_notices_before_retry": groups["run"]["notifications_before_retry"],
            "run_notices_after_retry": groups["run"]["notifications_after_retry"],
        },
    }
    normalized = normalize(core)
    results[language] = {"value_mapping": mapping, "normalized_full_publication_core": normalized}
    (ROOT / language / "normalized.json").write_text(json.dumps(results[language], indent=2) + "\n")
assert results["typescript"]["normalized_full_publication_core"] == results["python"]["normalized_full_publication_core"] == results["java"]["normalized_full_publication_core"]
(ROOT / "comparison.json").write_text(json.dumps({"exact_normalized_full_publication_core_equal": True, "normalization": "only reversible identity and actual per-language process-cwd substitutions; keys, fields, scalar types, generations and error structure preserved", "languages": results, "raw_fields_retained_outside_equality": ["measured settlement timing", "provider full request/context", "individual notification timestamps/content_hash/event_id", "all raw requests/responses"]}, indent=2) + "\n")
print("Three native publication responses, original index fields, complete typed errors and effect counters match.")
