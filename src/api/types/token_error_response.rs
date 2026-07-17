pub use crate::prelude::*;

/// Error response from the token endpoint when the request is invalid, for example when the client credentials are wrong.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TokenErrorResponse {
    /// The error category, for example `InvalidCredentials`.
    #[serde(rename = "errorType")]
    #[serde(default)]
    pub error_type: String,
    /// A human-readable error description.
    #[serde(rename = "errorMessage")]
    #[serde(default)]
    pub error_message: String,
}

impl TokenErrorResponse {
    pub fn builder() -> TokenErrorResponseBuilder {
        <TokenErrorResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TokenErrorResponseBuilder {
    error_type: Option<String>,
    error_message: Option<String>,
}

impl TokenErrorResponseBuilder {
    pub fn error_type(mut self, value: impl Into<String>) -> Self {
        self.error_type = Some(value.into());
        self
    }

    pub fn error_message(mut self, value: impl Into<String>) -> Self {
        self.error_message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TokenErrorResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`error_type`](TokenErrorResponseBuilder::error_type)
    /// - [`error_message`](TokenErrorResponseBuilder::error_message)
    pub fn build(self) -> Result<TokenErrorResponse, BuildError> {
        Ok(TokenErrorResponse {
            error_type: self
                .error_type
                .ok_or_else(|| BuildError::missing_field("error_type"))?,
            error_message: self
                .error_message
                .ok_or_else(|| BuildError::missing_field("error_message"))?,
        })
    }
}
