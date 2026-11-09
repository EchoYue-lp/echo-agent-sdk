#!/usr/bin/env python3
"""精确校验原Plan04退出与已批准340组合，不从Rust库存自动扩大接受范围。"""
import argparse
import hashlib
import json
import subprocess
from pathlib import Path

# Published rollback tag; its accepted contracts equal Plan04's local S baseline.
# CI fetches this exact protected tag before running the otherwise shallow job.
BASELINE = "0930eef9f878cf436ed88384030b6bc03d6f6c53"
EXTRAS = {
    ("echo_core::plugin::capability::PluginCapability::LspServer", "variant"),
    ("echo_agent::plugin::prepared::PluginWiringResult::lsp_discovered", "struct_field"),
    ("echo_core::plugin::registry::ResolvedComponents::lsp_config_file", "struct_field"),
}

# 340组合与ADR0046精确来源迁移均绑定真实生成行；其余记录仍须与原rollback基线精等。
FRAMEWORK_REVISION = '3413d64dae23353c6d02c546ac374deded855827'
COMBINATION_RETIRED = frozenset([('echo_agent::state::AgentCheckpoint::current_plan', 'struct_field'),
 ('echo_agent::state::delete_persisted_conversation', 'function'),
 ('echo_core::agent::AgentCallback::on_tool_end', 'method'),
 ('echo_core::agent::AgentCallback::on_tool_error', 'method'),
 ('echo_core::agent::AgentCallback::on_tool_start', 'method'),
 ('echo_core::agent::AgentEvent::ThinkEnd', 'variant'),
 ('echo_core::agent::AgentEvent::ThinkEnd::completion_tokens', 'struct_field'),
 ('echo_core::agent::AgentEvent::ThinkEnd::prompt_tokens', 'struct_field'),
 ('echo_core::agent::AgentEvent::ThinkStart', 'variant'),
 ('echo_core::agent::AgentEvent::Token', 'variant'),
 ('echo_core::agent::AgentEvent::Token::0', 'struct_field'),
 ('echo_core::llm::cache::layout::PromptCacheLayout::runtime_context', 'struct_field'),
 ('echo_core::llm::cache::layout::SegmentRanges::runtime_context', 'struct_field')])
COMBINATION_ACCEPTED_ROW_SHA = {('echo_core::agent::AgentCallback::on_tool_end_with_id', 'method'): '9c640da2af5b9466d16b6a8d27f0170bf2a436fa0fa814b3edc4d959e74a2a5e',
 ('echo_core::agent::AgentCallback::on_tool_error_with_id', 'method'): 'e3369a36a4dc578a58dd18a48e942ee8c96097e7f99189cfc2130dae69b3df61',
 ('echo_core::agent::AgentCallback::on_tool_start_with_id', 'method'): '0ecf0351b1582565c7f9cb58aea25260860605d143050badce0089bea4cac1b1',
 ('echo_core::llm::cache::layout::PromptCacheLayout::dynamic', 'struct_field'): '1f89b340f439be85c0251d7537909be7c67822b44ad11017e228b9ac04b38f7c',
 ('echo_core::llm::cache::layout::SegmentRanges::dynamic', 'struct_field'): '18b7c646e8fc3e97a5e9ef2dfcaecb36c454a1223e24a52fc87fb55d278d549c'}
