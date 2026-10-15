import ast
import asyncio
import contextlib
import io
import json
import subprocess
import unittest
from pathlib import Path
from unittest.mock import patch

CANDIDATE = Path(__file__).with_name("test-tool-value-effect-diagnostic-candidate.py")
ORIGINAL = Path("/Users/ls/MyWork/code/ylp_agent_learn/lp-agent/.worktrees/echo-agent-sdk-worktrees/sdk-semantic-governance/sdks/python/tests/test_tool_value_effect_host.py")


def extract(source, name, namespace):
    node = next(node for node in ast.walk(ast.parse(source.read_text())) if isinstance(node, ast.FunctionDef) and node.name == name)
    exec(compile(ast.Module(body=[node], type_ignores=[]), str(source), "exec"), namespace)
    return namespace[name]


class DiagnosticCandidates(unittest.IsolatedAsyncioTestCase):
    async def test_optional_capture_does_not_change_success_matcher(self):
        namespace = {"subprocess": subprocess, "asyncio": asyncio}
        current = extract(CANDIDATE, "processes", namespace)
        original = extract(ORIGINAL, "processes", {"subprocess": subprocess})
        work = Path("/owned/work")
        rows = "10 1 tail -f /owned/work/tail.txt\n11 1 unrelated --secret=must-not-be-logged\n"
        with patch.object(subprocess, "check_output", return_value=rows):
            expected = original(work)
            actual = current(work)
        self.assertEqual(actual, expected)
        self.assertEqual(actual, [{"pid":10,"ppid":1,"command":"tail -f /owned/work/tail.txt"}])

    async def test_diagnostic_argv_variants_are_not_accepted_as_pid_match(self):
        current = extract(CANDIDATE, "processes", {"subprocess": subprocess,"asyncio":asyncio})
        work = Path("/owned/work")
        diagnostics = {"host_pid":100,"started_at":asyncio.get_running_loop().time(),"snapshots":[],"seen_pid_sets":[]}
        default = "42 100 /usr/bin/tail -f /owned/work/tail.txt\n43 100 tail -f /owned/truncated\n99 1 other --secret=must-not-be-logged\n"
        wide = "42 100 /usr/bin/tail -f /owned/work/tail.txt\n43 100 tail -f /owned/work/tail.txt\n99 1 other --secret=must-not-be-logged\n"
        with patch.object(subprocess, "check_output", side_effect=["", default, wide, default]) as call:
            self.assertEqual(current(work, diagnostics), [])
            self.assertEqual(current(work, diagnostics), [])
            self.assertEqual(current(work, diagnostics), [])
        self.assertEqual(len(diagnostics["snapshots"]),2)
        self.assertEqual(len(diagnostics["snapshots"][1]["owned_default_ps"]),2)
        self.assertEqual(len(diagnostics["snapshots"][1]["owned_wide_ps"]),2)
        self.assertNotIn("must-not-be-logged",json.dumps(diagnostics))
        self.assertEqual(call.call_count,4)

    async def test_full_json_marker_preserves_primary_even_when_ps_diagnostics_fail(self):
        work=Path("/owned/work")
        failure=extract(CANDIDATE,"observation_failure",{"subprocess":subprocess,"asyncio":asyncio,"json":json,"work":work})
        result={"kind":{"kind":"command_output","exit_code":"-1"},"failure":{"category":"timeout"},"output":"actual test input 中文🙂"}
        task=asyncio.create_task(asyncio.sleep(0,result=result))
        await task
        stream=io.StringIO()
        diagnostics={"host_pid":100,"snapshots":[]}
        with patch.object(subprocess,"check_output",side_effect=OSError("auxiliary ps unavailable")),contextlib.redirect_stdout(stream):
            fact=failure("original PID count failure",[],task,asyncio.get_running_loop().time(),diagnostics)
        marker=stream.getvalue().strip()
        self.assertTrue(marker.startswith("TOOL_PID_OBSERVATION_FAILURE_JSON "))
        actual=json.loads(marker.split(" ",1)[1])
        self.assertEqual(actual,fact)
        self.assertEqual(actual["settled_invocation_result"],result)
        self.assertEqual(actual["reason"],"original PID count failure")
        self.assertEqual(actual["owned_wide_ps_diagnostic_error"]["type"],"OSError")
        self.assertEqual(actual["owned_default_ps_diagnostic_error"]["type"],"OSError")


if __name__ == "__main__":
    unittest.main()
