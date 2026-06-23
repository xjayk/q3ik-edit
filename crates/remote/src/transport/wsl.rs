pub struct WslConnectionOptions;

pub fn wsl_path_to_windows_path(_: &std::path::Path) -> std::path::PathBuf {
    std::path::PathBuf::new()
}
