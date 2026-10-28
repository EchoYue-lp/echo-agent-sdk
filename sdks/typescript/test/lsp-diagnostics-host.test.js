import test from "node:test";
import assert from "node:assert/strict";
import { FacadeCatalog } from "../dist/index.js";
test("the old LSP diagnostics route cannot resolve locally", () => {
 assert.throws(() => new FacadeCatalog().resolve("lsp.client.diagnostics"), /absent/);
});
