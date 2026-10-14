# Factory child completion and retry-owner evidence

This bounded slice repairs two existing Factory/CustomAgent lifecycle failures. [Receipt](receipt.json) and the [source diff](scoped-bridge.patch) bind the exact original and corrected source; [E2E diagnostics](scoped-e2e-diagnostics.patch) preserve the original assertions and add the actual result to the failure message.

The original Factory E2E filter fails two of five cases. A deterministic regression confirms that the consumer can observe the real terminal while the same child's original exclusive lease remains held. Child close then conflicts with its own completed invocation. The fix releases that same lease synchronously only after both the actual RPC answer and a validated remote terminal, before terminal delivery; pending answers, missing terminals and wrong generations still retain it. The original five Factory cases then pass.

A separate official ACP regression demonstrates that a failed child-close acknowledgement is cached permanently, preventing the explicitly retained owner from retrying. The same cleanup owner now caches only success. Failure preserves the child and blocks parent unregister; concurrent retry callers share successful cleanup, and later closes perform no extra RPC.

The final current source passes 171 Host library tests and all 34 bridge E2E tests, with zero failures or ignored tests. Strict all-target warning lint, four production panic lints and formatting also pass. Independent bounded review passes with zero critical, important or minor findings on the exact current source/test/documentation. Final whole-SDK/language delivery and global semantic closure remain separate prerequisites. No timeout increase, sleep, relaxed exclusivity or new wire/API is introduced.
