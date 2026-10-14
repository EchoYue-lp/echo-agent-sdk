//! Telemetry facade adapter.
//!
//! Telemetry is process-scoped in the framework, so the facade exposes only
//! the explicit init/status/shutdown lifecycle. Exporters and global
//! subscriber state remain owned by `echo_agent::telemetry`; the Host does not
//! create a second metrics or tracing authority.

use echo_sdk_protocol::error::{EchoSdkError, ExtensionErrorCode, Retryability};
use echo_sdk_protocol::methods::FeatureOperationRequest;
use echo_sdk_protocol::scalar::WireValue;

use super::super::{state::CoreProfileState, wire};
use std::sync::Arc;

const METHOD: &str = "_echo_agent/telemetry/op";

fn invalid(message: impl Into<String>) -> EchoSdkError {
    wire::sdk_error(
        ExtensionErrorCode::InvalidValue,
        message,
        Retryability::Never,
        METHOD,
    )
}

fn framework(error: impl Into<String>) -> EchoSdkError {
    wire::sdk_error(
        ExtensionErrorCode::FrameworkError,
        error,
        Retryability::Never,
        METHOD,
    )
}

fn json_arguments(
    request: &FeatureOperationRequest,
) -> Result<Vec<serde_json::Value>, EchoSdkError> {
    request
        .arguments
        .iter()
        .map(|value| {
            value
                .clone()
                .into_json()
                .map_err(|error| invalid(error.to_string()))
        })
        .collect()
}

fn enable_console_of(value: Option<&serde_json::Value>) -> Result<bool, EchoSdkError> {
    match value {
        None | Some(serde_json::Value::Null) => Ok(false),
        Some(serde_json::Value::Bool(value)) => Ok(*value),
        Some(_) => Err(invalid(
            "telemetry enable_console must be a boolean or null",
        )),
    }
}

/// Keep the sole blocking JoinHandle in a collector, independent of the RPC
/// wait. The existing profile close hook drains this original settlement count.
async fn blocking_lifecycle<T: Send + 'static>(
    state: Arc<CoreProfileState>,
    work: impl FnOnce() -> T + Send + 'static,
) -> Result<T, EchoSdkError> {
    let runtime =
        tokio::runtime::Handle::try_current().map_err(|error| framework(error.to_string()))?;
    let services = state
        .services()
        .map_err(|error| framework(error.to_string()))?;
    state.begin_settlement();
    if let Err(error) = services.ensure_admission() {
        state.finish_settlement(Ok(()));
        return Err(wire::sdk_error(
            ExtensionErrorCode::HostShuttingDown,
            error.to_string(),
            Retryability::Never,
            METHOD,
        ));
    }
    // No await occurs between the admission recheck and submission. Closing
    // either rejects this work or observes its already-reserved pending count.
    let job = runtime.spawn_blocking(work);
    let (sender, receiver) = tokio::sync::oneshot::channel();
    runtime.spawn(async move {
        let result = collect_lifecycle_job(state, job).await;
        let _ = sender.send(result);
    });
    receiver
        .await
        .map_err(|error| framework(format!("telemetry lifecycle collector ended: {error}")))?
}

async fn collect_lifecycle_job<T>(
    state: Arc<CoreProfileState>,
    job: tokio::task::JoinHandle<T>,
) -> Result<T, EchoSdkError> {
    let result = job
        .await
        .map_err(|error| framework(format!("telemetry lifecycle job failed: {error}")));
    state.finish_settlement(
        result
            .as_ref()
            .map(|_| ())
            .map_err(|error| error.message.clone()),
    );
    result
}

