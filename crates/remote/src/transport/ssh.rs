use std::sync::Arc;

/// Stub — all SSH transport logic removed in Phase 0.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct SshConnectionOptions {
    pub host: Arc<str>,
    pub username: Option<String>,
    pub port: Option<u16>,
    pub password: Option<String>,
    pub nickname: Option<String>,
    pub upload_binary_over_ssh: bool,
    pub args: Option<Vec<String>>,
    pub port_forwards: Option<Vec<super::super::SshPortForwardOption>>,
    pub connection_timeout: Option<u16>,
}

impl SshConnectionOptions {
    /// Stub: always returns an empty/default options struct.
    pub fn parse_command_line(_input: &str) -> anyhow::Result<Self> {
        Ok(Self {
            host: "".into(),
            ..Default::default()
        })
    }

    /// Stub: returns an empty connection string.
    pub fn connection_string(&self) -> String {
        self.host.to_string()
    }
}

#[derive(
    Debug, Clone, PartialEq, Eq, Hash,
    serde::Serialize, serde::Deserialize,
)]
pub enum SshPortForwardOption {
    Local(Vec<String>),
    Remote(Vec<String>),
    Dynamic(Vec<String>),
}
