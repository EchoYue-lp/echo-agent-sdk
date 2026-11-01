package com.echoagent.sdk;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertThrows;

class A2ATaskStateTest {
    @Test
    void retiredA2AClassesAreAbsent() {
        for (String name : new String[]{"A2AMessage", "A2ATask", "AgentCard", "TaskState", "AgentAuthentication", "AgentCapabilities", "AgentProvider", "AgentSkill", "AuthenticationScheme", "TaskArtifactUpdateEvent", "TaskStatusUpdateEvent"}) {
            assertThrows(ClassNotFoundException.class, () -> Class.forName("com.echoagent.sdk." + name));
        }
    }
}
