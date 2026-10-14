"""Shared inputs and oracle only; native clients own every real Host request.

The loopback server replaces only the external embedding provider. It records
model/input and response mode, never authorization headers or keys.
"""

import argparse
import json
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path

SDK = Path(__file__).resolve().parents[3]
PREFIX = "echo_core::project_rules::InstructionResolver::"
DOCUMENT = "中文😀甲乙丙丁戊己庚辛"
CHUNKS = [DOCUMENT[i : i + 4] for i in range(0, len(DOCUMENT), 4)]
QUERY = "查询😀第一块"


def write_json(path, value):
    path.write_text(json.dumps(value, ensure_ascii=False, indent=2) + "\n")


def prepare(work):
    work.mkdir(parents=True, exist_ok=True)
    for directory in ["repo/.git", "repo/nested/leaf", "other/.git", "empty/.git"]:
        (work / directory).mkdir(parents=True, exist_ok=True)
    files = {
        "repo/AGENTS.md": "根规则 中文😀\n",
        "repo/CLAUDE.md": "ignored root CLAUDE\n",
        "repo/nested/CLAUDE.md": "nested claude 中文\n",
        "repo/nested/leaf/AGENTS.override.md": "leaf override 😀\n",
        "repo/nested/leaf/AGENTS.md": "ignored leaf AGENTS\n",
        "other/AGENTS.md": "other isolated rules\n",
    }
    for path, content in files.items():
        (work / path).write_text(content)
    config = json.loads((SDK / "echo-sdk-host/config.sdk.example.json").read_text())
    config["sdk_profile"]["state_root"] = str(work / "state")
    config["default_agent"]["agent"]["memory_path"] = str(work / "memory.jsonl")
    write_json(work / "host.json", config)
    write_json(work / "mode.json", {"mode": "ok"})
    steps = []

    def call(identity, operation, arguments, session="a", save=None):
        steps.append(
            {
                "id": identity,
                "operation": operation,
                "arguments": arguments,
                "session": session,
                "save": save,
            }
        )

    call("chunk", "rag_chunk_document", [DOCUMENT, 4, 0])
    call("chunk-70-chinese", "rag_chunk_document", ["中" * 70, 300, 0])
    call("chunk-200-scalars", "rag_chunk_document", ["中" * 199 + "😀", 300, 0])
    call("chunk-201-scalars", "rag_chunk_document", ["中" * 200 + "😀", 300, 0])
    call("index", "rag_index", [DOCUMENT, "来源😀.txt", 4, 0])
    call("search", "rag_search", [QUERY, 8])
    call("isolated", "rag_search", [QUERY, 8], "b")
    for mode in ["second-503", "bad-json", "empty-data"]:
        steps.append({"id": "mode-" + mode, "action": "mode", "mode": mode})
        call(
            "failed-index-" + mode,
            "rag_index",
            ["失败😀文档甲乙丙丁戊", "failed", 4, 0],
        )
        steps.append({"id": "restore-" + mode, "action": "mode", "mode": "ok"})
        call("unchanged-" + mode, "rag_search", [QUERY, 8])
    call("invalid-chunks", "rag_index", [DOCUMENT, "invalid", 4, 4])
    call(
        "resolver",
        PREFIX + "new",
        [
            "$session",
            {
                "kind": "path",
                "value": {"encoding": "utf8", "path": "$WORK/repo/nested/leaf"},
            },
        ],
        save="resolver",
    )
    call(
        "set-root",
        PREFIX + "project_root",
        [
            "$session",
            "$resolver",
            {"kind": "path", "value": {"encoding": "utf8", "path": "$WORK/repo"}},
        ],
    )
    call("resolved-all", PREFIX + "resolve", ["$session", "$resolver"])
    call("resolved-family", "project-rules.resolve", [])
    call("load-family", "project-rules.load", [])
    call("cross-session", PREFIX + "resolve", ["$session", "$resolver"], "b")
    call("agents-only", PREFIX + "agents_files_only", ["$session", "$resolver"])
    call("resolved-agents", PREFIX + "resolve", ["$session", "$resolver"])
    steps.append(
        {
            "id": "edit",
            "action": "write",
            "path": "repo/nested/leaf/AGENTS.override.md",
            "text": "changed 中文🌊\n",
        }
    )
    call("dynamic", PREFIX + "resolve", ["$session", "$resolver"])
    steps.append(
        {
            "id": "invalid-utf8",
            "action": "write",
            "path": "repo/nested/leaf/AGENTS.override.md",
            "bytes": [255, 254],
        }
    )
    call("fallback", PREFIX + "resolve", ["$session", "$resolver"])
    steps.append(
        {"id": "directory", "action": "directory", "path": "repo/nested/leaf/AGENTS.md"}
    )
    call("directory-skip", PREFIX + "resolve", ["$session", "$resolver"])
    call("other-rules", "project-rules.resolve", [], "b")
    call("empty-rules", "project-rules.resolve", [], "empty")
    call("empty-load", "project-rules.load", [], "empty")
    steps.append({"id": "close-a", "action": "close", "session": "a"})
    call("closed-rag", "rag_search", [QUERY, 8])
    call("closed-resolver", PREFIX + "resolve", ["$session", "$resolver"])
    call("new-session-empty", "rag_search", [QUERY, 8], "new")
    write_json(work / "scenario.json", {"steps": steps})


