pub mod auth;

pub use proto;
pub use proto::{Receipt, TypedEnvelope, error::*};

pub mod conn;
pub mod notification;
pub mod peer;

pub use conn::Connection;
pub use notification::*;
pub use peer::*;

#[cfg(feature = "gpui")]
pub mod proto_client;
#[cfg(feature = "gpui")]
pub use proto_client::*;

pub const PROTOCOL_VERSION: u32 = 68;
