import { openSync, closeSync, ftruncateSync, writeSync, readSync, readFileSync, statSync } from "node:fs";
import { createHash } from "node:crypto";
const file = process.argv[2];
const size = 2 ** 31 + 1;
const fd = openSync(file, "w+");
try {
  ftruncateSync(fd, size);
  writeSync(fd, Buffer.from("start 中文😀\n"), 0, undefined, 0);
  writeSync(fd, Buffer.from("END"), 0, 3, size - 3);
} finally { closeSync(fd); }
let error;
try { readFileSync(file); throw Error("original full-buffer read unexpectedly succeeded"); }
catch (caught) { if (caught.code !== "ERR_FS_FILE_TOO_LARGE") throw caught; error = { code: caught.code, message: caught.message }; }
const before = process.memoryUsage();
const digest = createHash("sha256");
const reader = openSync(file, "r");
const buffer = Buffer.alloc(1024 * 1024);
let reads = 0, total = 0;
try { let count; while ((count = readSync(reader, buffer, 0, buffer.length, null)) !== 0) { digest.update(buffer.subarray(0, count)); total += count; reads++; } }
finally { closeSync(reader); }
if (total !== size || reads < 2049) throw Error("full-file hashing incomplete");
const stat = statSync(file);
console.log(JSON.stringify({node:process.version, executable:process.execPath, file, logicalBytes:stat.size, physicalBytes:stat.blocks*512, originalError:error, sha256:digest.digest("hex"), totalBytesHashed:total, readCalls:reads, maximumChunkBytes:buffer.length, rssBefore:before.rss, rssAfter:process.memoryUsage().rss, maxRSSKiB:process.resourceUsage().maxRSS}));
