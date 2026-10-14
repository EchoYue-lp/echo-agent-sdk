"""Inputs/oracle only. Native SDKs own every Host operation and callback."""

from __future__ import annotations

import argparse
import base64
import hashlib
import json
import os
import re
import sqlite3
import threading
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path

HTML = '<html><head><title>中文🙂 Title</title></head><body><h1>中文🙂 Heading</h1><p>段落 beta🙂</p><a href="/target">链接🙂</a><table><tr><td>one</td></tr></table></body></html>'
PAYLOAD = "中文🙂\nAβEND\nZ1234"
PROTECTED = ["sentinel.txt", "fixture.db", "src/main.py"]


def digest(data):
    return hashlib.sha256(data).hexdigest()


def snapshot(work):
    return {name: digest((work / name).read_bytes()) for name in PROTECTED}


def record(type_id, **values):
    def wire(value):
        if value is None:
            return {"kind": "null"}
        if isinstance(value, bool):
            return {"kind": "bool", "value": value}
        if isinstance(value, int):
            return {"kind": "u64", "value": str(value)}
        if isinstance(value, str):
            return {"kind": "string", "value": value}
        return value

    return {
        "kind": "record",
        "value": {
            "type_id": type_id,
            "fields": [{"name": k, "value": wire(v)} for k, v in values.items()],
        },
    }


