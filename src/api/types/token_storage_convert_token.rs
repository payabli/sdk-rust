pub use crate::prelude::*;

/// Object containing the information needed to convert a temporary token to a permanent token.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConvertToken {
    /// The type of payment method to tokenize. When converting a temp token to a permanent token, this should match the `method` set for the temporary token, either `ach` or `card`.
    #[serde(default)]
    pub method: String,
    /// A temporary stored token ID to be converted to permanent.
    #[serde(rename = "tokenId")]
    #[serde(default)]
    pub token_id: String,
}

impl ConvertToken {
    pub fn builder() -> ConvertTokenBuilder {
        <ConvertTokenBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConvertTokenBuilder {
    method: Option<String>,
    token_id: Option<String>,
}

impl ConvertTokenBuilder {
    pub fn method(mut self, value: impl Into<String>) -> Self {
        self.method = Some(value.into());
        self
    }

    pub fn token_id(mut self, value: impl Into<String>) -> Self {
        self.token_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConvertToken`].
    /// This method will fail if any of the following fields are not set:
    /// - [`method`](ConvertTokenBuilder::method)
    /// - [`token_id`](ConvertTokenBuilder::token_id)
    pub fn build(self) -> Result<ConvertToken, BuildError> {
        Ok(ConvertToken {
            method: self
                .method
                .ok_or_else(|| BuildError::missing_field("method"))?,
            token_id: self
                .token_id
                .ok_or_else(|| BuildError::missing_field("token_id"))?,
        })
    }
}
