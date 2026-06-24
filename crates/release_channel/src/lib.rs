//! Stub release_channel crate for q3ik-edit.
//!
//! Always reports `ReleaseChannel::Dev`. All env-var detection,
//! RELEASE_CHANNEL file reads, and ZED_DISABLE_STAFF logic have been
//! removed. The public API surface is preserved so downstream crates
//! compile without modification.

use gpui::{App, AppContext, Global};
use semver::Version;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Always `"dev"` in this fork.
pub static RELEASE_CHANNEL_NAME: std::sync::LazyLock<String> =
    std::sync::LazyLock::new(|| "dev".to_string());

/// Always `ReleaseChannel::Dev` in this fork.
pub static RELEASE_CHANNEL: std::sync::LazyLock<ReleaseChannel> =
    std::sync::LazyLock::new(|| ReleaseChannel::Dev);

/// The release channel of the editor.
///
/// In this fork this is always `Dev`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ReleaseChannel {
    Dev,
    Nightly,
    Preview,
    Stable,
}

impl ReleaseChannel {
    /// Returns the globally registered channel, defaulting to `Dev`.
    pub fn global(cx: &AppContext) -> Self {
        cx.try_global::<GlobalReleaseChannel>()
            .map(|g| g.0)
            .unwrap_or(ReleaseChannel::Dev)
    }

    /// Returns `Some(Dev)` always.
    pub fn try_global(cx: &AppContext) -> Option<Self> {
        Some(Self::global(cx))
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            ReleaseChannel::Dev => "Dev",
            ReleaseChannel::Nightly => "Nightly",
            ReleaseChannel::Preview => "Preview",
            ReleaseChannel::Stable => "Stable",
        }
    }

    pub fn dev_name(&self) -> &'static str {
        match self {
            ReleaseChannel::Dev => "dev",
            ReleaseChannel::Nightly => "nightly",
            ReleaseChannel::Preview => "preview",
            ReleaseChannel::Stable => "stable",
        }
    }

    pub fn link_prefix(&self) -> &'static str {
        match self {
            ReleaseChannel::Dev => "https://zed.dev/",
            ReleaseChannel::Nightly => "https://zed.dev/",
            ReleaseChannel::Preview => "https://zed.dev/",
            ReleaseChannel::Stable => "https://zed.dev/",
        }
    }

    pub fn docs_url(&self) -> &'static str {
        "https://zed.dev/docs"
    }

    pub fn is_staff(&self) -> bool {
        false
    }
}

impl std::fmt::Display for ReleaseChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.dev_name())
    }
}

#[derive(Clone, Copy, Global)]
struct GlobalReleaseChannel(ReleaseChannel);

/// The version of the application.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AppVersion(pub Version);

impl AppVersion {
    /// Returns the globally registered version, falling back to `0.0.0`.
    pub fn global(cx: &AppContext) -> Version {
        cx.try_global::<GlobalAppVersion>()
            .map(|g| g.0.clone())
            .unwrap_or_else(|| Version::new(0, 0, 0))
    }

    pub fn try_global(cx: &AppContext) -> Option<Version> {
        Some(Self::global(cx))
    }

    /// Parse a version from the `CARGO_PKG_VERSION` string.
    /// The `_release_channel` and `_sha` arguments are accepted for
    /// API compatibility but ignored.
    pub fn load(
        pkg_version: &str,
        _release_channel: Option<String>,
        _commit_sha: Option<String>,
    ) -> Version {
        Version::parse(pkg_version).unwrap_or_else(|_| Version::new(0, 0, 0))
    }
}

#[derive(Clone, Global)]
struct GlobalAppVersion(Version);

/// The commit SHA of the build.
#[derive(Debug, Clone)]
pub struct AppCommitSha(pub Arc<str>);

impl AppCommitSha {
    pub fn try_global(_cx: &AppContext) -> Option<Arc<str>> {
        None
    }
}

#[derive(Clone, Global)]
struct GlobalAppCommitSha(Arc<str>);

/// Register `ReleaseChannel::Dev` and the given `version` into the gpui
/// application context. Call this once during app startup.
pub fn init(version: Version, cx: &mut App) {
    cx.set_global(GlobalReleaseChannel(ReleaseChannel::Dev));
    cx.set_global(GlobalAppVersion(version));
}

/// Same as [`init`]; the `channel` argument is accepted for test-helper
/// compatibility but ignored — channel is always `Dev`.
pub fn init_test(version: Version, _channel: ReleaseChannel, cx: &mut App) {
    init(version, cx);
}