def prepare(work, url):
    work.mkdir(parents=True, exist_ok=True)
    (work / "src").mkdir()
    (work / "artifact-temp").mkdir()
    (work / "src/main.py").write_text(
        'def target_symbol():\n    return "中文🙂"\n', encoding="utf8"
    )
    (work / "sentinel.txt").write_text("protected 中文🙂\n", encoding="utf8")
    (work / "tail.txt").write_text("tail-ready 中文🙂\n", encoding="utf8")
    with sqlite3.connect(work / "fixture.db") as db:
        db.execute(
            "CREATE TABLE records(id INTEGER PRIMARY KEY, label TEXT NOT NULL, quantity INTEGER DEFAULT 7, note TEXT)"
        )
        db.executemany(
            "INSERT INTO records VALUES(?,?,?,?)",
            [(1, "中文🙂", 3, None), (2, "beta", 7, "note"), (3, "gamma", 9, "")],
        )
    rows = []

    def add(id, op, args, expected="success", **extra):
        rows.append(
            {
                "id": id,
                "operation": op.split(".", 1)[1]
                if op.split(".", 1)[0]
                in {"files", "web", "shell", "database", "research"}
                else op,
                "arguments": args,
                "expected": expected,
                **extra,
            }
        )

    add("file-create", "files.create_file", ["中文🙂/note.txt"])
    add("file-append", "files.append_file", ["中文🙂/note.txt", "one 中文🙂\n"])
    add("file-update", "files.update_file", ["中文🙂/note.txt", "one", "two"])
    add(
        "file-update-missing",
        "files.update_file",
        ["中文🙂/note.txt", "absent", "bad"],
        "tool_failure",
    )
    add(
        "file-create-existing", "files.create_file", ["中文🙂/note.txt"], "tool_failure"
    )
    add(
        "file-write-stale",
        "files.write_file",
        ["sentinel.txt", "bad", "0" * 64],
        "tool_failure",
    )
    add("file-move", "files.move_file", ["中文🙂/note.txt", "中文🙂/moved.txt"])
    add(
        "file-patch",
        "files.apply_patch",
        [
            "*** Begin Patch\n*** Update File: 中文🙂/moved.txt\n@@\n-two 中文🙂\n+three 中文🙂\n*** End Patch"
        ],
    )
    add(
        "file-patch-rollback",
        "files.apply_patch",
        [
            "*** Begin Patch\n*** Update File: 中文🙂/moved.txt\n@@\n-three 中文🙂\n+bad\n*** Update File: missing.txt\n@@\n-absent\n+bad\n*** End Patch"
        ],
        "tool_failure",
    )
    add("file-read", "files.read_file", ["中文🙂/moved.txt"])
    add("file-diff", "files.diff", ["中文🙂/moved.txt", None, "four 中文🙂\n"])
    add("file-list", "files.list_dir", ["src"])
    add("file-grep", "files.grep", ["中文🙂", "src", "*.py"])
    add("file-glob", "files.glob", ["*.py", "src"])
    add("file-map", "files.repo_map", ["src", 3, "symbols"])
    add("file-search", "files.code_search", ["target_symbol", "src", "*.py"])
    add("file-missing", "files.read_file", ["missing.txt"], "tool_failure")
    add("file-delete", "files.delete_file", ["中文🙂/moved.txt"])
    add(
        "file-delete-missing", "files.delete_file", ["中文🙂/moved.txt"], "tool_failure"
    )
    path = {
        "kind": "path",
        "value": {"encoding": "utf8", "path": str(work / "artifact-temp")},
    }
    config = record(
        "echo_core::tools::artifact::ToolOutputArtifactConfig",
        root_dir=path,
        retention="temporary_1h",
        threshold_bytes=1,
        max_age_secs=None,
    )
    ident = record(
        "echo_core::tools::artifact::ToolOutputArtifactIdentity",
        conversation_id="tool-proof",
        run_id=None,
        call_id="utf8",
        tool_name="tool-proof",
    )
    limit = {
        "kind": "variant",
        "value": {
            "type_id": "echo_tools::files::artifact::ArtifactPageLimit",
            "variant": "bytes",
            "fields": [{"name": "value", "value": {"kind": "u64", "value": "8"}}],
        },
    }
    add(
        "artifact-persist",
        "echo_core::tools::artifact::persist_tool_output",
        [config, ident, PAYLOAD],
        expected="value",
    )
    for n in range(4):
        cursor = None if n == 0 else {"$ref": f"artifact-page-{n - 1}.next_cursor"}
        add(
            f"artifact-page-{n}",
            "echo_tools::files::artifact::read_artifact_page",
            [config, {"$artifact": "artifact-persist"}, cursor, limit],
            expected="value",
        )
    add(
        "artifact-bad-cursor",
        "echo_tools::files::artifact::read_artifact_page",
        [config, {"$artifact": "artifact-persist"}, "malformed", limit],
        "rpc_error",
    )
    rows.append(
        {
            "id": "artifact-mutate",
            "action": "mutate_artifact",
            "ref": "artifact-persist",
        }
    )
    add(
        "artifact-stale",
        "echo_tools::files::artifact::read_artifact_page",
        [
            config,
            {"$artifact": "artifact-persist"},
            {"$ref": "artifact-page-0.next_cursor"},
            limit,
        ],
        "rpc_error",
    )
    add("shell-unicode", "shell.shell", ["printf '中文🙂\\n'"])
    add("shell-stderr", "shell.shell", ["ls missing-shell-proof"], "tool_failure")
    add(
        "shell-timeout",
        "shell.shell",
        [f"tail -f {work}/tail.txt", 1],
        "tool_failure",
        watch_process=True,
    )
    add(
        "shell-signal",
        "shell.shell",
        [f"tail -f {work}/tail.txt", 5],
        "tool_failure",
        watch_process=True,
        terminate_observed=True,
    )
    add("shell-policy", "shell.shell", ["python3 blocked.py"], "tool_failure")
    add("web-fetch", "web.web_fetch", [url + "/html"])
    add("web-truncated", "web.web_fetch", [url + "/large", 8])
    add("web-extract", "web.web_extract", [HTML, "all"])
    add("web-http-error", "web.web_fetch", [url + "/503"], "tool_failure")
    add(
        "web-closed-port",
        "web.web_fetch",
        ["http://127.0.0.1:1/unreachable"],
        "tool_failure",
    )
    db = "sqlite://" + str(work / "fixture.db")
    query = "SELECT id,label,quantity,note FROM records ORDER BY id"
    add("db-tables", "database.list_tables", [db])
    add("db-describe", "database.describe_table", [db, "records"])
    add("db-page-0", "database.sql_query", [db, query, 2])
    add(
        "db-page-1",
        "database.sql_query",
        [db, query, 2, {"$jsonref": "db-page-0.page.next_cursor"}],
    )
    add(
        "db-stale-cursor",
        "database.sql_query",
        [db, query + " DESC", 2, {"$jsonref": "db-page-0.page.next_cursor"}],
        "tool_failure",
    )
    add("db-bad-url", "database.sql_query", ["wrong://db", query], "rpc_error")
    add(
        "db-missing",
        "database.sql_query",
        ["sqlite://" + str(work / "missing.db"), query],
        "tool_failure",
    )
    add(
        "db-sql-error",
        "database.sql_query",
        [db, "SELECT unknown FROM records"],
        "tool_failure",
    )
    for op in [
        "arxiv_search",
        "semantic_scholar_search",
        "pubmed_search",
        "clinical_trials_search",
    ]:
        add(
            "research-" + op,
            "research." + op,
            ["agent artificial intelligence", 1],
            expected="external",
        )
    (work / "scenario.json").write_text(
        json.dumps({"rows": rows}, ensure_ascii=False), encoding="utf8"
    )
    (work / "host.json").write_text(
        json.dumps(
            {
                "schema_version": 1,
                "default_agent": {
                    "model": {
                        "provider": "fixture",
                        "name": "fixture-model",
                        "base_url": url + "/v1/chat/completions",
                        "api_protocol": "chat_completions",
                        "auth_token": "fixture-secret",
                    },
                    "agent": {
                        "name": "tools-proof",
                        "system_prompt": "fixture",
                        "enable_tools": True,
                        "enable_memory": False,
                        "max_iterations": 2,
                    },
                },
                "sdk_profile": {"state_root": str(work / "state")},
            }
        )
    )
    (work / "baseline.json").write_text(json.dumps(snapshot(work)))


