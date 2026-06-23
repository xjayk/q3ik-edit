use gpui::App;
use std::sync::LazyLock;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReleaseChannel {
    Dev,
    Nightly,
    Preview,
    Stable,
}

impl ReleaseChannel {
    pub fn dev_name(&self) -> &'static str {
        "dev"
    }

    pub fn display_name(&self) -> &'static str {
        "Dev"
    }

    pub fn release_query_param(&self) -> Option<&'static str> {
        None
    }

    pub fn try_global(cx: &App) -> Option<ReleaseChannel> {
        Some(ReleaseChannel::Dev)
    }

    pub fn global(cx: &App) -> Self {
        ReleaseChannel::Dev
    }
}

impl Default for ReleaseChannel {
    fn default() -> Self {
        ReleaseChannel::Dev
    }
}

pub struct AppVersion {
    version: semver::Version,
}

impl AppVersion {
    pub fn global(cx: &App) -> Self {
        Self {
            version: semver::Version::new(0, 0, 0),
        }
    }
}

impl std::fmt::Display for AppVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.version)
    }
}

pub struct AppCommitSha;

pub static RELEASE_CHANNEL: LazyLock<ReleaseChannel> = LazyLock::new(|| ReleaseChannel::Dev);

pub fn init(version: semver::Version, cx: &mut App) {}

pub fn init_test(version: semver::Version, channel: ReleaseChannel, cx: &mut App) {}
