"""Candidate test-only loopback model and reverse Store backend.

Store results are native JSON; language callbacks own WireValue conversion.
No permission, Host handles, Run status, ACK or memory activation authority.
"""
from __future__ import annotations
import argparse
import copy
import json
import threading
import time
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path

OPERATIONS = {"store_put", "store_get", "store_search", "store_search_with", "store_delete", "store_list_namespaces", "store_list", "store_prune_expired", "store_dedup_by_content"}


class Backend:
    def __init__(self, clock=time.time):
        self.lock = threading.RLock()
        self.items, self.calls = {}, []
        self.clock, self.fail_next = clock, None

    def call(self, owner, operation, value):
        if owner not in {"A", "B"} or operation not in OPERATIONS:
            raise ValueError("unsupported fixture Store operation or authority")
        with self.lock:
            self.calls.append({"authority": owner, "operation": operation, "input": copy.deepcopy(value)})
            if self.fail_next == operation:
                self.fail_next = None
                raise OSError("injected reverse Store failure")
            namespace = tuple(value.get("namespace", ()))
            if operation != "store_list_namespaces" and (not namespace or any(not isinstance(part, str) for part in namespace)):
                raise ValueError("namespace must be a nonempty string array")
            identity = (owner, namespace, value.get("key"))
            if operation == "store_put":
                now = int(self.clock())
                previous = self.items.get(identity)
                self.items[identity] = {"namespace": list(namespace), "key": value["key"], "value": copy.deepcopy(value["value"]), "created_at": previous["created_at"] if previous else now, "updated_at": now, "score": None, "importance": 5.0, "last_accessed": None, "expires_at": None}
                return None
            if operation == "store_get":
                return copy.deepcopy(self.items.get(identity))
            if operation == "store_delete":
                return self.items.pop(identity, None) is not None
            if operation == "store_list_namespaces":
                prefix = value.get("prefix")
                return [list(ns) for ns in sorted({ns for o, ns, _ in self.items if o == owner and (prefix is None or ns[:len(prefix)] == tuple(prefix))})]
            bucket = sorted([item for (o, ns, _), item in self.items.items() if o == owner and ns == namespace], key=lambda item: item["key"])
            if operation == "store_list":
                return copy.deepcopy(bucket)
            if operation in {"store_search", "store_search_with"}:
                query = value["query"]
                if isinstance(query, dict) and query.get("mode") != {"kind": "keyword"}:
                    raise ValueError("fixture declares keyword search only")
                text = query["text"] if isinstance(query, dict) else query
                limit = int(query["limit"] if isinstance(query, dict) else value["limit"])
                result = [copy.deepcopy(item) for item in bucket if text in json.dumps(item["value"], ensure_ascii=False)]
                for item in result:
                    item["score"] = 1.0
                return result[:limit]
            if operation == "store_prune_expired":
                removed = [item for item in bucket if item["expires_at"] is not None and item["expires_at"] < self.clock()]
            else:
                seen, removed = set(), []
                for item in bucket:
                    content = json.dumps(item["value"], sort_keys=True, ensure_ascii=False)
                    if content in seen:
                        removed.append(item)
                    seen.add(content)
            for item in removed:
                del self.items[(owner, namespace, item["key"])]
            return str(len(removed))


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--work", type=Path, required=True)
    args = parser.parse_args()
    args.work.mkdir(parents=True, exist_ok=True)
    backend, model_inputs = Backend(), []
    class Handler(BaseHTTPRequestHandler):
        def reply(self, value, status=200):
            content = json.dumps(value, ensure_ascii=False).encode()
            self.send_response(status)
            self.send_header("Content-Type", "application/json")
            self.send_header("Content-Length", str(len(content)))
            self.end_headers()
            self.wfile.write(content)
        def do_GET(self):
            if self.path != "/inspect":
                self.reply({"error": "unknown route"}, 404)
            else:
                with backend.lock:
                    self.reply({"store_calls": backend.calls, "model_requests": model_inputs})
        def do_POST(self):
            length = int(self.headers.get("Content-Length", "0"))
            if length > 2097152:
                self.reply({"error": "fixture request too large"}, 413)
                return
            value = json.loads(self.rfile.read(length))
            if self.path == "/store":
                try:
                    result = backend.call(value["authority"], value["operation"], value["input"])
                    self.reply({"outcome": "result", "result": {"operation": value["operation"], "value": result}})
                except (KeyError, ValueError, TypeError, OSError) as error:
                    self.reply({"outcome": "error", "error": {"code": "extension_failed", "message": str(error), "retryable": "never"}})
            elif self.path == "/fault":
                if value["operation"] not in OPERATIONS:
                    self.reply({"error": "unsupported operation"}, 400)
                else:
                    with backend.lock:
                        backend.fail_next = value["operation"]
                    self.reply({"armed": value["operation"]})
            elif self.path == "/v1/chat/completions":
                with backend.lock:
                    model_inputs.append(copy.deepcopy(value))
                    with (args.work / "model-requests.jsonl").open("a", encoding="utf-8") as output:
                        output.write(json.dumps(value, ensure_ascii=False) + "\n")
                user = next((message.get("content", "") for message in reversed(value["messages"]) if message.get("role") == "user"), "")
                effect_phase = "deny" if "permission-effect-deny" in user else "allow" if "permission-effect-allow" in user else None
                tool_completed = any(message.get("role") == "tool" for message in value["messages"])
                tool_call = effect_phase is not None and not tool_completed
                tool = {"index": 0, "id": "fixture-tool-" + str(effect_phase), "type": "function", "function": {"name": "candidate_write", "arguments": json.dumps({"path": str(args.work / ("effect-" + str(effect_phase) + ".txt")), "phase": effect_phase})}}
                if value.get("stream"):
                    self.send_response(200)
                    self.send_header("Content-Type", "text/event-stream")
                    self.end_headers()
                    delta = {"role": "assistant", "tool_calls": [tool]} if tool_call else {"role": "assistant", "content": "SDK consumer complete"}
                    chunk = {"id": "memory-fixture", "choices": [{"index": 0, "delta": delta, "finish_reason": "tool_calls" if tool_call else "stop"}]}
                    self.wfile.write(("data: " + json.dumps(chunk) + "\n\ndata: [DONE]\n\n").encode())
                else:
                    if tool_call:
                        nonstream_tool = {key: item for key, item in tool.items() if key != "index"}
                        message = {"role": "assistant", "content": None, "tool_calls": [nonstream_tool]}
                    else:
                        message = {"role": "assistant", "content": "SDK consumer complete"}
                    self.reply({"id": "memory-fixture", "choices": [{"index": 0, "message": message, "finish_reason": "tool_calls" if tool_call else "stop"}]})
            else:
                self.reply({"error": "unknown route"}, 404)
        def log_message(self, *_args):
            pass
    server = ThreadingHTTPServer(("127.0.0.1", 0), Handler)
    url = f"http://127.0.0.1:{server.server_port}"
    config = {"schema_version": 1, "default_agent": {"model": {"provider": "fixture", "name": "memory-model", "base_url": url + "/v1/chat/completions", "api_protocol": "chat_completions", "auth_token": "fixture"}, "agent": {"name": "memory-fixture", "max_iterations": 2, "enable_memory": True, "memory_path": str(args.work / "memory.jsonl"), "enable_tools": True}}, "sdk_profile": {"state_root": str(args.work / "state")}}
    (args.work / "backend.json").write_text(json.dumps({"url": url}), encoding="utf-8")
    (args.work / "host.json").write_text(json.dumps(config), encoding="utf-8")
    (args.work / "ready").touch()
    server.serve_forever()


if __name__ == "__main__":
    main()
