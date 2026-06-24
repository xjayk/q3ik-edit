use futures::channel::mpsc;

#[macro_export]
macro_rules! event {
    ($name:expr $(,)?) => {};
    ($name:expr, $($rest:tt)*) => {};
}

pub fn init(_: mpsc::UnboundedSender<telemetry_events::FlexibleEvent>) {}