def decode_path(value):
    if value["encoding"] == "utf8":
        return Path(value["path"])
    if value["encoding"] == "unix":
        return Path(os.fsdecode(base64.b64decode(value["bytes_base64"])))
    return Path(base64.b64decode(value["utf16_base64"]).decode("utf-16le"))


def artifact_wire(value):
    # Public persist operation decodes Record to an opaque tagged record in
    # all clients. Keep it explicit when passing it back to Rust.
    if value.get("kind") == "record":
        return value
    fields = []
    for k, v in value.items():
        if k == "path":
            w = {"kind": "path", "value": v}
        elif k in ("artifact_bytes", "payload_bytes"):
            w = {"kind": "u64", "value": str(v)}
        else:
            w = {"kind": "string", "value": v}
        fields.append({"name": k, "value": w})
    return {
        "kind": "record",
        "type_id": "echo_core::tools::artifact::ToolOutputArtifactRef",
        "fields": fields,
    }


def unrecord(value):
    if isinstance(value, dict) and value.get("kind") == "record":
        return {f["name"]: f["value"].get("value") for f in value["value"]["fields"]}
    return value


def resolve(value, results):
    if isinstance(value, list):
        return [resolve(v, results) for v in value]
    if isinstance(value, dict):
        if "$artifact" in value:
            return artifact_wire(results[value["$artifact"]]["result"])
        key = "$ref" if "$ref" in value else "$jsonref" if "$jsonref" in value else None
        if key:
            name, *path = value[key].split(".")
            v = results[name]["result"]
            if key == "$jsonref":
                v = json.JSONDecoder().raw_decode(v["output"])[0]
            v = unrecord(v)
            for p in path:
                v = v[p]
            return v
        return {k: resolve(v, results) for k, v in value.items()}
    return value


_EXTERNAL_OPERATIONS = frozenset(
    {
        "arxiv_search",
        "semantic_scholar_search",
        "pubmed_search",
        "clinical_trials_search",
    }
)
_OFFICIAL_ACP_TIMEOUT = re.compile(
    r"Did not observe any item or terminal signal within ([1-9][0-9]*)ms in 'source\(MonoDeferContextual\)' \(and no fallback has been configured\)"
)


def _verify_external_error(row, step):
    """Classify the original bounded external observation, never arbitrary transport errors."""
    assert (
        step["expected"] == "external" and step["operation"] in _EXTERNAL_OPERATIONS
    ), step
    assert "result" not in row and isinstance(row.get("error"), dict), row
    error = row["error"]
    assert isinstance(error.get("message"), str) and error["message"], row
    assert (
        error.get("retryable") == "never"
        and "handle" in error
        and error["handle"] is None
    ), row
    if error.get("code") == "framework_error":
        assert error.get("operation") == step["operation"], row
        assert isinstance(error.get("details"), dict), row
        fields = error["details"].get("fields")
        assert isinstance(fields, list) and len(fields) == 4, row
        assert {item["key"]: item["value"] for item in fields} == {
            "framework_failure_category": "tool",
            "framework_terminal_kind": "failed",
            "framework_failure_code": "tool",
            "framework_retryable": "false",
        }, row
        return "framework_error"
    assert (
        error.get("code") == "transport_error"
        and error.get("operation") == "_echo_agent/research/op"
    ), row
    assert "details" in error and error["details"] is None, row
    match = _OFFICIAL_ACP_TIMEOUT.fullmatch(error["message"])
    assert match is not None, row
    assert int(match.group(1)) == 30000, row
    return "transport_timeout"


