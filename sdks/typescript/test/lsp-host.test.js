import test from "node:test";
import assert from "node:assert/strict";
import { FacadeCatalog } from "../dist/index.js";
test("retired LSP family is absent while retained families remain cataloged", () => {
 const catalog = new FacadeCatalog();
 assert.equal(catalog.families().some(family => family.family === "lsp" || family.family === "a2a"), false);
 assert.ok(catalog.operations().length > 0);
});
