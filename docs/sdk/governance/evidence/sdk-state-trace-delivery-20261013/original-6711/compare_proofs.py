from pathlib import Path
import datetime
import hashlib
import json

ROOT = Path('/tmp/sdk-state-trace-delivery-candidate-fixture')
LANGUAGES = ['python', 'typescript', 'java']


def load(language, scene):
    folder = ROOT / (language + '-' + scene if scene != 'delivery' else language + '-delivery-exact-cause')
    return folder, json.loads((folder / (scene + '-observed.json')).read_text())


def normalized(value, key=None):
    if key == 'working_dir':
        return '$SCENE_WORK'
    if isinstance(value, bool) or value is None:
        return value
    if isinstance(value, int):
        return str(value)
    if isinstance(value, list):
        return [normalized(item) for item in value]
    if isinstance(value, dict):
        return {field: normalized(item, field) for field, item in value.items()}
    return value


comparisons = {}
side_effects = {}
for scene in ['state', 'trace']:
    projected = {}
    for language in LANGUAGES:
        folder, proof = load(language, scene)
        projected[language] = normalized(proof['after_real_host_restart'])
        assert (folder / 'state/host/generation').read_text().strip() == '2'
        assert len(proof['errors']) == 2
        assert all(error['code'] == 'framework_error' for error in proof['errors'])
        if scene == 'state':
            owners = [json.loads(path.read_text()) for path in (folder / 'state/runtime_state/_runtime_owners').glob('*.json')]
            assert [record['runtime_state_id'] for record in owners] == ['runtime-B-🙂']
            assert normalized(owners[0]['checkpoint']) == projected[language]
            side_effects[language + '-state'] = {'generation': 2, 'remaining_runtime_owners': 1,
                                               'scope_b_preserved': True, 'scope_a_absent': True}
        else:
            files = list((folder / 'traces').glob('*.jsonl'))
            assert len(files) == 1
            raw = files[0].read_bytes()
            assert b'sk-abcdefghijklmnopqrstuvwxyz123456' not in raw
            assert normalized(json.loads(raw.decode().splitlines()[0])) == projected[language]
            side_effects[language + '-trace'] = {'generation': 2, 'durable_file_sha256': hashlib.sha256(raw).hexdigest(),
                                               'sanitized_payload_matches_restarted_host': True}
    assert projected['python'] == projected['typescript'] == projected['java'], scene
    comparisons[scene] = {'full_dto_equal': True, 'only_fixture_specific_working_dir_normalized': scene == 'state',
                          'canonical_dto_sha256': hashlib.sha256(json.dumps(projected['python'], sort_keys=True, ensure_ascii=False).encode()).hexdigest()}

# IDs and claim timestamps legitimately vary per isolated ledger; all retained
# original bytes remain inspectable, and only their identity roles normalize.
normalized_delivery = {}
for language in LANGUAGES:
    folder, proof = load(language, 'delivery')
    claims = proof['concurrent_claims']
    assert [claim['attempt'] for claim in claims] == ['1', '2']
    assert len({claim['attempt_id'] for claim in claims}) == 2
    for claim in claims:
        assert set(claim) == {'payload', 'message_id', 'route', 'attempt_id', 'attempt', 'claimed_at'}
        datetime.datetime.fromisoformat(claim['claimed_at'].replace('Z', '+00:00'))
    current = proof['claim2']
    assert current['attempt'] == '3'
    assert current['payload'] == claims[0]['payload'] == claims[1]['payload']
    assert len(proof['errors']) == 4
    assert all(error['code'] == 'framework_error' and 'stale delivery claim' in error['message'] for error in proof['errors'])
    row = proof['snapshot']['records'][0]
    assert row['attempt_id'] == current['attempt_id'] and row['outcome'] == 'completed'
    assert proof['recovery']['last_applied_sequence'] == '8'
    assert row['claimed_at'] == current['claimed_at']
    timestamps = [datetime.datetime.fromisoformat(row[name].replace('Z', '+00:00'))
                  for name in ['persisted_at', 'claimed_at', 'effect_started_at', 'turn_settled_at']]
    assert timestamps == sorted(timestamps)
    matched = {key: value for key, value in row.items() if key not in {'persisted_at', 'attempt_id', 'claimed_at', 'effect_started_at',
                                                                     'mailbox_accepted_at', 'drained_at', 'turn_settled_at'}}
    normalized_delivery[language] = normalized(matched)
    side_effects[language + '-delivery'] = {'durability': 'memory same-instance only', 'pre_effect_supersession_attempts': [1, 2],
                                           'sole_effect_admitted_from_latest': True, 'post_effect_claim_returns_null': True,
                                           'defer_reclaim_attempt': 3, 'stale_commands_no_state_or_sequence_mutation': 4,
                                           'same_instance_recovered_sequence': 8}
assert normalized_delivery['python'] == normalized_delivery['typescript'] == normalized_delivery['java']
comparisons['delivery'] = {'canonical_record_equal': True,
                           'omitted_comparison_fields': ['per-ledger attempt_id and actual persisted/claim/effect/settle timestamps (individually validated and ordered)'],
                           'source_ownership_and_all_fields_checked_before_normalization': True}
report = {'host': '/tmp/echo-sdk-governance-parent-driver-host',
          'host_sha256': hashlib.sha256(Path('/tmp/echo-sdk-governance-parent-driver-host').read_bytes()).hexdigest(),
          'actual_languages': LANGUAGES, 'actual_scene_executions': 9, 'comparisons': comparisons,
          'durable_side_effects': side_effects,
          'limits': ['Temporary source fixture/public SDK consumers; not standalone published pin/main.',
                    'Delivery is in-memory and supersedes unstarted claims; uniqueness refers to current exact authority.',
                    'Native integer text must be explicitly wire-typed on repeat request; no new permissive Host parsing.',
                    'No universal all-or-none scope deletion promise; malformed authority error is not a missing record.',
                    'Three Java runs print existing SLF4J no-provider notices; compile is zero-warning.',
                    'Raw stdio frames are not captured; complete public outputs, durable files, input corpora and exact consumer assertion source are retained.']}
(ROOT / 'three-language-comparison.json').write_text(json.dumps(report, ensure_ascii=False, indent=2) + '\n')
print(json.dumps({'actual_scene_executions': 9, 'full_state_trace_dto_equal': True,
                  'delivery_current_authority_equal': True, 'host_sha256': report['host_sha256']}, indent=2))