def action(work, step):
    if step["action"] == "mode":
        write_json(work / "mode.json", {"mode": step["mode"], "calls": 0})
    elif step["action"] == "write":
        path = work / step["path"]
        if "bytes" in step:
            path.write_bytes(bytes(step["bytes"]))
        else:
            path.write_text(step["text"])
    elif step["action"] == "directory":
        path = work / step["path"]
        path.unlink()
        path.mkdir()


def serve(work):
    class Handler(BaseHTTPRequestHandler):
        def log_message(self, *_):
            pass

        def do_POST(self):
            body = json.loads(self.rfile.read(int(self.headers["Content-Length"])))
            state = json.loads((work / "mode.json").read_text())
            state["calls"] = state.get("calls", 0) + 1
            write_json(work / "mode.json", state)
            mode = state["mode"]
            with (work / "embedding.jsonl").open("a") as output:
                output.write(
                    json.dumps(
                        {
                            "path": self.path,
                            "request": body,
                            "mode": mode,
                            "call": state["calls"],
                        },
                        ensure_ascii=False,
                    )
                    + "\n"
                )
            status, payload = (
                200,
                {
                    "data": [
                        {
                            "embedding": [1.0, 0.0]
                            if body["input"] in [QUERY, CHUNKS[0]]
                            else [0.0, 1.0]
                        }
                    ]
                },
            )
            if mode == "second-503" and state["calls"] == 2:
                status, payload = 503, {"error": "fixture unavailable"}
            elif mode == "bad-json":
                payload = "{broken"
            elif mode == "empty-data":
                payload = {"data": []}
            encoded = (
                payload if isinstance(payload, str) else json.dumps(payload)
            ).encode()
            self.send_response(status)
            self.send_header("Content-Type", "application/json")
            self.send_header("Content-Length", str(len(encoded)))
            self.end_headers()
            self.wfile.write(encoded)

    server = ThreadingHTTPServer(("127.0.0.1", 0), Handler)
    write_json(
        work / "endpoint.json",
        {"url": f"http://127.0.0.1:{server.server_port}/v1/embeddings"},
    )
    server.serve_forever()


def decode(value):
    if isinstance(value, list):
        return [decode(item) for item in value]
    if not isinstance(value, dict):
        return value
    kind = value.get("kind")
    if not isinstance(kind, str):
        return {key: decode(item) for key, item in value.items()}
    if kind == "path":
        assert value["value"]["encoding"] == "utf8", value
        return value["value"]["path"]
    if kind in {"string", "bool", "u64", "i64", "f64"}:
        return value.get("value")
    if kind == "list":
        return decode(value["value"])
    if kind == "record":
        return {
            field["name"]: decode(field["value"]) for field in value["value"]["fields"]
        }
    if kind == "variant":
        return {
            "variant": value["value"]["variant"],
            "fields": {
                field["name"]: decode(field["value"])
                for field in value["value"]["fields"]
            },
        }
    return {key: decode(item) for key, item in value.items()}


