#![allow(unused_variables)]
use crate::proto;
use anyhow::Result;
use futures::future::BoxFuture;
use futures::stream::{BoxStream, StreamExt};
use std::sync::Arc;
use std::time::Duration;

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

    pub fn send<T: proto::EnvelopedMessage>(
        &self,
        connection_id: ConnectionId,
        message: T,
    ) -> Result<()> {
        Ok(())
    }

    pub fn add_connection<F, Fut>(
        &self,
        connection: super::Connection,
        create_timer: F,
    ) -> (
        ConnectionId,
        BoxFuture<'static, Result<()>>,
        BoxStream<'static, Box<dyn proto::AnyTypedEnvelope>>,
    )
    where
        F: Fn(Duration) -> Fut,
        F: Send + 'static,
        Fut: std::future::Future<Output = ()> + Send + 'static,
    {
        let (_, rx) = futures::channel::mpsc::unbounded();
        (
            ConnectionId { owner_id: 0, id: 0 },
            Box::pin(async move { Ok(()) }),
            rx.boxed(),
        )
    }

    pub fn disconnect(&self, connection_id: ConnectionId) {}

    pub fn teardown(&self) {}

    pub fn request_envelope<T: proto::RequestMessage>(
        &self,
        connection_id: ConnectionId,
        request: T,
    ) -> impl std::future::Future<Output = Result<proto::Envelope>> + 'static {
        async move { Err(anyhow::anyhow!("stubbed")) }
    }

    pub fn request_stream<T: proto::RequestMessage>(
        &self,
        connection_id: ConnectionId,
        request: T,
    ) -> impl std::future::Future<Output = Result<BoxStream<'static, Result<proto::Envelope>>>> + 'static
    {
        async move { Err(anyhow::anyhow!("stubbed")) }
    }

    pub fn request_dynamic(
        &self,
        connection_id: ConnectionId,
        envelope: proto::Envelope,
        request_type: &'static str,
    ) -> BoxFuture<'static, Result<proto::Envelope>> {
        Box::pin(async move { Err(anyhow::anyhow!("stubbed")) })
    }

    pub fn respond(
        &self,
        connection_id: ConnectionId,
        original_envelope: &proto::Envelope,
        message: impl Into<proto::Envelope>,
    ) -> Result<()> {
        Ok(())
    }

    pub fn respond_with_unhandled_message(
        &self,
        sender_id: proto::PeerId,
        request_id: u32,
        type_name: &'static str,
    ) -> Result<()> {
        Ok(())
    }

    pub fn request_stream_dynamic(
        &self,
        connection_id: ConnectionId,
        envelope: proto::Envelope,
        request_type: &'static str,
    ) -> BoxFuture<'static, Result<BoxStream<'static, Result<proto::Envelope>>>> {
        Box::pin(async move { Err(anyhow::anyhow!("stubbed")) })
    }

    pub fn send_dynamic(
        &self,
        connection_id: ConnectionId,
        envelope: proto::Envelope,
    ) -> Result<()> {
        Ok(())
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn add_test_connection(
        &self,
        connection: super::Connection,
        executor: gpui::BackgroundExecutor,
    ) -> (
        ConnectionId,
        BoxFuture<'static, Result<()>>,
        futures::channel::mpsc::UnboundedReceiver<Result<proto::Envelope>>,
    ) {
        let (_, incoming) = futures::channel::mpsc::unbounded();
        (
            ConnectionId { owner_id: 0, id: 0 },
            Box::pin(async move { Ok(()) }),
            incoming,
        )
    }
}
