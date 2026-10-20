//! Product-neutral, source-built standard ACP v1 Host for `echo-agent`.
//!
//! The crate owns configuration and Session Agent construction only. The root
//! framework adapter remains the authority for ACP methods, Session state,
//! cancellation, event projection, and bounded shutdown.
//!
//! With the `sdk-core-profile` feature and an explicit `sdk_profile` config
//! section, the Host additionally negotiates the `_echo_agent/*` core
//! profile over the same connection: generation-fenced Agent/Session/Run/
//! Stream handles, typed RPC, ACK-bounded live events, durable replay and
//! restart recovery under the configured state root. Without that section
//! the Host is standard-only and never advertises the extension.

// The typed `EchoSdkError` wire envelope is returned by value throughout the
// core profile handlers. It is a small, fixed-shape in-process DTO (never a
// heavyweight boxed tree), and boxing it at every handler boundary would
// churn the host code without changing behavior: these results never cross a
// hot loop and the process is local and single-tenant.
#![allow(clippy::result_large_err)]

mod config;
#[cfg(feature = "runtime")]
mod factory;
/// Compiled feature authority: the initialize advertisement derives from
/// this table, never from runtime config (design §13).
pub mod features;
#[cfg(feature = "runtime")]
mod mcp;

#[cfg(feature = "sdk-core-profile")]
mod bounded_stdio;
#[cfg(feature = "sdk-core-profile")]
mod core_profile;

pub use config::{
    HOST_CONFIG_SCHEMA_VERSION, HostCommand, MAX_HOST_CONFIG_BYTES, SdkHostConfig,
    SdkProfileConfig, SdkProfileLimits, parse_args,
};

#[cfg(feature = "sdk-core-profile")]
pub use core_profile::SdkCoreProfile;

#[cfg(feature = "runtime")]
use agent_client_protocol::{ConnectTo as _, Stdio};
#[cfg(feature = "sdk-core-profile")]
use echo_agent::acp::AcpAdapterConfig;
#[cfg(feature = "runtime")]
use echo_agent::acp::AcpAgentAdapter;
use std::fmt;
#[cfg(feature = "runtime")]
use std::io::Write as _;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Bounded startup/runtime error reported by the Host executable.
pub enum HostError {
    /// Invalid command-line input.
    Argument(String),
    /// Invalid Host configuration.
    Config(String),
    /// Configuration file access failed.
    ConfigFile { path: PathBuf, message: String },
    /// The official ACP connection failed after startup.
    Runtime(String),
}

impl fmt::Display for HostError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Argument(message) | Self::Config(message) | Self::Runtime(message) => {
                formatter.write_str(message)
            }
            Self::ConfigFile { path, message } => {
                write!(
                    formatter,
                    "failed to read Host config {}: {message}",
                    path.display()
                )
            }
        }
    }
}

impl std::error::Error for HostError {}

/// Validate a Host JSON file without opening ACP stdio.
pub fn validate_config(path: impl AsRef<std::path::Path>) -> Result<(), HostError> {
    SdkHostConfig::from_path(path)?.validate()
}

#[cfg(all(test, feature = "runtime"))]
type BeforeActivationHook = Box<dyn FnOnce() + Send>;

/// Cleanup ownership for one in-process Host connection.
///
/// Create and retain this owner before polling `run_stdio_with_close_owner`.
/// Dropping or timing out the run future does not drop the framework's close
/// owner. Call `close` until it succeeds. An owner is single-use; repeated
/// close after settlement is idempotent. This is a Rust/Host API only.
#[cfg(feature = "runtime")]
#[derive(Clone, Default)]
pub struct HostCloseOwner {
    retained: std::sync::Arc<std::sync::Mutex<Option<echo_agent::acp::AcpAdapterCloseHandle>>>,
    attached: std::sync::Arc<std::sync::atomic::AtomicBool>,
    close_gate: std::sync::Arc<tokio::sync::Mutex<()>>,
    stopping: std::sync::Arc<std::sync::atomic::AtomicBool>,
    actor_stop: std::sync::Arc<std::sync::atomic::AtomicBool>,
    stopped: std::sync::Arc<tokio::sync::Notify>,
    activated: std::sync::Arc<std::sync::atomic::AtomicBool>,
    activation: std::sync::Arc<tokio::sync::Notify>,
    actor: std::sync::Arc<std::sync::Mutex<Option<tokio::task::AbortHandle>>>,
    poll_started: std::sync::Arc<std::sync::atomic::AtomicBool>,
    actor_result: std::sync::Arc<std::sync::Mutex<Option<Result<(), HostError>>>>,
    actor_settled: std::sync::Arc<tokio::sync::Notify>,
    #[cfg(test)]
    before_activation: std::sync::Arc<std::sync::Mutex<Option<BeforeActivationHook>>>,
    #[cfg(test)]
    before_poll_wait: std::sync::Arc<std::sync::Mutex<Option<std::sync::Arc<tokio::sync::Notify>>>>,
    #[cfg(test)]
    automatic_close_failed: std::sync::Arc<tokio::sync::Notify>,
}

#[cfg(feature = "runtime")]
impl HostCloseOwner {
    pub fn new() -> Self {
        Self::default()
    }

    fn retain(&self, owner: echo_agent::acp::AcpAdapterCloseHandle) -> Result<(), HostError> {
        let mut retained = self.retained.lock().unwrap_or_else(|e| e.into_inner());
        if self
            .attached
            .swap(true, std::sync::atomic::Ordering::AcqRel)
        {
            return Err(HostError::Runtime(
                "Host close owner is already attached".to_string(),
            ));
        }
        *retained = Some(owner);
        Ok(())
    }

    async fn wait_stopped(&self) {
        loop {
            let stopped = self.stopped.notified();
            tokio::pin!(stopped);
            stopped.as_mut().enable();
            if self.actor_stop.load(std::sync::atomic::Ordering::Acquire) {
                return;
            }
            stopped.await;
        }
    }

