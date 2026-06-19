#![allow(unused_variables)]
use std::sync::Arc;
use anyhow::Result;
use futures::future::{BoxFuture, LocalBoxFuture};
use futures::stream::BoxStream;
use collections::{HashMap, TypeIdHashMap};
use gpui::{AnyEntity, AnyWeakEntity, AsyncApp, Entity};
use parking_lot::Mutex;

pub trait ProtoClient: Send + Sync {
    fn request(
        &self,
        envelope: proto::Envelope,
        request_type: &'static str,
    ) -> BoxFuture<'static, Result<proto::Envelope>>;

    fn request_stream(
        &self,
        envelope: proto::Envelope,
        request_type: &'static str,
    ) -> BoxFuture<'static, Result<BoxStream<'static, Result<proto::Envelope>>>>;

    fn send(&self, envelope: proto::Envelope, message_type: &'static str) -> Result<()>;

    fn send_response(&self, envelope: proto::Envelope, message_type: &'static str) -> Result<()>;

    fn message_handler_set(&self) -> &Mutex<ProtoMessageHandlerSet>;

    fn is_via_collab(&self) -> bool;
    fn has_wsl_interop(&self) -> bool;
}

pub enum EntityMessageSubscriber {
    Entity { handle: AnyWeakEntity },
    Pending(Vec<Box<dyn proto::AnyTypedEnvelope>>),
}

pub type ProtoMessageHandler = Arc<
    dyn Send + Sync + Fn(AnyEntity, Box<dyn proto::AnyTypedEnvelope>, AnyProtoClient, AsyncApp) -> LocalBoxFuture<'static, Result<()>>
>;

pub struct ProtoMessageHandlerSet {
    pub entity_types_by_message_type: TypeIdHashMap<std::any::TypeId>,
    pub entities_by_type_and_remote_id: HashMap<(std::any::TypeId, u64), EntityMessageSubscriber>,
    pub entity_id_extractors: TypeIdHashMap<fn(&dyn proto::AnyTypedEnvelope) -> u64>,
    pub entities_by_message_type: TypeIdHashMap<AnyWeakEntity>,
    pub message_handlers: TypeIdHashMap<ProtoMessageHandler>,
}

impl ProtoMessageHandlerSet {
    pub fn clear(&mut self) {
        self.entity_types_by_message_type.clear();
        self.entities_by_type_and_remote_id.clear();
        self.entity_id_extractors.clear();
        self.entities_by_message_type.clear();
        self.message_handlers.clear();
    }

    pub fn handle_message(this: &Mutex<Self>, message: Box<dyn proto::AnyTypedEnvelope>, client: AnyProtoClient, cx: AsyncApp) -> Option<LocalBoxFuture<'static, Result<()>>> {
        None
    }
}

impl Default for ProtoMessageHandlerSet {
    fn default() -> Self {
        Self {
            entity_types_by_message_type: TypeIdHashMap::default(),
            entities_by_type_and_remote_id: HashMap::default(),
            entity_id_extractors: TypeIdHashMap::default(),
            entities_by_message_type: TypeIdHashMap::default(),
            message_handlers: TypeIdHashMap::default(),
        }
    }
}

#[derive(Clone)]
pub struct AnyProtoClient(pub Arc<dyn ProtoClient>);

impl AnyProtoClient {
    pub fn new<T: ProtoClient + 'static>(client: Arc<T>) -> Self {
        Self(client)
    }

    pub fn is_via_collab(&self) -> bool {
        self.0.is_via_collab()
    }

    pub fn has_wsl_interop(&self) -> bool {
        self.0.has_wsl_interop()
    }

    pub fn request<T: proto::RequestMessage>(
        &self,
        _request: T,
    ) -> impl std::future::Future<Output = Result<T::Response>> + 'static {
        async move { Err(anyhow::anyhow!("RPC stubbed out")) }
    }

    pub fn send<T: proto::EnvelopedMessage>(&self, _request: T) -> Result<()> {
        Ok(())
    }

    pub fn send_response<T: proto::EnvelopedMessage>(&self, _request_id: u32, _request: T) -> Result<()> {
        Ok(())
    }

    pub fn add_request_handler<M, E, H, F>(&self, _entity: gpui::WeakEntity<E>, _handler: H)
    where
        M: proto::RequestMessage,
        E: 'static,
        H: 'static + Sync + Fn(gpui::Entity<E>, proto::TypedEnvelope<M>, AsyncApp) -> F,
        F: std::future::Future<Output = Result<M::Response>> + 'static {}

    pub fn add_entity_message_handler<M, E, H, F>(&self, _handler: H)
    where
        M: proto::EnvelopedMessage,
        E: 'static,
        H: 'static + Sync + Send + Fn(gpui::Entity<E>, proto::TypedEnvelope<M>, AsyncApp) -> F,
        F: std::future::Future<Output = Result<()>> + 'static {}

    pub fn subscribe_to_entity<E: 'static>(&self, _remote_id: u64, _entity: &gpui::Entity<E>) {}
}

impl<T: ProtoClient + 'static> From<Arc<T>> for AnyProtoClient {
    fn from(client: Arc<T>) -> Self {
        Self(client)
    }
}

pub struct NoopProtoClient {
    handler_set: Mutex<ProtoMessageHandlerSet>,
}

impl NoopProtoClient {
    pub fn new() -> Arc<Self> {
        Arc::new(Self { handler_set: Mutex::new(ProtoMessageHandlerSet::default()) })
    }
}

impl ProtoClient for NoopProtoClient {
    fn request(&self, _: proto::Envelope, _: &'static str) -> BoxFuture<'static, Result<proto::Envelope>> {
        Box::pin(async { Err(anyhow::anyhow!("stubbed")) })
    }

    fn request_stream(&self, _: proto::Envelope, _: &'static str) -> BoxFuture<'static, Result<BoxStream<'static, Result<proto::Envelope>>>> {
        Box::pin(async { Err(anyhow::anyhow!("stubbed")) })
    }

    fn send(&self, _: proto::Envelope, _: &'static str) -> Result<()> {
        Ok(())
    }

    fn send_response(&self, _: proto::Envelope, _: &'static str) -> Result<()> {
        Ok(())
    }

    fn message_handler_set(&self) -> &Mutex<ProtoMessageHandlerSet> {
        &self.handler_set
    }

    fn is_via_collab(&self) -> bool {
        false
    }

    fn has_wsl_interop(&self) -> bool {
        false
    }
}
