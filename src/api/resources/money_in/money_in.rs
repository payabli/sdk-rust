use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct MoneyInClient {
    pub http_client: HttpClient,
}

impl MoneyInClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// <Warning>
    /// This endpoint is deprecated. New integrations should use the [Authorize endpoint](/developers/api-reference/moneyinV2/authorize-a-transaction), then capture, void, or refund the resulting transaction with the corresponding endpoints. Transactions created with this legacy endpoint must be managed with the legacy lifecycle endpoints — they aren't interchangeable with the current ones.
    /// </Warning>
    ///
    ///
    /// Authorize a card transaction. This returns an authorization code and reserves funds for the merchant. Authorized transactions aren't flagged for settlement until [captured](/developers/api-reference/moneyin/capture-an-authorized-transaction).
    ///
    /// Only card transactions can be authorized. This endpoint can't be used for ACH transactions.
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
    ///         .money_in
    ///         .authorize(
    ///             &AuthorizeRequest {
    ///                 body: TransRequestBody {
    ///                     account_id: None,
    ///                     customer_data: Some(PayorDataRequest {
    ///                         customer_id: Some(CustomerId(4440)),
    ///                         ..Default::default()
    ///                     }),
    ///                     entry_point: Some(Entrypointfield("8cfec329267".to_string())),
    ///                     invoice_data: None,
    ///                     ipaddress: Some(IpAddress("255.255.255.255".to_string())),
    ///                     order_description: None,
    ///                     order_id: None,
    ///                     payment_details: PaymentDetail {
    ///                         service_fee: Some(0.0),
    ///                         total_amount: 100.0,
    ///                         ..Default::default()
    ///                     },
    ///                     payment_method: PaymentMethod::PayMethodCredit(PayMethodCredit {
    ///                         cardcvv: Some(Cardcvv("999".to_string())),
    ///                         cardexp: Cardexp("02/27".to_string()),
    ///                         card_holder: Some(Cardholder("John Cassian".to_string())),
    ///                         cardnumber: Cardnumber("4111111111111111".to_string()),
    ///                         cardzip: Some(Cardzip("12345".to_string())),
    ///                         initiator: Some(Initiator("payor".to_string())),
    ///                         method: PayMethodCreditMethod::Card,
    ///                         save_if_success: None,
    ///                     }),
    ///                     source: None,
    ///                     subdomain: None,
    ///                     subscription_id: None,
    ///                 },
    ///                 force_customer_creation: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn authorize(
        &self,
        request: &AuthorizeRequest,
        options: Option<RequestOptions>,
    ) -> Result<AuthResponse, ApiError> {
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
                "MoneyIn/authorize",
                Some(serde_json::to_value(&request.body).map_err(ApiError::Serialization)?),
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

    /// <Warning>
    /// This endpoint is deprecated. Use [POST `/capture/{transId}`](/developers/api-reference/moneyin/capture-an-authorized-transaction) instead, which supports partial captures and service fee adjustments.
    /// </Warning>
    ///
    /// Capture an [authorized
    /// transaction](/developers/api-reference/moneyin/authorize-a-transaction) to complete the transaction and move funds from the customer to merchant account.
    ///
    /// # Arguments
    ///
    /// * `trans_id` - ReferenceId for the transaction (PaymentId).
    /// * `amount` - Amount to be captured. The amount can't be greater the original total amount of the transaction. `0` captures the total amount authorized in the transaction. Partial captures aren't supported.
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
    ///         .money_in
    ///         .capture(
    ///             &"10-7d9cd67d-2d5d-4cd7-a1b7-72b8b201ec13".to_string(),
    ///             0.0,
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn capture(
        &self,
        trans_id: &str,
        amount: f64,
        options: Option<RequestOptions>,
    ) -> Result<CaptureResponse, ApiError> {
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
                &format!("MoneyIn/capture/{}/{}", trans_id, amount),
                None,
                None,
                options,
            )
            .await
    }

    /// <Warning>
    /// This endpoint is deprecated. Use it only to capture transactions originally authorized with the legacy [Authorize endpoint](/developers/api-reference/moneyin/authorize-a-transaction). New integrations should use the [Capture endpoint](/developers/api-reference/moneyinV2/capture-an-authorized-transaction), which only works on transactions authorized with the current [Authorize endpoint](/developers/api-reference/moneyinV2/authorize-a-transaction).
    /// </Warning>
    ///
    /// Capture an [authorized transaction](/developers/api-reference/moneyin/authorize-a-transaction) to complete the transaction and move funds from the customer to merchant account.
    ///
    /// You can use this endpoint to capture both full and partial amounts of the original authorized transaction. See [Capture an authorized transaction](/developers/developer-guides/pay-in-auth-and-capture) for more information about this endpoint.
    ///
    /// # Arguments
    ///
    /// * `trans_id` - ReferenceId for the transaction (PaymentId).
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
    ///         .money_in
    ///         .capture_auth(
    ///             &"10-7d9cd67d-2d5d-4cd7-a1b7-72b8b201ec13".to_string(),
    ///             &CaptureRequest {
    ///                 payment_details: CapturePaymentDetails {
    ///                     total_amount: 105.0,
    ///                     service_fee: Some(5.0),
    ///                     ..Default::default()
    ///                 },
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn capture_auth(
        &self,
        trans_id: &str,
        request: &CaptureRequest,
        options: Option<RequestOptions>,
    ) -> Result<CaptureResponse, ApiError> {
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
                &format!("MoneyIn/capture/{}", trans_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Make a temporary microdeposit in a customer account to verify the customer's ownership and access to the target account. Reverse the microdeposit with `reverseCredit`. Payabli doesn't automatically make microdeposits when you add a bank account, you must manually make the requests.
    ///
    /// This feature must be enabled by Payabli on a per-merchant basis. Contact support for help.
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
    ///         .money_in
    ///         .credit(
    ///             &RequestCredit {
    ///                 customer_data: PayorDataRequest {
    ///                     billing_address_1: Some(BillingAddressNullable(
    ///                         "5127 Linkwood ave".to_string(),
    ///                     )),
    ///                     customer_number: Some(CustomerNumberNullable("C-90010".to_string())),
    ///                     ..Default::default()
    ///                 },
    ///                 entrypoint: Some(Entrypointfield("8cfec329267".to_string())),
    ///                 payment_details: PaymentDetailCredit {
    ///                     service_fee: Some(0.0),
    ///                     total_amount: 1.0,
    ///                     ..Default::default()
    ///                 },
    ///                 payment_method: RequestCreditPaymentMethod {
    ///                     ach_account: Some(Achaccount("88354454".to_string())),
    ///                     ach_account_type: Some(Achaccounttype::Checking),
    ///                     ach_code: None,
    ///                     ach_holder: Some(AchHolder("John Smith".to_string())),
    ///                     ach_routing: Some(Achrouting("021000021".to_string())),
    ///                     method: RequestCreditPaymentMethodMethod::Ach,
    ///                 },
    ///                 force_customer_creation: None,
    ///                 account_id: None,
    ///                 order_description: None,
    ///                 order_id: None,
    ///                 source: None,
    ///                 subdomain: None,
    ///             },
    ///             Some(
    ///                 RequestOptions::new()
    ///                     .additional_header("idempotencyKey", "6B29FC40-CA47-1067-B31D-00DD010662DA"),
    ///             ),
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn credit(
        &self,
        request: &RequestCredit,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponse0, ApiError> {
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
                "MoneyIn/makecredit",
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

    /// Retrieve a processed transaction's details.
    ///
    /// # Arguments
    ///
    /// * `trans_id` - ReferenceId for the transaction (PaymentId).
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
    ///         .money_in
    ///         .details(&"45-as456777hhhhhhhhhh77777777-324".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn details(
        &self,
        trans_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<TransactionQueryRecordsCustomer, ApiError> {
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
                &format!("MoneyIn/details/{}", trans_id),
                None,
                None,
                options,
            )
            .await
    }

    /// <Warning>
    /// This endpoint is deprecated. New integrations should use the [Make a transaction endpoint](/developers/api-reference/moneyinV2/make-a-transaction) and manage the resulting transaction with the corresponding void or refund endpoints. Transactions created with this legacy endpoint must be managed with the legacy lifecycle endpoints — they aren't interchangeable with the current ones.
    /// </Warning>
    ///
    /// Make a single transaction. This method authorizes and captures a payment in one step.
    ///
    /// # Arguments
    ///
    /// * `ach_validation` - When `true`, enables real-time validation of ACH account and routing numbers. This is an add-on feature, contact Payabli for more information.
    /// * `force_customer_creation` - When `true`, the request creates a new customer record, regardless of whether customer identifiers match an existing customer. Defaults to `false`.
    /// * `include_details` - When `true`, transactionDetails object is returned in the response. See a full example of the `transactionDetails` object in the [Transaction integration guide](/developers/developer-guides/money-in-transaction-add#includedetailstrue-response).
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
    ///         .money_in
    ///         .getpaid(
    ///             &GetpaidRequest {
    ///                 body: TransRequestBody {
    ///                     account_id: None,
    ///                     customer_data: Some(PayorDataRequest {
    ///                         customer_id: Some(CustomerId(4440)),
    ///                         ..Default::default()
    ///                     }),
    ///                     entry_point: Some(Entrypointfield("8cfec329267".to_string())),
    ///                     invoice_data: None,
    ///                     ipaddress: Some(IpAddress("255.255.255.255".to_string())),
    ///                     order_description: None,
    ///                     order_id: None,
    ///                     payment_details: PaymentDetail {
    ///                         service_fee: Some(0.0),
    ///                         total_amount: 100.0,
    ///                         ..Default::default()
    ///                     },
    ///                     payment_method: PaymentMethod::PayMethodCredit(PayMethodCredit {
    ///                         cardcvv: Some(Cardcvv("999".to_string())),
    ///                         cardexp: Cardexp("02/27".to_string()),
    ///                         card_holder: Some(Cardholder("John Cassian".to_string())),
    ///                         cardnumber: Cardnumber("4111111111111111".to_string()),
    ///                         cardzip: Some(Cardzip("12345".to_string())),
    ///                         initiator: Some(Initiator("payor".to_string())),
    ///                         method: PayMethodCreditMethod::Card,
    ///                         save_if_success: None,
    ///                     }),
    ///                     source: None,
    ///                     subdomain: None,
    ///                     subscription_id: None,
    ///                 },
    ///                 ach_validation: None,
    ///                 force_customer_creation: None,
    ///                 include_details: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn getpaid(
        &self,
        request: &GetpaidRequest,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponseGetPaid, ApiError> {
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
                "MoneyIn/getpaid",
                Some(serde_json::to_value(&request.body).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .serialize("achValidation", request.ach_validation.clone())
                    .serialize(
                        "forceCustomerCreation",
                        request.force_customer_creation.clone(),
                    )
                    .bool("includeDetails", request.include_details.clone())
                    .build(),
                options,
            )
            .await
    }

    /// <Warning>
    /// This endpoint is deprecated and only works on transactions created with the legacy endpoints. There's no equivalent in the current endpoints. For transactions created with [Make a transaction](/developers/api-reference/moneyinV2/make-a-transaction) or [Authorize](/developers/api-reference/moneyinV2/authorize-a-transaction), check the transaction's settlement status and call [Void](/developers/api-reference/moneyinV2/void-a-transaction) or [Refund](/developers/api-reference/moneyinV2/refund-a-settled-transaction) based on the result.
    /// </Warning>
    ///
    /// A reversal either refunds or voids a transaction independent of the transaction's settlement status. Send a reversal request for a transaction, and Payabli automatically determines whether it's a refund or void. You don't need to know whether the transaction is settled or not. This endpoint only works on transactions made with the legacy endpoints. For transactions made with the current endpoints, check the transaction's settlement status and call void or refund based on the result.
    ///
    /// # Arguments
    ///
    /// * `trans_id` - ReferenceId for the transaction (PaymentId).
    /// * `amount` - Amount to reverse from original transaction, minus any service fees charged on the original transaction.
    ///
    /// The amount provided can't be greater than the original total amount of the transaction, minus service fees. For example, if a transaction was $90 plus a $10 service fee, you can reverse up to $90.
    ///
    /// An amount equal to zero will refunds the total amount authorized minus any service fee.
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
    ///         .money_in
    ///         .reverse(
    ///             &"10-3ffa27df-b171-44e0-b251-e95fbfc7a723".to_string(),
    ///             0.0,
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn reverse(
        &self,
        trans_id: &str,
        amount: f64,
        options: Option<RequestOptions>,
    ) -> Result<ReverseResponse, ApiError> {
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
                &format!("MoneyIn/reverse/{}/{}", trans_id, amount),
                None,
                None,
                options,
            )
            .await
    }

    /// <Warning>
    /// This endpoint is deprecated. Use it only to refund transactions originally created with the legacy endpoints. New integrations should use the [Refund endpoint](/developers/api-reference/moneyinV2/refund-a-settled-transaction), which only works on transactions created with [Make a transaction](/developers/api-reference/moneyinV2/make-a-transaction) or [Authorize](/developers/api-reference/moneyinV2/authorize-a-transaction).
    /// </Warning>
    ///
    /// Refund a transaction that has settled and send money back to the account holder. If a transaction hasn't been settled, void it instead.
    ///
    /// # Arguments
    ///
    /// * `trans_id` - ReferenceId for the transaction (PaymentId).
    /// * `amount` - Amount to refund from original transaction, minus any service fees charged on the original transaction.
    ///
    /// The amount provided can't be greater than the original total amount of the transaction, minus service fees. For example, if a transaction was \$90 plus a \$10 service fee, you can refund up to \$90.
    ///
    /// An amount equal to zero will refund the total amount authorized minus any service fee.
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
    ///         .money_in
    ///         .refund(
    ///             &"10-3ffa27df-b171-44e0-b251-e95fbfc7a723".to_string(),
    ///             0.0,
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn refund(
        &self,
        trans_id: &str,
        amount: f64,
        options: Option<RequestOptions>,
    ) -> Result<RefundResponse, ApiError> {
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
                &format!("MoneyIn/refund/{}/{}", trans_id, amount),
                None,
                None,
                options,
            )
            .await
    }

    /// <Warning>
    /// This endpoint is deprecated. Use it only to refund transactions originally created with the legacy endpoints. To refund a split-funded transaction created with [Make a transaction](/developers/api-reference/moneyinV2/make-a-transaction) or [Authorize](/developers/api-reference/moneyinV2/authorize-a-transaction), use the [Refund endpoint](/developers/api-reference/moneyinV2/refund-a-settled-transaction) with split instructions in the request body.
    /// </Warning>
    ///
    /// Refunds a settled transaction with split instructions.
    ///
    /// # Arguments
    ///
    /// * `trans_id` - ReferenceId for the transaction (PaymentId).
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
    ///         .money_in
    ///         .refund_with_instructions(
    ///             &"10-3ffa27df-b171-44e0-b251-e95fbfc7a723".to_string(),
    ///             &RequestRefund {
    ///                 amount: Some(100.0),
    ///                 order_description: Some(Orderdescription("Materials deposit".to_string())),
    ///                 refund_details: Some(RefundDetail {
    ///                     split_refunding: Some(vec![
    ///                         SplitFundingRefundContent {
    ///                             account_id: Some("187-342".to_string()),
    ///                             amount: Some(60.0),
    ///                             description: Some("Refunding undelivered materials".to_string()),
    ///                             origination_entry_point: Some("7f1a381696".to_string()),
    ///                             ..Default::default()
    ///                         },
    ///                         SplitFundingRefundContent {
    ///                             account_id: Some("187-343".to_string()),
    ///                             amount: Some(40.0),
    ///                             description: Some(
    ///                                 "Refunding deposit for undelivered materials".to_string(),
    ///                             ),
    ///                             origination_entry_point: Some("7f1a381696".to_string()),
    ///                             ..Default::default()
    ///                         },
    ///                     ]),
    ///                     ..Default::default()
    ///                 }),
    ///                 source: Some(Source("api".to_string())),
    ///                 ..Default::default()
    ///             },
    ///             Some(
    ///                 RequestOptions::new()
    ///                     .additional_header("idempotencyKey", "8A29FC40-CA47-1067-B31D-00DD010662DB"),
    ///             ),
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn refund_with_instructions(
        &self,
        trans_id: &str,
        request: &RequestRefund,
        options: Option<RequestOptions>,
    ) -> Result<RefundWithInstructionsResponse, ApiError> {
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
                &format!("MoneyIn/refund/{}", trans_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Reverse microdeposits that are used to verify customer account ownership and access. The `transId` value is returned in the success response for the original credit transaction made with `api/MoneyIn/makecredit`.
    ///
    /// # Arguments
    ///
    /// * `trans_id` - ReferenceId for the transaction (PaymentId).
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
    ///         .money_in
    ///         .reverse_credit(&"45-as456777hhhhhhhhhh77777777-324".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn reverse_credit(
        &self,
        trans_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponse, ApiError> {
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
                &format!("MoneyIn/reverseCredit/{}", trans_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Send a payment receipt for a transaction.
    ///
    /// # Arguments
    ///
    /// * `trans_id` - ReferenceId for the transaction (PaymentId).
    /// * `email` - Email address where the payment receipt should be sent.
    ///
    /// If not provided, the email address on file for the user owner of the transaction is used.
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
    ///         .money_in
    ///         .send_receipt_2_trans(
    ///             &"45-as456777hhhhhhhhhh77777777-324".to_string(),
    ///             &SendReceipt2TransQueryRequest {
    ///                 email: Some("example@email.com".to_string()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn send_receipt_2_trans(
        &self,
        trans_id: &str,
        request: &SendReceipt2TransQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ReceiptResponse, ApiError> {
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
                &format!("MoneyIn/sendreceipt/{}", trans_id),
                None,
                QueryBuilder::new()
                    .string("email", request.email.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Validates a card number without running a transaction or authorizing a charge.
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
    ///         .money_in
    ///         .validate(
    ///             &RequestPaymentValidate {
    ///                 entry_point: Entrypointfield("8cfec329267".to_string()),
    ///                 payment_method: RequestPaymentValidatePaymentMethod {
    ///                     method: RequestPaymentValidatePaymentMethodMethod::Card,
    ///                     cardnumber: Cardnumber("4360000001000005".to_string()),
    ///                     cardexp: Cardexp("12/29".to_string()),
    ///                     cardzip: Cardzip("14602-8328".to_string()),
    ///                     card_holder: Cardholder("Dianne Becker-Smith".to_string()),
    ///                 },
    ///                 account_id: None,
    ///                 order_description: None,
    ///                 order_id: None,
    ///             },
    ///             Some(
    ///                 RequestOptions::new()
    ///                     .additional_header("idempotencyKey", "6B29FC40-CA47-1067-B31D-00DD010662DA"),
    ///             ),
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn validate(
        &self,
        request: &RequestPaymentValidate,
        options: Option<RequestOptions>,
    ) -> Result<ValidateResponse, ApiError> {
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
                "MoneyIn/validate",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// <Warning>
    /// This endpoint is deprecated. Use it only to void transactions originally created with the legacy endpoints. New integrations should use the [Void endpoint](/developers/api-reference/moneyinV2/void-a-transaction), which only works on transactions created with [Make a transaction](/developers/api-reference/moneyinV2/make-a-transaction) or [Authorize](/developers/api-reference/moneyinV2/authorize-a-transaction).
    /// </Warning>
    ///
    /// Cancel a transaction that hasn't been settled yet. Voiding non-captured authorizations prevents future captures. If a transaction has been settled, refund it instead.
    ///
    /// # Arguments
    ///
    /// * `trans_id` - ReferenceId for the transaction (PaymentId).
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
    ///         .money_in
    ///         .void(&"10-3ffa27df-b171-44e0-b251-e95fbfc7a723".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn void(
        &self,
        trans_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<VoidResponse, ApiError> {
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
                &format!("MoneyIn/void/{}", trans_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Make a single transaction. This method authorizes and captures a payment in one step. This is the v2 version of the `api/MoneyIn/getpaid` endpoint, and returns the unified response format. See [Pay In unified response codes reference](/guides/pay-in-unified-response-codes-reference) for more information.
    ///
    /// # Arguments
    ///
    /// * `ach_validation` - When `true`, enables real-time validation of ACH account and routing numbers. This is an add-on feature, contact Payabli for more information.
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
    ///         .money_in
    ///         .getpaidv_2(
    ///             &Getpaidv2Request {
    ///                 body: TransRequestBody {
    ///                     account_id: None,
    ///                     customer_data: Some(PayorDataRequest {
    ///                         customer_id: Some(CustomerId(4440)),
    ///                         ..Default::default()
    ///                     }),
    ///                     entry_point: Some(Entrypointfield("8cfec329267".to_string())),
    ///                     invoice_data: None,
    ///                     ipaddress: Some(IpAddress("255.255.255.255".to_string())),
    ///                     order_description: None,
    ///                     order_id: None,
    ///                     payment_details: PaymentDetail {
    ///                         service_fee: Some(0.0),
    ///                         total_amount: 100.0,
    ///                         ..Default::default()
    ///                     },
    ///                     payment_method: PaymentMethod::PayMethodCredit(PayMethodCredit {
    ///                         cardcvv: Some(Cardcvv("999".to_string())),
    ///                         cardexp: Cardexp("02/27".to_string()),
    ///                         card_holder: Some(Cardholder("John Cassian".to_string())),
    ///                         cardnumber: Cardnumber("4111111111111111".to_string()),
    ///                         cardzip: Some(Cardzip("12345".to_string())),
    ///                         initiator: Some(Initiator("payor".to_string())),
    ///                         method: PayMethodCreditMethod::Card,
    ///                         save_if_success: None,
    ///                     }),
    ///                     source: None,
    ///                     subdomain: None,
    ///                     subscription_id: None,
    ///                 },
    ///                 ach_validation: None,
    ///                 force_customer_creation: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn getpaidv_2(
        &self,
        request: &Getpaidv2Request,
        options: Option<RequestOptions>,
    ) -> Result<V2TransactionResponseWrapper, ApiError> {
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
                "v2/MoneyIn/getpaid",
                Some(serde_json::to_value(&request.body).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .serialize("achValidation", request.ach_validation.clone())
                    .serialize(
                        "forceCustomerCreation",
                        request.force_customer_creation.clone(),
                    )
                    .build(),
                options,
            )
            .await
    }

    /// Authorize a card transaction. This returns an authorization code and reserves funds for the merchant. Authorized transactions aren't flagged for settlement until captured. This is the v2 version of the `api/MoneyIn/authorize` endpoint, and returns the unified response format. See [Pay In unified response codes reference](/guides/pay-in-unified-response-codes-reference) for more information.
    ///
    /// **Note**: Only card transactions can be authorized. This endpoint can't be used for ACH transactions.
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
    ///         .money_in
    ///         .authorizev_2(
    ///             &Authorizev2Request {
    ///                 body: TransRequestBody {
    ///                     account_id: None,
    ///                     customer_data: Some(PayorDataRequest {
    ///                         customer_id: Some(CustomerId(4440)),
    ///                         ..Default::default()
    ///                     }),
    ///                     entry_point: Some(Entrypointfield("8cfec329267".to_string())),
    ///                     invoice_data: None,
    ///                     ipaddress: Some(IpAddress("255.255.255.255".to_string())),
    ///                     order_description: None,
    ///                     order_id: None,
    ///                     payment_details: PaymentDetail {
    ///                         service_fee: Some(0.0),
    ///                         total_amount: 100.0,
    ///                         ..Default::default()
    ///                     },
    ///                     payment_method: PaymentMethod::PayMethodCredit(PayMethodCredit {
    ///                         cardcvv: Some(Cardcvv("999".to_string())),
    ///                         cardexp: Cardexp("02/27".to_string()),
    ///                         card_holder: Some(Cardholder("John Cassian".to_string())),
    ///                         cardnumber: Cardnumber("4111111111111111".to_string()),
    ///                         cardzip: Some(Cardzip("12345".to_string())),
    ///                         initiator: Some(Initiator("payor".to_string())),
    ///                         method: PayMethodCreditMethod::Card,
    ///                         save_if_success: None,
    ///                     }),
    ///                     source: None,
    ///                     subdomain: None,
    ///                     subscription_id: None,
    ///                 },
    ///                 force_customer_creation: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn authorizev_2(
        &self,
        request: &Authorizev2Request,
        options: Option<RequestOptions>,
    ) -> Result<V2TransactionResponseWrapper, ApiError> {
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
                "v2/MoneyIn/authorize",
                Some(serde_json::to_value(&request.body).map_err(ApiError::Serialization)?),
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

    /// Capture an authorized transaction to complete the transaction and move funds from the customer to merchant account. This is the v2 version of the `api/MoneyIn/capture/{transId}` endpoint, and returns the unified response format. See [Pay In unified response codes reference](/guides/pay-in-unified-response-codes-reference) for more information.
    ///
    /// # Arguments
    ///
    /// * `trans_id` - ReferenceId for the transaction (PaymentId).
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
    ///         .money_in
    ///         .capturev_2(
    ///             &"10-7d9cd67d-2d5d-4cd7-a1b7-72b8b201ec13".to_string(),
    ///             &CaptureRequest {
    ///                 payment_details: CapturePaymentDetails {
    ///                     total_amount: 105.0,
    ///                     service_fee: Some(5.0),
    ///                     ..Default::default()
    ///                 },
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn capturev_2(
        &self,
        trans_id: &str,
        request: &CaptureRequest,
        options: Option<RequestOptions>,
    ) -> Result<V2TransactionResponseWrapper, ApiError> {
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
                &format!("v2/MoneyIn/capture/{}", trans_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Give a full refund for a transaction that has settled and send money back to the account holder. To perform a partial refund, see [Partially refund a transaction](/developers/api-reference/moneyinV2/partial-refund-a-settled-transaction).
    ///
    /// This is the v2 version of the refund endpoint, and returns the unified response format. See [Pay In unified response codes reference](/guides/pay-in-unified-response-codes-reference) for more information.
    ///
    /// <Note>
    /// To refund a split-funded transaction, include split instructions in the request body. Omit the body for a standard refund.
    /// </Note>
    ///
    /// # Arguments
    ///
    /// * `trans_id` - ReferenceId for the transaction (PaymentId).
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
    ///         .money_in
    ///         .refundv_2(
    ///             &"10-3ffa27df-b171-44e0-b251-e95fbfc7a723".to_string(),
    ///             &Default::default(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn refundv_2(
        &self,
        trans_id: &str,
        request: &RefundV2Request,
        options: Option<RequestOptions>,
    ) -> Result<V2TransactionResponseWrapper, ApiError> {
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
                &format!("v2/MoneyIn/refund/{}", trans_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Refund a transaction that has settled and send money back to the account holder. If `amount` is set to 0, performs a full refund. When a non-zero `amount` is provided, this endpoint performs a partial refund.
    ///
    /// This is the v2 version of the refund endpoint, and returns the unified response format. See [Pay In unified response codes reference](/guides/pay-in-unified-response-codes-reference) for more information.
    ///
    /// <Note>
    /// For a standard refund, whether full (`amount` set to 0) or partial, send no request body. Include a request body only to refund a split-funded transaction, with split instructions in `refundDetails`.
    /// </Note>
    ///
    /// # Arguments
    ///
    /// * `trans_id` - ReferenceId for the transaction (PaymentId).
    /// * `amount` - Amount to refund from original transaction, minus any service fees charged on the original transaction. If set to 0, performs a full refund.
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
    ///         .money_in
    ///         .refundv_2_amount(
    ///             &"10-3ffa27df-b171-44e0-b251-e95fbfc7a723".to_string(),
    ///             0.0,
    ///             &Default::default(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn refundv_2_amount(
        &self,
        trans_id: &str,
        amount: f64,
        request: &RefundV2Request,
        options: Option<RequestOptions>,
    ) -> Result<V2TransactionResponseWrapper, ApiError> {
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
                &format!("v2/MoneyIn/refund/{}/{}", trans_id, amount),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Cancel a transaction that hasn't been settled yet. Voiding non-captured authorizations prevents future captures. This is the v2 version of the `api/MoneyIn/void/{transId}` endpoint, and returns the unified response format. See [Pay In unified response codes reference](/guides/pay-in-unified-response-codes-reference) for more information.
    ///
    /// # Arguments
    ///
    /// * `trans_id` - ReferenceId for the transaction (PaymentId).
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
    ///         .money_in
    ///         .voidv_2(&"10-3ffa27df-b171-44e0-b251-e95fbfc7a723".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn voidv_2(
        &self,
        trans_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<V2TransactionResponseWrapper, ApiError> {
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
                &format!("v2/MoneyIn/void/{}", trans_id),
                None,
                None,
                options,
            )
            .await
    }
}
