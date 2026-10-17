"""Exact generation backend obligations; these are not Host consumer proofs."""

import copy
import hashlib
import json
import tempfile
import threading
import unittest
from pathlib import Path
from unittest.mock import patch

from managed_persistence_backend import RETIRE, Backend, atomic_json
from test_managed_persistence_backend import batch, call, checkpoint


def prepare(scope_revision="1", version=None, scope_id="会话🙂"):
    version = version or {"managed": {"revision": "1"}}
    native = {"managed": {"revision": int(version["managed"]["revision"])}}
    payload = [
        "runtime-generation-retire-v1",
        1,
        scope_id,
        "generation-1",
        int(scope_revision),
        native,
    ]
    digest = hashlib.sha256(
        json.dumps(payload, ensure_ascii=False, separators=(",", ":")).encode()
    ).hexdigest()
    return {
        "schema_version": 1,
        "operation_id": "runtime-generation-retire-v1:" + digest,
        "payload_digest": digest,
        "scope_id": scope_id,
        "runtime_state_id": "generation-1",
        "expected_scope_revision": scope_revision,
        "expected_state_version": version,
    }


def durable(state):
    return {
        key: value
        for key, value in state.items()
        if key not in ("observations", "calls", "supported_operations")
    }


class RetirementBackendTest(unittest.TestCase):
    def setUp(self):
        root = Path(__file__).resolve().parents[3] / "target/managed-retirement-unit"
        root.mkdir(parents=True, exist_ok=True)
        self.directory = tempfile.TemporaryDirectory(dir=root)
        self.addCleanup(self.directory.cleanup)
        self.backend = Backend(self.directory.name, enable_retirement=True)
        self.backend.dispatch(
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
        self.save = {
            "scope_id": "会话🙂",
            "runtime_state_id": "generation-1",
            "conversation_epoch": "1",
            "expected_scope_revision": "0",
            "expected_state_version": "absent",
            "checkpoint": checkpoint(),
        }
        self.backend.dispatch(
            call("runtime_compare_and_save_checkpoint_with_context", request=self.save)
        )
        self.backend.dispatch(
            call("conversation_apply_transcript_projection_with_context", batch=batch())
        )

    def retire(self, request=None):
        return self.backend.dispatch(call(RETIRE, request=request or prepare()))

    def test_atomic_retirement_keeps_exact_projection_and_replays_after_backend_restart(
        self,
    ):
        before = self.backend.inspect()
        retired = self.retire()["result"]["value"]["receipt"]
        self.assertEqual(retired["status"], "retired")
        after = self.backend.inspect()
        self.assertIsNone(after["runtime"]["generation-1"]["checkpoint"])
        self.assertEqual(
            retired["version"],
            {"retired": {"revision": "2", "operation_id": prepare()["operation_id"]}},
        )
        self.assertEqual(
            retired["scope"], {**before["scopes"]["会话🙂"], "revision": "2"}
        )
        for name in ("receipts", "batches", "conversations"):
            self.assertEqual(before[name], after[name])
        self.backend = Backend(self.directory.name, enable_retirement=True)
        replay = self.retire()["result"]["value"]["receipt"]
        self.assertEqual(replay, {**retired, "status": "already_retired"})
        self.assertEqual(durable(after), durable(self.backend.inspect()))
        stale_save = self.backend.dispatch(
            call("runtime_compare_and_save_checkpoint_with_context", request=self.save)
        )
        self.assertEqual(
            stale_save["result"]["value"]["receipt"]["status"], "scope_fenced"
        )
        self.assertEqual(durable(after), durable(self.backend.inspect()))

    def test_default_seven_operations_reject_retirement_without_durable_mutation(self):
        self.backend = Backend(self.directory.name)
        before = durable(self.backend.inspect())
        rejected = self.retire()
        self.assertEqual(
            rejected["error"]["details"]["fields"][0]["value"], "unsupported"
        )
        self.assertEqual(before, durable(self.backend.inspect()))

    def test_stale_cas_and_wrong_scope_never_retire_or_change_projection(self):
        before = durable(self.backend.inspect())
        rejected = self.retire(prepare(scope_revision="42"))
        self.assertEqual(rejected["result"]["value"]["receipt"]["status"], "conflict")
        wrong = self.retire(prepare(scope_id="other-scope"))
        self.assertEqual(
            wrong["error"]["details"]["fields"][0]["value"], "semantic_conflict"
        )
        self.assertEqual(before, durable(self.backend.inspect()))

    def test_pending_projection_fences_retirement_without_acknowledging_it(self):
        state = self.backend._read()
        state["runtime"]["generation-1"]["checkpoint"] = checkpoint({"batch": batch()})
        atomic_json(self.backend.path, state)
        before = durable(self.backend.inspect())
        self.assertEqual(
            self.retire()["result"]["value"]["receipt"]["status"], "pending_projection"
        )
        self.assertEqual(before, durable(self.backend.inspect()))

    def test_invalid_digest_and_expired_deadline_start_no_durable_work(self):
        before = durable(self.backend.inspect())
        request = prepare()
        request["payload_digest"] = "tampered"
        invalid = self.retire(request)
        self.assertEqual(
            invalid["error"]["details"]["fields"][0]["value"], "invalid_configuration"
        )
        expired = call(RETIRE, request=prepare())
        expired["input"]["context"]["absolute_deadline_unix_ms"] = "1"
        rejected = self.backend.dispatch(expired)
        self.assertEqual(
            rejected["error"]["details"]["fields"][0]["value"], "deadline_exceeded"
        )
        self.assertEqual(before, durable(self.backend.inspect()))

    def test_concurrent_exact_retries_have_one_mutation_and_stable_retirement_identity(
        self,
    ):
        before = self.backend.inspect()
        results = []
        barrier = threading.Barrier(2)

        def run():
            barrier.wait()
            results.append(self.retire()["result"]["value"]["receipt"])

        threads = [threading.Thread(target=run) for _ in range(2)]
        for thread in threads:
            thread.start()
        for thread in threads:
            thread.join(timeout=5)
            self.assertFalse(thread.is_alive())
        self.assertEqual(
            sorted(item["status"] for item in results), ["already_retired", "retired"]
        )
        self.assertEqual(results[0]["version"], results[1]["version"])
        self.assertEqual(
            self.backend.inspect()["counters"]["mutations"],
            before["counters"]["mutations"] + 1,
        )

    def test_u64_revisions_remain_exact_and_atomic_failure_does_not_publish_partial_tombstone(
        self,
    ):
        state = self.backend._read()
        scope = state["scopes"]["会话🙂"]
        scope["revision"] = "9007199254740993"
        owner = state["runtime"]["generation-1"]
        owner["scope"] = copy.deepcopy(scope)
        owner["version"] = {"managed": {"revision": "9007199254740993"}}
        atomic_json(self.backend.path, state)
        request = prepare(scope["revision"], owner["version"])
        before = durable(self.backend.inspect())
        with patch(
            "managed_persistence_backend.atomic_json",
            side_effect=OSError("fixture publication failure"),
        ):
            rejected = self.retire(request)
        self.assertEqual(
            rejected["error"]["details"]["fields"][0]["value"], "outcome_unknown"
        )
        self.assertEqual(before, durable(self.backend.inspect()))
        receipt = self.retire(request)["result"]["value"]["receipt"]
        self.assertEqual(receipt["version"]["retired"]["revision"], "9007199254740994")
        self.assertEqual(receipt["scope"]["revision"], "9007199254740994")


if __name__ == "__main__":
    unittest.main()
