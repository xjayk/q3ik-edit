use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Event {
    Flexible(FlexibleEvent),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AssistantEventData {
    pub phase: AssistantPhase,
    pub conversation_id: String,
    pub kind: String,
    pub message_id: String,
    pub model: String,
    pub model_provider: String,
    pub response_latency: f64,
    pub error_message: Option<String>,
    pub language_name: Option<String>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum AssistantPhase {
    Response,
    Invoked,
    Accepted,
    Rejected,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EventWrapper {
    pub signed_in: bool,
    pub milliseconds_since_first_event: i64,
    pub event: Event,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FlexibleEvent {
    pub event_type: String,
    pub event_properties: HashMap<String, String>,
}
