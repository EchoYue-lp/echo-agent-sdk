import test from "node:test";
import assert from "node:assert/strict";
import { FacadeCatalog } from "../dist/catalog.js";

test("retired A2A operations are absent from the catalog", () => {
  const catalog = new FacadeCatalog();
  for (const operation of ["a2a.client.open", "a2a.discover", "a2a.task.stream.open"]) {
    assert.throws(() => catalog.resolve(operation), /absent/);
  }
});
