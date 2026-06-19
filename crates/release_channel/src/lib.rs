#![allow(unused_variables)]
use gpui::{App, Global};
use std::sync::LazyLock;

pub static RELEASE_CHANNEL_NAME: LazyLock<String> = LazyLock::new(|| "dev".to_string());

pub static RELEASE_CHANNEL: LazyLock<ReleaseChannel> = LazyLock::new(|| ReleaseChannel::Dev);

pub fn app_identifier() -> &'static str {
    "Zed-Editor-Dev"
}

#[derive(Clone, Eq, Debug, PartialEq)]
pub struct AppCommitSha(String);

struct GlobalAppCommitSha(AppCommitSha);
impl Global for GlobalAppCommitSha {}

impl AppCommitSha {
    pub fn new(sha: String) -> Self {
        AppCommitSha(sha)
    }
    pub fn try_global(cx: &App) -> Option<AppCommitSha> {
        cx.try_global::<GlobalAppCommitSha>()
            .map(|sha| sha.0.clone())
    }
    pub fn set_global(sha: AppCommitSha, cx: &mut App) {
        cx.set_global(GlobalAppCommitSha(sha))
    }
    pub fn full(&self) -> String {
        self.0.to_string()
    }
    pub fn short(&self) -> String {
        self.0.chars().take(7).collect()
    }
}

pub struct AppVersion;
impl AppVersion {
    pub fn load(
        pkg_version: &str,
        build_id: Option<&str>,
        commit_sha: Option<AppCommitSha>,
    ) -> semver::Version {
        pkg_version.parse().unwrap_or(semver::Version::new(0, 0, 0))
    }
    pub fn global(cx: &App) -> semver::Version {
        semver::Version::new(0, 0, 0)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum ReleaseChannel {
    #[default]
    Dev,
    Nightly,
    Preview,
    Stable,
}

impl ReleaseChannel {
    pub const ALL: [ReleaseChannel; 4] = [
        ReleaseChannel::Dev,
        ReleaseChannel::Nightly,
        ReleaseChannel::Preview,
        ReleaseChannel::Stable,
    ];
    pub fn global(cx: &App) -> Self {
        Self::Dev
    }
    pub fn try_global(cx: &App) -> Option<Self> {
        Some(Self::Dev)
    }
    pub fn poll_for_updates(&self) -> bool {
        false
    }
    pub fn display_name(&self) -> &'static str {
        "Zed Dev"
    }
    pub fn dev_name(&self) -> &'static str {
        "dev"
    }
    pub fn app_id(&self) -> &'static str {
        "dev.zed.Zed-Dev"
    }
    pub fn release_query_param(&self) -> Option<&'static str> {
        None
    }
    pub fn docs_url(&self, slug: &str) -> String {
        format!("https://zed.dev/docs/{}", slug)
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq)]
pub struct InvalidReleaseChannel;

impl std::str::FromStr for ReleaseChannel {
    type Err = InvalidReleaseChannel;
    fn from_str(channel: &str) -> Result<Self, Self::Err> {
        Ok(ReleaseChannel::Dev)
    }
}

pub fn init(app_version: semver::Version, cx: &mut App) {}
pub fn init_test(app_version: semver::Version, release_channel: ReleaseChannel, cx: &mut App) {}
pub fn docs_url(slug: &str, cx: &App) -> String {
    format!("https://zed.dev/docs/{}", slug)
}
