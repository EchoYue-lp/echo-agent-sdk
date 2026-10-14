package com.echoagent.sdk;
import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.node.ObjectNode;
import java.nio.file.*;
import java.util.*;
import java.util.concurrent.*;
import java.util.concurrent.atomic.AtomicInteger;
import static org.junit.jupiter.api.Assertions.*;

public class McpChannelHostTest {
 @org.junit.jupiter.api.BeforeAll public static void requireHost(){var host=System.getenv("ECHO_AGENT_SDK_HOST");if("1".equals(System.getenv("ECHO_AGENT_SDK_REQUIRE_HOST_E2E")))assertTrue(host!=null&&!host.isBlank(),"required MCP/channel Host missing");org.junit.jupiter.api.Assumptions.assumeTrue(host!=null&&!host.isBlank(),"real Host binary required");}
 static final Path DIR=Path.of("../shared/testing").toAbsolutePath(),SDK=Path.of("..").toAbsolutePath();
 static JsonNode tree(Object v){return JsonSupport.MAPPER.valueToTree(v);}
 static JsonNode parse(String v)throws Exception{return JsonSupport.MAPPER.readTree(v);}
 static ObjectNode obj(){return JsonSupport.MAPPER.createObjectNode();}
 static <T>T await(CompletionStage<T> s)throws Exception{return s.toCompletableFuture().get(15,TimeUnit.SECONDS);}
 static JsonNode call(EchoAgentClient s,SessionHandle session,String op,Object... args)throws Exception{return await(s.call(op,session.wire(),Arrays.asList(args)));}
 static JsonNode result(String op,JsonNode value){var r=obj().put("operation",op);if(value!=null)r.set("value",value);return r;}
 static JsonNode outcome(String op,JsonNode value){return obj().put("outcome","result").set("result",result(op,value));}
 static JsonNode error(String msg){return obj().put("outcome","error").set("error",obj().put("code","extension_conflict").put("message",msg).put("retryable","after_delay"));}
 static EchoAgentException fails(CompletionStage<?> s,String code)throws Exception{try{await(s);throw new AssertionError("expected error "+code);}catch(ExecutionException e){Throwable c=e.getCause();while(c instanceof CompletionException&&c.getCause()!=null)c=c.getCause();var typed=assertInstanceOf(EchoAgentException.class,c);assertEquals(code,typed.code());return typed;}}
 static Path setup(String name)throws Exception{Path work=Files.createTempDirectory("sdk-java-"+name+"-");assertEquals(0,new ProcessBuilder("python3",DIR.resolve("mcp_channel_config_fixture.py").toString(),work.toString()).inheritIO().start().waitFor());System.out.println("MCP/channel evidence "+work);return work;}
 static EchoAgentClient spawn(Path work)throws Exception{return await(EchoAgentClient.spawn(System.getenv("ECHO_AGENT_SDK_HOST"),List.of("--config",work.resolve("host.json").toString()),SDK.resolve("shared/facade-operation-catalog.json"),SDK.resolve("shared/contract-digests.json"),Map.of(),List.of(),List.of()));}
 static WireHandle handle(JsonNode v){return WireHandle.fromJson(v);}
 static boolean live(long pid){return ProcessHandle.of(pid).map(ProcessHandle::isAlive).orElse(false);}
 interface Check {boolean get()throws Exception;}
 static void until(Check c)throws Exception{long end=System.nanoTime()+TimeUnit.SECONDS.toNanos(5);while(!c.get()){assertTrue(System.nanoTime()<end);Thread.sleep(10);}}
 static String c(String name){return "echo_agent::mcp::McpClient::"+name;}
 static void stdio()throws Exception{
  Path work=setup("mcp-stdio");var rows=JsonSupport.MAPPER.createArrayNode();
  try(var sdk=spawn(work)){
   var agent=await(sdk.createAgent());var one=await(agent.createSession());var two=await(agent.createSession());
   var ma=handle(call(sdk,one,"mcp.manager.open").get("resource"));var mb=handle(call(sdk,two,"mcp.manager.open").get("resource"));
   var a=handle(call(sdk,one,"mcp.server.connect",ma,"same-name","stdio","python3",List.of(DIR.resolve("mcp_channel_stdio_fixture.py").toString(),work.toString(),"a")).get("client"));var b=handle(call(sdk,two,"mcp.server.connect",mb,"same-name","stdio","python3",List.of(DIR.resolve("mcp_channel_stdio_fixture.py").toString(),work.toString(),"b")).get("client"));
   long pa=Long.parseLong(Files.readString(work.resolve("a.pid"))),pb=Long.parseLong(Files.readString(work.resolve("b.pid")));assertTrue(live(pa)&&live(pb));
   var cap=call(sdk,one,c("server_capabilities"),a);assertEquals(parse("{\"prompts\":{\"listChanged\":false},\"resources\":{\"subscribe\":false,\"listChanged\":false},\"tools\":{}}"),cap);rows.add(obj().put("operation",c("server_capabilities")).set("value",cap));
   var tools=call(sdk,one,c("tools"),a);assertEquals(parse("[{\"name\":\"echo\",\"description\":\"echo Unicode\",\"inputSchema\":{\"type\":\"object\",\"properties\":{\"text\":{\"type\":\"string\"}},\"required\":[\"text\"]}}]"),tools);rows.add(obj().put("operation",c("tools")).set("value",tools));
   var resources=parse("[{\"uri\":\"fixture://中文/text\",\"name\":\"中文🙂\",\"description\":\"text meta\",\"mimeType\":\"text/plain\"},{\"uri\":\"fixture://binary\",\"name\":\"binary\",\"mimeType\":\"application/octet-stream\"}]");
   for(var op:List.of("resources","list_resources")){var v=call(sdk,one,c(op),a);assertEquals(resources,v);rows.add(obj().put("operation",c(op)).set("value",v));}
   var templates=call(sdk,one,c("list_resource_templates"),a);assertEquals(parse("[{\"uriTemplate\":\"fixture://item/{id}\",\"name\":\"模板🙂\",\"description\":\"template\",\"mimeType\":\"text/plain\"}]"),templates);rows.add(obj().put("operation",c("list_resource_templates")).set("value",templates));
   for(var uri:List.of("fixture://binary","fixture://中文/text")){var v=call(sdk,one,c("read_resource"),a,uri);var expected=uri.endsWith("binary")?parse("{\"contents\":[{\"uri\":\"fixture://binary\",\"mimeType\":\"application/octet-stream\",\"blob\":\"AP9BgA==\"}]}"):parse("{\"contents\":[{\"uri\":\"fixture://中文/text\",\"mimeType\":\"text/plain\",\"text\":\"资源 中文🙂\"}]}");assertEquals(expected,v);rows.add(obj().put("operation",c("read_resource")).set("value",v));}
   var prompt=call(sdk,one,c("get_prompt"),a,"compose",Map.of("topic","无损🧪"));assertEquals(parse("{\"description\":\"topic prompt\",\"messages\":[{\"role\":\"user\",\"content\":{\"type\":\"text\",\"text\":\"主题 无损🧪\"}}]}"),prompt);rows.add(obj().put("operation",c("get_prompt")).set("value",prompt));
   var tool=call(sdk,one,c("call_tool"),a,"echo",Map.of("text","工具🙂"));assertEquals(parse("{\"content\":[{\"type\":\"text\",\"text\":\"工具🙂\"}],\"isError\":false}"),tool);rows.add(obj().put("operation",c("call_tool")).set("value",tool));assertTrue(call(sdk,one,c("ping"),a).isNull());
   fails(sdk.call(c("ping"),two.wire(),List.of(a)),"invalid_value");call(sdk,one,"mcp.server.disconnect",ma,"same-name");until(()->!live(pa));assertTrue(live(pb));fails(sdk.call(c("ping"),one.wire(),List.of(a)),"closed_handle");assertTrue(call(sdk,two,c("ping"),b).isNull());one.close();fails(sdk.call("mcp.manager.server_names",two.wire(),List.of(ma)),"closed_handle");assertTrue(live(pb));two.close();until(()->!live(pb));agent.close();rows.add(tree(Map.of("process_cleanup",Map.of("a_dead",true,"b_survived_a_close",true,"b_dead_after_own_close",true))));Files.writeString(work.resolve("observed.json"),rows.toString());
  }
 }
 static void transport()throws Exception{
  Path work=setup("mcp-transport-retry");var calls=JsonSupport.MAPPER.createArrayNode();var closes=new AtomicInteger();
  try(var sdk=spawn(work)){
   var reg=await(sdk.registerExtension("agent_component","mcp-transport",tree(Map.of("kind","agent_component","descriptor_version",1,"component","mcp_transport","name","transport","capabilities",Map.of("supports_notifications",false))),(call,cancel)->{
    var inner=call.path("invocation").path("input").path("call");String op=inner.path("operation").asText();synchronized(calls){calls.add(inner.deepCopy());}
    if(op.equals("mcp_transport_close")&&closes.incrementAndGet()==1)return CompletableFuture.completedFuture(error("controlled transport close failure"));
    JsonNode res;if(op.equals("mcp_transport_send")){var req=JsonSupport.fromWire(inner.path("input").path("request"));var value=req.path("method").asText().equals("initialize")?tree(Map.of("protocolVersion","2024-11-05","capabilities",Map.of(),"serverInfo",Map.of("name","registered-中文🙂","version","1"))):obj();var response=obj().put("jsonrpc","2.0");if(req.has("id"))response.set("id",req.get("id"));response.set("result",value);res=result(op,obj().set("response",WireValues.value(JsonSupport.MAPPER.convertValue(response,Object.class))));}
    else if(op.equals("mcp_transport_try_notification"))res=result(op,obj().putNull("notification"));else res=result(op,null);
    return CompletableFuture.completedFuture(outcome("agent_component_call",obj().put("component","mcp_transport").set("result",res)));
   }));var agent=await(sdk.createAgent());var session=await(agent.createSession());var client=handle(await(sdk.invoke("echo_integration::mcp::client::McpClient::from_transport",agent.wire(),List.of(session.wire(),"registered-中文🙂",reg.wire()))));
   assertEquals("after_delay",fails(sdk.call(c("close"),session.wire(),List.of(client)),"framework_error").retryable());assertEquals(1,closes.get());assertEquals("registered-中文🙂",call(sdk,session,c("server_name"),client).textValue());assertTrue(call(sdk,session,c("close"),client).isNull());assertEquals(2,closes.get());fails(sdk.call(c("ping"),session.wire(),List.of(client)),"closed_handle");for(int i=0;i<4;i++)call(sdk,session,"mcp.manager.open");fails(sdk.call("mcp.manager.open",session.wire(),List.of()),"payload_too_large");session.close();reg.close();agent.close();Files.writeString(work.resolve("observed.json"),obj().set("calls",calls).toString());Files.writeString(work.resolve("summary.json"),tree(Map.of("closeCount",closes.get(),"released_quota_refilled",4)).toString());
  }
 }
 static void channels(String mode)throws Exception{
  Path work=setup("channels-"+mode);var calls=JsonSupport.MAPPER.createArrayNode();var stops=new HashMap<String,Integer>();var running=new HashMap<String,Boolean>();var children=new HashMap<String,Process>();for(var id:List.of("a","b")){stops.put(id,0);running.put(id,false);}
  try(var sdk=spawn(work)){
   var handler=await(sdk.registerChannelMessageHandler("handler",tree(Map.of("kind","channel_message_handler","descriptor_version",1,"handler_id","handler")),(call,cancel)->CompletableFuture.failedFuture(new AssertionError("no inbound path must not be faked"))));var regs=new ArrayList<ExtensionRegistration>();
   for(var id:List.of("a","b"))regs.add(await(sdk.registerChannelPlugin("plugin-"+id,tree(Map.of("kind","channel_plugin","descriptor_version",1,"channel_id",id,"label","通道 "+id,"handler_id","handler","capabilities",Map.of("chat_types",List.of("direct","group"),"supports_media",true,"supports_threads",true))),(call,cancel)->{
    String op=call.path("invocation").path("operation").asText();synchronized(calls){calls.add(obj().put("id",id).put("operation",op).set("input",call.path("invocation").path("input")));}
    try{if(op.equals("channel_start")){running.put(id,true);children.put(id,new ProcessBuilder("python3","-c","import time; time.sleep(60)").start());Files.writeString(work.resolve(id+".pid"),String.valueOf(children.get(id).pid()));}if(op.equals("channel_stop")){stops.put(id,stops.get(id)+1);if(mode.equals("retry")&&id.equals("a")&&stops.get(id)==1)return CompletableFuture.completedFuture(error("stop once fails"));children.get(id).destroy();assertTrue(children.get(id).waitFor(3,TimeUnit.SECONDS));running.put(id,false);}return CompletableFuture.completedFuture(outcome(op,JsonSupport.MAPPER.nullNode()));}catch(Exception e){return CompletableFuture.failedFuture(e);}
   })));
   var agent=await(sdk.createAgent());var session=await(agent.createSession());var manager=handle(call(sdk,session,"channels.manager.open").get("resource"));for(var reg:regs)assertTrue(call(sdk,session,"channels.plugin.register",manager,reg.wire(),handler.wire()).path("registered").asBoolean());assertEquals(tree(Map.of("channels","2","failures","0")),call(sdk,session,"channels.manager.start",manager,handler.wire()));
   var attachments=new ArrayList<Object>();int i=0;for(var kind:List.of("image","file","audio","video"))attachments.add(Map.of("kind",kind,"data_base64",Base64.getEncoder().encodeToString(new byte[]{0,(byte)255,(byte)i++,(byte)128}),"filename","文件-"+kind+"-🙂.bin"));var message=Map.of("channel_id","a","to","收件人🙂","chat_type","direct","text","hello 中文🧪","reply_to","原消息🙂","attachments",attachments);assertTrue(call(sdk,session,"channels.plugin.send",manager,"a",WireValues.value(message)).path("sent").asBoolean());JsonNode send=null;for(var c:calls)if(c.path("operation").asText().equals("channel_send"))send=c.get("input");assertEquals(tree(Map.of("message",message)),send);
   if(mode.equals("retry")){fails(sdk.call("channels.manager.stop_exact",session.wire(),List.of(manager,"a")),"framework_error");assertTrue(children.get("a").isAlive()&&children.get("b").isAlive());}assertTrue(running.get("b"));assertEquals(0,stops.get("b"));assertTrue(call(sdk,session,"channels.manager.stop_exact",manager,"a").isNull());assertFalse(children.get("a").isAlive());assertTrue(children.get("b").isAlive());assertEquals(mode.equals("retry")?2:1,stops.get("a"));assertEquals(0,stops.get("b"));assertTrue(call(sdk,session,"channels.manager.health",manager,"b").path("healthy").asBoolean());session.close();assertFalse(children.get("b").isAlive());assertEquals(1,stops.get("b"));for(var reg:regs)reg.close();handler.close();agent.close();Files.writeString(work.resolve("observed.json"),obj().set("calls",calls).toString());Files.writeString(work.resolve("summary.json"),tree(Map.of("stops",stops,"running",running,"inbound_handler_calls",0)).toString());
  }finally{for(var child:children.values())if(child.isAlive())child.destroy();}
 }
 @org.junit.jupiter.api.Test public void stdioMcpResourcesPromptsToolsAndExactOwnerCleanup()throws Exception{stdio();}
 @org.junit.jupiter.api.Test public void registeredTransportCloseFailureRetainsOwnerAndQuota()throws Exception{transport();}
 @org.junit.jupiter.api.Test public void channelAttachmentsAndExactStop()throws Exception{channels("normal");}
 @org.junit.jupiter.api.Test public void channelFailedStopRetainsOwnerForRetry()throws Exception{channels("retry");}

