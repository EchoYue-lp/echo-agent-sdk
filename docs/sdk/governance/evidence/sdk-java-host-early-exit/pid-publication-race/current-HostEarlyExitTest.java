package com.echoagent.sdk;

import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.ValueSource;
import java.nio.file.*;
import java.util.*;
import java.util.concurrent.*;
import static org.junit.jupiter.api.Assertions.*;

/** Real original child exit must fail handshake promptly; no fabricated transport/Host response. */
class HostEarlyExitTest {
    @ParameterizedTest
    @ValueSource(booleans={false,true})
    void originalChildExitBeforeOrDuringInitializeIsAnExplicitFailure(boolean afterRead) throws Exception {
        Path work=Files.createTempDirectory("sdk-java-early-exit-");Path pid=work.resolve("child.pid");
        // Publish the PID only after its complete contents exist; exists() is the readiness gate.
        String code="import os,sys,pathlib;pathlib.Path(sys.argv[1]+'.tmp').write_text(str(os.getpid()));os.replace(sys.argv[1]+'.tmp',sys.argv[1]);"
                +(afterRead?"sys.stdin.readline();":"")+"sys.stderr.write('controlled initialize child exit\\n');sys.stderr.flush();sys.exit(7)";
        var client=EchoAgentClient.spawn("python3",List.of("-c",code,pid.toString()),
                Path.of("../shared/facade-operation-catalog.json"),Path.of("../shared/contract-digests.json"),Map.of()).toCompletableFuture();
        try {
            var error=assertThrows(ExecutionException.class,()->client.get(2,TimeUnit.SECONDS),"the exited child must not remain a pending initialize request");
            Throwable cause=error.getCause();while(cause instanceof CompletionException && cause.getCause()!=null)cause=cause.getCause();
            var typed=assertInstanceOf(EchoAgentException.class,cause);assertEquals("transport_error",typed.code());assertEquals("initialize",typed.operation());
            assertTrue(Files.exists(pid));long child=Long.parseLong(Files.readString(pid));
            var handle=ProcessHandle.of(child);if(handle.isPresent())handle.get().onExit().get(2,TimeUnit.SECONDS);
            assertFalse(ProcessHandle.of(child).map(ProcessHandle::isAlive).orElse(false));
            assertTrue(client.isCompletedExceptionally());
        } finally { client.whenComplete((value,error)->{if(value!=null)value.close();}); }
    }

    @org.junit.jupiter.api.Test
    void cancellingInitializeWaitClosesTheSameUnresponsiveOriginalChild() throws Exception {
        Path work=Files.createTempDirectory("sdk-java-initialize-cancel-");Path pid=work.resolve("child.pid");
        // Publish the PID only after its complete contents exist; exists() is the readiness gate.
        String code="import os,sys,pathlib;pathlib.Path(sys.argv[1]+'.tmp').write_text(str(os.getpid()));os.replace(sys.argv[1]+'.tmp',sys.argv[1]);sys.stdin.readline();sys.stdin.read();sys.exit(0)";
        var client=EchoAgentClient.spawn("python3",List.of("-c",code,pid.toString()),Path.of("../shared/facade-operation-catalog.json"),Path.of("../shared/contract-digests.json"),Map.of()).toCompletableFuture();
        long deadline=System.nanoTime()+TimeUnit.SECONDS.toNanos(2);
        while(!Files.exists(pid)&&System.nanoTime()<deadline)Thread.onSpinWait();
        assertTrue(Files.exists(pid));long child=Long.parseLong(Files.readString(pid));var handle=ProcessHandle.of(child);assertTrue(handle.isPresent()&&handle.get().isAlive());
        assertTrue(client.cancel(true));handle.get().onExit().get(2,TimeUnit.SECONDS);
        assertFalse(ProcessHandle.of(child).map(ProcessHandle::isAlive).orElse(false));assertTrue(client.isCancelled());
    }