def expected_rules(work, sources):
    return {
        "sources": [
            {"path": str(work / path), "kind": kind, "precedence": str(index)}
            for index, (path, kind, _) in enumerate(sources)
        ],
        "content": "\n\n".join(
            f"<!-- PROJECT INSTRUCTIONS: {work / path} -->\n{text}\n<!-- END PROJECT INSTRUCTIONS -->"
            for path, _, text in sources
        ),
    }


def verify(work, evidence):
    rows = {row["id"]: row for row in evidence["rows"]}

    def val(identity):
        assert "error" not in rows[identity], rows[identity]
        return decode(rows[identity]["result"])

    def tool(identity):
        result = val(identity)
        assert result["success"] is True, result
        return json.loads(result["output"])

    search = tool("search")
    expected_hits = [
        {
            "rank": i + 1,
            "similarity_pct": "100.0" if i == 0 else "0.0",
            "chunk_index": i,
            "total_chunks": len(CHUNKS),
            "source": "来源😀.txt",
            "content": chunk,
        }
        for i, chunk in enumerate(CHUNKS)
    ]
    assert search == {
        "query": QUERY,
        "total_results": len(CHUNKS),
        "results": expected_hits,
    }, search
    assert val("index")["success"] is True
    preview = tool("chunk")
    assert [chunk["preview"] for chunk in preview["chunks"]] == CHUNKS
    assert [chunk["char_count"] for chunk in preview["chunks"]] == [
        len(chunk) for chunk in CHUNKS
    ]
    assert all(chunk["truncated"] is False for chunk in preview["chunks"])
    for identity, content in [
        ("chunk-70-chinese", "中" * 70),
        ("chunk-200-scalars", "中" * 199 + "😀"),
        ("chunk-201-scalars", "中" * 200 + "😀"),
    ]:
        actual = tool(identity)
        assert actual == {
            "chunk_size": 300,
            "overlap": 0,
            "total_chunks": 1,
            "chunks": [
                {
                    "index": 1,
                    "char_count": len(content),
                    "preview": content[:200],
                    "truncated": len(content) > 200,
                }
            ],
        }, actual
    for identity in ["isolated", "new-session-empty"]:
        assert "No relevant documents" in val(identity)["output"]
    for mode in ["second-503", "bad-json", "empty-data"]:
        failed = val("failed-index-" + mode)
        assert failed["success"] is False and "Embedding" in failed["error"], failed
        assert failed["failure"] == {
            "category": "permanent",
            "recovery": "stop",
            "side_effect": "none",
        }
        assert tool("unchanged-" + mode) == search
    for identity in [
        "invalid-chunks",
        "cross-session",
        "closed-rag",
        "closed-resolver",
        "invalid-config",
    ]:
        assert rows[identity].get("error", {}).get("code"), rows[identity]
        assert rows[identity]["error"]["operation"], rows[identity]
    expected_codes = {
        "invalid-config": "invalid_config",
        "invalid-chunks": "framework_error",
        "cross-session": "invalid_value",
        "closed-rag": "closed_handle",
        "closed-resolver": "closed_handle",
    }
    for identity, code in expected_codes.items():
        assert rows[identity]["error"]["code"] == code
        assert rows[identity]["error"]["retryable"] == "never"
    base = [("repo/AGENTS.md", "agents", "根规则 中文😀")]
    leaf = (
        "repo/nested/leaf/AGENTS.override.md",
        "agents_override",
        "leaf override 😀",
    )
    all_sources = base + [
        ("repo/nested/CLAUDE.md", "claude", "nested claude 中文"),
        leaf,
    ]
    for identity, sources in [
        ("resolved-all", all_sources),
        ("resolved-agents", base + [leaf]),
        ("dynamic", base + [(leaf[0], leaf[1], "changed 中文🌊")]),
        (
            "fallback",
            base + [("repo/nested/leaf/AGENTS.md", "agents", "ignored leaf AGENTS")],
        ),
        ("directory-skip", base),
    ]:
        actual, expected = val(identity), expected_rules(work, sources)
        assert actual["content"] == expected["content"], (identity, actual)
        assert actual["sources"] == expected["sources"], (identity, actual)
        assert actual["project_root"] == {
            "variant": "some",
            "fields": {"value": str(work / "repo")},
        }, actual
    family = val("resolved-family")
    assert family["content"] == expected_rules(work, all_sources)["content"]
    assert family["sources"] == expected_rules(work, all_sources)["sources"]
    assert val("load-family")["fields"] == {
        "path": str(work / leaf[0]),
        "content": family["content"],
    }
    assert "other isolated rules" in val("other-rules")["content"]
    assert val("empty-rules") == {"content": "", "sources": [], "is_empty": True}
    assert val("empty-load")["variant"] == "none"
    posts = [
        json.loads(line) for line in (work / "embedding.jsonl").read_text().splitlines()
    ]
    assert all(
        post["path"] == "/v1/embeddings"
        and post["request"]["model"] == "fixture-中文-embedding"
        for post in posts
    )
    assert [post["request"]["input"] for post in posts[: len(CHUNKS)]] == CHUNKS
    assert [
        post["request"]["input"] for post in posts if post["mode"] == "second-503"
    ] == ["失败😀文", "档甲乙丙"]
    assert sum(post["mode"] == "bad-json" for post in posts) == 1
    assert sum(post["mode"] == "empty-data" for post in posts) == 1
    return {
        "calls": len(rows),
        "embedding_posts": len(posts),
        "ranked_chunks": len(CHUNKS),
        "proof": "Session-local memory; no durable restart guarantee",
    }


