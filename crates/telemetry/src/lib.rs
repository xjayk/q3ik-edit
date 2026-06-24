use futures::channel::mpsc;

#[macro_export]
macro_rules! event {
    ($name:expr $(, $key:ident = $value:expr)* $(,)?) => {};
    ($name:expr $(, $field:ident)* $(,)?) => {};
}

pub fn init(_: mpsc::UnboundedSender<telemetry_events::FlexibleEvent>) {}
