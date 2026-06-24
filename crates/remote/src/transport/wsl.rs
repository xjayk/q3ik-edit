/// Stub — all WSL transport logic removed in Phase 0.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct WslConnectionOptions {
    pub distro_name: String,
    pub user: Option<String>,
}

#[cfg(target_os = "windows")]
pub fn wsl_path_to_windows_path(_path: &str) -> String {
    String::new()
}
