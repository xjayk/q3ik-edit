pub mod auth;
pub mod notification;
pub use notification::*;

pub use proto;
pub use proto::Envelope;
pub use proto::{ErrorCode, ErrorExt, Receipt, TypedEnvelope, error};

use std::sync::Arc;
use std::time::{Duration, Instant};

use anyhow::Result;
use futures::stream::BoxStream;
use futures::{Future, StreamExt};

pub const PROTOCOL_VERSION: u32 = 68;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ConnectionId {
    pub owner_id: u32,
    pub id: u32,
}

pub struct Connection;

impl Connection {
    pub fn new<S>(_: S) -> Self {
        Self
    }

    /// Creates a pair of in-memory connections for use in tests.
    /// Returns (client_conn, server_conn, channel) where channel is a dummy unit.
    pub fn in_memory(_executor: gpui::BackgroundExecutor) -> (Self, Self, ()) {
        (Self, Self, ())
    }
}

impl From<proto::PeerId> for ConnectionId {
    fn from(_: proto::PeerId) -> Self {
        ConnectionId { owner_id: 0, id: 0 }
    }
}

impl From<ConnectionId> for proto::PeerId {
    fn from(id: ConnectionId) -> Self {
        proto::PeerId {
            owner_id: id.owner_id,
            id: id.id,
        }
    }
}

pub struct Peer;

impl Peer {
    pub fn new(_: u32) -> Arc<Self> {
        Arc::new(Self)
    }
    pub fn add_connection<F, Fut, Out>(
        self: &Arc<Self>,
        _: Connection,
        _: F,
    ) -> (
        ConnectionId,
        impl Future<Output = anyhow::Result<()>> + Send + use<F, Fut, Out>,
        BoxStream<'static, Box<dyn proto::AnyTypedEnvelope>>,
    )
    where
        F: Send + Fn(Duration) -> Fut,
        Fut: Send + Future<Output = Out>,
        Out: Send,
    {
        let (_, rx) = futures::channel::mpsc::unbounded();
        (
            ConnectionId { owner_id: 0, id: 0 },
            async { Ok(()) },
            rx.boxed(),
        )
    }

    /// Test-only variant of `add_connection` that uses an in-memory `Connection`.
    pub fn add_test_connection(
        self: &Arc<Self>,
        _: Connection,
        _executor: gpui::BackgroundExecutor,
    ) -> (
        ConnectionId,
        impl Future<Output = anyhow::Result<()>> + Send + use<>,
        BoxStream<'static, Box<dyn proto::AnyTypedEnvelope>>,
    ) {
        let (_, rx) = futures::channel::mpsc::unbounded();
        (
            ConnectionId { owner_id: 0, id: 0 },
            async { Ok(()) },
            rx.boxed(),
        )
    }

    pub fn disconnect(&self, _: ConnectionId) {}
    pub fn teardown(&self) {}
    pub fn request<T: proto::RequestMessage>(
        &self,
        _: ConnectionId,
        _: T,
    ) -> gpui::Task<Result<TypedEnvelope<T::Response>>> {
        gpui::Task::ready(Err(anyhow::anyhow!("peer stub")))
    }
    pub fn request_envelope<T: proto::RequestMessage>(
        &self,
        _: ConnectionId,
        _: T,
    ) -> gpui::Task<Result<TypedEnvelope<T::Response>>> {
        gpui::Task::ready(Err(anyhow::anyhow!("peer stub")))
    }
    pub fn request_stream<T: proto::RequestMessage>(
        &self,
        _: ConnectionId,
        _: T,
    ) -> gpui::Task<Result<BoxStream<'static, Result<T::Response>>>> {
        gpui::Task::ready(Err(anyhow::anyhow!("peer stub")))
    }
    pub fn request_dynamic(
        &self,
        _: ConnectionId,
        _: proto::Envelope,
        _: &'static str,
    ) -> gpui::Task<Result<(proto::Envelope, Instant)>> {
        gpui::Task::ready(Err(anyhow::anyhow!("peer stub")))
    }
    pub fn request_stream_dynamic(
        &self,
        _: ConnectionId,
        _: proto::Envelope,
        _: &'static str,
    ) -> gpui::Task<Result<BoxStream<'static, Result<proto::Envelope>>>> {
        gpui::Task::ready(Err(anyhow::anyhow!("peer stub")))
    }
    pub fn send<T: proto::EnvelopedMessage>(&self, _: ConnectionId, _: T) -> Result<()> {
        Ok(())
    }
    pub fn send_dynamic(&self, _: ConnectionId, _: proto::Envelope) -> Result<()> {
        Ok(())
    }
    pub fn respond<T: proto::RequestMessage>(&self, _: Receipt<T>, _: T::Response) -> Result<()> {
        Ok(())
    }
    pub fn respond_with_unhandled_message(
        &self,
        _: ConnectionId,
        _: u32,
        _: &'static str,
    ) -> Result<()> {
        Ok(())
    }
}

