import { fileSha256 } from "../../shared/testing/file-sha256.mjs";
import { mkdirSync, mkdtempSync, readFileSync, writeFileSync, createWriteStream } from 'node:fs';
import { join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { execFileSync, spawn } from 'node:child_process';
import { once } from 'node:events';
import test from 'node:test';
import assert from 'node:assert/strict';
import { EchoAgentClient } from '../dist/client.js';

const fixture = fileURLToPath(new URL('../../shared/testing/rag_project_rules_fixture.py', import.meta.url));
const root = process.env.ECHO_AGENT_SDK_RAG_RULES_ROOT ?? fileURLToPath(new URL('../../../target/rag-project-rules', import.meta.url));
const host = process.env.ECHO_AGENT_SDK_HOST?.trim();
if (process.env.ECHO_AGENT_SDK_REQUIRE_HOST_E2E === '1' && !host) throw new Error('required RAG/project-rules Host missing');
function helper(work, ...args) { return execFileSync('python3', [fixture, '--work', work, ...args], { encoding: 'utf8' }); }
function portable(value) {
  if (value && typeof value === 'object' && typeof value.id === 'string' && value.generation !== undefined && typeof value.kind === 'string') return { kind: 'handle', value };
  if (Array.isArray(value)) return value.map(portable);
  if (value && typeof value === 'object') return Object.fromEntries(Object.entries(value).map(([key, item]) => [key, portable(item)]));
  return value;
}
const encode = value => JSON.stringify(value, (_, v) => typeof v === 'bigint' ? v.toString() : v, 2);

test('RAG and project rules retain original Session authority, failures and files', { skip: !host, timeout: 120000 }, async () => {
  mkdirSync(root, { recursive: true });
  const work = mkdtempSync(join(root, 'typescript-'));
  helper(work);
  const diagnostics = createWriteStream(join(work, 'server.stderr'));
  const server = spawn('python3', [fixture, '--work', work, '--serve'], { stdio: ['ignore', 'ignore', 'pipe'] });
  server.stderr.pipe(diagnostics);
  const rows = [], values = new Map();
  let sdk;
  try {
    let endpoint;
    for (let count = 0; count < 200; count++) {
      try { endpoint = JSON.parse(readFileSync(join(work, 'endpoint.json'), 'utf8')).url; break; } catch { await new Promise(resolve => setTimeout(resolve, 10)); }
    }
    assert.ok(endpoint, 'embedding server readiness');
    const env = { EMBEDDING_BASEURL: endpoint, EMBEDDING_MODEL: 'fixture-中文-embedding', EMBEDDING_APIKEY: 'fixture-key-not-secret' };
    sdk = await EchoAgentClient.spawn({ hostCommand: host, args: ['--config', join(work, 'host.json')], env });
    const agent = await sdk.createAgent();
    const sessions = Object.fromEntries(await Promise.all(Object.entries({ a: 'repo/nested/leaf', b: 'other', empty: 'empty', new: 'repo/nested/leaf' }).map(async ([name, cwd]) => [name, await agent.createSession(join(work, cwd))])));
    const resolve = (value, session) => value === '$session' ? session.wire : typeof value === 'string' && value.startsWith('$') && !value.startsWith('$WORK') ? values.get(value.slice(1)) : typeof value === 'string' ? value.replaceAll('$WORK', work) : value && typeof value === 'object' ? Object.fromEntries(Object.entries(value).map(([key, item]) => [key, resolve(item, session)])) : value;
    for (const step of JSON.parse(readFileSync(join(work, 'scenario.json'), 'utf8')).steps) {
      if (step.action) {
        if (step.action === 'close') await sessions[step.session].close();
        else { const path = join(work, 'action.json'); writeFileSync(path, encode(step)); helper(work, '--action', path); }
        continue;
      }
      const session = sessions[step.session], args = step.arguments.map(value => resolve(value, session));
      const row = { id: step.id, operation: step.operation, arguments: portable(args) };
      try {
        row.result = portable(await sdk.call(step.operation, step.operation.startsWith('echo_core::') ? agent.wire : session.wire, args));
        if (step.id.startsWith('chunk-')) {
          const preview = JSON.parse(row.result.output), characters = [...args[0]];
          assert.equal(preview.total_chunks, 1);
          assert.equal(preview.chunks[0].char_count, characters.length);
          assert.equal(preview.chunks[0].preview, characters.slice(0, 200).join(''));
          assert.equal(preview.chunks[0].truncated, characters.length > 200);
        }
        if (step.save) values.set(step.save, row.result);
      } catch (error) { row.error = { code: error.code, message: error.message, operation: error.operation, retryable: error.retryable, details: error.details ?? null, handle: portable(error.handle ?? null) }; }
      rows.push(row); writeFileSync(join(work, 'rows.json'), encode(rows));
    }
    for (const name of ['b', 'empty', 'new']) await sessions[name].close();
    await agent.close(); await sdk.close(); sdk = undefined;
    const bad = await EchoAgentClient.spawn({ hostCommand: host, args: ['--config', join(work, 'host.json')], env: { ...env, EMBEDDING_MODEL: '' } });
    try {
      const badAgent = await bad.createAgent(), session = await badAgent.createSession(join(work, 'empty'));
      await assert.rejects(bad.call('rag_search', session.wire, ['query', 1]), error => { rows.push({ id: 'invalid-config', operation: 'rag_search', error: { code: error.code, message: error.message, operation: error.operation, retryable: error.retryable, details: error.details ?? null, handle: portable(error.handle ?? null) } }); return true; });
      await session.close(); await badAgent.close();
    } finally { await bad.close(); }
    writeFileSync(join(work, 'results.json'), encode({ language: 'typescript', host, host_sha256: fileSha256(host), work, rows, server_pid: server.pid }));
    console.log('RAG/project-rules verified:', helper(work, '--verify', join(work, 'results.json')).trim(), 'evidence:', work);
  } finally {
    if (sdk) await sdk.close();
    const exit = once(server, 'exit'); server.kill('SIGTERM'); await exit; diagnostics.end();
    console.log('RAG/project-rules retained:', work);
  }
});