def verify(work, data):
    rows = data["rows"]
    expected = json.loads((work / "scenario.json").read_text())["rows"]
    assert [r["id"] for r in rows] == [r["id"] for r in expected]
    baseline = json.loads((work / "baseline.json").read_text())
    byid = {r["id"]: r for r in rows}
    for row, step in zip(rows, expected):
        assert row["protected_before"] == row["protected_after"] == baseline, row["id"]
        if step.get("action"):
            continue
        expectation = step["expected"]
        if expectation == "rpc_error":
            e = row["error"]
            assert (
                e["code"] == "framework_error"
                and e["message"]
                and e["retryable"] == "never"
            ), row
            assert e["operation"] == step["operation"] and e["handle"] is None, e
            if step["id"] == "db-bad-url":
                assert {
                    item["key"]: item["value"] for item in e["details"]["fields"]
                } == {
                    "framework_failure_category": "tool",
                    "framework_terminal_kind": "failed",
                    "framework_failure_code": "tool",
                    "framework_retryable": "false",
                }, e
            else:
                assert e["details"] is None, e
        elif expectation == "external":
            if "error" in row:
                _verify_external_error(row, step)
            else:
                assert isinstance(row["result"]["output"], str) and isinstance(
                    row["result"]["success"], bool
                ), row
        else:
            assert "error" not in row, row
            if expectation != "value":
                assert row["result"]["success"] == (expectation == "success"), row
                assert isinstance(row["result"]["output"], str) and isinstance(
                    row["result"]["kind"], dict
                ), row
                if expectation == "tool_failure":
                    assert (
                        row["result"]["error"]
                        and row["result"]["failure"]["category"]
                        and row["result"]["failure"]["recovery"]
                        and row["result"]["failure"]["side_effect"]
                    ), row
                if "data" in row["result"]:
                    from tool_behavior_fixture import decode_typed_data

                    projected = decode_typed_data(row["result"]["data"])
                    if row["result"]["kind"]["kind"] == "json":
                        assert (
                            projected
                            == json.JSONDecoder().raw_decode(row["result"]["output"])[0]
                        ), row

    def out(name):
        return byid[name]["result"]["output"]

    def parsed(name):
        return json.JSONDecoder().raw_decode(out(name))[0]

    assert (
        not (work / "中文🙂/note.txt").exists()
        and not (work / "中文🙂/moved.txt").exists()
    )
    assert "three 中文🙂" in out("file-read")
    assert "-three 中文🙂" in out("file-diff") and "+four 中文🙂" in out("file-diff")
    for name in ["file-list", "file-grep", "file-glob", "file-map", "file-search"]:
        assert "main.py" in out(name), (name, out(name))
    assert "target_symbol" in out("file-search") and "中文🙂" in out("file-grep")
    states = {
        "file-create": {"中文🙂/note.txt": "", "中文🙂/moved.txt": None},
        "file-append": {"中文🙂/note.txt": "one 中文🙂\n", "中文🙂/moved.txt": None},
        "file-update": {"中文🙂/note.txt": "two 中文🙂\n", "中文🙂/moved.txt": None},
        "file-move": {"中文🙂/note.txt": None, "中文🙂/moved.txt": "two 中文🙂\n"},
        "file-patch": {"中文🙂/note.txt": None, "中文🙂/moved.txt": "three 中文🙂\n"},
        "file-delete": {"中文🙂/note.txt": None, "中文🙂/moved.txt": None},
    }
    for name, state in states.items():
        assert byid[name]["files_after"] == state, (name, byid[name])
        effects = byid[name]["result"].get("effects", [])
        assert effects and all(e["effect"] == "file_edit" for e in effects), (
            name,
            effects,
        )
        assert all(
            Path(e["path"]).resolve().is_relative_to(work.resolve()) for e in effects
        )
    for name in [
        "file-update-missing",
        "file-create-existing",
        "file-write-stale",
        "file-patch-rollback",
        "file-missing",
        "file-delete-missing",
    ]:
        row = byid[name]
        assert row["files_before"] == row["files_after"], row
        assert row["result"]["failure"]["side_effect"] == "none" and not row[
            "result"
        ].get("effects"), row
    assert len(byid["file-move"]["result"]["effects"]) == 2
    pages = [unrecord(byid[f"artifact-page-{n}"]["result"]) for n in range(4)]
    assert "".join(p["content"] for p in pages) == PAYLOAD, pages
    assert [int(p["start_byte"]) for p in pages] == [
        0,
        int(pages[0]["end_byte"]),
        int(pages[1]["end_byte"]),
        int(pages[2]["end_byte"]),
    ]
    assert all(
        p["sha256"] == digest(PAYLOAD.encode())
        and int(p["total_bytes"]) == len(PAYLOAD.encode())
        for p in pages
    )
    assert (
        not pages[0]["complete"]
        and not pages[1]["complete"]
        and not pages[2]["complete"]
        and pages[3]["complete"]
        and pages[3]["next_cursor"] is None
    )
    assert int(pages[-1]["end_byte"]) == len(PAYLOAD.encode())
    shell = byid["shell-unicode"]["result"]
    assert shell["kind"] == {
        "kind": "command_output",
        "exit_code": "0",
    }, shell
    assert out("shell-unicode") == "中文🙂\n", shell
    assert byid["shell-stderr"]["result"]["kind"]["exit_code"] != "0"
    timeout = byid["shell-timeout"]
    assert (
        timeout["result"]["failure"]["category"] == "timeout"
        and timeout["process_before"]
        and timeout["process_after"] == []
    ), timeout
    signal = byid["shell-signal"]
    assert (
        signal["process_before"]
        and signal["process_after"] == []
        and signal["result"]["kind"]["kind"] == "command_output"
        and signal["result"]["kind"]["exit_code"] == "-1"
    ), signal
    assert (
        signal["result"]["failure"]["category"] == "permanent"
        and "timeout" not in signal["result"].get("error", "").lower()
    ), signal
    extract = parsed("web-extract")
    assert extract == {
        "headings": [{"tag": "h1", "text": "中文🙂 Heading"}],
        "links": [{"text": "链接🙂", "href": "/target"}],
        "paragraphs": ["段落 beta🙂"],
        "table_count": 1,
    }, extract
    assert (
        "中文🙂 Heading" in out("web-fetch")
        and byid["web-truncated"]["result"]["truncated"] is True
    )
    webartifact = byid["web-truncated"]["result"]["artifact"]
    b = decode_path(webartifact["path"]).read_bytes()
    assert digest(b) == webartifact["sha256"] and "large 中文🙂" in b.decode()
    assert (
        len(b)
        == int(webartifact["artifact_bytes"])
        == int(webartifact["payload_bytes"])
    )
    assert (
        decode_path(webartifact["path"])
        .resolve()
        .is_relative_to((work / "artifact-temp").resolve())
    )
    assert b.decode().count("large 中文🙂") == 120000
    assert b.decode().startswith(
        "URL: "
        + byid["web-truncated"]["arguments"][0]
        + "\nStatus: 200 OK\nContent-Type: text/html; charset=utf-8\n\n"
    )
    assert (
        "content truncated" in out("web-truncated") and len(out("web-truncated")) < 1000
    )
    assert webartifact["retention"] == "temporary_1h"
    assert parsed("db-tables") == {
        "columns": ["table_name"],
        "rows": [["records"]],
        "total_rows": 1,
    }
    describe = parsed("db-describe")
    assert describe["columns"] == [
        "cid",
        "name",
        "type",
        "notnull",
        "dflt_value",
        "pk",
    ], describe
    assert describe["rows"] == [
        [0, "id", "INTEGER", 0, None, 1],
        [1, "label", "TEXT", 1, None, 0],
        [2, "quantity", "INTEGER", 0, "7", 0],
        [3, "note", "TEXT", 0, None, 0],
    ], describe
    first, second = parsed("db-page-0"), parsed("db-page-1")
    assert first["rows"] + second["rows"] == [
        [1, "中文🙂", 3, None],
        [2, "beta", 7, "note"],
        [3, "gamma", 9, ""],
    ]
    assert first["page"]["next_cursor"] and second["page"].get("next_cursor") is None
    assert first["columns"] == second["columns"] == ["id", "label", "quantity", "note"]
    assert first["page"]["returned"] == 2 and first["page"]["truncated"] is True
    assert second["page"]["returned"] == 1 and second["page"]["truncated"] is False
    assert "SQL cursor does not match" in byid["db-stale-cursor"]["result"]["error"]
    assert (
        "Only" not in byid["db-sql-error"]["result"]["error"]
        and "Query failed" in byid["db-sql-error"]["result"]["error"]
    )
    assert (
        "table" in byid["db-missing"]["result"]["error"].lower()
        or "database" in byid["db-missing"]["result"]["error"].lower()
    )
    for name, list_key, fields in [
        (
            "research-arxiv_search",
            "papers",
            {
                "arxiv_id",
                "title",
                "authors",
                "abstract",
                "published",
                "pdf_url",
                "categories",
            },
        ),
        ("research-semantic_scholar_search", "papers", {"title", "authors"}),
        (
            "research-pubmed_search",
            "papers",
            {
                "pmid",
                "title",
                "authors",
                "abstract",
                "journal",
                "year",
                "doi",
                "mesh_terms",
            },
        ),
        (
            "research-clinical_trials_search",
            "studies",
            {
                "nct_id",
                "title",
                "status",
                "phase",
                "conditions",
                "interventions",
                "url",
            },
        ),
    ]:
        row = byid[name]
        if "result" in row and row["result"]["success"]:
            value = parsed(name)
            assert (
                value["query"] == "agent artificial intelligence"
                and isinstance(value[list_key], list)
                and len(value[list_key]) <= 1
            ), (name, value)
            for item in value[list_key]:
                assert fields.issubset(item), (name, item)
            if name == "research-clinical_trials_search":
                assert isinstance(value["total_results"], int) and value[
                    "total_results"
                ] >= value["returned"] == len(value["studies"]), value

    assert not (work / "missing.db").exists()
    return {
        "families": ["files", "shell", "web", "database", "research"],
        "rows": len(rows),
        "native_calls": sum("operation" in r for r in rows),
        "protected_file_count": len(PROTECTED),
        "external_observations": {
            r["id"]: (
                _verify_external_error(r, r)
                if "error" in r
                else "success"
                if r["result"]["success"]
                else "tool_failure"
            )
            for r in rows
            if r.get("expected") == "external"
        },
        "residual": [
            "Shell default policy cannot directly execute script; native tail timeout has one observed process, descendant cleanup needs separate framework evidence",
            "Fixed official research search results are external observations, not a deterministic common result corpus",
            "WebSearch custom provider and Sandbox reverse stream have separate consumer tests",
        ],
    }