pub(crate) async fn dispatch(
    state: Arc<CoreProfileState>,
    request: &FeatureOperationRequest,
) -> Result<WireValue, EchoSdkError> {
    let arguments = json_arguments(request)?;
    match request.operation.as_str() {
        "telemetry.init" => {
            let endpoint = arguments
                .first()
                .and_then(serde_json::Value::as_str)
                .filter(|value| !value.trim().is_empty())
                .ok_or_else(|| invalid("telemetry.init requires an OTLP endpoint"))?;
            let service_name = arguments
                .get(1)
                .and_then(serde_json::Value::as_str)
                .filter(|value| !value.trim().is_empty())
                .ok_or_else(|| invalid("telemetry.init requires a service name"))?;
            let enable_console = enable_console_of(arguments.get(2))?;
            let config = echo_agent::telemetry::TelemetryConfig {
                otlp_endpoint: endpoint.to_string(),
                service_name: service_name.to_string(),
                enable_console,
            };
            blocking_lifecycle(state, move || echo_agent::telemetry::init_telemetry(config))
                .await?
                .map_err(|error| framework(error.to_string()))?;
            WireValue::from_json(serde_json::json!({"initialized": true}))
                .map_err(|error| framework(error.to_string()))
        }
        "telemetry.status" => {
            if !arguments.is_empty() {
                return Err(invalid("telemetry.status does not accept arguments"));
            }
            WireValue::from_json(serde_json::json!({
                "initialized": echo_agent::telemetry::Metrics::get().is_some(),
            }))
            .map_err(|error| framework(error.to_string()))
        }
        "telemetry.shutdown" => {
            if !arguments.is_empty() {
                return Err(invalid("telemetry.shutdown does not accept arguments"));
            }
            blocking_lifecycle(state, echo_agent::telemetry::shutdown_telemetry)
                .await?
                .map_err(|error| framework(error.to_string()))?;
            WireValue::from_json(serde_json::json!({"shutdown": true}))
                .map_err(|error| framework(error.to_string()))
        }
        operation => Err(invalid(format!("unknown telemetry operation {operation}"))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn invalid_console_flag_is_rejected_before_process_initialization()
    -> Result<(), Box<dyn std::error::Error>> {
        for flag in [json!("false"), json!(0), json!([]), json!({})] {
            let request = FeatureOperationRequest {
                operation: "telemetry.init".to_string(),
                signature_digest: String::new(),
                handle: None,
                arguments: vec![
                    WireValue::String("http://127.0.0.1:4317".to_string()),
                    WireValue::String("test".to_string()),
                    WireValue::from_json(flag)?,
                ],
            };
            let arguments =
                json_arguments(&request).map_err(|error| std::io::Error::other(error.message))?;
            assert!(
                enable_console_of(arguments.get(2))
                    .is_err_and(|error| error.code == ExtensionErrorCode::InvalidValue)
            );
        }
        assert_eq!(enable_console_of(None).ok(), Some(false));
        assert_eq!(enable_console_of(Some(&json!(null))).ok(), Some(false));
        assert_eq!(enable_console_of(Some(&json!(true))).ok(), Some(true));
        Ok(())
    }
    fn lifecycle_state()
    -> Result<(tempfile::TempDir, Arc<CoreProfileState>), Box<dyn std::error::Error>> {
        let root = tempfile::tempdir()?;
        let config: crate::SdkHostConfig = serde_json::from_value(json!({
            "schema_version":1,"default_agent":{
                "model":{"provider":"fixture","name":"fixture","base_url":"http://127.0.0.1:1/v1","api_protocol":"chat_completions","auth_token":"fixture"},
                "agent":{"name":"fixture","enable_tools":true}
            }
        }))?;
        let prepared = config.prepare()?;
        let state = CoreProfileState::install(
            root.path(),
            crate::config::SdkProfileLimits::default(),
            prepared.framework,
            prepared.llm_client,
        )?;
        let registry = Arc::new(echo_agent::acp::SessionRegistry::new(
            Arc::new(state.session_factory.clone()),
            4,
        ));
        state.bind_services(Arc::new(echo_agent::acp::AcpConnectionServices::new(
            registry,
            Arc::new(echo_agent::acp::AcpAdapterConfig::default()),
        )));
        Ok((root, state))
    }

    async fn retained_blocking_owner(mode: &str) -> Result<(), Box<dyn std::error::Error>> {
        use echo_agent::acp::AcpConnectionProfile as _;
        let (_root, state) = lifecycle_state()?;
        let (entered_sender, entered_receiver) = tokio::sync::oneshot::channel();
        let (release_sender, release_receiver) = std::sync::mpsc::channel();
        let waiter = tokio::spawn({
            let state = state.clone();
            async move {
                blocking_lifecycle(state, move || {
                    let _ = entered_sender.send(());
                    release_receiver.recv().map_err(|error| error.to_string())
                })
                .await
            }
        });
        entered_receiver.await?;
        if mode == "cancel_waiter" {
            waiter.abort();
        }
        let services = state.services()?;
        services.close_admission();
        if mode == "eof" {
            services.extensions().cancel_all();
        }
        let profile = super::super::super::SdkCoreProfile {
            state: state.clone(),
        };
        assert!(
            profile
                .wait_for_settlements(std::time::Duration::from_millis(2))
                .await
                .is_err(),
            "close cannot release an unfinished original blocking job"
        );
        assert!(
            blocking_lifecycle(state.clone(), || ())
                .await
                .is_err_and(|error| error.code == ExtensionErrorCode::HostShuttingDown)
        );
        release_sender.send(())?;
        profile
            .wait_for_settlements(std::time::Duration::from_secs(1))
            .await
            .map_err(std::io::Error::other)?;
        if mode == "cancel_waiter" {
            assert!(waiter.await.is_err());
        } else {
            waiter
                .await?
                .map_err(|error| std::io::Error::other(error.message))??;
        }
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn dropped_rpc_wait_keeps_original_blocking_job_in_close_drain()
    -> Result<(), Box<dyn std::error::Error>> {
        retained_blocking_owner("cancel_waiter").await
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn closed_input_cancellation_does_not_settle_blocking_job()
    -> Result<(), Box<dyn std::error::Error>> {
        retained_blocking_owner("eof").await
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn close_before_blocking_completion_rejects_new_work_until_actual_exit()
    -> Result<(), Box<dyn std::error::Error>> {
        retained_blocking_owner("close").await
    }
    #[tokio::test]
    async fn failed_join_keeps_original_cause_in_close_drain()
    -> Result<(), Box<dyn std::error::Error>> {
        let (_root, state) = lifecycle_state()?;
        state.begin_settlement();
        // An explicitly aborted pending task produces the real JoinError
        // without panicking; this isolates collector error projection.
        let job = tokio::spawn(std::future::pending::<()>());
        job.abort();
        let error = collect_lifecycle_job(state.clone(), job)
            .await
            .err()
            .ok_or_else(|| std::io::Error::other("cancelled join unexpectedly succeeded"))?;
        let close_error = state
            .wait_settlements(std::time::Duration::from_secs(1))
            .await
            .err()
            .ok_or_else(|| std::io::Error::other("close discarded the join failure"))?;
        assert_eq!(close_error, error.message);
        assert!(close_error.contains("telemetry lifecycle job failed"));
        Ok(())
    }
}
