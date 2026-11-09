import test from "node:test";
import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { mkdtemp, readFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

test("the source package loads its exact catalog and digests outside the SDK repository", async () => {
  const source = fileURLToPath(new URL("..", import.meta.url));
  const work = await mkdtemp(join(tmpdir(), "sdk-typescript-pack-"));
  // npm test提供真实CLI路径；Windows直接经node启动JS，避免把.cmd当原生可执行文件。
  const npmCli = process.env.npm_execpath ?? (process.platform === "win32"
    ? join(dirname(process.execPath), "node_modules/npm/bin/npm-cli.js") : undefined);
  const command = npmCli ? process.execPath : "npm";
  const args = [...(npmCli ? [npmCli] : []), "pack", "--ignore-scripts", "--json", "--pack-destination", work];
  const [packed] = JSON.parse(execFileSync(command, args, { cwd: source, encoding: "utf8" }));
  const paths = new Set(packed.files.map(file => file.path));
  for (const name of ["facade-operation-catalog.json", "contract-digests.json"]) {
    assert.ok(paths.has(`contracts/${name}`), `package missing contracts/${name}`);
  }
  execFileSync("tar", ["-xzf", join(work, packed.filename), "-C", work]);
  const extracted = join(work, "package");
  for (const name of ["facade-operation-catalog.json", "contract-digests.json"]) {
    assert.deepEqual(await readFile(join(extracted, "contracts", name)),
      await readFile(fileURLToPath(new URL(`../../shared/${name}`, import.meta.url))));
  }
  const { FacadeCatalog } = await import(pathToFileURL(join(extracted, "dist/catalog.js")).href);
  const catalog = new FacadeCatalog();
  assert.equal(catalog.resolve("memory.store.put").method, "_echo_agent/memory/op");
  assert.ok(catalog.operations().length > 0);
  const metadata = JSON.parse(await readFile(join(extracted, "package.json"), "utf8"));
  assert.equal(metadata.private, true);
});
