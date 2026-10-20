"""Owned Git repositories and per-call filesystem oracles; never sends ACP traffic."""

from __future__ import annotations

import argparse
import base64
import hashlib
import json
import os
import subprocess
from pathlib import Path

GIT_ENV = {
    "LC_ALL": "C",
    "TZ": "UTC",
    "GIT_AUTHOR_DATE": "2026-10-15T00:00:00+00:00",
    "GIT_COMMITTER_DATE": "2026-10-15T00:00:00+00:00",
}
BASE = "base 中文🙂\n"
DIRTY = "dirty same-name 中文🙂\n"
NEW = "commit payload 中文🙂\n"
SOURCE = "source merged 中文🙂\n"
# Exact positional ABI follows framework tool.parameters() declaration order.
# Catalog declares WireValueArray but does not carry parameter names.
KEYS = {
    "git_status": ["repo_path"],
    "git_diff": ["repo_path", "staged", "target", "file_path", "limit", "cursor"],
    "git_log": ["repo_path", "count", "oneline", "author", "since"],
    "git_blame": [
        "file_path",
        "repo_path",
        "start_line",
        "end_line",
        "limit",
        "cursor",
    ],
    "git_branch": ["repo_path", "name", "switch", "delete"],
    "git_commit": ["repo_path", "message", "files"],
    "enter_worktree": ["branch", "base", "path_suffix", "repo_path"],
    "exit_worktree": ["worktree_path", "merge_to", "repo_path"],
    "list_worktrees": ["repo_path"],
}
PREFIX = "echo_tools::git_worktree::"
CHECKPOINT_PREFIX = "echo_tools::git_checkpoint::"


def git(repo: Path, *args: str) -> str:
    return subprocess.check_output(
        ["git", "-C", str(repo), *args],
        env={**os.environ, **GIT_ENV},
        text=True,
        stderr=subprocess.PIPE,
    ).strip()


def dump(path: Path, value) -> None:
    path.write_text(
        json.dumps(value, ensure_ascii=False, indent=2) + "\n", encoding="utf-8"
    )


def path_value(path: Path):
    return {"kind": "path", "value": {"encoding": "utf8", "path": str(path)}}


