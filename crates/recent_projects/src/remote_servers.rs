//! remote_servers — STUBBED for local-only fork.
//!
//! All SSH-server / DevContainer / WSL picker UI has been eliminated via DCE.
//! `RemoteServerProjects` is retained as a minimal non-constructible enum so
//! that existing `pub use` and action-handler call-sites continue to compile.
//!
//! If you need the real implementation, revert this file from upstream Zed.

use gpui::{App, Context, Entity, Window};
use std::sync::Arc;
use workspace::Workspace;

// ---------------------------------------------------------------------------
// RemoteServerProjects — hollow shell retained for pub-use compatibility.
// ---------------------------------------------------------------------------

/// Non-instantiable stub.  All constructors panic at runtime.
pub struct RemoteServerProjects {
    _private: std::convert::Infallible,
}

impl RemoteServerProjects {
    /// Stub constructor — panics if called.
    #[allow(unused_variables)]
    pub fn new(
        _create_new_window: bool,
        _fs: Arc<dyn project::Fs>,
        _window: &mut Window,
        _workspace: gpui::WeakEntity<Workspace>,
        _cx: &mut Context<Workspace>,
    ) -> Self {
        panic!("RemoteServerProjects::new: remote servers are not supported in this build")
    }

    /// Stub constructor — panics if called.
    #[allow(unused_variables)]
    pub fn new_dev_container(
        _fs: Arc<dyn project::Fs>,
        _configs: Vec<std::path::PathBuf>,
        _app_state: std::sync::Arc<workspace::AppState>,
        _dev_container_context: dev_container::DevContainerContext,
        _window: &mut Window,
        _workspace: gpui::WeakEntity<Workspace>,
        _cx: &mut Context<Workspace>,
    ) -> Self {
        panic!(
            "RemoteServerProjects::new_dev_container: remote servers are not supported in this build"
        )
    }

    /// WSL stub — panics if called.
    #[cfg(target_os = "windows")]
    #[allow(unused_variables)]
    pub fn wsl(
        _create_new_window: bool,
        _fs: Arc<dyn project::Fs>,
        _window: &mut Window,
        _workspace: gpui::WeakEntity<Workspace>,
        _cx: &mut Context<Workspace>,
    ) -> Self {
        panic!("RemoteServerProjects::wsl: WSL is not supported in this build")
    }
}

// gpui requires Render for modal views; provide a vacuous impl.
impl gpui::Render for RemoteServerProjects {
    fn render(
        &mut self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> impl gpui::IntoElement {
        gpui::div()
    }
}

impl gpui::EventEmitter<workspace::DismissEvent> for RemoteServerProjects {}
