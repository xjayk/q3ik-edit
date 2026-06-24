//! Stub implementation of feature_flags for q3ik-edit fork.
//! All flags are permanently disabled (no server to resolve them against).
//! The full public API surface is preserved so downstream crates compile
//! without modification.

#![allow(unused_variables, dead_code)]

use gpui::{App, AppContext as _, Global};

// ---------------------------------------------------------------------------
// Core traits
// ---------------------------------------------------------------------------

/// A feature flag.  Implement this on a unit struct to define a flag.
pub trait FeatureFlag: 'static {
    const NAME: &'static str;
}

/// Extension trait on `AppContext` / `App` for flag queries.
pub trait FeatureFlagAppExt {
    /// Returns `true` if the current user is staff.  Always `false` in this fork.
    fn is_staff(&self) -> bool {
        false
    }
    /// Returns `true` if the given flag is enabled.  Always `false` in this fork.
    fn has_flag<F: FeatureFlag>(&self) -> bool {
        false
    }
    /// Alias used by most call-sites.
    fn enabled<F: FeatureFlag>(&self) -> bool {
        false
    }
}

impl FeatureFlagAppExt for App {}
impl FeatureFlagAppExt for gpui::AsyncApp {}

/// Marker trait used by `PresenceFlag` pattern.
pub trait FeatureFlagValue: 'static + Send + Sync {
    fn override_key(&self) -> &'static str;
    fn label(&self) -> &'static str;
}

// ---------------------------------------------------------------------------
// Descriptor / Variant types (used by the settings UI)
// ---------------------------------------------------------------------------

pub struct FeatureFlagVariant {
    pub override_key: &'static str,
    pub label: &'static str,
}

pub struct FeatureFlagDescriptor {
    pub name: &'static str,
    pub variants: fn() -> Vec<FeatureFlagVariant>,
}

// ---------------------------------------------------------------------------
// FeatureFlagStore global
// ---------------------------------------------------------------------------

#[derive(Default)]
pub struct FeatureFlagStore;

impl Global for FeatureFlagStore {}

impl FeatureFlagStore {
    /// Register the global.  No-op in this fork.
    pub fn init(cx: &mut App) {
        cx.set_global(FeatureFlagStore);
    }

    /// Returns an empty iterator — no flags are registered.
    pub fn known_flags() -> impl Iterator<Item = &'static FeatureFlagDescriptor> {
        std::iter::empty()
    }

    /// Always returns `false` — no flags are forced on.
    pub fn is_forced_on(_descriptor: &FeatureFlagDescriptor) -> bool {
        false
    }

    /// Returns the default override key (first variant if any, else empty string).
    pub fn resolved_key<'a>(
        &self,
        descriptor: &'a FeatureFlagDescriptor,
        _cx: &App,
    ) -> &'static str {
        ""
    }

    /// Always returns `None` — no overrides are stored.
    pub fn override_for(_name: &str, _cx: &App) -> Option<String> {
        None
    }

    /// No-op: overrides are not persisted in this fork.
    pub fn set_override(
        _name: &str,
        _value: String,
        _fs: &dyn fs::Fs,
        _cx: &mut App,
    ) {
    }

    /// No-op.
    pub fn clear_override(_name: &str, _fs: &dyn fs::Fs, _cx: &mut App) {}
}

// ---------------------------------------------------------------------------
// register_feature_flag! macro
// ---------------------------------------------------------------------------

/// No-op macro — in the real crate this registers a flag with the inventory.
#[macro_export]
macro_rules! register_feature_flag {
    ($flag:ty) => {};
}

/// No-op macro used by PresenceFlag / EnumFeatureFlag patterns.
#[macro_export]
macro_rules! define_feature_flag_variants {
    ($($tt:tt)*) => {};
}

// ---------------------------------------------------------------------------
// generate_feature_flags_schema
// ---------------------------------------------------------------------------

/// Returns an empty JSON object schema.  The real implementation enumerates
/// every registered flag; the stub produces a valid-but-empty schema so the
/// settings JSON schema generation path still compiles.
pub fn generate_feature_flags_schema() -> schemars::Schema {
    schemars::json_schema!({
        "type": "object",
        "additionalProperties": false,
        "properties": {},
    })
}

// ---------------------------------------------------------------------------
// PresenceFlag helper (used with register_feature_flag!)
// ---------------------------------------------------------------------------

/// A flag whose presence (enabled/disabled) is controlled by the feature flag system.
/// In this fork it is always disabled.
pub struct PresenceFlag;

impl FeatureFlag for PresenceFlag {
    const NAME: &'static str = "presence";
}

// ---------------------------------------------------------------------------
// Concrete flag types referenced throughout the codebase.
// Each is a unit struct that implements FeatureFlag with a stable name string.
// ---------------------------------------------------------------------------

macro_rules! declare_flag {
    ($name:ident, $str:literal) => {
        pub struct $name;
        impl FeatureFlag for $name {
            const NAME: &'static str = $str;
        }
    };
}

declare_flag!(DiffReviewFeatureFlag,             "diff-review");
declare_flag!(PanicFeatureFlag,                  "panic");
declare_flag!(SandboxingFeatureFlag,             "sandboxing");
declare_flag!(AcpBetaFeatureFlag,                "acp-beta");
declare_flag!(AgentSharingFeatureFlag,           "agent-sharing");
declare_flag!(CreateThreadToolFeatureFlag,       "create-thread-tool");
declare_flag!(LspToolFeatureFlag,                "lsp-tool");
declare_flag!(RenameToolFeatureFlag,             "rename-tool");
declare_flag!(ProjectPanelUndoRedoFeatureFlag,   "project-panel-undo-redo");
declare_flag!(AgentThreadWorktreeLabel,          "agent-thread-worktree-label");
declare_flag!(AgentThreadWorktreeLabelFlag,      "agent-thread-worktree-label-flag");
declare_flag!(TabularDataPreviewFeatureFlag,     "tabular-data-preview");
