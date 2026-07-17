pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateServerSideTokenRequest {
    /// The client ID issued for your integration when credentials are provisioned in the Payabli Portal.
    #[serde(rename = "clientId")]
    #[serde(default)]
    pub client_id: String,
    /// The client secret issued alongside the client ID. Keep it on your backend and never expose it in client-side code.
    #[serde(rename = "clientSecret")]
    #[serde(default)]
    pub client_secret: String,
    /// An optional opaque value echoed back in the response. Use it to correlate the request with its response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// An optional array of permission IDs that scopes the token to a subset of the credential's granted permissions. When omitted, the token carries all permissions granted to the credential.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
}

impl CreateServerSideTokenRequest {
    pub fn builder() -> CreateServerSideTokenRequestBuilder {
        <CreateServerSideTokenRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateServerSideTokenRequestBuilder {
    client_id: Option<String>,
    client_secret: Option<String>,
    state: Option<String>,
    permissions: Option<Vec<String>>,
}

impl CreateServerSideTokenRequestBuilder {
    pub fn client_id(mut self, value: impl Into<String>) -> Self {
        self.client_id = Some(value.into());
        self
    }

    pub fn client_secret(mut self, value: impl Into<String>) -> Self {
        self.client_secret = Some(value.into());
        self
    }

    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    pub fn permissions(mut self, value: Vec<String>) -> Self {
        self.permissions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateServerSideTokenRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`client_id`](CreateServerSideTokenRequestBuilder::client_id)
    /// - [`client_secret`](CreateServerSideTokenRequestBuilder::client_secret)
    pub fn build(self) -> Result<CreateServerSideTokenRequest, BuildError> {
        Ok(CreateServerSideTokenRequest {
            client_id: self
                .client_id
                .ok_or_else(|| BuildError::missing_field("client_id"))?,
            client_secret: self
                .client_secret
                .ok_or_else(|| BuildError::missing_field("client_secret"))?,
            state: self.state,
            permissions: self.permissions,
        })
    }
}
