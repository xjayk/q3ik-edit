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

    pub fn app_id(&self) -> &'static str {
        "Zed-Editor-Dev"
    }

    pub const ALL: [Self; 4] = [Self::Dev, Self::Nightly, Self::Preview, Self::Stable];
}

impl Default for ReleaseChannel {
    fn default() -> Self {
        ReleaseChannel::Dev
    }
}

pub struct AppVersion;
impl AppVersion {
    pub fn global(_cx: &App) -> semver::Version {
        semver::Version::new(0, 0, 0)
    }
}

pub struct AppCommitSha;

impl AppCommitSha {
    pub fn try_global(_cx: &App) -> Option<Self> {
        None
    }
    pub fn full(&self) -> String {
        String::new()
    }
}

pub static RELEASE_CHANNEL: LazyLock<ReleaseChannel> = LazyLock::new(|| ReleaseChannel::Dev);

pub fn init(version: semver::Version, cx: &mut App) {}

pub fn init_test(version: semver::Version, channel: ReleaseChannel, cx: &mut App) {}

pub fn docs_url(slug: &str, _cx: &App) -> String {
    format!("https://zed.dev/docs/{}", slug)
}

pub fn app_identifier() -> &'static str {
    "Zed-Editor-Dev"
}

pub static RELEASE_CHANNEL_NAME: LazyLock<String> = LazyLock::new(|| "dev".to_string());
