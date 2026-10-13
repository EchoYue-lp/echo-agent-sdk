package com.echoagent.sdk;

import com.fasterxml.jackson.databind.JsonNode;

import java.util.ArrayDeque;
import java.util.Objects;
import java.math.BigInteger;
import java.util.concurrent.Flow;
import java.util.concurrent.ForkJoinPool;
import java.util.function.Consumer;

/**
 * Bounded live delivery: each queue retains at most capacity data items and one
 * coalesced gap. Successful closure drains retained values before completion.
 * Failure discards undelivered values and reports an error; those values are
 * never acknowledged, so callers can recover from their consumed cursor.
 */
final class BoundedPublisher implements Flow.Publisher<JsonNode>, AutoCloseable {
    static final int DEFAULT_CAPACITY = 256;
    private record Delivery(JsonNode value, boolean gap) {}
    private final ArrayDeque<Delivery> pending = new ArrayDeque<>();
    private final java.util.LinkedHashSet<Subscription> subscriptions = new java.util.LinkedHashSet<>();
    private Throwable failure;
    private final int capacity;
    private final Consumer<JsonNode> onConsumed;
    private WireHandle expectedStream;
    private BigInteger lastSequence = BigInteger.ZERO;
    // These are bounded-delivery projections, not framework replay authority.
    // The client verifies external replay coverage before advancing consumption.
    private BigInteger consumedSequence = BigInteger.ZERO;
    private record LiveRange(BigInteger base,BigInteger end) {}
    // Inclusive intervals of actual live facts; a pending ACK jump cannot fill holes.
    private final java.util.List<LiveRange> liveRanges = new java.util.ArrayList<>();
    private BigInteger lastGapWatermark = BigInteger.ZERO;
    private final java.util.Map<Object, BigInteger> pendingAcknowledgements = new java.util.IdentityHashMap<>();
    private boolean closed;

    BoundedPublisher() {
        this(DEFAULT_CAPACITY);
    }

    BoundedPublisher(int capacity) {
        this(capacity, ignored -> {});
    }

    BoundedPublisher(int capacity, Consumer<JsonNode> onConsumed) {
        if (capacity < 1) throw new IllegalArgumentException("publisher capacity must be positive");
        this.onConsumed = Objects.requireNonNull(onConsumed, "onConsumed");
        this.capacity = capacity;
    }

    BoundedPublisher(int capacity, Consumer<JsonNode> onConsumed, WireHandle expectedStream) {
        this(capacity, onConsumed);
        if (expectedStream != null && !"stream".equals(expectedStream.kind())) {
            throw new IllegalArgumentException("event publisher requires a stream handle");
        }
        this.expectedStream = expectedStream;
    }

    synchronized boolean validateExpectedStream(WireHandle stream) {
        Objects.requireNonNull(stream, "stream");
        if (!"stream".equals(stream.kind())) {
            fail(new EchoAgentException("serialization_violation", "event publisher requires a stream handle",
                    "never", "_echo_agent/event", stream.toJson()));
            return false;
        }
        if (expectedStream == null) expectedStream = stream;
        if (!expectedStream.equals(stream)) {
            fail(new EchoAgentException("handle_mismatch", "event stream handle changed generation",
                    "never", "_echo_agent/event", stream.toJson()));
            return false;
        }
        return true;
    }

    @Override
    public void subscribe(Flow.Subscriber<? super JsonNode> subscriber) {
        Objects.requireNonNull(subscriber, "subscriber");
        var subscription = new Subscription(subscriber);
        synchronized (this) {
            subscription.queue.addAll(pending);
            pending.clear();
            subscriptions.add(subscription);
        }
        try { subscriber.onSubscribe(subscription); }
        catch (Throwable error) { subscription.cancel(); return; }
        synchronized (this) { subscription.started = true; subscription.schedule(); }
    }

