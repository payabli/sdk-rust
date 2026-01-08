use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct OcrClient {
    pub http_client: HttpClient,
}

impl OcrClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Use this endpoint to upload an image file for OCR processing. The accepted file formats include PDF, JPG, JPEG, PNG, and GIF. Specify the desired type of result (either 'bill' or 'invoice') in the path parameter `typeResult`. The response will contain the OCR processing results, including extracted data such as bill number, vendor information, bill items, and more.
    ///
    /// # Arguments
    ///
    /// * `request` - The image file to OCR. Accepted formats include PDF, JPG, JPEG, PNG, GIF.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn ocr_document_form(
        &self,
        type_result: &TypeResult,
        request: &FileContentImageOnly,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponseOcr, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("/Import/ocrDocumentForm/{}", type_result.0),
                Some(serde_json::to_value(request).unwrap_or_default()),
                None,
                options,
            )
            .await
    }

    /// Use this endpoint to submit a Base64-encoded image file for OCR processing. The accepted file formats include PDF, JPG, JPEG, PNG, and GIF. Specify the desired type of result (either 'bill' or 'invoice') in the path parameter `typeResult`. The response will contain the OCR processing results, including extracted data such as bill number, vendor information, bill items, and more.
    ///
    /// # Arguments
    ///
    /// * `request` - Base64-encoded file content for OCR processing
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn ocr_document_json(
        &self,
        type_result: &TypeResult,
        request: &FileContentImageOnly,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponseOcr, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("/Import/ocrDocumentJson/{}", type_result.0),
                Some(serde_json::to_value(request).unwrap_or_default()),
                None,
                options,
            )
            .await
    }
}