#[cfg(feature = "gpui")]
mod gpui_impl {
    use super::*;
    use futures::future::LocalBoxFuture;
    use gpui::{AnyEntity, AnyWeakEntity, AsyncApp};
    use parking_lot::Mutex;
    use std::any::TypeId;
    use std::collections::HashMap;

    pub type ProtoMessageHandler = Arc<
        dyn Send
            + Sync
            + Fn(
                AnyEntity,
                Box<dyn proto::AnyTypedEnvelope>,
                AnyProtoClient,
                AsyncApp,
            ) -> LocalBoxFuture<'static, Result<()>>,
    >;

    #[derive(Default)]
    pub struct ProtoMessageHandlerSet {
        pub entity_types_by_message_type: HashMap<TypeId, TypeId>,
        pub entities_by_type_and_remote_id: HashMap<(TypeId, u64), EntityMessageSubscriber>,
        pub entity_id_extractors: HashMap<TypeId, fn(&dyn proto::AnyTypedEnvelope) -> u64>,
        pub entities_by_message_type: HashMap<TypeId, AnyWeakEntity>,
        pub message_handlers: HashMap<TypeId, ProtoMessageHandler>,
    }

    impl ProtoMessageHandlerSet {
        pub fn clear(&mut self) {
            self.entity_types_by_message_type.clear();
            self.entities_by_type_and_remote_id.clear();
            self.entity_id_extractors.clear();
            self.entities_by_message_type.clear();
            self.message_handlers.clear();
        }

        pub fn handle_message(
            _this: &Mutex<Self>,
            _: Box<dyn proto::AnyTypedEnvelope>,
            _: AnyProtoClient,
            _: AsyncApp,
        ) -> Option<LocalBoxFuture<'static, Result<()>>> {
            None
        }
    }

    pub enum EntityMessageSubscriber {
        Entity { handle: AnyWeakEntity },
        Pending(Vec<Box<dyn proto::AnyTypedEnvelope>>),
    }
}

#[cfg(feature = "gpui")]
pub use gpui_impl::*;

#[cfg(feature = "gpui")]
mod proto_client {
    use super::*;
    use anyhow::Result;
    use futures::future::BoxFuture;
    use futures::stream::BoxStream;
    use parking_lot::Mutex;
    use proto::{Envelope, EnvelopedMessage, RequestMessage};
    use std::sync::Arc;

