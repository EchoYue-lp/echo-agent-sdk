"""复用原 Rust PluginRegistry 输入；只准备文件，不实现注册或执行逻辑。"""

from __future__ import annotations

import argparse
import json
from pathlib import Path

from file_sha256 import file_sha256


def prepare(work: Path) -> Path:
    root = work / "retained-plugin"
    for relative in ("skills/example", "agents", "hooks"):
        (root / relative).mkdir(parents=True, exist_ok=True)
    (root / "skills/example/SKILL.md").write_text(
        "---\nname: example\ndescription: Retained plugin control\n---\n中文🙂\n",
        encoding="utf-8",
    )
    (root / "agents/reviewer.md").write_text(
        "---\nname: reviewer\ndescription: Retained plugin control\n---\nReview.\n",
        encoding="utf-8",
    )
    (root / "hooks/hooks.yaml").write_text("{}\n", encoding="utf-8")
    (root / "mcp.json").write_text(
        json.dumps(
            {
                "$schema": "https://agent-plugins.org/schemas/1.0.0/mcp.schema.json",
                "mcpServers": {},
            }
        ),
        encoding="utf-8",
    )
    (root / "plugin.json").write_text(
        json.dumps(
            {
                "$schema": "https://agent-plugins.org/schemas/1.0.0/plugin.schema.json",
                "name": "retained.tools",
                "version": "1.0.0",
                "description": "Retained 中文🙂",
            },
            ensure_ascii=False,
        ),
        encoding="utf-8",
    )
    return root


