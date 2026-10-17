import test from "node:test";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { toWireValue, fromWireValue } from "../dist/index.js";

const corpus = JSON.parse(readFileSync(new URL("../../shared/fixtures/testing-contract-values.json", import.meta.url), "utf8"));
for (const item of corpus.cases) {
  test(`testing accepted value payload round-trip: ${item.name}`, () => {
    const encoded = toWireValue(item.native_input);
    assert.deepEqual(encoded, item.expected_wire);
    const serialized = JSON.parse(JSON.stringify(encoded));
    const projected = fromWireValue(serialized);
    assert.deepEqual(projected, item.expected_projection);
    // Typed values retain their discriminator; ordinary maps retain the
    // optional/null distinction and all Unicode identity fields.
    if (item.name === "context-absence-distinct") assert.equal(Object.hasOwn(projected, "call_id"), false);
    if (item.name === "context-null-identities") assert.equal(projected.call_id, null);
    if (item.name === "delay-large-subsecond") {
      assert.equal(projected.value.seconds, "18446744073709551615");
      assert.equal(projected.value.nanos, 999999999);
    }
  });
}
for (const item of corpus.invalid_cases) {
  test(`testing malformed value projection fails explicitly: ${item.name}`, () => {
    assert.throws(() => fromWireValue(item.wire), error => error instanceof Error && error.message.length > 0);
  });
}