COMBINATION_RETAINED_ROW_SHA_PAIRS = {('echo_agent::agent::react::ReactAgent::set_store', 'method'): ('fdb1b1e34e3358a2cfd63578f41dfe56c2425a7415bc7bc8e92f57887a57934d',
                                                                 '6b6d0f1078fc14dbdbbf8178b108b56d341cfc59e60c60bfa7ab520800cee8c5'),
 ('echo_agent::testing::mock_llm::StreamChunk::Delta::0', 'struct_field'): ('e8fa927445ba6eeefe237f7bd50ed52fcb58982012065572d15e73e18788b209',
                                                                            '718cf50b9a5ef2994e1ea19ed9f749245a288d0d510aa22f26d819f752165c02'),
 ('echo_core::agent::Agent::execute', 'method'): ('3c676cb509f6cb678f6674ab6d0423d83710960c91b323667ebb16aaf2671121',
                                                  'cf4061b2246fb308dd16f7bfec2f95290e331da0cc276429eace0f6a7e4b15bb'),
 ('echo_core::agent::Agent::execute_stream', 'method'): ('0d0e7cc943f12ae1e8dd49f91acb0de075d3bce7638e6f3dbd358246a23d86b0',
                                                         'cd720c225a8c15419c32ba28979d17c09ea0cd9b016174c67639f041e2ba510f'),
 ('echo_core::llm::ChatRequest::tool_choice', 'struct_field'): ('2bb2b80801db6779c19e65b84b1a2bd57b807190d20430f647589eae0b5e5bbc',
                                                                '1b7ed1d07a00f39766c874d9b7ceb5cc4854bda5f19f2fc1615031214578eacd'),
 ('echo_core::llm::types::ChatCompletionRequest::tool_choice', 'struct_field'): ('546bb4c19006847acf2e0c3073638ea3f2b726ea30d0f122f2a5a83e215841d5',
                                                                                 '13590a6b950b58d82adfe91e211eeec737c1237fb9a6ae6411e5aaa51e91ca0d'),
 ('echo_core::llm::types::Usage::output_tokens_details', 'struct_field'): ('8c34139aaa1d61a7a9314e986c4484fca71bff04895f9cf704c74821ff371000',
                                                                           '180f83aafc91ee2eb13e12c02915e488ab2c6852ae8b88aa6373a7978b008a8f'),
 ('echo_core::sandbox::SandboxCommand', 'struct'): ('1d3636140f8e29ce7e68ac871919bdc4eb68fb01d883ea884f1f8f6d82f1060f',
                                                    '72ed08f4a66ab88194d02961954dda620cc3f1992e3c1a0fdf5422ffa8fcaae3'),
 ('echo_core::tools::permission::PermissionMode::AcceptEdits', 'variant'): ('1471cfe10f785a5c84a454ef30599f45c41fd55e1640b6af6c0e013f8931ae95',
                                                                            '9504f372bfbc5e0ac6c2668fff0ff07674a64cb53e2ed5cf891f08c5df0b2dce'),
 ('echo_core::tools::permission::PermissionMode::BypassPermissions', 'variant'): ('ca6b8a7a2904836011cb041d8d80f73cd35dee4d8e391204ab4f59425436a3b2',
                                                                                  '8c43fee46fabed458435d90ecaccbb0def91ffbd91160e4c76595707c005c215'),
 ('echo_core::tools::permission::PermissionMode::DontAsk', 'variant'): ('03466353c940ffb971cfcf554755f63a2740d6b216441d9682549ac391ffa4f7',
                                                                        '98ee8d0eb88b83e9eb04280a2bfc499a40d0f47fbb0d26542d5a97b21ec09502'),
 ('echo_core::tools::permission::PermissionMode::StrictConfirm', 'variant'): ('d4b4ebf7359d3f086a2dce0418fe2b8b1f00806442739e92f360a8d87c1cb32f',
                                                                              '5b5b3a5b0bb9a256dc1506f223474b7191fadf370f7257a84df7eb48930e1b21'),
 ('echo_orchestration::runtime::turn_driver::TurnInput::Message::0', 'struct_field'): ('a9bb6b3e31635496b3ef47eebedb1735ec82449eff7901f5d62a1c5c4fc3044d',
                                                                                       '4cdc86b02d933646b381bed31eba08c44941729bea4ab4b56e3bd6c5c4bacc8d'),
 ('echo_core::compression::ContextProjection', 'struct'): ('221e3185b395fa7a97bc7d58d8d499ac721587bb7029d37c9e96f3d6d1e3970b',
                                                        '3331449588012d29a05e8a8a0c8b3d4adccd947ddad496e40af661342200638b')}
