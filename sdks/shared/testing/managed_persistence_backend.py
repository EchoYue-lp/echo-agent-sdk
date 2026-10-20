"""Shared, test-only file backend for real Host managed persistence scenarios.

Start: python3 managed_persistence_backend.py --work <isolated scenario directory>
Read backend.json for the loopback URL and host.json for a scripted-model Host
configuration. POST /call accepts the canonical {operation, input} component call
and returns {kind: result, result: {operation, value}} or {kind: error, error:
EchoSdkError}. The language handler wraps the former in its typed component
outcome and forwards the latter as its typed extension error.

POST /fault {mode: ...} configures a one-shot fault: expire_before_work_once waits
past the first managed call's absolute deadline; lose_first_apply_response commits
then returns OutcomeUnknown; hold_first_apply_response commits then holds the HTTP
response until POST /release (or backend shutdown). GET /inspect reads real durable
receipts/checkpoints/items/counters. Restart with the same --work to recover them.

The seven advertised operations implement backend transactions only. Scheduling,
pending-intent recovery and terminal state belong to the real framework in Host.
Exact generation retirement can be enabled explicitly with --enable-retirement.
The other three managed delete/scope-retirement operations and TaskStore CAS
are deliberately Unsupported in this test backend, never synthetic successes.
The counters are observation evidence, not Agent/Run state.
"""

from __future__ import annotations

import argparse
import base64
import binascii
import copy
import hashlib
import json
import os
import re
import tempfile
import threading
import time
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path

ENSURE = "conversation_ensure_projection_epoch_with_context"
AUTHORITY = "conversation_get_projection_authority_with_context"
APPLY = "conversation_apply_transcript_projection_with_context"
LOAD_STATE = "runtime_load_managed_state_with_context"
LOAD_SCOPE = "runtime_load_scope_authority_with_context"
SAVE = "runtime_compare_and_save_checkpoint_with_context"
ACK = "runtime_acknowledge_transcript_projection_with_context"
RETIRE = "runtime_retire_generation_with_context"
SUPPORTED = (ENSURE, AUTHORITY, APPLY, LOAD_STATE, LOAD_SCOPE, SAVE, ACK)
FAULT_MODES = (
    "none",
    "expire_before_work_once",
    "lose_first_apply_response",
    "hold_first_apply_response",
)
MAX_U64 = (1 << 64) - 1


class BackendError(Exception):
    def __init__(self, error_class, message=None, code="extension_failed"):
        self.error_class = error_class
        self.message = message or f"managed test backend {error_class}"
        self.code = code
        super().__init__(self.message)

    def outcome(self):
        return {
            "kind": "error",
            "error": {
                "code": self.code,
                "message": self.message,
                "retryable": "never",
                "details": {
                    "fields": [
                        {"key": "persistence_error_class", "value": self.error_class}
                    ]
                },
            },
        }


def invalid(message):
    return BackendError("invalid_configuration", message, "invalid_value")


def fields(value, required, optional=()):
    if (
        not isinstance(value, dict)
        or set(value) - set(required) - set(optional)
        or set(required) - set(value)
    ):
        raise invalid("missing, unknown or invalid object fields")
    return value


def text(value):
    if not isinstance(value, str):
        raise invalid("expected a string")
    return value


def number(value, signed=False):
    pattern = r"(?:0|[1-9][0-9]*|-[1-9][0-9]*)" if signed else r"(?:0|[1-9][0-9]*)"
    if not isinstance(value, str) or re.fullmatch(pattern, value) is None:
        raise invalid("integer must be a canonical decimal wire string")
    parsed = int(value)
    low, high = (-(1 << 63), (1 << 63) - 1) if signed else (0, MAX_U64)
    if not low <= parsed <= high:
        raise invalid("integer outside wire range")
    return parsed


def increment(value):
    parsed = number(value)
    if parsed == MAX_U64:
        raise BackendError("corrupt_state", "revision overflow")
    return str(parsed + 1)


