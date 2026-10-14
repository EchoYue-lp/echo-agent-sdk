// Diagnostic only: reproduce the exact old post-spawn readFileSync failure.
// The sparse artifact is NOT a Host executable or a business E2E substitute.
import { spawn, execFileSync } from "node:child_process";
import { createHash } from "node:crypto";
import { openSync, ftruncateSync, closeSync, readFileSync, existsSync, writeFileSync, mkdirSync, mkdtempSync, unlinkSync } from "node:fs";
import { join, dirname } from "node:path";
import { fileURLToPath } from "node:url";
const base = dirname(fileURLToPath(import.meta.url));
if (!process.env.ECHO_AGENT_SDK_OTLP_COLLECTOR) throw Error("Root-qualified real collector required");
const root = join(base, "before-run"); mkdirSync(root, { recursive: true });
const work = mkdtempSync(join(root, "setup-"));
const fixture = join(base, "source-before/sdks/shared/testing/telemetry_host_fixture.py");
const artifact = join(work, "oversized-not-executable");
const fd = openSync(artifact, "w"); try { ftruncateSync(fd, 2959264016); } finally { closeSync(fd); }
execFileSync("python3", [fixture, "--work", work]);
let service, exited;
const facts = { scope: "original hash/setup leak; not Host E2E", work, collector: process.env.ECHO_AGENT_SDK_OTLP_COLLECTOR };
try {
  service = spawn("python3", [fixture, "--work", work, "--serve"], { stdio: ["ignore", "ignore", "inherit"] });
  exited = new Promise(resolve => service.once("exit", (code, signal) => resolve({code, signal})));
  // Exact old expression, before the original native test's try/finally.
  try { createHash("sha256").update(readFileSync(artifact)).digest("hex"); }
  catch (error) { facts.primary = {code: error.code, message: error.message}; }
  if (facts.primary?.code !== "ERR_FS_FILE_TOO_LARGE") throw Error("did not reproduce original failure");
  const deadline = Date.now() + 10000;
  while (!existsSync(join(work, "ready"))) {
    if (service.exitCode !== null || Date.now() > deadline) throw Error("original fixture did not become ready after hash failure");
    await new Promise(resolve => setTimeout(resolve, 5));
  }
  const collector = JSON.parse(readFileSync(join(work, "collector.json"), "utf8"));
  facts.service_pid = service.pid; facts.collector_pid = collector.pid;
  process.kill(service.pid, 0); process.kill(collector.pid, 0);
  facts.both_original_pids_alive_after_primary = true;
  facts.original_stop_absent = !existsSync(join(work, "stop"));
} finally {
  // Explicit diagnostic cleanup AFTER observing the unmodified old leak.
  writeFileSync(join(work, "release-export"), "diagnostic cleanup");
  writeFileSync(join(work, "stop"), "diagnostic cleanup");
  if (service) {
    let timer;
    try {
      facts.cleanup_exit = await Promise.race([exited, new Promise((_, reject) => {timer=setTimeout(()=>reject(Error("diagnostic cleanup timeout")),5000);})]);
    } finally {clearTimeout(timer); if (service.exitCode === null) service.kill("SIGTERM");}
  }
  unlinkSync(artifact);
  writeFileSync(join(work,"observed.json"),JSON.stringify(facts,null,2));
}
if (facts.cleanup_exit.code !== 0) throw Error("original fixture cleanup failed");
for (const pid of [facts.service_pid,facts.collector_pid]) {
  try {process.kill(pid,0); throw Error(`original PID remains alive: ${pid}`);} catch(error) {if (error.code !== "ESRCH") throw error;}
}
console.log(JSON.stringify(facts));