COMBINATION_RETAINED_ROUTE_SHA_PAIRS = {'bridge:agent_callback': ('8d5b70273ba0852b8214851cfd5e5fcd1d639aa34767b4439ddd903691d2360a',
                           '8aebbd620aac1cba4d69ee2297417844d8dc8832babceca760bb953b1562e6f8'),
 'source:echo_agent::agent::react::ReactAgent::set_store': ('27c9eac8593c00ab5ed3df111554fff7945cfe020b860a701da1f52ae26765a0',
                                                            '06154914e6a4c139f6e5de230e74c855a104b1651bdced3f650f12f683b5d185'),
 'source:echo_core::agent::Agent::execute': ('a2c7c5f7de73577ee4f51aa0401f9b32cf4f9fd406525deb1c9569be0adbe0fa',
                                             '49a646cf23a101dfe7e1c503f95eec70aa277c7955602976a137b4744fe7bbe1'),
 'source:echo_core::agent::Agent::execute_stream': ('b626a68a12fde9db001b32e4ad510cf794ce3e3d9db1b2fbb11f1d833f6db462',
                                                    '5fae28feeedc96be064dafdcd00308e68af2b1420aec6a76825f51f4e029ad12'),
 'standard:session/update': ('7a6c863668be2c11446bd9ea41509c2890226161eb45bbb95dd1aac99b56ced4',
                             '01210afb2786d60b2535bde9de341a6ef6668903c1bb14e1fa9b0199c90870ba'),
 'value': ('125c9df0acc8ebfedd6529dc69a0328aab2f2a53f6235eb3e76c7f1abb86f57f',
           'fde7e6f46c915538e13de227dfbb7f7f0fb76aa6429a04918329973541dec445'),
 'value:state': ('9ebfc48491f389bd175f74da05b6e3123192c1aa2fa73147db5f04232aee5445',
                 '6173417199ebb28ba299e997107de9b463d9c16d9a7d1f67cc7d301d87107f6a')}
COMBINATION_RETIRED_ROUTE = 'source:echo_agent::state::delete_persisted_conversation'

def canonical_sha(value):
    return hashlib.sha256(json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":")).encode()).hexdigest()


def source_key(entry):
    return (entry["source_paths"][0], entry["kind"])


def family_entry(entry):
    return any(
        feature in entry["features"]
        or entry["route"]["family"] == feature
        or any(f"::{feature}::" in name for name in [entry["path"], *entry["source_paths"]])
        for feature in ("a2a", "lsp")
    )


