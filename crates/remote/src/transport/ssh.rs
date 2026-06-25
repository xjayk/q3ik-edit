#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct SshConnectionOptions {
    pub username: Option<String>,
    pub host: String,
    pub port: Option<u16>,
}