def prepare(work: Path) -> None:
    work.mkdir(parents=True, exist_ok=True)
    repo = work / "repo"
    repo.mkdir()
    (work / "not-a-repo").mkdir()
    git(repo, "init", "--initial-branch=main")
    for key, value in {
        "commit.gpgsign": "false",
        "user.name": "SDK Git Fixture",
        "user.email": "sdk-git@example.invalid",
        "core.quotePath": "false",
    }.items():
        git(repo, "config", key, value)
    (repo / "sdk-proof").write_text(BASE, encoding="utf-8")
    (repo / "tracked.txt").write_text(BASE, encoding="utf-8")
    (repo / ".gitignore").write_text(".worktrees/\n", encoding="utf-8")
    git(repo, "add", ".")
    git(repo, "-c", "commit.gpgsign=false", "commit", "-m", "fixture base 中文🙂")
    git(repo, "branch", "sdk-proof")
    (repo / "sdk-proof").write_text(DIRTY, encoding="utf-8")
    # Independent owned repository for the existing per-file checkpoint API.
    checkpoint_repo = work / "checkpoint-repo"
    checkpoint_repo.mkdir()
    git(checkpoint_repo, "init", "--initial-branch=main")
    for key, value in {
        "commit.gpgsign": "false",
        "user.name": "SDK Git Fixture",
        "user.email": "sdk-git@example.invalid",
    }.items():
        git(checkpoint_repo, "config", key, value)
    for name in ["target.txt", "peer.txt"]:
        (checkpoint_repo / name).write_text(BASE, encoding="utf-8")
    git(checkpoint_repo, "add", ".")
    git(
        checkpoint_repo, "-c", "commit.gpgsign=false", "commit", "-m", "checkpoint base"
    )
    rows = []

    def tool(id_, operation, *, checkpoint=None, error=False, **params):
        args = [params.get(key) for key in KEYS[operation]]
        while args and args[-1] is None:
            args.pop()
        rows.append(
            {
                "id": id_,
                "operation": operation,
                "arguments": args,
                "checkpoint": checkpoint,
                "error": error,
                "mode": "tool",
            }
        )

    def source(id_, operation, args, *, checkpoint=None, error=False):
        rows.append(
            {
                "id": id_,
                "operation": PREFIX + operation,
                "arguments": args,
                "checkpoint": checkpoint,
                "error": error,
                "mode": "source",
            }
        )

    tool("same-name-switch", "git_branch", switch="sdk-proof")
    tool("switch-main", "git_branch", switch="main")
    tool("status-dirty", "git_status")
    tool("diff-dirty", "git_diff")
    tool("log-base", "git_log")
    tool("blame-base", "git_blame", file_path="tracked.txt")
    tool("branch-list", "git_branch")
    tool("branch-create", "git_branch", name="unicode-中文")
    tool("branch-delete", "git_branch", delete="unicode-中文")
    tool("invalid-branch", "git_branch", name="invalid branch", error=True)
    tool("missing-switch", "git_branch", switch="missing-ref", error=True)
    tool("non-repo", "git_status", repo_path=str(work / "not-a-repo"), error=True)
    tool(
        "commit-add-fails",
        "git_commit",
        files=["new.txt", "missing.txt"],
        message="must fail",
        checkpoint="prepare-commit",
        error=True,
    )
    tool("commit-staged", "git_commit", message="SDK commit 中文🙂")
    tool("log-commit", "git_log")
    tool("blame-commit", "git_blame", file_path="new.txt")
    managed = repo / ".worktrees/managed-proof"
    tool(
        "managed-enter",
        "enter_worktree",
        branch="managed-proof",
        checkpoint="prepare-managed",
    )
    tool("managed-list", "list_worktrees")
    tool("managed-explicit-cwd", "git_status", repo_path=str(managed))
    tool(
        "managed-dirty-exit",
        "exit_worktree",
        worktree_path=str(managed),
        checkpoint="dirty-managed",
        error=True,
    )
    tool(
        "managed-clean-exit",
        "exit_worktree",
        worktree_path=str(managed),
        checkpoint="clean-managed",
    )
    tool("managed-after-list", "list_worktrees")
    repo_wire = path_value(repo)
    resource = {"$ref": "source-create"}
    source(
        "source-create",
        "create_worktree",
        [
            repo_wire,
            {"branch": "source-proof", "base": "main", "path_suffix": "source-proof"},
        ],
    )
    source("source-list", "list_worktrees", [repo_wire])
    source(
        "source-dirty-remove",
        "remove_worktree",
        [repo_wire, resource],
        checkpoint="dirty-source",
        error=True,
    )
    source(
        "source-merge",
        "merge_worktree",
        [repo_wire, resource, "main"],
        checkpoint="commit-source",
    )
    source("source-remove", "remove_worktree", [repo_wire, resource])
    source("source-after-list", "list_worktrees", [repo_wire])

    def checkpoint_source(
        identifier, operation, arguments, checkpoint=None, error=False
    ):
        rows.append(
            {
                "id": identifier,
                "mode": "checkpoint",
                "operation": CHECKPOINT_PREFIX + operation,
                "arguments": arguments,
                "checkpoint": checkpoint,
                "error": error,
            }
        )

    target = path_value(checkpoint_repo / "target.txt")
    untracked = path_value(checkpoint_repo / "untracked.txt")
    first = {"$ref": "checkpoint-tracked-create"}
    checkpoint_source(
        "checkpoint-tracked-create",
        "create_checkpoint",
        [target],
        checkpoint="prepare-file-checkpoint",
    )
    checkpoint_source(
        "checkpoint-wrong-target",
        "rollback_to_checkpoint",
        [path_value(checkpoint_repo / "peer.txt"), first],
        error=True,
    )
    checkpoint_source(
        "checkpoint-tracked-rollback",
        "rollback_to_checkpoint",
        [target, first],
        checkpoint="mutate-file-checkpoint",
    )
    checkpoint_source("checkpoint-untracked-create", "create_checkpoint", [untracked])
    checkpoint_source(
        "checkpoint-untracked-rollback",
        "rollback_to_checkpoint",
        [untracked, {"$ref": "checkpoint-untracked-create"}],
        checkpoint="mutate-untracked-checkpoint",
    )
    checkpoint_source("checkpoint-keep-one", "cleanup_old_checkpoints", [target, 1])
    checkpoint_source("checkpoint-keep-zero", "cleanup_old_checkpoints", [target, 0])
    dump(
        work / "scenario.json",
        {
            "rows": rows,
            "repo": str(repo),
            "host_env": {**GIT_ENV, "GIT_CEILING_DIRECTORIES": str(work)},
        },
    )
    dump(
        work / "host.json",
        {
            "schema_version": 1,
            "default_agent": {
                "model": {
                    "provider": "fixture",
                    "name": "fixture-model",
                    "base_url": "http://127.0.0.1:9/v1/chat/completions",
                    "api_protocol": "chat_completions",
                    "auth_token": "fixture",
                },
                "agent": {
                    "name": "git-proof",
                    "system_prompt": "fixture",
                    "enable_tools": True,
                    "enable_memory": False,
                    "max_iterations": 2,
                },
            },
            "sdk_profile": {"state_root": str(work / "state")},
        },
    )
    dump(work / "baseline.json", snapshot(work))


