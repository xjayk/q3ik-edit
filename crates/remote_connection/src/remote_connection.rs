use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use gpui::{
    App, AppContext, AsyncWindowContext, DismissEvent, Entity, EventEmitter, FocusHandle,
    Focusable, IntoElement, Render, SharedString, Task, WeakEntity, Window,
};

pub struct RemoteConnectionModal {
    _focus: FocusHandle,
    pub prompt: Entity<RemoteConnectionPrompt>,
}

impl RemoteConnectionModal {
    pub fn new(
        _connection_options: &remote::RemoteConnectionOptions,
        _paths: Vec<PathBuf>,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Self {
        Self {
            _focus: _cx.focus_handle(),
            prompt: _cx.new(|_cx| RemoteConnectionPrompt::new("".into(), None, false, false, _window, _cx)),
        }
    }
    pub fn new_ssh(
        _connection_options: &remote::SshConnectionOptions,
        _paths: Vec<PathBuf>,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Self {
        Self {
            _focus: _cx.focus_handle(),
            prompt: _cx.new(|_cx| RemoteConnectionPrompt::new("".into(), None, false, false, _window, _cx)),
        }
    }
    pub fn finished(&mut self, _cx: &mut App) {}
}

impl Focusable for RemoteConnectionModal {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self._focus.clone()
    }
}

impl EventEmitter<DismissEvent> for RemoteConnectionModal {}

impl Render for RemoteConnectionModal {
    fn render(
        &mut self,
        _window: &mut Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        gpui::Empty
    }
}

use workspace::ModalView;

impl ModalView for RemoteConnectionModal {}

pub fn connect_with_modal<T>(
    _workspace: &Entity<T>,
    _connection_options: remote::RemoteConnectionOptions,
    _window: &mut Window,
    _cx: &mut App,
) -> Task<anyhow::Result<Option<Entity<remote::RemoteClient>>>> {
    Task::ready(Ok(None))
}

pub fn dismiss_connection_modal<T>(_workspace: &Entity<T>, _cx: &mut AsyncWindowContext) {
    let _ = _cx;
}

pub fn connect_reusing_pool(
    _connection: remote::RemoteConnectionOptions,
    _cx: &mut App,
) -> Task<anyhow::Result<Option<Entity<remote::RemoteClient>>>> {
    Task::ready(Ok(None))
}

pub fn connect(
    _connection_identifier: remote::remote_client::ConnectionIdentifier,
    _connection_options: remote::RemoteConnectionOptions,
    _prompt: Entity<RemoteConnectionPrompt>,
    _window: &mut Window,
    _cx: &mut App,
) -> Task<anyhow::Result<Option<Entity<remote::RemoteClient>>>> {
    Task::ready(Ok(None))
}

use gpui::WeakEntity;

pub struct RemoteClientDelegate {
    _handle: gpui::AnyWindowHandle,
    _prompt: WeakEntity<RemoteConnectionPrompt>,
}

impl RemoteClientDelegate {
    pub fn new(
        handle: gpui::AnyWindowHandle,
        prompt: WeakEntity<RemoteConnectionPrompt>,
    ) -> Arc<Self> {
        Arc::new(Self {
            _handle: handle,
            _prompt: prompt,
        })
    }
}

impl remote::RemoteClientDelegate for RemoteClientDelegate {}

pub struct RemoteConnectionPrompt;

impl RemoteConnectionPrompt {
    pub fn new(
        _connection_string: impl Into<SharedString>,
        _nickname: Option<SharedString>,
        _show_projects: bool,
        _projects_locked: bool,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Self {
        Self
    }
    pub fn set_pending(&mut self, _: bool, _cx: &mut App) {}
    pub fn set_error(&mut self, _: Option<&str>, _cx: &mut App) {}
    pub fn dismiss(&mut self, _window: &mut Window, _cx: &mut App) {}
    pub fn confirm(&mut self, _window: &mut Window, _cx: &mut App) {}
    pub fn set_cancellation_tx(&mut self, _tx: futures::channel::oneshot::Sender<()>) {}
}

impl Render for RemoteConnectionPrompt {
    fn render(
        &mut self,
        _window: &mut Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        gpui::Empty
    }
}

pub struct SshConnectionHeader {
    pub connection_string: SharedString,
    pub paths: Vec<PathBuf>,
    pub nickname: Option<SharedString>,
    pub is_wsl: bool,
    pub is_devcontainer: bool,
}

impl SshConnectionHeader {
    pub fn forward_ports_section(&self) -> Option<Entity<gpui::AnyElement>> {
        None
    }
    pub fn render(&self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        gpui::Empty
    }
}
