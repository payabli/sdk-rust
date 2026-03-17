pub use crate::prelude::*;

/// Event associated with transaction processing
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TransactionDetailEvent {
    #[serde(rename = "transEvent")]
    pub trans_event: String,
    #[serde(rename = "eventData")]
    pub event_data: String,
    #[serde(rename = "eventTime")]
    pub event_time: String,
}