def checkpoint(work: Path, name: str) -> None:
    repo = work / "repo"
    if name == "prepare-commit":
        (repo / "new.txt").write_text(NEW, encoding="utf-8")
    elif name == "prepare-managed":
        # Controlled fixture setup only. This is not claimed as Host rollback.
        (repo / "sdk-proof").write_text(BASE, encoding="utf-8")
    elif name in {"dirty-managed", "dirty-source"}:
        branch = "managed-proof" if name == "dirty-managed" else "source-proof"
        (repo / ".worktrees" / branch / "tracked.txt").write_text(
            SOURCE, encoding="utf-8"
        )
    elif name == "clean-managed":
        git(repo / ".worktrees/managed-proof", "restore", "--", "tracked.txt")
    elif name == "commit-source":
        target = repo / ".worktrees/source-proof"
        git(target, "add", "tracked.txt")
        git(
            target, "-c", "commit.gpgsign=false", "commit", "-m", "source commit 中文🙂"
        )
    elif name == "prepare-file-checkpoint":
        target = work / "checkpoint-repo/target.txt"
        target.write_text("checkpoint staged 中文🙂\n", encoding="utf-8")
        git(target.parent, "add", "target.txt")
        target.write_text("checkpoint worktree 中文🙂\n", encoding="utf-8")
        (target.parent / "peer.txt").write_text(
            "peer staged remains\n", encoding="utf-8"
        )
        git(target.parent, "add", "peer.txt")
        (target.parent / "untracked.txt").write_text(
            "untracked original 中文🙂\n", encoding="utf-8"
        )
    elif name == "mutate-file-checkpoint":
        repo = work / "checkpoint-repo"
        (repo / "target.txt").write_text(
            "checkpoint mutation staged\n", encoding="utf-8"
        )
        git(repo, "add", "target.txt")
        (repo / "target.txt").write_text(
            "checkpoint mutation worktree\n", encoding="utf-8"
        )
        (repo / "peer.txt").write_text(
            "peer worktree must survive rollback\n", encoding="utf-8"
        )
    elif name == "mutate-untracked-checkpoint":
        repo = work / "checkpoint-repo"
        (repo / "untracked.txt").write_text(
            "untracked mutation staged\n", encoding="utf-8"
        )
        git(repo, "add", "untracked.txt")
        (repo / "untracked.txt").write_text(
            "untracked mutation worktree\n", encoding="utf-8"
        )
    else:
        raise ValueError(f"unknown owned fixture checkpoint {name}")


