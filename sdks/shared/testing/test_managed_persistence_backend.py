"""Backend transaction evidence only; real Host evidence belongs to SDK E2E."""

import base64
import copy
import json
import subprocess
import sys
import tempfile
import threading
import time
import unittest
import urllib.request
from pathlib import Path

from managed_persistence_backend import Backend, make_server


def call(operation, **arguments):
    return {
        "operation": operation,
        "input": {
            **arguments,
            "context": {
                "absolute_deadline_unix_ms": str(time.time_ns() // 1_000_000 + 10000)
            },
        },
    }


def checkpoint(pending=None):
    return {
        "conversation_id": "generation-1",
        "messages_json": json.dumps(
            {"pending_transcript_projection": pending}, ensure_ascii=False
        ),
        "current_plan": None,
        "active_skills": [],
        "blocked_reason": None,
        "working_dir": {"encoding": "unix", "bytes_base64": "L3RtcC//"},
        "timestamp": {"unix_seconds": "9007199254740993", "nanos": 123},
    }


def batch():
    return {
        "schema_version": 1,
        "operation_id": "projection-1",
        "payload_digest": "digest-1",
        "conversation_id": "会话🙂",
        "conversation_epoch": "1",
        "generation_id": "generation-1",
        "items": [
            {
                "ordinal": "0",
                "digest": "item-1",
                "message": {
                    "id": "9007199254740993",
                    "conversation_id": "会话🙂",
                    "role": "assistant",
                    "content": "中文🙂",
                    "attachments_json": None,
                    "tool_calls_json": None,
                    "tool_result_json": None,
                    "created_at": "2026-10-12T00:00:00.000000123Z",
                },
            }
        ],
    }


class BackendTest(unittest.TestCase):
    def setUp(self):
        self.directory = tempfile.TemporaryDirectory()
        self.addCleanup(self.directory.cleanup)
        self.path = Path(self.directory.name)
        self.backend = Backend(self.path)

    def ensure(self):
        return self.backend.dispatch(
            call(
                "conversation_ensure_projection_epoch_with_context",
                request={
                    "conversation": {
                        "conversation_id": "会话🙂",
                        "user_id": "user",
                        "agent_type": None,
                        "title": None,
                    },
                    "expected_tombstone_epoch": None,
                },
            )
        )

    def apply(self):
        return self.backend.dispatch(
            call("conversation_apply_transcript_projection_with_context", batch=batch())
        )

    def test_commit_lost_response_restart_keeps_exact_receipt_once(self):
        self.ensure()
        self.backend.set_fault("lose_first_apply_response")
        lost = self.apply()
        self.assertEqual(
            lost["error"]["details"]["fields"][0]["value"], "outcome_unknown"
        )
        before = self.backend.inspect()
        self.assertEqual(before["counters"]["projected_items"], 1)
        self.backend = Backend(self.path)
        retry = self.apply()
        self.assertEqual(
            retry["result"]["value"]["receipt"]["status"], "already_applied"
        )
        after = self.backend.inspect()
        self.assertEqual(before["receipts"], after["receipts"])
        self.assertEqual(after["counters"]["projected_items"], 1)
        self.assertEqual(
            after["conversations"]["会话🙂"]["messages"][0]["message"],
            batch()["items"][0]["message"],
        )

    def test_expired_absolute_deadline_before_work_has_no_durable_mutation(self):
        request = call(
            "conversation_get_projection_authority_with_context",
            conversation_id="会话🙂",
        )
        request["input"]["context"]["absolute_deadline_unix_ms"] = "1"
        rejected = self.backend.dispatch(request)
        self.assertEqual(
            rejected["error"]["details"]["fields"][0]["value"], "deadline_exceeded"
        )
        self.assertEqual(self.backend.inspect()["counters"]["mutations"], 0)
        self.assertEqual(self.backend.inspect()["conversations"], {})

    def test_expire_fault_delays_before_work_without_commit(self):
        self.backend.set_fault("expire_before_work_once")
        request = call(
            "conversation_ensure_projection_epoch_with_context",
            request={
                "conversation": {
                    "conversation_id": "会话🙂",
                    "user_id": "user",
                    "agent_type": None,
                    "title": None,
                },
                "expected_tombstone_epoch": None,
            },
        )
        request["input"]["context"]["absolute_deadline_unix_ms"] = str(
            time.time_ns() // 1_000_000 + 20
        )
        result = self.backend.dispatch(request)
        self.assertEqual(
            result["error"]["details"]["fields"][0]["value"], "deadline_exceeded"
        )
        self.assertEqual(self.backend.inspect()["counters"]["mutations"], 0)
        self.assertFalse(self.backend.path.exists())

    def test_cli_process_restart_preserves_file_receipt_and_host_config(self):
        def start():
            ready = self.path / "ready"
            ready.unlink(missing_ok=True)
            child = subprocess.Popen(
                [
                    sys.executable,
                    str(Path(__file__).with_name("managed_persistence_backend.py")),
                    "--work",
                    str(self.path),
                ],
                stdout=subprocess.DEVNULL,
                stderr=subprocess.PIPE,
            )

            def stop():
                if child.poll() is None:
                    child.terminate()
                child.communicate(timeout=5)

            self.addCleanup(stop)
            deadline = time.monotonic() + 5
            while (
                not ready.exists()
                and child.poll() is None
                and time.monotonic() < deadline
            ):
                time.sleep(0.01)
            self.assertTrue(ready.exists(), "backend CLI failed to publish ready")
            url = json.loads((self.path / "backend.json").read_text())["url"]
            config = json.loads((self.path / "host.json").read_text())
            self.assertEqual(
                config["default_agent"]["model"]["base_url"],
                url + "/v1/chat/completions",
            )
            self.assertTrue(config["default_agent"]["agent"]["enable_tools"])
            return child, url

        def post(url, endpoint, body):
            request = urllib.request.Request(
                url + endpoint,
                data=json.dumps(body).encode(),
                headers={"Content-Type": "application/json"},
            )
            with urllib.request.urlopen(request, timeout=5) as response:
                return json.load(response)

        self.ensure()
        first, url = start()
        post(url, "/fault", {"mode": "lose_first_apply_response"})
        result = post(
            url,
            "/call",
            call(
                "conversation_apply_transcript_projection_with_context", batch=batch()
            ),
        )
        self.assertEqual(
            result["error"]["details"]["fields"][0]["value"], "outcome_unknown"
        )
        committed = self.backend.inspect()["receipts"]
        first.terminate()
        first.communicate(timeout=5)
        second, url = start()
        result = post(
            url,
            "/call",
            call(
                "conversation_apply_transcript_projection_with_context", batch=batch()
            ),
        )
        self.assertEqual(
            result["result"]["value"]["receipt"]["status"], "already_applied"
        )
        with urllib.request.urlopen(url + "/inspect", timeout=5) as response:
            actual = json.load(response)
        self.assertEqual(actual["receipts"], committed)
        self.assertEqual(actual["counters"]["projected_items"], 1)
        second.terminate()
        second.communicate(timeout=5)

    def test_cas_and_ack_commit_atomically_with_conflict_and_replay(self):
        self.ensure()
        request = {
            "scope_id": "会话🙂",
            "runtime_state_id": "generation-1",
            "conversation_epoch": "1",
            "expected_scope_revision": "0",
            "expected_state_version": "absent",
            "checkpoint": checkpoint(batch()),
        }
        cas = call("runtime_compare_and_save_checkpoint_with_context", request=request)
        saved = self.backend.dispatch(cas)["result"]["value"]["receipt"]
        self.assertEqual(saved["status"], "applied")
        replay = self.backend.dispatch(cas)["result"]["value"]["receipt"]
        self.assertEqual(replay["status"], "already_current")
        different = copy.deepcopy(cas)
        different["input"]["request"]["checkpoint"]["blocked_reason"] = "changed"
        self.assertEqual(
            self.backend.dispatch(different)["result"]["value"]["receipt"]["status"],
            "revision_conflict",
        )
        receipt = self.apply()["result"]["value"]["receipt"]
        ack = call(
            "runtime_acknowledge_transcript_projection_with_context",
            request={
                "scope_id": "会话🙂",
                "runtime_state_id": "generation-1",
                "conversation_epoch": "1",
                "expected_scope_revision": "1",
                "expected_state_version": {"managed": {"revision": "1"}},
                "projection_receipt": receipt,
                "checkpoint": checkpoint(),
            },
        )
        accepted = self.backend.dispatch(ack)["result"]["value"]["receipt"]
        self.assertEqual(accepted["status"], "applied")
        self.assertEqual(accepted["version"], {"managed": {"revision": "2"}})
        self.assertEqual(
            self.backend.dispatch(ack)["result"]["value"]["receipt"]["status"],
            "already_current",
        )
        snapshot = self.backend.dispatch(
            call(
                "runtime_load_managed_state_with_context",
                scope_id="会话🙂",
                runtime_state_id="generation-1",
            )
        )["result"]["value"]["snapshot"]
        self.assertEqual(snapshot["checkpoint"], checkpoint())

    def test_same_operation_changed_batch_or_ordinal_never_mutates(self):
        self.ensure()
        self.apply()
        changed = batch()
        changed["items"][0]["message"]["content"] = "wrong"
        rejected = self.backend.dispatch(
            call("conversation_apply_transcript_projection_with_context", batch=changed)
        )
        self.assertEqual(
            rejected["error"]["details"]["fields"][0]["value"], "semantic_conflict"
        )
        changed["operation_id"] = "projection-2"
        changed["items"][0]["digest"] = "wrong"
        conflict = self.backend.dispatch(
            call("conversation_apply_transcript_projection_with_context", batch=changed)
        )
        self.assertEqual(
            conflict["result"]["value"]["receipt"]["status"]["conflict"]["kind"],
            "ordinal_digest",
        )
        self.assertEqual(self.backend.inspect()["counters"]["projected_items"], 1)

    def test_optional_null_checkpoint_and_scope_owner_are_preserved(self):
        value = checkpoint()
        value["timestamp"]["rfc3339"] = None
        value["working_dir"]["display"] = None
        request = {
            "scope_id": "会话🙂",
            "runtime_state_id": "generation-1",
            "conversation_epoch": "1",
            "expected_scope_revision": "0",
            "expected_state_version": "absent",
            "checkpoint": value,
        }
        self.backend.dispatch(
            call("runtime_compare_and_save_checkpoint_with_context", request=request)
        )
        snapshot = self.backend.dispatch(
            call(
                "runtime_load_managed_state_with_context",
                scope_id="会话🙂",
                runtime_state_id="generation-1",
            )
        )
        self.assertEqual(snapshot["result"]["value"]["snapshot"]["checkpoint"], value)
        request["scope_id"] = "another-scope"
        rejected = self.backend.dispatch(
            call("runtime_compare_and_save_checkpoint_with_context", request=request)
        )
        self.assertEqual(
            rejected["error"]["details"]["fields"][0]["value"], "semantic_conflict"
        )
        separate = Backend(self.path / "separate-scenario")
        self.assertEqual(separate.inspect()["runtime"], {})

    def test_canonical_path_branches_are_preserved_without_platform_conversion(self):
        paths = (
            {"encoding": "utf8", "path": "/目录🙂/保存"},
            {"encoding": "utf8", "path": "C:\\目录🙂\\保存"},
            {"encoding": "utf8", "path": "\\\\server\\共享🙂"},
            {"encoding": "unix", "bytes_base64": "L3RtcC//", "display": None},
            {
                "encoding": "windows",
                "utf16_base64": base64.b64encode("C:\\目录🙂".encode("utf-16-le"))
                .decode()
                .rstrip("="),
                "display": "C:\\目录🙂",
            },
            None,
        )
        for index, path in enumerate(paths):
            with self.subTest(path=path):
                backend = Backend(self.path / str(index))
                value = checkpoint()
                value["working_dir"] = path
                request = {
                    "scope_id": "会话🙂",
                    "runtime_state_id": "generation-1",
                    "conversation_epoch": "1",
                    "expected_scope_revision": "0",
                    "expected_state_version": "absent",
                    "checkpoint": value,
                }
                outcome = backend.dispatch(
                    call(
                        "runtime_compare_and_save_checkpoint_with_context",
                        request=request,
                    )
                )
                self.assertEqual(outcome["kind"], "result", outcome)
                self.assertEqual(
                    backend.inspect()["runtime"]["generation-1"]["checkpoint"][
                        "working_dir"
                    ],
                    path,
                )

    def test_invalid_path_shapes_fail_before_any_checkpoint_commit(self):
        paths = (
            {"encoding": "utf8", "path": "relative"},
            {"encoding": "utf8", "path": ""},
            {"encoding": "utf8", "path": "/nul\0suffix"},
            {"encoding": "utf8", "path": None},
            {"encoding": "utf8", "value": "/wrong-field"},
            {"encoding": "utf8", "path": "/ok", "display": "extra"},
            {"encoding": "unix", "bytes_base64": "bm90LWFic29sdXRl"},
            {"encoding": "unix", "bytes_base64": "LwA"},
            {"encoding": "unix", "bytes_base64": "Lw=="},
            {"encoding": "windows", "utf16_base64": "QwA6AA"},
            {"encoding": "windows", "utf16_base64": "Lw"},
            {"encoding": "unknown", "path": "/ok"},
        )
        for path in paths:
            with self.subTest(path=path):
                value = checkpoint()
                value["working_dir"] = path
                request = {
                    "scope_id": "会话🙂",
                    "runtime_state_id": "generation-1",
                    "conversation_epoch": "1",
                    "expected_scope_revision": "0",
                    "expected_state_version": "absent",
                    "checkpoint": value,
                }
                outcome = self.backend.dispatch(
                    call(
                        "runtime_compare_and_save_checkpoint_with_context",
                        request=request,
                    )
                )
                self.assertEqual(outcome["kind"], "error", outcome)
                self.assertEqual(outcome["error"]["code"], "invalid_value")
                self.assertEqual(self.backend.inspect()["counters"]["mutations"], 0)

    def test_concurrent_duplicate_apply_serializes_one_transaction(self):
        self.ensure()
        results = []
        threads = [
            threading.Thread(target=lambda: results.append(self.apply()))
            for _ in range(8)
        ]
        for thread in threads:
            thread.start()
        for thread in threads:
            thread.join()
        statuses = [
            result["result"]["value"]["receipt"]["status"] for result in results
        ]
        self.assertEqual(statuses.count("applied"), 1)
        self.assertEqual(statuses.count("already_applied"), 7)
        self.assertEqual(self.backend.inspect()["counters"]["projected_items"], 1)

    def test_hold_http_response_exposes_durable_commit_before_release(self):
        self.ensure()
        self.backend.set_fault("hold_first_apply_response")
        server = make_server(self.backend)
        thread = threading.Thread(target=server.serve_forever, daemon=True)
        thread.start()
        self.addCleanup(server.server_close)
        self.addCleanup(server.shutdown)
        url = f"http://127.0.0.1:{server.server_port}"
        result = []

        def invoke():
            request = urllib.request.Request(
                url + "/call",
                data=json.dumps(
                    call(
                        "conversation_apply_transcript_projection_with_context",
                        batch=batch(),
                    )
                ).encode(),
                headers={"Content-Type": "application/json"},
            )
            with urllib.request.urlopen(request, timeout=5) as response:
                result.append(json.load(response))

        pending = threading.Thread(target=invoke)
        pending.start()
        deadline = time.monotonic() + 3
        while (
            self.backend.inspect()["counters"]["projected_items"] == 0
            and time.monotonic() < deadline
        ):
            time.sleep(0.01)
        on_disk = json.loads((self.path / "managed-state.json").read_text())
        self.assertEqual(on_disk["counters"]["projected_items"], 1)
        self.assertEqual(result, [])
        self.backend.release()
        pending.join(5)
        self.assertEqual(result[0]["result"]["value"]["receipt"]["status"], "applied")

    def test_unsupported_and_malformed_fail_explicitly(self):
        for operation in (
            "revisioned_task_compare_and_commit_execution",
            "runtime_retire_generation_with_context",
            "unknown",
        ):
            result = self.backend.dispatch(call(operation))
            self.assertEqual(
                result["error"]["details"]["fields"][0]["value"], "unsupported"
            )
        invalid = call("runtime_load_scope_authority_with_context", scope_id="scope")
        invalid["input"]["context"]["absolute_deadline_unix_ms"] = 9007199254740993
        self.assertEqual(
            self.backend.dispatch(invalid)["error"]["code"], "invalid_value"
        )

    def test_corrupt_state_is_not_overwritten(self):
        self.ensure()
        state = self.path / "managed-state.json"
        state.write_text("corrupt", encoding="utf-8")
        result = self.ensure()
        self.assertEqual(
            result["error"]["details"]["fields"][0]["value"], "corrupt_state"
        )
        self.assertEqual(state.read_text(), "corrupt")


if __name__ == "__main__":
    unittest.main()
