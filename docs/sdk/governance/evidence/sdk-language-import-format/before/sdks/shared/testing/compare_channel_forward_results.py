"""Compare genuine language-consumer receipts; never create Host observations."""
from __future__ import annotations
import argparse
import json
from pathlib import Path


def normalize_message(value: dict) -> dict:
    result = dict(value)
    result.setdefault("attachments", [])
    result.setdefault("reply_to", None)
    result["attachments"] = [dict(attachment, filename=attachment.get("filename"))
                             for attachment in result["attachments"]]
    return result


def normalize_core(value: dict) -> dict:
    return {**value, "single": normalize_message(value["single"]),
            "items": [normalize_message(item) for item in value["items"]]}


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("receipts", nargs="+", type=Path)
    args = parser.parse_args()
    scenario = json.loads(Path(__file__).with_name("channel_forward_scenarios.json").read_text())
    expected = normalize_core(scenario["normalized_core_expected"])
    languages = set()
    observed_invalid = {}
    for path in args.receipts:
        receipt = json.loads(path.read_text())
        language = receipt["language"]
        if language in languages:
            raise ValueError(f"duplicate language receipt: {language}")
        languages.add(language)
        if not receipt.get("host_sha256") or not receipt.get("source_hashes"):
            raise ValueError(f"missing actual binary/source provenance: {path}")
        core = next(row for row in receipt["scenarios"] if row["scenario"] == "single-stream-reply")
        if normalize_core(core["observed"]) != expected:
            raise ValueError(f"native output/count/event mismatch: {path}")
        rows = {row["scenario"]: row for row in receipt["scenarios"]}
        invalid = {}
        for case in scenario["invalid_inbound"]:
            row = rows.get(case["id"])
            if row is None:
                raise ValueError(f"missing native malformed-input receipt: {path}: {case['id']}")
            error_code = row.get("error_code", row.get("error", {}).get("code"))
            if error_code != "invalid_value" or row.get("callback_delta") != 0 or row.get("callback_delta_expected") != 0:
                raise ValueError(f"malformed input failed to preserve error/callback-zero: {path}: {case['id']}")
            invalid[case["id"]] = {"error_code": error_code, "callback_delta": row["callback_delta"]}
        observed_invalid[language] = invalid
        for row in receipt["scenarios"]:
            if row.get("callback_delta_expected") == 0 and row.get("callback_delta") != 0:
                raise ValueError(f"invalid request invoked the native handler: {path}: {row['scenario']}")
    if languages != {"typescript", "python", "java"}:
        raise ValueError(f"comparison requires all three native languages, got {sorted(languages)}")
    values = list(observed_invalid.values())
    if any(value != values[0] for value in values[1:]):
        raise ValueError("native malformed-input error/callback results differ across languages")
    print("three native Channel forward results, exact sequences, terminal, callback counts and twelve invalid-input errors agree")


if __name__ == "__main__":
    main()
