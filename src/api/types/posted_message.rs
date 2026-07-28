pub use crate::prelude::*;

/// The result of posting a note to a case.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostedMessage {
    /// The new message's identifier.
    #[serde(rename = "messageId")]
    #[serde(default)]
    pub message_id: i64,
    /// The message room the note was posted to.
    #[serde(rename = "roomId")]
    #[serde(default)]
    pub room_id: i64,
    /// When the note was posted.
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc")]
    pub created_at: DateTime<Utc>,
}

impl PostedMessage {
    pub fn builder() -> PostedMessageBuilder {
        <PostedMessageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostedMessageBuilder {
    message_id: Option<i64>,
    room_id: Option<i64>,
    created_at: Option<DateTime<Utc>>,
}

impl PostedMessageBuilder {
    pub fn message_id(mut self, value: i64) -> Self {
        self.message_id = Some(value);
        self
    }

    pub fn room_id(mut self, value: i64) -> Self {
        self.room_id = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<Utc>) -> Self {
        self.created_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostedMessage`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message_id`](PostedMessageBuilder::message_id)
    /// - [`room_id`](PostedMessageBuilder::room_id)
    /// - [`created_at`](PostedMessageBuilder::created_at)
    pub fn build(self) -> Result<PostedMessage, BuildError> {
        Ok(PostedMessage {
            message_id: self
                .message_id
                .ok_or_else(|| BuildError::missing_field("message_id"))?,
            room_id: self
                .room_id
                .ok_or_else(|| BuildError::missing_field("room_id"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
