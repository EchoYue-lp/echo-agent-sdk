package com.echoagent.sdk;
import java.util.concurrent.CompletionStage;
/** Typed reverse callback with transport cancellation supplied by the client. */
@FunctionalInterface
public interface CallbackHandler {
    CompletionStage<? extends ExtensionOutcome> handle(CallbackCall call, ExtensionCancellation cancellation);
}
