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

const requireWindows = process.env.ECHO_AGENT_SDK_REQUIRE_WINDOWS_NATIVE === "1";
if (requireWindows && (process.platform !== "win32" || !host || !process.env.ECHO_AGENT_SDK_CONFIG)) {
  throw new Error("required Windows native Host/config/platform missing");
}

test("Windows native PluginRegistry paths preserve UTF-16 units through a real consumer", { skip: process.platform !== "win32" || !host, timeout: 30000 }, async () => {
  const work = await mkdtemp(join(process.env.RUNNER_TEMP ?? tmpdir(), "sdk-ts-windows-path-"));
  // Manual code units preserve an emoji pair and two isolated surrogates.
  const units = [0x0043, 0x003a, 0x005c, 0x4e2d, 0x6587, 0xd83d, 0xde42, 0x002d, 0xd800, 0x0041, 0xdc00];
  const encode = values => {
    const bytes = Buffer.alloc(values.length * 2);
    values.forEach((unit, index) => bytes.writeUInt16LE(unit, index * 2));
    return bytes.toString("base64").replace(/=+$/u, "");
  };
  const path = { kind: "path", value: { encoding: "windows", utf16_base64: encode(units), display: "Z:/display-诱饵" } };
  const childA = [...units, ...Array.from("\\child-A", value => value.charCodeAt(0))];
  const childB = [...childA, ...Array.from("\\child-B", value => value.charCodeAt(0))];
  let sdk;
  try {
    const config = JSON.parse(await readFile(process.env.ECHO_AGENT_SDK_CONFIG, "utf8"));
    config.sdk_profile.state_root = join(work, "state");
    if (config.default_agent.agent.memory_path !== undefined) config.default_agent.agent.memory_path = join(work, "memory.jsonl");
    const configPath = join(work, "host.json");
    await writeFile(configPath, JSON.stringify(config));
    sdk = await EchoAgentClient.spawn({ hostCommand: host, args: ["--config", configPath] });
    const agent = await sdk.createAgent();
    const session = await agent.createSession(work);
    const operation = "echo_core::plugin::registry::PluginRegistry::with_paths";
    const dataDir = "echo_core::plugin::registry::PluginRegistry::data_dir_for";
    const registry = await session.invoke(operation, [path, path, null]);
    const first = await session.invoke(dataDir, [registry, "child-A"]);
    assert.equal(first.kind, "path");
    assert.equal(first.value.encoding, "windows");
    assert.equal(first.value.utf16_base64, encode(childA));
    assert.notEqual(first.value.display, path.value.display);
    // Reuse the complete returned WireValue through the public encoder.
    const secondRegistry = await session.invoke(operation, [path, first, null]);
    const second = await session.invoke(dataDir, [secondRegistry, "child-B"]);
    assert.equal(second.kind, "path");
    assert.equal(second.value.encoding, "windows");
    assert.equal(second.value.utf16_base64, encode(childB));
    await writeFile(join(work, "windows-path-proof.json"), JSON.stringify({ platform: process.platform, units, first, second, nativeCalls: 4 }));
    console.log("Windows native path evidence", JSON.stringify({platform: process.platform, units, first, second, nativeCalls: 4}));
    console.log("Windows native path consumer: 1 passed, 0 failed, 0 skipped; 4 native calls");
    await session.close();
    await agent.close();
  } finally {
    await sdk?.close();
    // No invalid-Unicode directory was created by either registry operation.
    await rm(work, { recursive: true, force: true });
  }
});
