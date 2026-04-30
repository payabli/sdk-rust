use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct ManagementClient {
    pub http_client: HttpClient,
}

impl ManagementClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Verifies a bank account and returns detailed verification results from the verification network, including bank name, account status, and response codes. Unlike a pass/fail verification, this endpoint returns granular data to support decision-making and troubleshooting.
    ///
    /// When bank authentication is enabled for the paypoint's organization, the endpoint performs an identity verification check on the account holder. Otherwise, it performs an account existence check. When bank authentication is enabled, the `accountHolderType` and `holderName` fields are required.
    ///
    /// Requires `inboundpayments_create` or `outboundpayments_create` permission.
    ///
    /// # Arguments
    ///
    /// * `entry` - The paypoint's entry name identifier.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn verify_account_details(
        &self,
        entry: &str,
        request: &VerifyAccountDetailsRequest,
        options: Option<RequestOptions>,
    ) -> Result<VerifyAccountDetailsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("Management/verifyAccountDetails/{}", entry),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