def verify_search(work, data):
    calls = data["calls"]
    assert [c["request"]["query"] for c in calls] == [
        "success 中文🙂",
        "failure 中文🙂",
        "failure 中文🙂",
        "failure 中文🙂",
    ], calls
    assert all(
        c["request"]["operation"] == "search_provider_search"
        and int(c["request"]["max_results"]) == 10
        for c in calls
    ), calls
    assert len({c["invocation_id"] for c in calls}) == 4
    assert all(
        c["deadline"] == {"seconds": "30", "nanos": 0} and not c["cancelled"]
        for c in calls
    )
    assert data["rows"][0]["settled"]["terminal"]["status"] == "completed"
    assert data["rows"][1]["settled"]["terminal"]["status"] == "completed"
    requests = [
        json.loads(line) for line in (work / "model.jsonl").read_text().splitlines()
    ]
    messages = [m for r in requests for m in r["messages"] if m["role"] == "tool"]
    assert any(
        "标题中文🙂" in str(m["content"])
        and "https://example.invalid/paper" in str(m["content"])
        and "摘要 β🙂" in str(m["content"])
        for m in messages
    ), messages
    assert any("provider failed 中文🙂" in str(m["content"]) for m in messages), (
        messages
    )
    assert all(r["protected_before"] == r["protected_after"] for r in data["rows"])
    return {
        "provider_calls": 4,
        "success_calls": 1,
        "failure_attempts": 3,
        "title_url_snippet_exact": True,
        "canonical_agent_run_count": 2,
    }


