use anyhow::Result;
use gpui::{
    App, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, ManagedView, Render, Task,
    Window,
};

pub struct RemoteConnectionModal {
    _focus: FocusHandle,
}

impl RemoteConnectionModal {
    pub fn new(
        _connection_options: &remote::RemoteConnectionOptions,
        _paths: Vec<std::path::PathBuf>,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Task<Result<Entity<Self>>> {
        Task::ready(Err(anyhow::anyhow!("not implemented")))
    }
    pub fn new_ssh(
        _connection_options: &remote::SshConnectionOptions,
        _paths: Vec<std::path::PathBuf>,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Task<Result<Entity<Self>>> {
        Task::ready(Err(anyhow::anyhow!("not implemented")))
    }
    pub fn prompt(&self) -> Option<Entity<RemoteConnectionPrompt>> {
        None
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

pub fn connect_with_modal(
    _workspace: &Entity<()>,
    _connection_options: (),
    _window: &mut Window,
    _cx: &mut App,
) -> Task<Result<()>> {
    Task::ready(Ok(()))
}

pub fn dismiss_connection_modal(_workspace: &Entity<()>, _cx: &mut App) {}

pub fn connect_reusing_pool(
    _connection: (),
    _cx: &mut App,
) -> Task<Result<Entity<RemoteConnectionModal>>> {
    Task::ready(Err(anyhow::anyhow!("not implemented")))
}

pub fn connect(_connection_options: (), _window: &mut Window, _cx: &mut App) -> Task<Result<()>> {
    Task::ready(Ok(()))
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
