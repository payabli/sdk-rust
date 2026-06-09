use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct NotificationClient {
    pub http_client: HttpClient,
}

impl NotificationClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Create a new notification or auto-generated report.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn add_notification(
        &self,
        request: &AddNotificationRequest,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponseNotifications, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "Notification",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves a single notification or auto-generated report's details.
    ///
    /// # Arguments
    ///
    /// * `n_id` - Notification ID.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_notification(
        &self,
        n_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<NotificationQueryRecord, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("Notification/{}", n_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update a notification or auto-generated report.
    ///
    /// # Arguments
    ///
    /// * `n_id` - Notification ID.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn update_notification(
        &self,
        n_id: &str,
        request: &UpdateNotificationRequest,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponseNotifications, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("Notification/{}", n_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Deletes a single notification or auto-generated report.
    ///
    /// # Arguments
    ///
    /// * `n_id` - Notification ID.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn delete_notification(
        &self,
        n_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponseNotifications, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("Notification/{}", n_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Gets a copy of a generated report by ID.
    ///
    /// # Arguments
    ///
    /// * `id` - Report ID
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_report_file(
        &self,
        id: i64,
        options: Option<RequestOptions>,
    ) -> Result<File, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("Export/notificationReport/{}", id),
                None,
                None,
                options,
            )
            .await
    }
}
