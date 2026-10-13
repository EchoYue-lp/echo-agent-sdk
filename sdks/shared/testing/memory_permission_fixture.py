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

SCENARIOS = json.loads(r"""{
  "classification": "unexecuted candidate scenarios, no Host evidence yet",
  "catalog_sha256": "ecca20dc7eb2921b76bb9db23c45fbc4dab27eb510f2d80f4fdd6d76ab321cc0",
  "scenarios": [
    {
      "id": "raw-native-store-resource",
      "mechanism": "real framework InMemoryStore via accepted resource receiver",
      "steps": [
        {
          "operation": "echo_state::memory::store::InMemoryStore::new",
          "receiver": "$agent",
          "args": [
            "$sessionA"
          ],
          "save": "$store"
        },
        {
          "operation": "memory.resource.put",
          "receiver": "$sessionA",
          "args": [
            "$store",
            [
              "sdk",
              "中文"
            ],
            "one",
            {
              "content": "duplicate-needle中文🌊"
            }
          ]
        },
        {
          "operation": "memory.resource.put",
          "receiver": "$sessionA",
          "args": [
            "$store",
            [
              "sdk",
              "中文"
            ],
            "two",
            {
              "content": "duplicate-needle中文🌊"
            }
          ]
        },
        {
          "operation": "echo_state::memory::store::InMemoryStore::put_raw",
          "receiver": "$agent",
          "args": [
            "$sessionA",
            "$store",
            {
              "namespace": [
                "sdk",
                "中文"
              ],
              "key": "expired",
              "value": {
                "content": "expired-needle"
              },
              "created_at": 1,
              "updated_at": 1,
              "score": null,
              "importance": 5.0,
              "last_accessed": null,
              "expires_at": 2
            }
          ]
        },
        {
          "operation": "memory.resource.get",
          "receiver": "$sessionA",
          "args": [
            "$store",
            [
              "sdk",
              "中文"
            ],
            "expired"
          ],
          "assert": "raw get preserves expires_at=2 before explicit prune; do not invent implicit deletion"
        },
        {
          "operation": "memory.resource.search_with",
          "receiver": "$sessionA",
          "args": [
            "$store",
            [
              "sdk",
              "中文"
            ],
            {
              "text": "duplicate-needle",
              "limit": 5,
              "mode": "Keyword"
            }
          ],
          "assert": "exact two original keys, unicode preserved"
        },
        {
          "operation": "memory.resource.prune_expired",
          "receiver": "$sessionA",
          "args": [
            "$store",
            [
              "sdk",
              "中文"
            ]
          ],
          "assert": "1 then 0; expired get null, two others retained"
        },
        {
          "operation": "memory.resource.dedup_by_content",
          "receiver": "$sessionA",
          "args": [
            "$store",
            [
              "sdk",
              "中文"
            ]
          ],
          "assert": "1 then 0; one exact original content survives"
        },
        {
          "operation": "memory.resource.list",
          "receiver": "$sessionA",
          "args": [
            "$store",
            [
              "sdk",
              "中文"
            ]
          ],
          "assert": "one item only; truncated false"
        },
        {
          "operation": "memory.resource.list_namespaces",
          "receiver": "$sessionA",
          "args": [
            "$store",
            [
              "sdk"
            ]
          ],
          "assert": "[[sdk,中文]]"
        },
        {
          "operation": "memory.resource.get",
          "receiver": "$sessionB",
          "args": [
            "$store",
            [
              "sdk",
              "中文"
            ],
            "one"
          ],
          "expect_error": "owner mismatch, exact actual code to record; A remains valid"
        }
      ],
      "coverage_limits": [
        "Keyword only, no semantic embedder score claim",
        "resource APIs use real native Store, test-only backend dedup is not proof"
      ]
    },
    {
      "id": "reverse-store-live-authority-replacement",
      "mechanism": "two registered Store callbacks A/B, exact Session Agent set_store changes main CRUD authority",
      "steps": [
        {
          "operation": "echo_agent::agent::react::ReactAgent::set_store",
          "receiver": "$agent",
          "args": [
            "$sessionA",
            "$extensionA"
          ]
        },
        {
          "operation": "memory.store.put",
          "receiver": "$sessionA",
          "args": [
            [
              "sdk",
              "中文"
            ],
            "key",
            {
              "中文": "🌊",
              "__proto__": {
                "id": "domain",
                "generation": "literal"
              }
            }
          ]
        },
        {
          "operation": "memory.store.get",
          "receiver": "$sessionA",
          "args": [
            [
              "sdk",
              "中文"
            ],
            "key"
          ],
          "assert": "A callback read same exact value; trace has Host issued invocation/extension IDs"
        },
        {
          "operation": "echo_agent::agent::react::ReactAgent::set_store",
          "receiver": "$agent",
          "args": [
            "$sessionA",
            "$extensionB"
          ]
        },
        {
          "operation": "memory.store.get",
          "receiver": "$sessionA",
          "args": [
            [
              "sdk",
              "中文"
            ],
            "key"
          ],
          "assert": "B missing/null; A old callback is not called after switch"
        },
        {
          "operation": "memory.store.put",
          "receiver": "$sessionA",
          "args": [
            [
              "sdk",
              "中文"
            ],
            "key",
            "B-new"
          ]
        },
        {
          "operation": "memory.store.search",
          "receiver": "$sessionA",
          "args": [
            [
              "sdk",
              "中文"
            ],
            "B-new",
            5
          ],
          "assert": "B key only; A inspect still original"
        },
        {
          "operation": "memory.store.delete",
          "receiver": "$sessionA",
          "args": [
            [
              "sdk",
              "中文"
            ],
            "key"
          ],
          "assert": "true then false; B get null"
        }
      ],
      "faults": [
        "arm store_put native OSError before mutation; Host returns error, A/B counters unchanged except invocation trace, retry returns new real commit",
        "invalid namespace [sdk,3] fails before reverse callback; preserve typed code/method metadata"
      ],
      "coverage_limits": [
        "set_store is raw Store replacement, does not install MemoryLayerManager/promoter",
        "do not call unaccepted set_memory_store"
      ]
    },
    {
      "id": "real-automatic-recall-default-store",
      "mechanism": "enable_memory default real FileStore plus real Run, model input is independent consumer evidence",
      "steps": [
        {
          "operation": "memory.store.put",
          "receiver": "$sessionA",
          "args": [
            [
              "agent",
              "memories"
            ],
            "approved-active",
            {
              "content": "SDK-Recall-Active 中文🌊 prefers Rust",
              "meta": {
                "memory_type": "user_preference",
                "confidence": 1.0,
                "stability": 0.85,
                "recall_weight": 0.9,
                "risk": "low",
                "status": "active",
                "source": "explicit_save",
                "topic": "sdk-governance",
                "revision_count": 0,
                "recall_count": 0,
                "provenance": {
                  "trust": "user",
                  "evidence": [
                    {
                      "role": "user",
                      "quote": "SDK-Recall-Active 中文🌊 prefers Rust"
                    }
                  ],
                  "approval": {
                    "approval_id": "candidate-fixture-reviewed-snapshot",
                    "reviewer": "test-fixture-owner",
                    "approved_at": 1
                  }
                }
              }
            }
          ]
        },
        {
          "operation": "memory.store.put",
          "receiver": "$sessionA",
          "args": [
            [
              "agent",
              "memories"
            ],
            "unapproved-draft",
            {
              "content": "SDK-Recall-Draft do-not-inject",
              "meta": {
                "memory_type": "user_preference",
                "confidence": 1.0,
                "stability": 0.85,
                "recall_weight": 0.9,
                "risk": "low",
                "status": "draft",
                "source": "explicit_save",
                "topic": "sdk-governance",
                "revision_count": 0,
                "recall_count": 0,
                "provenance": {
                  "trust": "user",
                  "evidence": [
                    {
                      "role": "user",
                      "quote": "SDK-Recall-Active 中文🌊 prefers Rust"
                    }
                  ]
                }
              }
            }
          ]
        },
        {
          "operation": "memory.store.put",
          "receiver": "$sessionA",
          "args": [
            [
              "agent",
              "memories"
            ],
            "legacy-unapproved",
            {
              "content": "SDK-Recall-Legacy do-not-inject",
              "meta": {
                "memory_type": "user_preference",
                "confidence": 1.0,
                "stability": 0.85,
                "recall_weight": 0.9,
                "risk": "low",
                "status": "active",
                "source": "explicit_save",
                "topic": "sdk-governance",
                "revision_count": 0,
                "recall_count": 0
              }
            }
          ]
        },
        {
          "operation": "memory.store.get",
          "receiver": "$sessionA",
          "args": [
            [
              "agent",
              "memories"
            ],
            "approved-active"
          ],
          "assert": "same exact seeded provenance before Run"
        }
      ],
      "runtime_action": "start accepted Run chat SDK-Recall; wait actual settlement; inspect loopback actual model request includes Active marker exactly once in current user memory_context, excludes Draft/Legacy; journal memory_recalled count agrees; afterwards get metadata same content/provenance with telemetry increase only",
      "coverage_limits": [
        "fixture seed uses existing generic Store value grammar, not a new typed activation API",
        "approved test fixture data does not claim production activation workflow evidence",
        "separate Store replacement scenario runs fresh Session to avoid prior memory-context projection"
      ]
    },
    {
      "id": "permission-rules-effects-context-and-session-isolation",
      "mechanism": "same AgentDefinition two real Sessions, independent live PermissionService owners",
      "steps": [
        {
          "operation": "permission.clear_rules",
          "receiver": "$sessionA",
          "args": []
        },
        {
          "operation": "permission.clear_cache",
          "receiver": "$sessionA",
          "args": []
        },
        {
          "operation": "permission.add_rule",
          "receiver": "$sessionA",
          "args": [
            {
              "matcher": {
                "type": "tool",
                "name": "candidate_write"
              },
              "behavior": {
                "type": "deny",
                "reason": "fixture deny 中文🌊"
              },
              "source": "session",
              "description": "exact accepted rule"
            }
          ]
        },
        {
          "operation": "permission.all_rules",
          "receiver": "$sessionA",
          "args": [],
          "assert": "exact rule/source/description; B rules empty"
        },
        {
          "operation": "permission.check_with_permissions_result_in_mode_and_context",
          "receiver": "$sessionA",
          "args": [
            "candidate_write",
            {
              "path": "$work/effect.txt"
            },
            [
              "write"
            ],
            "default",
            {
              "scope_id": "shared-scope-中文🌊",
              "request_id": "permission-native-request",
              "session_id": "$sessionA.acp_id",
              "agent_name": "memory-fixture",
              "timeout_ms": 1000,
              "permission": {
                "working_directory": "$work",
                "affected_files": [
                  "$work/effect.txt"
                ],
                "estimated_impact": "one fixture file",
                "metadata": {
                  "中文": "🌊",
                  "__proto__": "plain data"
                }
              },
              "classifier": {
                "workspace_path": "$work",
                "project_type": "fixture",
                "recent_files": [
                  "effect.txt"
                ],
                "risk_context": {
                  "has_sensitive_files": false,
                  "is_destructive": false,
                  "directory_depth": 1,
                  "repetition_count": 0
                }
              }
            }
          ],
          "assert": "deny exact reason and updated_input null, no effect file"
        },
        {
          "operation": "permission.remove_rule",
          "receiver": "$sessionA",
          "args": [
            {
              "matcher": {
                "type": "tool",
                "name": "candidate_write"
              },
              "behavior": {
                "type": "deny",
                "reason": "fixture deny 中文🌊"
              },
              "source": "session",
              "description": "exact accepted rule"
            }
          ],
          "assert": "true then false"
        },
        {
          "operation": "permission.record_approval",
          "receiver": "$sessionA",
          "args": [
            "same-scope",
            "candidate_write",
            {
              "x": 1
            },
            "session"
          ]
        },
        {
          "operation": "permission.is_approved",
          "receiver": "$sessionA",
          "args": [
            "same-scope",
            "candidate_write",
            {
              "x": 1
            }
          ],
          "assert": "true; x2 false; different scope false; B same exact scope/tool/input false"
        },
        {
          "operation": "permission.revoke_cache",
          "receiver": "$sessionA",
          "args": [
            "same-scope",
            "candidate_write"
          ],
          "assert": "A false; B independent previously recorded approval remains"
        },
        {
          "operation": "permission.record_approval",
          "receiver": "$sessionA",
          "args": [
            "same-scope",
            "candidate_write",
            {
              "x": 1
            },
            "once"
          ],
          "assert": "never cached"
        },
        {
          "operation": "permission.record_approval",
          "receiver": "$sessionA",
          "args": [
            "same-scope",
            "candidate_write",
            {
              "x": 1
            },
            "session_tool"
          ],
          "assert": "A x1/x2 true; B false; revokeA false"
        },
        {
          "operation": "permission.stats",
          "receiver": "$sessionA",
          "args": [],
          "assert": "exact canonical u64 projection counts; ttl_ms records actual configured TTL"
        },
        {
          "operation": "permission.cleanup_expired",
          "receiver": "$sessionA",
          "args": [],
          "assert": "0 when entries younger than actual TTL; do not claim expiration happened"
        }
      ],
      "runtime_action": "register accepted HumanLoopProvider before Sessions; issue context check without deny rule and with strict; reverse callback must see request/session/agent ids, original args and approval_context fields; answer modified_args scope once and check updated_input identical; do not infer effect executed from check result",
      "effect_action": "separate real Agent Run selects registered candidate_write Tool; denial callback means tool counter0/file absent, approval means actual tool counter1/file content; requires fixture model tool-call mode addition before publishing candidate",
      "coverage_limits": [
        "no accepted shortTTL setter or fake clock; actual30min boundary opt-in only",
        "PermissionCheck approval_receipt currently omitted by Host check_value; existing accepted response does not expose new field, log source potential contract gap rather than invent JSON",
        "direct facade tool execution alone is not permission enforcement proof"
      ]
    }
  ],
  "limitations": [
    "short permission TTL configuration not accepted",
    "no new recall/install/clock API",
    "all actual Host results pending parent approved publish/run"
  ]
}
""")

