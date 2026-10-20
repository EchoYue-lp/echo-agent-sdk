"""Shared fixtures and assertions for native Rust/TypeScript/Python/Java Host calls.

This process owns only deterministic input files and a loopback HTTP server.
Every SDK issues its own real Host calls; the verifier never sends ACP traffic.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import sqlite3
import subprocess
import threading
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path

CSV = "label,value,group\n中文,3,a\nemoji🙂,1,b\nalpha,2,a\n"
TEXT = "中文🙂\nalpha beta\n中文🙂\n"
PDF_TEXT = "SDK controlled paper."
BIB = "@article{li2026,\n  title = {中文 SDK \\& Evidence},\n  author = {Ada Li and 中文 作者},\n  year = {2026},\n  doi = {10.0000/sdk.fixture}\n}"


def git(repo: Path, *args: str) -> str:
    return subprocess.check_output(["git", "-C", str(repo), *args], text=True).strip()


def pdf_bytes() -> bytes:
    stream = f"BT /F1 12 Tf 30 100 Td ({PDF_TEXT}) Tj ET".encode()
    objects = [
        b"<< /Type /Catalog /Pages 2 0 R >>",
        b"<< /Type /Pages /Kids [3 0 R] /Count 1 >>",
        b"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 200 200] /Resources << /Font << /F1 4 0 R >> >> /Contents 5 0 R >>",
        b"<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>",
        b"<< /Length "
        + str(len(stream)).encode()
        + b" >>\nstream\n"
        + stream
        + b"\nendstream",
    ]
    content = b"%PDF-1.4\n"
    offsets = [0]
    for number, body in enumerate(objects, 1):
        offsets.append(len(content))
        content += f"{number} 0 obj\n".encode() + body + b"\nendobj\n"
    xref = len(content)
    content += f"xref\n0 {len(offsets)}\n0000000000 65535 f \n".encode()
    content += b"".join(f"{offset:010d} 00000 n \n".encode() for offset in offsets[1:])
    return (
        content
        + f"trailer\n<< /Size {len(offsets)} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n".encode()
    )


def prepare(work: Path, url: str) -> None:
    work.mkdir(parents=True, exist_ok=True)
    (work / "artifact-temp").mkdir()
    (work / "input.csv").write_text(CSV, encoding="utf-8")
    (work / "input.txt").write_text(TEXT, encoding="utf-8")
    (work / "must-survive.txt").write_bytes(b"sentinel\n")
    (work / "must-survive.bib").write_bytes(b"bib-sentinel\n")
    (work / "paper.pdf").write_bytes(pdf_bytes())
    with sqlite3.connect(work / "fixture.db") as db:
        db.execute(
            "CREATE TABLE records (id INTEGER, label TEXT, quantity INTEGER, note TEXT)"
        )
        db.executemany(
            "INSERT INTO records VALUES (?,?,?,?)",
            [(1, "中文🙂", 3, None), (2, "beta", 7, "note")],
        )
    before = {}
    for name in ["repo-a", "repo-b"]:
        repo = work / name
        repo.mkdir()
        git(repo, "init", "--initial-branch=main")
        git(repo, "config", "commit.gpgsign", "false")
        git(repo, "config", "user.name", "SDK Fixture")
        git(repo, "config", "user.email", "sdk-fixture@example.invalid")
        (repo / "tracked.txt").write_text("base\n", encoding="utf-8")
        git(repo, "add", "tracked.txt")
        git(repo, "-c", "commit.gpgsign=false", "commit", "-m", "fixture")
        before[name] = git(repo, "rev-parse", "HEAD")
    (work / "repo-a/tracked.txt").write_text("changed 中文🙂\n", encoding="utf-8")
    expected_diff = subprocess.check_output(
        ["git", "-C", str(work / "repo-a"), "diff"], text=True
    )
    (work / "baseline.json").write_text(
        json.dumps(
            {
                "database_sha256": hashlib.sha256(
                    (work / "fixture.db").read_bytes()
                ).hexdigest(),
                "heads": before,
                "git_diff": expected_diff,
            }
        ),
        encoding="utf-8",
    )
    path = lambda name: str(work / name)
    steps = []

    def add(name: str, operation: str, arguments: list, **extra: object) -> None:
        family, operation = operation.split(".", 1)
        steps.append(
            {
                "id": name,
                "family": family,
                "operation": operation,
                "arguments": arguments,
                "session": "a",
                **extra,
            }
        )

    add("data-sort", "data.transform_data", [path("input.csv"), "sort", "value:asc"])
    add(
        "data-select",
        "data.transform_data",
        [path("input.csv"), "select", "label,value"],
    )
    add(
        "data-missing-column",
        "data.transform_data",
        [path("input.csv"), "select", "not_a_column"],
        error=True,
    )
    add(
        "data-export",
        "data.export_data",
        [path("input.csv"), path("output.json"), "json", None, "label,value"],
    )
    add(
        "data-bad-format",
        "data.export_data",
        [path("input.csv"), path("must-survive.txt"), "not_a_format"],
        error=True,
    )
    add(
        "chart-exact",
        "chart.generate_chart",
        [
            "bar",
            "中文🙂 chart",
            "label",
            "value",
            None,
            [{"label": "中文🙂", "value": 3}, {"label": "beta", "value": 7}],
            321,
            123,
            "json",
        ],
    )
    add(
        "chart-missing-data",
        "chart.generate_chart",
        ["bar", "bad", "label", "value"],
        error=True,
    )
    db_url = f"sqlite://{work / 'fixture.db'}"
    add(
        "database-read",
        "database.sql_query",
        [db_url, "SELECT id,label,quantity,note FROM records ORDER BY id"],
    )
    add("database-refuse-write", "database.sql_query", [db_url, "DELETE FROM records"])
    add(
        "database-reread",
        "database.sql_query",
        [db_url, "SELECT id,label,quantity,note FROM records ORDER BY id"],
    )
    add("git-status-a", "git.git_status", [])
    add("git-diff", "git.git_diff", [])
    add("git-branch-create", "git.git_branch", [None, "sdk-proof"])
    add("git-invalid-branch", "git.git_branch", [None, "invalid branch"], error=True)
    add("git-status-b", "git.git_status", [], session="b")
    add("media-stats", "media.text_stats", [path("input.txt")])
    add("media-process", "media.process_text", [path("input.txt"), "unique"])
    add(
        "media-export",
        "media.export_text",
        [path("input.txt"), path("output.txt"), "unique"],
    )
    add(
        "media-export-failure",
        "media.export_text",
        [path("missing.txt"), path("must-survive.txt")],
        error=True,
    )
    add("research-pdf", "research.pdf_fetch", [url + "/paper.pdf", "all", 1000])
    add("research-pdf-artifact", "research.pdf_fetch", [url + "/paper.pdf", "all", 10])
    add(
        "research-http-error",
        "research.pdf_fetch",
        [url + "/failed", "all", 1000],
        error=True,
    )
    add(
        "research-not-pdf",
        "research.pdf_fetch",
        [url + "/not-pdf", "all", 1000],
        error=True,
    )
    add(
        "research-malformed-pdf",
        "research.pdf_fetch",
        [url + "/malformed", "all", 1000],
        error=True,
    )
    papers = [
        {
            "title": "中文 SDK & Evidence",
            "authors": ["Ada Li", "中文 作者"],
            "year": 2026,
            "doi": "10.0000/sdk.fixture",
            "additional_provider_field": {"unicode": "🙂"},
        }
    ]
    add("research-bibtex", "research.bibtex_generate", [papers, path("output.bib")])
    add(
        "research-bibtex-failure",
        "research.bibtex_generate",
        [[], path("must-survive.bib")],
        error=True,
    )
    steps.append({"id": "close-a", "action": "close", "session": "a"})
    add(
        "git-closed-session",
        "git.git_status",
        [],
        error=True,
        error_code="closed_handle",
    )
    add("git-b-after-a-close", "git.git_status", [], session="b")
    (work / "scenario.json").write_text(
        json.dumps(
            {"steps": steps, "sessions": {"a": path("repo-a"), "b": path("repo-b")}},
            ensure_ascii=False,
        ),
        encoding="utf-8",
    )
    config = {
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
                "name": "tool-proof",
                "system_prompt": "fixture",
                "enable_tools": True,
                "enable_memory": False,
                "max_iterations": 2,
            },
        },
        "sdk_profile": {"state_root": path("state")},
    }
    (work / "host.json").write_text(json.dumps(config), encoding="utf-8")


def decode_typed_data(value: object) -> object:
    # This checks the tagged ToolResult.data contract transported by every
    # native SDK, rather than interpreting display text as the data authority.
    assert isinstance(value, dict) and "kind" in value, value
    kind, payload = value["kind"], value.get("value")
    if kind == "null":
        return None
    if kind in {"string", "bool"}:
        return payload
    if kind in {"u64", "i64"}:
        assert isinstance(payload, str) and str(int(payload)) == payload
        return int(payload)
    if kind == "f64":
        return float(payload)
    if kind == "list":
        return [decode_typed_data(item) for item in payload]
    if kind == "map":
        pairs = [
            (decode_typed_data(item["key"]), decode_typed_data(item["value"]))
            for item in payload
        ]
        assert all(isinstance(key, str) for key, _ in pairs)
        assert len({key for key, _ in pairs}) == len(pairs)
        return dict(pairs)
    raise AssertionError(f"unexpected tool data kind: {kind}")


def verify(work: Path, results_path: Path) -> dict:
    scenario = json.loads((work / "scenario.json").read_text())
    rows = json.loads(results_path.read_text())
    assert [row["id"] for row in rows] == [step["id"] for step in scenario["steps"]], (
        "missing/duplicate/reordered SDK calls"
    )
    results = {row["id"]: row for row in rows}
    for step, row in zip(scenario["steps"], rows):
        if step.get("action") == "close":
            assert row["outcome"] == "closed", row
        elif step.get("error"):
            assert row["outcome"] == "error", row
            expected = step.get("error_code", "framework_error")
            assert row["code"] == expected, (step["id"], row)
            assert row["message"], row
            if expected == "framework_error":
                assert row.get("operation") == step["operation"], row
                assert row["retryable"] == "never", row
                fields = {
                    item["key"]: item["value"] for item in row["details"]["fields"]
                }
                assert fields == {
                    "framework_failure_category": "tool",
                    "framework_terminal_kind": "failed",
                    "framework_failure_code": "tool",
                    "framework_retryable": "false",
                }, row
        else:
            assert row["outcome"] == "result", row
            result = row["value"]
            assert result["success"] is (step["id"] != "database-refuse-write"), row
            assert isinstance(result["output"], str), row
            if step["id"] == "database-refuse-write":
                assert result["kind"] == {
                    "kind": "structured_error",
                    "error_code": "tool_error",
                }, row
                assert result["failure"] == {
                    "category": "permanent",
                    "recovery": "stop",
                    "side_effect": "none",
                }, row
            else:
                assert result["kind"] == {
                    "kind": "text"
                    if step["family"] == "git"
                    or step["id"] in {"chart-exact", "media-export"}
                    else "json"
                }, row
            # Empty reported effect/content collections are not proof of no
            # filesystem mutation; compare actual artifacts separately below.
            assert result.get("effects", []) == [], row
            assert result.get("model_content", []) == [], row
            assert result["truncated"] is (step["id"] == "research-pdf-artifact"), row

    def output(name: str):
        return results[name]["value"]["output"]

    def parsed(name: str):
        return json.loads(output(name))

    for name in [
        "data-sort",
        "data-select",
        "data-export",
        "database-read",
        "database-reread",
        "media-stats",
        "media-process",
        "research-pdf",
        "research-bibtex",
    ]:
        assert decode_typed_data(results[name]["value"]["data"]) == parsed(name), name
    original = [
        {"label": "中文", "value": 3, "group": "a"},
        {"label": "emoji🙂", "value": 1, "group": "b"},
        {"label": "alpha", "value": 2, "group": "a"},
    ]
    assert parsed("data-sort") == {
        "tool": "transform_data",
        "rows": 3,
        "columns": 3,
        "column_names": ["label", "value", "group"],
        "truncated": False,
        "data": [original[1], original[2], original[0]],
        "operation": "sort",
        "params": "value:asc",
    }
    selected = [{"label": row["label"], "value": row["value"]} for row in original]
    assert parsed("data-select") == {
        "tool": "transform_data",
        "rows": 3,
        "columns": 2,
        "column_names": ["label", "value"],
        "truncated": False,
        "data": selected,
        "operation": "select",
        "params": "label,value",
    }
    assert json.loads((work / "output.json").read_text()) == selected
    assert (work / "input.csv").read_bytes() == CSV.encode()
    assert parsed("chart-exact") == {
        "$schema": "https://vega.github.io/schema/vega-lite/v5.json",
        "width": 321,
        "height": 123,
        "title": "中文🙂 chart",
        "mark": "bar",
        "data": {
            "values": [{"label": "中文🙂", "value": 3}, {"label": "beta", "value": 7}]
        },
        "encoding": {
            "x": {"field": "label", "type": "nominal"},
            "y": {"field": "value", "type": "quantitative"},
        },
    }
    db_result = parsed("database-read")
    assert db_result["columns"] == ["id", "label", "quantity", "note"]
    assert db_result["rows"] == [[1, "中文🙂", 3, None], [2, "beta", 7, "note"]]
    assert parsed("database-reread") == db_result
    assert (
        results["database-refuse-write"]["value"]["error"]
        == "Only read-only queries allowed (SELECT/SHOW/DESCRIBE/EXPLAIN), received: DELETE FROM records"
    )
    baseline = json.loads((work / "baseline.json").read_text())
    assert (
        hashlib.sha256((work / "fixture.db").read_bytes()).hexdigest()
        == baseline["database_sha256"]
    )
    assert output("git-status-a") == "Git status:\nM tracked.txt"
    assert output("git-diff") == "```diff\n" + baseline["git_diff"].strip() + "\n```"
    assert (
        output("git-status-b")
        == output("git-b-after-a-close")
        == "Working directory clean, no changes"
    )
    for repo, head in baseline["heads"].items():
        assert git(work / repo, "rev-parse", "HEAD") == head
    assert git(work / "repo-a", "rev-parse", "sdk-proof") == baseline["heads"]["repo-a"]
    assert (
        git(work / "repo-b", "for-each-ref", "--format=%(refname)") == "refs/heads/main"
    )
    assert (work / "repo-a/tracked.txt").read_bytes() == "changed 中文🙂\n".encode()
    assert (work / "repo-b/tracked.txt").read_bytes() == b"base\n"
    stats = parsed("media-stats")
    for key, value in {
        "lines": 3,
        "chars": len(TEXT),
        "words": 4,
        "chinese_chars": 4,
        "english_words": 2,
        "max_line_len": 10,
        "avg_line_len": 10.0,
        "file_size_kb": len(TEXT.encode()) / 1024,
    }.items():
        assert stats[key] == value, (key, stats)
    assert parsed("media-process") == {
        "file": str(work / "input.txt"),
        "operation": "unique",
        "original_lines": 3,
        "result_lines": 2,
        "preview": ["中文🙂", "alpha beta"],
        "truncated": False,
    }
    assert (work / "output.txt").read_bytes() == "中文🙂\nalpha beta".encode()
    assert (work / "input.txt").read_bytes() == TEXT.encode()
    assert (work / "must-survive.txt").read_bytes() == b"sentinel\n"
    assert (work / "must-survive.bib").read_bytes() == b"bib-sentinel\n"
    pdf = parsed("research-pdf")
    assert pdf["pages_requested"] == "all" and pdf["metadata"] == {"pages": 1}
    assert pdf["text"] == f"\n--- Page 1 ---\n{PDF_TEXT}\n\n"
    assert pdf["text_length"] == len(pdf["text"])
    artifact_result = results["research-pdf-artifact"]["value"]
    artifact_preview = decode_typed_data(artifact_result["data"])
    assert artifact_preview == {**pdf, "text": pdf["text"][:10], "text_complete": False}
    artifact = artifact_result["artifact"]
    wire_path = artifact["path"]
    if wire_path["encoding"] == "unix":
        import base64

        artifact_path = Path(base64.b64decode(wire_path["bytes_base64"]).decode())
    elif wire_path["encoding"] == "utf8":
        artifact_path = Path(wire_path["path"])
    else:
        import base64

        artifact_path = Path(
            base64.b64decode(wire_path["utf16_base64"]).decode("utf-16-le")
        )
    assert artifact_path.resolve().is_relative_to((work / "artifact-temp").resolve()), (
        artifact_path
    )
    artifact_bytes = artifact_path.read_bytes()
    assert hashlib.sha256(artifact_bytes).hexdigest() == artifact["sha256"]
    assert (
        len(artifact_bytes)
        == int(artifact["artifact_bytes"])
        == int(artifact["payload_bytes"])
    )
    assert json.loads(artifact_bytes) == pdf
    assert artifact["retention"] == "temporary_1h"

    assert parsed("research-bibtex") == {
        "entries_count": 1,
        "bibtex": BIB,
        "output_file": str(work / "output.bib"),
    }
    assert (work / "output.bib").read_bytes() == BIB.encode()
    requests = [
        json.loads(line) for line in (work / "http.jsonl").read_text().splitlines()
    ]
    assert [request["path"] for request in requests] == [
        "/paper.pdf",
        "/paper.pdf",
        "/failed",
        "/not-pdf",
        "/malformed",
    ]
    assert all(request["method"] == "GET" for request in requests), (
        "leaf calls must not secretly start model/Run work"
    )
    runs = work / "state/runs"
    assert not runs.exists() or not any(runs.rglob("*")), (
        "tool leaf calls minted unexpected Run state"
    )
    return {
        "capability_families": 6,
        "native_calls": len(rows),
        "failure_paths": sum(bool(step.get("error")) for step in scenario["steps"]),
        "reported_effects": "empty",
        "actual_file_and_ref_effects_checked": True,
        "uncovered": [
            "fixed-official-endpoint scholarly search providers",
            "other accepted media parsers/data transforms",
            "runtime effect-ledger interpretation",
        ],
    }


# The non-text media story exercises the accepted leaf Tools, never a model or
# an alternate parser. Existing six-family preparation/assertions stay intact.
MEDIA_NON_TEXT = (
    "view_image",
    "pdf_extract",
    "pdf_info",
    "read_excel",
    "excel_info",
    "excel_to_csv",
    "excel_profile",
    "excel_write",
    "read_word",
    "word_info",
    "word_structure",
    "image_fetch",
)


def media_zip(parts: dict[str, str]) -> bytes:
    import io
    import zipfile

    output = io.BytesIO()
    with zipfile.ZipFile(output, "w", zipfile.ZIP_DEFLATED) as archive:
        for name, content in parts.items():
            entry = zipfile.ZipInfo(name, date_time=(2020, 1, 1, 0, 0, 0))
            entry.compress_type = zipfile.ZIP_DEFLATED
            archive.writestr(entry, content.encode("utf-8"))
    return output.getvalue()


def media_png() -> bytes:
    import struct
    import zlib

    def chunk(kind: bytes, data: bytes) -> bytes:
        return (
            struct.pack(">I", len(data))
            + kind
            + data
            + struct.pack(">I", zlib.crc32(kind + data))
        )

    return (
        b"\x89PNG\r\n\x1a\n"
        + chunk(b"IHDR", struct.pack(">IIBBBBB", 1, 1, 8, 2, 0, 0, 0))
        + chunk(b"IDAT", zlib.compress(b"\x00\xff\x00\x00"))
        + chunk(b"IEND", b"")
    )


def prepare_media(work: Path, url: str) -> None:
    work.mkdir(parents=True, exist_ok=True)
    (work / "artifact-temp").mkdir()
    (work / "paper.pdf").write_bytes(pdf_bytes())
    (work / "pixel.png").write_bytes(media_png())
    rel = "http://schemas.openxmlformats.org/package/2006/relationships"
    office = "http://schemas.openxmlformats.org/officeDocument/2006/relationships"
    word = "http://schemas.openxmlformats.org/wordprocessingml/2006/main"
    (work / "document.docx").write_bytes(
        media_zip(
            {
                "[Content_Types].xml": '<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"><Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/><Default Extension="xml" ContentType="application/xml"/><Override PartName="/word/document.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"/></Types>',
                "_rels/.rels": f'<Relationships xmlns="{rel}"><Relationship Id="rId1" Type="{office}/officeDocument" Target="word/document.xml"/></Relationships>',
                "word/_rels/document.xml.rels": f'<Relationships xmlns="{rel}"/>',
                "word/document.xml": f'<w:document xmlns:w="{word}"><w:body><w:p><w:r><w:t>媒体标题 中文🙂</w:t></w:r></w:p><w:p><w:r><w:t>Body media parser 42.</w:t></w:r></w:p><w:tbl><w:tr><w:tc><w:p><w:r><w:t>left</w:t></w:r></w:p></w:tc><w:tc><w:p><w:r><w:t>right</w:t></w:r></w:p></w:tc></w:tr></w:tbl><w:sectPr/></w:body></w:document>',
            }
        )
    )
    for name in ["broken.pdf", "broken.xlsx", "broken.docx", "unsupported.png"]:
        (work / name).write_bytes(b"invalid controlled media\n")
    for name in ["protected.xlsx", "protected.csv"]:
        (work / name).write_bytes(b"must-survive-media-failure\n")
    inputs = {
        p.name: hashlib.sha256(p.read_bytes()).hexdigest()
        for p in work.iterdir()
        if p.is_file()
    }
    (work / "media-baseline.json").write_text(json.dumps(inputs))
    path = lambda name: str(work / name)
    steps = []

    def add(name: str, operation: str, args: list, **extra: object) -> None:
        steps.append(
            {
                "id": name,
                "family": "media",
                "operation": operation,
                "arguments": args,
                "session": "a",
                **extra,
            }
        )

    add("pdf-info", "pdf_info", [path("paper.pdf")])
    add("pdf-text", "pdf_extract", [path("paper.pdf"), "1", True])
    add(
        "excel-write",
        "excel_write",
        [
            path("generated.xlsx"),
            "数据",
            ["label", "value"],
            [["中文🙂", 3], ["beta", 7]],
        ],
    )
    add("excel-read", "read_excel", [path("generated.xlsx"), "数据", 10, 0])
    add("excel-info", "excel_info", [path("generated.xlsx")])
    add("excel-profile", "excel_profile", [path("generated.xlsx"), "数据"])
    add(
        "excel-csv",
        "excel_to_csv",
        [path("generated.xlsx"), path("generated.csv"), "数据"],
    )
    add("word-read", "read_word", [path("document.docx"), False])
    add("word-info", "word_info", [path("document.docx")])
    add("word-structure", "word_structure", [path("document.docx")])
    add("image-view", "view_image", [path("pixel.png"), "low"])
    add("image-fetch", "image_fetch", [url + "/pixel.png", 1])
    add("pdf-malformed", "pdf_extract", [path("broken.pdf"), "all", False], error=True)
    add("excel-malformed", "read_excel", [path("broken.xlsx")], error=True)
    add("word-malformed", "read_word", [path("broken.docx"), False], error=True)
    add(
        "image-unsupported",
        "view_image",
        [path("unsupported.png"), "high"],
        failure="invalid_arguments",
    )
    add(
        "image-url-unsupported",
        "image_fetch",
        ["ftp://example.invalid/image.png", 1],
        failure="permanent",
    )
    add(
        "excel-invalid-write",
        "excel_write",
        [path("protected.xlsx"), "数据", ["label"], "not-an-array"],
        error=True,
    )
    add(
        "excel-invalid-export",
        "excel_to_csv",
        [path("broken.xlsx"), path("protected.csv"), "数据"],
        error=True,
    )
    assert {step["operation"] for step in steps} == set(MEDIA_NON_TEXT)
    (work / "scenario.json").write_text(
        json.dumps(
            {
                "steps": steps,
                "sessions": {"a": str(work), "b": str(work)},
                "media_non_text_operations": list(MEDIA_NON_TEXT),
            },
            ensure_ascii=False,
        )
    )
    config = {
        "schema_version": 1,
        "default_agent": {
            "model": {
                "provider": "fixture",
                "name": "media-fixture",
                "base_url": url + "/v1/chat/completions",
                "api_protocol": "chat_completions",
                "auth_token": "fixture-secret",
            },
            "agent": {
                "name": "media-proof",
                "system_prompt": "fixture",
                "enable_tools": True,
                "enable_memory": False,
                "max_iterations": 2,
            },
        },
        "sdk_profile": {"state_root": str(work / "state")},
    }
    (work / "host.json").write_text(json.dumps(config))


def verify_media(work: Path, results_path: Path) -> dict:
    import base64
    import xml.etree.ElementTree as ET
    import zipfile

    scenario = json.loads((work / "scenario.json").read_text())
    rows = json.loads(results_path.read_text())
    assert [r["id"] for r in rows] == [s["id"] for s in scenario["steps"]], (
        "missing/duplicate/reordered calls"
    )
    assert len(rows) == 19
    results = {row["id"]: row for row in rows}
    for step, row in zip(scenario["steps"], rows):
        if step.get("error"):
            assert row["outcome"] == "error", row
            assert row["code"] == "framework_error", row
            assert row["operation"] == step["operation"], row
            assert row["retryable"] == "never", row
            fields = {item["key"]: item["value"] for item in row["details"]["fields"]}
            assert fields == {
                "framework_failure_category": "tool",
                "framework_terminal_kind": "failed",
                "framework_failure_code": "tool",
                "framework_retryable": "false",
            }, row
            continue
        assert row["outcome"] == "result", row
        value = row["value"]
        assert value["success"] is (not bool(step.get("failure"))), row
        assert value["truncated"] is False, row
        assert value.get("effects", []) == [], row
        if step.get("failure"):
            assert value["kind"] == {
                "kind": "structured_error",
                "error_code": "tool_error",
            }, row
            category = step["failure"]
            assert value["failure"] == {
                "category": category,
                "recovery": "correct_arguments"
                if category == "invalid_arguments"
                else "stop",
                "side_effect": "none",
            }, row
            assert value["error"], row
        elif step["id"] == "image-view":
            assert value["kind"] == {"kind": "image", "mime_type": "image/png"}, row
        else:
            assert value["kind"] == {"kind": "text"}, row
        if step["id"] != "image-view":
            assert value.get("model_content", []) == [], row

    def output(name: str) -> str:
        return results[name]["value"]["output"]

    assert "Page count: 1" in output("pdf-info")
    assert PDF_TEXT in output("pdf-text") and "pages 1, total 1 pages" in output(
        "pdf-text"
    )
    assert "PDF Metadata" in output("pdf-text")
    read = output("excel-read")
    assert "Sheet: 数据\nTotal rows: 3\nTotal cols: 2" in read
    assert "label\tvalue\n中文🙂\t3\nbeta\t7" in read
    assert "数据 (3 rows x 2 cols)" in output("excel-info")
    assert "value [numeric]: 0 nulls (0.0%) | min=3.00 max=7.00 mean=5.00" in output(
        "excel-profile"
    )
    expected_csv = "\ufefflabel,value\n中文🙂,3\nbeta,7".encode()
    assert (work / "generated.csv").read_bytes() == expected_csv
    assert "2 rows, 2 cols" in output("excel-write")
    with zipfile.ZipFile(work / "generated.xlsx") as workbook:
        ns = {"s": "http://schemas.openxmlformats.org/spreadsheetml/2006/main"}
        strings = [
            "".join(node.itertext())
            for node in ET.fromstring(workbook.read("xl/sharedStrings.xml")).findall(
                "s:si", ns
            )
        ]
        cells = ET.fromstring(workbook.read("xl/worksheets/sheet1.xml")).findall(
            "s:sheetData/s:row/s:c", ns
        )
        actual = {}
        for cell in cells:
            value = cell.find("s:v", ns)
            assert value is not None
            actual[cell.attrib["r"]] = (
                strings[int(value.text)] if cell.attrib.get("t") == "s" else value.text
            )
        assert actual == {
            "A1": "label",
            "B1": "value",
            "A2": "中文🙂",
            "B2": "3",
            "A3": "beta",
            "B3": "7",
        }
    assert (
        output("word-read")
        == "媒体标题 中文🙂\nBody media parser 42.\n\n[Table content]\nleft | right\n"
    )
    assert "Paragraph count: 2" in output("word-info")
    assert "Paragraph 1: 媒体标题 中文🙂" in output("word-structure")
    assert "Statistics: 2 paragraphs, 1 tables" in output("word-structure")
    png = (work / "pixel.png").read_bytes()
    uri = "data:image/png;base64," + base64.b64encode(png).decode()
    view = results["image-view"]["value"]
    assert view["mime_type"] == "image/png"
    assert view["metadata"] == {
        "path": str((work / "pixel.png").resolve()),
        "byte_size": str(len(png)),
    }
    assert view["model_content"] == [{"kind": "image_url", "url": uri, "detail": "low"}]
    assert base64.b64decode(view["model_content"][0]["url"].split(",", 1)[1]) == png
    assert output("image-fetch").endswith("Data URI: " + uri)
    assert "Content-Type: image/png" in output("image-fetch")
    assert (
        "Unsupported or invalid image" in results["image-unsupported"]["value"]["error"]
    )
    assert (
        "URL must start with http:// or https://"
        in results["image-url-unsupported"]["value"]["error"]
    )
    for name, digest in json.loads((work / "media-baseline.json").read_text()).items():
        assert hashlib.sha256((work / name).read_bytes()).hexdigest() == digest, name
    requests = [
        json.loads(line) for line in (work / "http.jsonl").read_text().splitlines()
    ]
    assert requests == [{"method": "GET", "path": "/pixel.png"}], requests
    assert not (work / "state/runs").exists() or not any(
        (work / "state/runs").rglob("*")
    )
    summary = {
        "accepted_non_text_operations": sorted(MEDIA_NON_TEXT),
        "native_calls": len(rows),
        "successful_operations": 12,
        "typed_rpc_errors": 5,
        "structured_tool_failures": 2,
        "png_sha256": hashlib.sha256(png).hexdigest(),
        "csv_sha256": hashlib.sha256(expected_csv).hexdigest(),
        "xlsx_sha256": hashlib.sha256(
            (work / "generated.xlsx").read_bytes()
        ).hexdigest(),
        "view_image_model_content_exact": True,
        "image_fetch_textual_uri_exact": True,
        "input_and_failure_sentinels_unchanged": True,
        "limits": [
            "one-page unencrypted PDF, generated OOXML workbook and DOCX, one PNG only",
            "view_image model_content wire projection; no model Run or external viewer execution",
            "image_fetch returns a textual data URI, not inferred model_content",
            "not arbitrary codec, oversized input or every optional field combinations",
        ],
    }
    (work / "media-verified.json").write_text(
        json.dumps(summary, ensure_ascii=False, indent=2)
    )
    return summary


def serve_media(work: Path) -> None:
    class Handler(BaseHTTPRequestHandler):
        def do_GET(self) -> None:
            with (work / "http.jsonl").open("a", encoding="utf-8") as output:
                output.write(json.dumps({"method": "GET", "path": self.path}) + "\n")
            body = media_png() if self.path == "/pixel.png" else b"not media"
            self.send_response(200 if self.path == "/pixel.png" else 404)
            self.send_header(
                "Content-Type",
                "image/png" if self.path == "/pixel.png" else "text/plain",
            )
            self.send_header("Content-Length", str(len(body)))
            self.end_headers()
            self.wfile.write(body)

        def log_message(self, *_args: object) -> None:
            pass

    server = ThreadingHTTPServer(("127.0.0.1", 0), Handler)
    prepare_media(work, f"http://127.0.0.1:{server.server_port}")
    (work / "ready").touch()
    server.serve_forever()


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--work", type=Path, required=True)
    parser.add_argument("--verify", type=Path)
    parser.add_argument("--media", action="store_true")
    args = parser.parse_args()
    if args.media:
        if args.verify:
            print(json.dumps(verify_media(args.work, args.verify)))
        else:
            serve_media(args.work)
        return
    if args.verify:
        print(json.dumps(verify(args.work, args.verify)))
        return
    work = args.work
    lock = threading.Lock()

    class Handler(BaseHTTPRequestHandler):
        def do_GET(self) -> None:
            with lock, (work / "http.jsonl").open("a", encoding="utf-8") as stream:
                stream.write(json.dumps({"method": "GET", "path": self.path}) + "\n")
            if self.path == "/paper.pdf":
                status, body = 200, pdf_bytes()
            elif self.path == "/failed":
                status, body = 503, b"controlled unavailable"
            elif self.path == "/malformed":
                status, body = 200, b"%PDF-1.4\ninvalid controlled pdf\n"
            else:
                status, body = 200, b"not a pdf"
            self.send_response(status)
            self.send_header("Content-Type", "application/pdf")
            self.send_header("Content-Length", str(len(body)))
            self.end_headers()
            self.wfile.write(body)

        def log_message(self, *_args: object) -> None:
            pass

    server = ThreadingHTTPServer(("127.0.0.1", 0), Handler)
    prepare(work, f"http://127.0.0.1:{server.server_port}")
    (work / "ready").touch()
    server.serve_forever()


if __name__ == "__main__":
    main()
