#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct SshConnectionOptions {
    pub username: Option<String>,
    pub host: String,
    pub port: Option<u16>,
}

impl Default for SshConnectionOptions {
    fn default() -> Self {
        Self {
            username: None,
            host: String::new(),
            port: None,
        }
    }
}
