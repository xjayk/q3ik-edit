use std::sync::Arc;

pub trait ProtoClient: Send + Sync {}

#[derive(Clone)]
pub struct AnyProtoClient;

impl AnyProtoClient {
    pub fn new<T: ProtoClient + 'static>(_client: Arc<T>) -> Self {
        Self
    }

    pub fn is_via_collab(&self) -> bool {
        false
    }

    pub fn request<T: crate::proto::RequestMessage>(
        &self,
        _request: T,
    ) -> impl std::future::Future<Output = anyhow::Result<T::Response>> {
        async move { Err(anyhow::anyhow!("stubbed")) }
    }
}

pub struct NoopProtoClient;

impl NoopProtoClient {
    pub fn new() -> Arc<Self> {
        Arc::new(Self)
    }
}

impl ProtoClient for NoopProtoClient {}
