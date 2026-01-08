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

    /// Authorize a card transaction. This returns an authorization code and reserves funds for the merchant. Authorized transactions aren't flagged for settlement until [captured](/api-reference/moneyin/capture-an-authorized-transaction).
    /// Only card transactions can be authorized. This endpoint can't be used for ACH transactions.
    /// <Tip>
    /// Consider migrating to the [v2 Authorize endpoint](/developers/api-reference/moneyinV2/authorize-a-transaction) to take advantage of unified response codes and improved response consistency.
    /// </Tip>
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn authorize(
        &self,
        request: &AuthorizeRequest,
        options: Option<RequestOptions>,
    ) -> Result<AuthResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "MoneyIn/authorize",
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

    /// <Warning>
    /// This endpoint is deprecated and will be sunset on November 24, 2025. Migrate to [POST `/capture/{transId}`](/api-reference/moneyin/capture-an-authorized-transaction)`.
    /// </Warning>
    ///
    /// Capture an [authorized
    /// transaction](/api-reference/moneyin/authorize-a-transaction) to complete the transaction and move funds from the customer to merchant account.
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
    pub async fn capture(
        &self,
        trans_id: &String,
        amount: f64,
        options: Option<RequestOptions>,
    ) -> Result<CaptureResponse, ApiError> {
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

    /// Capture an [authorized transaction](/api-reference/moneyin/authorize-a-transaction) to complete the transaction and move funds from the customer to merchant account.
    ///
    /// You can use this endpoint to capture both full and partial amounts of the original authorized transaction. See [Capture an authorized transaction](/developers/developer-guides/pay-in-auth-and-capture) for more information about this endpoint.
    ///
    /// <Tip>
    /// Consider migrating to the [v2 Capture endpoint](/developers/api-reference/moneyinV2/capture-an-authorized-transaction) to take advantage of unified response codes and improved response consistency.
    /// </Tip>
    ///
    /// # Arguments
    ///
    /// * `trans_id` - ReferenceId for the transaction (PaymentId).
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn capture_auth(
        &self,
        trans_id: &String,
        request: &CaptureRequest,
        options: Option<RequestOptions>,
    ) -> Result<CaptureResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("MoneyIn/capture/{}", trans_id),
                Some(serde_json::to_value(request).unwrap_or_default()),
                None,
                options,
            )
            .await
    }

    /// Make a temporary microdeposit in a customer account to verify the customer's ownership and access to the target account. Reverse the microdeposit with `reverseCredit`.
    ///
    /// This feature must be enabled by Payabli on a per-merchant basis. Contact support for help.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn credit(
        &self,
        request: &RequestCredit,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponse0, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "MoneyIn/makecredit",
                Some(serde_json::to_value(request).unwrap_or_default()),
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
    pub async fn details(
        &self,
        trans_id: &String,
        options: Option<RequestOptions>,
    ) -> Result<TransactionQueryRecordsCustomer, ApiError> {
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

    /// Make a single transaction. This method authorizes and captures a payment in one step.
    ///
    /// <Tip>
    /// Consider migrating to the [v2 Make a transaction endpoint](/developers/api-reference/moneyinV2/make-a-transaction) to take advantage of unified response codes and improved response consistency.
    /// </Tip>
    ///
    /// # Arguments
    ///
    /// * `include_details` - When `true`, transactionDetails object is returned in the response. See a full example of the `transactionDetails` object in the [Transaction integration guide](/developers/developer-guides/money-in-transaction-add#includedetailstrue-response).
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn getpaid(
        &self,
        request: &GetpaidRequest,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponseGetPaid, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "MoneyIn/getpaid",
                Some(serde_json::to_value(&request.body).unwrap_or_default()),
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

    /// A reversal either refunds or voids a transaction independent of the transaction's settlement status. Send a reversal request for a transaction, and Payabli automatically determines whether it's a refund or void. You don't need to know whether the transaction is settled or not.
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
    pub async fn reverse(
        &self,
        trans_id: &String,
        amount: f64,
        options: Option<RequestOptions>,
    ) -> Result<ReverseResponse, ApiError> {
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

    /// Refund a transaction that has settled and send money back to the account holder. If a transaction hasn't been settled, void it instead.
    ///
    /// <Tip>
    /// Consider migrating to the [v2 Refund endpoint](/developers/api-reference/moneyinV2/refund-a-settled-transaction) to take advantage of unified response codes and improved response consistency.
    /// </Tip>
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
    pub async fn refund(
        &self,
        trans_id: &String,
        amount: f64,
        options: Option<RequestOptions>,
    ) -> Result<RefundResponse, ApiError> {
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
    pub async fn refund_with_instructions(
        &self,
        trans_id: &String,
        request: &RequestRefund,
        options: Option<RequestOptions>,
    ) -> Result<RefundWithInstructionsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("MoneyIn/refund/{}", trans_id),
                Some(serde_json::to_value(request).unwrap_or_default()),
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
    pub async fn reverse_credit(
        &self,
        trans_id: &String,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponse, ApiError> {
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
    pub async fn send_receipt_2_trans(
        &self,
        trans_id: &String,
        request: &SendReceipt2TransQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ReceiptResponse, ApiError> {
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
    pub async fn validate(
        &self,
        request: &RequestPaymentValidate,
        options: Option<RequestOptions>,
    ) -> Result<ValidateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "MoneyIn/validate",
                Some(serde_json::to_value(request).unwrap_or_default()),
                None,
                options,
            )
            .await
    }

    /// Cancel a transaction that hasn't been settled yet. Voiding non-captured authorizations prevents future captures. If a transaction has been settled, refund it instead.
    ///
    /// <Tip>
    /// Consider migrating to the [v2 Void endpoint](/developers/api-reference/moneyinV2/void-a-transaction) to take advantage of unified response codes and improved response consistency.
    /// </Tip>
    ///
    /// # Arguments
    ///
    /// * `trans_id` - ReferenceId for the transaction (PaymentId).
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn void(
        &self,
        trans_id: &String,
        options: Option<RequestOptions>,
    ) -> Result<VoidResponse, ApiError> {
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

    /// Make a single transaction. This method authorizes and captures a payment in one step. This is the v2 version of the `api/MoneyIn/getpaid` endpoint, and returns the unified response format. See [Pay In unified response codes reference](/developers/references/pay-in-unified-response-codes) for more information.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn getpaidv_2(
        &self,
        request: &Getpaidv2Request,
        options: Option<RequestOptions>,
    ) -> Result<V2TransactionResponseWrapper, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v2/MoneyIn/getpaid",
                Some(serde_json::to_value(&request.body).unwrap_or_default()),
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

    /// Authorize a card transaction. This returns an authorization code and reserves funds for the merchant. Authorized transactions aren't flagged for settlement until captured. This is the v2 version of the `api/MoneyIn/authorize` endpoint, and returns the unified response format. See [Pay In unified response codes reference](/developers/references/pay-in-unified-response-codes) for more information.
    ///
    /// **Note**: Only card transactions can be authorized. This endpoint can't be used for ACH transactions.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn authorizev_2(
        &self,
        request: &Authorizev2Request,
        options: Option<RequestOptions>,
    ) -> Result<V2TransactionResponseWrapper, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v2/MoneyIn/authorize",
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

    /// Capture an authorized transaction to complete the transaction and move funds from the customer to merchant account. This is the v2 version of the `api/MoneyIn/capture/{transId}` endpoint, and returns the unified response format. See [Pay In unified response codes reference](/developers/references/pay-in-unified-response-codes) for more information.
    ///
    /// # Arguments
    ///
    /// * `trans_id` - ReferenceId for the transaction (PaymentId).
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn capturev_2(
        &self,
        trans_id: &String,
        request: &CaptureRequest,
        options: Option<RequestOptions>,
    ) -> Result<V2TransactionResponseWrapper, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v2/MoneyIn/capture/{}", trans_id),
                Some(serde_json::to_value(request).unwrap_or_default()),
                None,
                options,
            )
            .await
    }

    /// Give a full refund for a transaction that has settled and send money back to the account holder. To perform a partial refund, see [Partially refund a transaction](developers/api-reference/moneyinV2/partial-refund-a-settled-transaction).
    ///
    /// This is the v2 version of the refund endpoint, and returns the unified response format. See [Pay In unified response codes reference](/developers/references/pay-in-unified-response-codes) for more information.
    ///
    /// # Arguments
    ///
    /// * `trans_id` - ReferenceId for the transaction (PaymentId).
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn refundv_2(
        &self,
        trans_id: &String,
        options: Option<RequestOptions>,
    ) -> Result<V2TransactionResponseWrapper, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v2/MoneyIn/refund/{}", trans_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Refund a transaction that has settled and send money back to the account holder. If `amount` is omitted or set to 0, performs a full refund. When a non-zero `amount` is provided, this endpoint performs a partial refund.
    ///
    /// This is the v2 version of the refund endpoint, and returns the unified response format. See [Pay In unified response codes reference](/developers/references/pay-in-unified-response-codes) for more information.
    ///
    /// # Arguments
    ///
    /// * `trans_id` - ReferenceId for the transaction (PaymentId).
    /// * `amount` - Amount to refund from original transaction, minus any service fees charged on the original transaction. If omitted or set to 0, performs a full refund.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn refundv_2_amount(
        &self,
        trans_id: &String,
        amount: f64,
        options: Option<RequestOptions>,
    ) -> Result<V2TransactionResponseWrapper, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v2/MoneyIn/refund/{}/{}", trans_id, amount),
                None,
                None,
                options,
            )
            .await
    }

    /// Cancel a transaction that hasn't been settled yet. Voiding non-captured authorizations prevents future captures. This is the v2 version of the `api/MoneyIn/void/{transId}` endpoint, and returns the unified response format. See [Pay In unified response codes reference](/developers/references/pay-in-unified-response-codes) for more information.
    ///
    /// # Arguments
    ///
    /// * `trans_id` - ReferenceId for the transaction (PaymentId).
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn voidv_2(
        &self,
        trans_id: &String,
        options: Option<RequestOptions>,
    ) -> Result<V2TransactionResponseWrapper, ApiError> {
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
