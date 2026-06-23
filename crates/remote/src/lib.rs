pub mod transport;

use std::collections::HashMap;
use std::sync::Arc;

use anyhow::Result;

pub use transport::docker::DockerConnectionOptions;
pub use transport::ssh::SshConnectionOptions;
pub use transport::wsl::WslConnectionOptions;
pub use transport::wsl::wsl_path_to_windows_path;

#[cfg(any(test, feature = "test-support"))]
pub use transport::mock::{MockConnection, MockConnectionOptions, MockConnectionRegistry, MockDelegate};

pub enum Interactive {
    Yes,
    No,
    Once,
}

pub struct RemoteClient;

impl RemoteClient {
    pub fn restore_connection() {}
    pub fn is_connected(&self) -> bool {
        false
    }
    pub fn proto_client(&self) -> rpc::AnyProtoClient {
        rpc::AnyProtoClient::new(Arc::new(rpc::NoopProtoClient::new()))
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
            env: HashMap::new(),
        })
    }
    pub fn connection_options(&self) -> RemoteConnectionOptions {
        RemoteConnectionOptions::Ssh(SshConnectionOptions::default())
    }
}

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

pub struct RemoteConnection;

impl RemoteConnection {
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
            env: HashMap::new(),
        })
    }
    pub fn build_forward_ports_command(
        &self,
        _: Vec<(u16, String, u16)>,
    ) -> Result<CommandTemplate> {
        Ok(CommandTemplate {
            program: String::new(),
            args: Vec::new(),
            env: HashMap::new(),
        })
    }
}

pub struct CommandTemplate {
    pub program: String,
    pub args: Vec<String>,
    pub env: HashMap<String, String>,
}

pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Reconnecting,
}

pub enum RemoteClientEvent {
    Disconnected { server_not_running: bool },
}

pub struct RemoteArch;
pub struct RemoteOs;
pub struct RemotePlatform;
pub struct RemoteConnectionIdentity;
pub struct ConnectionIdentifier;

pub trait RemoteClientDelegate {}

pub fn has_active_connection() -> bool {
    false
}
pub fn connect() {}
pub fn remote_connection_identity() {}
pub fn same_remote_connection_identity() -> bool {
    false
}
