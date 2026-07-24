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
    ///     client.subscription.get_subscription(231, None).await;
    /// }
    /// ```
    pub async fn get_subscription(
        &self,
        sub_id: i64,
        options: Option<RequestOptions>,
    ) -> Result<SubscriptionQueryRecords, ApiError> {
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
    ///         .subscription
    ///         .update_subscription(
    ///             231,
    ///             &RequestUpdateSchedule {
    ///                 set_pause: Some(SetPause(true)),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_subscription(
        &self,
        sub_id: i64,
        request: &RequestUpdateSchedule,
        options: Option<RequestOptions>,
    ) -> Result<UpdateSubscriptionResponse, ApiError> {
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
                Method::PUT,
                &format!("Subscription/{}", sub_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
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
    ///     client.subscription.remove_subscription(231, None).await;
    /// }
    /// ```
    pub async fn remove_subscription(
        &self,
        sub_id: i64,
        options: Option<RequestOptions>,
    ) -> Result<RemoveSubscriptionResponse, ApiError> {
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
                &format!("Subscription/{}", sub_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Creates a subscription or scheduled payment to run at a specified time and frequency. You can use stored payment method tokens for card, ACH, and digital wallets by passing them into the `paymentMethod.storedMethodId` field.
    ///
    /// # Arguments
    ///
    /// * `force_customer_creation` - When `true`, the request creates a new customer record, regardless of whether customer identifiers match an existing customer. Defaults to `false`.
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
    ///         .subscription
    ///         .new_subscription(
    ///             &RequestSchedule {
    ///                 customer_data: Some(PayorDataRequest {
    ///                     customer_id: Some(CustomerId(4440)),
    ///                     ..Default::default()
    ///                 }),
    ///                 entry_point: Some(Entrypointfield("8cfec329267".to_string())),
    ///                 payment_details: Some(PaymentDetail {
    ///                     service_fee: Some(0.0),
    ///                     total_amount: 100.0,
    ///                     ..Default::default()
    ///                 }),
    ///                 payment_method: Some(RequestSchedulePaymentMethod::PayMethodCredit(
    ///                     PayMethodCredit {
    ///                         cardcvv: Some(Cardcvv("123".to_string())),
    ///                         cardexp: Cardexp("12/29".to_string()),
    ///                         card_holder: Some(Cardholder("John Cassian".to_string())),
    ///                         cardnumber: Cardnumber("4111111111111111".to_string()),
    ///                         cardzip: Some(Cardzip("37615".to_string())),
    ///                         initiator: Some(Initiator("payor".to_string())),
    ///                         method: PayMethodCreditMethod::Card,
    ///                         save_if_success: None,
    ///                     },
    ///                 )),
    ///                 schedule_details: Some(ScheduleDetail {
    ///                     end_date: Some("2025-03-20".to_string()),
    ///                     frequency: Some(Frequency::Weekly),
    ///                     plan_id: Some(1),
    ///                     start_date: Some("2024-09-20".to_string()),
    ///                     ..Default::default()
    ///                 }),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn new_subscription(
        &self,
        request: &RequestSchedule,
        options: Option<RequestOptions>,
    ) -> Result<AddSubscriptionResponse, ApiError> {
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
                "Subscription/add",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
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
}
