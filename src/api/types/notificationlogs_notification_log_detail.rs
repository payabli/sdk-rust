pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct NotificationLogDetail {
    #[serde(flatten)]
    pub notification_log_fields: NotificationLog,
    #[serde(rename = "webHeaders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_headers: Option<Vec<StringStringKeyValuePair>>,
    #[serde(rename = "responseHeaders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_headers: Option<Vec<KeyValueArray>>,
    #[serde(rename = "responseContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_content: Option<String>,
}
