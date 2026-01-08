use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct HostedPaymentPagesClient {
    pub http_client: HttpClient,
}

impl HostedPaymentPagesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Loads all of a payment page's details including `pageIdentifier` and `validationCode`. This endpoint requires an `application` API token.
    ///
    /// # Arguments
    ///
    /// * `entry` - The paypoint's entrypoint identifier. [Learn more](/api-reference/api-overview#entrypoint-vs-entry)
    /// * `subdomain` - Payment page identifier. The subdomain value is the last part of the payment page URL. For example, in`https://paypages-sandbox.payabli.com/513823dc10/pay-your-fees-1`, the subdomain is `pay-your-fees-1`.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn load_page(
        &self,
        entry: &String,
        subdomain: &String,
        options: Option<RequestOptions>,
    ) -> Result<PayabliPages, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("Paypoint/load/{}/{}", entry, subdomain),
                None,
                None,
                options,
            )
            .await
    }

    /// Creates a new payment page for a paypoint.
    /// Note: this operation doesn't create a new paypoint, just a payment page for an existing paypoint. Paypoints are created by the Payabli team when a boarding application is approved.
    ///
    /// # Arguments
    ///
    /// * `entry` - The paypoint's entrypoint identifier. [Learn more](/api-reference/api-overview#entrypoint-vs-entry)
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn new_page(
        &self,
        entry: &String,
        request: &PayabliPages,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponse00Responsedatanonobject, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("Paypoint/{}", entry),
                Some(serde_json::to_value(request).unwrap_or_default()),
                None,
                options,
            )
            .await
    }

    /// Updates a payment page in a paypoint.
    ///
    /// # Arguments
    ///
    /// * `entry` - The paypoint's entrypoint identifier. [Learn more](/api-reference/api-overview#entrypoint-vs-entry)
    /// * `subdomain` - Payment page identifier. The subdomain value is the last part of the payment page URL. For example, in`https://paypages-sandbox.payabli.com/513823dc10/pay-your-fees-1`, the subdomain is `pay-your-fees-1`.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn save_page(
        &self,
        entry: &String,
        subdomain: &String,
        request: &PayabliPages,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponse00Responsedatanonobject, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("Paypoint/{}/{}", entry, subdomain),
                Some(serde_json::to_value(request).unwrap_or_default()),
                None,
                options,
            )
            .await
    }
}
