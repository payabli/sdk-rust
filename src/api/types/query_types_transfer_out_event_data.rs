pub use crate::prelude::*;

/// Event data associated with an outbound transfer.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransferOutEventData {
    /// Description of the event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The time the event occurred.
    #[serde(rename = "eventTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_time: Option<String>,
    /// Reference data for the event.
    #[serde(rename = "refData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_data: Option<String>,
    /// Additional event data, which may contain detailed transaction information.
    #[serde(rename = "extraData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_data: Option<serde_json::Value>,
    /// The source of the event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}