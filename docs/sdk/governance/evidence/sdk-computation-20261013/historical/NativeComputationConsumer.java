package com.echoagent.sdk;
import com.fasterxml.jackson.databind.*;
import com.fasterxml.jackson.databind.node.*;
import java.nio.file.*;
import java.util.*;
import java.util.concurrent.*;
import java.security.MessageDigest;

/** TMP public SDK consumer only, no reflection or alternate runtime authority. */
public final class NativeComputationConsumer {
 static final ObjectMapper JSON=new ObjectMapper();
 static final Path ROOT=Path.of("/tmp/sdk-computation-candidate-fixture");
 static <T>T await(CompletionStage<T> stage)throws Exception{return stage.toCompletableFuture().get(15,TimeUnit.SECONDS);}
 static JsonNode decode(JsonNode value){return switch(value.path("kind").asText()){
  case "null"->NullNode.instance;case "bool","string","u64","i64","f64"->value.path("value");
  case "list"->{var a=JSON.createArrayNode();for(var v:value.path("value"))a.add(decode(v));yield a;}
  case "map"->{var o=JSON.createObjectNode();for(var e:value.path("value"))o.set(e.path("key").path("value").asText(),decode(e.path("value")));yield o;}
  default->value;
 };}
 static JsonNode rebase(JsonNode value,String work){if(value.isTextual())return TextNode.valueOf(value.textValue().replace("$WORK",work));if(value.isArray()){var a=JSON.createArrayNode();for(var child:value)a.add(rebase(child,work));return a;}if(value.isObject()){var o=JSON.createObjectNode();value.fields().forEachRemaining(e->o.set(e.getKey(),rebase(e.getValue(),work)));return o;}return value;}
 public static void main(String[] args)throws Exception{
  String host=System.getenv().getOrDefault("ECHO_AGENT_SDK_HOST","/tmp/echo-sdk-governance-guard-pivot-first-host");Path work=Files.createTempDirectory("sdk-computation-native-java-");
  var prepare=new ProcessBuilder("python3",ROOT.resolve("portable_fixture.py").toString(),work.toString()).inheritIO().start();if(prepare.waitFor()!=0)throw new IllegalStateException("fixture preparation failed");
  var rows=JSON.createArrayNode();var identity=JSON.createObjectNode();
  try(var sdk=await(EchoAgentClient.spawn(host,List.of("--config",work.resolve("host.json").toString()),Path.of("/Users/ls/MyWork/code/ylp_agent_learn/echo-agent-sdk-worktrees/sdk-semantic-governance/sdks/shared/facade-operation-catalog.json"),Path.of("/Users/ls/MyWork/code/ylp_agent_learn/echo-agent-sdk-worktrees/sdk-semantic-governance/sdks/shared/contract-digests.json"),Map.of()))){
   var agent=await(sdk.createAgent());var session=await(agent.createSession(work.toString()));identity.set("agent",agent.wire().toJson());identity.set("session",session.wire().toJson());identity.set("capability",sdk.capability());
   for(var step:JSON.readTree(ROOT.resolve("inputs.json").toFile()).path("rows")){
    String operation=step.path("operation").textValue();var arguments=rebase(step.path("arguments"),work.toString());var row=(ObjectNode)step.deepCopy();row.set("arguments",arguments);
    try{
     JsonNode result;
     if(operation.equals("structured_output.validate")){var request=JSON.createObjectNode().put("operation",operation).put("signature_digest","sha256:"+"0123456789abcdef".repeat(4));request.set("handle",session.wire().toJson());var wireArgs=request.putArray("arguments");for(var value:arguments)wireArgs.add(WireValues.value(JSON.convertValue(value,Object.class)));result=decode(await(sdk.request("_echo_agent/structured_output/validate",request)).path("value"));}
     else{List<Object> nativeArguments=new ArrayList<>();for(var value:arguments)nativeArguments.add(JSON.convertValue(value,Object.class));result=await(session.invoke(operation,nativeArguments));}
     row.set("result",result);if(!step.path("expected_error").isNull())throw new AssertionError("expected native error "+step.path("id").asText());
    }catch(ExecutionException exception){if(!(exception.getCause()instanceof EchoAgentException error))throw exception;var node=row.putObject("error").put("code",error.code()).put("message",error.getMessage()).put("operation",error.operation()).put("retryable",error.retryable());node.set("handle",error.handle()==null?NullNode.instance:error.handle().toJson());node.set("details",error.details()==null?NullNode.instance:error.details());if(!error.code().equals(step.path("expected_error").asText()))throw exception;}
    rows.add(row);
   }
   session.close();agent.close();
  }
  var result=JSON.createObjectNode().put("host",host).put("host_sha256",HexFormat.of().formatHex(MessageDigest.getInstance("SHA-256").digest(Files.readAllBytes(Path.of(host))))).put("work",work.toString());result.set("rows",rows);result.set("raw_identity",identity);var exports=result.putObject("actual_exports").put("output_text",Files.readString(work.resolve("output.txt")));exports.set("output_json",JSON.readTree(work.resolve("output.json").toFile()));result.putObject("actual_side_effects").put("sentinel_contents",Files.readString(work.resolve("sentinel.txt"))).put("sentinel_sha256",HexFormat.of().formatHex(MessageDigest.getInstance("SHA-256").digest(Files.readAllBytes(work.resolve("sentinel.txt")))));JSON.writerWithDefaultPrettyPrinter().writeValue(ROOT.resolve("java-final-computation-evidence.json").toFile(),result);System.out.println("Java native public corpus "+rows.size()+" work "+work);
 }
}
