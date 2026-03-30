use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct PaypointClient {
    pub http_client: HttpClient,
}

impl PaypointClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Gets the basic details for a paypoint.
    ///
    /// # Arguments
    ///
    /// * `entry` - The paypoint's entrypoint identifier. [Learn more](/developers/api-reference/api-overview#entrypoint-vs-entry)
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_basic_entry(
        &self,
        entry: &str,
        options: Option<RequestOptions>,
    ) -> Result<GetBasicEntryResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("Paypoint/basic/{}", entry),
                None,
                None,
                options,
            )
            .await
    }

    /// Retrieves the basic details for a paypoint by ID.
    ///
    /// # Arguments
    ///
    /// * `id_paypoint` - Paypoint ID. You can find this value by querying `/api/Query/paypoints/{orgId}`
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_basic_entry_by_id(
        &self,
        id_paypoint: &str,
        options: Option<RequestOptions>,
    ) -> Result<GetBasicEntryByIdResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("Paypoint/basicById/{}", id_paypoint),
                None,
                None,
                options,
            )
            .await
    }

    /// Gets the details for a single paypoint.
    ///
    /// # Arguments
    ///
    /// * `entry` - The paypoint's entrypoint identifier. [Learn more](/developers/api-reference/api-overview#entrypoint-vs-entry)
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_entry_config(
        &self,
        entry: &str,
        request: &GetEntryConfigQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetEntryConfigResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("Paypoint/{}", entry),
                None,
                QueryBuilder::new()
                    .string("entrypages", request.entrypages.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Gets the details for single payment page for a paypoint.
    ///
    /// # Arguments
    ///
    /// * `entry` - The paypoint's entrypoint identifier. [Learn more](/developers/api-reference/api-overview#entrypoint-vs-entry)
    /// * `subdomain` - Payment page identifier. The subdomain value is the last portion of the payment page URL. For example, in`https://paypages-sandbox.payabli.com/513823dc10/pay-your-fees-1`, the subdomain is `pay-your-fees-1`.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_page(
        &self,
        entry: &str,
        subdomain: &str,
        options: Option<RequestOptions>,
    ) -> Result<PayabliPages, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("Paypoint/{}/{}", entry, subdomain),
                None,
                None,
                options,
            )
            .await
    }

    /// Deletes a payment page in a paypoint.
    ///
    /// # Arguments
    ///
    /// * `entry` - The paypoint's entrypoint identifier. [Learn more](/developers/api-reference/api-overview#entrypoint-vs-entry)
    /// * `subdomain` - Payment page identifier. The subdomain value is the last portion of the payment page URL. For example, in`https://paypages-sandbox.payabli.com/513823dc10/pay-your-fees-1`, the subdomain is `pay-your-fees-1`.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn remove_page(
        &self,
        entry: &str,
        subdomain: &str,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponseGeneric2Part, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("Paypoint/{}/{}", entry, subdomain),
                None,
                None,
                options,
            )
            .await
    }

    /// Updates a paypoint logo.
    ///
    /// # Arguments
    ///
    /// * `entry` - The paypoint's entrypoint identifier. [Learn more](/developers/api-reference/api-overview#entrypoint-vs-entry)
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn save_logo(
        &self,
        entry: &str,
        request: &FileContent,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponse00Responsedatanonobject, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("Paypoint/logo/{}", entry),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves an paypoint's basic settings like custom fields, identifiers, and invoicing settings.
    ///
    /// # Arguments
    ///
    /// * `entry` - The paypoint's entrypoint identifier. [Learn more](/developers/api-reference/api-overview#entrypoint-vs-entry)
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn settings_page(
        &self,
        entry: &str,
        options: Option<RequestOptions>,
    ) -> Result<SettingsQueryRecord, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("Paypoint/settings/{}", entry),
                None,
                None,
                options,
            )
            .await
    }

    /// Migrates a paypoint to a new parent organization.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn migrate(
        &self,
        request: &PaypointMoveRequest,
        options: Option<RequestOptions>,
    ) -> Result<MigratePaypointResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "Paypoint/migrate",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
