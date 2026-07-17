use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct TokenClient {
    pub http_client: HttpClient,
}

impl TokenClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Exchanges a client ID and client secret for a short-lived Bearer access token using the OAuth2 client-credentials flow. Designed for server-to-server use: the credentials and the returned token stay on your backend. Send the returned `access_token` in the `Authorization` header as `Bearer <access_token>` on subsequent API calls. See the [OAuth authentication guide](/developers/oauth-authentication) for the full flow.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn create_server_side_token(
        &self,
        request: &CreateServerSideTokenRequest,
        options: Option<RequestOptions>,
    ) -> Result<PayabliAccessTokenResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v2/Token/serverside",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
