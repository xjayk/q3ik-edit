use gpui::SharedString;

#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct SshConnectionOptions {
    pub username: Option<String>,
    pub host: String,
    pub port: Option<u16>,
    pub password: Option<String>,
    pub nickname: Option<SharedString>,
    pub args: Option<Vec<String>>,
    pub port_forwards: Option<Vec<(u16, String, u16)>>,
    pub connection_timeout: Option<u64>,
    pub upload_binary_over_ssh: Option<bool>,
}

impl SshConnectionOptions {
    pub fn connection_string(&self) -> SharedString {
        SharedString::default()
    }
    pub fn parse_command_line(_input: &str) -> anyhow::Result<Self> {
        anyhow::bail!("not implemented")
    }
}

impl Default for SshConnectionOptions {
    fn default() -> Self {
        Self {
            username: None,
            host: String::new(),
            port: None,
            password: None,
            nickname: None,
            args: None,
            port_forwards: None,
            connection_timeout: None,
            upload_binary_over_ssh: None,
        }
    }
}