def check(root, baseline):
    def old(path):
        return json.loads(subprocess.check_output(["git", "show", f"{baseline}:{path}"], cwd=root))

    def new(path):
        return json.loads((root / path).read_text())

    previous = old("contracts/sdk/accepted-external-contract.json")["entries"]
    current = new("contracts/sdk/accepted-external-contract.json")["entries"]
    # 先核数组身份，避免dict覆盖重复source_key后掩盖输入。
    assert len(previous) == len({source_key(entry) for entry in previous}), "duplicate baseline accepted source_key"
    assert len(current) == len({source_key(entry) for entry in current}), "duplicate current accepted source_key"
    old_entries = {source_key(entry): entry for entry in previous}
    new_entries = {source_key(entry): entry for entry in current}
    original = {source_key(entry) for entry in previous if family_entry(entry)}
    assert len(original) == 266
    assert EXTRAS <= old_entries.keys() and not original & EXTRAS
    withdrawn = original | EXTRAS
    assert len(withdrawn) == 269
    assert len(COMBINATION_RETIRED) == 13
    assert COMBINATION_RETIRED <= old_entries.keys() and not withdrawn & COMBINATION_RETIRED
    assert old_entries.keys() - new_entries.keys() == withdrawn | COMBINATION_RETIRED
    assert new_entries.keys() - old_entries.keys() == COMBINATION_ACCEPTED_ROW_SHA.keys()
    assert len(previous) == 5621 and len(previous) - len(withdrawn) == 5352
    assert len(current) == 5344 and len(COMBINATION_ACCEPTED_ROW_SHA) == 5
    for key, digest in COMBINATION_ACCEPTED_ROW_SHA.items():
        assert canonical_sha(new_entries[key]) == digest, f"Accepted replacement row drift: {key}"
    # 仅承接原13行与ADR0046批准的1行派生元数据；完整行哈希继续拒绝其它漂移。
    changes = {key for key in new_entries.keys() & old_entries.keys()
               if new_entries[key] != old_entries[key]}
    assert changes == COMBINATION_RETAINED_ROW_SHA_PAIRS.keys()
    assert len(changes) == 14
    for key, pair in COMBINATION_RETAINED_ROW_SHA_PAIRS.items():
        assert (canonical_sha(old_entries[key]), canonical_sha(new_entries[key])) == pair, key
    assert all(new_entries[key] == old_entries[key]
               for key in (new_entries.keys() & old_entries.keys()) - changes)
    assert not any(family_entry(entry) for entry in current)
    before = old("contracts/sdk/accepted-facade-operation-catalog.json")
    after = new("contracts/sdk/accepted-facade-operation-catalog.json")
    # 在任何route集合/字典构造前核唯一性与header，不能让后真行覆盖先影子行。
    before_route_ids = [route["route"] for route in before["routes"]]
    after_route_ids = [route["route"] for route in after["routes"]]
    assert len(before_route_ids) == len(set(before_route_ids)), "duplicate baseline route ID"
    assert len(after_route_ids) == len(set(after_route_ids)), "duplicate current route ID"
    assert len(before["routes"]) == before["total_items"], "baseline route array/header length mismatch"
    assert len(after["routes"]) == after["total_items"], "current route array/header length mismatch"
    old_routes = set(before_route_ids)
    new_routes = set(after_route_ids)
    retired_routes = {route["route"] for route in before["routes"] if route["family"] in ("a2a", "lsp")}
    assert len(retired_routes) == 23
    assert COMBINATION_RETIRED_ROUTE in old_routes and COMBINATION_RETIRED_ROUTE not in retired_routes
    assert old_routes - new_routes == retired_routes | {COMBINATION_RETIRED_ROUTE}
    assert not new_routes - old_routes
    assert len(old_routes) == 454 and len(new_routes) == 430
    assert before["total_items"] == 454 and after["total_items"] == 430
    assert not any(family["family"] in ("a2a", "lsp") for family in after["families"])
    assert after["families"] == [family for family in before["families"]
                                  if family["family"] not in ("a2a", "lsp")]
    old_route_rows = {route["route"]: route for route in before["routes"]}
    new_route_rows = {route["route"]: route for route in after["routes"]}
    route_changes = {route for route in new_routes & old_routes
                     if old_route_rows[route] != new_route_rows[route]}
    assert route_changes == COMBINATION_RETAINED_ROUTE_SHA_PAIRS.keys()
    for route, pair in COMBINATION_RETAINED_ROUTE_SHA_PAIRS.items():
        assert (canonical_sha(old_route_rows[route]), canonical_sha(new_route_rows[route])) == pair, route
    assert all(old_route_rows[route] == new_route_rows[route]
               for route in (new_routes & old_routes) - route_changes)
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
    host_manifest = (root / "echo-sdk-host/Cargo.toml").read_text()
    assert f'rev = "{FRAMEWORK_REVISION}"' in host_manifest
    assert 'git = "https://github.com/EchoYue-lp/echo-agent.git"' in host_manifest
    print(json.dumps({"baseline": baseline, "a2a_lsp_retired_identities": 269,
                      "additional_retired_identities": 13, "retained_identities": 5344,
                      "a2a_lsp_retired_routes": 23, "additional_retired_routes": 1,
                      "retained_routes": 430, "retired_operations": 39,
                      "named_new_acceptances": 5, "exact_retained_entry_changes": 14,
                      "unexplained_retained_entry_changes": 0}, indent=2))


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--baseline", default=BASELINE)
    args = parser.parse_args()
    check(Path(__file__).resolve().parents[1], args.baseline)
