use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EventRequestBody {
    pub system_id: Option<String>,
    pub installation_id: Option<String>,
    pub session_id: Option<String>,
    pub metrics_id: Option<String>,
    pub is_staff: Option<bool>,
    pub app_version: String,
    pub os_name: String,
    pub os_version: Option<String>,
    pub architecture: String,
    pub release_channel: Option<String>,
    pub events: Vec<EventWrapper>,
}

impl EventRequestBody {
    pub fn semver(&self) -> Option<semver::Version> {
        self.app_version.parse().ok()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EventWrapper {
    pub signed_in: bool,
    pub milliseconds_since_first_event: i64,
    pub event: Event,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Event {
    Flexible(FlexibleEvent),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FlexibleEvent {
    pub event_type: String,
    pub event_properties: HashMap<String, serde_json::Value>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum EditPredictionRating {
    Positive,
    Negative,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssistantEventData {
    pub conversation_id: Option<String>,
    pub message_id: Option<String>,
    pub kind: AssistantKind,
    pub phase: AssistantPhase,
    pub model: String,
    pub model_provider: String,
    pub response_latency: Option<std::time::Duration>,
    pub error_message: Option<String>,
    pub language_name: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AssistantKind {
    Panel,
    Inline,
    InlineTerminal,
}

impl std::fmt::Display for AssistantKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "panel")
    }
}

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AssistantPhase {
    #[default]
    Response,
    Invoked,
    Accepted,
    Rejected,
}

impl std::fmt::Display for AssistantPhase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "response")
    }
}
