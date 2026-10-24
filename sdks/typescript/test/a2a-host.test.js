import test from "node:test";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { EchoAgentClient } from "../dist/index.js";
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
