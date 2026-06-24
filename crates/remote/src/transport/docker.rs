use std::collections::BTreeMap;

/// Stub — all Docker transport logic removed in Phase 0.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct DockerConnectionOptions {
    pub name: String,
    pub remote_user: String,
    pub container_id: String,
    pub upload_binary_over_docker_exec: bool,
    pub use_podman: bool,
    pub remote_env: BTreeMap<String, String>,
}
