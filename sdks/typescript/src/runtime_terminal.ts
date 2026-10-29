import { EchoAgentError } from "./errors.js";
import type { WireEventEnvelope } from "./types.js";
import { fromWireValue } from "./wire.js";

/** 只投影原Framework的V6末尾事实；正文、Candidate及观察错误不结束流。 */
export function runtimeTerminal(envelope: WireEventEnvelope): boolean {
  if (envelope.payload.event_type !== "execution_status") return false;
  let value: unknown;
  try {
    value = fromWireValue(envelope.payload.data);
  } catch (error) {
    throw new EchoAgentError("serialization_violation", "malformed runtime execution status: " + (error instanceof Error ? error.message : String(error)), "never", "_echo_agent/event");
  }
  return runtimeStatusTerminal(value);
}

/** 校验已存在的状态载体；读取其事实，不创建执行状态。 */
function runtimeStatusTerminal(value: unknown): boolean {
  const invalid = () => new EchoAgentError("serialization_violation", "malformed runtime execution status", "never", "_echo_agent/event");
  if (!value || typeof value !== "object" || Array.isArray(value)) throw invalid();
  const status = value as Record<string, unknown>;
  const phase = status.phase;
  const kind = status.status;
  const text = (value: unknown): value is string => typeof value === "string" && value.trim().length > 0;
  if (!text(status.execution_id) || (status.request_id != null && !text(status.request_id))
    || typeof phase !== "string" || !["Thinking", "Acting", "Terminal"].includes(phase)
    || typeof kind !== "string" || !["started", "waiting", "tool_pending", "candidate", "settled", "cancelled", "failed"].includes(kind)
    || (status.reason != null && typeof status.reason !== "string")) throw invalid();
  let sequence: bigint;
  if (typeof status.sequence === "bigint") sequence = status.sequence;
  else if (typeof status.sequence === "string" && /^(0|[1-9][0-9]{0,19})$/.test(status.sequence)) sequence = BigInt(status.sequence);
  else throw invalid();
  if (sequence < 0n || sequence > (1n << 64n) - 1n) throw invalid();
  return phase === "Terminal" && ["settled", "cancelled", "failed"].includes(kind);
}

/** Reverse wire 必须使用原始十进制字符串；普通 FinalAnswer 不关闭流。 */
export function validateReverseAgentValue(value: unknown, complete: boolean): void {
  if (!value || typeof value !== "object" || (value as {kind?: unknown}).kind !== "agent") return;
  const event = (value as {value?: {event?: unknown; data?: unknown}}).value;
  if (event?.event !== "execution_status") return;
  const data = event.data as Record<string, unknown> | undefined;
  if (!data || typeof data.sequence !== "string" || Object.keys(data).some((key) => !["execution_id", "request_id", "sequence", "phase", "status", "reason"].includes(key)))
    throw new TypeError("reverse execution status requires the closed native wire fields and u64 text");
  const terminal = runtimeStatusTerminal(data);
  const terminalKind = ["settled", "failed", "cancelled"].includes(String(data.status));
  if (terminalKind !== complete || (terminalKind && !terminal))
    throw new TypeError("reverse execution status must match chunk or terminal completion");
}
