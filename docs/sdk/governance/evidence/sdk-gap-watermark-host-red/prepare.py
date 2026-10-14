"""Make an isolated, hashed SDK workspace; restore only the old ACK cursor defect."""
import difflib
import hashlib
import json
import pathlib
import shutil
import subprocess

BASE = pathlib.Path(__file__).resolve().parent
SDK = BASE.parents[2] / ".worktrees/echo-agent-sdk-worktrees/sdk-semantic-governance"
FRAMEWORK = BASE.parents[2] / ".worktrees/echo-agent-worktrees/channel-handler-forward"
SOURCE = BASE / "source"
EVENTS = "echo-sdk-host/src/core_profile/events.rs"


def sha(data):
    return hashlib.sha256(data).hexdigest()


def selected_files(root, predicate):
    paths = subprocess.check_output(
        ["git", "ls-files", "-co", "--exclude-standard", "-z"], cwd=root
    ).decode().split("\0")
    return sorted(set(p for p in paths if p and predicate(p) and (root / p).is_file()))


def main():
    if SOURCE.exists():
        raise SystemExit("Refusing to replace an existing experimental source.")
    paths = selected_files(
        SDK,
        lambda p: p in ("Cargo.toml", "Cargo.lock", "rust-toolchain.toml", "rust-toolchain")
        or p.startswith((".cargo/", "echo-sdk-host/", "echo-sdk-protocol/", "contracts/", "sdks/shared/")),
    )
    records = []
    for relative in paths:
        data = (SDK / relative).read_bytes()
        target = SOURCE / relative
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_bytes(data)
        records.append({"path": relative, "bytes": len(data), "sha256": sha(data)})
    framework_files = selected_files(
        FRAMEWORK, lambda p: p.endswith(".rs") or pathlib.PurePosixPath(p).name in ("Cargo.toml", "Cargo.lock")
    )
    framework_records = [
        {"path": p, "sha256": sha((FRAMEWORK / p).read_bytes())}
        for p in framework_files
    ]
    fixed = (SOURCE / EVENTS).read_text()
    old_line = "        self.last_sent_sequence = self.last_sent_sequence.max(acked);\n"
    if fixed.count(old_line) != 1:
        raise SystemExit("ACK cursor update must match exactly once.")
    regressed = fixed.replace(old_line, "", 1)
    (BASE / "events-fixed.rs").write_text(fixed)
    (BASE / "events-regressed.rs").write_text(regressed)
    (SOURCE / EVENTS).write_text(regressed)
    (BASE / "watermark-only-regression.patch").write_text(
        "".join(difflib.unified_diff(fixed.splitlines(True), regressed.splitlines(True),
                                     fromfile="a/" + EVENTS, tofile="b/" + EVENTS))
    )
    historical = subprocess.check_output(
        ["git", "show", "30d7e219cc7d3aef0f75a19217c82898c363f50d:" + EVENTS], cwd=SDK
    )
    (BASE / "events-30d7e219.rs").write_bytes(historical)
    manifest = {
        "sdkRoot": str(SDK), "sdkHead": subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=SDK).decode().strip(),
        "sdkCopy": str(SOURCE), "classification": "single-variable historical watermark mechanism regression on current SDK source",
        "isFullHistoricalSdkReproduction": False, "sdkFiles": records,
        "frameworkRoot": str(FRAMEWORK), "frameworkHead": subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=FRAMEWORK).decode().strip(),
        "frameworkFiles": framework_records,
        "fixedEventsSha256": sha(fixed.encode()), "regressedEventsSha256": sha(regressed.encode()),
        "historicalEventsSha256": sha(historical),
        "onlyMutation": {"path": EVENTS, "deletedLine": old_line.strip()},
    }
    (BASE / "source-manifest.json").write_text(json.dumps(manifest, indent=2) + "\n")
    print(json.dumps({"copiedFiles": len(records), "copiedBytes": sum(p["bytes"] for p in records),
                      "frameworkFiles": len(framework_records), "fixedEventsSha256": manifest["fixedEventsSha256"],
                      "regressedEventsSha256": manifest["regressedEventsSha256"]}))


if __name__ == "__main__":
    main()
