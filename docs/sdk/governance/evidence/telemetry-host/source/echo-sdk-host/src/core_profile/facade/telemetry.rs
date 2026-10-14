//! Telemetry facade adapter.
//!
//! Telemetry is process-scoped in the framework, so the facade exposes only
//! the explicit init/status/shutdown lifecycle. Exporters and global
//! subscriber state remain owned by `echo_agent::telemetry`; the Host does not
//! create a second metrics or tracing authority.

use echo_sdk_protocol::error::{EchoSdkError, ExtensionErrorCode, Retryability};
use echo_sdk_protocol::methods::FeatureOperationRequest;
use echo_sdk_protocol::scalar::WireValue;

use super::super::wire;

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

pub(crate) fn dispatch(request: &FeatureOperationRequest) -> Result<WireValue, EchoSdkError> {
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
            echo_agent::telemetry::init_telemetry(echo_agent::telemetry::TelemetryConfig {
                otlp_endpoint: endpoint.to_string(),
                service_name: service_name.to_string(),
                enable_console,
            })
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
            echo_agent::telemetry::shutdown_telemetry()
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
            assert!(
                dispatch(&request)
                    .is_err_and(|error| error.code == ExtensionErrorCode::InvalidValue)
            );
        }
        assert_eq!(enable_console_of(None).ok(), Some(false));
        assert_eq!(enable_console_of(Some(&json!(null))).ok(), Some(false));
        assert_eq!(enable_console_of(Some(&json!(true))).ok(), Some(true));
        Ok(())
    }
}