def verify_sandbox(work, data):
    facts = data["facts"]
    assert [f["mode"] for f in facts] == ["success", "cancel", "close"]
    assert all(
        f["started"] and f["finally"] == 1 and not f["pid_alive"] for f in facts
    ), facts
    assert facts[0]["sent"] == [{"channel": "stdout", "chunk": "sandbox 中文🙂\n"}]
    from tool_behavior_fixture import decode_typed_data

    events = facts[0]["events"]
    assert [event["value"]["variant"] for event in events] == [
        "item",
        "item",
        "complete",
    ], events
    sequence = []
    items = []
    for event in events:
        fields = {f["name"]: f["value"] for f in event["value"]["fields"]}
        sequence.append(int(fields["sequence"]["value"]))
        assert fields["stream"]["value"] == facts[0]["stream"]
        if "value" in fields:
            items.append(decode_typed_data(fields["value"]))
    assert sequence == [1, 2, 3]
    assert items[0] == {
        "event_type": "output",
        "channel": "stdout",
        "chunk": "sandbox 中文🙂\n",
    }
    assert items[1]["event_type"] == "complete"
    for key, value in {
        "exit_code": 0,
        "stdout": "sandbox 中文🙂\n",
        "stderr": "",
        "sandbox_type": "owned-native-fixture",
        "timed_out": False,
        "cancelled": False,
        "output_truncated": False,
        "stdout_bytes": 19,
        "stderr_bytes": 0,
    }.items():
        assert items[1][key] == value, (key, items[1])
    assert (
        set(items[1]["duration"]) == {"secs", "nanos"}
        and 0 <= items[1]["duration"]["nanos"] < 1000000000
    )
    assert all(f["repeated_close"] is False for f in facts)
    assert facts[0]["events"][-1]["value"]["variant"] == "complete"
    assert facts[0]["terminal_returned_after_cleanup"] is True
    assert facts[2]["control_returned_after_cleanup"] is True
    assert all(f["cleanup_calls"] == 1 for f in facts)
    assert all(f["cancel_observed"] and f["exit_code"] != 0 for f in facts[1:]), facts
    return {
        "producer_processes": 3,
        "physically_joined": 3,
        "cancel_and_close_pending_invocation": 2,
        "post_answer_cancel": False,
    }


