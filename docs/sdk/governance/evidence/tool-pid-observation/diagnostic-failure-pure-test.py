import ast
import asyncio
import json
import subprocess
from pathlib import Path
from unittest.mock import patch

SOURCE = Path("/Users/ls/MyWork/code/ylp_agent_learn/lp-agent/.worktrees/echo-agent-sdk-worktrees/sdk-semantic-governance/sdks/python/tests/test_tool_value_effect_host.py")
parsed = ast.parse(SOURCE.read_text())
function = next(node for node in ast.walk(parsed) if isinstance(node, ast.FunctionDef) and node.name == "observation_failure")
namespace = {"asyncio": asyncio, "subprocess": subprocess, "work": Path("/owned-only/no-child")}
exec(compile(ast.Module(body=[function], type_ignores=[]), str(SOURCE), "exec"), namespace)
observation_failure = namespace["observation_failure"]


async def main():
    rows = []
    for error in [FileNotFoundError("diagnostic ps unavailable"), subprocess.CalledProcessError(3, ["ps"])] :
        task = asyncio.create_task(asyncio.sleep(0, result={"success": False, "kind": {"kind": "command_output", "exit_code": "9"}}))
        await task
        with patch.object(subprocess, "check_output", side_effect=error):
            fact = observation_failure("original noPID assertion", [], task, asyncio.get_running_loop().time())
        assert fact["reason"] == "original noPID assertion"
        assert fact["pids"] == [] and fact["task_done"]
        assert fact["settled_invocation_result"] == task.result()
        assert fact["owned_default_ps_diagnostic_error"]["type"] == type(error).__name__
        assert fact["owned_wide_ps_diagnostic_error"]["type"] == type(error).__name__
        rows.append(fact)
    async def fail():
        raise RuntimeError("original invocation error")
    task = asyncio.create_task(fail())
    await asyncio.gather(task, return_exceptions=True)
    with patch.object(subprocess, "check_output", side_effect=OSError("diagnostic failure")):
        fact = observation_failure("invocation ended without owned PID", [], task, asyncio.get_running_loop().time())
    assert fact["settled_invocation_error"] == {"type": "RuntimeError", "message": "original invocation error"}
    assert fact["reason"] == "invocation ended without owned PID"
    rows.append(fact)
    print(json.dumps({"tests": 3, "passed": 3, "sourceRole": "directly compiled actual nested diagnostic function, no mirrored implementation; noHost", "facts": rows}))


asyncio.run(main())
