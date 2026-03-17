pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct NotificationRequest {
    /// Complete HTTP URL receiving the notification
    #[serde(rename = "notificationUrl")]
    pub notification_url: String,
    /// A dictionary of key-value pairs to be inserted in the header when the notification request is submitted
    #[serde(rename = "webHeaderParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_header_parameters: Option<Vec<WebHeaderParameter>>,
}
