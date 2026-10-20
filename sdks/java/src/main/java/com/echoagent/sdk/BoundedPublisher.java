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
    private record AckProjection(BigInteger cursor, java.util.concurrent.CompletableFuture<Void> completion,
            reactor.core.publisher.Mono<reactor.core.publisher.Signal<Void>> observed) {}
    private final java.util.Map<Object, AckProjection> pendingAcknowledgements = new java.util.IdentityHashMap<>();
    private java.util.concurrent.CompletableFuture<Void> admissionChanged = new java.util.concurrent.CompletableFuture<>();
    private int mutationDepth;
    private boolean closed;

    private long dataCount(ArrayDeque<Delivery> queue) {
        return queue.stream().filter(item -> !item.gap()).count();
    }

    private java.util.List<Object> admissionState() {
        var state = new java.util.ArrayList<Object>();
        state.add(closed); state.add(failure); state.add(dataCount(pending));
        for (var subscription : subscriptions) {
            state.add(subscription); state.add(dataCount(subscription.queue));
        }
        return state;
    }

    /** Nested validate/fail/publish calls notify only after the OUTERMOST lock exits. */
    private <T> T mutate(java.util.function.Supplier<T> operation) {
        java.util.concurrent.CompletableFuture<Void> changed = null;
        try {
            synchronized (this) {
                boolean outer = mutationDepth++ == 0;
                var before = outer ? admissionState() : null;
                try { return operation.get(); }
                finally {
                    mutationDepth--;
                    if (outer && !before.equals(admissionState())) {
                        changed = admissionChanged;
                        admissionChanged = new java.util.concurrent.CompletableFuture<>();
                    }
                }
            }
        } finally {
            if (changed != null) changed.complete(null);
        }
    }

    private void mutate(Runnable operation) {
        mutate(() -> { operation.run(); return null; });
    }

    private boolean dataCapacityAvailable() {
        if (subscriptions.isEmpty()) return dataCount(pending) < capacity;
        return subscriptions.stream().allMatch(sub -> dataCount(sub.queue) < capacity);
    }

    private boolean freesFullQueues(BigInteger cursor) {
        java.util.function.Predicate<ArrayDeque<Delivery>> frees = queue -> dataCount(queue) < capacity
                || queue.stream().anyMatch(item -> !item.gap()
                    && item.value().path("envelope").path("sequence").isTextual()
                    && new BigInteger(item.value().path("envelope").path("sequence").textValue()).compareTo(cursor) <= 0);
        return subscriptions.isEmpty() ? frees.test(pending)
                : subscriptions.stream().allMatch(sub -> frees.test(sub.queue));
    }

    reactor.core.publisher.Mono<Void> beforeEventAdmission(WireHandle stream, BigInteger sequence) {
        final java.util.List<AckProjection> candidates;
        synchronized (this) {
            if (closed || !ownsStream(stream) || sequence.compareTo(consumedSequence) <= 0
                    || sequence.compareTo(lastSequence) <= 0 || dataCapacityAvailable()) return reactor.core.publisher.Mono.empty();
            candidates = pendingAcknowledgements.values().stream()
                    .filter(ack -> sequence.equals(ack.cursor().add(BigInteger.ONE)) && freesFullQueues(ack.cursor()))
                    .toList();
        }
        return awaitAdmission(stream, candidates);
    }

    private reactor.core.publisher.Mono<Void> awaitAdmission(WireHandle stream, java.util.List<AckProjection> candidates) {
        return reactor.core.publisher.Mono.defer(() -> {
            final java.util.List<reactor.core.publisher.Mono<Void>> changes = new java.util.ArrayList<>();
            synchronized (this) {
                if (closed || !ownsStream(stream) || dataCapacityAvailable()) return reactor.core.publisher.Mono.empty();
                var pending = candidates.stream().filter(ack -> !ack.completion().isDone()).toList();
                if (pending.isEmpty()) return reactor.core.publisher.Mono.empty();
                // Capture this exact stage in the same monitor as the state check.
                changes.add(reactor.core.publisher.Mono.fromFuture(admissionChanged, true));
                pending.forEach(ack -> changes.add(ack.observed().then()));
            }
            return reactor.core.publisher.Mono.firstWithSignal(changes)
                    .then(reactor.core.publisher.Mono.defer(() -> awaitAdmission(stream, candidates)));
        });
    }

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

    boolean validateExpectedStream(WireHandle stream) {
        return mutate(() -> {
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
        });
    }

    @Override
    public void subscribe(Flow.Subscriber<? super JsonNode> subscriber) {
        Objects.requireNonNull(subscriber, "subscriber");
        var subscription = new Subscription(subscriber);
        mutate(() -> {
            subscription.queue.addAll(pending);
            pending.clear();
            subscriptions.add(subscription);
        });
        try { subscriber.onSubscribe(subscription); }
        catch (Throwable error) { subscription.cancel(); return; }
        mutate(() -> { subscription.started = true; subscription.schedule(); });
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
            mutate(() -> {
                if (cancelled) return;
                if (count <= 0) { requestFailure = new IllegalArgumentException("subscriber demand must be positive"); queue.clear(); }
                else demand = count > Long.MAX_VALUE - demand ? Long.MAX_VALUE : demand + count;
                schedule();
            });
        }
        @Override public void cancel() {
            mutate(() -> { cancelled = true; queue.clear(); subscriptions.remove(this); });
        }
        private void schedule() {
            if (!started || cancelled || draining) return;
            draining = true;
            ForkJoinPool.commonPool().execute(this::drain);
        }
        private void drain() {
            while (true) {
                record Step(Delivery next, Throwable error, boolean complete, boolean stopped) {}
                Step step = mutate(() -> {
                    if (cancelled) { draining = false; return new Step(null, null, false, true); }
                    Throwable error = requestFailure != null ? requestFailure : failure;
                    boolean complete = error == null && closed && queue.isEmpty();
                    if (error != null || complete) {
                        cancelled = true; queue.clear(); subscriptions.remove(this); draining = false;
                        return new Step(null, error, complete, false);
                    } else if (demand > 0 && !queue.isEmpty()) {
                        Delivery next = queue.removeFirst();
                        if (demand != Long.MAX_VALUE) demand--;
                        return new Step(next, null, false, false);
                    }
                    draining = false; return new Step(null, null, false, true);
                });
                if (step.stopped()) return;
                Delivery next = step.next(); Throwable error = step.error(); boolean complete = step.complete();
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

    void acknowledgeReplay(WireHandle stream, BigInteger sequence) {
        mutate(() -> {
            if(validateExpectedStream(stream) && sequence.compareTo(lastSequence)>0)lastSequence=sequence;
        });
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
        beginAcknowledgement(submission, sequence, java.util.concurrent.CompletableFuture.completedFuture(null));
    }

    synchronized void beginAcknowledgement(Object submission, BigInteger sequence,
            java.util.concurrent.CompletableFuture<Void> projection) {
        pendingAcknowledgements.put(submission, new AckProjection(sequence, projection,
                reactor.core.publisher.Mono.fromFuture(projection, true).materialize().cache()));
    }

    synchronized void endAcknowledgement(Object submission) {
        pendingAcknowledgements.remove(submission);
    }

    /** Called only after the client proves live/gap/replay consumption coverage. */
    void acknowledgeConsumedCursor(WireHandle stream, BigInteger sequence) {
        mutate(() -> {
            WireValues.u64(sequence);
            if (!ownsStream(stream) || sequence.compareTo(consumedSequence) <= 0) return;
            consumedSequence = sequence;
            if (sequence.compareTo(lastSequence) > 0) lastSequence = sequence;
            retireConsumed(pending);
            for (Subscription subscription : subscriptions) {
                retireConsumed(subscription.queue);
                subscription.schedule();
            }
        });
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

    void publish(JsonNode value) {
        mutate(() -> {     publish(value, false);
        });
    }

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
    void publishEvent(JsonNode value, String expectedStreamId) {
        mutate(() -> {
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
                        .noneMatch(predecessor -> sequence.equals(predecessor.cursor().add(BigInteger.ONE))))) {
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
        });
    }

    /** Publish a typed gap and move the contiguous cursor to its watermark. */
    void publishGap(JsonNode value, String expectedStreamId) {
        mutate(() -> {
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
                                    && fromValue.equals(predecessor.cursor().add(BigInteger.ONE))))) {
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
        });
    }

    void fail(Throwable error) {
        mutate(() -> {
            if (closed) return;
            closed = true;
            failure = Objects.requireNonNull(error, "error");
            pending.clear();
            for (Subscription subscription : subscriptions) { subscription.queue.clear(); subscription.schedule(); }
        });
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
    public void close() {
        mutate(() -> {
            if (closed) return;
            closed = true;
            for (Subscription subscription : subscriptions) subscription.schedule();
        });
    }
}
