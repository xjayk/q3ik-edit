#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct DockerConnectionOptions {
    pub name: String,
    pub remote_user: String,
    pub container_id: String,
    pub upload_binary_over_docker_exec: bool,
    pub use_podman: bool,
    pub remote_env: std::collections::BTreeMap<String, String>,
}
