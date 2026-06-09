use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct PayoutSubscriptionClient {
    pub http_client: HttpClient,
}

impl PayoutSubscriptionClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Creates a payout subscription to automatically send payouts to a vendor on a recurring schedule. See [Manage payout subscriptions](/guides/pay-out-developer-payout-subscriptions-manage) for a step-by-step guide.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn create_payout_subscription(
        &self,
        request: &RequestPayoutSchedule,
        options: Option<RequestOptions>,
    ) -> Result<AddPayoutSubscriptionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "PayoutSubscription",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves a single payout subscription's details. See [Manage payout subscriptions](/guides/pay-out-developer-payout-subscriptions-manage) for more information.
    ///
    /// # Arguments
    ///
    /// * `id` - The payout subscription ID.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_payout_subscription(
        &self,
        id: i64,
        options: Option<RequestOptions>,
    ) -> Result<GetPayoutSubscriptionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("PayoutSubscription/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Updates a payout subscription's details. See [Manage payout subscriptions](/guides/pay-out-developer-payout-subscriptions-manage) for more information.
    ///
    /// # Arguments
    ///
    /// * `id` - The payout subscription ID.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn update_payout_subscription(
        &self,
        id: i64,
        request: &UpdatePayoutSubscriptionBody,
        options: Option<RequestOptions>,
    ) -> Result<UpdatePayoutSubscriptionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("PayoutSubscription/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Deletes a payout subscription and prevents future payouts. See [Manage payout subscriptions](/guides/pay-out-developer-payout-subscriptions-manage) for more information.
    ///
    /// # Arguments
    ///
    /// * `id` - The payout subscription ID.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn delete_payout_subscription(
        &self,
        id: i64,
        options: Option<RequestOptions>,
    ) -> Result<DeletePayoutSubscriptionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("PayoutSubscription/{}", id),
                None,
                None,
                options,
            )
            .await
    }
}
