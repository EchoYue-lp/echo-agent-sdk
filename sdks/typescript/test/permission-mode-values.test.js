import test from "node:test";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
const {
  PermissionMode,
  permissionModeAllowsWrite,
  permissionModeParse,
  permissionModeRequiresInteraction,
  permissionModeUsesClassifier,
} = await import(process.env.ECHO_AGENT_SDK_PERMISSION_FOCUSED_SOURCE ?? "../dist/index.js");

// 常规npm入口仍用原dist；显式focused源路径才直接加载真实生产parser，不伪造catalog。
const modes = [
  ["default", false, true, false],
  ["plan", false, true, false],
  ["auto-edit", true, false, false],
  ["full-auto", true, false, false],
  ["auto", false, false, true],
  ["bubble", false, true, false],
  ["dont-ask", false, false, false],
  ["strict", false, true, false],
];
for (const [id, write, interaction, classifier] of modes) {
  test(`permission mode exact canonical ${id} keeps its wire and policy`, () => {
    const mode = permissionModeParse(id);
    assert.equal(mode, id);
    assert.equal(JSON.parse(JSON.stringify(mode)), id);
    assert.equal(permissionModeAllowsWrite(mode), write);
    assert.equal(permissionModeRequiresInteraction(mode), interaction);
    assert.equal(permissionModeUsesClassifier(mode), classifier);
  });
}
const retiredAliases = ["ask", "accept-edits", "autoedit", "acceptedits", "fullauto", "bypass", "bypass-permissions", "bypasspermissions", "dontask", "strict-confirm", "strict-confirmation"];
const rejected = [...retiredAliases, ...modes.flatMap(([id]) => [id.toUpperCase(), ` ${id}`, `${id} `, `\t${id}\n`]), "unknown", "", "__proto__", "toString"];
for (const value of rejected) {
  test(`permission mode rejects noncanonical ${JSON.stringify(value)}`, () => {
    assert.throws(() => permissionModeParse(value), /invalid permission mode/);
  });
}
for (const value of [null, undefined, 1, {}]) {
  test(`permission mode rejects nontext ${String(value)}`, () => {
    assert.throws(() => permissionModeParse(value), TypeError);
  });
}
test("permission modes keep all eight identifiers and the default", () => {
  assert.equal(PermissionMode.Default, "default");
  assert.deepEqual(Object.values(PermissionMode).sort(), modes.map(([id]) => id).sort());
});

test("permission mode helper mappings are complete", () => {
  const manifest = JSON.parse(
    readFileSync(new URL("../../../contracts/sdk/parity-manifest.json", import.meta.url), "utf8"),
  );
  const entries = manifest.entries.filter(
    (entry) => entry.canonical && entry.languages.typescript.contract_test.endsWith("/permission_mode_values"),
  );
  assert.equal(entries.length, 6);
  for (const entry of entries) assert.equal(entry.languages.typescript.status, "done", entry.path);
});
