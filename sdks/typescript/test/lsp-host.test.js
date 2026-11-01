import test from "node:test";
import assert from "node:assert/strict";
import { FacadeCatalog } from "../dist/catalog.js";

test("LSP family is absent from the catalog", () => {
  const catalog = new FacadeCatalog();
  assert.throws(() => catalog.resolve("lsp.manager.open"), /absent/);
});