    async fn wait_activated(&self) {
        loop {
            let activated = self.activation.notified();
            tokio::pin!(activated);
            activated.as_mut().enable();
            if self.activated.load(std::sync::atomic::Ordering::Acquire) {
                return;
            }
            activated.await;
        }
    }

    async fn wait_actor_result(&self) -> Result<(), HostError> {
        loop {
            let changed = self.actor_settled.notified();
            tokio::pin!(changed);
            changed.as_mut().enable();
            let result = self
                .actor_result
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .clone();
            if let Some(result) = result {
                return result;
            }
            changed.await;
        }
    }

    /// Settle the framework connection owner. Cancellation and errors retain
    /// the same handle; successful close releases it exactly once.
    pub async fn close(&self) -> Result<(), HostError> {
        self.stopping
            .store(true, std::sync::atomic::Ordering::Release);
        let _attempt = self.close_gate.lock().await;
        let owner = {
            let retained = self.retained.lock().unwrap_or_else(|e| e.into_inner());
            // Closing a scope before the run is polled fences later admission.
            self.attached
                .store(true, std::sync::atomic::Ordering::Release);
            retained.clone()
        };
        if let Some(owner) = owner {
            self.wait_activated().await;
            if !self.poll_started.load(std::sync::atomic::Ordering::Acquire) {
                // A joined actor that never polled ConnectTo admitted no
                // framework resources. Report its actual failure and release
                // this unbound handle instead of waiting forever for bind.
                let result = self.wait_actor_result().await;
                self.actor.lock().unwrap_or_else(|e| e.into_inner()).take();
                *self.retained.lock().unwrap_or_else(|e| e.into_inner()) = None;
                return result;
            }
            owner
                .close()
                .await
                .map_err(|e| HostError::Runtime(e.to_string()))?;
            // Keep the official connection actor polling until the framework
            // has settled every driver/callback and closed its Agents.
            self.actor_stop
                .store(true, std::sync::atomic::Ordering::Release);
            self.stopped.notify_waiters();
            let transport_result = self.wait_actor_result().await;
            self.actor.lock().unwrap_or_else(|e| e.into_inner()).take();
            *self.retained.lock().unwrap_or_else(|e| e.into_inner()) = None;
            return transport_result;
        }
        Ok(())
    }
}

#[cfg(feature = "runtime")]
async fn connect_owned(
    owner: &HostCloseOwner,
    framework_owner: echo_agent::acp::AcpAdapterCloseHandle,
    connection: impl std::future::Future<Output = agent_client_protocol::Result<()>> + Send + 'static,
) -> Result<(), HostError> {
    let runtime = tokio::runtime::Handle::try_current().map_err(|error| {
        HostError::Runtime(format!(
            "Host connection requires an active Tokio runtime: {error}"
        ))
    })?;
    owner.retain(framework_owner)?;
    let actor_owner = owner.clone();
    // The official builder composes a large connection future. Heap-pin it
    // before transferring to Tokio so it is never embedded in an actor's
    // stack frame (stdio previously happened to run on the main stack).
    let mut connection = Box::pin(connection);
    let actor = runtime.spawn(async move {
        #[cfg(test)]
        if let Some(before_activation) = actor_owner
            .before_activation
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .take()
        {
            before_activation();
        }
        #[cfg(test)]
        {
            let paused = actor_owner
                .before_poll_wait
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .clone();
            if let Some(paused) = paused {
                paused.notified().await;
            }
        }
        let initial = std::future::poll_fn(|context| {
            actor_owner
                .poll_started
                .store(true, std::sync::atomic::Ordering::Release);
            let polled = connection.as_mut().poll(context);
            actor_owner
                .activated
                .store(true, std::sync::atomic::Ordering::Release);
            actor_owner.activation.notify_waiters();
            std::task::Poll::Ready(polled)
        })
        .await;
        match initial {
            std::task::Poll::Ready(result) => result,
            std::task::Poll::Pending => tokio::select! {
                result = &mut connection => result,
                _ = actor_owner.wait_stopped() => Ok(()),
            },
        }
        .map_err(|e| HostError::Runtime(e.to_string()))
    });
    *owner.actor.lock().unwrap_or_else(|e| e.into_inner()) = Some(actor.abort_handle());
    let collector_owner = owner.clone();
    // Exactly one collector consumes the real actor JoinHandle. Panic and
    // abort therefore publish a transport failure and wake all waiters too.
    runtime.spawn(async move {
        let result = match actor.await {
            Ok(result) => result,
            Err(error) => Err(HostError::Runtime(format!(
                "Host connection actor failed: {error}"
            ))),
        };
        *collector_owner
            .actor_result
            .lock()
            .unwrap_or_else(|e| e.into_inner()) = Some(result);
        collector_owner
            .activated
            .store(true, std::sync::atomic::Ordering::Release);
        collector_owner.activation.notify_waiters();
        collector_owner.actor_settled.notify_waiters();
    });
    let connected = owner.wait_actor_result().await;
    owner.close().await?;
    connected
}

#[cfg(feature = "sdk-core-profile")]
fn core_adapter_config(limits: &SdkProfileLimits) -> AcpAdapterConfig {
    AcpAdapterConfig {
        max_extension_concurrency: limits
            .max_callback_concurrency
            .min(limits.max_extension_invocations),
        shutdown_timeout: std::time::Duration::from_secs(limits.shutdown_timeout_secs),
        ..AcpAdapterConfig::default()
    }
}

#[cfg(feature = "runtime")]
struct HostRunDropGuard {
    owner: Option<HostCloseOwner>,
}

#[cfg(feature = "runtime")]
impl HostRunDropGuard {
    fn new(owner: HostCloseOwner) -> Self {
        Self { owner: Some(owner) }
    }
    fn disarm(&mut self) {
        self.owner = None;
    }
}