def verify(work: Path) -> dict:
    """核对原透明代理的真实帧；不合成成功、路由或执行状态。"""
    retired = json.loads(
        (Path(__file__).parents[0] / "a2a_lsp_retirement_contract.json").read_text(
            encoding="utf-8"
        )
    )
    requests = [
        json.loads(line)
        for line in (work / "client-frames.jsonl")
        .read_text(encoding="utf-8")
        .splitlines()
        if line
    ]
    frames = [
        json.loads(line)
        for line in (work / "frames.jsonl").read_text(encoding="utf-8").splitlines()
        if line
    ]
    replies = {
        str(row["id"]): row for row in frames if "id" in row and "method" not in row
    }
    initialize = next(row for row in requests if row.get("method") == "initialize")
    hello = initialize["params"]["clientCapabilities"]["_meta"]["echo_agent"]
    current_digests = json.loads(
        (Path(__file__).parents[1] / "contract-digests.json").read_text(
            encoding="utf-8"
        )
    )
    assert hello["contract_digest"] == current_digests["contract_digest"]
    assert hello["source_contract_digest"] == current_digests["source_contract_digest"]
    assert hello["required_features"] == hello["required_capabilities"] == []
    capability = replies[str(initialize["id"])]["result"]["agentCapabilities"]["_meta"][
        "echo_agent"
    ]
    assert "a2a" not in capability["features"] and "lsp" not in capability["features"]
    assert "mcp" in capability["features"]
    process = json.loads((work / "host-process.json").read_text(encoding="utf-8"))
    used = json.loads((work / "host-used.json").read_text(encoding="utf-8"))
    assert process["mode"] == used["proxy_mode"] == "normal"
    assert process["host_command"] == used["host"]
    assert file_sha256(used["host"]) == used["host_sha256"]
    assert process["observed_host_exit"] == 0
    methods = {item["methods"][0] for item in retired["families"]}
    old_sources = {item["operation"] for item in retired["routes"] if item["operation"]}
    family_errors = []
    source_errors = []
    typed_errors = []
    typed_positives = []
    positives = []
    for request in requests:
        if "method" not in request or "id" not in request:
            continue
        reply = replies[str(request["id"])]
        method = request["method"]
        params = request.get("params", {})
        operation = params.get("operation")
        if method in methods:
            assert reply["error"]["code"] == -32601 and "result" not in reply
            family_errors.append({"request": request, "reply": reply})
        elif operation in old_sources:
            error = reply["error"]["data"]
            assert error["code"] == "invalid_value"
            assert "not a canonical route of this contract" in error["message"]
            assert error["details"]["facade"]["operation"] == operation
            source_errors.append({"request": request, "reply": reply})
        elif operation == "retirement.probe":
            error = reply["error"]["data"]
            assert error["code"] == "invalid_value"
            if "invalid facade argument" in error["message"]:
                typed_errors.append({"request": request, "reply": reply})
            else:
                assert "not a canonical route of this contract" in error["message"]
                typed_positives.append({"request": request, "reply": reply})
        elif (
            operation
            == "echo_core::plugin::registry::PluginRegistry::validate_plugin_dir"
        ):
            value = reply["result"]["value"]
            assert value["kind"] == "list" and len(value["value"]) == 2
            components = value["value"][1]
            assert components["kind"] == "record"
            assert (
                components["value"]["type_id"]
                == "echo_core::plugin::registry::ResolvedComponents"
            )
            fields = {
                item["name"]: item["value"] for item in components["value"]["fields"]
            }
            assert set(fields) == {
                "skill_dirs",
                "agent_files",
                "hooks_file",
                "mcp_config_file",
                "diagnostics",
            }
            assert (
                len(fields["skill_dirs"]["value"])
                == len(fields["agent_files"]["value"])
                == 1
            )
            assert fields["skill_dirs"]["value"][0]["value"]["path"] == str(
                (work / "retained-plugin/skills").resolve()
            )
            assert fields["agent_files"]["value"][0]["value"]["path"] == str(
                (work / "retained-plugin/agents/reviewer.md").resolve()
            )
            for name, relative in [
                ("hooks_file", "hooks/hooks.yaml"),
                ("mcp_config_file", "mcp.json"),
            ]:
                assert fields[name]["value"]["variant"] == "some"
                assert fields[name]["value"]["fields"][0]["value"]["value"][
                    "path"
                ] == str((work / "retained-plugin" / relative).resolve())
            positives.append({"request": request, "reply": reply})
        elif operation == "echo_core::agent::Agent::name":
            assert reply["result"]["value"] == {"kind": "string", "value": "echo-agent"}
            positives.append({"request": request, "reply": reply})
        elif operation == "content-guard.redact_exact":
            assert method == "_echo_agent/content-guard/op"
            assert reply["result"]["value"] == {
                "kind": "string",
                "value": "中😀 [REDACTED:EMAIL] 尾",
            }
            positives.append({"request": request, "reply": reply})
    assert len(family_errors) == 39 and len(source_errors) == 19
    assert {
        (item["request"]["method"], item["request"]["params"]["operation"])
        for item in family_errors
    } == {
        (family["methods"][0], operation)
        for family in retired["families"]
        for operation in family["operations"]
    }
    assert {
        item["request"]["params"]["operation"] for item in source_errors
    } == old_sources
    assert len(typed_errors) == len(typed_positives) == 3

    def typed_key(item):
        value = item["request"]["params"]["arguments"][0]
        inner = value["value"]
        return (
            value["kind"],
            inner["type_id"],
            inner["variant"]
            if value["kind"] == "variant"
            else inner["fields"][0]["name"],
        )

    assert {typed_key(item) for item in typed_errors} == {
        ("variant", "echo_core::plugin::capability::PluginCapability", "LspServer"),
        (
            "record",
            "echo_agent::plugin::prepared::PluginWiringResult",
            "lsp_discovered",
        ),
        (
            "record",
            "echo_core::plugin::registry::ResolvedComponents",
            "lsp_config_file",
        ),
    }
    assert {typed_key(item) for item in typed_positives} == {
        ("variant", "echo_core::plugin::capability::PluginCapability", "McpServer"),
        ("record", "echo_agent::plugin::prepared::PluginWiringResult", "diagnostics"),
        ("record", "echo_core::plugin::registry::ResolvedComponents", "diagnostics"),
    }
    assert len(positives) == 4
    observation = {
        "retired_family_method_not_found": family_errors,
        "retired_source_contract_refusal": source_errors,
        "retired_typed_argument_refusal": typed_errors,
        "retained_typed_prevalidation_only": typed_positives,
        "retained_executed_operations": positives,
        "standard_hello": hello,
        "host_process": process,
        "host_used": used,
        "scope": "合法typed值仅证明参数校验通过；四个正向RPC实际执行普通插件/source/family，不把未知operation的拒绝称执行成功。",
    }
    (work / "verified-observed.json").write_text(
        json.dumps(observation, ensure_ascii=False, indent=2), encoding="utf-8"
    )
    return {
        "family": 39,
        "source": 19,
        "typed_negative": 3,
        "typed_prevalidation_positive": 3,
        "executed_positive": 4,
    }


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--work", type=Path, required=True)
    parser.add_argument("--verify", action="store_true")
    args = parser.parse_args()
    print(
        json.dumps(
            verify(args.work)
            if args.verify
            else {"root": str(prepare(args.work).resolve())}
        )
    )
