package com.echoagent.sdk;

import com.agentclientprotocol.sdk.json.AcpJsonMapper;
import com.agentclientprotocol.sdk.json.TypeRef;
import com.agentclientprotocol.sdk.spec.AcpClientSession;
import com.agentclientprotocol.sdk.spec.AcpClientTransport;
import com.agentclientprotocol.sdk.spec.AcpSchema;
import org.junit.jupiter.api.Test;
import reactor.core.publisher.Mono;
import java.nio.file.Path;
import java.time.Duration;
import java.util.List;
import java.util.Map;
import java.util.concurrent.*;
import java.util.concurrent.atomic.*;
import java.util.function.Function;
import static org.junit.jupiter.api.Assertions.*;

/** Uses the official ACP interface/session with a controlled native-owner close acknowledgement. */
class JavaClientCloseLifecycleTest {
    private static final class NativeOwnerTransport implements AcpClientTransport {
        final AtomicInteger connects = new AtomicInteger();
        final AtomicInteger messages = new AtomicInteger();
        final AtomicInteger closeAttempts = new AtomicInteger();
        final AtomicBoolean stopped = new AtomicBoolean();
        final CountDownLatch firstCloseEntered = new CountDownLatch(1);
        final CountDownLatch secondCloseEntered = new CountDownLatch(1);
        final CompletableFuture<Void> first = new CompletableFuture<>();
        final CompletableFuture<Void> second = new CompletableFuture<>();
        private final AcpJsonMapper mapper = AcpJsonMapper.createDefault();
        @Override public Mono<Void> connect(Function<Mono<AcpSchema.JSONRPCMessage>, Mono<AcpSchema.JSONRPCMessage>> handler) {
            connects.incrementAndGet(); return Mono.empty();
        }
        @Override public Mono<Void> sendMessage(AcpSchema.JSONRPCMessage message) { messages.incrementAndGet(); return Mono.empty(); }
        @Override public <T> T unmarshalFrom(Object value, TypeRef<T> type) { return mapper.convertValue(value, type); }
        @Override public Mono<Void> closeGracefully() {
            return Mono.defer(() -> {
                int attempt = closeAttempts.incrementAndGet();
                if (attempt == 1) firstCloseEntered.countDown();
                else if (attempt == 2) secondCloseEntered.countDown();
                else return Mono.error(new IllegalStateException("unexpected native close attempt " + attempt));
                return Mono.fromFuture(attempt == 1 ? first : second, true).doOnSuccess(ignored -> stopped.set(true));
            });
        }
    }
    private static EchoAgentClient client(NativeOwnerTransport transport) throws Exception {
        var session = new AcpClientSession(Duration.ofSeconds(1), transport, Map.of(), Map.of(), Function.identity());
        var catalog = new FacadeCatalog(Path.of("../shared/facade-operation-catalog.json"), Path.of("../shared/contract-digests.json"));
        var constructor = EchoAgentClient.class.getDeclaredConstructor(AcpClientSession.class, FacadeCatalog.class, AcpClientTransport.class);
        constructor.setAccessible(true); return constructor.newInstance(session, catalog, transport);
    }
    private static boolean flag(EchoAgentClient sdk, String name) throws Exception {
        var field = EchoAgentClient.class.getDeclaredField(name); field.setAccessible(true); return field.getBoolean(sdk);
    }
    private static void admissionClosed(EchoAgentClient sdk) {
        var failure = assertThrows(ExecutionException.class, () -> sdk.request("test/request", JsonSupport.MAPPER.createObjectNode()).toCompletableFuture().get(1, TimeUnit.SECONDS));
        assertEquals("host_exited", EchoAgentException.from(failure, "test/request").code());
    }
    private record NativeCloseChoice(CompletableFuture<Void> future, java.util.function.Consumer<Runnable> select) {}
    private static NativeCloseChoice nativeCloseChoice() throws Exception {
        var type=Class.forName("com.echoagent.sdk.EchoAgentClient$NativeCloseCompletion");
        var constructor=type.getDeclaredConstructor();constructor.setAccessible(true);var owner=constructor.newInstance();
        var result=type.getDeclaredField("future");result.setAccessible(true);
        @SuppressWarnings("unchecked") var future=(CompletableFuture<Void>)result.get(owner);
        var select=type.getDeclaredMethod("select",Runnable.class);select.setAccessible(true);
        return new NativeCloseChoice(future, action->{try{select.invoke(owner,action);}catch(java.lang.reflect.InvocationTargetException error){throw new CompletionException(error.getCause());}catch(ReflectiveOperationException error){throw new CompletionException(error);}});
    }
    @Test void selectedNativeFailureCannotBeOvertakenWhileItsPublicationIsHeld() throws Exception {
        var close=nativeCloseChoice();var selected=new CountDownLatch(1);var release=new CountDownLatch(1);var successEffects=new AtomicInteger();
        var originalError=new EchoAgentException("timeout","Host transport did not exit within the five-second close deadline","after_delay","client/close",null);
        var owner=CompletableFuture.runAsync(()->close.select().accept(()->{
            selected.countDown();try{assertTrue(release.await(2,TimeUnit.SECONDS));}catch(InterruptedException interrupted){Thread.currentThread().interrupt();throw new CompletionException(interrupted);}
            close.future().completeExceptionally(originalError);
        }));
        try {
            assertTrue(selected.await(2,TimeUnit.SECONDS));close.select().accept(()->{successEffects.incrementAndGet();close.future().complete(null);});
            assertFalse(close.future().isDone(),"a selected native failure still owns publication while its original work is held");assertEquals(0,successEffects.get());
            release.countDown();owner.get(2,TimeUnit.SECONDS);var failure=assertThrows(ExecutionException.class,()->close.future().get(2,TimeUnit.SECONDS));assertSame(originalError,failure.getCause());
        } finally {release.countDown();owner.get(2,TimeUnit.SECONDS);}
    }
    @Test void successfulNativeCloseChoicePreventsLateDeadlineEffects() throws Exception {
        var close=nativeCloseChoice();var deadlineEffects=new AtomicInteger();
        close.select().accept(()->close.future().complete(null));close.select().accept(()->{deadlineEffects.incrementAndGet();close.future().completeExceptionally(new IllegalStateException("late deadline"));});
        close.future().get(2,TimeUnit.SECONDS);assertEquals(0,deadlineEffects.get(),"a completed native close cannot later force its child or replace its result");
    }
    @Test void sharedCloseWaitsForOfficialTransportOwnerAcknowledgement() throws Exception {
        var transport = new NativeOwnerTransport(); var sdk = client(transport);
        try {
            var first = sdk.closeAsync(); var simultaneous = sdk.closeAsync(); assertNotSame(first, simultaneous);
            assertTrue(transport.firstCloseEntered.await(2, TimeUnit.SECONDS));
            assertEquals(1, transport.connects.get()); assertEquals(1, transport.closeAttempts.get());
            assertFalse(first.toCompletableFuture().isDone(), "ACP notification drain does not prove native owner termination");
            assertFalse(transport.stopped.get()); assertFalse(flag(sdk, "closed")); assertTrue(flag(sdk, "closing"));
            admissionClosed(sdk); assertEquals(0, transport.messages.get(), "closing admission must not send work to the native owner");
            transport.first.complete(null); first.toCompletableFuture().get(2, TimeUnit.SECONDS); simultaneous.toCompletableFuture().get(2, TimeUnit.SECONDS);
            assertTrue(transport.stopped.get()); assertTrue(flag(sdk, "closed")); assertFalse(flag(sdk, "closing"));
            sdk.closeAsync().toCompletableFuture().get(1, TimeUnit.SECONDS); assertEquals(1, transport.closeAttempts.get());
        } finally { transport.first.complete(null); transport.second.complete(null); sdk.closeAsync().toCompletableFuture().get(2, TimeUnit.SECONDS); }
    }
    @Test void failedNativeCloseIsObservableAndRetrySharesItsOwnInFlightStage() throws Exception {
        var transport = new NativeOwnerTransport(); var sdk = client(transport);
        try {
            var failed = sdk.closeAsync(); var otherFailed = sdk.closeAsync();
            assertTrue(transport.firstCloseEntered.await(2, TimeUnit.SECONDS));
            var nativeFailure = new IllegalStateException("native owner refused stop 中文🙂"); transport.first.completeExceptionally(nativeFailure);
            var error = assertThrows(ExecutionException.class, () -> failed.toCompletableFuture().get(2, TimeUnit.SECONDS));
            assertSame(nativeFailure, error.getCause()); var simultaneousFailure = assertThrows(ExecutionException.class, () -> otherFailed.toCompletableFuture().get(2, TimeUnit.SECONDS)); assertSame(nativeFailure, simultaneousFailure.getCause()); assertFalse(transport.stopped.get()); assertFalse(flag(sdk, "closed"));
            assertTrue(flag(sdk, "closing")); admissionClosed(sdk);
            var retry = sdk.closeAsync(); var joinedRetry = sdk.closeAsync(); assertNotSame(failed, retry);
            assertTrue(transport.secondCloseEntered.await(2, TimeUnit.SECONDS)); assertEquals(2, transport.closeAttempts.get());
            assertFalse(retry.toCompletableFuture().isDone()); assertFalse(flag(sdk, "closed"));
            transport.second.complete(null); retry.toCompletableFuture().get(2, TimeUnit.SECONDS); joinedRetry.toCompletableFuture().get(2, TimeUnit.SECONDS);
            assertTrue(transport.stopped.get()); assertTrue(flag(sdk, "closed")); assertFalse(flag(sdk, "closing"));
            assertEquals(0, transport.messages.get()); sdk.closeAsync().toCompletableFuture().get(1, TimeUnit.SECONDS);
            assertEquals(2, transport.closeAttempts.get(), "successful close must not issue another native shutdown");
        } finally { transport.first.complete(null); transport.second.complete(null); sdk.closeAsync().toCompletableFuture().get(2, TimeUnit.SECONDS); }
    }
    @Test void cancelledWaiterLeavesOtherWaitersAwaitingOneActualNativeCloseOwner()throws Exception{
        var transport=new NativeOwnerTransport();var sdk=client(transport);
        try {
            var one=sdk.closeAsync().toCompletableFuture();var two=sdk.closeAsync().toCompletableFuture();assertTrue(transport.firstCloseEntered.await(2,TimeUnit.SECONDS));
            assertTrue(one.cancel(true));assertFalse(two.isDone());assertFalse(transport.first.isCancelled());assertFalse(flag(sdk,"closed"));assertEquals(1,transport.closeAttempts.get());
            var later=sdk.closeAsync().toCompletableFuture();assertFalse(later.isDone());assertEquals(1,transport.closeAttempts.get());
            transport.first.complete(null);two.get(2,TimeUnit.SECONDS);later.get(2,TimeUnit.SECONDS);assertTrue(flag(sdk,"closed"));assertEquals(1,transport.closeAttempts.get());sdk.closeAsync().toCompletableFuture().get(1,TimeUnit.SECONDS);assertEquals(1,transport.closeAttempts.get());
        }finally{transport.first.complete(null);transport.second.complete(null);sdk.closeAsync().toCompletableFuture().get(2,TimeUnit.SECONDS);}
    }
}
