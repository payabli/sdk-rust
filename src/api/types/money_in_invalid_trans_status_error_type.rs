pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvalidTransStatusErrorType {
    /// Error message describing the reason for the decline
    #[serde(rename = "responseText")]
    #[serde(default)]
    pub response_text: String,
}

impl InvalidTransStatusErrorType {
    pub fn builder() -> InvalidTransStatusErrorTypeBuilder {
        <InvalidTransStatusErrorTypeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvalidTransStatusErrorTypeBuilder {
    response_text: Option<String>,
}

impl InvalidTransStatusErrorTypeBuilder {
    pub fn response_text(mut self, value: impl Into<String>) -> Self {
        self.response_text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InvalidTransStatusErrorType`].
    /// This method will fail if any of the following fields are not set:
    /// - [`response_text`](InvalidTransStatusErrorTypeBuilder::response_text)
    pub fn build(self) -> Result<InvalidTransStatusErrorType, BuildError> {
        Ok(InvalidTransStatusErrorType {
            response_text: self
                .response_text
                .ok_or_else(|| BuildError::missing_field("response_text"))?,
        })
    }
}
