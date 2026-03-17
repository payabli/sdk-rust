use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct WalletClient {
    pub http_client: HttpClient,
}

impl WalletClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Configure and activate Apple Pay for a Payabli organization
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn configure_apple_pay_organization(
        &self,
        request: &ConfigureOrganizationRequestApplePay,
        options: Option<RequestOptions>,
    ) -> Result<ConfigureApplePayOrganizationApiResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "Wallet/applepay/configure-organization",
                Some(serde_json::to_value(request).unwrap_or_default()),
                None,
                options,
            )
            .await
    }

    /// Configure and activate Apple Pay for a Payabli paypoint
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn configure_apple_pay_paypoint(
        &self,
        request: &ConfigurePaypointRequestApplePay,
        options: Option<RequestOptions>,
    ) -> Result<ConfigureApplePaypointApiResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "Wallet/applepay/configure-paypoint",
                Some(serde_json::to_value(request).unwrap_or_default()),
                None,
                options,
            )
            .await
    }

    /// Configure and activate Google Pay for a Payabli organization
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn configure_google_pay_organization(
        &self,
        request: &ConfigureOrganizationRequestGooglePay,
        options: Option<RequestOptions>,
    ) -> Result<ConfigureApplePayOrganizationApiResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "Wallet/googlepay/configure-organization",
                Some(serde_json::to_value(request).unwrap_or_default()),
                None,
                options,
            )
            .await
    }

    /// Configure and activate Google Pay for a Payabli paypoint
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn configure_google_pay_paypoint(
        &self,
        request: &ConfigurePaypointRequestGooglePay,
        options: Option<RequestOptions>,
    ) -> Result<ConfigureGooglePaypointApiResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "Wallet/googlepay/configure-paypoint",
                Some(serde_json::to_value(request).unwrap_or_default()),
                None,
                options,
            )
            .await
    }
}
