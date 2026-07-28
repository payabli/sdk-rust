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
    ///         .charge_backs
    ///         .add_response(
    ///             1000000,
    ///             &ResponseChargeBack {
    ///                 ..Default::default()
    ///             },
    ///             Some(
    ///                 RequestOptions::new()
    ///                     .additional_header("idempotencyKey", "6B29FC40-CA47-1067-B31D-00DD010662DA"),
    ///             ),
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn add_response(
        &self,
        id: i64,
        request: &ResponseChargeBack,
        options: Option<RequestOptions>,
    ) -> Result<AddResponseResponse, ApiError> {
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
                &format!("ChargeBacks/response/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves a chargeback record and its details.
    ///
    /// # Arguments
    ///
    /// * `id` - ID of the chargeback or return record. This is returned as `chargebackID` in the [ReceivedChargeBack](/developers/webhooks/payops-chargeback-received) and [ReceivedAchReturn](/developers/webhooks/payops-ach-return-received) webhook notifications.
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
    ///     client.charge_backs.get_chargeback(1000000, None).await;
    /// }
    /// ```
    pub async fn get_chargeback(
        &self,
        id: i64,
        options: Option<RequestOptions>,
    ) -> Result<ChargebackQueryRecords, ApiError> {
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
    ///         .charge_backs
    ///         .get_chargeback_attachment(1000000, &"fileName".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_chargeback_attachment(
        &self,
        id: i64,
        file_name: &str,
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
                &format!("ChargeBacks/getChargebackAttachments/{}/{}", id, file_name),
                None,
                None,
                options,
            )
            .await
    }
}
