import java.net.URI;
import java.net.http.*;
public final class Counter {
 public static void main(String[] args) throws Exception {
  var client=HttpClient.newBuilder().build();
  for(int i=1;i<=2;i++){
   System.out.println("SEND "+i);
   try {
    var r=client.send(HttpRequest.newBuilder(URI.create(args[0]+"/store")).POST(HttpRequest.BodyPublishers.ofString("{\"operation\":\"load\"}")).build(),HttpResponse.BodyHandlers.ofString());
    System.out.println("RESULT "+i+" "+r.statusCode()+" "+r.body());
   } catch(Exception e){System.out.println("ERROR "+i+" "+e);e.printStackTrace(System.out);System.exit(1);}
  }
 }
}
