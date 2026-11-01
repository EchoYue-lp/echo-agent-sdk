import test from "node:test";
import assert from "node:assert/strict";
import { FacadeCatalog } from "../dist/catalog.js";

test("LSP diagnostics is absent from the catalog", () => {
  assert.throws(() => new FacadeCatalog().resolve("lsp.client.diagnostics"), /absent/);
});
