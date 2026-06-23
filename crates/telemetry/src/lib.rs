use futures::channel::mpsc;

#[macro_export]
macro_rules! event {
    ($name:expr $(, $key:ident = $value:expr)* $(,)?) => {};
}

pub fn init(_: mpsc::UnboundedSender<telemetry_events::FlexibleEvent>) {}

