import test from "node:test";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { JwtClaims, JwtConfig } from "../dist/index.js";

test("JWT config and claims preserve local value semantics", () => {
  const config = JwtConfig.hs256("secret").withIssuer("echo-agent").withAudience("a2a");
  assert.equal(config.isEnabled(), true);
  assert.match(config.toString(), /verification_key: \[redacted\]/);
  assert.equal(JwtConfig.disabled().isEnabled(), false);
  assert.equal(new JwtClaims(null, "subject").subject(), "subject");
  assert.throws(() => JwtConfig.rs256("invalid"), /invalid RSA public key/);
});

test("retired A2A JWT mappings stay outside the external contract", () => {
  const manifest = JSON.parse(
    readFileSync(new URL("../../../contracts/sdk/parity-manifest.json", import.meta.url), "utf8"),
  );
  const accepted = JSON.parse(
    readFileSync(new URL("../../../contracts/sdk/accepted-external-contract.json", import.meta.url), "utf8"),
  );
  const adr = readFileSync(new URL("../../../docs/adr/0042-sdk-a2a-lsp-retirement-boundary.md", import.meta.url), "utf8");
  // 只检查ADR0042已批准退出的九项；上面的通用JWT本地值正控保持。
  const retired = [
    ["echo_agent::a2a::auth::JwtClaims::subject", "method"],
    ["echo_agent::a2a::auth::JwtConfig", "struct"],
    ["echo_agent::a2a::auth::JwtConfig::disabled", "method"],
    ["echo_agent::a2a::auth::JwtConfig::hs256", "method"],
    ["echo_agent::a2a::auth::JwtConfig::impl<Debug>", "trait_impl"],
    ["echo_agent::a2a::auth::JwtConfig::is_enabled", "method"],
    ["echo_agent::a2a::auth::JwtConfig::rs256", "method"],
    ["echo_agent::a2a::auth::JwtConfig::with_audience", "method"],
    ["echo_agent::a2a::auth::JwtConfig::with_issuer", "method"],
  ];
  assert.equal(retired.length, 9);
  for (const [source, kind] of retired) {
    assert.ok(adr.includes(`- Kind: ${kind}\n- Source: ${source}\n`), source);
    const matches = (entry) => entry.kind === kind && entry.source_paths.includes(source);
    assert.equal(accepted.entries.some(matches), false, source);
    // 原生遥测可保留或被上游删除，均不能恢复退役语言义务。
    for (const entry of manifest.entries.filter(matches)) {
      assert.equal(entry.sdk_scope, "host_or_rust_only", entry.path);
      assert.equal(entry.languages.typescript.status, "not_implemented", entry.path);
      assert.equal(entry.languages.typescript.contract_test, "sdk-parity/typescript/retired_a2a_lsp", entry.path);
    }
  }
});
