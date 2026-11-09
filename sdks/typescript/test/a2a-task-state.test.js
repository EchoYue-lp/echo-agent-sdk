import test from "node:test";
import assert from "node:assert/strict";
import * as sdk from "../dist/index.js";

test("retired A2A value helpers are absent from the public SDK", () => {
  for (const name of ["A2AMessage", "A2ATask", "AgentCard", "AgentCardBuilder", "TaskState", "JwtConfig", "JwtClaims"]) {
    assert.equal(Object.hasOwn(sdk, name), false, name);
  }
  assert.equal(typeof sdk.EchoAgentClient, "function");
  const catalog = new sdk.FacadeCatalog();
  for (const operation of ["a2a.client.open", "lsp.manager.open", "lsp.client.hover"]) {
    assert.throws(() => catalog.resolve(operation), /absent/);
  }
});