    private record LiveChild(EchoAgentClient sdk, com.agentclientprotocol.sdk.spec.AcpClientSession session,
                             com.agentclientprotocol.sdk.client.transport.StdioAcpClientTransport transport, Process process) {}
    private static LiveChild child() throws Exception {
        var transportType=Class.forName("com.echoagent.sdk.EchoAgentClient$OwnedStdioTransport");
        var constructor=transportType.getDeclaredConstructor(com.agentclientprotocol.sdk.client.transport.AgentParameters.class,com.agentclientprotocol.sdk.json.AcpJsonMapper.class);constructor.setAccessible(true);
        var parameters=com.agentclientprotocol.sdk.client.transport.AgentParameters.builder("python3").args(List.of("-c","import sys;sys.stdin.read()")).build();
        var transport=(com.agentclientprotocol.sdk.client.transport.StdioAcpClientTransport)constructor.newInstance(parameters,com.agentclientprotocol.sdk.json.AcpJsonMapper.createDefault());
        var session=new com.agentclientprotocol.sdk.spec.AcpClientSession(java.time.Duration.ofSeconds(30),transport,Map.of(),Map.of(),java.util.function.Function.identity());
        var clientConstructor=EchoAgentClient.class.getDeclaredConstructor(com.agentclientprotocol.sdk.spec.AcpClientSession.class,FacadeCatalog.class,com.agentclientprotocol.sdk.spec.AcpClientTransport.class);clientConstructor.setAccessible(true);
        var client=(EchoAgentClient)clientConstructor.newInstance(session,new FacadeCatalog(Path.of("../shared/facade-operation-catalog.json"),Path.of("../shared/contract-digests.json")),transport);
        var processField=com.agentclientprotocol.sdk.client.transport.StdioAcpClientTransport.class.getDeclaredField("process");processField.setAccessible(true);
        return new LiveChild(client,session,transport,(Process)processField.get(transport));
    }
    @org.junit.jupiter.api.Test
    void failedCleanupDoesNotSuppressLaterActualChildExitOfPendingRequest() throws Exception {
        var child=child();
        try {
            var pending=child.sdk().request("fixture-pending",JsonSupport.MAPPER.createObjectNode()).toCompletableFuture();
            var field=EchoAgentClient.class.getDeclaredField("factoryCleanupJobs");field.setAccessible(true);
            @SuppressWarnings("unchecked") var jobs=(Map<WireHandle,CompletableFuture<com.fasterxml.jackson.databind.JsonNode>>)field.get(child.sdk());
            jobs.put(new WireHandle("child","1","extension"),CompletableFuture.failedFuture(new EchoAgentException("extension_failed","controlled owned cleanup failure","after_delay","client/close",null)));
            assertThrows(ExecutionException.class,()->child.sdk().closeAsync().toCompletableFuture().get(2,TimeUnit.SECONDS));assertTrue(child.process().isAlive());
            child.process().destroy();assertTrue(child.process().waitFor(2,TimeUnit.SECONDS));
            var error=assertThrows(ExecutionException.class,()->pending.get(2,TimeUnit.SECONDS));assertEquals("transport_error",EchoAgentException.from(error,"fixture-pending").code());
            var slots=com.agentclientprotocol.sdk.spec.AcpClientSession.class.getDeclaredField("pendingResponses");slots.setAccessible(true);
            jobs.clear();child.sdk().closeAsync().toCompletableFuture().get(2,TimeUnit.SECONDS);
            assertTrue(((Map<?,?>)slots.get(child.session())).isEmpty());
        } finally {if(child.process().isAlive())child.process().destroy();child.session().close();child.transport().closeGracefully().block();}
    }
    @org.junit.jupiter.api.Test
    void actualChildExitFailsIdleEventSubscriberAndClearsLocalCancelAddresses() throws Exception {
        var child=child();var failure=new CompletableFuture<Throwable>();
        try {
            child.sdk().events(new WireHandle("event-stream","1","stream")).subscribe(new java.util.concurrent.Flow.Subscriber<>() {
                public void onSubscribe(java.util.concurrent.Flow.Subscription subscription){subscription.request(Long.MAX_VALUE);}
                public void onNext(com.fasterxml.jackson.databind.JsonNode value){fail("no event was produced");}
                public void onError(Throwable error){failure.complete(error);}
                public void onComplete(){failure.completeExceptionally(new IllegalStateException("unexpected successful EOF"));}
            });
            var field=EchoAgentClient.class.getDeclaredField("invocationCancellations");field.setAccessible(true);
            @SuppressWarnings("unchecked")var addresses=(Map<String,ExtensionCancellation>)field.get(child.sdk());var cancellation=new ExtensionCancellation();addresses.put("original",cancellation);
            child.process().destroy();assertTrue(child.process().waitFor(2,TimeUnit.SECONDS));
            assertEquals("transport_error",EchoAgentException.from(failure.get(2,TimeUnit.SECONDS),"events").code());assertTrue(cancellation.isCancelled());assertTrue(addresses.isEmpty());
            child.sdk().closeAsync().toCompletableFuture().get(2,TimeUnit.SECONDS);
        } finally {if(child.process().isAlive())child.process().destroy();child.session().close();child.transport().closeGracefully().block();}
    }

