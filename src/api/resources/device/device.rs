use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct DeviceClient {
    pub http_client: HttpClient,
}

impl DeviceClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Generates a one-time, 6-digit verification code for activating a
    /// semi-integrated card-present device in a paypoint. After calling this endpoint, an operator enters the returned code
    /// on the device's terminal, along with a device name, to register the
    /// device to the paypoint resolved from `{entry}`.
    ///
    /// A code expires 5 minutes after it's issued. A paypoint can have several
    /// codes active at once — for example, when activating a batch of devices —
    /// and a code binds to whichever device enters it first.
    ///
    /// Authenticate with an OAuth2 Bearer token that has the `device_registry` scope.
    ///
    /// # Arguments
    ///
    /// * `entry` - The paypoint's entrypoint identifier. [Learn more](/developers/api-reference/api-overview#entrypoint-vs-entry)
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
    ///         .device
    ///         .challenge(&"8cfec329267".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn challenge(
        &self,
        entry: &str,
        options: Option<RequestOptions>,
    ) -> Result<DeviceChallengeResponse, ApiError> {
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
                &format!("Device/challenge/{}", entry),
                None,
                None,
                options,
            )
            .await
    }
}
