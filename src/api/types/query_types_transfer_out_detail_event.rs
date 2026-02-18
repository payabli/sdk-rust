pub use crate::prelude::*;

/// Event data for an outbound transfer detail.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TransferOutDetailEvent {
    /// Description of the transaction event.
    #[serde(rename = "TransEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trans_event: Option<String>,
    /// Additional event data.
    #[serde(rename = "EventData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_data: Option<String>,
    /// Time the event occurred.
    #[serde(rename = "EventTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_time: Option<String>,
}