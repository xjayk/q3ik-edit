use serde::{Deserialize, Serialize};
use strum::VariantNames;

#[derive(Debug, Clone, PartialEq, Eq, VariantNames, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum Notification {
    ContactRequest {
        sender_id: u64,
    },
    ContactRequestAccepted {
        responder_id: u64,
    },
    ChannelInvitation {
        channel_id: u64,
        channel_name: String,
        inviter_id: u64,
    },
}

impl Notification {
    pub fn to_proto(&self) -> proto::Notification {
        proto::Notification::default()
    }

    pub fn from_proto(_notification: &proto::Notification) -> Option<Self> {
        None
    }

    pub fn all_variant_names() -> &'static [&'static str] {
        Self::VARIANTS
    }
}
