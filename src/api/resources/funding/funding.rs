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
    pub async fn deposit_funds(
        &self,
        request: &DepositFundsRequest,
        options: Option<RequestOptions>,
    ) -> Result<DepositFundsResponse, ApiError> {
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
