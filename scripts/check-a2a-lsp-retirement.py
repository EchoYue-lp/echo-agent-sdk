#!/usr/bin/env python3
"""Verify Plan04's exact accepted subtraction independently of Rust inventory."""
import argparse
import json
import subprocess
from pathlib import Path

# Published rollback tag; its accepted contracts equal Plan04's local S baseline.
# CI fetches this exact protected tag before running the otherwise shallow job.
BASELINE = "0930eef9f878cf436ed88384030b6bc03d6f6c53"
EXTRAS = {
    ("echo_core::plugin::capability::PluginCapability::LspServer", "variant"),
    ("echo_agent::plugin::prepared::PluginWiringResult::lsp_discovered", "struct_field"),
    ("echo_core::plugin::registry::ResolvedComponents::lsp_config_file", "struct_field"),
}


def source_key(entry):
    return (entry["source_paths"][0], entry["kind"])


def family_entry(entry):
    return any(
        feature in entry["features"]
        or entry["route"]["family"] == feature
        or any(f"::{feature}::" in name for name in [entry["path"], *entry["source_paths"]])
        for feature in ("a2a", "lsp")
    )


def check(root, baseline):
    def old(path):
        return json.loads(subprocess.check_output(["git", "show", f"{baseline}:{path}"], cwd=root))

    def new(path):
        return json.loads((root / path).read_text())

    previous = old("contracts/sdk/accepted-external-contract.json")["entries"]
    current = new("contracts/sdk/accepted-external-contract.json")["entries"]
    old_entries = {source_key(entry): entry for entry in previous}
    new_entries = {source_key(entry): entry for entry in current}
    original = {source_key(entry) for entry in previous if family_entry(entry)}
    assert len(original) == 266
    assert EXTRAS <= old_entries.keys() and not original & EXTRAS
    withdrawn = original | EXTRAS
    assert len(withdrawn) == 269
    assert old_entries.keys() - new_entries.keys() == withdrawn
    assert not new_entries.keys() - old_entries.keys()
    assert len(previous) == 5621 and len(current) == 5352
    # The framework pin is unchanged: any unrelated retained-source drift is
    # an error, not a reason to refresh accepted contracts opportunistically.
    changes = [key for key in new_entries if new_entries[key] != old_entries[key]]
    assert not changes, f"Unexplained retained accepted entry changes: {changes}"
    before = old("contracts/sdk/accepted-facade-operation-catalog.json")
    after = new("contracts/sdk/accepted-facade-operation-catalog.json")
    old_routes = {route["route"] for route in before["routes"]}
    new_routes = {route["route"] for route in after["routes"]}
    retired_routes = {route["route"] for route in before["routes"] if route["family"] in ("a2a", "lsp")}
    assert len(retired_routes) == 23
    assert old_routes - new_routes == retired_routes and not new_routes - old_routes
    assert len(old_routes) == 454 and len(new_routes) == 431
    assert not any(family["family"] in ("a2a", "lsp") for family in after["families"])
    old_operations = {(family["family"], operation) for family in before["families"] for operation in family["operations"]}
    new_operations = {(family["family"], operation) for family in after["families"] for operation in family["operations"]}
    assert len(old_operations - new_operations) == 39 and not new_operations - old_operations
    adr = (root / "docs/adr/0042-sdk-a2a-lsp-retirement-boundary.md").read_text()
    pairs = []
    for section in adr.split("### Retire ")[1:]:
        fields = dict(line[2:].split(": ", 1) for line in section.splitlines() if line.startswith("- ") and ": " in line)
        assert fields["Action"] == "retire" and fields["Replacement"].startswith("unsupported")
        pairs.append((fields["Source"], fields["Kind"]))
    assert len(pairs) == 269 and set(pairs) == withdrawn
    assert 'rev = "f7bac62eee04a963f92a3e4308b6ea2336e91cc0"' in (root / "echo-sdk-host/Cargo.toml").read_text()
    print(json.dumps({"baseline": baseline, "retired_identities": 269, "retained_identities": 5352, "retired_routes": 23, "retained_routes": 431, "retired_operations": 39, "new_acceptances": 0, "retained_entry_drift": 0}, indent=2))


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--baseline", default=BASELINE)
    args = parser.parse_args()
    check(Path(__file__).resolve().parents[1], args.baseline)
