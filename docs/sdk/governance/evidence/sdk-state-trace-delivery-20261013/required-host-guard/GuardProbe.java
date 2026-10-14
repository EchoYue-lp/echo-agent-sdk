public class GuardProbe {
    public static void main(String[] args) throws Exception {
        Class<?> type = Class.forName("com.echoagent.sdk.StateTraceDeliveryHostTest");
        var constructor = type.getDeclaredConstructor(); constructor.setAccessible(true);
        var method = type.getDeclaredMethod("stateFullFieldsFaultAndRealHostRestart"); method.setAccessible(true);
        try { method.invoke(constructor.newInstance()); throw new AssertionError("required missing Host did not fail"); }
        catch (java.lang.reflect.InvocationTargetException error) {
            if (!(error.getCause() instanceof AssertionError)
                    || !error.getCause().getMessage().contains("required State/Trace/Delivery Host binary missing")) throw error;
            System.out.println("Required Host guard failed explicitly as expected.");
        }
    }
}
