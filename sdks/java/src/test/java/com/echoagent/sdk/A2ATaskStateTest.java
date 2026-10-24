package com.echoagent.sdk;
import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;
class A2ATaskStateTest {
    @Test void retiredValuesAreNotPublicClasses() {
        for (String name : java.util.List.of("A2AMessage", "A2ATask", "AgentCard", "TaskState", "JwtConfig", "JwtClaims"))
            assertThrows(ClassNotFoundException.class, () -> Class.forName("com.echoagent.sdk." + name));
        assertNotNull(EchoAgentClient.class);
    }
}
