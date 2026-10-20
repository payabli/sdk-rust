use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct PaypointsClient {
    pub http_client: HttpClient,
}

impl PaypointsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists paypoints with optional filtering and pagination.
    ///
    /// # Arguments
    ///
    /// * `business_reference` - Filter by the business the paypoint belongs to.
    /// * `status` - Filter by paypoint status.
    /// * `search` - Free-text search across paypoints.
    /// * `limit` - Number of results per page.
    /// * `page` - Page number to return.
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
    ///         .paypoints
    ///         .list_paypoints(
    ///             &PaypointsListPaypointsQueryRequest {
    ///                 limit: Some(1),
    ///                 page: Some(1),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_paypoints(
        &self,
        request: &PaypointsListPaypointsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListPaypointsEnvelope, ApiError> {
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
                Method::GET,
                "v2/paypoints",
                None,
                QueryBuilder::new()
                    .string("businessReference", request.business_reference.clone())
                    .string("status", request.status.clone())
                    .string("search", request.search.clone())
                    .int("limit", request.limit.clone())
                    .int("page", request.page.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Creates a paypoint. New paypoints start in `pending`. Provide exactly
    /// one of `businessReference` (an existing business) or `business` (to
    /// create the business inline).
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
    ///         .paypoints
    ///         .create_paypoint(
    ///             &CreatePaypointPaypointsRequest::CreatePaypointWithBusinessReference(
    ///                 CreatePaypointWithBusinessReference {
    ///                     paypoint_writable_fields_fields: PaypointWritableFields {
    ///                         paypoint_type: Some("merchant".to_string()),
    ///                         doing_business_as: "Rose Gardens".to_string(),
    ///                         description: Some("Boutique flower shop".to_string()),
    ///                         website: Some("https://example.com".to_string()),
    ///                         customer_service_phone: Some("3055551212".to_string()),
    ///                         customer_service_email: Some("support@example.com".to_string()),
    ///                         timezone: Some("America/New_York".to_string()),
    ///                         ..Default::default()
    ///                     },
    ///                     business_reference: "019fed1a-3d6e-75d3-a81e-cba3874b0a55".to_string(),
    ///                     ..Default::default()
    ///                 },
    ///             ),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_paypoint(
        &self,
        request: &CreatePaypointPaypointsRequest,
        options: Option<RequestOptions>,
    ) -> Result<PaypointCreatedEnvelope, ApiError> {
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
                "v2/paypoints",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves a paypoint by reference.
    ///
    /// # Arguments
    ///
    /// * `paypoint_reference` - The paypoint reference.
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
    ///         .paypoints
    ///         .get_paypoint(&"019ff064-c690-7679-a73e-688418aadc7c".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_paypoint(
        &self,
        paypoint_reference: &str,
        options: Option<RequestOptions>,
    ) -> Result<PaypointDetailEnvelope, ApiError> {
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
                Method::GET,
                &format!("v2/paypoints/{}", paypoint_reference),
                None,
                None,
                options,
            )
            .await
    }

    /// Applies a partial update to a paypoint. Send only the fields you want to change.
    ///
    /// # Arguments
    ///
    /// * `paypoint_reference` - The paypoint reference.
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
    ///         .paypoints
    ///         .update_paypoint(
    ///             &"019ff064-c690-7679-a73e-688418aadc7c".to_string(),
    ///             &UpdatePaypointRequest {
    ///                 description: Some("Updated description".to_string()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_paypoint(
        &self,
        paypoint_reference: &str,
        request: &UpdatePaypointRequest,
        options: Option<RequestOptions>,
    ) -> Result<PaypointUpdatedEnvelope, ApiError> {
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
                Method::PATCH,
                &format!("v2/paypoints/{}", paypoint_reference),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
