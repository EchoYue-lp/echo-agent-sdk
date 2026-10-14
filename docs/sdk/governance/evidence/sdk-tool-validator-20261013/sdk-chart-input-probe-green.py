import asyncio
import json
import os
import tempfile
from pathlib import Path

from echo_agent_sdk import EchoAgentClient, EchoAgentError

ROOT = Path('/Users/ls/MyWork/code/ylp_agent_learn/echo-agent-sdk-worktrees/sdk-semantic-governance')

async def main():
    work = Path(tempfile.mkdtemp(prefix='sdk-chart-input-probe-'))
    config = json.loads((ROOT / 'echo-sdk-host/config.sdk.example.json').read_text())
    config['sdk_profile']['state_root'] = str(work / 'state')
    path = work / 'host.json'
    path.write_text(json.dumps(config))
    sdk = await EchoAgentClient.spawn('/tmp/echo-sdk-governance-tool-validation-host', '--config', str(path), env=dict(os.environ))
    results = []
    try:
        agent = await sdk.create_agent()
        session = await agent.create_session(str(work))
        for chart_type in ['bar', 'not_a_chart_type']:
            args = [chart_type, '中文🙂', 'label', 'value', None, [{'label': 'a', 'value': 1}], 321, 123, 'json']
            try:
                value = await sdk.call('generate_chart', session.wire, args)
                results.append({'chart_type': chart_type, 'result': value})
            except EchoAgentError as error:
                results.append({'chart_type': chart_type, 'error': {'code': error.code, 'message': str(error), 'operation': error.operation}})
        await session.close()
        await agent.close()
    finally:
        await sdk.close()
    (work / 'results.json').write_text(json.dumps(results, ensure_ascii=False, indent=2))
    print(json.dumps({'work': str(work), 'results': results}, ensure_ascii=False))

asyncio.run(main())
