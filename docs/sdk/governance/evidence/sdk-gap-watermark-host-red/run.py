"""Run only after the coordinator grants the exclusive shared Cargo lease."""
import datetime
import hashlib
import json
import os
import pathlib
import shutil
import subprocess
import sys

BASE = pathlib.Path(__file__).resolve().parent
MANIFEST = json.loads((BASE / "source-manifest.json").read_text())
SOURCE = BASE / "source"
EVENTS = "echo-sdk-host/src/core_profile/events.rs"
TARGET = BASE.parents[2] / ".worktrees/echo-agent-sdk-worktrees/sdk-final-framework-pin/target"


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def check_sources():
    framework = pathlib.Path(MANIFEST["frameworkRoot"])
    changed = [r["path"] for r in MANIFEST["frameworkFiles"] + MANIFEST.get("frameworkAuxiliaryFiles", [])
               if digest(framework / r["path"]) != r["sha256"]]
    if changed:
        raise SystemExit("Framework source changed: " + ", ".join(changed))
    sdk_events = pathlib.Path(MANIFEST["sdkRoot"]) / EVENTS
    if digest(sdk_events) != MANIFEST["fixedEventsSha256"]:
        raise SystemExit("Production SDK events changed during reproduction.")
    observations = MANIFEST.get("fixtureObservation")
    for record in MANIFEST["sdkFiles"]:
        if record["path"] == EVENTS:
            continue
        expected = record["sha256"]
        if observations and record["path"] == observations["path"]:
            expected = observations["observedSha256"]
        if digest(SOURCE / record["path"]) != expected:
            raise SystemExit("Experimental SDK source changed: " + record["path"])


def run(phase, case):
    check_sources()
    expected = MANIFEST["regressedEventsSha256" if phase == "red" else "fixedEventsSha256"]
    if digest(SOURCE / EVENTS) != expected:
        raise SystemExit("Unexpected experimental events source for " + phase)
    command = ["cargo", "test", "-p", "echo-sdk-host", "--all-features", "--locked"]
    if case == "host":
        command += ["--test", "core_profile_e2e", "ack_window_snapshot_ack_resumes_strictly_after_watermark", "--", "--exact", "--nocapture"]
    elif case == "unit":
        command += ["--lib", "core_profile::events::tests::byte_bound_gap_retires_only_after_snapshot_ack", "--", "--exact", "--nocapture"]
    else:
        raise SystemExit("case must be host or unit")
    env = os.environ.copy()
    changes = {"CARGO_TARGET_DIR": str(TARGET), "CARGO_BUILD_JOBS": "2", "CARGO_PROFILE_DEV_DEBUG": "0", "CARGO_PROFILE_TEST_DEBUG": "0", "CARGO_INCREMENTAL": "0"}
    env.update(changes)
    start = datetime.datetime.now(datetime.timezone.utc).isoformat()
    log = BASE / (phase + "-" + case + ".log")
    if log.exists():
        raise SystemExit("Refusing to replace raw log " + str(log))
    with log.open("wb") as output:
        process = subprocess.run(command, cwd=SOURCE, env=env, stdout=output, stderr=subprocess.STDOUT)
    check_sources()
    receipt = {"phase": phase, "case": case, "command": command, "cwd": str(SOURCE), "environment": changes,
               "startedAt": start, "finishedAt": datetime.datetime.now(datetime.timezone.utc).isoformat(),
               "exitCode": process.returncode, "eventsSha256": expected, "logSha256": digest(log),
               "productionSdkEventsUnchanged": True, "frameworkSourcesUnchanged": True}
    (BASE / (phase + "-" + case + "-receipt.json")).write_text(json.dumps(receipt, indent=2) + "\n")
    print(json.dumps(receipt))


if __name__ == "__main__":
    if len(sys.argv) == 2 and sys.argv[1] == "restore":
        check_sources()
        shutil.copyfile(BASE / "events-fixed.rs", SOURCE / EVENTS)
        print("Only experimental ACK cursor line restored.")
    elif len(sys.argv) == 3:
        run(sys.argv[1], sys.argv[2])
    else:
        raise SystemExit("run.py red|green host|unit, or run.py restore; exclusive Cargo lease required")
