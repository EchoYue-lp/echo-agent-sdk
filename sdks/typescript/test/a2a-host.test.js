import test from "node:test";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { EchoAgentClient } from "../dist/index.js";
import { execFile } from "node:child_process";
import { promisify } from "node:util";
import { mkdtemp, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { fileSha256 } from "../../shared/testing/file-sha256.mjs";
const host = process.env.ECHO_AGENT_SDK_HOST;
const config = process.env.ECHO_AGENT_SDK_CONFIG;
if (process.env.ECHO_AGENT_SDK_REQUIRE_HOST_E2E === "1" && (!host || !config)) throw new Error("real Host/config required");
const retired = JSON.parse(readFileSync(new URL("../../shared/testing/a2a_lsp_retirement_contract.json", import.meta.url), "utf8"));

test("retired features and old source digest cannot negotiate an SDK connection", { skip: !host || !config, timeout: 30000 }, async () => {
  for (const feature of ["a2a", "lsp"]) {
    await assert.rejects(EchoAgentClient.spawn({hostCommand: host, args: ["--config", config], requiredFeatures: [feature]}), error => error.code === "feature_unavailable");
  }
  await assert.rejects(EchoAgentClient.spawn({hostCommand: host, args: ["--config", config], sourceContractDigest: retired.old_source_contract_digest}), error => error.code === "extension_capability_mismatch");
});

test("retirement refusal layers preserve real plugin and family operations", { skip: !host, timeout: 60000 }, async () => {
  const work = await mkdtemp(join(tmpdir(), "sdk-retired-typed-ts-"));
  const helper = fileURLToPath(new URL("../../shared/testing/retirement_plugin_fixture.py", import.meta.url));
  const proxy = fileURLToPath(new URL("../../python/tests/protocol_host_proxy.py", import.meta.url));
  const runHelper = async (...args) => promisify(execFile)(process.env.PYTHON ?? "python3", ["-B", helper, "--work", work, ...args], { timeout: 10000 });
  const plugin = JSON.parse((await runHelper()).stdout).root;
  const catalog = JSON.parse(readFileSync(new URL("../../shared/facade-operation-catalog.json", import.meta.url), "utf8"));
  let sdk;
  try {
    sdk = await EchoAgentClient.spawn({ hostCommand: process.env.PYTHON ?? "python3", args: [proxy, "--host", host, "--work", work, "--mode", "normal"] });
    const agent = await sdk.createAgent();
    const session = await agent.createSession();
    assert.equal(await session.invoke("echo_core::agent::Agent::name"), "echo-agent");
    const operation = "echo_core::plugin::registry::PluginRegistry::validate_plugin_dir";
    const route = catalog.routes.find((item) => item.operation === operation);
    assert.ok(route);
    await sdk.request("_echo_agent/facade/invoke", { operation, signature_digest: route.signature_digests[0], handle: agent.wire, arguments: [{kind:"handle",value:session.wire},{kind:"path",value:{encoding:"utf8",path:plugin}}] });
    assert.equal(await sdk.call("content-guard.redact_exact", session.wire, ["中😀 alice@example.com 尾"]), "中😀 [REDACTED:EMAIL] 尾");
    for (const family of retired.families) {
      const old = retired.routes.find((item) => item.route === `family:${family.family}`);
      for (const signature of old.operation_signatures) {
        await assert.rejects(sdk.request(family.methods[0], {operation:signature.operation,signature_digest:signature.signature_digests[0],handle:session.wire,arguments:[]}));
      }
    }
    for (const old of retired.routes.filter((item) => item.operation)) {
      await assert.rejects(sdk.request("_echo_agent/facade/invoke", {operation:old.operation,signature_digest:old.signature_digests[0],handle:agent.wire,arguments:[{kind:"handle",value:session.wire}]}));
    }
    for (const value of [
      {kind:"variant",value:{type_id:"echo_core::plugin::capability::PluginCapability",variant:"LspServer",fields:[]}},
      {kind:"record",value:{type_id:"echo_agent::plugin::prepared::PluginWiringResult",fields:[{name:"lsp_discovered",value:{kind:"null"}}]}},
      {kind:"record",value:{type_id:"echo_core::plugin::registry::ResolvedComponents",fields:[{name:"lsp_config_file",value:{kind:"null"}}]}},
      {kind:"variant",value:{type_id:"echo_core::plugin::capability::PluginCapability",variant:"McpServer",fields:[]}},
      ...["echo_agent::plugin::prepared::PluginWiringResult","echo_core::plugin::registry::ResolvedComponents"].map(type_id=>({kind:"record",value:{type_id,fields:[{name:"diagnostics",value:{kind:"list",value:[]}}]}})),
    ]) {
      await assert.rejects(sdk.request("_echo_agent/facade/invoke", {operation:"retirement.probe",signature_digest:"sha256:"+"0".repeat(64),handle:agent.wire,arguments:[value]}));
    }
    assert.equal(await session.invoke("echo_core::agent::Agent::name"), "echo-agent");
    await session.close();
    await agent.close();
  } finally {
    await sdk?.close();
    await writeFile(join(work, "host-used.json"), JSON.stringify({host,host_sha256:fileSha256(host),proxy_mode:"normal",qualification:"actual preserved Host; standard handshake; no Frame source read or Cargo"}));
    console.log("retirement raw frame evidence:", work);
  }
  console.log((await runHelper("--verify")).stdout);
});

test("all39 retired operations and19 source invokes fail against the real retained Host", { skip: !host || !config, timeout: 60000 }, async () => {
  const sdk = await EchoAgentClient.spawn({hostCommand: host, args: ["--config", config]});
  try {
    const agent = await sdk.createAgent();
    const session = await agent.createSession();
    let operations = 0;
    for (const family of retired.families) {
      const route = retired.routes.find(route => route.route === `family:${family.family}`);
      for (const operation of family.operations) {
        const signature = route.operation_signatures.find(item => item.operation === operation).signature_digests[0];
        await assert.rejects(sdk.request(family.methods[0], {operation, signature_digest: signature, handle: session.wire, arguments: []}));
        operations++;
      }
    }
    assert.equal(operations, 39);
    let sources = 0;
    for (const route of retired.routes.filter(route => route.operation)) {
      await assert.rejects(sdk.request("_echo_agent/facade/invoke", {operation: route.operation, signature_digest: route.signature_digests[0], handle: agent.wire, arguments: [{kind:"handle",value:session.wire}]}));
      sources++;
    }
    assert.equal(sources, 19);
    for (const value of [
      {kind:"variant",value:{type_id:"echo_core::plugin::capability::PluginCapability",variant:"LspServer",fields:[]}},
      {kind:"record",value:{type_id:"echo_agent::plugin::prepared::PluginWiringResult",fields:[{name:"lsp_discovered",value:{kind:"null"}}]}},
      {kind:"record",value:{type_id:"echo_core::plugin::registry::ResolvedComponents",fields:[{name:"lsp_config_file",value:{kind:"null"}}]}},
    ]) {
      await assert.rejects(sdk.request("_echo_agent/facade/invoke", {operation:"retirement.probe",signature_digest:"sha256:"+"0".repeat(64),handle:agent.wire,arguments:[value]}), error => error.code === "invalid_value" && error.message.includes("invalid facade argument"));
    }
    await session.close();
    // A separate retained Session stays available after the first close.
    const other = await agent.createSession();
    await other.close();
    await agent.close();
  } finally { await sdk.close(); }
});