    private final class Subscription implements Flow.Subscription {
        private final Flow.Subscriber<? super JsonNode> subscriber;
        private final ArrayDeque<Delivery> queue = new ArrayDeque<>();
        private long demand;
        private boolean started;
        private boolean cancelled;
        private boolean draining;
        private Throwable requestFailure;
        Subscription(Flow.Subscriber<? super JsonNode> subscriber) { this.subscriber = subscriber; }
        @Override public void request(long count) {
            synchronized (BoundedPublisher.this) {
                if (cancelled) return;
                if (count <= 0) { requestFailure = new IllegalArgumentException("subscriber demand must be positive"); queue.clear(); }
                else demand = count > Long.MAX_VALUE - demand ? Long.MAX_VALUE : demand + count;
                schedule();
            }
        }
        @Override public void cancel() {
            synchronized (BoundedPublisher.this) {
                cancelled = true; queue.clear(); subscriptions.remove(this);
            }
        }
        private void schedule() {
            if (!started || cancelled || draining) return;
            draining = true;
            ForkJoinPool.commonPool().execute(this::drain);
        }
        private void drain() {
            while (true) {
                Delivery next;
                Throwable error;
                boolean complete;
                synchronized (BoundedPublisher.this) {
                    if (cancelled) { draining = false; return; }
                    error = requestFailure != null ? requestFailure : failure;
                    complete = error == null && closed && queue.isEmpty();
                    if (error != null || complete) {
                        cancelled = true; queue.clear(); subscriptions.remove(this); draining = false; next = null;
                    } else if (demand > 0 && !queue.isEmpty()) {
                        next = queue.removeFirst();
                        if (demand != Long.MAX_VALUE) demand--;
                    } else { draining = false; return; }
                }
                if (error != null || complete) {
                    try { if (error != null) subscriber.onError(error); else subscriber.onComplete(); }
                    catch (Throwable ignored) { /* A subscriber failure cannot reopen a terminated subscription. */ }
                    return;
                }
                try {
                    JsonNode item = next.value().deepCopy();
                    subscriber.onNext(item);
                    // Consumers may mutate their own JsonNode copy; ACK facts
                    // must remain the original delivered stream and sequence.
                    onConsumed.accept(next.value().deepCopy());
                } catch (Throwable subscriberFailure) { cancel(); return; }
            }
        }
    }

    synchronized void acknowledgeReplay(WireHandle stream, BigInteger sequence) {
        if(validateExpectedStream(stream) && sequence.compareTo(lastSequence)>0)lastSequence=sequence;
    }

    /** User ACK validation is read-only: a stale caller cannot poison a valid live feed. */
    synchronized boolean ownsStream(WireHandle stream) {
        return stream != null && "stream".equals(stream.kind()) && stream.equals(expectedStream);
    }

    /** Observed live prefix or the exact validated snapshot watermark can support an ACK. */
    synchronized boolean canAcknowledgeObserved(BigInteger sequence) {
        return sequence != null && sequence.signum() > 0
                && (sequence.compareTo(consumedSequence) <= 0
                    || liveRanges.stream().anyMatch(range -> range.base().compareTo(sequence)<=0 && range.end().compareTo(sequence)>=0)
                    || sequence.equals(lastGapWatermark));
    }

    /** Pending writes only permit the peer's immediate contiguous continuation. */
    synchronized void beginAcknowledgement(Object submission, BigInteger sequence) {
        pendingAcknowledgements.put(submission, sequence);
    }

    synchronized void endAcknowledgement(Object submission) {
        pendingAcknowledgements.remove(submission);
    }

    /** Called only after the client proves live/gap/replay consumption coverage. */
    synchronized void acknowledgeConsumedCursor(WireHandle stream, BigInteger sequence) {
        WireValues.u64(sequence);
        if (!ownsStream(stream) || sequence.compareTo(consumedSequence) <= 0) return;
        consumedSequence = sequence;
        if (sequence.compareTo(lastSequence) > 0) lastSequence = sequence;
        retireConsumed(pending);
        for (Subscription subscription : subscriptions) {
            retireConsumed(subscription.queue);
            subscription.schedule();
        }
    }

    private void retireConsumed(ArrayDeque<Delivery> queue) {
        var retained = new ArrayDeque<Delivery>();
        for (Delivery item : queue) {
            if (item.gap()) {
                BigInteger watermark = new BigInteger(item.value().path("gap").path("snapshot_watermark").textValue());
                if (watermark.compareTo(consumedSequence) <= 0) continue;
                var value = item.value().deepCopy();
                var gap = (com.fasterxml.jackson.databind.node.ObjectNode) value.path("gap");
                BigInteger from = new BigInteger(gap.path("from_sequence").textValue());
                if (from.compareTo(consumedSequence) <= 0) {
                    BigInteger remainingFrom = consumedSequence.add(BigInteger.ONE);
                    BigInteger to = new BigInteger(gap.path("to_sequence").textValue());
                    gap.put("from_sequence", remainingFrom.toString());
                    gap.put("to_sequence", to.max(remainingFrom).toString());
                }
                retained.addLast(new Delivery(value, true));
            } else {
                JsonNode sequence = item.value().path("envelope").get("sequence");
                // The same publisher also supports ordinary ACP updates, which
                // have no event cursor and must never be retired by an event ACK.
                if (sequence == null || !sequence.isTextual()
                        || new BigInteger(sequence.textValue()).compareTo(consumedSequence) > 0) retained.addLast(item);
            }
        }
        queue.clear(); queue.addAll(retained);
    }

