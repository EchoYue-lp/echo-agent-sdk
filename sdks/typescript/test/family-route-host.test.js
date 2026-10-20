import { mkdtemp, readFile, writeFile, rm } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import test from "node:test";
import assert from "node:assert/strict";
import { EchoAgentClient } from "../dist/index.js";

const host = process.env.ECHO_AGENT_SDK_HOST?.trim();
if (process.env.ECHO_AGENT_SDK_REQUIRE_HOST_E2E === "1" && !host) {
  throw new Error("required family-route Host missing");
}

test("native family calls use the catalog's hyphenated wire method", { skip: !host }, async () => {
  const work = await mkdtemp(join(tmpdir(), "sdk-ts-family-route-"));
  let sdk;
  try {
    const config = JSON.parse(await readFile(process.env.ECHO_AGENT_SDK_CONFIG, "utf8"));
    // Parallel smoke tests must not share the native file-authority lease.
    config.sdk_profile.state_root = join(work, "state");
    if (config.default_agent.agent.memory_path !== undefined) config.default_agent.agent.memory_path = join(work, "memory.jsonl");
    const configPath = join(work, "host.json");
    await writeFile(configPath, JSON.stringify(config));
    sdk = await EchoAgentClient.spawn({ hostCommand: host, args: ["--config", configPath] });
    const agent = await sdk.createAgent();
    const session = await agent.createSession();
    const text = "中😀 alice@example.com 尾";
    assert.equal(await sdk.call("content-guard.redact_exact", session.wire, [text]), "中😀 [REDACTED:EMAIL] 尾");
    assert.equal(await sdk.family("content_guard", "content-guard.redact_exact", session.wire, [text]), "中😀 [REDACTED:EMAIL] 尾");
    await assert.rejects(sdk.family("memory", "content-guard.redact_exact", session.wire, [text]), /does not belong/);
    await session.close();
    await agent.close();
  } finally {
    await sdk?.close();
    await rm(work, { recursive: true, force: true });
  }
});
