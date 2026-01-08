use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct CheckCaptureClient {
    pub http_client: HttpClient,
}

impl CheckCaptureClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Captures a check for Remote Deposit Capture (RDC) using the provided check images and details. This endpoint handles the OCR extraction of check data including MICR, routing number, account number, and amount. See the [RDC guide](/developers/developer-guides/pay-in-rdc) for more details.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn check_processing(
        &self,
        request: &CheckCaptureRequestBody,
        options: Option<RequestOptions>,
    ) -> Result<CheckCaptureResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "CheckCapture/CheckProcessing",
                Some(serde_json::to_value(request).unwrap_or_default()),
                None,
                options,
            )
            .await
    }
}
