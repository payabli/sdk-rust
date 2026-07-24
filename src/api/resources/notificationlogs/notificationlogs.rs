use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct NotificationlogsClient {
    pub http_client: HttpClient,
}

impl NotificationlogsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Search notification logs with filtering and pagination.
    /// - Start date and end date cannot be more than 30 days apart
    /// - Either `orgId` or `paypointId` must be provided
    ///
    /// This endpoint requires the `notifications_create` OR `notifications_read` permission.
    ///
    /// # Arguments
    ///
    /// * `page_size` - Number of records on each response page.
    /// * `page` - The page number to retrieve. Defaults to 1 if not provided.
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
    ///         .notificationlogs
    ///         .search_notification_logs(
    ///             &SearchNotificationLogsRequest {
    ///                 page_size: Some(Pagesize(20)),
    ///                 start_date: DateTime::parse_from_rfc3339("2024-01-01T00:00:00Z")
    ///                     .unwrap()
    ///                     .with_timezone(&Utc),
    ///                 end_date: DateTime::parse_from_rfc3339("2024-01-31T23:59:59Z")
    ///                     .unwrap()
    ///                     .with_timezone(&Utc),
    ///                 notification_event: Some("ActivatedMerchant".to_string()),
    ///                 succeeded: Some(true),
    ///                 org_id: Some(123),
    ///                 page: None,
    ///                 paypoint_id: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn search_notification_logs(
        &self,
        request: &SearchNotificationLogsRequest,
        options: Option<RequestOptions>,
    ) -> Result<Vec<NotificationLog>, ApiError> {
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
                "v2/notificationlogs",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .serialize("PageSize", request.page_size.clone())
                    .int("Page", request.page.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get detailed information for a specific notification log entry.
    /// This endpoint requires the `notifications_create` OR `notifications_read` permission.
    ///
    /// # Arguments
    ///
    /// * `uuid` - The notification log entry.
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
    ///         .notificationlogs
    ///         .get_notification_log(&"550e8400-e29b-41d4-a716-446655440000".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_notification_log(
        &self,
        uuid: &str,
        options: Option<RequestOptions>,
    ) -> Result<NotificationLogDetail, ApiError> {
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
                &format!("v2/notificationlogs/{}", uuid),
                None,
                None,
                options,
            )
            .await
    }

    /// Retry sending a specific notification.
    ///
    /// **Permissions:** notifications_create
    ///
    /// # Arguments
    ///
    /// * `uuid` - Unique id
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
    ///         .notificationlogs
    ///         .retry_notification_log(&"550e8400-e29b-41d4-a716-446655440000".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn retry_notification_log(
        &self,
        uuid: &str,
        options: Option<RequestOptions>,
    ) -> Result<NotificationLogDetail, ApiError> {
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
                &format!("v2/notificationlogs/{}/retry", uuid),
                None,
                None,
                options,
            )
            .await
    }

    /// Retry sending multiple notifications (maximum 50 IDs).
    /// This is an async process, so use the search endpoint again to check the notification status.
    ///
    /// This endpoint requires the `notifications_create` permission.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
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
    ///         .notificationlogs
    ///         .bulk_retry_notification_logs(
    ///             &BulkRetryRequest(vec![
    ///                 "550e8400-e29b-41d4-a716-446655440000".to_string(),
    ///                 "550e8400-e29b-41d4-a716-446655440001".to_string(),
    ///                 "550e8400-e29b-41d4-a716-446655440002".to_string(),
    ///             ]),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn bulk_retry_notification_logs(
        &self,
        request: &BulkRetryRequest,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
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
                "v2/notificationlogs/retry",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
