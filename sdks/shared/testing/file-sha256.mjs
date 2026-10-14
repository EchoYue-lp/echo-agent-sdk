import { createHash } from "node:crypto";
import { openSync, readSync, closeSync } from "node:fs";

// Host 可能超过整文件 Buffer 上限；固定分块保留完整二进制摘要和原 IO 错误。
export function fileSha256(path) {
  const digest = createHash("sha256");
  const buffer = Buffer.alloc(1024 * 1024);
  const file = openSync(path, "r");
  try {
    let count;
    while ((count = readSync(file, buffer, 0, buffer.length, null)) !== 0) {
      digest.update(buffer.subarray(0, count));
    }
  } finally {
    closeSync(file);
  }
  return digest.digest("hex");
}
