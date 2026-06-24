use anyhow::Result;
use gpui::{
    App, AsyncWindowContext, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, Render,
    Task, Window,
};

pub struct RemoteConnectionModal {
    _focus: FocusHandle,
    pub prompt: Option<Entity<RemoteConnectionPrompt>>,
}

impl RemoteConnectionModal {
    pub fn new(
        _connection_options: &remote::RemoteConnectionOptions,
        _paths: Vec<std::path::PathBuf>,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Self {
        Self {
            _focus: _cx.focus_handle(),
            prompt: None,
        }
    }
    pub fn new_ssh(
        _connection_options: &remote::SshConnectionOptions,
        _paths: Vec<std::path::PathBuf>,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Self {
        Self {
            _focus: _cx.focus_handle(),
            prompt: None,
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
    _prompt: Option<Entity<RemoteConnectionPrompt>>,
    _window: &mut Window,
    _cx: &mut App,
) -> Task<anyhow::Result<Option<Entity<remote::RemoteClient>>>> {
    Task::ready(Ok(None))
}

pub trait RemoteClientDelegate {}

pub struct RemoteConnectionPrompt;

impl RemoteConnectionPrompt {
    pub fn new(
        _connection_options: &remote::RemoteConnectionOptions,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Entity<Self> {
        unimplemented!()
    }
    pub fn set_pending(&mut self, _: bool, _cx: &mut App) {}
    pub fn set_error(&mut self, _: Option<&str>, _cx: &mut App) {}
    pub fn dismiss(&mut self, _window: &mut Window, _cx: &mut App) {}
}

pub struct SshConnectionHeader;

impl SshConnectionHeader {
    pub fn forward_ports_section(&self) -> Option<Entity<gpui::AnyElement>> {
        None
    }
}
