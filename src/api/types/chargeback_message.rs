pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ChargebackMessage {
    /// Message identifier.
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: i64,
    /// Room identifier for the message.
    #[serde(rename = "RoomId")]
    #[serde(default)]
    pub room_id: i64,
    /// User identifier who sent the message.
    #[serde(rename = "UserId")]
    #[serde(default)]
    pub user_id: i64,
    /// Name of the user who sent the message.
    #[serde(rename = "UserName")]
    #[serde(default)]
    pub user_name: String,
    /// Content of the message.
    #[serde(rename = "Content")]
    #[serde(default)]
    pub content: String,
    /// Timestamp when the message was created.
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc")]
    pub created_at: DateTime<Utc>,
    /// Type of message.
    #[serde(rename = "MessageType")]
    #[serde(default)]
    pub message_type: i64,
    /// Additional properties of the message.
    #[serde(rename = "MessageProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_properties: Option<HashMap<String, String>>,
}

impl ChargebackMessage {
    pub fn builder() -> ChargebackMessageBuilder {
        <ChargebackMessageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChargebackMessageBuilder {
    id: Option<i64>,
    room_id: Option<i64>,
    user_id: Option<i64>,
    user_name: Option<String>,
    content: Option<String>,
    created_at: Option<DateTime<Utc>>,
    message_type: Option<i64>,
    message_properties: Option<HashMap<String, String>>,
}

impl ChargebackMessageBuilder {
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

    pub fn created_at(mut self, value: DateTime<Utc>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn message_type(mut self, value: i64) -> Self {
        self.message_type = Some(value);
        self
    }

    pub fn message_properties(mut self, value: HashMap<String, String>) -> Self {
        self.message_properties = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ChargebackMessage`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ChargebackMessageBuilder::id)
    /// - [`room_id`](ChargebackMessageBuilder::room_id)
    /// - [`user_id`](ChargebackMessageBuilder::user_id)
    /// - [`user_name`](ChargebackMessageBuilder::user_name)
    /// - [`content`](ChargebackMessageBuilder::content)
    /// - [`created_at`](ChargebackMessageBuilder::created_at)
    /// - [`message_type`](ChargebackMessageBuilder::message_type)
    pub fn build(self) -> Result<ChargebackMessage, BuildError> {
        Ok(ChargebackMessage {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            room_id: self
                .room_id
                .ok_or_else(|| BuildError::missing_field("room_id"))?,
            user_id: self
                .user_id
                .ok_or_else(|| BuildError::missing_field("user_id"))?,
            user_name: self
                .user_name
                .ok_or_else(|| BuildError::missing_field("user_name"))?,
            content: self
                .content
                .ok_or_else(|| BuildError::missing_field("content"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            message_type: self
                .message_type
                .ok_or_else(|| BuildError::missing_field("message_type"))?,
            message_properties: self.message_properties,
        })
    }
}
