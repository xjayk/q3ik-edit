// rpc is a thin re-export shim so existing `use rpc::proto`, `use rpc::AnyProtoClient`,
// `use rpc::TypedEnvelope` etc. call-sites continue to compile without change.
// No functional logic lives here — see crates/proto for the real types.

pub use proto;
pub use proto::{AnyProtoClient, ErrorCode, ErrorExt, Receipt, TypedEnvelope, error::*};

// ConnectionId is retained because it is part of the public API surface
// referenced by crates outside this repo boundary (e.g. collab tests).
// It is a pure data struct with no network behaviour.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ConnectionId {
    pub owner_id: u32,
    pub id: u32,
}

pub const PROTOCOL_VERSION: u32 = 68;
