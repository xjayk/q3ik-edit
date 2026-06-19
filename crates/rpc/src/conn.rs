#![allow(unused_imports)]
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

pub struct Connection;

impl Connection {
    pub fn new<S>(_stream: S) -> Self
    where
        S: 'static + Send + Unpin + futures::Sink<async_tungstenite::tungstenite::Message, Error = anyhow::Error>
            + futures::Stream<Item = anyhow::Result<async_tungstenite::tungstenite::Message>>,
    {
        Self
    }

    pub async fn send(&mut self, _message: async_tungstenite::tungstenite::Message) -> anyhow::Result<()> {
        Ok(())
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn in_memory(_executor: gpui::BackgroundExecutor) -> (Self, Self, Arc<AtomicBool>) {
        (Self, Self, Arc::new(AtomicBool::new(false)))
    }
}
