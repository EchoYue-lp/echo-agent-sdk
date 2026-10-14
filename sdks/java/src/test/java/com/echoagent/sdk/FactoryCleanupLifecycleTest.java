package com.echoagent.sdk;

import com.agentclientprotocol.sdk.json.AcpJsonMapper;
import com.agentclientprotocol.sdk.json.TypeRef;
import com.agentclientprotocol.sdk.spec.AcpClientSession;
import com.agentclientprotocol.sdk.spec.AcpClientTransport;
import com.agentclientprotocol.sdk.spec.AcpSchema;
import com.fasterxml.jackson.databind.JsonNode;
import org.junit.jupiter.api.Test;
import reactor.core.publisher.Mono;
import java.lang.reflect.*;
import java.nio.file.Path;
import java.time.Duration;
import java.util.*;
import java.util.concurrent.*;
import java.util.concurrent.atomic.*;
import java.util.function.Function;
import static org.junit.jupiter.api.Assertions.*;

/** Exercises the real reverse-call adapter with controlled user completion futures. No Run receipt is fabricated. */
class FactoryCleanupLifecycleTest {
    private static final class Transport implements AcpClientTransport {
        private final AcpJsonMapper mapper=AcpJsonMapper.createDefault();
        public Mono<Void> connect(Function<Mono<AcpSchema.JSONRPCMessage>,Mono<AcpSchema.JSONRPCMessage>> handler){return Mono.empty();}
        public Mono<Void> sendMessage(AcpSchema.JSONRPCMessage message){return Mono.empty();}
        public <T>T unmarshalFrom(Object value,TypeRef<T> type){return mapper.convertValue(value,type);}
        public Mono<Void> closeGracefully(){return Mono.empty();}
    }
    private record Fixture(EchoAgentClient client,WireHandle parent,WireHandle child){}
    @SuppressWarnings("unchecked") private static <K,V> Map<K,V> map(EchoAgentClient client,String name)throws Exception{
        var field=EchoAgentClient.class.getDeclaredField(name);field.setAccessible(true);return (Map<K,V>)field.get(client);
    }
    private static Fixture fixture(ExtensionHandler handler)throws Exception{
        var transport=new Transport();var session=new AcpClientSession(Duration.ofSeconds(1),transport,Map.of(),Map.of(),Function.identity());
        var catalog=new FacadeCatalog(Path.of("../shared/facade-operation-catalog.json"),Path.of("../shared/contract-digests.json"));
        var ctor=EchoAgentClient.class.getDeclaredConstructor(AcpClientSession.class,FacadeCatalog.class,AcpClientTransport.class);ctor.setAccessible(true);var client=ctor.newInstance(session,catalog,transport);
        var parent=new WireHandle("factory","1","extension");var child=new WireHandle("child","1","extension");
        Class<?> metadata=Class.forName("com.echoagent.sdk.EchoAgentClient$ExtensionRegistrationMetadata");var mc=metadata.getDeclaredConstructor(WireHandle.class,String.class);mc.setAccessible(true);
        map(client,"extensionRegistrations").put(parent.id(),mc.newInstance(parent,"agent_factory"));map(client,"extensionHandlers").put(parent.id(),handler);map(client,"factoryChildren").put(child,parent);
        return new Fixture(client,parent,child);
    }
    private static JsonNode call(Fixture fixture,String id,long nanos){
        var value=JsonSupport.MAPPER.createObjectNode().put("invocation_id",id);value.set("extension",fixture.child().toJson());value.set("factory_origin",fixture.parent().toJson());
        value.putObject("deadline").put("seconds","0").put("nanos",nanos);value.putObject("invocation").put("operation","agent_close").putObject("input");return value;
    }
    @SuppressWarnings("unchecked") private static Mono<JsonNode> invoke(Fixture fixture,String id,long nanos)throws Exception{
        var method=EchoAgentClient.class.getDeclaredMethod("invokeExtension",JsonNode.class);method.setAccessible(true);return (Mono<JsonNode>)method.invoke(fixture.client(),call(fixture,id,nanos));
    }
    private static JsonNode success(){var value=JsonSupport.MAPPER.createObjectNode().put("outcome","result");value.putObject("result").put("operation","agent_close").putNull("value");return value;}
    private static JsonNode get(CompletionStage<JsonNode> value)throws Exception{return value.toCompletableFuture().get(2,TimeUnit.SECONDS);}
    @Test void twoWaitingClosesShareOneActualConstructionAndCleanup()throws Exception{
        var calls=new AtomicInteger();var cleanup=new CompletableFuture<JsonNode>();var fixture=fixture((call,cancel)->{calls.incrementAndGet();return cleanup;});var construction=new CompletableFuture<Void>();map(fixture.client(),"pendingFactoryConstruction").put(fixture.child(),construction);
        var first=invoke(fixture,"one",500_000_000).toFuture();var second=invoke(fixture,"two",500_000_000).toFuture();assertEquals(0,calls.get());construction.complete(null);assertEquals(1,calls.get());assertFalse(cleanup.isCancelled());assertFalse(first.isDone());assertFalse(second.isDone());cleanup.complete(success());
        assertEquals(success(),get(first));assertEquals(success(),get(second));assertEquals(1,calls.get());assertFalse(map(fixture.client(),"factoryChildren").containsKey(fixture.child()));assertEquals(fixture.parent(),map(fixture.client(),"closedFactoryChildren").get(fixture.child()));assertEquals(success(),get(invoke(fixture,"duplicate",500_000_000).toFuture()));assertEquals(1,calls.get());fixture.client().close();
    }
    @Test void timeoutAndCancelledSubscriptionCannotCancelActualCleanup()throws Exception{
        var calls=new AtomicInteger();var cleanup=new CompletableFuture<JsonNode>();var fixture=fixture((call,cancel)->{calls.incrementAndGet();return cleanup;});
        var timed=invoke(fixture,"timed",1_000_000).toFuture();assertEquals("extension_timeout",get(timed).path("error").path("code").textValue());assertFalse(cleanup.isCancelled());
        var dropped=invoke(fixture,"dropped",500_000_000).toFuture();dropped.cancel(true);assertFalse(cleanup.isCancelled());
        var retry=invoke(fixture,"retry",500_000_000).toFuture();assertEquals(1,calls.get());assertFalse(retry.isDone());cleanup.complete(success());assertEquals(success(),get(retry));assertEquals(success(),get(invoke(fixture,"duplicate",500_000_000).toFuture()));assertEquals(1,calls.get());fixture.client().close();
    }
    @Test void invocationCancellationDoesNotCancelOwnerAndLateSuccessStillTombstones()throws Exception{
        var cleanup=new CompletableFuture<JsonNode>();var cancellations=new ArrayList<ExtensionCancellation>();var fixture=fixture((call,cancel)->{cancellations.add(cancel);return cleanup;});
        var cancelled=invoke(fixture,"cancel",500_000_000).toFuture();((ExtensionCancellation)map(fixture.client(),"invocationCancellations").get("cancel")).cancel();var retry=invoke(fixture,"retry",500_000_000).toFuture();assertEquals(1,cancellations.size());assertFalse(cancellations.get(0).isCancelled());cleanup.complete(success());
        assertEquals("cancelled",get(cancelled).path("error").path("code").textValue());assertEquals(success(),get(retry));assertEquals(success(),get(invoke(fixture,"later",500_000_000).toFuture()));fixture.client().close();
    }
    @Test void actualFailureRetainsLiveChildAndAllowsNewSharedAttempt()throws Exception{
        var actual=new ArrayList<CompletableFuture<JsonNode>>();var fixture=fixture((call,cancel)->{var job=new CompletableFuture<JsonNode>();actual.add(job);return job;});
        var first=invoke(fixture,"first",500_000_000).toFuture();var joined=invoke(fixture,"joined",500_000_000).toFuture();assertEquals(1,actual.size());actual.get(0).completeExceptionally(new IllegalStateException("actual native close fault"));assertEquals("extension_failed",get(first).path("error").path("code").textValue());assertEquals(get(first),get(joined));assertTrue(map(fixture.client(),"factoryChildren").containsKey(fixture.child()));assertFalse(map(fixture.client(),"closedFactoryChildren").containsKey(fixture.child()));
        var retry=invoke(fixture,"retry",500_000_000).toFuture();var joinedRetry=invoke(fixture,"joinedRetry",500_000_000).toFuture();assertEquals(2,actual.size());actual.get(1).complete(success());assertEquals(success(),get(retry));assertEquals(success(),get(joinedRetry));fixture.client().close();
    }
    @Test void invalidSuccessOrExplicitFailureNeverReleasesChild()throws Exception{
        for(boolean malformed:List.of(false,true)){
            var calls=new AtomicInteger();var fixture=fixture((call,cancel)->{if(calls.incrementAndGet()==1){var value=malformed?success():ExtensionOutcome.error("extension_failed","retry","after_delay").toJson();if(malformed)((com.fasterxml.jackson.databind.node.ObjectNode)value.path("result")).put("value","invalid");return CompletableFuture.completedFuture(value);}return CompletableFuture.completedFuture(success());});
            var failure=get(invoke(fixture,"first",500_000_000).toFuture());assertEquals(malformed?"serialization_violation":"extension_failed",failure.path("error").path("code").textValue());assertTrue(map(fixture.client(),"factoryChildren").containsKey(fixture.child()));assertEquals(success(),get(invoke(fixture,"retry",500_000_000).toFuture()));assertEquals(2,calls.get());fixture.client().close();
        }
    }
    @Test void independentChildrenHaveIndependentCleanupCollectors()throws Exception{
        var jobs=new ConcurrentHashMap<WireHandle,CompletableFuture<JsonNode>>();var fixture=fixture((call,cancel)->jobs.computeIfAbsent(WireHandle.fromJson(call.path("extension")),ignored->new CompletableFuture<>()));var other=new WireHandle("other","1","extension");map(fixture.client(),"factoryChildren").put(other,fixture.parent());var otherFixture=new Fixture(fixture.client(),fixture.parent(),other);
        var one=invoke(fixture,"one",500_000_000).toFuture();var two=invoke(otherFixture,"two",500_000_000).toFuture();assertEquals(2,jobs.size());jobs.get(other).complete(success());assertEquals(success(),get(two));assertFalse(one.isDone());jobs.get(fixture.child()).complete(success());assertEquals(success(),get(one));fixture.client().close();
    }
    @Test void connectionCloseSignalsAndAwaitsTheActualCleanupResult()throws Exception{
        var actual=new CompletableFuture<JsonNode>();var token=new AtomicReference<ExtensionCancellation>();var attempts=new AtomicInteger();
        var fixture=fixture((call,cancel)->{token.set(cancel);return attempts.incrementAndGet()==1?actual:CompletableFuture.completedFuture(success());});var waiting=invoke(fixture,"wait",500_000_000).toFuture();
        var closing=fixture.client().closeAsync().toCompletableFuture();assertTrue(token.get().isCancelled());assertFalse(actual.isCancelled());assertFalse(closing.isDone());
        actual.completeExceptionally(new CancellationException("native cleanup confirmed cancellation"));assertThrows(ExecutionException.class,()->closing.get(2,TimeUnit.SECONDS));
        assertEquals("cancelled",get(waiting).path("error").path("code").textValue());assertTrue(map(fixture.client(),"factoryCleanupJobs").isEmpty());assertTrue(map(fixture.client(),"factoryCleanupExecutions").isEmpty());assertFalse(flag(fixture.client(),"closed"));
        fixture.client().closeAsync().toCompletableFuture().get(2,TimeUnit.SECONDS);assertTrue(flag(fixture.client(),"closed"));assertEquals(2,attempts.get());
    }

