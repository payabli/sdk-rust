pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostCaseMessageRequest {
    /// The note text (1 to 4000 characters).
    #[serde(default)]
    pub content: String,
}

impl PostCaseMessageRequest {
    pub fn builder() -> PostCaseMessageRequestBuilder {
        <PostCaseMessageRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostCaseMessageRequestBuilder {
    content: Option<String>,
}

impl PostCaseMessageRequestBuilder {
    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostCaseMessageRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content`](PostCaseMessageRequestBuilder::content)
    pub fn build(self) -> Result<PostCaseMessageRequest, BuildError> {
        Ok(PostCaseMessageRequest {
            content: self
                .content
                .ok_or_else(|| BuildError::missing_field("content"))?,
        })
    }
}