def atomic_json(path, value):
    """One file contains projection + receipt, or scope + checkpoint + CAS proof."""
    descriptor, temporary = tempfile.mkstemp(prefix=".managed-", dir=path.parent)
    try:
        with os.fdopen(descriptor, "w", encoding="utf-8") as output:
            json.dump(
                value,
                output,
                ensure_ascii=False,
                separators=(",", ":"),
                allow_nan=False,
            )
            output.flush()
            os.fsync(output.fileno())
        os.replace(temporary, path)
        directory = os.open(path.parent, os.O_RDONLY)
        try:
            os.fsync(directory)
        finally:
            os.close(directory)
    finally:
        if os.path.exists(temporary):
            os.unlink(temporary)


def empty_state():
    return {
        "schema_version": 1,
        "conversations": {},
        "runtime": {},
        "scopes": {},
        "receipts": {},
        "batches": {},
        "cas_proofs": {},
        "counters": {"mutations": 0, "projected_items": 0, "duplicate_applies": 0},
    }


def validate_checkpoint(value):
    fields(
        value,
        (
            "conversation_id",
            "messages_json",
            "current_plan",
            "active_skills",
            "blocked_reason",
            "working_dir",
            "timestamp",
        ),
    )
    text(value["conversation_id"])
    text(value["messages_json"])
    for name in ("current_plan", "blocked_reason"):
        if value[name] is not None:
            text(value[name])
    if not isinstance(value["active_skills"], list):
        raise invalid("active_skills must be an array")
    for skill in value["active_skills"]:
        text(skill)
    # Do not convert native path units or time to Python datetime/Path: their
    # exact bytes, optional display and precision are part of the wire evidence.
    timestamp = fields(value["timestamp"], ("unix_seconds", "nanos"), ("rfc3339",))
    number(timestamp["unix_seconds"], signed=True)
    if (
        not isinstance(timestamp["nanos"], int)
        or isinstance(timestamp["nanos"], bool)
        or not 0 <= timestamp["nanos"] < 1_000_000_000
    ):
        raise invalid("invalid timestamp nanoseconds")
    if timestamp.get("rfc3339") is not None:
        text(timestamp["rfc3339"])
    path = value["working_dir"]
    if path is not None:
        if not isinstance(path, dict):
            raise invalid("working_dir must be a typed path or null")
        if path.get("encoding") == "utf8":
            fields(path, ("encoding", "path"))
            value = text(path["path"])
            # Match protocol::scalar::is_absolute_utf8_path across platforms;
            # Path.is_absolute() would wrongly reject Windows paths on Unix.
            absolute = (
                value.startswith(("/", "\\\\"))
                or re.match(r"^[A-Za-z]:[/\\]", value) is not None
            )
            if "\0" in value or not absolute:
                raise invalid("UTF-8 path must be absolute and NUL-free")
            return
        encodings = {"unix": "bytes_base64", "windows": "utf16_base64"}
        units = encodings.get(path.get("encoding"))
        if units is None:
            raise invalid("unknown path encoding")
        fields(path, ("encoding", units), ("display",))
        encoded = text(path[units])
        try:
            raw = base64.b64decode(encoded + "=" * (-len(encoded) % 4), validate=True)
        except (ValueError, binascii.Error) as error:
            raise invalid("native path is not canonical base64") from error
        if base64.b64encode(raw).decode("ascii").rstrip("=") != encoded:
            raise invalid("native path is not canonical unpadded base64")
        if path["encoding"] == "unix":
            absolute = raw.startswith(b"/") and b"\0" not in raw
        else:
            native = [
                int.from_bytes(raw[index : index + 2], "little")
                for index in range(0, len(raw), 2)
            ]
            absolute = (
                len(raw) % 2 == 0
                and 0 not in native
                and (
                    len(native) >= 3
                    and (65 <= native[0] <= 90 or 97 <= native[0] <= 122)
                    and native[1] == 58
                    and native[2] in (47, 92)
                    or len(native) >= 2
                    and native[:2] == [92, 92]
                )
            )
        if not absolute:
            raise invalid("native path must be absolute and NUL-free")
        if path.get("display") is not None:
            text(path["display"])