 static CompletionStage<JsonNode> closeManager(EchoAgentClient sdk,SessionHandle session,WireHandle manager){
  var params=obj().put("operation","facade.resource.close").put("signature_digest",sdk.catalog().resolve("facade.resource.close").signature());params.set("handle",session.wire().toJson());params.putArray("arguments").add(JsonSupport.wire(manager));return sdk.request("_echo_agent/facade/invoke",params);
 }
 record ChannelOwner(SessionHandle session,WireHandle manager,List<String> ids){}
 @org.junit.jupiter.api.Test public void ownedChannelCallbacksReenterMetadataAndPreserveCloseOwner()throws Exception{
  Path work=setup("channels-owned-reentry");var events=JsonSupport.MAPPER.createArrayNode();var owners=new ConcurrentHashMap<String,ChannelOwner>();var phases=new ConcurrentHashMap<String,String>();var stops=new ConcurrentHashMap<String,Integer>();for(var key:List.of("a","b","other"))stops.put(key,0);
  var parkHealth=new java.util.concurrent.atomic.AtomicBoolean();var failCloseOnce=new java.util.concurrent.atomic.AtomicBoolean(true);var parked=new CompletableFuture<Void>();var release=new CompletableFuture<Void>();
  try(var sdk=spawn(work)){
   var handler=await(sdk.registerChannelMessageHandler("owned-handler",tree(Map.of("kind","channel_message_handler","descriptor_version",1,"handler_id","owned-handler")),(call,cancel)->CompletableFuture.failedFuture(new AssertionError("no accepted inbound consumer; do not fabricate one"))));
   var agent=await(sdk.createAgent());var one=await(agent.createSession());var two=await(agent.createSession());var ma=handle(call(sdk,one,"channels.manager.open").get("resource"));var mb=handle(call(sdk,two,"channels.manager.open").get("resource"));var regs=new ArrayList<ExtensionRegistration>();
   for(var key:List.of("a","b","other")){
    String channel=key.equals("other")?"c":key;var owner=new ChannelOwner(key.equals("other")?two:one,key.equals("other")?mb:ma,key.equals("other")?List.of("c"):List.of("a","b"));owners.put(key,owner);phases.put(key,"active");
    var reg=await(sdk.registerChannelPlugin("owned-"+key,tree(Map.of("kind","channel_plugin","descriptor_version",1,"channel_id",channel,"label","owned 中文🙂 "+key,"handler_id","owned-handler","capabilities",Map.of("chat_types",List.of("direct"),"supports_media",false,"supports_threads",false))),(callback,cancel)->CompletableFuture.supplyAsync(()->{
     String operation=callback.path("invocation").path("operation").asText(),phase=phases.get(key);var current=owners.get(key);
     try{
      for(var metadata:List.of("ids","list")){
       String observation;
       try{var value=sdk.call("channels.manager."+metadata,current.session().wire(),List.of(current.manager())).toCompletableFuture().get(2,TimeUnit.SECONDS);assertEquals("active",phase);var ids=new ArrayList<String>();for(var id:metadata.equals("ids")?value:value.path("channels"))ids.add(id.textValue());Collections.sort(ids);assertEquals(current.ids(),ids);observation="read-before-return";}
       catch(ExecutionException exception){Throwable cause=exception.getCause();while(cause instanceof CompletionException&&cause.getCause()!=null)cause=cause.getCause();var error=assertInstanceOf(EchoAgentException.class,cause);assertNotEquals("active",phase);assertEquals("closed_handle",error.code());observation=error.code();}
       synchronized(events){events.add(tree(Map.of("id",key,"operation",operation,"phase",phase,"metadata",metadata,"result",observation)));Files.writeString(work.resolve("observed.json"),events.toString());}
      }
      if(key.equals("a")&&operation.equals("channel_health")&&parkHealth.get()){parked.complete(null);release.get(10,TimeUnit.SECONDS);}
      if(operation.equals("channel_stop")){stops.compute(key,(id,count)->count+1);if(key.equals("a")&&phase.equals("resource-close")&&failCloseOnce.compareAndSet(true,false))return error("owned stop once fails");}
      return outcome(operation,JsonSupport.MAPPER.nullNode());
     }catch(Exception exception){throw new CompletionException(exception);}
    })));
    regs.add(reg);call(sdk,owner.session(),"channels.plugin.register",owner.manager(),reg.wire(),handler.wire());
   }
   assertEquals(tree(Map.of("channels","2","failures","0")),call(sdk,one,"channels.manager.start",ma,handler.wire()));assertEquals(tree(Map.of("channels","1","failures","0")),call(sdk,two,"channels.manager.start",mb,handler.wire()));
   var message=Map.of("channel_id","a","to","用户🙂","chat_type","direct","text","重入中文");assertTrue(call(sdk,one,"channels.plugin.send",ma,"a",WireValues.value(message)).path("sent").asBoolean());
   parkHealth.set(true);var health=sdk.call("channels.manager.health",one.wire(),List.of(ma,"a")).toCompletableFuture();
   try{parked.get(3,TimeUnit.SECONDS);assertFalse(health.isDone());var ids=new ArrayList<String>();for(var id:sdk.call("channels.manager.ids",one.wire(),List.of(ma)).toCompletableFuture().get(2,TimeUnit.SECONDS))ids.add(id.textValue());Collections.sort(ids);assertEquals(List.of("a","b"),ids);assertTrue(sdk.call("channels.manager.health",one.wire(),List.of(ma,"b")).toCompletableFuture().get(2,TimeUnit.SECONDS).path("healthy").asBoolean());assertTrue(sdk.call("channels.manager.health",two.wire(),List.of(mb,"c")).toCompletableFuture().get(2,TimeUnit.SECONDS).path("healthy").asBoolean());assertFalse(health.isDone());}finally{release.complete(null);}
   assertTrue(await(health).path("healthy").asBoolean());assertTrue(call(sdk,one,"channels.manager.stop",ma,"b").path("stopped").asBoolean());assertTrue(call(sdk,one,"channels.manager.stop_exact",ma,"a").isNull());assertEquals(Map.of("a",1,"b",1,"other",0),stops);
   call(sdk,one,"channels.manager.start",ma,handler.wire());assertTrue(call(sdk,one,"channels.manager.stop_all",ma).isNull());assertEquals(Map.of("a",2,"b",2,"other",0),stops);call(sdk,one,"channels.manager.start",ma,handler.wire());phases.put("a","resource-close");phases.put("b","resource-close");
   fails(closeManager(sdk,one,ma),"framework_error");assertEquals(Map.of("a",3,"b",3,"other",0),stops);assertTrue(call(sdk,two,"channels.manager.health",mb,"c").path("healthy").asBoolean());await(closeManager(sdk,one,ma));assertEquals(Map.of("a",4,"b",3,"other",0),stops);fails(sdk.call("channels.manager.ids",one.wire(),List.of(ma)),"closed_handle");
   for(int i=0;i<3;i++)call(sdk,one,"channels.manager.open");fails(sdk.call("channels.manager.open",one.wire(),List.of()),"payload_too_large");one.close();phases.put("other","session-close");two.close();assertEquals(Map.of("a",4,"b",3,"other",1),stops);for(var reg:regs)reg.close();handler.close();agent.close();
   synchronized(events){Files.writeString(work.resolve("observed.json"),tree(Map.of("events",events,"stops",stops,"parked_other_access",true,"quota_refilled",3,"inbound_handler_calls",0)).toString());}
  }finally{release.complete(null);}
 }

}
