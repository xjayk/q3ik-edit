use crate::transport::docker::DockerConnectionOptions;
use crate::transport::ssh::SshConnectionOptions;
use crate::transport::wsl::WslConnectionOptions;
use anyhow::Result;
use gpui::{App, Context, Entity, EventEmitter, Task, Window};
use std::sync::Arc;

pub use crate::SshPortForwardOption;

// ─── Connection identifier ────────────────────────────────────────────────────

pub struct ConnectionIdentifier;

impl ConnectionIdentifier {
    pub fn setup() -> Self {
        ConnectionIdentifier
    }
}

// ─── Connection options enum ──────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum RemoteConnectionOptions {
    Ssh(SshConnectionOptions),
    Wsl(WslConnectionOptions),
    Docker(DockerConnectionOptions),
    #[cfg(any(test, feature = "test-support"))]
    Mock(crate::transport::mock::MockConnectionOptions),
}

impl From<SshConnectionOptions> for RemoteConnectionOptions {
    fn from(opts: SshConnectionOptions) -> Self {
        RemoteConnectionOptions::Ssh(opts)
    }
}

impl From<WslConnectionOptions> for RemoteConnectionOptions {
    fn from(opts: WslConnectionOptions) -> Self {
        RemoteConnectionOptions::Wsl(opts)
    }
}

// ─── Remote connection trait (build_command stub) ─────────────────────────────

pub struct CommandOutput {
    pub program: String,
    pub args: Vec<String>,
    pub env: Vec<(String, String)>,
}

#[derive(Debug, Clone, Default)]
pub struct CommandEnv;

pub trait RemoteConnection: Send + Sync {
    fn build_command(
        &self,
        program: Option<String>,
        args: &[String],
        env: &CommandEnv,
        _cwd: Option<String>,
        _uid: Option<u32>,
        _interactive: Interactive,
    ) -> Result<CommandOutput>;
}

pub struct StubRemoteConnection;

impl RemoteConnection for StubRemoteConnection {
    fn build_command(
        &self,
        _program: Option<String>,
        _args: &[String],
        _env: &CommandEnv,
        _cwd: Option<String>,
        _uid: Option<u32>,
        _interactive: Interactive,
    ) -> Result<CommandOutput> {
        Err(anyhow::anyhow!("stub: remote transport removed in Phase 0"))
    }
}

// ─── RemoteClient ─────────────────────────────────────────────────────────────

pub enum RemoteClientEvent {}

pub struct RemoteClient;

impl EventEmitter<RemoteClientEvent> for RemoteClient {}

impl RemoteClient {
    /// Stub: returns None because there is no live connection.
    pub fn remote_connection(&self) -> Option<Arc<dyn RemoteConnection>> {
        None
    }

    /// Stub: returns the disconnected state.
    pub fn connection_state(&self) -> ConnectionState {
        ConnectionState::Disconnected
    }

    /// Stub: no-op path-style query.
    pub fn path_style(&self) -> util::paths::PathStyle {
        util::paths::PathStyle::Posix
    }

    /// Stub: no-op force for tests.
    #[allow(unused_variables)]
    pub fn force_server_not_running(&mut self, cx: &mut Context<RemoteClient>) {
        cx.emit(RemoteClientEvent::Disconnected);
    }
}

impl RemoteClientEvent {
    pub const Disconnected: RemoteClientEvent = RemoteClientEvent::_Disconnected;
}

// ─── Misc small types ─────────────────────────────────────────────────────────

#[allow(dead_code)]
pub enum ConnectionState {
    Connected,
    Connecting,
    Disconnected,
    ServerNotRunning,
}

pub enum Interactive {
    Yes,
    No,
}

pub enum RemoteArch {
    X86,
    Arm,
}

pub enum RemoteOs {
    Linux,
    MacOs,
    Windows,
}

pub struct RemotePlatform;

pub trait RemoteClientDelegate {}

pub struct CommandTemplate;

pub async fn connect(
    _options: RemoteConnectionOptions,
    _delegate: Arc<dyn RemoteClientDelegate>,
    _cx: &mut gpui::AsyncApp,
) -> Result<Arc<dyn RemoteConnection>> {
    Err(anyhow::anyhow!("stub: remote transport removed in Phase 0"))
}

pub fn has_active_connection() -> bool {
    false
}

// ─── Test-support helpers (stub) ──────────────────────────────────────────────

#[cfg(any(test, feature = "test-support"))]
impl RemoteClient {
    pub fn fake_server(
        _cx: &mut gpui::TestAppContext,
        _server_cx: &mut gpui::TestAppContext,
    ) -> (RemoteConnectionOptions, (), ()) {
        (
            RemoteConnectionOptions::Mock(crate::transport::mock::MockConnectionOptions::default()),
            (),
            (),
        )
    }

    pub fn fake_server_with_opts(
        _opts: &RemoteConnectionOptions,
        _cx: &mut gpui::TestAppContext,
        _server_cx: &mut gpui::TestAppContext,
    ) -> ((), ()) {
        ((), ())
    }
}
