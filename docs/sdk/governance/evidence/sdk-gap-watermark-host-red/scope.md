# Issue #120 isolated Host watermark regression

This experiment copies the current SDK Rust workspace and its compile/contract
inputs. It does not change the production SDK or framework worktrees. The only
behavior change between the red and green runs is restoring the original missing
ACK cursor advancement in `DeliveryInner::acknowledge`: the red copy omits
`self.last_sent_sequence = self.last_sent_sequence.max(acked)`.

The historical `30d7e219` source is retained to show that its ACK resume path used
`Some(inner.last_sent_sequence)` without advancing that cursor to the accepted
snapshot ACK. Other current queue publication, gap authorization, serialization,
protocol, handle and framework behavior remain fixed. This is a historical
watermark-mechanism regression, **not a build or E2E result for the complete old
SDK revision**.

Reuse: the existing real Host
`core_profile_e2e::ack_window_snapshot_ack_resumes_strictly_after_watermark`
creates an actual ACP Host child, pauses the model fixture after journal events,
observes the negotiated delivery gap, obtains a Run snapshot, ACKs the gap's
snapshot watermark, and checks subsequent live/replay sequences. The experimental
fixture adds three stdout observations only; its bounds, inputs, assertions,
timing, Host startup and continuation flow are unchanged and identical in both
runs.

Framework owns journal facts and Run state. Host delivery owns only the transport
cursor and ACK window. No new API, alternate journal, test-only prune operation,
or replacement SDK state authority is added. The failure under examination is
resuming from an older transport cursor after a snapshot already covers it.

Source archives, source manifest, exact one-line behavior patch, observation-only
fixture patch and historical source establish the impact closure. Raw process
logs and command/exit/source-check receipts will establish the runtime result.
The experiment supports only this defect and this native Rust Host scenario;
separate evidence is required for the three language clients and final main.

Cargo is serialized through the coordinator's exclusive shared target lease.
Each run checks every copied SDK input and each framework Rust/Cargo input before
and after execution, and separately confirms production SDK `events.rs` unchanged.
