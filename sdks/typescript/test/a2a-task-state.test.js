import test from "node:test";
import assert from "node:assert/strict";
import { FacadeCatalog } from "../dist/catalog.js";

test("A2A and LSP routes are absent from the catalog", () => {
  const catalog = new FacadeCatalog();
  for (const operation of ["a2a.client.open", "a2a.task.send", "lsp.manager.open", "lsp.client.hover"]) {
    assert.throws(() => catalog.resolve(operation), /absent/);
  }
});
