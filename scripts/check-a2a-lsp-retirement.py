#!/usr/bin/env python3
"""Verify Plan04's exact accepted subtraction independently of Rust inventory."""
import argparse
import json
import re
import subprocess
from pathlib import Path

# Published rollback tag; its accepted contracts equal Plan04's local S baseline.
# CI fetches this exact protected tag before running the otherwise shallow job.
BASELINE = "0930eef9f878cf436ed88384030b6bc03d6f6c53"
FRAMEWORK_REVISION = "0087353fd78eb73ffaa68b255cf4ef807ca96d4a"
FRAMEWORK_URL = "https://github.com/EchoYue-lp/echo-agent.git"
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


def check_framework_manifests(root):
    # 检查本仓既有inline声明形状；不接受任意revision或把file传输写成源码URL。
    def dependency(path, table, name):
        text = (root / path).read_text()
        sections = re.findall(
            rf"(?ms)^\[{re.escape(table)}\][ \t]*\n(.*?)(?=^\[|\Z)", text
        )
        assert len(sections) == 1, f"Missing or duplicate {table}: {path}"
        declarations = re.findall(
            rf"(?m)^{re.escape(name)}[ \t]*=[ \t]*\{{([^{{}}\n]+)\}}[ \t]*$",
            sections[0],
        )
        assert len(declarations) == 1, f"Missing or duplicate {name}: {path}"
        fields = re.findall(r'\b(git|rev)[ \t]*=[ \t]*"([^"]+)"', declarations[0])
        assert sorted(fields) == [("git", FRAMEWORK_URL), ("rev", FRAMEWORK_REVISION)], (
            f"{path}: {name} must use canonical URL and exact {FRAMEWORK_REVISION}"
        )

    dependency("echo-sdk-host/Cargo.toml", "dependencies", "echo_agent")
    for package in ("polars-arrow", "polars-parquet"):
        dependency("Cargo.toml", "patch.crates-io", package)


def check(root, baseline):
    check_framework_manifests(root)
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
    # 来源已独立升级到精确008；退役集合和保留accepted行仍须完全相同，
    # 不能借pin变更顺带接纳新API或刷新无关签名。
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
    print(json.dumps({"baseline": baseline, "retired_identities": 269, "retained_identities": 5352, "retired_routes": 23, "retained_routes": 431, "retired_operations": 39, "new_acceptances": 0, "retained_entry_drift": 0}, indent=2))


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--baseline", default=BASELINE)
    args = parser.parse_args()
    check(Path(__file__).resolve().parents[1], args.baseline)