#[cfg(feature = "runtime")]
impl Drop for HostRunDropGuard {
    fn drop(&mut self) {
        let Some(owner) = self.owner.take() else {
            return;
        };
        let runtime = match tokio::runtime::Handle::try_current() {
            Ok(runtime) => runtime,
            Err(error) => {
                let _ = writeln!(
                    std::io::stderr(),
                    "Host cancellation cleanup cannot start: {error}"
                );
                return;
            }
        };
        runtime.spawn(async move {
            if let Err(error) = owner.close().await {
                #[cfg(test)]
                owner.automatic_close_failed.notify_one();
                let _ = writeln!(
                    std::io::stderr(),
                    "Host cancellation cleanup remains pending: {error}"
                );
                // A timed-out close retains the framework's sole Run owner.
                // Keep its original actor polling and retry the same close
                // handle: cooperative settlement need not cause transport EOF.
                // Pacing bounds attempts when an external cleanup stays stuck;
                // each attempt still uses the framework's configured deadline.
                let mut retry_delay = std::time::Duration::from_millis(100);
                let mut failed_attempts = 1_u64;
                let mut last_error = error.to_string();
                loop {
                    tokio::time::sleep(retry_delay).await;
                    match owner.close().await {
                        Ok(()) => break,
                        Err(error) => {
                            failed_attempts = failed_attempts.saturating_add(1);
                            let current_error = error.to_string();
                            // Report changed failures immediately and sample
                            // identical failures at exponentially spaced counts.
                            if current_error != last_error || failed_attempts.is_power_of_two() {
                                let _ = writeln!(
                                    std::io::stderr(),
                                    "Host cancellation cleanup remains pending after {failed_attempts} attempts: {error}"
                                );
                            }
                            last_error = current_error;
                            retry_delay = retry_delay
                                .saturating_mul(2)
                                .min(std::time::Duration::from_secs(1));
                        }
                    }
                }
            }
        });
    }
}

#[cfg(feature = "runtime")]
async fn run_with_automatic_cleanup(
    owner: HostCloseOwner,
    run: impl std::future::Future<Output = Result<(), HostError>>,
) -> Result<(), HostError> {
    let mut cleanup = HostRunDropGuard::new(owner);
    let result = run.await;
    if result.is_ok() {
        cleanup.disarm();
    }
    result
}

/// Run the configured standard ACP v1 Agent on the official stdio transport.
/// When the config carries an `sdk_profile` section (and the Host was built
/// with `sdk-core-profile`), the negotiated core profile rides the same
/// official connection and bounded stdio transport.
#[cfg(feature = "runtime")]
pub async fn run_stdio(path: impl AsRef<std::path::Path>) -> Result<(), HostError> {
    let owner = HostCloseOwner::new();
    run_with_automatic_cleanup(owner.clone(), run_stdio_with_close_owner(path, &owner)).await
}

/// Serve stdio while retaining cancellation-safe cleanup in a caller-owned
/// scope. Configuration preparation performs no async resource admission;
/// all connection resources enter the retained framework owner before await.
#[cfg(feature = "runtime")]
pub async fn run_stdio_with_close_owner(
    path: impl AsRef<std::path::Path>,
    owner: &HostCloseOwner,
) -> Result<(), HostError> {
    if owner.stopping.load(std::sync::atomic::Ordering::Acquire) {
        return Err(HostError::Runtime(
            "Host close owner is already closed".to_string(),
        ));
    }
    let prepared = SdkHostConfig::from_path(path)?.prepare()?;
    match prepared.sdk_profile {
        #[cfg(feature = "sdk-core-profile")]
        Some(profile_config) => {
            let profile = core_profile::SdkCoreProfile::install_with_llm_config(
                &profile_config.state_root,
                profile_config.limits,
                prepared.framework,
                prepared.llm_client,
                Some(prepared.llm_config),
            )?;
            // The standard Session factory shares the profile's default
            // definition, so standard `session/new` and
            // `_echo_agent/session/create` construct Sessions from the same
            // definition authority.
            let factory = profile.session_factory();
            let transport = bounded_stdio::bounded_stdio(profile_config.limits.max_frame_bytes);
            let adapter =
                AcpAgentAdapter::with_config(factory, core_adapter_config(&profile_config.limits))
                    .map_err(|error| HostError::Config(error.to_string()))?;
            let adapter = adapter.with_profile(profile);
            connect_owned(owner, adapter.close_owner(), adapter.connect_to(transport)).await
        }
        #[cfg(not(feature = "sdk-core-profile"))]
        Some(profile_config) => Err(HostError::Config(format!(
            "sdk_profile requires a Host built with the sdk-core-profile feature \
             (state root {} was configured)",
            profile_config.state_root.display()
        ))),
        None => {
            let factory = factory::DefaultHostSessionFactory::new_configured(
                prepared.framework,
                prepared.llm_client,
                prepared.llm_config,
            );
            let adapter = AcpAgentAdapter::new(factory);
            connect_owned(
                owner,
                adapter.close_owner(),
                adapter.connect_to(Stdio::new()),
            )
            .await
        }
    }
}

/// Help text printed only when the executable is not serving ACP.
pub const HELP: &str = "echo-agent-sdk-host\n\nUSAGE:\n  echo-agent-sdk-host --config <path> [--check-config]\n  echo-agent-sdk-host --help\n  echo-agent-sdk-host --version\n";

/// Version of the source-built Host crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(all(test, feature = "runtime"))]
mod ownership_tests {
    use super::*;
    use agent_client_protocol::{
        Channel, Client,
        schema::{ProtocolVersion, v1},
    };
    use echo_agent::agent::{Agent, AgentEvent};
    use futures::{StreamExt as _, future::BoxFuture, stream::BoxStream};
    use std::sync::{
        Arc,
        atomic::{AtomicBool, AtomicUsize, Ordering},
    };

