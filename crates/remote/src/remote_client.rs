#[allow(dead_code)]

pub struct RemoteClient;

pub struct RemoteConnectionOptions;

pub struct RemoteConnection;

pub struct ConnectionIdentifier;

impl ConnectionIdentifier {
    pub fn setup() -> Self {
        ConnectionIdentifier
    }
}

pub enum ConnectionState {
    Connected,
    Disconnected,
}

pub enum Interactive {
    Yes,
    No,
}

pub enum RemoteArch {
    X86,
    Arm,
}

pub enum RemoteOs {
    Linux,
    MacOs,
    Windows,
}

pub struct RemotePlatform;

pub trait RemoteClientDelegate {}

pub enum RemoteClientEvent {}

pub struct CommandTemplate;

pub async fn connect() -> Result<(), ()> {
    Ok(())
}

pub fn has_active_connection() -> bool {
    false
}
