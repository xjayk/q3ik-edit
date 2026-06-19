pub struct WslConnectionOptions;

#[cfg(target_os = "windows")]
pub fn wsl_path_to_windows_path(_path: &str) -> String {
    String::new()
}
