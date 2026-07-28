pub use crate::prelude::*;

/// A cursor-paginated page of case notes, ordered oldest to newest.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MessagePage {
    /// The notes on this page.
    #[serde(default)]
    pub messages: Vec<RoomMessageView>,
    /// The cursor for the next page. Null when there are no more notes.
    #[serde(rename = "nextCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

impl MessagePage {
    pub fn builder() -> MessagePageBuilder {
        <MessagePageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MessagePageBuilder {
    messages: Option<Vec<RoomMessageView>>,
    next_cursor: Option<String>,
}

impl MessagePageBuilder {
    pub fn messages(mut self, value: Vec<RoomMessageView>) -> Self {
        self.messages = Some(value);
        self
    }

    pub fn next_cursor(mut self, value: impl Into<String>) -> Self {
        self.next_cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MessagePage`].
    /// This method will fail if any of the following fields are not set:
    /// - [`messages`](MessagePageBuilder::messages)
    pub fn build(self) -> Result<MessagePage, BuildError> {
        Ok(MessagePage {
            messages: self
                .messages
                .ok_or_else(|| BuildError::missing_field("messages"))?,
            next_cursor: self.next_cursor,
        })
    }
}
