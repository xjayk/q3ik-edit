use std::sync::Arc;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ConnectionId {
    pub owner_id: u32,
    pub id: u32,
}

pub struct Peer;

impl Peer {
    pub fn new(_epoch: u32) -> Arc<Self> {
        Arc::new(Self)
    }
}
