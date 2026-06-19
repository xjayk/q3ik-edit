#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RemoteConnectionIdentity;

pub fn remote_connection_identity() -> RemoteConnectionIdentity {
    RemoteConnectionIdentity
}

pub fn same_remote_connection_identity(
    _a: &RemoteConnectionIdentity,
    _b: &RemoteConnectionIdentity,
) -> bool {
    false
}
