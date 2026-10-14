import test from "node:test";
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { openSync, closeSync, ftruncateSync, writeSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { fileSha256 } from "../../shared/testing/file-sha256.mjs";

function work(t) {
  const directory = mkdtempSync(join(tmpdir(), "sdk-file-sha256-"));
  t.after(() => rmSync(directory, { recursive: true, force: true }));
  return directory;
}
for (const [name, bytes] of [
  ["empty", Buffer.alloc(0)],
  ["Unicode", Buffer.from("中文😀\n")],
  ["chunk boundary and partial tail", Buffer.concat([Buffer.alloc(1024 * 1024, 97), Buffer.from("中文🙂")])],
]) test(`file SHA256 preserves ${name}`, t => {
  const file = join(work(t), "input.bin");
  writeFileSync(file, bytes);
  assert.equal(fileSha256(file), createHash("sha256").update(bytes).digest("hex"));
});
test("file SHA256 reads a sparse binary above the Node 20 whole-file limit", t => {
  const file = join(work(t), "large.bin"), size = 2 ** 31 + 1;
  const fd = openSync(file, "w");
  try {
    ftruncateSync(fd, size);
    writeSync(fd, Buffer.from("start 中文😀\n"), 0, undefined, 0);
    writeSync(fd, Buffer.from("END"), 0, 3, size - 3);
  } finally { closeSync(fd); }
  assert.equal(fileSha256(file), "05aa6dd2c62008228e2cb1ab20553b85be8565f3ddf82f0527661905c4015750");
});
test("file SHA256 preserves missing-path IO failure", t => {
  assert.throws(() => fileSha256(join(work(t), "missing")), { code: "ENOENT" });
});
test("file SHA256 preserves directory-read IO failure", t => {
  assert.throws(() => fileSha256(work(t)), { code: "EISDIR" });
});
