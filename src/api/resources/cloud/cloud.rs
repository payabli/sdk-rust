use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct CloudClient {
    pub http_client: HttpClient,
}

impl CloudClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Register a cloud device to an entrypoint. See [Devices Quickstart](/developers/developer-guides/devices-quickstart#devices-quickstart) for a complete guide.
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
    ///         .cloud
    ///         .add_device(
    ///             &"8cfec329267".to_string(),
    ///             &DeviceEntry {
    ///                 description: Some("Front Desk POS".to_string()),
    ///                 registration_code: Some("YS7DS5".to_string()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn add_device(
        &self,
        entry: &str,
        request: &DeviceEntry,
        options: Option<RequestOptions>,
    ) -> Result<AddDeviceResponse, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(
                &options,
                &[&["BearerAuth"] as &[&str], &["APIKeyAuth"] as &[&str]],
            )
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
                &format!("Cloud/register/{}", entry),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Remove a cloud device from an entrypoint.
    ///
    /// # Arguments
    ///
    /// * `entry` - The paypoint's entrypoint identifier. [Learn more](/developers/api-reference/api-overview#entrypoint-vs-entry)
    /// * `device_id` - ID of the cloud device.
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
    ///         .cloud
    ///         .remove_device(
    ///             &"8cfec329267".to_string(),
    ///             &"499585-389fj484-3jcj8hj3".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn remove_device(
        &self,
        entry: &str,
        device_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<RemoveDeviceResponse, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(
                &options,
                &[&["BearerAuth"] as &[&str], &["APIKeyAuth"] as &[&str]],
            )
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
                Method::DELETE,
                &format!("Cloud/register/{}/{}", entry, device_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Retrieve the registration history for a device.
    ///
    /// # Arguments
    ///
    /// * `entry` - The paypoint's entrypoint identifier. [Learn more](/developers/api-reference/api-overview#entrypoint-vs-entry)
    /// * `device_id` - ID of the cloud device.
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
    ///         .cloud
    ///         .history_device(
    ///             &"8cfec329267".to_string(),
    ///             &"499585-389fj484-3jcj8hj3".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn history_device(
        &self,
        entry: &str,
        device_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<CloudQueryApiResponse, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(
                &options,
                &[&["BearerAuth"] as &[&str], &["APIKeyAuth"] as &[&str]],
            )
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
                Method::GET,
                &format!("Cloud/history/{}/{}", entry, device_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Use [List devices by paypoint](/developers/api-reference/get-list-of-devices-for-a-paypoint) instead, which supports filters, sorting, and pagination.
    ///
    /// Get a list of cloud devices registered to an entrypoint.
    ///
    /// # Arguments
    ///
    /// * `entry` - The paypoint's entrypoint identifier. [Learn more](/developers/api-reference/api-overview#entrypoint-vs-entry)
    /// * `force_refresh` - When `true`, the request retrieves an updated list of devices from the processor instead of returning a cached list of devices.
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
    ///         .cloud
    ///         .list_device(
    ///             &"8cfec329267".to_string(),
    ///             &ListDeviceQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_device(
        &self,
        entry: &str,
        request: &ListDeviceQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<CloudQueryApiResponse, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(
                &options,
                &[&["BearerAuth"] as &[&str], &["APIKeyAuth"] as &[&str]],
            )
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
                Method::GET,
                &format!("Cloud/list/{}", entry),
                None,
                QueryBuilder::new()
                    .bool("forceRefresh", request.force_refresh.clone())
                    .build(),
                options,
            )
            .await
    }
}
