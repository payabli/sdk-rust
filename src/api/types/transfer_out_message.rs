pub use crate::prelude::*;

/// A message associated with an outbound transfer.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
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

impl TransferOutMessage {
    pub fn builder() -> TransferOutMessageBuilder {
        <TransferOutMessageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransferOutMessageBuilder {
    id: Option<i64>,
    room_id: Option<i64>,
    user_id: Option<i64>,
    user_name: Option<String>,
    content: Option<String>,
    created_at: Option<String>,
    message_type: Option<i64>,
    message_properties: Option<TransferOutMessageProperties>,
}

impl TransferOutMessageBuilder {
    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn room_id(mut self, value: i64) -> Self {
        self.room_id = Some(value);
        self
    }

    pub fn user_id(mut self, value: i64) -> Self {
        self.user_id = Some(value);
        self
    }

    pub fn user_name(mut self, value: impl Into<String>) -> Self {
        self.user_name = Some(value.into());
        self
    }

    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn message_type(mut self, value: i64) -> Self {
        self.message_type = Some(value);
        self
    }

    pub fn message_properties(mut self, value: TransferOutMessageProperties) -> Self {
        self.message_properties = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TransferOutMessage`].
    pub fn build(self) -> Result<TransferOutMessage, BuildError> {
        Ok(TransferOutMessage {
            id: self.id,
            room_id: self.room_id,
            user_id: self.user_id,
            user_name: self.user_name,
            content: self.content,
            created_at: self.created_at,
            message_type: self.message_type,
            message_properties: self.message_properties,
        })
    }
}
