import unittest
from fixture import Backend


class BackendTests(unittest.TestCase):
    def test_authority_and_unicode_roundtrip(self):
        store = Backend(clock=lambda: 100)
        value = {"中文": "🌊", "__proto__": {"id": "domain", "generation": "literal"}}
        store.call("A", "store_put", {"namespace": ["sdk", "中文"], "key": "x", "value": value})
        request = {"namespace": ["sdk", "中文"], "key": "x"}
        self.assertEqual(store.call("A", "store_get", request)["value"], value)
        self.assertIsNone(store.call("B", "store_get", request))
    def test_prune_dedup_fixture_only_not_framework_proof(self):
        store = Backend(clock=lambda: 100)
        for key in ("one", "two", "expire"):
            store.call("A", "store_put", {"namespace": ["sdk"], "key": key, "value": "same中文🌊"})
        store.items[("A", ("sdk",), "expire")]["expires_at"] = 99
        self.assertEqual(store.call("A", "store_prune_expired", {"namespace": ["sdk"]}), "1")
        self.assertEqual(store.call("A", "store_dedup_by_content", {"namespace": ["sdk"]}), "1")
    def test_failure_no_write_and_retry_same_backend(self):
        store = Backend(clock=lambda: 100)
        store.fail_next = "store_put"
        request = {"namespace": ["sdk"], "key": "x", "value": "accepted"}
        with self.assertRaises(OSError):
            store.call("A", "store_put", request)
        self.assertFalse(store.items)
        store.call("A", "store_put", request)
        self.assertEqual(store.call("A", "store_search_with", {"namespace": ["sdk"], "query": {"text": "accepted", "limit": "5", "mode": {"kind": "keyword"}}})[0]["key"], "x")


if __name__ == "__main__":
    unittest.main()
