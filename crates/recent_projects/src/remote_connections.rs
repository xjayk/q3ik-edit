//! remote_connections — STUBBED for local-only fork.
//!
//! All SSH / WSL / RemoteClient connection logic has been eliminated via DCE.
//! The public surface (`navigate_to_positions`, `open_remote_project`,
//! `RemoteSettings`) is preserved as panicking / no-op stubs so that call-
//! sites in `recent_projects.rs` continue to compile without modification.
//!
//! DO NOT restore any `remote` / `remote_connection` imports here.

use gpui::App;
use settings::Settings;
use std::path::PathBuf;
use workspace::{AppState, OpenOptions};

// ---------------------------------------------------------------------------
// RemoteSettings — kept as a hollow settings struct so that the Settings
// impl and `pub use` in lib.rs still resolve.
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct RemoteSettings {
    // All SSH / WSL fields removed. Struct kept to avoid orphan-settings panic.
}

impl Settings for RemoteSettings {
    const KEY: Option<&'static str> = Some("remote");
    type FileContent = Self;

    fn load(
        sources: settings::SettingsSources<'_, Self::FileContent>,
        _cx: &mut App,
    ) -> anyhow::Result<Self> {
        Ok(sources.user.cloned().unwrap_or_default())
    }
}

// ---------------------------------------------------------------------------
// navigate_to_positions — no-op; remote navigation is not supported.
// ---------------------------------------------------------------------------

/// Stub: does nothing. Remote navigation is not supported in this build.
pub fn navigate_to_positions(
    _positions: Vec<(PathBuf, std::ops::Range<usize>)>,
    _cx: &mut App,
) {
    // DCE: remote navigation removed.
}

// ---------------------------------------------------------------------------
// open_remote_project — always returns an error; callers must handle it.
// ---------------------------------------------------------------------------

/// Stub: always fails. Remote project opening is not supported in this build.
pub async fn open_remote_project(
    _connection_options: remote::RemoteConnectionOptions,
    _paths: Vec<PathBuf>,
    _app_state: std::sync::Arc<AppState>,
    _open_options: OpenOptions,
    _cx: &mut gpui::AsyncApp,
) -> anyhow::Result<()> {
    anyhow::bail!("open_remote_project: remote connections are not supported in this build")
}