    @Test void nativeMinimalStageIsNotSettledByCancellingItsDependentCompletableFuture()throws Exception{
        var nativeCleanup=new CompletableFuture<JsonNode>();var token=new AtomicReference<ExtensionCancellation>();
        var fixture=fixture((call,cancellation)->{token.set(cancellation);return nativeCleanup.minimalCompletionStage();});
        invoke(fixture,"wait",500_000_000).toFuture();
        var closing=fixture.client().closeAsync().toCompletableFuture();
        try {
            assertTrue(token.get().isCancelled(),"connection close must signal the actual cleanup owner");
            assertFalse(nativeCleanup.isDone(),"native stage still running");
            assertFalse(closing.isDone(),"dependent cancellation cannot prove native completion");
            nativeCleanup.complete(success());closing.get(2,TimeUnit.SECONDS);
        } finally {nativeCleanup.complete(success());fixture.client().closeAsync().toCompletableFuture().get(2,TimeUnit.SECONDS);}
    }
    private static boolean flag(EchoAgentClient client,String name)throws Exception{
        var field=EchoAgentClient.class.getDeclaredField(name);field.setAccessible(true);return field.getBoolean(client);
    }
    @Test void nativeMinimalStageDeadlineFailureRetainsOwnerForActualCompletionAndRetry()throws Exception{
        var nativeCleanup=new CompletableFuture<JsonNode>();var token=new AtomicReference<ExtensionCancellation>();var calls=new AtomicInteger();
        var fixture=fixture((call,cancel)->{calls.incrementAndGet();token.set(cancel);return nativeCleanup.minimalCompletionStage();});invoke(fixture,"wait",500_000_000).toFuture();
        var first=fixture.client().closeAsync().toCompletableFuture();
        var error=assertThrows(ExecutionException.class,()->first.get(7,TimeUnit.SECONDS));assertEquals("extension_timeout",EchoAgentException.from(error,"client/close").code());
        assertFalse(flag(fixture.client(),"closed"));assertTrue(flag(fixture.client(),"closing"));assertTrue(token.get().isCancelled());assertFalse(nativeCleanup.isDone());
        assertEquals(1,map(fixture.client(),"factoryCleanupJobs").size());assertEquals(1,map(fixture.client(),"factoryCleanupExecutions").size());assertTrue(map(fixture.client(),"factoryChildren").containsKey(fixture.child()));
        var retry=fixture.client().closeAsync().toCompletableFuture();assertFalse(retry.isDone());nativeCleanup.complete(success());retry.get(2,TimeUnit.SECONDS);assertTrue(flag(fixture.client(),"closed"));assertEquals(1,calls.get());
    }
    @Test void cancelledCloseWaiterCannotCompleteOrCancelNativeMinimalStage()throws Exception{
        var nativeCleanup=new CompletableFuture<JsonNode>();var fixture=fixture((call,cancel)->nativeCleanup.minimalCompletionStage());invoke(fixture,"wait",500_000_000).toFuture();
        var first=fixture.client().closeAsync().toCompletableFuture();first.cancel(true);assertFalse(nativeCleanup.isDone());assertFalse(flag(fixture.client(),"closed"));
        nativeCleanup.complete(success());
        long until=System.nanoTime()+TimeUnit.SECONDS.toNanos(2);while(!flag(fixture.client(),"closed")){assertTrue(System.nanoTime()<until);Thread.onSpinWait();}
        fixture.client().closeAsync().toCompletableFuture().get(2,TimeUnit.SECONDS);
    }
    @Test void malformedCleanupOutcomesExplicitlySettleEveryWaiterAndRetainRetry()throws Exception{
        List<JsonNode> invalid=new ArrayList<>();invalid.add(null);invalid.add(JsonSupport.MAPPER.createObjectNode().put("outcome","stream").putObject("stream"));
        invalid.add(JsonSupport.MAPPER.createObjectNode().put("outcome","result"));invalid.add(JsonSupport.MAPPER.createObjectNode().put("outcome","error").putObject("error"));
        for(JsonNode value:invalid){
            var nativeCleanup=new CompletableFuture<JsonNode>();var calls=new AtomicInteger();var fixture=fixture((call,cancel)->calls.incrementAndGet()==1?nativeCleanup.minimalCompletionStage():CompletableFuture.completedFuture(success()));
            var one=invoke(fixture,"one",500_000_000).toFuture();var two=invoke(fixture,"two",500_000_000).toFuture();nativeCleanup.complete(value);
            assertEquals("serialization_violation",get(one).path("error").path("code").textValue());assertEquals(get(one),get(two));assertTrue(map(fixture.client(),"factoryChildren").containsKey(fixture.child()));
            assertEquals(success(),get(invoke(fixture,"retry",500_000_000).toFuture()));assertEquals(2,calls.get());fixture.client().close();
        }
    }
    @Test void shutdownDuringHandlerAdmissionRetainsSameOriginalStageAndSignalsOwner()throws Exception{
        var nativeCleanup=new CompletableFuture<JsonNode>();var token=new AtomicReference<ExtensionCancellation>();var entered=new CountDownLatch(1);var release=new CountDownLatch(1);
        var fixture=fixture((call,cancel)->{token.set(cancel);entered.countDown();try{assertTrue(release.await(2,TimeUnit.SECONDS));}catch(InterruptedException error){Thread.currentThread().interrupt();return CompletableFuture.failedFuture(error);}return nativeCleanup.minimalCompletionStage();});
        var executor=Executors.newSingleThreadExecutor();
        try {
            var admission=executor.submit(()->invoke(fixture,"admission",500_000_000).toFuture());assertTrue(entered.await(2,TimeUnit.SECONDS));
            var closing=fixture.client().closeAsync().toCompletableFuture();assertTrue(token.get().isCancelled());assertFalse(closing.isDone());assertFalse(nativeCleanup.isDone());
            release.countDown();admission.get(2,TimeUnit.SECONDS);assertFalse(closing.isDone());nativeCleanup.complete(success());closing.get(2,TimeUnit.SECONDS);assertTrue(flag(fixture.client(),"closed"));
        } finally {release.countDown();nativeCleanup.complete(success());executor.shutdownNow();fixture.client().closeAsync().toCompletableFuture().get(2,TimeUnit.SECONDS);}
    }
    @Test void nullReturnedStageAndSynchronousUserFailureBothRetainFreshRetry()throws Exception{
        for(boolean nullStage:List.of(false,true)) {
            var calls=new AtomicInteger();var fixture=fixture((call,cancel)->{if(calls.incrementAndGet()==1){if(nullStage)return null;throw new IllegalStateException("native admission failure");}return CompletableFuture.completedFuture(success());});
            var first=get(invoke(fixture,"first",500_000_000).toFuture());assertEquals("extension_failed",first.path("error").path("code").textValue());assertTrue(map(fixture.client(),"factoryChildren").containsKey(fixture.child()));assertEquals(success(),get(invoke(fixture,"retry",500_000_000).toFuture()));assertEquals(2,calls.get());fixture.client().close();
        }
    }
    @Test void cancelledNativeCleanupWaiterLeavesNewCloseWaitingOnSameOriginalStage()throws Exception{
        var nativeCleanup=new CompletableFuture<JsonNode>();var calls=new AtomicInteger();var fixture=fixture((call,cancel)->{calls.incrementAndGet();return nativeCleanup.minimalCompletionStage();});invoke(fixture,"wait",500_000_000).toFuture();
        var dropped=fixture.client().closeAsync().toCompletableFuture();assertTrue(dropped.cancel(true));
        var second=fixture.client().closeAsync().toCompletableFuture();assertFalse(second.isDone());assertFalse(nativeCleanup.isDone());assertFalse(flag(fixture.client(),"closed"));assertEquals(1,calls.get());
        nativeCleanup.complete(success());second.get(2,TimeUnit.SECONDS);assertTrue(flag(fixture.client(),"closed"));assertEquals(1,calls.get());fixture.client().close();
    }
    @Test void actualNativeCleanupFailureDuringShutdownNeverReleasesTheResource()throws Exception{
        var nativeCleanup=new CompletableFuture<JsonNode>();var calls=new AtomicInteger();var fixture=fixture((call,cancel)->{if(calls.incrementAndGet()==1)return nativeCleanup.minimalCompletionStage();return CompletableFuture.completedFuture(success());});invoke(fixture,"wait",500_000_000).toFuture();
        var closing=fixture.client().closeAsync().toCompletableFuture();nativeCleanup.completeExceptionally(new java.io.IOException("native resource still live"));
        assertThrows(ExecutionException.class,()->closing.get(2,TimeUnit.SECONDS));assertFalse(flag(fixture.client(),"closed"));assertTrue(map(fixture.client(),"factoryChildren").containsKey(fixture.child()));
        fixture.client().closeAsync().toCompletableFuture().get(2,TimeUnit.SECONDS);assertTrue(flag(fixture.client(),"closed"));assertEquals(2,calls.get());
    }
    @Test void explicitFailedOutcomeDuringShutdownRetainsRetryAddress()throws Exception{
        var nativeCleanup=new CompletableFuture<JsonNode>();var calls=new AtomicInteger();var fixture=fixture((call,cancel)->calls.incrementAndGet()==1?nativeCleanup.minimalCompletionStage():CompletableFuture.completedFuture(success()));invoke(fixture,"wait",500_000_000).toFuture();
        var closing=fixture.client().closeAsync().toCompletableFuture();nativeCleanup.complete(ExtensionOutcome.error("extension_failed","resource still live","after_delay").toJson());
        assertThrows(ExecutionException.class,()->closing.get(2,TimeUnit.SECONDS));assertFalse(flag(fixture.client(),"closed"));assertTrue(map(fixture.client(),"factoryChildren").containsKey(fixture.child()));fixture.client().closeAsync().toCompletableFuture().get(2,TimeUnit.SECONDS);assertEquals(2,calls.get());
    }
    @Test void malformedOutcomeDuringShutdownReportsFailureAndRetriesOnlyFailedChild()throws Exception{
        var failedStage=new CompletableFuture<JsonNode>();var successStage=new CompletableFuture<JsonNode>();var attempts=new ConcurrentHashMap<WireHandle,AtomicInteger>();
        var fixture=fixture((call,cancel)->{var child=WireHandle.fromJson(call.path("extension"));int attempt=attempts.computeIfAbsent(child,ignored->new AtomicInteger()).incrementAndGet();if(attempt>1)return CompletableFuture.completedFuture(success());return child.id().equals("child")?failedStage.minimalCompletionStage():successStage.minimalCompletionStage();});
        var other=new WireHandle("other","1","extension");map(fixture.client(),"factoryChildren").put(other,fixture.parent());var otherFixture=new Fixture(fixture.client(),fixture.parent(),other);invoke(fixture,"bad",500_000_000).toFuture();invoke(otherFixture,"good",500_000_000).toFuture();
        var closing=fixture.client().closeAsync().toCompletableFuture();failedStage.complete(null);successStage.complete(success());
        var error=assertThrows(ExecutionException.class,()->closing.get(2,TimeUnit.SECONDS));assertEquals("serialization_violation",EchoAgentException.from(error,"client/close").code());assertFalse(flag(fixture.client(),"closed"));assertTrue(map(fixture.client(),"factoryChildren").containsKey(fixture.child()));assertFalse(map(fixture.client(),"factoryChildren").containsKey(other));assertFalse(map(fixture.client(),"extensionHandlers").isEmpty());
        fixture.client().closeAsync().toCompletableFuture().get(2,TimeUnit.SECONDS);assertEquals(2,attempts.get(fixture.child()).get());assertEquals(1,attempts.get(other).get());assertTrue(flag(fixture.client(),"closed"));
    }
    @Test void synchronousRepeatedShutdownFailureRetainsTheErrorAndTheSameRetryAddress()throws Exception{
        var attempts=new AtomicInteger();var fixture=fixture((call,cancel)->{if(attempts.incrementAndGet()<4)return CompletableFuture.failedFuture(new java.io.IOException("persistent native failure"));return CompletableFuture.completedFuture(success());});
        var initial=get(invoke(fixture,"initial",500_000_000).toFuture());assertEquals("extension_failed",initial.path("error").path("code").textValue());
        for(int count=0;count<2;count++){var error=assertThrows(ExecutionException.class,()->fixture.client().closeAsync().toCompletableFuture().get(2,TimeUnit.SECONDS));assertEquals("transport_error",EchoAgentException.from(error,"client/close").code());assertNotNull(error.getCause().getCause());assertFalse(flag(fixture.client(),"closed"));assertTrue(map(fixture.client(),"factoryChildren").containsKey(fixture.child()));}
        fixture.client().closeAsync().toCompletableFuture().get(2,TimeUnit.SECONDS);assertEquals(4,attempts.get());assertTrue(flag(fixture.client(),"closed"));
    }
    @SuppressWarnings("unchecked") private static Mono<JsonNode> construct(Fixture fixture)throws Exception{
        map(fixture.client(),"factoryChildren").remove(fixture.child());var call=JsonSupport.MAPPER.createObjectNode().put("invocation_id","construction");call.set("extension",fixture.parent().toJson());call.putObject("deadline").put("seconds","1").put("nanos",0);
        call.putObject("invocation").put("operation","factory_create_agent").putObject("input").set("instance",fixture.child().toJson());var method=EchoAgentClient.class.getDeclaredMethod("invokeExtension",JsonNode.class);method.setAccessible(true);return (Mono<JsonNode>)method.invoke(fixture.client(),call);
    }
    private static JsonNode constructed(){var value=JsonSupport.MAPPER.createObjectNode().put("outcome","result");value.putObject("result").put("operation","factory_create_agent").putObject("value").put("name","factory").put("model_name","model").put("system_prompt","prompt").putArray("tool_names");return value;}
    @Test void originalConstructorMinimalStageMustCompleteAndLateChildCleanupBeforeCloseSucceeds()throws Exception{
        var nativeConstructor=new CompletableFuture<JsonNode>();var token=new AtomicReference<ExtensionCancellation>();var constructed=new AtomicBoolean();var cleanupCalls=new AtomicInteger();
        var fixture=fixture((call,cancel)->{if("factory_create_agent".equals(call.path("invocation").path("operation").textValue())){token.set(cancel);return nativeConstructor.minimalCompletionStage();}assertTrue(constructed.get());cleanupCalls.incrementAndGet();return CompletableFuture.completedFuture(success());});
        construct(fixture).toFuture();var closing=fixture.client().closeAsync().toCompletableFuture();
        try{assertTrue(token.get().isCancelled());assertFalse(nativeConstructor.isDone());assertFalse(closing.isDone(),"native constructor still owns a future object");assertFalse(flag(fixture.client(),"closed"));assertEquals(0,cleanupCalls.get());constructed.set(true);nativeConstructor.complete(constructed());closing.get(2,TimeUnit.SECONDS);assertEquals(1,cleanupCalls.get());assertTrue(flag(fixture.client(),"closed"));}
        finally{constructed.set(true);nativeConstructor.complete(constructed());fixture.client().closeAsync().toCompletableFuture().get(2,TimeUnit.SECONDS);}
    }
    @Test void constructorShutdownDeadlineRetainsOriginalStageAndLateCleanupRetry()throws Exception{
        var nativeConstructor=new CompletableFuture<JsonNode>();var token=new AtomicReference<ExtensionCancellation>();var constructionCalls=new AtomicInteger();var cleanupCalls=new AtomicInteger();
        var fixture=fixture((call,cancel)->{if("factory_create_agent".equals(call.path("invocation").path("operation").textValue())){constructionCalls.incrementAndGet();token.set(cancel);return nativeConstructor.minimalCompletionStage();}cleanupCalls.incrementAndGet();return CompletableFuture.completedFuture(success());});construct(fixture).toFuture();
        var closing=fixture.client().closeAsync().toCompletableFuture();var error=assertThrows(ExecutionException.class,()->closing.get(7,TimeUnit.SECONDS));assertEquals("extension_timeout",EchoAgentException.from(error,"close").code());assertTrue(token.get().isCancelled());assertFalse(nativeConstructor.isDone());assertFalse(flag(fixture.client(),"closed"));assertTrue(map(fixture.client(),"factoryChildren").containsKey(fixture.child()));assertEquals(1,map(fixture.client(),"factoryConstructionJobs").size());assertEquals(0,cleanupCalls.get());
        var retry=fixture.client().closeAsync().toCompletableFuture();assertFalse(retry.isDone());nativeConstructor.complete(constructed());retry.get(2,TimeUnit.SECONDS);assertEquals(1,constructionCalls.get());assertEquals(1,cleanupCalls.get());assertTrue(flag(fixture.client(),"closed"));
    }
    @Test void constructorAdmissionRaceShutdownSignalsActualStageAndCleansLateChild()throws Exception{
        var nativeConstructor=new CompletableFuture<JsonNode>();var token=new AtomicReference<ExtensionCancellation>();var entered=new CountDownLatch(1);var release=new CountDownLatch(1);var cleanups=new AtomicInteger();
        var fixture=fixture((call,cancel)->{if("factory_create_agent".equals(call.path("invocation").path("operation").textValue())){token.set(cancel);entered.countDown();try{assertTrue(release.await(2,TimeUnit.SECONDS));}catch(InterruptedException error){Thread.currentThread().interrupt();return CompletableFuture.failedFuture(error);}return nativeConstructor.minimalCompletionStage();}cleanups.incrementAndGet();return CompletableFuture.completedFuture(success());});
        var executor=Executors.newSingleThreadExecutor();try{var admission=executor.submit(()->construct(fixture).toFuture());assertTrue(entered.await(2,TimeUnit.SECONDS));var closing=fixture.client().closeAsync().toCompletableFuture();assertTrue(token.get().isCancelled());assertFalse(closing.isDone());release.countDown();admission.get(2,TimeUnit.SECONDS);assertFalse(nativeConstructor.isDone());nativeConstructor.complete(constructed());closing.get(2,TimeUnit.SECONDS);assertEquals(1,cleanups.get());assertTrue(flag(fixture.client(),"closed"));}finally{release.countDown();nativeConstructor.complete(constructed());executor.shutdownNow();fixture.client().closeAsync().toCompletableFuture().get(2,TimeUnit.SECONDS);}
    }
    @Test void droppedConstructorSubscriptionDoesNotReleaseLateConstructionOrItsCleanupLease()throws Exception{
        var nativeConstructor=new CompletableFuture<JsonNode>();var cleanups=new AtomicInteger();var fixture=fixture((call,cancel)->"factory_create_agent".equals(call.path("invocation").path("operation").textValue())?nativeConstructor.minimalCompletionStage():CompletableFuture.completedFuture(cleanupCalls(cleanups)));
        var subscription=construct(fixture).toFuture();subscription.cancel(true);assertFalse(nativeConstructor.isDone());var closing=fixture.client().closeAsync().toCompletableFuture();assertFalse(closing.isDone());nativeConstructor.complete(constructed());closing.get(2,TimeUnit.SECONDS);assertEquals(1,cleanups.get());assertTrue(flag(fixture.client(),"closed"));
    }
    private static JsonNode cleanupCalls(AtomicInteger count){count.incrementAndGet();return success();}
    @Test void constructorFailureDoesNotSkipReservedChildCleanupOnShutdown()throws Exception{
        var nativeConstructor=new CompletableFuture<JsonNode>();var cleanups=new AtomicInteger();var fixture=fixture((call,cancel)->"factory_create_agent".equals(call.path("invocation").path("operation").textValue())?nativeConstructor.minimalCompletionStage():CompletableFuture.completedFuture(cleanupCalls(cleanups)));
        construct(fixture).toFuture();var closing=fixture.client().closeAsync().toCompletableFuture();nativeConstructor.completeExceptionally(new java.io.IOException("constructor failed after partial resource allocation"));closing.get(2,TimeUnit.SECONDS);assertEquals(1,cleanups.get());assertTrue(flag(fixture.client(),"closed"));
    }
    @Test void cleanupFailureWhileAnotherConstructorIsPendingMustNotBeHiddenBySameCloseRetry()throws Exception{
        var cleanup=new CompletableFuture<JsonNode>();var constructor=new CompletableFuture<JsonNode>();var attempts=new ConcurrentHashMap<WireHandle,AtomicInteger>();
        var fixture=fixture((call,cancel)->{if("factory_create_agent".equals(call.path("invocation").path("operation").textValue()))return constructor.minimalCompletionStage();var child=WireHandle.fromJson(call.path("extension"));int count=attempts.computeIfAbsent(child,ignored->new AtomicInteger()).incrementAndGet();return child.id().equals("child")&&count==1?cleanup.minimalCompletionStage():CompletableFuture.completedFuture(success());});
        invoke(fixture,"cleanupA",500_000_000).toFuture();var other=new WireHandle("constructorB","1","extension");var otherFixture=new Fixture(fixture.client(),fixture.parent(),other);construct(otherFixture).toFuture();
        var closing=fixture.client().closeAsync().toCompletableFuture();cleanup.completeExceptionally(new java.io.IOException("cleanup failed while constructor still pending"));assertFalse(closing.isDone());constructor.complete(constructed());
        assertThrows(ExecutionException.class,()->closing.get(2,TimeUnit.SECONDS));assertFalse(flag(fixture.client(),"closed"));assertEquals(1,attempts.get(fixture.child()).get());assertTrue(map(fixture.client(),"factoryChildren").containsKey(fixture.child()));
        fixture.client().closeAsync().toCompletableFuture().get(2,TimeUnit.SECONDS);assertEquals(2,attempts.get(fixture.child()).get());assertEquals(1,attempts.get(other).get());assertTrue(flag(fixture.client(),"closed"));
    }
    @Test void nestedTypedCleanupFailurePreservesCauseWithoutCreatingExceptionCycle()throws Exception{
        var nativeCleanup=new CompletableFuture<JsonNode>();var attempts=new AtomicInteger();var fixture=fixture((call,cancel)->attempts.incrementAndGet()==1?nativeCleanup.minimalCompletionStage():CompletableFuture.completedFuture(success()));invoke(fixture,"cleanup",500_000_000).toFuture();var closing=fixture.client().closeAsync().toCompletableFuture();
        var typed=new EchoAgentException("extension_failed","native cleanup failure","after_delay","native-close",null);nativeCleanup.completeExceptionally(new CompletionException(new CompletionException(typed)));
        var failure=assertThrows(ExecutionException.class,()->closing.get(2,TimeUnit.SECONDS));assertEquals("extension_failed",EchoAgentException.from(failure,"close").code());var seen=Collections.newSetFromMap(new IdentityHashMap<Throwable,Boolean>());Throwable cause=failure;while(cause!=null){assertTrue(seen.add(cause),"error mapping must not create a cyclic cause chain");cause=cause.getCause();}
        assertFalse(flag(fixture.client(),"closed"));fixture.client().closeAsync().toCompletableFuture().get(2,TimeUnit.SECONDS);assertEquals(2,attempts.get());
    }
}
