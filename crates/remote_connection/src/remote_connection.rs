use anyhow::Result;
use gpui::{
    AnyElement, App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable,
    IntoElement, SharedString, Task, WeakEntity, Window,
};
use remote::{RemoteClientDelegate as RemoteClientDelegateTrait, RemoteConnectionOptions};
use std::path::PathBuf;
use std::sync::Arc;

// ─── SshConnectionHeader ─────────────────────────────────────────────────────

pub struct SshConnectionHeader {
    pub connection_string: SharedString,
    pub paths: Vec<PathBuf>,
    pub nickname: Option<SharedString>,
    pub is_wsl: bool,
    pub is_devcontainer: bool,
}

impl SshConnectionHeader {
    pub fn render(&self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        gpui::div()
    }
}

// ─── RemoteConnectionPrompt ───────────────────────────────────────────────────

pub struct RemoteConnectionPrompt {
    focus_handle: FocusHandle,
}

impl RemoteConnectionPrompt {
    pub fn new(
        _connection_string: String,
        _nickname: Option<String>,
        _is_wsl: bool,
        _is_devcontainer: bool,
        _window: &mut Window,
        cx: &mut App,
    ) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
        }
    }

    pub fn confirm(&mut self, _window: &mut Window, _cx: &mut App) {}

    pub fn set_cancellation_tx(&mut self, _tx: futures::channel::oneshot::Sender<()>) {}
}

impl Focusable for RemoteConnectionPrompt {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

// ─── RemoteConnectionModal ────────────────────────────────────────────────────

pub struct RemoteConnectionModal {
    pub prompt: Entity<RemoteConnectionPrompt>,
}

impl RemoteConnectionModal {
    pub fn new(
        connection_options: &RemoteConnectionOptions,
        _paths: Vec<PathBuf>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let connection_string = match connection_options {
            RemoteConnectionOptions::Ssh(opts) => opts.connection_string(),
            RemoteConnectionOptions::Wsl(opts) => opts.distro_name.clone(),
            RemoteConnectionOptions::Docker(opts) => opts.name.clone(),
            #[cfg(any(test, feature = "test-support"))]
            RemoteConnectionOptions::Mock(opts) => format!("mock-{}", opts.id),
        };
        let prompt = cx.new(|cx| {
            RemoteConnectionPrompt::new(connection_string, None, false, false, window, cx)
        });
        Self { prompt }
    }

    pub fn finished(&mut self, _cx: &mut Context<Self>) {}
}

impl EventEmitter<DismissEvent> for RemoteConnectionModal {}

impl gpui::Render for RemoteConnectionModal {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        gpui::div()
    }
}

// ─── RemoteClientDelegate ─────────────────────────────────────────────────────

pub struct RemoteClientDelegate {
    window: gpui::AnyWindowHandle,
    ui: WeakEntity<RemoteConnectionPrompt>,
    password: Option<askpass::EncryptedPassword>,
}

impl RemoteClientDelegate {
    pub fn new(
        window: gpui::AnyWindowHandle,
        ui: WeakEntity<RemoteConnectionPrompt>,
        password: Option<askpass::EncryptedPassword>,
    ) -> Self {
        Self {
            window,
            ui,
            password,
        }
    }
}

impl RemoteClientDelegateTrait for RemoteClientDelegate {}

// ─── connect() ───────────────────────────────────────────────────────────────

/// Stub top-level connect function re-exported by recent_projects.
/// Always returns a task that immediately errors, satisfying the compiler while
/// Phase 0 has no live transport layer.
pub fn connect(
    _id: remote::ConnectionIdentifier,
    _options: RemoteConnectionOptions,
    _prompt: Entity<RemoteConnectionPrompt>,
    _window: &mut Window,
    _cx: &mut App,
) -> Task<Result<Option<Entity<remote::RemoteClient>>>> {
    Task::ready(Err(anyhow::anyhow!(
        "stub: remote transport removed in Phase 0"
    )))
}
