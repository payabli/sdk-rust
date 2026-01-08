use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct OrganizationClient {
    pub http_client: HttpClient,
}

impl OrganizationClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Creates an organization under a parent organization. This is also referred to as a suborganization.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn add_organization(
        &self,
        request: &AddOrganizationRequest,
        options: Option<RequestOptions>,
    ) -> Result<AddOrganizationResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "Organization",
                Some(serde_json::to_value(request).unwrap_or_default()),
                None,
                options,
            )
            .await
    }

    /// Delete an organization by ID.
    ///
    /// # Arguments
    ///
    /// * `org_id` - The numeric identifier for organization, assigned by Payabli.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn delete_organization(
        &self,
        org_id: i64,
        options: Option<RequestOptions>,
    ) -> Result<DeleteOrganizationResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("Organization/{}", org_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Updates an organization's details by ID.
    ///
    /// # Arguments
    ///
    /// * `org_id` - The numeric identifier for organization, assigned by Payabli.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn edit_organization(
        &self,
        org_id: i64,
        request: &OrganizationData,
        options: Option<RequestOptions>,
    ) -> Result<EditOrganizationResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("Organization/{}", org_id),
                Some(serde_json::to_value(request).unwrap_or_default()),
                None,
                options,
            )
            .await
    }

    /// Gets an organization's basic information by entry name (entrypoint identifier).
    ///
    /// # Arguments
    ///
    /// * `entry` - The paypoint's entrypoint identifier. [Learn more](/api-reference/api-overview#entrypoint-vs-entry)
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_basic_organization(
        &self,
        entry: &String,
        options: Option<RequestOptions>,
    ) -> Result<OrganizationQueryRecord, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("Organization/basic/{}", entry),
                None,
                None,
                options,
            )
            .await
    }

    /// Gets an organizations basic details by org ID.
    ///
    /// # Arguments
    ///
    /// * `org_id` - The numeric identifier for organization, assigned by Payabli.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_basic_organization_by_id(
        &self,
        org_id: i64,
        options: Option<RequestOptions>,
    ) -> Result<OrganizationQueryRecord, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("Organization/basicById/{}", org_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Retrieves details for an organization by ID.
    ///
    /// # Arguments
    ///
    /// * `org_id` - The numeric identifier for organization, assigned by Payabli.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_organization(
        &self,
        org_id: i64,
        options: Option<RequestOptions>,
    ) -> Result<OrganizationQueryRecord, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("Organization/read/{}", org_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Retrieves an organization's settings.
    ///
    /// # Arguments
    ///
    /// * `org_id` - The numeric identifier for organization, assigned by Payabli.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_settings_organization(
        &self,
        org_id: i64,
        options: Option<RequestOptions>,
    ) -> Result<SettingsQueryRecord, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("Organization/settings/{}", org_id),
                None,
                None,
                options,
            )
            .await
    }
}
