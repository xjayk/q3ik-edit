pub struct Connection;

impl Connection {
    pub fn new<S>(_stream: S) -> Self where S: 'static + Send + Unpin + futures::Sink<async_tungstenite::tungstenite::Message, Error = anyhow::Error> + futures::Stream<Item = anyhow::Result<async_tungstenite::tungstenite::Message>> {
        Self
    }

    pub async fn send(&mut self, _message: async_tungstenite::tungstenite::Message) -> anyhow::Result<()> {
        Ok(())
    }
}