    pub trait ProtoClient: Send + Sync {
        fn request(
            &self,
            envelope: Envelope,
            request_type: &'static str,
        ) -> BoxFuture<'static, Result<Envelope>>;
        fn request_stream(
            &self,
            envelope: Envelope,
            request_type: &'static str,
        ) -> BoxFuture<'static, Result<BoxStream<'static, Result<Envelope>>>>;
        fn send(&self, envelope: Envelope, message_type: &'static str) -> Result<()>;
        fn send_response(&self, envelope: Envelope, message_type: &'static str) -> Result<()>;
        fn message_handler_set(&self) -> &Mutex<gpui_impl::ProtoMessageHandlerSet>;
        fn is_via_collab(&self) -> bool;
        fn has_wsl_interop(&self) -> bool;
    }

    pub struct NoopProtoClient;

    impl NoopProtoClient {
        pub fn new() -> Self {
            Self
        }
    }

    impl ProtoClient for NoopProtoClient {
        fn request(&self, _: Envelope, _: &'static str) -> BoxFuture<'static, Result<Envelope>> {
            Box::pin(async { Err(anyhow::anyhow!("noop")) })
        }
        fn request_stream(
            &self,
            _: Envelope,
            _: &'static str,
        ) -> BoxFuture<'static, Result<BoxStream<'static, Result<Envelope>>>> {
            Box::pin(async { Err(anyhow::anyhow!("noop")) })
        }
        fn send(&self, _: Envelope, _: &'static str) -> Result<()> {
            Ok(())
        }
        fn send_response(&self, _: Envelope, _: &'static str) -> Result<()> {
            Ok(())
        }
        fn message_handler_set(&self) -> &Mutex<gpui_impl::ProtoMessageHandlerSet> {
            panic!("noop")
        }
        fn is_via_collab(&self) -> bool {
            false
        }
        fn has_wsl_interop(&self) -> bool {
            false
        }
    }

    #[derive(Clone)]
    pub struct AnyProtoClient(#[allow(dead_code)] Arc<dyn ProtoClient>);

    impl std::fmt::Debug for AnyProtoClient {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("AnyProtoClient").finish_non_exhaustive()
        }
    }

    impl AnyProtoClient {
        pub fn new(client: Arc<dyn ProtoClient>) -> Self {
            Self(client)
        }

        /// Convenience constructor wrapping a `NoopProtoClient`.
        pub fn noop() -> Self {
            Self(Arc::new(NoopProtoClient::new()))
        }

        pub fn add_entity_message_handler<M, E, H, F>(&self, _: H)
        where
            M: EnvelopedMessage,
            E: 'static,
            H: 'static + Send + Sync + Fn(gpui::Entity<E>, TypedEnvelope<M>, gpui::AsyncApp) -> F,
            F: 'static + std::future::Future<Output = Result<()>>,
        {
        }
        pub fn request<T: RequestMessage>(&self, _: T) -> gpui::Task<anyhow::Result<T::Response>> {
            gpui::Task::ready(Err(anyhow::anyhow!("rpc stub")))
        }
        pub fn request_stream<T: RequestMessage>(
            &self,
            _: T,
        ) -> gpui::Task<anyhow::Result<BoxStream<'static, anyhow::Result<T::Response>>>> {
            gpui::Task::ready(Err(anyhow::anyhow!("rpc stub")))
        }
        pub fn send<T: EnvelopedMessage>(&self, _: T) -> anyhow::Result<()> {
            Ok(())
        }
        pub fn send_response<T: EnvelopedMessage>(&self, _: u32, _: T) -> anyhow::Result<()> {
            Ok(())
        }
        pub fn send_lsp_response<T: proto::LspRequestMessage>(
            &self,
            _: u64,
            _: proto::LspRequestId,
            _: std::collections::HashMap<u64, T::Response, impl std::hash::BuildHasher>,
        ) -> anyhow::Result<()> {
            Ok(())
        }
        pub fn handle_lsp_response(&self, _: TypedEnvelope<proto::LspQueryResponse>) {}
        pub fn is_via_collab(&self) -> bool {
            false
        }
        pub fn has_wsl_interop(&self) -> bool {
            false
        }
        pub fn add_entity_stream_request_handler<M, E, H, F, S>(&self, _: H)
        where
            M: EnvelopedMessage + RequestMessage + proto::EntityMessage,
            E: 'static,
            H: 'static + Send + Sync + Fn(gpui::Entity<E>, TypedEnvelope<M>, gpui::AsyncApp) -> F,
            F: 'static + std::future::Future<Output = Result<S>>,
            S: futures::Stream<Item = Result<M::Response>> + Send + 'static,
        {
        }
        pub fn request_lsp<T>(
            &self,
            _: u64,
            _: Option<u64>,
            _: std::time::Duration,
            _: gpui::BackgroundExecutor,
            _: T,
        ) -> gpui::Task<
            anyhow::Result<Option<proto::TypedEnvelope<Vec<proto::ProtoLspResponse<T::Response>>>>>,
        >
        where
            T: proto::LspRequestMessage,
        {
            gpui::Task::ready(Ok(None))
        }
        pub fn add_entity_request_handler<M, E, H, F>(&self, _: H)
        where
            M: EnvelopedMessage + RequestMessage + proto::EntityMessage,
            E: 'static,
            H: 'static + Send + Sync + Fn(gpui::Entity<E>, TypedEnvelope<M>, gpui::AsyncApp) -> F,
            F: 'static + std::future::Future<Output = Result<M::Response>>,
        {
        }
        pub fn subscribe_to_entity<E: 'static>(&self, _: u64, _: &gpui::Entity<E>) {}
    }

    impl<T: ProtoClient + 'static> From<Arc<T>> for AnyProtoClient {
        fn from(client: Arc<T>) -> Self {
            Self(client)
        }
    }
}

#[cfg(feature = "gpui")]
pub use proto_client::*;
