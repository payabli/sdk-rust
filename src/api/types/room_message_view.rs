pub use crate::prelude::*;

/// A note on a case.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RoomMessageView {
    /// The message's identifier.
    #[serde(default)]
    pub id: i64,
    /// The numeric id of the user who posted the note.
    #[serde(rename = "userId")]
    #[serde(default)]
    pub user_id: i64,
    /// The note text.
    #[serde(default)]
    pub content: String,
    /// When the note was posted.
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc")]
    pub created_at: DateTime<Utc>,
    /// When the note was last edited. Null when never edited.
    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc::option")]
    pub updated_at: Option<DateTime<Utc>>,
}

impl RoomMessageView {
    pub fn builder() -> RoomMessageViewBuilder {
        <RoomMessageViewBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RoomMessageViewBuilder {
    id: Option<i64>,
    user_id: Option<i64>,
    content: Option<String>,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
}

impl RoomMessageViewBuilder {
    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn user_id(mut self, value: i64) -> Self {
        self.user_id = Some(value);
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

    pub fn updated_at(mut self, value: DateTime<Utc>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RoomMessageView`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](RoomMessageViewBuilder::id)
    /// - [`user_id`](RoomMessageViewBuilder::user_id)
    /// - [`content`](RoomMessageViewBuilder::content)
    /// - [`created_at`](RoomMessageViewBuilder::created_at)
    pub fn build(self) -> Result<RoomMessageView, BuildError> {
        Ok(RoomMessageView {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            user_id: self
                .user_id
                .ok_or_else(|| BuildError::missing_field("user_id"))?,
            content: self
                .content
                .ok_or_else(|| BuildError::missing_field("content"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self.updated_at,
        })
    }
}
