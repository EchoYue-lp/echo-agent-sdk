package com.echoagent.sdk;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.node.ObjectNode;
import org.junit.jupiter.api.Test;
import java.nio.file.*;
import java.util.*;
import java.util.concurrent.*;
import java.util.concurrent.atomic.AtomicInteger;
import static org.junit.jupiter.api.Assertions.*;
import static org.junit.jupiter.api.Assumptions.assumeTrue;

/** Accepted aliases call the real framework scanner; no scanner implementation here. */
class SecretScanHostTest {
    private static final Path FIXTURE=Path.of("../shared/testing/secret_scan_host_fixture.py").toAbsolutePath();
    private static <T>T await(CompletionStage<T> value)throws Exception{return value.toCompletableFuture().get(10,TimeUnit.SECONDS);}
    private static JsonNode helper(Path work,String...arguments)throws Exception {
        var command=new ArrayList<>(List.of("python3",FIXTURE.toString(),"--work",work.toString()));command.addAll(List.of(arguments));
        Path output=Files.createTempFile(work,"helper-",".out"),error=Files.createTempFile(work,"helper-",".err");var child=new ProcessBuilder(command).redirectOutput(output.toFile()).redirectError(error.toFile()).start();
        try{assertTrue(child.waitFor(5,TimeUnit.SECONDS));assertEquals(0,child.exitValue(),Files.readString(error));var text=Files.readString(output);return text.isBlank()?JsonSupport.MAPPER.nullNode():JsonSupport.MAPPER.readTree(text);}finally{if(child.isAlive()){child.destroyForcibly();assertTrue(child.waitFor(5,TimeUnit.SECONDS));}}
    }
    private static ObjectNode request(EchoAgentClient sdk,AgentHandle agent,SessionHandle session,String operation,String text,boolean wrong) {
        var resolved=sdk.catalog().resolve(operation);var request=JsonSupport.MAPPER.createObjectNode().put("operation",operation).put("signature_digest",wrong?"sha256:"+"0".repeat(64):resolved.signature());request.set("handle",agent.wire().toJson());var args=request.putArray("arguments");args.add(WireValues.value(session.wire()));if(text!=null)args.add(WireValues.value(text));return request;
    }
    @Test void securityRootAliasesKeepCompleteMatchByteOffsetAndUnicodeRedactionWithoutModelEffects()throws Exception {
        String host=System.getenv("ECHO_AGENT_SDK_HOST");if("1".equals(System.getenv("ECHO_AGENT_SDK_REQUIRE_HOST_E2E")))assertNotNull(host,"required Security Host missing");assumeTrue(host!=null&&!host.isBlank(),"real Host required");String configured=System.getenv("ECHO_AGENT_SDK_SECRET_ROOT");Path root=configured==null?Path.of(System.getProperty("java.io.tmpdir")):Path.of(configured);Files.createDirectories(root);Path work=Files.createTempDirectory(root,"java-");helper(work);var spec=JsonSupport.MAPPER.readTree(work.resolve("scenario.json").toFile());var facts=JsonSupport.MAPPER.createObjectNode();var rows=facts.putArray("rows");var provider=new AtomicInteger();
        try(var sdk=await(EchoAgentClient.spawn(host,List.of("--config",work.resolve("host.json").toString()),Path.of("../shared/facade-operation-catalog.json"),Path.of("../shared/contract-digests.json"),Map.of()))) {
            var descriptor=JsonSupport.MAPPER.createObjectNode().put("kind","llm_client").put("descriptor_version",1).put("model_name","security-unused-model").put("supports_streaming",false);
            var registration=await(sdk.registerLlmClient("security-unused-model",descriptor,(call,cancel)->{provider.incrementAndGet();return CompletableFuture.failedFuture(new AssertionError("Security helpers must not invoke model"));}));var agent=await(sdk.createAgent());var session=await(agent.createSession(work.toString()));var identity=facts.putObject("identity");identity.set("agent",agent.wire().toJson());identity.set("session",session.wire().toJson());facts.set("before",helper(work,"--snapshot"));
            for(var op:spec.path("operations")){
                String operation=op.asText();
                for(var item:spec.path("texts")){var row=rows.addObject().put("operation",operation).put("mode","value").put("text_id",item.path("id").asText());row.set("result",await(session.invoke(operation,List.of(item.path("text").asText()))));}
                for(String mode:List.of("wrong-signature","missing-text")){
                    var params=request(sdk,agent,session,operation,mode.equals("missing-text")?null:spec.path("texts").path(0).path("text").asText(),mode.equals("wrong-signature"));
                    var failure=assertThrows(ExecutionException.class,()->await(sdk.request(sdk.catalog().resolve(operation).method(),params)));var error=assertInstanceOf(EchoAgentException.class,failure.getCause());var row=rows.addObject().put("operation",operation).put("mode",mode);row.set("request",params);var value=row.putObject("error").put("code",error.code()).put("retryable",error.retryable()).put("operation",error.operation()).put("message",error.getMessage());value.set("details",error.details()==null?JsonSupport.MAPPER.nullNode():error.details());value.set("handle",error.handle()==null?JsonSupport.MAPPER.nullNode():error.handle().toJson());
                }
            }
            String scan="echo_agent::security::scan_secrets";facts.set("raw_scan",await(sdk.request(sdk.catalog().resolve(scan).method(),request(sdk,agent,session,scan,spec.path("texts").path(0).path("text").asText(),false))));facts.put("provider_calls",provider.get());facts.set("after",helper(work,"--snapshot"));session.close();agent.close();registration.close();
        }
        Files.writeString(work.resolve("observed.json"),facts.toPrettyString());System.out.println("Security native evidence: "+work+" "+helper(work,"--verify"));
    }
}
