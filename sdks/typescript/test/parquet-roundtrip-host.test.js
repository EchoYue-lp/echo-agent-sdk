import test from "node:test";
import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { mkdtemp, readFile, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { EchoAgentClient } from "../dist/index.js";
import { fileSha256 } from "../../shared/testing/file-sha256.mjs";

const host = process.env.ECHO_AGENT_SDK_HOST?.trim();
if (process.env.ECHO_AGENT_SDK_REQUIRE_HOST_E2E === "1" && !host) {
  throw new Error("required Parquet Host binary is missing");
}
const fixture = fileURLToPath(new URL("../../shared/testing/parquet_roundtrip_fixture.py", import.meta.url));

test("real Host Parquet roundtrip preserves Unicode, nulls, numbers and failed-write sentinel", { skip: !host, timeout: 30000 }, async () => {
  const work = await mkdtemp(join(tmpdir(), "sdk-parquet-typescript-"));
  const helper = (...args) => execFileSync(process.env.PYTHON ?? "python3", [fixture, "--work", work, ...args], { encoding: "utf8" });
  helper();
  const rows = [];
  const catalogPath = process.env.ECHO_AGENT_SDK_CATALOG_PATH;
  const sdk = await EchoAgentClient.spawn({ hostCommand: host, args: ["--config", join(work, "host.json")], catalogPath });
  try {
    const agent = await sdk.createAgent();
    const session = await agent.createSession(work);
    for (const step of JSON.parse(await readFile(join(work, "scenario.json"), "utf8"))) {
      const row = { ...step };
      try {
        row.result = await session.invoke(step.operation, step.arguments);
        assert.equal(step.expected_error, null, step.id);
      } catch (error) {
        row.error = { code: error.code, message: error.message, operation: error.operation, retryable: error.retryable,
          handle: error.handle ?? null, details: error.details ?? null };
        assert.equal(error.code, step.expected_error, JSON.stringify(row));
      }
      rows.push(row);
    }
    await session.close();
    await agent.close();
  } finally {
    await sdk.close();
  }
  await writeFile(join(work, "results.json"), JSON.stringify({ host, host_sha256: fileSha256(host), catalog_path: catalogPath ?? null, rows }, null, 2));
  console.log("Parquet evidence:", work, helper("--verify", join(work, "results.json")).trim());
});
