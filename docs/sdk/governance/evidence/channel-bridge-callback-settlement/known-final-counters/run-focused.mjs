import { spawn } from "node:child_process";
const [rootPath, logPath, ...command] = process.argv.slice(2);
if (!rootPath || !logPath || command.length === 0) throw new Error("root, log and exact command required");
const profile = ["TMPDIR=/Users/ls/MyWork/code/ylp_agent_learn/lp-agent/.worktrees/admin/channel-forward-runtime", "CARGO_TARGET_DIR=/Users/ls/MyWork/code/ylp_agent_learn/lp-agent/.worktrees/echo-agent-sdk-worktrees/sdk-final-framework-pin/target", "CARGO_BUILD_JOBS=2", "CARGO_PROFILE_DEV_DEBUG=0", "CARGO_PROFILE_TEST_DEBUG=0", "CARGO_INCREMENTAL=0"];
const child = spawn(process.execPath, ["/Users/ls/.codex/plugins/cache/supreme/supreme/0.4.12/skills/build/scripts/run-command.mjs", "--cwd", rootPath, "--log", logPath, "--", "env", ...profile, ...command], { stdio: "inherit" });
child.once("exit", (code, signal) => { if (signal) process.kill(process.pid, signal); else process.exit(code ?? 1); });