OPERATIONS = {
    "store_put",
    "store_get",
    "store_search",
    "store_search_with",
    "store_delete",
    "store_list_namespaces",
    "store_list",
    "store_prune_expired",
    "store_dedup_by_content",
}


class Backend:
    def __init__(self, clock=time.time):
        self.lock = threading.RLock()
        self.items, self.calls = {}, []
        self.clock, self.fail_next = clock, None

    def call(self, owner, operation, value):
        if owner not in {"A", "B"} or operation not in OPERATIONS:
            raise ValueError("unsupported fixture Store operation or authority")
        with self.lock:
            self.calls.append(
                {
                    "authority": owner,
                    "operation": operation,
                    "input": copy.deepcopy(value),
                }
            )
            if self.fail_next == operation:
                self.fail_next = None
                raise OSError("injected reverse Store failure")
            namespace = tuple(value.get("namespace", ()))
            if operation != "store_list_namespaces" and (
                not namespace or any(not isinstance(part, str) for part in namespace)
            ):
                raise ValueError("namespace must be a nonempty string array")
            identity = (owner, namespace, value.get("key"))
            if operation == "store_put":
                now = int(self.clock())
                previous = self.items.get(identity)
                self.items[identity] = {
                    "namespace": list(namespace),
                    "key": value["key"],
                    "value": copy.deepcopy(value["value"]),
                    "created_at": previous["created_at"] if previous else now,
                    "updated_at": now,
                    "score": None,
                    "importance": 5.0,
                    "last_accessed": None,
                    "expires_at": None,
                }
                return None
            if operation == "store_get":
                return copy.deepcopy(self.items.get(identity))
            if operation == "store_delete":
                return self.items.pop(identity, None) is not None
            if operation == "store_list_namespaces":
                prefix = value.get("prefix")
                return [
                    list(ns)
                    for ns in sorted(
                        {
                            ns
                            for o, ns, _ in self.items
                            if o == owner
                            and (prefix is None or ns[: len(prefix)] == tuple(prefix))
                        }
                    )
                ]
            bucket = sorted(
                [
                    item
                    for (o, ns, _), item in self.items.items()
                    if o == owner and ns == namespace
                ],
                key=lambda item: item["key"],
            )
            if operation == "store_list":
                return copy.deepcopy(bucket)
            if operation in {"store_search", "store_search_with"}:
                query = value["query"]
                if isinstance(query, dict) and query.get("mode") != {"kind": "keyword"}:
                    raise ValueError("fixture declares keyword search only")
                text = query["text"] if isinstance(query, dict) else query
                limit = int(
                    query["limit"] if isinstance(query, dict) else value["limit"]
                )
                result = [
                    copy.deepcopy(item)
                    for item in bucket
                    if text in json.dumps(item["value"], ensure_ascii=False)
                ]
                for item in result:
                    item["score"] = 1.0
                return result[:limit]
            if operation == "store_prune_expired":
                removed = [
                    item
                    for item in bucket
                    if item["expires_at"] is not None
                    and item["expires_at"] < self.clock()
                ]
            else:
                seen, removed = set(), []
                for item in bucket:
                    content = json.dumps(
                        item["value"], sort_keys=True, ensure_ascii=False
                    )
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
                    self.reply(
                        {"store_calls": backend.calls, "model_requests": model_inputs}
                    )

        def do_POST(self):
            length = int(self.headers.get("Content-Length", "0"))
            if length > 2097152:
                self.reply({"error": "fixture request too large"}, 413)
                return
            value = json.loads(self.rfile.read(length))
            if self.path == "/store":
                try:
                    result = backend.call(
                        value["authority"], value["operation"], value["input"]
                    )
                    self.reply(
                        {
                            "outcome": "result",
                            "result": {
                                "operation": value["operation"],
                                "value": result,
                            },
                        }
                    )
                except (KeyError, ValueError, TypeError, OSError) as error:
                    self.reply(
                        {
                            "outcome": "error",
                            "error": {
                                "code": "extension_failed",
                                "message": str(error),
                                "retryable": "never",
                            },
                        }
                    )
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
                    with (args.work / "model-requests.jsonl").open(
                        "a", encoding="utf-8"
                    ) as output:
                        output.write(json.dumps(value, ensure_ascii=False) + "\n")
                user = next(
                    (
                        message.get("content", "")
                        for message in reversed(value["messages"])
                        if message.get("role") == "user"
                    ),
                    "",
                )
                effect_phase = (
                    "deny"
                    if "permission-effect-deny" in user
                    else "allow"
                    if "permission-effect-allow" in user
                    else None
                )
                tool_completed = any(
                    message.get("role") == "tool" for message in value["messages"]
                )
                tool_call = effect_phase is not None and not tool_completed
                tool = {
                    "index": 0,
                    "id": "fixture-tool-" + str(effect_phase),
                    "type": "function",
                    "function": {
                        "name": "candidate_write",
                        "arguments": json.dumps(
                            {
                                "path": str(
                                    args.work / ("effect-" + str(effect_phase) + ".txt")
                                ),
                                "phase": effect_phase,
                            }
                        ),
                    },
                }
                if value.get("stream"):
                    self.send_response(200)
                    self.send_header("Content-Type", "text/event-stream")
                    self.end_headers()
                    delta = (
                        {"role": "assistant", "tool_calls": [tool]}
                        if tool_call
                        else {"role": "assistant", "content": "SDK consumer complete"}
                    )
                    chunk = {
                        "id": "memory-fixture",
                        "choices": [
                            {
                                "index": 0,
                                "delta": delta,
                                "finish_reason": "tool_calls" if tool_call else "stop",
                            }
                        ],
                    }
                    self.wfile.write(
                        ("data: " + json.dumps(chunk) + "\n\ndata: [DONE]\n\n").encode()
                    )
                else:
                    if tool_call:
                        nonstream_tool = {
                            key: item for key, item in tool.items() if key != "index"
                        }
                        message = {
                            "role": "assistant",
                            "content": None,
                            "tool_calls": [nonstream_tool],
                        }
                    else:
                        message = {
                            "role": "assistant",
                            "content": "SDK consumer complete",
                        }
                    self.reply(
                        {
                            "id": "memory-fixture",
                            "choices": [
                                {
                                    "index": 0,
                                    "message": message,
                                    "finish_reason": "tool_calls"
                                    if tool_call
                                    else "stop",
                                }
                            ],
                        }
                    )
            else:
                self.reply({"error": "unknown route"}, 404)

        def log_message(self, *_args):
            pass

    server = ThreadingHTTPServer(("127.0.0.1", 0), Handler)
    url = f"http://127.0.0.1:{server.server_port}"
    config = {
        "schema_version": 1,
        "default_agent": {
            "model": {
                "provider": "fixture",
                "name": "memory-model",
                "base_url": url + "/v1/chat/completions",
                "api_protocol": "chat_completions",
                "auth_token": "fixture",
            },
            "agent": {
                "name": "memory-fixture",
                "max_iterations": 2,
                "enable_memory": True,
                "memory_path": str(args.work / "memory.jsonl"),
                "enable_tools": True,
            },
        },
        "sdk_profile": {"state_root": str(args.work / "state")},
    }
    (args.work / "scenarios.json").write_text(
        json.dumps(SCENARIOS, ensure_ascii=False), encoding="utf-8"
    )
    (args.work / "backend.json").write_text(json.dumps({"url": url}), encoding="utf-8")
    (args.work / "host.json").write_text(json.dumps(config), encoding="utf-8")
    (args.work / "ready").touch()
    server.serve_forever()


if __name__ == "__main__":
    main()
