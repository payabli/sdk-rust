use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct MoneyOutClient {
    pub http_client: HttpClient,
}

impl MoneyOutClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Authorizes a transaction for payout.
    ///
    /// If you don't pass `autoCapture` with a value of `true`, authorized transactions aren't flagged for settlement until captured. Use the `referenceId` returned in the response to capture the transaction.
    ///
    /// When `autoCapture` is `true`, Payabli captures the transaction asynchronously after authorization. The response confirms only that the transaction was authorized; it doesn't confirm that capture succeeded. To confirm capture, listen for the [`payout_transaction_approvedcaptured`](/developers/api-reference/webhooks-overview/payout-transaction-approved-captured) webhook event.
    ///
    /// If a velocity fraud alert is triggered, the endpoint returns a `202` response with `responseCode` `9051`, and the authorization is held for risk review rather than rejected. If a risk policy blocks the transaction, the endpoint returns a `422` response with `responseCode` `9005`, a terminal rejection.
    ///
    /// For check payouts, Payabli validates the remit (mailing) address at authorization. If the address fails deliverability validation, the endpoint returns a `422` response and doesn't charge the paypoint. Correct the address and re-authorize. Other payout rails (ACH, RTP, virtual card, wire, and managed payables) aren't affected.
    ///
    /// # Arguments
    ///
    /// * `allow_duplicated_bills` - When `true`, the authorization bypasses the requirement for unique bills, identified by vendor invoice number. This allows you to make more than one payout authorization for a bill, like a split payment.
    /// * `do_not_create_bills` - When `true`, Payabli won't automatically create a bill for this payout transaction.
    /// * `force_vendor_creation` - When `true`, the request creates a new vendor record, regardless of whether the vendor already exists.
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
    ///         .money_out
    ///         .authorize_out(
    ///             &RequestOutAuthorize {
    ///                 entry_point: Entrypointfield("8cfec329267".to_string()),
    ///                 order_description: Some(Orderdescription("Window Painting".to_string())),
    ///                 payment_method: AuthorizePaymentMethod {
    ///                     method: "managed".to_string(),
    ///                     ..Default::default()
    ///                 },
    ///                 payment_details: RequestOutAuthorizePaymentDetails {
    ///                     total_amount: Some(47.0),
    ///                     unbundled: Some(false),
    ///                     ..Default::default()
    ///                 },
    ///                 vendor_data: RequestOutAuthorizeVendorData {
    ///                     vendor_number: Some(VendorNumber("VEN-123".to_string())),
    ///                     ..Default::default()
    ///                 },
    ///                 invoice_data: vec![RequestOutAuthorizeInvoiceData {
    ///                     bill_id: Some(BillId(54323)),
    ///                     ..Default::default()
    ///                 }],
    ///                 auto_capture: Some(AutoCapture(true)),
    ///                 allow_duplicated_bills: None,
    ///                 do_not_create_bills: None,
    ///                 force_vendor_creation: None,
    ///                 source: None,
    ///                 order_id: None,
    ///                 account_id: None,
    ///                 subdomain: None,
    ///                 subscription_id: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn authorize_out(
        &self,
        request: &RequestOutAuthorize,
        options: Option<RequestOptions>,
    ) -> Result<AuthCapturePayoutResponse, ApiError> {
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
                "MoneyOut/authorize",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .bool(
                        "allowDuplicatedBills",
                        request.allow_duplicated_bills.clone(),
                    )
                    .bool("doNotCreateBills", request.do_not_create_bills.clone())
                    .bool("forceVendorCreation", request.force_vendor_creation.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Cancels an array of payout transactions.
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
    ///         .money_out
    ///         .cancel_all_out(
    ///             &vec!["2-29".to_string(), "2-28".to_string(), "2-27".to_string()],
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn cancel_all_out(
        &self,
        request: &Vec<String>,
        options: Option<RequestOptions>,
    ) -> Result<CaptureAllOutResponse, ApiError> {
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
                "MoneyOut/cancelAll",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Cancel a payout transaction by ID.
    ///
    /// # Arguments
    ///
    /// * `reference_id` - The ID for the payout transaction.
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
    ///         .money_out
    ///         .cancel_out_get(&"129-219".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn cancel_out_get(
        &self,
        reference_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponse0000, ApiError> {
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
                &format!("MoneyOut/cancel/{}", reference_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Cancel a payout transaction by ID.
    ///
    /// # Arguments
    ///
    /// * `reference_id` - The ID for the payout transaction.
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
    ///         .money_out
    ///         .cancel_out_delete(&"129-219".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn cancel_out_delete(
        &self,
        reference_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponse0000, ApiError> {
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
                &format!("MoneyOut/cancel/{}", reference_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Captures an array of authorized payout transactions for settlement. The maximum number of transactions that can be captured in a single request is 500.
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
    ///         .money_out
    ///         .capture_all_out(
    ///             &vec!["2-29".to_string(), "2-28".to_string(), "2-27".to_string()],
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn capture_all_out(
        &self,
        request: &Vec<String>,
        options: Option<RequestOptions>,
    ) -> Result<CaptureAllOutResponse, ApiError> {
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
                "MoneyOut/captureAll",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Captures a single authorized payout transaction by ID. If the transaction was authorized with `autoCapture` set to `true`, you don't need to call this endpoint to capture the transaction for processing.
    ///
    /// If a velocity fraud alert is triggered, the endpoint returns a `202` response with `responseCode` `9051`, and the capture is held for risk review rather than rejected. If a risk policy blocks the transaction, the endpoint returns a `422` response with `responseCode` `9005`, a terminal rejection.
    ///
    /// # Arguments
    ///
    /// * `reference_id` - The ID for the payout transaction.
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
    ///         .money_out
    ///         .capture_out(&"129-219".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn capture_out(
        &self,
        reference_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<AuthCapturePayoutResponse, ApiError> {
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
                &format!("MoneyOut/capture/{}", reference_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Returns details for a processed money out transaction.
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
    ///         .money_out
    ///         .payout_details(&"45-as456777hhhhhhhhhh77777777-324".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn payout_details(
        &self,
        trans_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<BillDetailResponse, ApiError> {
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
                &format!("MoneyOut/details/{}", trans_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Retrieves vCard details for a single card in an entrypoint.
    ///
    /// # Arguments
    ///
    /// * `card_token` - ID for a virtual card.
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
    ///         .money_out
    ///         .v_card_get(&"20230403315245421165".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn v_card_get(
        &self,
        card_token: &str,
        options: Option<RequestOptions>,
    ) -> Result<VCardGetResponse, ApiError> {
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
                &format!("MoneyOut/vcard/{}", card_token),
                None,
                None,
                options,
            )
            .await
    }

    /// Renews an expired or expiring virtual card by extending its expiration date to a future month.
    ///
    /// The card must be a virtual card that hasn't been fully used. The new expiration date must be in `MM-YYYY` or `MM/YYYY` format and no more than 2 years and 363 days in the future. The card expires on the last day of the month you specify.
    ///
    /// On success, `referenceId` holds the renewed card's token (the card processor may issue a new token). The response reuses the standard payout result object, so the payment-transaction fields it carries don't apply to renewal and always return `null`.
    ///
    /// # Arguments
    ///
    /// * `card_token` - ID for the virtual card to renew.
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
    ///         .money_out
    ///         .renew_v_card(
    ///             &"20231206142225226104".to_string(),
    ///             &RenewVCardRequest {
    ///                 expiration_date: "12-2027".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn renew_v_card(
        &self,
        card_token: &str,
        request: &RenewVCardRequest,
        options: Option<RequestOptions>,
    ) -> Result<RenewVCardResponse, ApiError> {
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
                &format!("MoneyOutCard/vcard/{}/renew", card_token),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Sends a virtual card link via email to the vendor associated with the `transId`.
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
    ///         .money_out
    ///         .send_v_card_link(
    ///             &SendVCardLinkRequest {
    ///                 trans_id: "01K33Z6YQZ6GD5QVKZ856MJBSC".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn send_v_card_link(
        &self,
        request: &SendVCardLinkRequest,
        options: Option<RequestOptions>,
    ) -> Result<OperationResult, ApiError> {
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
                "vcard/send-card-link",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieve the image of a check associated with a processed transaction.
    /// The check image is returned in the response body as a base64-encoded string.
    /// The check image is only available for payouts that have been processed.
    ///
    /// # Arguments
    ///
    /// * `asset_name` - Name of the check asset to retrieve. This is returned as `filename` in the `CheckData` object
    /// in the response when you make a GET request to `/MoneyOut/details/{transId}`.
    /// ```
    /// "CheckData": {
    /// "ftype": "PDF",
    /// "filename": "check133832686289732320_01JKBNZ5P32JPTZY8XXXX000000.pdf",
    /// "furl": "",
    /// "fContent": ""
    /// }
    /// ```
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
    ///         .money_out
    ///         .get_check_image(
    ///             &"check133832686289732320_01JKBNZ5P32JPTZY8XXXX000000.pdf".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_check_image(
        &self,
        asset_name: &str,
        options: Option<RequestOptions>,
    ) -> Result<String, ApiError> {
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
                &format!("MoneyOut/checkimage/{}", asset_name),
                None,
                None,
                options,
            )
            .await
    }

    /// Updates the status of a processed check payment transaction. This endpoint handles the status transition, updates related bills, creates audit events, and triggers notifications.
    ///
    /// The transaction must meet all of the following criteria:
    /// - **Status**: Must be in Processing or Processed status.
    /// - **Payment method**: Must be a check payment method.
    ///
    /// ### Allowed status values
    ///
    /// | Value | Status | Description |
    /// |-------|--------|-------------|
    /// | `0` | Cancelled/Voided | Cancels the check transaction. Reverts associated bills to their previous state (Approved or Active), creates "Cancelled" events, and sends a `payout_transaction_voidedcancelled` notification if the notification is enabled. |
    /// | `5` | Paid | Marks the check transaction as paid. Updates associated bills to "Paid" status, creates "Paid" events, and sends a `payout_transaction_paid` notification if the notification is enabled. |
    ///
    /// # Arguments
    ///
    /// * `trans_id` - The Payabli transaction ID for the check payment.
    /// * `check_payment_status` - The new status to apply to the check transaction. To mark a check as `Paid`, send 5. To mark a check as `Cancelled`, send 0.
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
    ///         .money_out
    ///         .update_check_payment_status(
    ///             &"TRANS123456".to_string(),
    ///             &AllowedCheckPaymentStatus::Paid,
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_check_payment_status(
        &self,
        trans_id: &str,
        check_payment_status: &AllowedCheckPaymentStatus,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponse00Responsedatanonobject, ApiError> {
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
                Method::PATCH,
                &format!("MoneyOut/status/{}/{}", trans_id, check_payment_status),
                None,
                None,
                options,
            )
            .await
    }

    /// Reissues a payout transaction with a new payment method. This creates a new transaction linked to the original and marks the original transaction as reissued.
    ///
    /// The original transaction must be in **Processing** or **Processed** status. The payment method in the request body is used directly. The endpoint doesn't fall back to vendor-managed payment methods.
    ///
    /// The new transaction goes through the standard authorize-and-capture flow automatically. Both the original and new transactions are linked through their event histories for audit purposes.
    ///
    /// # Arguments
    ///
    /// * `trans_id` - The transaction ID of the payout to reissue.
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
    ///         .money_out
    ///         .reissue_out(
    ///             &ReissueOutRequest {
    ///                 trans_id: "129-219".to_string(),
    ///                 payment_method: ReissuePaymentMethod {
    ///                     method: "ach".to_string(),
    ///                     ach_holder: Some("Acme Corp".to_string()),
    ///                     ach_routing: Some("021000021".to_string()),
    ///                     ach_account: Some("9876543210".to_string()),
    ///                     ach_account_type: Some("savings".to_string()),
    ///                     ach_holder_type: Some(AchHolderType::Business),
    ///                     ..Default::default()
    ///                 },
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn reissue_out(
        &self,
        request: &ReissueOutRequest,
        options: Option<RequestOptions>,
    ) -> Result<ReissuePayoutResponse, ApiError> {
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
                "MoneyOut/reissue",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .string("transId", request.trans_id.clone())
                    .build(),
                options,
            )
            .await
    }
}
