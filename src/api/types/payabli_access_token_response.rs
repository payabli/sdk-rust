pub use crate::prelude::*;

/// Successful response from the token endpoint. Returns the access token, its lifetime, and any state echoed from the request.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PayabliAccessTokenResponse {
    /// The token type. Send the access token in the `Authorization` header as `Bearer <access_token>`.
    #[serde(default)]
    pub token_type: String,
    /// The access token to send on subsequent API calls.
    #[serde(default)]
    pub access_token: String,
    /// The token's lifetime in seconds. Request a new token when it expires.
    #[serde(default)]
    pub expires_in: i64,
    /// The opaque value sent in the request, echoed back. Present only when you send `state` in the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

impl PayabliAccessTokenResponse {
    pub fn builder() -> PayabliAccessTokenResponseBuilder {
        <PayabliAccessTokenResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PayabliAccessTokenResponseBuilder {
    token_type: Option<String>,
    access_token: Option<String>,
    expires_in: Option<i64>,
    state: Option<String>,
}

impl PayabliAccessTokenResponseBuilder {
    pub fn token_type(mut self, value: impl Into<String>) -> Self {
        self.token_type = Some(value.into());
        self
    }

    pub fn access_token(mut self, value: impl Into<String>) -> Self {
        self.access_token = Some(value.into());
        self
    }

    pub fn expires_in(mut self, value: i64) -> Self {
        self.expires_in = Some(value);
        self
    }

    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PayabliAccessTokenResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`token_type`](PayabliAccessTokenResponseBuilder::token_type)
    /// - [`access_token`](PayabliAccessTokenResponseBuilder::access_token)
    /// - [`expires_in`](PayabliAccessTokenResponseBuilder::expires_in)
    pub fn build(self) -> Result<PayabliAccessTokenResponse, BuildError> {
        Ok(PayabliAccessTokenResponse {
            token_type: self
                .token_type
                .ok_or_else(|| BuildError::missing_field("token_type"))?,
            access_token: self
                .access_token
                .ok_or_else(|| BuildError::missing_field("access_token"))?,
            expires_in: self
                .expires_in
                .ok_or_else(|| BuildError::missing_field("expires_in"))?,
            state: self.state,
        })
    }
}