    #[derive(Default)]
    struct CleanupProbe {
        attempts: AtomicUsize,
        settled: AtomicBool,
        fail: AtomicBool,
        stalled: AtomicBool,
        release: tokio::sync::Notify,
    }

    struct CleanupAgent(Arc<CleanupProbe>);

    impl Agent for CleanupAgent {
        fn name(&self) -> &str {
            "sdk-cleanup-fixture"
        }
        fn model_name(&self) -> &str {
            "fixture"
        }
        fn system_prompt(&self) -> &str {
            "fixture"
        }
        fn execute<'a>(&'a self, _: &'a str) -> BoxFuture<'a, echo_agent::error::Result<String>> {
            Box::pin(async { Ok("done".to_string()) })
        }
        fn execute_stream<'a>(
            &'a self,
            _: &'a str,
        ) -> BoxFuture<
            'a,
            echo_agent::error::Result<BoxStream<'a, echo_agent::error::Result<AgentEvent>>>,
        > {
            Box::pin(async { Ok(futures::stream::empty().boxed()) })
        }
        fn close(&self) -> BoxFuture<'_, echo_agent::error::Result<()>> {
            Box::pin(async move {
                self.0.attempts.fetch_add(1, Ordering::AcqRel);
                if self.0.fail.load(Ordering::Acquire) {
                    return Err(echo_agent::error::ReactError::Other(
                        "fixture close failed".to_string(),
                    ));
                }
                while self.0.stalled.load(Ordering::Acquire) {
                    self.0.release.notified().await;
                }
                self.0.settled.store(true, Ordering::Release);
                Ok(())
            })
        }
    }

    #[derive(Default)]
    struct CapturedRunProfile {
        run: Arc<std::sync::Mutex<Option<Arc<echo_agent::acp::RunEntry>>>>,
        spawned: Arc<tokio::sync::Notify>,
    }
    impl echo_agent::acp::AcpConnectionProfile for CapturedRunProfile {
        fn negotiate_hello(&self, _: &serde_json::Value) -> Result<(), String> {
            Err("standard".to_string())
        }
        fn run_spawned(&self, entry: Arc<echo_agent::acp::RunEntry>) {
            *self.run.lock().unwrap_or_else(|e| e.into_inner()) = Some(entry);
            self.spawned.notify_one();
        }
        fn attach(
            &self,
            _: Arc<echo_agent::acp::AcpConnectionServices>,
        ) -> agent_client_protocol::Builder<
            agent_client_protocol::Agent,
            impl agent_client_protocol::HandleDispatchFrom<Client>,
            impl agent_client_protocol::RunWithConnectionTo<Client>,
            impl agent_client_protocol::HandleConnectionClose<Client>,
            agent_client_protocol::RawConnectionContext,
        > {
            agent_client_protocol::Agent.builder()
        }
    }

    #[derive(Default)]
    struct ActiveRunProbe {
        started: tokio::sync::Notify,
        cancel_started: tokio::sync::Notify,
        release_cleanup: tokio::sync::Notify,
        cleanup_done: AtomicBool,
        close_done: AtomicBool,
    }
    struct ParkedCooperativeAgent(Arc<ActiveRunProbe>);
    impl Agent for ParkedCooperativeAgent {
        fn name(&self) -> &str {
            "parked-cooperative"
        }
        fn model_name(&self) -> &str {
            "fixture"
        }
        fn system_prompt(&self) -> &str {
            "fixture"
        }
        fn execute<'a>(&'a self, _: &'a str) -> BoxFuture<'a, echo_agent::error::Result<String>> {
            Box::pin(async { Ok(String::new()) })
        }
        fn execute_stream<'a>(
            &'a self,
            _: &'a str,
        ) -> BoxFuture<
            'a,
            echo_agent::error::Result<BoxStream<'a, echo_agent::error::Result<AgentEvent>>>,
        > {
            Box::pin(async { Ok(futures::stream::empty().boxed()) })
        }
        fn chat_stream_with_cancel<'a>(
            &'a self,
            _: &'a str,
            cancel: tokio_util::sync::CancellationToken,
        ) -> BoxFuture<
            'a,
            echo_agent::error::Result<BoxStream<'a, echo_agent::error::Result<AgentEvent>>>,
        > {
            Box::pin(async move {
                Ok(futures::stream::once(async move {
                    self.0.started.notify_one();
                    cancel.cancelled().await;
                    self.0.cancel_started.notify_one();
                    self.0.release_cleanup.notified().await;
                    self.0.cleanup_done.store(true, Ordering::Release);
                    Ok(AgentEvent::Cancelled)
                })
                .boxed())
            })
        }
        fn close(&self) -> BoxFuture<'_, echo_agent::error::Result<()>> {
            Box::pin(async move {
                if !self.0.cleanup_done.load(Ordering::Acquire) {
                    return Err(echo_agent::error::ReactError::Other(
                        "active cleanup is not settled".to_string(),
                    ));
                }
                self.0.close_done.store(true, Ordering::Release);
                Ok(())
            })
        }
    }

    async fn prove_active_actor_survives_close_failure_and_caller_abort(
        abort_caller: bool,
        abort_actor: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let probe = Arc::new(ActiveRunProbe::default());
        let profile = CapturedRunProfile::default();
        let run_slot = profile.run.clone();
        let adapter = AcpAgentAdapter::with_config(
            {
                let probe = probe.clone();
                move |_: echo_agent::acp::AcpSessionContext| {
                    let probe = probe.clone();
                    async move { Ok(Box::new(ParkedCooperativeAgent(probe)) as Box<dyn Agent>) }
                }
            },
            echo_agent::acp::AcpAdapterConfig {
                shutdown_timeout: std::time::Duration::from_millis(150),
                ..echo_agent::acp::AcpAdapterConfig::default()
            },
        )?
        .with_profile(profile);
        let owner = Arc::new(HostCloseOwner::new());
        let (agent_transport, client_transport) = Channel::duplex();
        let run = tokio::spawn({
            let owner = owner.clone();
            async move {
                connect_owned(
                    &owner,
                    adapter.close_owner(),
                    adapter.connect_to(agent_transport),
                )
                .await
            }
        });
        let directory = tempfile::tempdir()?;
        let cwd = directory.path().to_path_buf();
        let client = tokio::spawn(async move {
            Client
                .builder()
                .connect_with(client_transport, async move |connection| {
                    connection
                        .send_request(v1::InitializeRequest::new(ProtocolVersion::V1))
                        .block_task()
                        .await?;
                    let session = connection
                        .send_request(v1::NewSessionRequest::new(cwd))
                        .block_task()
                        .await?
                        .session_id;
                    connection
                        .send_request(v1::PromptRequest::new(
                            session,
                            vec![v1::ContentBlock::Text(v1::TextContent::new(
                                "park until cancel",
                            ))],
                        ))
                        .block_task()
                        .await?;
                    std::future::pending::<agent_client_protocol::Result<()>>().await
                })
                .await
        });
        tokio::time::timeout(std::time::Duration::from_secs(2), probe.started.notified()).await?;
        let entry = run_slot
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .clone()
            .ok_or("framework Run entry not captured")?;
        if abort_actor {
            owner
                .actor
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .as_ref()
                .ok_or("actor abort handle absent")?
                .abort();
            assert!(
                tokio::time::timeout(std::time::Duration::from_secs(2), owner.wait_actor_result())
                    .await?
                    .is_err()
            );
            assert!(
                tokio::time::timeout(std::time::Duration::from_secs(2), owner.close())
                    .await?
                    .is_err()
            );
            assert!(entry.receipt().is_none());
            assert!(!probe.close_done.load(Ordering::Acquire));
            assert!(
                owner
                    .retained
                    .lock()
                    .unwrap_or_else(|e| e.into_inner())
                    .is_some()
            );
            run.abort();
            let _ = run.await;
            client.abort();
            let _ = client.await;
            return Ok(());
        }
        if abort_caller {
            run.abort();
        }
        assert!(
            tokio::time::timeout(std::time::Duration::from_secs(2), owner.close())
                .await?
                .is_err()
        );
        tokio::time::timeout(
            std::time::Duration::from_secs(2),
            probe.cancel_started.notified(),
        )
        .await?;
        assert!(entry.receipt().is_none());
        assert!(
            owner
                .retained
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .is_some()
        );
        probe.release_cleanup.notify_one();
        tokio::time::timeout(std::time::Duration::from_secs(2), owner.close()).await??;
        let receipt = entry
            .receipt()
            .ok_or("active Run never produced a terminal receipt")?;
        assert!(matches!(
            receipt.outcome,
            echo_agent::runtime::TurnOutcome::Cancelled
        ));
        assert!(probe.cleanup_done.load(Ordering::Acquire));
        assert!(probe.close_done.load(Ordering::Acquire));
        assert!(
            owner
                .retained
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .is_none()
        );
        owner.close().await?;
        let result = tokio::time::timeout(std::time::Duration::from_secs(2), run).await?;
        if abort_caller {
            assert!(result.is_err_and(|e| e.is_cancelled()));
        } else {
            result??;
        }
        client.abort();
        let _ = client.await;
        Ok(())
    }

    #[tokio::test]
    async fn manual_close_keeps_active_run_actor_until_cleanup_and_receipt_settle()
    -> Result<(), Box<dyn std::error::Error>> {
        prove_active_actor_survives_close_failure_and_caller_abort(false, false).await
    }
    #[tokio::test]
    async fn caller_abort_keeps_active_run_actor_for_retryable_close()
    -> Result<(), Box<dyn std::error::Error>> {
        prove_active_actor_survives_close_failure_and_caller_abort(true, false).await
    }

    #[tokio::test]
    async fn actor_abort_after_active_run_reports_failure_without_inventing_receipt()
    -> Result<(), Box<dyn std::error::Error>> {
        prove_active_actor_survives_close_failure_and_caller_abort(false, true).await
    }

    #[test]
    fn missing_tokio_runtime_is_an_explicit_host_error_before_actor_admission() {
        let owner = HostCloseOwner::new();
        let result = futures::executor::block_on(connect_owned(
            &owner,
            echo_agent::acp::AcpAdapterCloseHandle::default(),
            async { Ok(()) },
        ));
        assert!(
            matches!(result, Err(HostError::Runtime(message)) if message.contains("active Tokio runtime"))
        );
        assert!(!owner.attached.load(Ordering::Acquire));
    }

    #[tokio::test]
    async fn actor_abort_before_first_poll_settles_waiters_without_admitting_resources()
    -> Result<(), Box<dyn std::error::Error>> {
        let owner = Arc::new(HostCloseOwner::new());
        let paused = Arc::new(tokio::sync::Notify::new());
        *owner
            .before_poll_wait
            .lock()
            .unwrap_or_else(|e| e.into_inner()) = Some(paused);
        let entered = Arc::new(tokio::sync::Notify::new());
        *owner
            .before_activation
            .lock()
            .unwrap_or_else(|e| e.into_inner()) = Some(Box::new({
            let entered = entered.clone();
            move || entered.notify_one()
        }));
        let adapter = AcpAgentAdapter::new(|_: echo_agent::acp::AcpSessionContext| async {
            Ok(Box::new(CleanupAgent(Arc::new(CleanupProbe::default()))) as Box<dyn Agent>)
        });
        let (agent_transport, _client) = Channel::duplex();
        let run = tokio::spawn({
            let owner = owner.clone();
            async move {
                connect_owned(
                    &owner,
                    adapter.close_owner(),
                    adapter.connect_to(agent_transport),
                )
                .await
            }
        });
        tokio::time::timeout(std::time::Duration::from_secs(2), entered.notified()).await?;
        let abort = owner
            .actor
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .clone()
            .ok_or("actor abort handle absent")?;
        abort.abort();
        let result = tokio::time::timeout(std::time::Duration::from_secs(2), run).await??;
        assert!(matches!(result,Err(HostError::Runtime(message)) if message.contains("cancelled")));
        assert!(
            owner
                .retained
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .is_none()
        );
        owner.close().await?;
        Ok(())
    }

    #[tokio::test]
    async fn actor_panic_before_bind_publishes_join_error_and_wakes_close()
    -> Result<(), Box<dyn std::error::Error>> {
        let owner = HostCloseOwner::new();
        // Controlled test-only unwind exercises Tokio's real JoinError path.
        *owner
            .before_activation
            .lock()
            .unwrap_or_else(|e| e.into_inner()) = Some(Box::new(|| {
            std::panic::resume_unwind(Box::new("injected actor unwind"))
        }));
        let adapter = AcpAgentAdapter::new(|_: echo_agent::acp::AcpSessionContext| async {
            Ok(Box::new(CleanupAgent(Arc::new(CleanupProbe::default()))) as Box<dyn Agent>)
        });
        let (agent_transport, _client) = Channel::duplex();
        let result = tokio::time::timeout(
            std::time::Duration::from_secs(2),
            connect_owned(
                &owner,
                adapter.close_owner(),
                adapter.connect_to(agent_transport),
            ),
        )
        .await?;
        assert!(matches!(result,Err(HostError::Runtime(message)) if message.contains("panicked")));
        assert!(
            owner
                .retained
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .is_none()
        );
        owner.close().await?;
        Ok(())
    }

    async fn prove_convenience_cleanup_without_transport_eof(
        release_after_failed_close: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let probe = Arc::new(ActiveRunProbe::default());
        let profile = CapturedRunProfile::default();
        let run_slot = profile.run.clone();
        let adapter = AcpAgentAdapter::with_config(
            {
                let probe = probe.clone();
                move |_: echo_agent::acp::AcpSessionContext| {
                    let probe = probe.clone();
                    async move { Ok(Box::new(ParkedCooperativeAgent(probe)) as Box<dyn Agent>) }
                }
            },
            echo_agent::acp::AcpAdapterConfig {
                shutdown_timeout: std::time::Duration::from_millis(150),
                ..echo_agent::acp::AcpAdapterConfig::default()
            },
        )?
        .with_profile(profile);
        let owner = HostCloseOwner::new();
        let weak = Arc::downgrade(&owner.retained);
        let first_close_failed = owner.automatic_close_failed.clone();
        let (agent_transport, client_transport) = Channel::duplex();
        let run = tokio::spawn(async move {
            run_with_automatic_cleanup(
                owner.clone(),
                connect_owned(
                    &owner,
                    adapter.close_owner(),
                    adapter.connect_to(agent_transport),
                ),
            )
            .await
        });
        let directory = tempfile::tempdir()?;
        let cwd = directory.path().to_path_buf();
        let client = tokio::spawn(async move {
            Client
                .builder()
                .connect_with(client_transport, async move |connection| {
                    connection
                        .send_request(v1::InitializeRequest::new(ProtocolVersion::V1))
                        .block_task()
                        .await?;
                    let session = connection
                        .send_request(v1::NewSessionRequest::new(cwd))
                        .block_task()
                        .await?
                        .session_id;
                    connection
                        .send_request(v1::PromptRequest::new(
                            session,
                            vec![v1::ContentBlock::Text(v1::TextContent::new(
                                "active cancellation",
                            ))],
                        ))
                        .block_task()
                        .await?;
                    std::future::pending::<agent_client_protocol::Result<()>>().await
                })
                .await
        });
        tokio::time::timeout(std::time::Duration::from_secs(2), probe.started.notified()).await?;
        let entry = run_slot
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .clone()
            .ok_or("no framework active Run")?;
        run.abort();
        let _ = run.await;
        tokio::time::timeout(
            std::time::Duration::from_secs(2),
            probe.cancel_started.notified(),
        )
        .await?;
        if release_after_failed_close {
            tokio::time::timeout(
                std::time::Duration::from_secs(2),
                first_close_failed.notified(),
            )
            .await?;
            assert!(entry.receipt().is_none());
            assert!(!probe.close_done.load(Ordering::Acquire));
            assert!(weak.upgrade().is_some());
            assert!(!client.is_finished());
        }
        probe.release_cleanup.notify_one();
        tokio::time::timeout(std::time::Duration::from_secs(2), async {
            while entry.receipt().is_none() {
                tokio::task::yield_now().await;
            }
        })
        .await?;
        assert!(probe.cleanup_done.load(Ordering::Acquire));
        assert!(matches!(
            entry
                .receipt()
                .ok_or("missing framework cancellation receipt")?
                .outcome,
            echo_agent::runtime::TurnOutcome::Cancelled
        ));
        tokio::time::timeout(std::time::Duration::from_secs(2), async {
            loop {
                if probe.close_done.load(Ordering::Acquire) && weak.upgrade().is_none() {
                    break;
                }
                tokio::task::yield_now().await;
            }
        })
        .await.map_err(|_| format!(
            "framework Cancelled receipt exists, but convenience cleanup did not finish: agent_closed={}, owner_retained={}, client_finished={}",
            probe.close_done.load(Ordering::Acquire), weak.upgrade().is_some(), client.is_finished()
        ))?;
        assert!(matches!(
            entry
                .receipt()
                .ok_or("missing cancellation receipt")?
                .outcome,
            echo_agent::runtime::TurnOutcome::Cancelled
        ));
        assert!(probe.cleanup_done.load(Ordering::Acquire));
        client.abort();
        let _ = client.await;
        Ok(())
    }

    #[tokio::test]
    async fn convenience_future_abort_closes_active_actor_without_caller_owned_scope()
    -> Result<(), Box<dyn std::error::Error>> {
        prove_convenience_cleanup_without_transport_eof(false).await
    }

    #[tokio::test]
    async fn convenience_cleanup_retries_failed_deadline_without_transport_eof()
    -> Result<(), Box<dyn std::error::Error>> {
        prove_convenience_cleanup_without_transport_eof(true).await
    }

    #[cfg(feature = "sdk-core-profile")]
    #[test]
    fn nondefault_profile_shutdown_deadline_reaches_framework_configuration() {
        let limits = SdkProfileLimits {
            shutdown_timeout_secs: 41,
            max_callback_concurrency: 3,
            max_extension_invocations: 2,
            ..SdkProfileLimits::default()
        };
        let config = core_adapter_config(&limits);
        assert_eq!(config.shutdown_timeout, std::time::Duration::from_secs(41));
        assert_eq!(config.max_extension_concurrency, 2);
    }

    #[tokio::test]
    async fn manual_concurrent_close_terminates_live_host_without_transport_eof()
    -> Result<(), Box<dyn std::error::Error>> {
        let probe = Arc::new(CleanupProbe::default());
        let adapter = AcpAgentAdapter::new({
            let probe = probe.clone();
            move |_: echo_agent::acp::AcpSessionContext| {
                let probe = probe.clone();
                async move { Ok(Box::new(CleanupAgent(probe)) as Box<dyn Agent>) }
            }
        });
        let owner = Arc::new(HostCloseOwner::new());
        let (agent_transport, client_transport) = Channel::duplex();
        let run = tokio::spawn({
            let owner = owner.clone();
            async move {
                connect_owned(
                    &owner,
                    adapter.close_owner(),
                    adapter.connect_to(agent_transport),
                )
                .await
            }
        });
        let admitted = Arc::new(tokio::sync::Notify::new());
        let directory = tempfile::tempdir()?;
        let cwd = directory.path().to_path_buf();
        let client = tokio::spawn({
            let admitted = admitted.clone();
            async move {
                Client
                    .builder()
                    .connect_with(client_transport, async move |connection| {
                        connection
                            .send_request(v1::InitializeRequest::new(ProtocolVersion::V1))
                            .block_task()
                            .await?;
                        connection
                            .send_request(v1::NewSessionRequest::new(cwd))
                            .block_task()
                            .await?;
                        admitted.notify_one();
                        std::future::pending::<agent_client_protocol::Result<()>>().await
                    })
                    .await
            }
        });
        tokio::time::timeout(std::time::Duration::from_secs(3), admitted.notified()).await?;
        let (first, second) = tokio::join!(owner.close(), owner.close());
        first?;
        second?;
        tokio::time::timeout(std::time::Duration::from_secs(3), run).await???;
        assert!(probe.settled.load(Ordering::Acquire));
        assert_eq!(probe.attempts.load(Ordering::Acquire), 1);
        client.abort();
        let _ = client.await;
        Ok(())
    }

    #[cfg(feature = "sdk-core-profile")]
    #[tokio::test]
    async fn nondefault_shutdown_deadline_bounds_real_agent_close_and_retains_retry_owner()
    -> Result<(), Box<dyn std::error::Error>> {
        let probe = Arc::new(CleanupProbe::default());
        probe.stalled.store(true, Ordering::Release);
        let factory = {
            let probe = probe.clone();
            move |_: echo_agent::acp::AcpSessionContext| {
                let probe = probe.clone();
                async move { Ok(Box::new(CleanupAgent(probe)) as Box<dyn Agent>) }
            }
        };
        let config = core_adapter_config(&SdkProfileLimits {
            shutdown_timeout_secs: 1,
            ..SdkProfileLimits::default()
        });
        let adapter = AcpAgentAdapter::with_config(factory, config)?;
        let owner = Arc::new(HostCloseOwner::new());
        let (agent_transport, client_transport) = Channel::duplex();
        let run = tokio::spawn({
            let owner = owner.clone();
            async move {
                connect_owned(
                    &owner,
                    adapter.close_owner(),
                    adapter.connect_to(agent_transport),
                )
                .await
            }
        });
        let admitted = Arc::new(tokio::sync::Notify::new());
        let directory = tempfile::tempdir()?;
        let cwd = directory.path().to_path_buf();
        let client = tokio::spawn({
            let admitted = admitted.clone();
            async move {
                Client
                    .builder()
                    .connect_with(client_transport, async move |connection| {
                        connection
                            .send_request(v1::InitializeRequest::new(ProtocolVersion::V1))
                            .block_task()
                            .await?;
                        connection
                            .send_request(v1::NewSessionRequest::new(cwd))
                            .block_task()
                            .await?;
                        admitted.notify_one();
                        std::future::pending::<agent_client_protocol::Result<()>>().await
                    })
                    .await
            }
        });
        tokio::time::timeout(std::time::Duration::from_secs(3), admitted.notified()).await?;
        let started = std::time::Instant::now();
        assert!(owner.close().await.is_err());
        let elapsed = started.elapsed();
        assert!(elapsed >= std::time::Duration::from_millis(200));
        assert!(elapsed < std::time::Duration::from_millis(1250));
        assert!(
            owner
                .retained
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .is_some()
        );
        probe.stalled.store(false, Ordering::Release);
        probe.release.notify_waiters();
        owner.close().await?;
        let _ = tokio::time::timeout(std::time::Duration::from_secs(3), run).await?;
        assert!(probe.settled.load(Ordering::Acquire));
        client.abort();
        let _ = client.await;
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn concurrent_close_between_retention_and_framework_bind_waits_for_activation()
    -> Result<(), Box<dyn std::error::Error>> {
        let probe = Arc::new(CleanupProbe::default());
        let adapter = AcpAgentAdapter::new({
            let probe = probe.clone();
            move |_: echo_agent::acp::AcpSessionContext| {
                let probe = probe.clone();
                async move { Ok(Box::new(CleanupAgent(probe)) as Box<dyn Agent>) }
            }
        });
        let owner = Arc::new(HostCloseOwner::new());
        let retained = Arc::new(std::sync::Barrier::new(2));
        let release = Arc::new(std::sync::Barrier::new(2));
        *owner
            .before_activation
            .lock()
            .unwrap_or_else(|e| e.into_inner()) = Some(Box::new({
            let retained = retained.clone();
            let release = release.clone();
            move || {
                retained.wait();
                release.wait();
            }
        }));
        let (agent_transport, _client_transport) = Channel::duplex();
        let run = tokio::spawn({
            let owner = owner.clone();
            async move {
                connect_owned(
                    &owner,
                    adapter.close_owner(),
                    adapter.connect_to(agent_transport),
                )
                .await
            }
        });
        retained.wait();
        let close = owner.close();
        tokio::pin!(close);
        assert!(matches!(
            futures::poll!(&mut close),
            std::task::Poll::Pending
        ));
        assert!(owner.stopping.load(Ordering::Acquire));
        assert!(!owner.activated.load(Ordering::Acquire));
        release.wait();
        tokio::time::timeout(std::time::Duration::from_secs(3), &mut close).await??;
        tokio::time::timeout(std::time::Duration::from_secs(3), run).await???;
        assert!(
            owner
                .retained
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .is_none()
        );
        Ok(())
    }

    #[tokio::test]
    async fn closed_owner_rejects_stdio_before_configuration_access()
    -> Result<(), Box<dyn std::error::Error>> {
        let directory = tempfile::tempdir()?;
        let path = directory.path().join("does-not-exist.json");
        let owner = HostCloseOwner::new();
        owner.close().await?;
        let result = run_stdio_with_close_owner(&path, &owner).await;
        assert!(
            matches!(result, Err(HostError::Runtime(message)) if message == "Host close owner is already closed")
        );
        assert_eq!(std::fs::read_dir(directory.path())?.count(), 0);
        Ok(())
    }

    #[tokio::test]
    async fn close_before_first_run_poll_fences_admission() -> Result<(), Box<dyn std::error::Error>>
    {
        let owner = HostCloseOwner::new();
        owner.close().await?;
        let entered = Arc::new(AtomicBool::new(false));
        let for_run = entered.clone();
        let result = connect_owned(
            &owner,
            echo_agent::acp::AcpAdapterCloseHandle::default(),
            async move {
                for_run.store(true, Ordering::Release);
                Ok(())
            },
        )
        .await;
        assert!(result.is_err());
        assert!(!entered.load(Ordering::Acquire));
        Ok(())
    }

    /// Exercise the same retained-owner helper used by both stdio modes over
    /// a real ACP connection. Session Agent close is the observable resource
    /// side effect; no Host-only simulated Session map participates.
    #[tokio::test]
    async fn owned_host_close_survives_run_abort_failure_and_timeout()
    -> Result<(), Box<dyn std::error::Error>> {
        let probe = Arc::new(CleanupProbe::default());
        probe.fail.store(true, Ordering::Release);
        let adapter = AcpAgentAdapter::new({
            let probe = probe.clone();
            move |_: echo_agent::acp::AcpSessionContext| {
                let probe = probe.clone();
                async move { Ok(Box::new(CleanupAgent(probe)) as Box<dyn Agent>) }
            }
        });
        let owner = Arc::new(HostCloseOwner::new());
        let (agent_transport, client_transport) = Channel::duplex();
        let run = tokio::spawn({
            let owner = owner.clone();
            async move {
                connect_owned(
                    &owner,
                    adapter.close_owner(),
                    adapter.connect_to(agent_transport),
                )
                .await
            }
        });
        let admitted = Arc::new(tokio::sync::Notify::new());
        let directory = tempfile::tempdir()?;
        let cwd = directory.path().to_path_buf();
        let client = tokio::spawn({
            let admitted = admitted.clone();
            async move {
                Client
                    .builder()
                    .connect_with(client_transport, async move |connection| {
                        connection
                            .send_request(v1::InitializeRequest::new(ProtocolVersion::V1))
                            .block_task()
                            .await?;
                        connection
                            .send_request(v1::NewSessionRequest::new(cwd))
                            .block_task()
                            .await?;
                        admitted.notify_one();
                        std::future::pending::<agent_client_protocol::Result<()>>().await
                    })
                    .await
            }
        });
        tokio::time::timeout(std::time::Duration::from_secs(3), admitted.notified()).await?;
        run.abort();
        assert!(run.await.is_err_and(|error| error.is_cancelled()));
        assert!(owner.close().await.is_err());
        assert!(
            owner
                .retained
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .is_some()
        );
        probe.fail.store(false, Ordering::Release);
        probe.stalled.store(true, Ordering::Release);
        assert!(
            tokio::time::timeout(std::time::Duration::from_millis(20), owner.close())
                .await
                .is_err()
        );
        assert!(
            owner
                .retained
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .is_some()
        );
        probe.stalled.store(false, Ordering::Release);
        probe.release.notify_waiters();
        owner.close().await?;
        assert!(probe.settled.load(Ordering::Acquire));
        assert!(
            owner
                .retained
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .is_none()
        );
        let attempts = probe.attempts.load(Ordering::Acquire);
        owner.close().await?;
        assert_eq!(probe.attempts.load(Ordering::Acquire), attempts);
        client.abort();
        let _ = client.await;
        Ok(())
    }
}