    synchronized void publish(JsonNode value) { publish(value, false); }

    private void publish(JsonNode value, boolean gap) {
        Objects.requireNonNull(value, "value");
        if (closed) return;
        Delivery delivery = new Delivery(value.deepCopy(), gap);
        if (subscriptions.isEmpty()) {
            if (!append(pending, delivery)) overflow();
            return;
        }
        for (Subscription subscription : java.util.List.copyOf(subscriptions)) {
            if (!append(subscription.queue, delivery)) { overflow(); return; }
            subscription.schedule();
        }
    }

    private boolean append(ArrayDeque<Delivery> queue, Delivery delivery) {
        if (delivery.gap()) {
            Delivery existing = null;
            for (Delivery item : queue) if (item.gap()) { existing = item; break; }
            if (existing != null) {
                // The merged gap explicitly covers every replaced undelivered
                // value. No covered value gains a consumption ACK. Keeping the
                // oldest start and newest watermark preserves the recovery range.
                var merged = delivery.value().deepCopy();
                ((com.fasterxml.jackson.databind.node.ObjectNode) merged.path("gap"))
                        .set("from_sequence", existing.value().path("gap").path("from_sequence").deepCopy());
                boolean covered = false;
                var iterator = queue.iterator();
                while (iterator.hasNext()) {
                    Delivery item = iterator.next();
                    if (item == existing) covered = true;
                    if (covered) iterator.remove();
                }
                queue.addLast(new Delivery(merged, true));
            } else queue.addLast(delivery);
            return true;
        }
        long buffered = queue.stream().filter(item -> !item.gap()).count();
        if (buffered >= capacity) return false;
        queue.addLast(delivery);
        return true;
    }

    private void overflow() {
        fail(new EchoAgentException("event_gap", "publisher data buffer is full; replay from the last acknowledged cursor",
                "after_delay", "_echo_agent/event", null));
    }

    /** Publish one full event envelope, enforcing stream identity and order. */
    synchronized void publishEvent(JsonNode value, String expectedStreamId) {
        if (closed) return;
        if (value == null || !value.isObject()) {
            fail(new EchoAgentException("serialization_violation", "event must be an object",
                    "never", "_echo_agent/event", null));
            return;
        }
        JsonNode stream = value.path("stream");
        JsonNode envelope = value.path("envelope");
        final WireHandle actualStream;
        try {
            actualStream = WireHandle.fromJson(stream);
        } catch (RuntimeException error) {
            fail(new EchoAgentException("serialization_violation", "event stream handle is malformed",
                    "never", "_echo_agent/event", value));
            return;
        }
        if (!"stream".equals(actualStream.kind())) {
            fail(new EchoAgentException("serialization_violation", "event stream handle kind is invalid",
                    "never", "_echo_agent/event", value));
            return;
        }
        if (!validateExpectedStream(actualStream)) return;
        String streamId = actualStream.id();
        String envelopeStreamId = envelope.path("stream_id").asText("");
        String sequenceText = envelope.path("sequence").isTextual()
                ? envelope.path("sequence").textValue() : "";
        if (streamId.isEmpty() || !streamId.equals(expectedStreamId)
                || !streamId.equals(envelopeStreamId)) {
            fail(new EchoAgentException("serialization_violation", "event stream identity does not match",
                    "never", "_echo_agent/event", value));
            return;
        }
        final BigInteger sequence;
        try {
            // WireValues performs canonical u64 validation; parse the text
            // directly because Jackson's TextNode#bigIntegerValue defaults
            // to zero instead of converting textual content.
            WireValues.u64(sequenceText);
            sequence = new BigInteger(sequenceText);
        } catch (RuntimeException error) {
            fail(new EchoAgentException("serialization_violation", "event sequence is not canonical u64",
                    "never", "_echo_agent/event", value));
            return;
        }
        if (sequence.signum() == 0) {
            fail(new EchoAgentException("serialization_violation", "event sequence must be positive",
                    "never", "_echo_agent/event", value));
            return;
        }
        if (sequence.compareTo(consumedSequence) <= 0) return; // Already consumed externally; Host retries cannot replay it locally.
        if (lastSequence.signum() != 0) {
            BigInteger expected = lastSequence.add(BigInteger.ONE);
            if (sequence.equals(lastSequence)) return; // duplicate delivery is idempotent
            if (!sequence.equals(expected) && (sequence.compareTo(lastSequence) <= 0
                    || pendingAcknowledgements.values().stream()
                    .noneMatch(predecessor -> sequence.equals(predecessor.add(BigInteger.ONE))))) {
                fail(new EchoAgentException("event_gap", "event sequence is not contiguous; replay is required",
                        "after_delay", "_echo_agent/event", value));
                return;
            }
        }
        lastSequence = sequence;
        if(!liveRanges.isEmpty() && liveRanges.get(liveRanges.size()-1).end().add(BigInteger.ONE).equals(sequence)) {
            var previous=liveRanges.remove(liveRanges.size()-1);liveRanges.add(new LiveRange(previous.base(),sequence));
        } else liveRanges.add(new LiveRange(sequence,sequence));
        if(liveRanges.size()>512)liveRanges.remove(0);
        publish(value);
    }

