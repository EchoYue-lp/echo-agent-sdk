# DRAFT. Intended final path sdks/python/tests/test_workflow_host.py.
import asyncio
import json
import os
import shutil
import sys
import tempfile
from pathlib import Path

import pytest
from echo_agent_sdk import EchoAgentClient, EchoAgentError

HOST=os.environ.get("ECHO_AGENT_SDK_HOST")
if os.environ.get("ECHO_AGENT_SDK_REQUIRE_HOST_E2E")=="1" and not HOST:
    raise RuntimeError("required Workflow Host missing")
FIXTURE=Path("/tmp/sdk-workflow-candidate-fixture/workflow_fixture.py")

async def helper(work, *arguments):
    process=await asyncio.create_subprocess_exec(sys.executable,str(FIXTURE),"--work",str(work),*arguments,stdout=asyncio.subprocess.PIPE,stderr=asyncio.subprocess.PIPE)
    output,diagnostics=await process.communicate()
    assert process.returncode==0,diagnostics.decode()
    return output

@pytest.mark.asyncio
@pytest.mark.skipif(not HOST,reason="real Host binary required")
async def test_workflow_checkpoint_branch_owner_and_shape_boundary():
    work=Path(tempfile.mkdtemp(prefix="sdk-py-workflow-"));sdk=None;passed=False
    try:
        await helper(work,"--prepare")
        scenario=json.loads((work/"scenario.json").read_text())
        sdk=await EchoAgentClient.spawn(HOST,"--config",str(work/"host.json"))
        agent=await sdk.create_agent()
        sessions={"a":await agent.create_session(),"b":await agent.create_session()}
        rows=[];results={}
        for step in scenario["steps"]:
            session=sessions[step["session"]]
            if step.get("action")=="close":
                await session.close();row={"id":step["id"],"outcome":"closed"};rows.append(row);results[step["id"]]=row;(work/"results.json").write_text(json.dumps(rows,ensure_ascii=False));continue
            resolution=work/"resolve.json";resolution.write_text(json.dumps({"value":step["arguments"],"results":results},ensure_ascii=False))
            arguments=json.loads(await helper(work,"--resolve",str(resolution)));(work/f"call-{step['id']}.json").write_text(json.dumps({"operation":step["operation"],"arguments":arguments},ensure_ascii=False))
            try:
                row={"id":step["id"],"outcome":"result","value":await sdk.call(step["operation"],session.wire,arguments)}
            except EchoAgentError as error:
                row={"id":step["id"],"outcome":"error","code":error.code,"message":str(error),"operation":error.operation,"retryable":error.retryable,"details":error.details}
            rows.append(row);results[step["id"]]=row;(work/"results.json").write_text(json.dumps(rows,ensure_ascii=False))
        await sessions["b"].close();await agent.close();await sdk.close();sdk=None
        path=work/"results.json";path.write_text(json.dumps(rows,ensure_ascii=False))
        print("Workflow evidence: "+(await helper(work,"--verify",str(path))).decode());passed=True
    finally:
        if sdk is not None:await sdk.close()
        if passed and not os.environ.get("ECHO_AGENT_SDK_RETAIN_EVIDENCE"):shutil.rmtree(work)
        else:print(f"Workflow evidence retained: {work}",file=sys.stderr)
