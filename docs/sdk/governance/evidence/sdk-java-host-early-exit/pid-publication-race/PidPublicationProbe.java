import java.nio.file.*;
import java.io.*;
import java.util.concurrent.TimeUnit;
public final class PidPublicationProbe {
 public static void main(String[] args)throws Exception{
  boolean atomic=args[0].equals("after");Path work=Files.createTempDirectory(Path.of(args[1]),"pid-publication-"),target=work.resolve("child.pid");
  String code="import os,sys,pathlib;p=pathlib.Path(sys.argv[1]"+(atomic?"+'.tmp'":"")+");f=p.open('w');print('opened',flush=True);sys.stdin.readline();f.write(str(os.getpid()));f.close();"+(atomic?"os.replace(str(p),sys.argv[1]);":"");
  Process child=new ProcessBuilder("python3","-c",code,target.toString()).start();
  try{
   String ready=new BufferedReader(new InputStreamReader(child.getInputStream())).readLine();if(!"opened".equals(ready))throw new AssertionError("child never opened publication file");
   System.out.println("actual child="+child.pid()+" targetExistsWhilePayloadHeld="+Files.exists(target));
   if(!atomic){System.out.println("old exists readiness accepts an empty PID file");Long.parseLong(Files.readString(target));throw new AssertionError("old readiness unexpectedly parsed partial contents");}
   if(Files.exists(target))throw new AssertionError("atomic fixture published incomplete payload");
   child.getOutputStream().write("release\n".getBytes(java.nio.charset.StandardCharsets.UTF_8));child.getOutputStream().flush();
   long deadline=System.nanoTime()+TimeUnit.SECONDS.toNanos(2);while(!Files.exists(target)&&System.nanoTime()<deadline)Thread.onSpinWait();
   if(!Files.exists(target))throw new AssertionError("original PID publication timed out");long published=Long.parseLong(Files.readString(target));if(published<=0||published!=child.pid())throw new AssertionError("published PID is not the original child");if(!child.waitFor(2,TimeUnit.SECONDS))throw new AssertionError("child did not exit");System.out.println("published same positive child PID="+published+" childAlive="+child.isAlive());
  }finally{child.getOutputStream().close();if(child.isAlive())child.destroy();child.waitFor(2,TimeUnit.SECONDS);}
 }
}
