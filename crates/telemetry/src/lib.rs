use futures::channel::mpsc;

#[macro_export]
macro_rules! event {
    ($($tt:tt)*) => {
        ()
    };
}

pub fn init(_: mpsc::UnboundedSender<telemetry_events::FlexibleEvent>) {}