    @ParameterizedTest
    @ValueSource(booleans={false,true})
    void actualChildExitImmediatelyFailsNewEventOrUpdateSubscriber(boolean updates) throws Exception {
        var child=child();var failure=new CompletableFuture<Throwable>();
        try {
            child.process().destroy();assertTrue(child.process().waitFor(2,TimeUnit.SECONDS));
            var type=child.transport().getClass();var field=type.getDeclaredField("failure");field.setAccessible(true);
            ((CompletableFuture<?>)field.get(child.transport())).get(2,TimeUnit.SECONDS);
            java.util.concurrent.Flow.Publisher<com.fasterxml.jackson.databind.JsonNode> publisher=updates
                    ? child.sdk().updates("late-session") : child.sdk().events(new WireHandle("late-stream","1","stream"));
            publisher.subscribe(new java.util.concurrent.Flow.Subscriber<>() {
                public void onSubscribe(java.util.concurrent.Flow.Subscription subscription){subscription.request(Long.MAX_VALUE);}
                public void onNext(com.fasterxml.jackson.databind.JsonNode value){fail("no event was produced after child exit");}
                public void onError(Throwable error){failure.complete(error);}
                public void onComplete(){failure.completeExceptionally(new IllegalStateException("unexpected successful EOF after child exit"));}
            });
            assertEquals("transport_error",EchoAgentException.from(failure.get(2,TimeUnit.SECONDS),"late-subscription").code());
            child.sdk().closeAsync().toCompletableFuture().get(2,TimeUnit.SECONDS);
        } finally {if(child.process().isAlive())child.process().destroy();child.session().close();child.transport().closeGracefully().block();}
    }

    @ParameterizedTest
    @ValueSource(booleans={false,true})
    void newEventOrUpdateSubscriberDuringOwnedCleanupGetsNaturalCompletion(boolean updates) throws Exception {
        var child=child();var complete=new CompletableFuture<Void>();var cleanup=new CompletableFuture<com.fasterxml.jackson.databind.JsonNode>();
        try {
            var field=EchoAgentClient.class.getDeclaredField("factoryCleanupJobs");field.setAccessible(true);
            @SuppressWarnings("unchecked") var jobs=(Map<WireHandle,CompletableFuture<com.fasterxml.jackson.databind.JsonNode>>)field.get(child.sdk());
            jobs.put(new WireHandle("held-child","1","extension"),cleanup);
            var closing=child.sdk().closeAsync().toCompletableFuture();assertFalse(closing.isDone());assertTrue(child.process().isAlive());
            java.util.concurrent.Flow.Publisher<com.fasterxml.jackson.databind.JsonNode> publisher=updates
                    ?child.sdk().updates("created-during-close"):child.sdk().events(new WireHandle("created-during-close","1","stream"));
            publisher.subscribe(new java.util.concurrent.Flow.Subscriber<>() {
                public void onSubscribe(java.util.concurrent.Flow.Subscription subscription){subscription.request(Long.MAX_VALUE);}
                public void onNext(com.fasterxml.jackson.databind.JsonNode value){fail("no value is produced during closing");}
                public void onError(Throwable error){complete.completeExceptionally(error);}
                public void onComplete(){complete.complete(null);}
            });
            var result=JsonSupport.MAPPER.createObjectNode().put("outcome","result");result.putObject("result").put("operation","agent_close").putNull("value");cleanup.complete(result);
            closing.get(2,TimeUnit.SECONDS);complete.get(2,TimeUnit.SECONDS);assertFalse(child.process().isAlive());
        } finally {cleanup.complete(JsonSupport.MAPPER.createObjectNode().put("outcome","result"));if(child.process().isAlive())child.process().destroy();child.session().close();child.transport().closeGracefully().block();}
    }
}
