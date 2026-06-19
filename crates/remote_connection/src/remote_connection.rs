use anyhow::Result;
use gpui::{App, Entity, Task, Window};

pub struct RemoteConnectionModal;

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

pub struct SshConnectionHeader;
