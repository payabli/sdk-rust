use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct FundingClient {
    pub http_client: HttpClient,
}

impl FundingClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Deposits funds into a paypoint's available payout balance. Deposited funds enter a pending state and aren't available for instant payouts until confirmed through FBO reconciliation.
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
    ///         .funding
    ///         .deposit_funds(
    ///             &DepositFundsRequest {
    ///                 amount: 1500.0,
    ///                 entrypoint: Entrypointfield("48acde49".to_string()),
    ///                 account_id: "333".to_string(),
    ///                 paypoint_id: None,
    ///                 same_day_ach: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn deposit_funds(
        &self,
        request: &DepositFundsRequest,
        options: Option<RequestOptions>,
    ) -> Result<DepositFundsResponse, ApiError> {
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
                "Funding/depositFunds",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