def compare(paths):
    facts = []
    summaries = []
    for path in paths:
        evidence = json.loads(path.read_text())
        work = Path(evidence["work"])
        summaries.append(verify(work, evidence))
        ids = {}

        def canonical(value, work=work, ids=ids):
            if isinstance(value, str):
                return value.replace(str(work), "$WORK")
            if isinstance(value, list):
                return [canonical(item) for item in value]
            if isinstance(value, dict):
                if value.get("kind") == "handle" and "value" in value:
                    return canonical(value["value"])
                if set(value) == {"id", "generation", "kind"}:
                    identity = value["id"]
                    ids.setdefault(identity, f"$HANDLE{len(ids)}")
                    return {**value, "id": ids[identity]}
                if value.get("kind") == "map" and isinstance(value.get("value"), list):
                    return {
                        entry["key"]["value"]: canonical(entry["value"])
                        for entry in value["value"]
                    }
                return {key: canonical(item) for key, item in value.items()}
            return value

        facts.append(canonical(evidence["rows"]))
    for other in facts[1:]:
        assert len(other) == len(facts[0])
        for expected, actual in zip(facts[0], other):
            assert expected == actual, (expected["id"], expected, actual)
    return {
        "native_comparison": "equal",
        "consumers": summaries,
        "normalization": [
            "isolated working root",
            "Host opaque handle IDs by first use",
            "typed handle wrapper and native handle projection",
            "map association order",
        ],
        "preserved": [
            "error code/message/operation/retryability/details/generation",
            "ToolResult failure and raw output",
            "typed instruction record/path/Option",
            "UTF-8 content",
            "ordered sources and ranks",
        ],
    }


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--work", type=Path)
    parser.add_argument("--compare", type=Path, nargs=3)
    parser.add_argument("--serve", action="store_true")
    parser.add_argument("--action", type=Path)
    parser.add_argument("--verify", type=Path)
    args = parser.parse_args()
    if args.compare:
        print(json.dumps(compare(args.compare), ensure_ascii=False, indent=2))
    elif args.serve:
        serve(args.work)
    elif args.action:
        action(args.work, json.loads(args.action.read_text()))
    elif args.verify:
        print(json.dumps(verify(args.work, json.loads(args.verify.read_text()))))
    else:
        prepare(args.work)
