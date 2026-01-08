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

    /// Authorizes transaction for payout. Authorized transactions aren't flagged for settlement until captured. Use `referenceId` returned in the response to capture the transaction.
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
    pub async fn authorize_out(
        &self,
        request: &AuthorizeOutRequest,
        options: Option<RequestOptions>,
    ) -> Result<AuthCapturePayoutResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "MoneyOut/authorize",
                Some(serde_json::to_value(&request.body).unwrap_or_default()),
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
    /// * `request` - Array of identifiers of payout transactions to cancel.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn cancel_all_out(
        &self,
        request: &Vec<String>,
        options: Option<RequestOptions>,
    ) -> Result<CaptureAllOutResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "MoneyOut/cancelAll",
                Some(serde_json::to_value(request).unwrap_or_default()),
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
    pub async fn cancel_out_get(
        &self,
        reference_id: &String,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponse0000, ApiError> {
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
    pub async fn cancel_out_delete(
        &self,
        reference_id: &String,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponse0000, ApiError> {
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
    /// * `request` - Array of identifiers of payout transactions to capture.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn capture_all_out(
        &self,
        request: &Vec<String>,
        options: Option<RequestOptions>,
    ) -> Result<CaptureAllOutResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "MoneyOut/captureAll",
                Some(serde_json::to_value(request).unwrap_or_default()),
                None,
                options,
            )
            .await
    }

    /// Captures a single authorized payout transaction by ID.
    ///
    /// # Arguments
    ///
    /// * `reference_id` - The ID for the payout transaction.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn capture_out(
        &self,
        reference_id: &String,
        options: Option<RequestOptions>,
    ) -> Result<AuthCapturePayoutResponse, ApiError> {
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
    pub async fn payout_details(
        &self,
        trans_id: &String,
        options: Option<RequestOptions>,
    ) -> Result<BillDetailResponse, ApiError> {
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
    pub async fn v_card_get(
        &self,
        card_token: &String,
        options: Option<RequestOptions>,
    ) -> Result<VCardGetResponse, ApiError> {
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

    /// Sends a virtual card link via email to the vendor associated with the `transId`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn send_v_card_link(
        &self,
        request: &SendVCardLinkRequest,
        options: Option<RequestOptions>,
    ) -> Result<OperationResult, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "vcard/send-card-link",
                Some(serde_json::to_value(request).unwrap_or_default()),
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
    pub async fn get_check_image(
        &self,
        asset_name: &String,
        options: Option<RequestOptions>,
    ) -> Result<String, ApiError> {
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
}
