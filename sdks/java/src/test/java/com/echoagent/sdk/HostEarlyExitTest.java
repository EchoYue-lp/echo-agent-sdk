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
                             com.agentclientprotocol.sdk.spec.AcpClientTransport transport, Process process) {}
    private static LiveChild child() throws Exception { return child("import sys;sys.stdin.read()"); }
    private static LiveChild child(String code) throws Exception {
        var transportType=Class.forName("com.echoagent.sdk.EchoAgentClient$OwnedStdioTransport");
        var constructor=transportType.getDeclaredConstructor(com.agentclientprotocol.sdk.client.transport.AgentParameters.class,com.agentclientprotocol.sdk.json.AcpJsonMapper.class);constructor.setAccessible(true);
        var parameters=com.agentclientprotocol.sdk.client.transport.AgentParameters.builder("python3").args(List.of("-c",code)).build();
        var transport=(com.agentclientprotocol.sdk.spec.AcpClientTransport)constructor.newInstance(parameters,com.agentclientprotocol.sdk.json.AcpJsonMapper.createDefault());
        var session=new com.agentclientprotocol.sdk.spec.AcpClientSession(java.time.Duration.ofSeconds(30),transport,Map.of(),Map.of(),java.util.function.Function.identity());
        var clientConstructor=EchoAgentClient.class.getDeclaredConstructor(com.agentclientprotocol.sdk.spec.AcpClientSession.class,FacadeCatalog.class,com.agentclientprotocol.sdk.spec.AcpClientTransport.class);clientConstructor.setAccessible(true);
        var client=(EchoAgentClient)clientConstructor.newInstance(session,new FacadeCatalog(Path.of("../shared/facade-operation-catalog.json"),Path.of("../shared/contract-digests.json")),transport);
        var processField=transportType.getDeclaredField("process");processField.setAccessible(true);
        return new LiveChild(client,session,transport,(Process)processField.get(transport));
    }
    @org.junit.jupiter.api.Test
    void failedCleanupDoesNotSuppressLaterActualChildExitOfPendingRequest() throws Exception {
        Path work=Files.createTempDirectory("sdk-java-pending-frame-");Path frame=work.resolve("frame.json");
        // Establish a response-pending request, rather than racing an unwritten outbound frame.
        String code="import json,os,pathlib,sys;p=pathlib.Path("+JsonSupport.MAPPER.writeValueAsString(frame.toString())+");line=sys.stdin.readline();assert json.loads(line)['method']=='fixture-pending';p.with_suffix('.tmp').write_text(line);os.replace(p.with_suffix('.tmp'),p);sys.stdin.read()";
        var child=child(code);
        try {
            var pending=child.sdk().request("fixture-pending",JsonSupport.MAPPER.createObjectNode()).toCompletableFuture();
            long deadline=System.nanoTime()+TimeUnit.SECONDS.toNanos(2);
            while(!Files.exists(frame)&&System.nanoTime()<deadline)Thread.onSpinWait();
            assertTrue(Files.exists(frame),"original child must read the pending request before cleanup failure and exit");
            assertEquals("fixture-pending",JsonSupport.MAPPER.readTree(frame.toFile()).path("method").asText());
            assertFalse(pending.isDone(),"child read the request but deliberately has not answered it");
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
    @org.junit.jupiter.api.Test
    void publicCloseDeliversStdinEofBeforeTermAndWaitsForOriginalChild() throws Exception {
        Path work=Files.createTempDirectory("sdk-java-stdin-close-");
        String code="import pathlib,sys,signal,time\np=pathlib.Path("+JsonSupport.MAPPER.writeValueAsString(work.toString())+")\ndef term(*_):\n (p/'term').touch()\n sys.exit(0)\nsignal.signal(signal.SIGTERM,term)\n(p/'ready').touch()\nsys.stdin.read()\n(p/'eof').touch()\nwhile not (p/'release').exists(): time.sleep(0.01)\n";
        var child=child(code);
        try {
            long deadline=System.nanoTime()+TimeUnit.SECONDS.toNanos(2);
            while(!Files.exists(work.resolve("ready"))&&System.nanoTime()<deadline)Thread.onSpinWait();
            assertTrue(Files.exists(work.resolve("ready")));
            long originalPid=child.process().pid();
            var closing=child.sdk().closeAsync().toCompletableFuture();
            while(!Files.exists(work.resolve("eof"))&&!closing.isDone()&&System.nanoTime()<deadline)Thread.onSpinWait();
            assertTrue(Files.exists(work.resolve("eof")),"public close must send EOF to the original child before TERM");
            assertFalse(Files.exists(work.resolve("term")),"cooperative close must not send TERM");
            assertThrows(TimeoutException.class,()->closing.get(100,TimeUnit.MILLISECONDS));
            assertEquals(originalPid,child.process().pid());assertTrue(child.process().isAlive());
            Files.createFile(work.resolve("release"));closing.get(2,TimeUnit.SECONDS);
            assertFalse(child.process().isAlive());assertFalse(Files.exists(work.resolve("term")));
        } finally {Files.writeString(work.resolve("release"),"");if(child.process().isAlive())child.process().destroyForcibly();child.session().close();child.transport().closeGracefully().block();}
    }

    @org.junit.jupiter.api.Test
    void acceptedFrameMustFlushBeforeEofAndCancelledCloseWaiterDoesNotDropIt() throws Exception {
        Path work=Files.createTempDirectory("sdk-java-write-drain-");
        String code="import pathlib,sys,time\np=pathlib.Path("+JsonSupport.MAPPER.writeValueAsString(work.toString())+")\n(p/'ready').touch()\nwhile not (p/'read').exists(): time.sleep(0.01)\nline=sys.stdin.buffer.readline()\n(p/'frame').write_bytes(line)\nsys.stdin.buffer.read()\n(p/'eof').touch()\n";
        var child=child(code);
        try {
            long deadline=System.nanoTime()+TimeUnit.SECONDS.toNanos(2);
            while(!Files.exists(work.resolve("ready"))&&System.nanoTime()<deadline)Thread.onSpinWait();assertTrue(Files.exists(work.resolve("ready")));
            String payload="中文🌊\n"+"x".repeat(524288);
            var frame=new com.agentclientprotocol.sdk.spec.AcpSchema.JSONRPCNotification("fixture/write-drain",Map.of("payload",payload));
            var written=child.transport().sendMessage(frame).toFuture();
            var cancelled=child.sdk().closeAsync().toCompletableFuture();var joined=child.sdk().closeAsync().toCompletableFuture();assertTrue(cancelled.cancel(true));
            assertThrows(TimeoutException.class,()->written.get(100,TimeUnit.MILLISECONDS),"enqueue is not physical writer completion");
            assertThrows(TimeoutException.class,()->joined.get(100,TimeUnit.MILLISECONDS));assertTrue(child.process().isAlive());assertFalse(Files.exists(work.resolve("eof")));
            Files.createFile(work.resolve("read"));written.get(2,TimeUnit.SECONDS);joined.get(2,TimeUnit.SECONDS);
            assertFalse(child.process().isAlive());assertTrue(Files.exists(work.resolve("eof")));
            var actual=JsonSupport.MAPPER.readTree(Files.readString(work.resolve("frame")));assertEquals("fixture/write-drain",actual.path("method").asText());assertEquals(payload,actual.path("params").path("payload").asText());
        } finally {Files.writeString(work.resolve("read"),"");if(child.process().isAlive())child.process().destroyForcibly();child.session().close();child.transport().closeGracefully().block();}
    }

    @org.junit.jupiter.api.Test
    void blockedOfficialWriterCannotHoldFiveSecondForceStopBehindItsMonitor() throws Exception {
        var child=child("import time;time.sleep(60)");
        try {
            long originalPid=child.process().pid();
            var frame=new com.agentclientprotocol.sdk.spec.AcpSchema.JSONRPCNotification("fixture/write-blocked",Map.of("payload","x".repeat(524288)));
            var written=child.transport().sendMessage(frame).toFuture();long started=System.nanoTime();
            var closing=child.sdk().closeAsync().toCompletableFuture();var failure=assertThrows(ExecutionException.class,()->closing.get(7,TimeUnit.SECONDS));
            assertEquals("timeout",EchoAgentException.from(failure,"client/close").code());
            assertTrue(TimeUnit.NANOSECONDS.toMillis(System.nanoTime()-started)<6500,"force deadline must not wait on the blocked writer monitor");
            assertTrue(child.process().waitFor(2,TimeUnit.SECONDS));assertEquals(originalPid,child.process().pid());assertFalse(child.process().isAlive());
            assertThrows(ExecutionException.class,()->written.get(2,TimeUnit.SECONDS),"aborted frame cannot become a successful write");
            child.sdk().closeAsync().toCompletableFuture().get(2,TimeUnit.SECONDS);
        } finally {if(child.process().isAlive())child.process().destroyForcibly();child.session().close();child.transport().closeGracefully().onErrorResume(ignored->reactor.core.publisher.Mono.empty()).block();}
    }

    @org.junit.jupiter.api.Test
    void observedOriginalExitRejectsNewFrameWithoutInventingASuccessfulWrite() throws Exception {
        var child=child();
        try {
            child.process().getOutputStream().close();assertTrue(child.process().waitFor(2,TimeUnit.SECONDS));
            var written=child.transport().sendMessage(new com.agentclientprotocol.sdk.spec.AcpSchema.JSONRPCNotification("fixture/failed-write",Map.of())).toFuture();
            assertThrows(ExecutionException.class,()->written.get(2,TimeUnit.SECONDS));
            child.sdk().closeAsync().toCompletableFuture().get(2,TimeUnit.SECONDS);
            assertTrue(written.isCompletedExceptionally());assertFalse(child.process().isAlive());
        }finally{if(child.process().isAlive())child.process().destroyForcibly();child.session().close();child.transport().closeGracefully().onErrorResume(ignored->reactor.core.publisher.Mono.empty()).block();}
    }

    @org.junit.jupiter.api.Test
    void acceptedOutputFailureRemainsFailureThroughNativeCloseRetry() throws Exception {
        var child=child("import sys,time;sys.stdin.read();time.sleep(60)");
        try {
            child.process().getOutputStream().close();assertTrue(child.process().isAlive());
            var written=child.transport().sendMessage(new com.agentclientprotocol.sdk.spec.AcpSchema.JSONRPCNotification("fixture/failing-output",Map.of())).toFuture();
            assertThrows(ExecutionException.class,()->written.get(2,TimeUnit.SECONDS));
            var originalPid=child.process().pid();var causeField=child.transport().getClass().getDeclaredField("nativeFailure");causeField.setAccessible(true);
            var originalCause=((java.util.concurrent.atomic.AtomicReference<?>)causeField.get(child.transport())).get();assertNotNull(originalCause);
            long started=System.nanoTime();var closeFailure=assertThrows(ExecutionException.class,()->child.sdk().closeAsync().toCompletableFuture().get(7,TimeUnit.SECONDS));
            assertSame(originalCause,closeFailure.getCause(),"the deadline must preserve the first observed native write failure");
            assertTrue(TimeUnit.NANOSECONDS.toMillis(System.nanoTime()-started)<6500,"an observed tail failure must not disable the original native force-close deadline");
            assertTrue(child.process().waitFor(2,TimeUnit.SECONDS));assertEquals(originalPid,child.process().pid());
            child.sdk().closeAsync().toCompletableFuture().get(2,TimeUnit.SECONDS);
            assertTrue(written.isCompletedExceptionally());assertFalse(child.process().isAlive());
        }finally{if(child.process().isAlive())child.process().destroyForcibly();child.session().close();child.transport().closeGracefully().onErrorResume(ignored->reactor.core.publisher.Mono.empty()).block();}
    }

    @org.junit.jupiter.api.Test
    void missingOriginalCommandFailsWithoutStartingOrRetainingAChild() throws Exception {
        Path missing=Files.createTempDirectory("sdk-java-start-failure-").resolve("not-an-executable");
        var result=EchoAgentClient.spawn(missing.toString(),List.of(),Path.of("../shared/facade-operation-catalog.json"),Path.of("../shared/contract-digests.json"),Map.of()).toCompletableFuture();
        var failed=assertThrows(ExecutionException.class,()->result.get(2,TimeUnit.SECONDS));
        assertEquals("transport_error",EchoAgentException.from(failed,"initialize").code());assertTrue(result.isCompletedExceptionally());
    }

    @org.junit.jupiter.api.Test
    void repeatedConnectJoinsOriginalChildAndCloseJoinsLargeStderrDrain() throws Exception {
        Path work=Files.createTempDirectory("sdk-java-stderr-drain-");
        String code="import pathlib,sys,os\np=pathlib.Path("+JsonSupport.MAPPER.writeValueAsString(work.toString())+")\nwith (p/'starts').open('a') as log: log.write(str(os.getpid())+'\\n')\nsys.stderr.write('x'*131072+'\\n');sys.stderr.flush()\n(p/'stderr-written').touch()\nsys.stdin.read()\n";
        var child=child(code);
        try {
            long pid=child.process().pid();
            child.transport().connect(ignored->reactor.core.publisher.Mono.empty()).block(java.time.Duration.ofSeconds(2));
            long deadline=System.nanoTime()+TimeUnit.SECONDS.toNanos(2);
            while(!Files.exists(work.resolve("stderr-written"))&&System.nanoTime()<deadline)Thread.onSpinWait();
            assertTrue(Files.exists(work.resolve("stderr-written")),"stderr beyond pipe capacity must be drained before close");
            assertEquals(List.of(Long.toString(pid)),Files.readAllLines(work.resolve("starts")));
            child.sdk().closeAsync().toCompletableFuture().get(2,TimeUnit.SECONDS);assertFalse(child.process().isAlive());
            var drained=child.transport().getClass().getDeclaredField("stderrDrained");drained.setAccessible(true);
            assertTrue(((CompletableFuture<?>)drained.get(child.transport())).isDone(),"successful close must join the original stderr drainer");
        } finally {if(child.process().isAlive())child.process().destroyForcibly();child.session().close();child.transport().closeGracefully().onErrorResume(ignored->reactor.core.publisher.Mono.empty()).block();}
    }

}
