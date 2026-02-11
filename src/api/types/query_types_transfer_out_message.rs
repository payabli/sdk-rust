pub use crate::prelude::*;

/// A message associated with an outbound transfer.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TransferOutMessage {
    /// Unique identifier for the message.
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The ID of the room where the message was sent.
    #[serde(rename = "RoomId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_id: Option<i64>,
    /// The ID of the user who sent the message.
    #[serde(rename = "UserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// The name of the user who sent the message.
    #[serde(rename = "UserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    /// The content of the message.
    #[serde(rename = "Content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// The time the message was created.
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// The type of message.
    #[serde(rename = "MessageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_type: Option<i64>,
    /// Additional properties for the message.
    #[serde(rename = "MessageProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_properties: Option<TransferOutMessageProperties>,
}
