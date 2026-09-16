use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct TaptopayClient {
    pub http_client: HttpClient,
}

impl TaptopayClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Issues a short-lived activation code for a Tap to Pay device in the
    /// `Pending` state. This endpoint is for Tap to Pay devices only.
    /// Deliver the code to the device to complete activation.
    ///
    /// A code is valid for 30 minutes after it's issued. Calling this
    /// endpoint again for the same device before the code expires returns
    /// the same code, with `alreadyIssued` set to `true`, instead of
    /// generating a new one. A new code is only generated when no valid
    /// code exists.
    ///
    /// Authenticate with an OAuth2 bearer token that has the `pos_create`
    /// permission. See [Accept Tap to Pay payments](/guides/pay-in-developer-tap-to-pay)
    /// for the full integration guide.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use payabli_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .taptopay
    ///         .activation_challenge(
    ///             &TapToPayActivationChallengeRequest {
    ///                 entry: Entry("8cfec329267".to_string()),
    ///                 device_id: "499585-389fj484-3jcj8hj3".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn activation_challenge(
        &self,
        request: &TapToPayActivationChallengeRequest,
        options: Option<RequestOptions>,
    ) -> Result<TapToPayActivationChallengeResponse, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(&options, &[&["BearerAuth"] as &[&str]])
            .await?;
        let options = {
            let mut o = options.unwrap_or_default();
            for (header_key, header_value) in endpoint_auth_headers {
                o.additional_headers.insert(header_key, header_value);
            }
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                "v2/device/taptopay/activate/challenge",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
