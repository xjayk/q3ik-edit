//! Stub implementation of the telemetry crate for the q3ik-edit fork.
//! All telemetry is silently dropped — no data is ever sent.
//! The full public API surface is preserved so downstream crates compile
//! without modification.

#![allow(unused_variables, dead_code, unused_imports)]

use std::sync::Arc;

// ---------------------------------------------------------------------------
// event! macro
// ---------------------------------------------------------------------------

/// No-op telemetry event macro.  Accepts any arguments and discards them.
#[macro_export]
macro_rules! event {
    ($($tt:tt)*) => {};
}

// ---------------------------------------------------------------------------
// OS helpers
// ---------------------------------------------------------------------------

/// Returns an empty string — OS name is not reported in this fork.
pub fn os_name() -> &'static str {
    ""
}

/// Returns an empty string — OS version is not reported in this fork.
pub fn os_version() -> Option<&'static str> {
    None
}

// ---------------------------------------------------------------------------
// Telemetry struct
// ---------------------------------------------------------------------------

/// No-op telemetry handle.  The real type lives in `client::telemetry`;
/// this stub satisfies `use telemetry::Telemetry` imports.
#[derive(Clone, Default)]
pub struct Telemetry;

impl Telemetry {
    pub fn new() -> Arc<Self> {
        Arc::new(Self)
    }

    pub fn flush_events(&self) -> gpui::Task<()> {
        gpui::Task::ready(())
    }

    pub fn start(
        &self,
        _installation_id: Option<String>,
        _session_id: String,
        _cx: &mut gpui::App,
    ) {
    }

    pub fn set_authenticated_user_info(
        &self,
        _metrics_id: Option<String>,
        _is_staff: bool,
        _cx: &gpui::App,
    ) {
    }

    pub fn report_event(&self, _kind: &str, _properties: serde_json::Value) {}

    pub fn report_app_event(&self, _operation: String) {}

    pub fn report_editor_event(
        &self,
        _file_extension: Option<String>,
        _vim_mode: bool,
        _operation: &'static str,
        _copilot_enabled: bool,
        _copilot_enabled_for_language: bool,
    ) {
    }

    pub fn report_assistant_event(&self, _event: impl std::fmt::Debug) {}

    pub fn report_call_event(&self, _operation: &'static str) {}

    pub fn report_discovered_project_events(
        &self,
        _workspace_id: u64,
        _events: Vec<()>,
    ) {
    }

    pub fn stop(&self, _cx: &gpui::App) {}
}
