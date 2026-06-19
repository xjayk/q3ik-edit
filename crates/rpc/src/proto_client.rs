use std::sync::Arc;

pub trait ProtoClient: Send + Sync {}

pub struct AnyProtoClient;

impl AnyProtoClient {
    pub fn new<T: ProtoClient + 'static>(_client: Arc<T>) -> Self {
        Self
    }

    pub fn is_via_collab(&self) -> bool { false }
}

pub struct NoopProtoClient;

impl NoopProtoClient {
    pub fn new() -> Arc<Self> {
        Arc::new(Self)
    }
}

impl ProtoClient for NoopProtoClient {}