def compare_result_sets(paths):
    import re

    datasets = [json.loads(path.read_text()) for path in paths]
    assert len(datasets) == 3
    reports = []

    def normalized(value, work):
        if isinstance(value, str):
            value = value.replace(work, "$WORK")
            value = re.sub(r"http://127\.0\.0\.1:[0-9]+", "http://loopback", value)
            return value
        if isinstance(value, list):
            return [normalized(x, work) for x in value]
        if isinstance(value, dict):
            return {k: normalized(v, work) for k, v in value.items()}
        return value

    maps = [{row["id"]: row for row in data["rows"]} for data in datasets]
    ids = list(maps[0])
    assert all(list(m) == ids for m in maps)
    for name in ids:
        if name.startswith("research-"):
            reports.append(
                {
                    "id": name,
                    "comparison": "external_observation",
                    "reason": "Fixed official provider results/errors depend on service state; each result/error is retained; final shape and explicit error oracle must independently pass before delivery",
                }
            )
            continue
        rows = [m[name] for m in maps]
        if rows[0].get("action"):
            assert all(r["files_before"] == r["files_after"] for r in rows)
            reports.append(
                {
                    "id": name,
                    "comparison": "matched",
                    "fields": ["artifact mutation action", "protected files"],
                }
            )
            continue
        fields = []
        for field in [
            "protected_before",
            "protected_after",
            "files_before",
            "files_after",
        ]:
            if field.startswith("protected"):
                assert all(
                    r["protected_before"] == r["protected_after"] for r in rows
                ), (name, field)
                assert all(
                    r[field]["sentinel.txt"] == rows[0][field]["sentinel.txt"]
                    and r[field]["src/main.py"] == rows[0][field]["src/main.py"]
                    for r in rows
                ), (name, field)
            else:
                assert all(r[field] == rows[0][field] for r in rows), (name, field)
            fields.append(field)
        if "error" in rows[0]:
            values = [
                normalized(
                    {
                        k: r["error"][k]
                        for k in [
                            "code",
                            "message",
                            "operation",
                            "retryable",
                            "handle",
                            "details",
                        ]
                    },
                    d["work"],
                )
                for r, d in zip(rows, datasets)
            ]
            assert values[1:] == values[:-1], (name, values)
            fields.append("complete error payload")
        elif rows[0]["expected"] == "value":
            values = [r["result"] for r in rows]
            if name == "artifact-persist":
                refs = [unrecord(v) for v in values]
                assert all(
                    {
                        k: v[k]
                        for k in [
                            "artifact_bytes",
                            "payload_bytes",
                            "sha256",
                            "retention",
                        ]
                    }
                    == {
                        k: refs[0][k]
                        for k in [
                            "artifact_bytes",
                            "payload_bytes",
                            "sha256",
                            "retention",
                        ]
                    }
                    for v in refs
                )
                fields.append(
                    "artifact bytes/hash/retention and separately verified typed path"
                )
            else:
                assert all(
                    {
                        k: v[k]
                        for k in [
                            "content",
                            "complete",
                            "sha256",
                            "start_byte",
                            "end_byte",
                            "total_bytes",
                        ]
                    }
                    == {
                        k: values[0][k]
                        for k in [
                            "content",
                            "complete",
                            "sha256",
                            "start_byte",
                            "end_byte",
                            "total_bytes",
                        ]
                    }
                    for v in values
                )
                assert all(
                    bool(v["next_cursor"]) == bool(values[0]["next_cursor"])
                    for v in values
                )
                fields.append(
                    "page content/hash/offset/total/complete; opaque cursor independently roundtripped"
                )
        else:
            views = []
            for r, d in zip(rows, datasets):
                v = r["result"]
                view = {
                    k: v.get(k)
                    for k in ["kind", "success", "failure", "truncated", "mime_type"]
                }
                view["effects"] = normalized(v.get("effects", []), d["work"])
                if name == "web-truncated":
                    view["artifact"] = {
                        k: v["artifact"][k]
                        for k in ["artifact_bytes", "payload_bytes", "retention"]
                    }
                    view["preview_chars"] = 8
                elif name in [
                    "file-create",
                    "file-append",
                    "file-update",
                    "file-move",
                    "file-patch",
                    "file-delete",
                ]:
                    view["output"] = normalized(v["output"], d["work"])
                elif name.startswith(("file-", "db-")):
                    if v.get("data") and v["kind"]["kind"] == "json":
                        view["output_data"] = json.JSONDecoder().raw_decode(
                            v["output"]
                        )[0]
                    else:
                        view["output"] = normalized(v["output"], d["work"])
                    if name == "db-page-0":
                        view["output_data"]["page"]["next_cursor"] = "$OPAQUE_CURSOR"
                    if v.get("error"):
                        view["error"] = normalized(v["error"], d["work"])
                else:
                    view["output"] = normalized(v["output"], d["work"])
                    if v.get("error"):
                        view["error"] = normalized(v["error"], d["work"])
                views.append(normalized(view, d["work"]))
            assert views[1:] == views[:-1], (name, views)
            fields.append(
                "kind/success/failure/truncated/mime/effects and applicable values"
            )
        reports.append({"id": name, "comparison": "matched", "fields": fields})
    return {
        "languages": 3,
        "rows": len(ids),
        "matched_rows": sum(r["comparison"] == "matched" for r in reports),
        "external_observations": 4,
        "rows_compared": reports,
        "limitations": [
            "Opaque cursor bytes, runtime-generated artifact paths/checkpoint metadata, process identity/duration and per-service result text are checked against per-language authority; SQLite file hashes compare before/after within each run because independent databases need not be byte-identical. Equality is not correctness: final value oracle must pass separately"
        ],
    }


