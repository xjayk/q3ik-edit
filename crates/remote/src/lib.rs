pub mod remote_client;
pub mod transport;

use std::sync::Arc;

use anyhow::Result;
use collections::HashMap;
use gpui::EventEmitter;

pub use transport::docker::DockerConnectionOptions;
pub use transport::ssh::SshConnectionOptions;
pub use transport::wsl::WslConnectionOptions;
pub use transport::wsl::wsl_path_to_windows_path;

#[cfg(any(test, feature = "test-support"))]
pub use transport::mock::{MockConnection, MockConnectionOptions, MockConnectionRegistry, MockDelegate};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Interactive {
    Yes,
    No,
    Once,
}

pub struct RemoteClient;

impl RemoteClient {
    pub fn new(
        _: remote_client::ConnectionIdentifier,
        _: Arc<dyn RemoteConnection>,
        _: futures::channel::oneshot::Receiver<()>,
        _: Arc<dyn RemoteClientDelegate>,
        _: &mut gpui::App,
    ) -> gpui::Task<anyhow::Result<Option<gpui::Entity<RemoteClient>>>> {
        gpui::Task::ready(Ok(None))
    }
    pub fn restore_connection() {}
    pub fn is_connected(&self) -> bool {
        false
    }
    pub fn is_disconnected(&self) -> bool {
        true
    }
    pub fn proto_client(&self) -> rpc::AnyProtoClient {
        rpc::AnyProtoClient::new(Arc::new(rpc::NoopProtoClient::new()))
    }
    pub fn remote_connection(&self) -> Option<Arc<dyn RemoteConnection>> {
        None
    }
    pub fn build_command(
        &self,
        _: Option<String>,
        _: &[String],
        _: &HashMap<String, String>,
        _: Option<String>,
        _: Option<(u16, String, u16)>,
        _: Interactive,
    ) -> Result<CommandTemplate> {
        Ok(CommandTemplate {
            program: String::new(),
            args: Vec::new(),
            env: HashMap::default(),
        })
    }
    pub fn connection_options(&self) -> RemoteConnectionOptions {
        RemoteConnectionOptions::Ssh(SshConnectionOptions::default())
    }
    pub fn shell(&self) -> Option<String> {
        None
    }
    pub fn path_style(&self) -> util::paths::PathStyle {
        util::paths::PathStyle::Posix
    }
    pub fn shutdown_processes(
        &mut self,
        _: Option<rpc::proto::ShutdownRemoteServer>,
        _: gpui::BackgroundExecutor,
    ) -> Option<gpui::Task<()>> {
        None
    }
    pub fn connection_state(&self) -> ConnectionState {
        ConnectionState::Disconnected
    }
    pub fn has_wsl_interop(&self) -> bool {
        false
    }
    pub fn default_system_shell(&self) -> Option<String> {
        None
    }
}

impl EventEmitter<RemoteClientEvent> for RemoteClient {}

#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum RemoteConnectionOptions {
    Ssh(SshConnectionOptions),
    Wsl(WslConnectionOptions),
    Docker(DockerConnectionOptions),
}

impl RemoteConnectionOptions {
    pub fn display_name(&self) -> String {
        String::new()
    }
}

pub use util::paths::PathStyle;

pub trait RemoteConnection: Send + Sync {
    fn build_command(
        &self,
        _: Option<String>,
        _: &[String],
        _: &HashMap<String, String>,
        _: Option<String>,
        _: Option<(u16, String, u16)>,
        _: Interactive,
    ) -> Result<CommandTemplate> {
        Ok(CommandTemplate {
            program: String::new(),
            args: Vec::new(),
            env: HashMap::default(),
        })
    }
    fn build_forward_ports_command(&self, _: Vec<(u16, String, u16)>) -> Result<CommandTemplate> {
        Ok(CommandTemplate {
            program: String::new(),
            args: Vec::new(),
            env: HashMap::default(),
        })
    }
    fn connection_options(&self) -> RemoteConnectionOptions {
        RemoteConnectionOptions::Ssh(SshConnectionOptions::default())
    }
}

pub struct CommandTemplate {
    pub program: String,
    pub args: Vec<String>,
    pub env: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Reconnecting,
    HeartbeatMissed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RemoteClientEvent {
    Disconnected { server_not_running: bool },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum RemoteConnectionIdentity {
    Ssh {
        username: Option<String>,
        host: String,
        port: Option<u16>,
    },
    Wsl {
        user: Option<String>,
        distro_name: String,
    },
    Docker {
        name: String,
        remote_user: String,
        container_id: String,
    },
}

impl RemoteConnectionIdentity {
    pub fn connection_id(&self) -> u64 {
        0
    }
}

pub trait RemoteClientDelegate {}

pub fn has_active_connection() -> bool {
    false
}
pub fn connect() {}
pub fn remote_connection_identity(
    opts: &RemoteConnectionOptions,
) -> RemoteConnectionIdentity {
    match opts {
        RemoteConnectionOptions::Ssh(ssh) => RemoteConnectionIdentity::Ssh {
            username: ssh.username.clone(),
            host: ssh.host.clone(),
            port: ssh.port,
        },
        RemoteConnectionOptions::Wsl(wsl) => RemoteConnectionIdentity::Wsl {
            user: wsl.user.clone(),
            distro_name: wsl.distro_name.clone(),
        },
        RemoteConnectionOptions::Docker(docker) => RemoteConnectionIdentity::Docker {
            name: docker.name.clone(),
            remote_user: docker.remote_user.clone(),
            container_id: docker.container_id.clone(),
        },
    }
}
pub fn same_remote_connection_identity(
    _: Option<&RemoteConnectionOptions>,
    _: Option<&RemoteConnectionOptions>,
) -> bool {
    false
}
