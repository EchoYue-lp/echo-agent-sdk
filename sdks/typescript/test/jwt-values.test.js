import test from "node:test";
import assert from "node:assert/strict";
import * as sdk from "../dist/index.js";
test("A2A JWT constructors are withdrawn while non-A2A security helpers remain", () => {
  assert.equal(Object.hasOwn(sdk, "JwtConfig"), false);
  assert.equal(Object.hasOwn(sdk, "JwtClaims"), false);
  assert.equal(typeof sdk.EchoAgentClient, "function");
});
