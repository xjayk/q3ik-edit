#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct WslConnectionOptions {
    pub user: Option<String>,
    pub distro_name: String,
}

impl WslConnectionOptions {
    pub fn abs_windows_path_to_wsl_path(
        &self,
        _: &std::path::Path,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<std::path::PathBuf>> + Send>> {
        Box::pin(std::future::ready(Ok(std::path::PathBuf::new())))
    }
}

pub fn wsl_path_to_windows_path(_: &std::path::Path) -> std::path::PathBuf {
    std::path::PathBuf::new()
}