class Backend:
    def __init__(self, work, enable_retirement=False):
        self.work = Path(work)
        self.work.mkdir(parents=True, exist_ok=True)
        self.path = self.work / "managed-state.json"
        self.lock = threading.RLock()
        self.fault = "none"
        self.release_event = threading.Event()
        self.supported = SUPPORTED + ((RETIRE,) if enable_retirement else ())
        self.calls = []
        self.observations = {
            "expired": 0,
            "held_responses": 0,
            "lost_responses": 0,
            "model_calls": 0,
        }

    def _read(self):
        if not self.path.exists():
            return empty_state()
        try:
            state = json.loads(self.path.read_text(encoding="utf-8"))
            fields(state, empty_state().keys())
            if state["schema_version"] != 1:
                raise ValueError("unsupported persisted schema")
            for key in (
                "conversations",
                "runtime",
                "scopes",
                "receipts",
                "batches",
                "cas_proofs",
                "counters",
            ):
                if not isinstance(state[key], dict):
                    raise TypeError("invalid persisted map")
            return state
        except (TypeError, ValueError, OSError, BackendError) as error:
            raise BackendError("corrupt_state", str(error)) from error

    def inspect(self):
        with self.lock:
            return {
                **self._read(),
                "observations": dict(self.observations),
                "supported_operations": list(self.supported),
                "calls": copy.deepcopy(self.calls),
            }

    def set_fault(self, mode):
        if mode not in FAULT_MODES:
            raise invalid("unknown fixture fault mode")
        with self.lock:
            self.fault = mode
            self.release_event.clear()

    def release(self):
        self.release_event.set()

    def dispatch(self, call):
        try:
            fields(call, ("operation", "input"))
            operation = text(call["operation"])
            with self.lock:
                self.calls.append(copy.deepcopy(call))
            if operation not in self.supported:
                raise BackendError(
                    "unsupported", f"test backend does not implement {operation}"
                )
            arguments = call["input"]
            if not isinstance(arguments, dict) or "context" not in arguments:
                raise invalid("managed call requires context")
            fields(arguments["context"], ("absolute_deadline_unix_ms",))
            deadline = number(
                arguments["context"]["absolute_deadline_unix_ms"], signed=True
            )
            with self.lock:
                expire = self.fault == "expire_before_work_once"
                if expire:
                    self.fault = "none"
            if expire:
                delay = max(0, (deadline - time.time_ns() // 1_000_000) / 1000) + 0.03
                self.release_event.wait(delay)
            hold = False
            with self.lock:
                # Recheck after the transaction lock: a deadline may expire while
                # queued, so no durable work (including observation counters) starts.
                if deadline <= time.time_ns() // 1_000_000:
                    self.observations["expired"] += 1
                    raise BackendError("deadline_exceeded")
                state = self._read()
                original = copy.deepcopy(state)
                value = self._execute(state, operation, arguments)
                if state != original:
                    atomic_json(self.path, state)
                outcome = {
                    "kind": "result",
                    "result": {"operation": operation, "value": value},
                }
                if operation == APPLY and value["receipt"]["status"] == "applied":
                    fault = self.fault
                    if fault in (
                        "lose_first_apply_response",
                        "hold_first_apply_response",
                    ):
                        self.fault = "none"
                        if fault == "lose_first_apply_response":
                            self.observations["lost_responses"] += 1
                            raise BackendError(
                                "outcome_unknown",
                                "projection committed; acknowledgement deliberately lost",
                            )
                        self.observations["held_responses"] += 1
                        hold = True
            # Never hold the transaction lock: /inspect and restarted Host
            # callbacks must be able to see and reconcile the committed receipt.
            if hold and not self.release_event.wait(60):
                return BackendError(
                    "outcome_unknown", "committed projection response hold expired"
                ).outcome()
            return outcome
        except BackendError as error:
            return error.outcome()
        except (KeyError, TypeError, ValueError) as error:
            return invalid(str(error)).outcome()
        except OSError as error:
            # A write/fsync error cannot prove whether replacement committed.
            return BackendError("outcome_unknown", str(error)).outcome()

    def _execute(self, state, operation, arguments):
        shapes = {
            ENSURE: ("request", "context"),
            AUTHORITY: ("conversation_id", "context"),
            APPLY: ("batch", "context"),
            LOAD_STATE: ("scope_id", "runtime_state_id", "context"),
            LOAD_SCOPE: ("scope_id", "context"),
            SAVE: ("request", "context"),
            ACK: ("request", "context"),
            RETIRE: ("request", "context"),
        }
        fields(arguments, shapes[operation])
        if operation == ENSURE:
            request = fields(
                arguments["request"], ("conversation", "expected_tombstone_epoch")
            )
            conversation = fields(
                request["conversation"],
                ("conversation_id", "user_id", "agent_type", "title"),
            )
            identity = text(conversation["conversation_id"])
            text(conversation["user_id"])
            for name in ("agent_type", "title"):
                if conversation[name] is not None:
                    text(conversation[name])
            if request["expected_tombstone_epoch"] is not None:
                number(request["expected_tombstone_epoch"])
            existing = state["conversations"].get(identity)
            if existing is None:
                authority = {
                    "conversation_id": identity,
                    "epoch": "1",
                    "revision": "1",
                    "lifecycle": "live",
                    "delete_receipt_retention_floor_epoch": "0",
                }
                state["conversations"][identity] = {
                    "authority": authority,
                    "conversation": copy.deepcopy(conversation),
                    "messages": [],
                }
                state["counters"]["mutations"] += 1
                return {"receipt": {"authority": authority, "status": "created"}}
            return {
                "receipt": {"authority": existing["authority"], "status": "existing"}
            }
        if operation == AUTHORITY:
            record = state["conversations"].get(text(arguments["conversation_id"]))
            return {"authority": record["authority"] if record is not None else None}
        if operation == LOAD_SCOPE:
            return {"authority": state["scopes"].get(text(arguments["scope_id"]))}
        if operation == LOAD_STATE:
            scope_id, identity = (
                text(arguments["scope_id"]),
                text(arguments["runtime_state_id"]),
            )
            record = state["runtime"].get(identity)
            if record is not None and record["scope"]["scope_id"] != scope_id:
                raise BackendError(
                    "semantic_conflict", "runtime state belongs to a different scope"
                )
            if record is not None:
                record = {**record, "scope": state["scopes"][scope_id]}
            return {"snapshot": record}
        if operation == APPLY:
            return {"receipt": self._apply(state, arguments["batch"])}
        if operation == RETIRE:
            return {"receipt": self._retire(state, arguments["request"])}
        return {"receipt": self._save(state, arguments["request"], operation)}

    def _retire(self, state, request):
        fields(
            request,
            (
                "schema_version",
                "operation_id",
                "payload_digest",
                "scope_id",
                "runtime_state_id",
                "expected_scope_revision",
                "expected_state_version",
            ),
        )
        if request["schema_version"] != 1 or isinstance(
            request["schema_version"], bool
        ):
            raise invalid("unsupported retirement schema")
        scope_id, identity = (
            text(request["scope_id"]),
            text(request["runtime_state_id"]),
        )
        if not scope_id or not identity:
            raise invalid("retirement identities must not be empty")
        scope_revision = number(request["expected_scope_revision"])
        expected = request["expected_state_version"]
        if expected == "absent":
            native_expected = expected
        else:
            fields(expected, ("managed",))
            fields(expected["managed"], ("revision",))
            native_expected = {
                "managed": {"revision": number(expected["managed"]["revision"])}
            }
        # Framework prepare/validate hashes native serde JSON, whose u64 values
        # are numbers. Wire strings remain unchanged in the retained CAS proof.
        encoded = json.dumps(
            [
                "runtime-generation-retire-v1",
                1,
                scope_id,
                identity,
                scope_revision,
                native_expected,
            ],
            ensure_ascii=False,
            separators=(",", ":"),
        ).encode("utf-8")
        digest = hashlib.sha256(encoded).hexdigest()
        if (
            text(request["payload_digest"]) != digest
            or text(request["operation_id"]) != "runtime-generation-retire-v1:" + digest
        ):
            raise invalid("retirement identity does not match its payload")
        scope = state["scopes"].get(
            scope_id,
            {
                "scope_id": scope_id,
                "conversation_epoch": None,
                "revision": "0",
                "lifecycle": "active",
            },
        )
        previous = state["runtime"].get(identity)
        if previous is not None and previous["scope"]["scope_id"] != scope_id:
            raise BackendError(
                "semantic_conflict", "runtime owner does not match scope"
            )
        version = previous["version"] if previous is not None else "absent"
        receipt = {
            "scope": copy.deepcopy(scope),
            "runtime_state_id": identity,
            "version": copy.deepcopy(version),
        }
        proof = {"operation": RETIRE, "request": request}
        if isinstance(version, dict) and "retired" in version:
            return {
                **receipt,
                "status": "already_retired"
                if state["cas_proofs"].get(identity) == proof
                else "conflict",
            }
        if scope["lifecycle"] != "active":
            return {**receipt, "status": "scope_fenced"}
        if previous is not None and previous["checkpoint"] is not None:
            payload = json.loads(previous["checkpoint"]["messages_json"])
            if payload.get("pending_transcript_projection") is not None:
                return {**receipt, "status": "pending_projection"}
        if (
            scope["revision"] != request["expected_scope_revision"]
            or version != expected
        ):
            return {**receipt, "status": "conflict"}
        retired = {
            "retired": {
                "revision": increment(version["managed"]["revision"])
                if version != "absent"
                else "1",
                "operation_id": request["operation_id"],
            }
        }
        authority = {**scope, "revision": increment(scope["revision"])}
        # One atomic document publishes the tombstone, scope revision and exact
        # replay proof together. Retirement leaves conversation projections intact.
        state["scopes"][scope_id] = authority
        state["runtime"][identity] = {
            "scope": copy.deepcopy(authority),
            "runtime_state_id": identity,
            "version": retired,
            "checkpoint": None,
        }
        state["cas_proofs"][identity] = copy.deepcopy(proof)
        state["counters"]["mutations"] += 1
        return {
            "scope": authority,
            "runtime_state_id": identity,
            "version": retired,
            "status": "retired",
        }

    def _apply(self, state, batch):
        fields(
            batch,
            (
                "schema_version",
                "operation_id",
                "payload_digest",
                "conversation_id",
                "conversation_epoch",
                "generation_id",
                "items",
            ),
        )
        if batch["schema_version"] != 1 or isinstance(batch["schema_version"], bool):
            raise invalid("unsupported batch schema")
        identity, digest = text(batch["operation_id"]), text(batch["payload_digest"])
        conversation_id = text(batch["conversation_id"])
        text(batch["generation_id"])
        number(batch["conversation_epoch"])
        if not isinstance(batch["items"], list):
            raise invalid("items must be an array")
        for item in batch["items"]:
            fields(item, ("ordinal", "digest", "message"))
            number(item["ordinal"])
            text(item["digest"])
            message = fields(
                item["message"],
                (
                    "id",
                    "conversation_id",
                    "role",
                    "content",
                    "attachments_json",
                    "tool_calls_json",
                    "tool_result_json",
                    "created_at",
                ),
            )
            if message["id"] is not None:
                number(message["id"], signed=True)
            if text(message["conversation_id"]) != conversation_id:
                raise BackendError(
                    "semantic_conflict",
                    "projection message belongs to a different conversation",
                )
            for name in ("role", "created_at"):
                text(message[name])
            for name in (
                "content",
                "attachments_json",
                "tool_calls_json",
                "tool_result_json",
            ):
                if message[name] is not None:
                    text(message[name])
        existing = state["receipts"].get(identity)
        if existing is not None:
            if state["batches"].get(identity) != batch:
                raise BackendError(
                    "semantic_conflict",
                    "operation identity reused with different payload",
                )
            state["counters"]["duplicate_applies"] += 1
            return {**copy.deepcopy(existing), "status": "already_applied"}
        record = state["conversations"].get(conversation_id)
        if record is None:
            raise BackendError(
                "corrupt_state", "projection has no conversation authority"
            )
        authority = record["authority"]
        receipt = {
            "operation_id": identity,
            "payload_digest": digest,
            "authority": copy.deepcopy(authority),
        }
        if (
            authority["epoch"] != batch["conversation_epoch"]
            or authority["lifecycle"] != "live"
        ):
            receipt["status"] = {
                "fenced": {
                    "current_epoch": authority["epoch"],
                    "lifecycle": authority["lifecycle"],
                }
            }
            return receipt
        ordinal_map = {item["ordinal"]: item for item in record["messages"]}
        new_items = []
        for item in batch["items"]:
            previous = ordinal_map.get(item["ordinal"])
            if previous is not None and previous != item:
                receipt["status"] = {
                    "conflict": {
                        "current_epoch": authority["epoch"],
                        "current_revision": authority["revision"],
                        "kind": "ordinal_digest",
                    }
                }
                return receipt
            if previous is None:
                ordinal_map[item["ordinal"]] = item
                new_items.append(copy.deepcopy(item))
        authority["revision"] = increment(authority["revision"])
        record["messages"].extend(new_items)
        receipt["authority"] = copy.deepcopy(authority)
        receipt["status"] = "applied"
        state["receipts"][identity] = copy.deepcopy(receipt)
        state["batches"][identity] = copy.deepcopy(batch)
        state["counters"]["projected_items"] += len(new_items)
        state["counters"]["mutations"] += 1
        return receipt

    def _save(self, state, request, operation):
        required = (
            "scope_id",
            "runtime_state_id",
            "conversation_epoch",
            "expected_scope_revision",
            "expected_state_version",
            "checkpoint",
        )
        fields(
            request,
            required + (("projection_receipt",) if operation == ACK else ()),
            ("managed_import",) if operation == SAVE else (),
        )
        if request.get("managed_import") is not None:
            raise BackendError(
                "unsupported",
                "test backend does not implement managed import epoch transition",
            )
        scope_id, identity = (
            text(request["scope_id"]),
            text(request["runtime_state_id"]),
        )
        if request["conversation_epoch"] is not None:
            number(request["conversation_epoch"])
        number(request["expected_scope_revision"])
        expected = request["expected_state_version"]
        if expected != "absent":
            fields(expected, ("managed",))
            fields(expected["managed"], ("revision",))
            number(expected["managed"]["revision"])
        validate_checkpoint(request["checkpoint"])
        if request["checkpoint"]["conversation_id"] != identity:
            raise BackendError(
                "semantic_conflict", "checkpoint identity differs from runtime owner"
            )
        scope = state["scopes"].get(
            scope_id,
            {
                "scope_id": scope_id,
                "conversation_epoch": None,
                "revision": "0",
                "lifecycle": "active",
            },
        )
        previous = state["runtime"].get(identity)
        if previous is not None and previous["scope"]["scope_id"] != scope_id:
            raise BackendError(
                "semantic_conflict", "runtime owner does not match scope"
            )
        version = previous["version"] if previous is not None else "absent"
        receipt = {
            "scope": copy.deepcopy(scope),
            "runtime_state_id": identity,
            "version": copy.deepcopy(version),
        }
        if (
            (isinstance(version, dict) and "retired" in version)
            or scope["lifecycle"] != "active"
            or (
                scope["revision"] != "0"
                and scope["conversation_epoch"] != request["conversation_epoch"]
            )
        ):
            return {**receipt, "status": "scope_fenced"}
        proof = {"operation": operation, "request": request}
        if state["cas_proofs"].get(identity) == proof:
            return {**receipt, "status": "already_current"}
        if (
            scope["revision"] != request["expected_scope_revision"]
            or version != expected
        ):
            return {**receipt, "status": "revision_conflict"}
        if operation == ACK:
            projection = request["projection_receipt"]
            fields(
                projection, ("operation_id", "payload_digest", "authority", "status")
            )
            original = state["receipts"].get(projection["operation_id"])
            normalized = {**projection, "status": "applied"}
            if (
                projection["status"] not in ("applied", "already_applied")
                or normalized != original
                or projection["authority"]["epoch"] != request["conversation_epoch"]
            ):
                raise BackendError(
                    "semantic_conflict",
                    "acknowledgement does not reference committed projection receipt",
                )
            if previous is None:
                raise BackendError(
                    "semantic_conflict",
                    "acknowledgement requires existing managed checkpoint",
                )
            batch = state["batches"].get(projection["operation_id"])
            if (
                batch is None
                or batch["conversation_id"] != scope_id
                or batch["generation_id"] != identity
            ):
                raise BackendError(
                    "semantic_conflict",
                    "projection receipt belongs to a different scope or runtime owner",
                )
        revision = (
            increment(version["managed"]["revision"]) if version != "absent" else "1"
        )
        authority = {
            "scope_id": scope_id,
            "conversation_epoch": request["conversation_epoch"],
            "revision": increment(scope["revision"]),
            "lifecycle": "active",
        }
        version = {"managed": {"revision": revision}}
        state["scopes"][scope_id] = authority
        state["runtime"][identity] = {
            "scope": copy.deepcopy(authority),
            "runtime_state_id": identity,
            "version": version,
            "checkpoint": copy.deepcopy(request["checkpoint"]),
        }
        state["cas_proofs"][identity] = copy.deepcopy(proof)
        state["counters"]["mutations"] += 1
        return {
            "scope": authority,
            "runtime_state_id": identity,
            "version": version,
            "status": "applied",
        }


def make_server(backend):
    class Handler(BaseHTTPRequestHandler):
        def _reply(self, value, status=200):
            body = json.dumps(value, ensure_ascii=False, allow_nan=False).encode(
                "utf-8"
            )
            self.send_response(status)
            self.send_header("Content-Type", "application/json")
            self.send_header("Content-Length", str(len(body)))
            self.end_headers()
            try:
                self.wfile.write(body)
            except (BrokenPipeError, ConnectionResetError):
                pass

        def do_GET(self):
            if self.path != "/inspect":
                self._reply({"error": "unknown endpoint"}, 404)
                return
            try:
                self._reply(backend.inspect())
            except BackendError as error:
                self._reply(error.outcome())

        def do_POST(self):
            try:
                length = int(self.headers.get("Content-Length", "0"))
                if not 0 < length <= 16 * 1024 * 1024:
                    raise invalid("missing or oversized request")
                request = json.loads(self.rfile.read(length).decode("utf-8"))
                if self.path == "/call":
                    self._reply(backend.dispatch(request))
                elif self.path == "/fault":
                    backend.set_fault(fields(request, ("mode",))["mode"])
                    self._reply({"configured": request["mode"]})
                elif self.path == "/release":
                    fields(request, ())
                    backend.release()
                    self._reply({"released": True})
                elif self.path == "/v1/chat/completions":
                    with backend.lock:
                        backend.observations["model_calls"] += 1
                        turn = backend.observations["model_calls"]
                    chunk = {
                        "id": "managed-fixture",
                        "choices": [
                            {
                                "index": 0,
                                "delta": {
                                    "role": "assistant",
                                    "content": f"managed answer {turn} 中文🙂",
                                },
                                "finish_reason": "stop",
                            }
                        ],
                    }
                    body = (
                        "data: "
                        + json.dumps(chunk, ensure_ascii=False)
                        + "\n\ndata: [DONE]\n\n"
                    ).encode("utf-8")
                    self.send_response(200)
                    self.send_header("Content-Type", "text/event-stream")
                    self.send_header("Content-Length", str(len(body)))
                    self.end_headers()
                    self.wfile.write(body)
                else:
                    self._reply({"error": "unknown endpoint"}, 404)
            except (BackendError, ValueError, UnicodeDecodeError) as error:
                outcome = (
                    error.outcome()
                    if isinstance(error, BackendError)
                    else invalid(str(error)).outcome()
                )
                self._reply(outcome, 400)

        def log_message(self, *_arguments):
            pass

    server = ThreadingHTTPServer(("127.0.0.1", 0), Handler)
    server.daemon_threads = True
    return server


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--work", type=Path, required=True)
    parser.add_argument("--fault", choices=FAULT_MODES, default="none")
    parser.add_argument("--enable-retirement", action="store_true")
    arguments = parser.parse_args()
    backend = Backend(arguments.work, enable_retirement=arguments.enable_retirement)
    backend.set_fault(arguments.fault)
    server = make_server(backend)
    url = f"http://127.0.0.1:{server.server_port}"
    atomic_json(
        arguments.work / "backend.json",
        {
            "url": url,
            "state_path": str(backend.path.resolve()),
            "supported_operations": list(backend.supported),
        },
    )
    config = {
        "schema_version": 1,
        "default_agent": {
            "model": {
                "provider": "fixture",
                "name": "fixture-model",
                "base_url": url + "/v1/chat/completions",
                "api_protocol": "chat_completions",
                "auth_token": "fixture-secret",
            },
            "agent": {
                "name": "managed-fixture",
                "system_prompt": "Answer directly.",
                "max_iterations": 4,
                # ACP prepares its stdio MCP tool adapter even for this model's
                # direct answers; use the real Host's required startup contract.
                "enable_tools": True,
            },
        },
        "sdk_profile": {"state_root": str((arguments.work / "host-state").resolve())},
    }
    atomic_json(arguments.work / "host.json", config)
    (arguments.work / "ready").touch()
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        pass
    finally:
        backend.release()
        server.server_close()


if __name__ == "__main__":
    main()
