use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct PaymentLinkClient {
    pub http_client: HttpClient,
}

impl PaymentLinkClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Generates a payment link for an invoice from the invoice ID.
    ///
    /// # Arguments
    ///
    /// * `id_invoice` - Invoice ID
    /// * `amount_fixed` - Indicates whether customer can modify the payment amount. A value of `true` means the amount isn't modifiable, a value `false` means the payor can modify the amount to pay.
    /// * `mail_2` - List of recipient email addresses. When there is more than one, separate them by a semicolon (;).
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn add_pay_link_from_invoice(
        &self,
        id_invoice: i64,
        request: &AddPayLinkFromInvoiceRequest,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponsePaymentLinks, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("PaymentLink/{}", id_invoice),
                Some(serde_json::to_value(&request.body).unwrap_or_default()),
                QueryBuilder::new()
                    .bool("amountFixed", request.amount_fixed.clone())
                    .string("mail2", request.mail_2.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Generates a payment link for a bill from the bill ID.
    ///
    /// # Arguments
    ///
    /// * `bill_id` - The Payabli ID for the bill.
    /// * `amount_fixed` - Indicates whether customer can modify the payment amount. A value of `true` means the amount isn't modifiable, a value `false` means the payor can modify the amount to pay.
    /// * `mail_2` - List of recipient email addresses. When there is more than one, separate them by a semicolon (;).
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn add_pay_link_from_bill(
        &self,
        bill_id: i64,
        request: &AddPayLinkFromBillRequest,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponsePaymentLinks, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("PaymentLink/bill/{}", bill_id),
                Some(serde_json::to_value(&request.body).unwrap_or_default()),
                QueryBuilder::new()
                    .bool("amountFixed", request.amount_fixed.clone())
                    .string("mail2", request.mail_2.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Deletes a payment link by ID.
    ///
    /// # Arguments
    ///
    /// * `pay_link_id` - ID for the payment link.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn delete_pay_link_from_id(
        &self,
        pay_link_id: &String,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponsePaymentLinks, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("PaymentLink/{}", pay_link_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Retrieves a payment link by ID.
    ///
    /// # Arguments
    ///
    /// * `paylink_id` - ID for payment link
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_pay_link_from_id(
        &self,
        paylink_id: &String,
        options: Option<RequestOptions>,
    ) -> Result<GetPayLinkFromIdResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("PaymentLink/load/{}", paylink_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Send a payment link to the specified email addresses or phone numbers.
    ///
    /// # Arguments
    ///
    /// * `pay_link_id` - ID for the payment link.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn push_pay_link_from_id(
        &self,
        pay_link_id: &String,
        request: &PushPayLinkRequest,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponsePaymentLinks, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("PaymentLink/push/{}", pay_link_id),
                Some(serde_json::to_value(request).unwrap_or_default()),
                None,
                options,
            )
            .await
    }

    /// Refresh a payment link's content after an update.
    ///
    /// # Arguments
    ///
    /// * `pay_link_id` - ID for the payment link.
    /// * `amount_fixed` - Indicates whether customer can modify the payment amount. A value of `true` means the amount isn't modifiable, a value `false` means the payor can modify the amount to pay.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn refresh_pay_link_from_id(
        &self,
        pay_link_id: &String,
        request: &RefreshPayLinkFromIdQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponsePaymentLinks, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("PaymentLink/refresh/{}", pay_link_id),
                None,
                QueryBuilder::new()
                    .bool("amountFixed", request.amount_fixed.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Sends a payment link to the specified email addresses.
    ///
    /// # Arguments
    ///
    /// * `pay_link_id` - ID for the payment link.
    /// * `attachfile` - When `true`, attaches a PDF version of invoice to the email.
    /// * `mail_2` - List of recipient email addresses. When there is more than one, separate them by a semicolon (;).
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn send_pay_link_from_id(
        &self,
        pay_link_id: &String,
        request: &SendPayLinkFromIdQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponsePaymentLinks, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("PaymentLink/send/{}", pay_link_id),
                None,
                QueryBuilder::new()
                    .bool("attachfile", request.attachfile.clone())
                    .string("mail2", request.mail_2.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Updates a payment link's details.
    ///
    /// # Arguments
    ///
    /// * `pay_link_id` - ID for the payment link.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn update_pay_link_from_id(
        &self,
        pay_link_id: &String,
        request: &PayLinkUpdateData,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponsePaymentLinks, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("PaymentLink/update/{}", pay_link_id),
                Some(serde_json::to_value(request).unwrap_or_default()),
                None,
                options,
            )
            .await
    }

    /// Generates a vendor payment link for a specific bill lot number. This allows you to pay all bills with the same lot number for a vendor with a single payment link.
    ///
    /// # Arguments
    ///
    /// * `lot_number` - Lot number of the bills to pay. All bills with this lot number will be included.
    /// * `vendor_number` - The vendor number for the vendor being paid with this payment link.
    /// * `mail_2` - List of recipient email addresses. When there is more than one, separate them by a semicolon (;).
    /// * `amount_fixed` - Indicates whether customer can modify the payment amount. A value of `true` means the amount isn't modifiable, a value `false` means the payor can modify the amount to pay.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn add_pay_link_from_bill_lot_number(
        &self,
        lot_number: &String,
        request: &AddPayLinkFromBillLotNumberRequest,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponsePaymentLinks, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("PaymentLink/bill/lotNumber/{}", lot_number),
                Some(serde_json::to_value(&request.body).unwrap_or_default()),
                QueryBuilder::new()
                    .serialize("entryPoint", Some(request.entry_point.clone()))
                    .string("vendorNumber", request.vendor_number.clone())
                    .string("mail2", request.mail_2.clone())
                    .string("amountFixed", request.amount_fixed.clone())
                    .build(),
                options,
            )
            .await
    }
}
