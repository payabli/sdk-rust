pub use crate::prelude::*;

/// Query parameters for GetMethod
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetMethodQueryRequest {
    /// Format for card expiration dates in the response.
    ///
    /// Accepted values:
    ///
    /// - 0: default, no formatting. Expiration dates are returned in the format they're saved in.
    ///
    /// - 1: MMYY
    ///
    /// - 2: MM/YY
    #[serde(rename = "cardExpirationFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_expiration_format: Option<i64>,
    /// When `true`, the request will include temporary tokens in the search and return details for a matching temporary token. The default behavior searches only for permanent tokens.
    #[serde(rename = "includeTemporary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_temporary: Option<bool>,
}

impl GetMethodQueryRequest {
    pub fn builder() -> GetMethodQueryRequestBuilder {
        <GetMethodQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetMethodQueryRequestBuilder {
    card_expiration_format: Option<i64>,
    include_temporary: Option<bool>,
}

impl GetMethodQueryRequestBuilder {
    pub fn card_expiration_format(mut self, value: i64) -> Self {
        self.card_expiration_format = Some(value);
        self
    }

    pub fn include_temporary(mut self, value: bool) -> Self {
        self.include_temporary = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetMethodQueryRequest`].
    pub fn build(self) -> Result<GetMethodQueryRequest, BuildError> {
        Ok(GetMethodQueryRequest {
            card_expiration_format: self.card_expiration_format,
            include_temporary: self.include_temporary,
        })
    }
}
