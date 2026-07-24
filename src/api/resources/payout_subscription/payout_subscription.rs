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
    ///         .payout_subscription
    ///         .create_payout_subscription(
    ///             &RequestPayoutSchedule {
    ///                 entry_point: Entrypointfield("8cfec329267".to_string()),
    ///                 payment_method: AuthorizePaymentMethod {
    ///                     method: "ach".to_string(),
    ///                     ach_holder: Some("Herman Coatings".to_string()),
    ///                     ach_routing: Some("021000021".to_string()),
    ///                     ach_account: Some("3453445666".to_string()),
    ///                     ach_account_type: Some("checking".to_string()),
    ///                     ..Default::default()
    ///                 },
    ///                 payment_details: Some(PayoutPaymentDetail {
    ///                     total_amount: 500.0,
    ///                     service_fee: Some(0.0),
    ///                     currency: Some("USD".to_string()),
    ///                     ..Default::default()
    ///                 }),
    ///                 vendor_data: RequestOutAuthorizeVendorData {
    ///                     vendor_id: Some(Vendorid(456)),
    ///                     ..Default::default()
    ///                 },
    ///                 bill_data: Some(vec![BillPayOutDataRequest {
    ///                     due_date: Some(NaiveDate::parse_from_str("2025-08-15", "%Y-%m-%d").unwrap()),
    ///                     invoice_date: Some(
    ///                         NaiveDate::parse_from_str("2025-08-01", "%Y-%m-%d").unwrap(),
    ///                     ),
    ///                     invoice_number: Some(InvoiceNumber("INV-2345".to_string())),
    ///                     net_amount: Some(NetAmountstring("500".to_string())),
    ///                     ..Default::default()
    ///                 }]),
    ///                 schedule_details: Some(PayoutScheduleDetail {
    ///                     start_date: Some("09/01/2027".to_string()),
    ///                     end_date: Some("09/01/2026".to_string()),
    ///                     frequency: Some(Frequency::Monthly),
    ///                     ..Default::default()
    ///                 }),
    ///                 subdomain: None,
    ///                 account_id: None,
    ///                 source: None,
    ///                 set_pause: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_payout_subscription(
        &self,
        request: &RequestPayoutSchedule,
        options: Option<RequestOptions>,
    ) -> Result<AddPayoutSubscriptionResponse, ApiError> {
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
    ///         .payout_subscription
    ///         .get_payout_subscription(42, None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_payout_subscription(
        &self,
        id: i64,
        options: Option<RequestOptions>,
    ) -> Result<GetPayoutSubscriptionResponse, ApiError> {
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
    ///         .payout_subscription
    ///         .update_payout_subscription(
    ///             42,
    ///             &UpdatePayoutSubscriptionBody {
    ///                 set_pause: Some(PayoutSetPause(true)),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_payout_subscription(
        &self,
        id: i64,
        request: &UpdatePayoutSubscriptionBody,
        options: Option<RequestOptions>,
    ) -> Result<UpdatePayoutSubscriptionResponse, ApiError> {
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
    ///         .payout_subscription
    ///         .delete_payout_subscription(42, None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_payout_subscription(
        &self,
        id: i64,
        options: Option<RequestOptions>,
    ) -> Result<DeletePayoutSubscriptionResponse, ApiError> {
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
                &format!("PayoutSubscription/{}", id),
                None,
                None,
                options,
            )
            .await
    }
}
