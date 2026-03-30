pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TransferMessage {
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "RoomId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_id: Option<i64>,
    #[serde(rename = "UserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[serde(rename = "UserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(rename = "Content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "MessageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_type: Option<i64>,
    #[serde(rename = "MessageProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_properties: Option<TransferMessageProperties>,
}

impl TransferMessage {
    pub fn builder() -> TransferMessageBuilder {
        <TransferMessageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransferMessageBuilder {
    id: Option<i64>,
    room_id: Option<i64>,
    user_id: Option<i64>,
    user_name: Option<String>,
    content: Option<String>,
    created_at: Option<String>,
    message_type: Option<i64>,
    message_properties: Option<TransferMessageProperties>,
}

impl TransferMessageBuilder {
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

    pub fn message_properties(mut self, value: TransferMessageProperties) -> Self {
        self.message_properties = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TransferMessage`].
    pub fn build(self) -> Result<TransferMessage, BuildError> {
        Ok(TransferMessage {
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
