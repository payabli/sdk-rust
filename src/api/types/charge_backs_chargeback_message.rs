pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChargebackMessage {
    /// Message identifier.
    #[serde(rename = "Id")]
    pub id: i64,
    /// Room identifier for the message.
    #[serde(rename = "RoomId")]
    pub room_id: i64,
    /// User identifier who sent the message.
    #[serde(rename = "UserId")]
    pub user_id: i64,
    /// Name of the user who sent the message.
    #[serde(rename = "UserName")]
    pub user_name: String,
    /// Content of the message.
    #[serde(rename = "Content")]
    pub content: String,
    /// Timestamp when the message was created.
    #[serde(rename = "CreatedAt")]
    #[serde(with = "crate::core::flexible_datetime::utc")]
    pub created_at: DateTime<Utc>,
    /// Type of message.
    #[serde(rename = "MessageType")]
    pub message_type: i64,
    /// Additional properties of the message.
    #[serde(rename = "MessageProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_properties: Option<HashMap<String, String>>,
}
