use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;
use uuid::Uuid;

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
    /// * `page` - The page number to retrieve. Defaults to 1 if not provided.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn search_notification_logs(
        &self,
        request: &SearchNotificationLogsRequest,
        options: Option<RequestOptions>,
    ) -> Result<Vec<NotificationLog>, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "/v2/notificationlogs",
                Some(serde_json::to_value(&request.body).unwrap_or_default()),
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
    pub async fn get_notification_log(
        &self,
        uuid: &Uuid,
        options: Option<RequestOptions>,
    ) -> Result<NotificationLogDetail, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("/v2/notificationlogs/{}", uuid),
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
    pub async fn retry_notification_log(
        &self,
        uuid: &Uuid,
        options: Option<RequestOptions>,
    ) -> Result<NotificationLogDetail, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("/v2/notificationlogs/{}/retry", uuid),
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
    pub async fn bulk_retry_notification_logs(
        &self,
        request: &BulkRetryRequest,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "/v2/notificationlogs/retry",
                Some(serde_json::to_value(request).unwrap_or_default()),
                None,
                options,
            )
            .await
    }
}
