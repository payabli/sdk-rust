use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct SubscriptionClient {
    pub http_client: HttpClient,
}

impl SubscriptionClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Retrieves a single subscription's details.
    ///
    /// # Arguments
    ///
    /// * `sub_id` - The subscription ID.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_subscription(
        &self,
        sub_id: i64,
        options: Option<RequestOptions>,
    ) -> Result<SubscriptionQueryRecords, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("Subscription/{}", sub_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Creates a subscription or scheduled payment to run at a specified time and frequency.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn new_subscription(
        &self,
        request: &NewSubscriptionRequest,
        options: Option<RequestOptions>,
    ) -> Result<AddSubscriptionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "Subscription/add",
                Some(serde_json::to_value(&request.body).unwrap_or_default()),
                QueryBuilder::new()
                    .serialize(
                        "forceCustomerCreation",
                        request.force_customer_creation.clone(),
                    )
                    .build(),
                options,
            )
            .await
    }

    /// Deletes a subscription, autopay, or recurring payment and prevents future charges.
    ///
    /// # Arguments
    ///
    /// * `sub_id` - The subscription ID.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn remove_subscription(
        &self,
        sub_id: i64,
        options: Option<RequestOptions>,
    ) -> Result<RemoveSubscriptionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("Subscription/{}", sub_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Updates a subscription's details.
    ///
    /// # Arguments
    ///
    /// * `sub_id` - The subscription ID.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn update_subscription(
        &self,
        sub_id: i64,
        request: &RequestUpdateSchedule,
        options: Option<RequestOptions>,
    ) -> Result<UpdateSubscriptionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("Subscription/{}", sub_id),
                Some(serde_json::to_value(request).unwrap_or_default()),
                None,
                options,
            )
            .await
    }
}
