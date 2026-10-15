import json


def fact():
    return {"reason":"owned tail count is not exactly one", "pids":[], "observation_watchdog_seconds":10, "observation_elapsed_seconds":5.028703057, "task_done":True, "task_cancelled":False, "settled_invocation_result":{"kind":{"kind":"command_output","exit_code":"-1"},"failure":{"category":"timeout"},"output":"only synthetic diagnostic serialization input 中文🙂"}, "owned_default_ps":[], "owned_wide_ps":[]}


def test_original_dictionary_assertion_is_abbreviated():
    value=fact()
    assert len(value["pids"])==1,value


def test_explicit_json_marker_is_retained():
    value=fact()
    print("TOOL_PID_OBSERVATION_FAILURE_JSON "+json.dumps(value, ensure_ascii=False),flush=True)
    assert len(value["pids"])==1,json.dumps(value,ensure_ascii=False)
