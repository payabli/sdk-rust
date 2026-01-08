pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ApplicationDetailsRecordMessagesItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<CreatedAt>,
    #[serde(rename = "currentApplicationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_application_status: Option<i64>,
    #[serde(rename = "currentApplicationSubStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_application_sub_status: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "messageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_type: Option<i64>,
    #[serde(rename = "originalApplicationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_application_status: Option<i64>,
    #[serde(rename = "originalApplicationSubStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_application_sub_status: Option<i64>,
    #[serde(rename = "roomId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_id: Option<i64>,
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[serde(rename = "userName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}
