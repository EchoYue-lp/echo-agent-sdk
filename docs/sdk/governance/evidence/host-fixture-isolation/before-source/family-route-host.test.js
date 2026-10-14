import test from "node:test";
import assert from "node:assert/strict";
import { EchoAgentClient } from "../dist/index.js";

const host = process.env.ECHO_AGENT_SDK_HOST?.trim();
if (process.env.ECHO_AGENT_SDK_REQUIRE_HOST_E2E === "1" && !host) {
  throw new Error("required family-route Host missing");
}

test("native family calls use the catalog's hyphenated wire method", { skip: !host }, async () => {
  const sdk = await EchoAgentClient.spawn({ hostCommand: host, args: ["--config", process.env.ECHO_AGENT_SDK_CONFIG] });
  try {
    const agent = await sdk.createAgent();
    const session = await agent.createSession();
    const text = "中😀 alice@example.com 尾";
    assert.equal(await sdk.call("content-guard.redact_exact", session.wire, [text]), "中😀 [REDACTED:EMAIL] 尾");
    assert.equal(await sdk.family("content_guard", "content-guard.redact_exact", session.wire, [text]), "中😀 [REDACTED:EMAIL] 尾");
    await assert.rejects(sdk.family("memory", "content-guard.redact_exact", session.wire, [text]), /does not belong/);
    await session.close();
    await agent.close();
  } finally {
    await sdk.close();
  }
});
