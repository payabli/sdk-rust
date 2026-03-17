use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct ChargeBacksClient {
    pub http_client: HttpClient,
}

impl ChargeBacksClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Add a response to a chargeback or ACH return.
    ///
    /// # Arguments
    ///
    /// * `id` - ID of the chargeback or return record.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn add_response(
        &self,
        id: i64,
        request: &ResponseChargeBack,
        options: Option<RequestOptions>,
    ) -> Result<AddResponseResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("ChargeBacks/response/{}", id),
                Some(serde_json::to_value(request).unwrap_or_default()),
                None,
                options,
            )
            .await
    }

    /// Retrieves a chargeback record and its details.
    ///
    /// # Arguments
    ///
    /// * `id` - ID of the chargeback or return record. This is returned as `chargebackId` in the [RecievedChargeback](/developers/developer-guides/webhook-payloads#receivedChargeback) and [ReceivedAchReturn](/developers/developer-guides/webhook-payloads#receivedachreturn) webhook notifications.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_chargeback(
        &self,
        id: i64,
        options: Option<RequestOptions>,
    ) -> Result<ChargebackQueryRecords, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("ChargeBacks/read/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Retrieves a chargeback attachment file by its file name.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of chargeback or return record.
    /// * `file_name` - The chargeback attachment's file name.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Text response
    pub async fn get_chargeback_attachment(
        &self,
        id: i64,
        file_name: &String,
        options: Option<RequestOptions>,
    ) -> Result<String, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("ChargeBacks/getChargebackAttachments/{}/{}", id, file_name),
                None,
                None,
                options,
            )
            .await
    }
}