def snapshot(work: Path):
    repo = work / "repo"
    values = {}
    for label, directory in [
        ("repo", repo),
        ("managed", repo / ".worktrees/managed-proof"),
        ("source", repo / ".worktrees/source-proof"),
    ]:
        if not directory.exists():
            values[label] = {"exists": False}
            continue
        files = {}
        for name in ["sdk-proof", "tracked.txt", "new.txt", ".gitignore"]:
            file = directory / name
            files[name] = (
                hashlib.sha256(file.read_bytes()).hexdigest()
                if file.is_file()
                else None
            )
        values[label] = {
            "exists": True,
            "head": git(directory, "rev-parse", "HEAD"),
            "branch": git(directory, "symbolic-ref", "--short", "HEAD"),
            "refs": git(directory, "for-each-ref", "--format=%(refname) %(objectname)"),
            "index_entries": git(directory, "ls-files", "--stage"),
            "tree_entries": git(directory, "ls-tree", "-r", "HEAD"),
            "status": git(
                directory, "status", "--porcelain=v1", "--untracked-files=all"
            ),
            "files": files,
            "merge_active": (
                directory / git(directory, "rev-parse", "--git-path", "MERGE_HEAD")
            ).is_file(),
        }
    checkpoint_repo = work / "checkpoint-repo"
    metadata = Path(git(checkpoint_repo, "rev-parse", "--git-path", "echo-checkpoints"))
    if not metadata.is_absolute():
        metadata = checkpoint_repo / metadata
    values["checkpoint_repo"] = {
        "head": git(checkpoint_repo, "rev-parse", "HEAD"),
        "refs": git(
            checkpoint_repo, "for-each-ref", "--format=%(refname) %(objectname)"
        ),
        "index_entries": git(checkpoint_repo, "ls-files", "--stage"),
        "status": git(
            checkpoint_repo, "status", "--porcelain=v1", "--untracked-files=all"
        ),
        "files": {
            name: hashlib.sha256((checkpoint_repo / name).read_bytes()).hexdigest()
            if (checkpoint_repo / name).is_file()
            else None
            for name in ["target.txt", "peer.txt", "untracked.txt"]
        },
        "checkpoints": {
            entry.name: {
                str(file.relative_to(entry)): hashlib.sha256(
                    file.read_bytes()
                ).hexdigest()
                for file in sorted(entry.rglob("*"))
                if file.is_file()
            }
            for entry in sorted(metadata.iterdir())
            if entry.is_dir()
        }
        if metadata.exists()
        else {},
    }
    values["registered_worktrees"] = git(repo, "worktree", "list", "--porcelain")
    return values


def decode(value):
    if isinstance(value, list):
        return [decode(item) for item in value]
    if not isinstance(value, dict) or "kind" not in value:
        return value
    kind = value["kind"]
    if kind == "record":
        return {
            field["name"]: decode(field["value"]) for field in value["value"]["fields"]
        }
    if kind == "map":
        return {decode(item["key"]): decode(item["value"]) for item in value["value"]}
    if kind == "list":
        return [decode(item) for item in value["value"]]
    if kind == "null":
        return None
    return value["value"]


def decode_path(value):
    if value["encoding"] == "utf8":
        return value["path"]
    if value["encoding"] == "unix":
        return os.fsdecode(base64.b64decode(value["bytes_base64"], validate=True))
    if value["encoding"] == "windows":
        return base64.b64decode(value["utf16_base64"], validate=True).decode(
            "utf-16-le"
        )
    raise ValueError("unknown WirePath encoding")


