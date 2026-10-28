import { EchoAgentError } from "./errors.js";
import type { WireEventEnvelope } from "./types.js";
import { fromWireValue } from "./wire.js";

/** 只投影原Framework的V6末尾事实；正文、Candidate及观察错误不结束流。 */
export function runtimeTerminal(envelope: WireEventEnvelope): boolean {
  if (envelope.payload.event_type !== "execution_status") return false;
  const invalid = () => new EchoAgentError("serialization_violation", "malformed runtime execution status", "never", "_echo_agent/event");
  let value: unknown;
  try {
    value = fromWireValue(envelope.payload.data);
  } catch (error) {
    throw new EchoAgentError("serialization_violation", "malformed runtime execution status: " + (error instanceof Error ? error.message : String(error)), "never", "_echo_agent/event");
  }
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
