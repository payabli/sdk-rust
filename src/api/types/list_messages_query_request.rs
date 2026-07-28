pub use crate::prelude::*;

/// Query parameters for ListMessages
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListMessagesQueryRequest {
    /// The maximum number of notes to return (default 50, max 200).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// An opaque cursor for the next page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl ListMessagesQueryRequest {
    pub fn builder() -> ListMessagesQueryRequestBuilder {
        <ListMessagesQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListMessagesQueryRequestBuilder {
    limit: Option<i64>,
    cursor: Option<String>,
}

impl ListMessagesQueryRequestBuilder {
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListMessagesQueryRequest`].
    pub fn build(self) -> Result<ListMessagesQueryRequest, BuildError> {
        Ok(ListMessagesQueryRequest {
            limit: self.limit,
            cursor: self.cursor,
        })
    }
}
