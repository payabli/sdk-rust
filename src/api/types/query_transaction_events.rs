pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryTransactionEvents {
    /// Any data associated to the event received from processor. Contents vary by event type.
    #[serde(rename = "EventData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_data: Option<QueryTransactionEventsEventData>,
    /// Date and time of event.
    #[serde(rename = "EventTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc::option")]
    pub event_time: Option<DateTime<Utc>>,
    /// Event descriptor. See [TransEvent Reference](/developers/references/transevents) for more details.
    #[serde(rename = "TransEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trans_event: Option<String>,
}