    /** Publish a typed gap and move the contiguous cursor to its watermark. */
    synchronized void publishGap(JsonNode value, String expectedStreamId) {
        if (closed) return;
        if (value == null || !value.isObject()) {
            fail(new EchoAgentException("serialization_violation", "gap must be an object",
                    "never", "_echo_agent/gap", null));
            return;
        }
        final WireHandle actualStream;
        try {
            actualStream = WireHandle.fromJson(value.path("stream"));
        } catch (RuntimeException error) {
            fail(new EchoAgentException("serialization_violation", "gap stream handle is malformed",
                    "never", "_echo_agent/gap", value));
            return;
        }
        if (!"stream".equals(actualStream.kind())) {
            fail(new EchoAgentException("serialization_violation", "gap stream handle kind is invalid",
                    "never", "_echo_agent/gap", value));
            return;
        }
        if (!validateExpectedStream(actualStream)) return;
        String streamId = actualStream.id();
        JsonNode gap = value.path("gap");
        String from = gap.path("from_sequence").isTextual()
                ? gap.path("from_sequence").textValue() : "";
        String to = gap.path("to_sequence").isTextual()
                ? gap.path("to_sequence").textValue() : "";
        String watermark = gap.path("snapshot_watermark").isTextual()
                ? gap.path("snapshot_watermark").textValue() : "";
        String reason = gap.path("reason").asText("");
        if (!streamId.equals(expectedStreamId) || reason.isBlank()
                || !from.matches("[1-9][0-9]*") || !to.matches("[1-9][0-9]*")
                || !watermark.matches("[1-9][0-9]*")) {
            fail(new EchoAgentException("serialization_violation", "gap identity or range is malformed",
                    "never", "_echo_agent/gap", value));
            return;
        }
        try {
            WireValues.u64(from);
            WireValues.u64(to);
            WireValues.u64(watermark);
            BigInteger fromValue = new BigInteger(from);
            BigInteger toValue = new BigInteger(to);
            BigInteger watermarkValue = new BigInteger(watermark);
            if (toValue.compareTo(fromValue) < 0
                    || watermarkValue.compareTo(toValue) < 0) {
                fail(new EchoAgentException("serialization_violation", "gap sequence range is malformed",
                        "never", "_echo_agent/gap", value));
                return;
            }
            if (watermarkValue.compareTo(consumedSequence) <= 0) return;
            if (watermarkValue.compareTo(lastSequence) < 0
                    || (watermarkValue.compareTo(lastSequence) > 0
                        && lastSequence.signum() > 0
                        && !fromValue.equals(lastSequence.add(BigInteger.ONE))
                        // The peer can continue before ACK write completion;
                        // only a validated pending predecessor permits this gap.
                        && pendingAcknowledgements.values().stream()
                            .noneMatch(predecessor -> fromValue.compareTo(lastSequence) > 0
                                && fromValue.equals(predecessor.add(BigInteger.ONE))))) {
                fail(new EchoAgentException("serialization_violation", "gap sequence range is malformed",
                        "never", "_echo_agent/gap", value));
                return;
            }
            if (watermarkValue.equals(lastSequence)) return;
            lastSequence = watermarkValue;
            lastGapWatermark = watermarkValue;
        } catch (RuntimeException error) {
            fail(new EchoAgentException("serialization_violation", "gap watermark is not canonical u64",
                    "never", "_echo_agent/gap", value));
            return;
        }
        publish(value, true);
    }

    synchronized void fail(Throwable error) {
        if (closed) return;
        closed = true;
        failure = Objects.requireNonNull(error, "error");
        pending.clear();
        for (Subscription subscription : subscriptions) { subscription.queue.clear(); subscription.schedule(); }
    }

    synchronized boolean isClosed() {
        return closed;
    }

    synchronized BigInteger lastSequence() {
        return lastSequence;
    }

    int capacity() {
        return capacity;
    }

    @Override
    public synchronized void close() {
        if (closed) return;
        closed = true;
        for (Subscription subscription : subscriptions) subscription.schedule();
    }
}