def verify(work: Path):
    data = json.loads((work / "results.json").read_text())
    scenario = json.loads((work / "scenario.json").read_text())
    rows = data["rows"]
    assert [row["id"] for row in rows] == [row["id"] for row in scenario["rows"]]
    mapped = {row["id"]: row for row in rows}
    # Primary regression: a real branch transition cannot restore a same-name file.
    switch = mapped["same-name-switch"]
    assert switch["after"]["repo"]["branch"] == "sdk-proof", (
        "switch did not change HEAD",
        switch,
    )
    assert switch["before"]["repo"]["files"] == switch["after"]["repo"]["files"], (
        "switch overwrote tracked dirty file",
        switch,
    )
    assert (
        switch["before"]["repo"]["index_entries"]
        == switch["after"]["repo"]["index_entries"]
    )
    for step, row in zip(scenario["rows"], rows):
        if step["error"]:
            error = row["error"]
            assert (
                error["code"] == "framework_error"
                and error["retryable"] == "never"
                and error["message"]
            ), row
            assert error["operation"] == step["operation"], row
            assert error["handle"] is None, row
            if step["mode"] == "tool":
                assert error["details"] == {
                    "fields": [
                        {"key": "framework_failure_category", "value": "tool"},
                        {"key": "framework_terminal_kind", "value": "failed"},
                        {"key": "framework_failure_code", "value": "tool"},
                        {"key": "framework_retryable", "value": "false"},
                    ]
                }, row
            elif step["mode"] == "checkpoint":
                assert error["details"] is None, row
                assert (
                    error["message"]
                    == "checkpoint does not belong to the requested file"
                ), row
                assert row["before"] == row["after"], row
            else:
                # The accepted source String error helper has no structured details.
                assert error["details"] is None, row
                assert (
                    error["message"]
                    == "Refusing to remove a worktree with uncommitted changes"
                ), row
        else:
            assert "error" not in row, row
            if step["mode"] == "tool":
                result = row["result"]
                assert result["success"] is True and result["kind"] == {
                    "kind": "text"
                }, row
                assert (
                    isinstance(result["output"], str) and result["truncated"] is False
                ), row
                assert (
                    result.get("effects", []) == []
                    and result.get("model_content", []) == []
                ), row
        if step["id"] in {
            "status-dirty",
            "diff-dirty",
            "log-base",
            "blame-base",
            "branch-list",
            "invalid-branch",
            "missing-switch",
            "non-repo",
            "log-commit",
            "blame-commit",
            "managed-list",
            "managed-explicit-cwd",
            "managed-dirty-exit",
            "managed-after-list",
            "source-list",
            "source-dirty-remove",
            "source-after-list",
        }:
            assert row["before"] == row["after"], row
    assert mapped["switch-main"]["after"]["repo"]["branch"] == "main"
    branches = [
        line.split(" ", 1)[0].removeprefix("refs/heads/")
        for line in mapped["branch-list"]["before"]["repo"]["refs"].splitlines()
    ]
    assert mapped["branch-list"]["result"][
        "output"
    ] == "List all branches:\n" + "\n".join(
        ("* " if branch == "main" else "  ") + branch for branch in branches
    )
    assert mapped["status-dirty"]["result"]["output"] == "Git status:\nM sdk-proof"
    assert DIRTY.strip() in mapped["diff-dirty"]["result"]["output"]
    assert "refs/heads/unicode-中文" in mapped["branch-create"]["after"]["repo"]["refs"]
    assert (
        "refs/heads/unicode-中文"
        not in mapped["branch-delete"]["after"]["repo"]["refs"]
    )
    failed = mapped["commit-add-fails"]
    assert failed["before"]["repo"]["head"] == failed["after"]["repo"]["head"]
    assert failed["before"]["repo"]["files"] == failed["after"]["repo"]["files"]
    # The real tool stages earlier valid paths before a later add fails; no rollback is promised.
    assert (
        "new.txt" not in failed["before"]["repo"]["index_entries"]
        and "new.txt" in failed["after"]["repo"]["index_entries"]
    )
    committed = mapped["commit-staged"]
    head = committed["after"]["repo"]["head"]
    assert head != committed["before"]["repo"]["head"]
    assert committed["before"]["repo"]["files"] == committed["after"]["repo"]["files"]
    assert "M sdk-proof" in committed["after"]["repo"]["status"]
    expected_blob = (
        subprocess.check_output(
            ["git", "-C", str(work / "repo"), "hash-object", "--stdin"],
            input=NEW.encode(),
        )
        .decode()
        .strip()
    )
    assert (
        f"100644 blob {expected_blob}\tnew.txt"
        in committed["after"]["repo"]["tree_entries"]
    )
    assert (
        f"100644 {expected_blob} 0\tnew.txt"
        in committed["after"]["repo"]["index_entries"]
    )
    assert "SDK commit 中文🙂" in mapped["log-commit"]["result"]["output"]
    assert NEW.strip() in mapped["blame-commit"]["result"]["output"]
    entered = mapped["managed-enter"]
    assert entered["after"]["managed"]["exists"]
    assert entered["result"]["metadata"] == {
        "branch": "managed-proof",
        "worktree_path": str(work / "repo/.worktrees/managed-proof"),
    }
    assert entered["before"]["repo"]["head"] == entered["after"]["repo"]["head"]
    assert (
        mapped["managed-explicit-cwd"]["result"]["output"]
        == "Working directory clean, no changes"
    )
    assert (
        mapped["managed-list"]["result"]["output"]
        == f"Worktrees (2):\n  {work / 'repo'} (branch: main)\n  {work / 'repo/.worktrees/managed-proof'} (branch: managed-proof)"
    )
    assert (
        mapped["managed-after-list"]["result"]["output"]
        == f"Worktrees (1):\n  {work / 'repo'} (branch: main)"
    )
    assert not mapped["managed-clean-exit"]["after"]["managed"]["exists"]
    assert (
        "refs/heads/managed-proof"
        in mapped["managed-clean-exit"]["after"]["repo"]["refs"]
    )
    resource = decode(mapped["source-create"]["result"])
    assert set(resource) == {"path", "branch", "managed"}
    assert decode_path(resource["path"]) == str(work / "repo/.worktrees/source-proof")
    assert resource["branch"] == "source-proof" and resource["managed"] is True

    def assert_source_list(raw, expected):
        assert isinstance(raw, list) and len(raw) == len(expected)
        observed = []
        for record in raw:
            assert set(record) == {"kind", "value"} and record["kind"] == "record"
            value = record["value"]
            assert set(value) == {"type_id", "fields"}
            assert value["type_id"] == "echo_tools::git_worktree::ManagedWorktree"
            assert [field["name"] for field in value["fields"]] == [
                "path",
                "branch",
                "managed",
            ]
            assert all(set(field) == {"name", "value"} for field in value["fields"])
            fields = {field["name"]: field["value"] for field in value["fields"]}
            assert fields["path"]["kind"] == "path"
            assert fields["branch"]["kind"] == "string"
            assert fields["managed"]["kind"] == "bool"
            observed.append(
                (
                    decode_path(fields["path"]["value"]),
                    fields["branch"]["value"],
                    fields["managed"]["value"],
                )
            )
        assert observed == expected

    assert_source_list(
        mapped["source-list"]["result"],
        [
            (str(work / "repo"), "main", False),
            (str(work / "repo/.worktrees/source-proof"), "source-proof", True),
        ],
    )
    merged = mapped["source-merge"]
    assert merged["after"]["repo"]["head"] == merged["before"]["source"]["head"]
    assert (
        merged["after"]["repo"]["files"]["tracked.txt"]
        == hashlib.sha256(SOURCE.encode()).hexdigest()
    )
    assert (
        merged["after"]["repo"]["branch"] == "main"
        and not merged["after"]["repo"]["merge_active"]
    )
    assert not mapped["source-remove"]["after"]["source"]["exists"]
    assert "refs/heads/source-proof" in mapped["source-remove"]["after"]["repo"]["refs"]
    assert_source_list(
        mapped["source-after-list"]["result"], [(str(work / "repo"), "main", False)]
    )

    def checkpoint_state(row, side):
        return row[side]["checkpoint_repo"]

    def without_checkpoints(state):
        return {key: value for key, value in state.items() if key != "checkpoints"}

    def index_by_file(state):
        return {
            line.split("\t", 1)[1]: line.split("\t", 1)[0]
            for line in state["index_entries"].splitlines()
        }

    tracked_create = mapped["checkpoint-tracked-create"]
    untracked_create = mapped["checkpoint-untracked-create"]
    first_id = decode(tracked_create["result"])
    second_id = decode(untracked_create["result"])
    assert (
        isinstance(first_id, str)
        and first_id
        and isinstance(second_id, str)
        and second_id != first_id
    )
    for identifier, issued in [
        ("checkpoint-tracked-create", first_id),
        ("checkpoint-untracked-create", second_id),
    ]:
        before, after = (
            checkpoint_state(mapped[identifier], side) for side in ["before", "after"]
        )
        assert without_checkpoints(before) == without_checkpoints(after), identifier
        assert set(after["checkpoints"]) == set(before["checkpoints"]) | {issued}, (
            identifier
        )
        assert set(after["checkpoints"][issued]) == {"manifest.json", "worktree.bin"}, (
            identifier
        )
        for old_id, contents in before["checkpoints"].items():
            assert after["checkpoints"][old_id] == contents, identifier
    for identifier, target, saved in [
        (
            "checkpoint-tracked-rollback",
            "target.txt",
            checkpoint_state(tracked_create, "before"),
        ),
        (
            "checkpoint-untracked-rollback",
            "untracked.txt",
            checkpoint_state(untracked_create, "before"),
        ),
    ]:
        row = mapped[identifier]
        before, after = (checkpoint_state(row, side) for side in ["before", "after"])
        assert row["result"] is None, row
        assert after["files"][target] == saved["files"][target], row
        assert index_by_file(after).get(target) == index_by_file(saved).get(target), row
        assert {k: v for k, v in before["files"].items() if k != target} == {
            k: v for k, v in after["files"].items() if k != target
        }, row
        assert {k: v for k, v in index_by_file(before).items() if k != target} == {
            k: v for k, v in index_by_file(after).items() if k != target
        }, row
        assert after["head"] == before["head"] and after["refs"] == before["refs"], row
        assert after["checkpoints"] == before["checkpoints"], row
    assert "untracked.txt" not in index_by_file(
        checkpoint_state(mapped["checkpoint-untracked-rollback"], "after")
    )
    for identifier, keep in [("checkpoint-keep-one", 1), ("checkpoint-keep-zero", 0)]:
        row = mapped[identifier]
        before, after = (checkpoint_state(row, side) for side in ["before", "after"])
        assert row["result"] is None, row
        assert without_checkpoints(before) == without_checkpoints(after), row
        retained = sorted(before["checkpoints"], reverse=True)[:keep]
        assert after["checkpoints"] == {
            key: before["checkpoints"][key] for key in retained
        }, row
    # Session.invoke supplies this issued owner at the wire boundary. The
    # recorded user arguments must remain unchanged, without a second Session.
    assert data["issued_session"]["kind"] == "session" and data["issued_session"]["id"]
    for step, row in zip(scenario["rows"], rows):
        if step["mode"] == "checkpoint":
            assert len(row["arguments"]) == len(step["arguments"]), row
            assert row["arguments"][0] == step["arguments"][0], row
    # Checkpoint operations have their own result shapes; no ToolResult coercion.
    for row in rows:
        if row["id"].startswith("checkpoint-"):
            assert row["before"]["repo"] == row["after"]["repo"], row

    proof = {
        "tool_operations": sorted(KEYS),
        "source_operations": [
            PREFIX + name
            for name in [
                "create_worktree",
                "list_worktrees",
                "merge_worktree",
                "remove_worktree",
            ]
        ],
        "checkpoint_operations": [
            CHECKPOINT_PREFIX + name
            for name in [
                "create_checkpoint",
                "rollback_to_checkpoint",
                "cleanup_old_checkpoints",
            ]
        ],
        "checkpoint_calls": 7,
        "native_calls": len(rows),
        "groups": [
            "branch-and-read-safety",
            "commit-and-partial-stage",
            "managed-worktree-and-source-merge",
        ],
        "host_sha256": data["host_sha256"],
        "model_run_started_by_test": False,
        "limitations": [
            "no claim of index rollback after a partial stage failure",
            "no remote push",
            "no concurrent branch mutation",
            "merge-conflict recovery covered by framework tests, not this native corpus",
        ],
    }
    dump(work / "verified.json", proof)
    return proof