def main():
    p = argparse.ArgumentParser()
    p.add_argument("--work", type=Path)
    p.add_argument("--compare", type=Path, nargs=3)
    p.add_argument("--verify", type=Path)
    p.add_argument("--kind", choices=["tools", "search", "sandbox"], default="tools")
    args = p.parse_args()
    if args.compare:
        print(
            json.dumps(compare_result_sets(args.compare), ensure_ascii=False, indent=2)
        )
        return
    if args.work is None:
        p.error("--work is required unless --compare")
    if args.verify:
        print(
            json.dumps(
                {"tools": verify, "search": verify_search, "sandbox": verify_sandbox}[
                    args.kind
                ](args.work, json.loads(args.verify.read_text())),
                ensure_ascii=False,
            )
        )
        return
    lock = threading.Lock()

    class Handler(BaseHTTPRequestHandler):
        def do_GET(self):
            with lock, (args.work / "http.jsonl").open("a") as f:
                f.write(json.dumps({"method": "GET", "path": self.path}) + "\n")
            status, body = (
                (503, b"controlled unavailable")
                if self.path == "/503"
                else (
                    200,
                    (
                        ("large 中文🙂\n" * 120000).encode()
                        if self.path == "/large"
                        else HTML.encode()
                    ),
                )
            )
            self.send_response(status)
            self.send_header("Content-Type", "text/html; charset=utf-8")
            self.send_header("Content-Length", str(len(body)))
            self.end_headers()
            self.wfile.write(body)

        def do_POST(self):
            request = json.loads(self.rfile.read(int(self.headers["Content-Length"])))
            with lock, (args.work / "model.jsonl").open("a", encoding="utf8") as f:
                f.write(json.dumps(request, ensure_ascii=False) + "\n")
            messages = request.get("messages", [])
            prompt = next(
                (
                    m.get("content", "")
                    for m in reversed(messages)
                    if m.get("role") == "user"
                ),
                "",
            )
            if messages and messages[-1].get("role") == "tool":
                message = {
                    "role": "assistant",
                    "content": "search-result-consumed 中文🙂",
                }
                reason = "stop"
            else:
                query = (
                    "failure 中文🙂"
                    if "search-error" in str(prompt)
                    else "success 中文🙂"
                )
                message = {
                    "role": "assistant",
                    "content": None,
                    "tool_calls": [
                        {
                            "id": "search-proof",
                            "type": "function",
                            "function": {
                                "name": "web_search",
                                "arguments": json.dumps(
                                    {"query": query, "limit": 2},
                                    ensure_ascii=False,
                                ),
                            },
                        }
                    ],
                }
                reason = "tool_calls"
            response = {
                "id": "fixture-search",
                "object": "chat.completion",
                "created": 0,
                "model": "fixture-model",
                "choices": [{"index": 0, "message": message, "finish_reason": reason}],
                "usage": {
                    "prompt_tokens": 1,
                    "completion_tokens": 1,
                    "total_tokens": 2,
                },
            }
            if request.get("stream"):
                delta = dict(message)
                if "tool_calls" in delta:
                    delta["tool_calls"] = [
                        {"index": 0, **item} for item in delta["tool_calls"]
                    ]
                chunk = {
                    "id": "fixture-search",
                    "object": "chat.completion.chunk",
                    "created": 0,
                    "model": "fixture-model",
                    "choices": [{"index": 0, "delta": delta, "finish_reason": None}],
                }
                finish = {
                    "id": "fixture-search",
                    "object": "chat.completion.chunk",
                    "created": 0,
                    "model": "fixture-model",
                    "choices": [{"index": 0, "delta": {}, "finish_reason": reason}],
                    "usage": {
                        "prompt_tokens": 1,
                        "completion_tokens": 1,
                        "total_tokens": 2,
                    },
                }
                body = (
                    "data: "
                    + json.dumps(chunk, ensure_ascii=False)
                    + "\n\ndata: "
                    + json.dumps(finish, ensure_ascii=False)
                    + "\n\ndata: [DONE]\n\n"
                ).encode()
                content_type = "text/event-stream"
            else:
                body = json.dumps(response, ensure_ascii=False).encode()
                content_type = "application/json"
            self.send_response(200)
            self.send_header("Content-Type", content_type)
            self.send_header("Content-Length", str(len(body)))
            self.end_headers()
            self.wfile.write(body)

        def log_message(self, *args):
            pass

    server = ThreadingHTTPServer(("127.0.0.1", 0), Handler)
    prepare(args.work, f"http://127.0.0.1:{server.server_port}")
    (args.work / "ready").touch()
    server.serve_forever()


if __name__ == "__main__":
    main()
