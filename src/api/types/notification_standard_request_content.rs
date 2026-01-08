pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct NotificationStandardRequestContent {
    /// The notification's event name.
    #[serde(rename = "eventType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<NotificationStandardRequestContentEventType>,
    /// Array of pairs key:value to insert in request body to target in **method** = *web*.
    #[serde(rename = "internalData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_data: Option<Vec<KeyValueDuo>>,
    /// Used internally to reference the entity or object generating the event.
    #[serde(rename = "transactionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    /// Array of pairs key:value to insert in header of request to target in **method** = *web*.
    #[serde(rename = "webHeaderParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_header_parameters: Option<Vec<KeyValueDuo>>,
}