def compare(paths):
    normalized = []
    for path in paths:
        data = json.loads(Path(path).read_text())
        work = data["work"]
        checkpoint_ids = {
            decode(row["result"]): f"<checkpoint-{index}>"
            for index, row in enumerate(
                [
                    row
                    for row in data["rows"]
                    if row["id"]
                    in {"checkpoint-tracked-create", "checkpoint-untracked-create"}
                ],
                1,
            )
        }

        def normalize(value, owned_root=work, checkpoint_ids=checkpoint_ids):
            if isinstance(value, str):
                return checkpoint_ids.get(
                    value, value.replace(owned_root, "<owned-root>")
                )
            if isinstance(value, list):
                return [normalize(item) for item in value]
            if isinstance(value, dict):
                return {
                    checkpoint_ids.get(key, key): normalize(item)
                    for key, item in value.items()
                }
            return value

        normalized.append(
            {"host_sha256": data["host_sha256"], "rows": normalize(data["rows"])}
        )
    assert len(normalized) == 3 and normalized[0] == normalized[1] == normalized[2], (
        "native Git result/error/effect corpus differs"
    )
    return {
        "languages": 3,
        "native_calls_each": len(normalized[0]["rows"]),
        "normalization": "owned-root paths and the two actual random checkpoint IDs only; all response/error/ref/index/tree/file/checkpoint-byte fields retained",
    }


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--work", type=Path)
    parser.add_argument("--compare", nargs=3)
    parser.add_argument("--prepare", action="store_true")
    parser.add_argument("--checkpoint")
    parser.add_argument("--snapshot", action="store_true")
    parser.add_argument("--verify", action="store_true")
    args = parser.parse_args()
    if args.compare:
        print(json.dumps(compare(args.compare)))
    elif args.work is None:
        parser.error("--work required for fixture operations")
    elif args.prepare:
        prepare(args.work)
    elif args.checkpoint:
        checkpoint(args.work, args.checkpoint)
    elif args.snapshot:
        print(json.dumps(snapshot(args.work), ensure_ascii=False))
    elif args.verify:
        print(json.dumps(verify(args.work), ensure_ascii=False))
    else:
        parser.error("choose a fixture operation")


if __name__ == "__main__":
    main()
