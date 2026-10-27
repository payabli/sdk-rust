use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct OrganizationsClient {
    pub http_client: HttpClient,
}

impl OrganizationsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists organizations with optional filtering and pagination. Only
    /// organizations are returned. Other paypoints never appear in this list.
    ///
    /// # Arguments
    ///
    /// * `business_reference` - Filter by the business the organization belongs to.
    /// * `status` - Filter by organization status. One of `active`, `inactive`, `suspended`, or `closed`. Any other value returns `400`.
    /// * `search` - Free-text search across organizations.
    /// * `limit` - Number of results per page.
    /// * `page` - Page number to return.
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
    ///         .organizations
    ///         .list_organizations(
    ///             &OrganizationsListOrganizationsQueryRequest {
    ///                 business_reference: Some("6e2b9d14-7a3c-4f58-b1e6-0c9d3a5f8e27".to_string()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_organizations(
        &self,
        request: &OrganizationsListOrganizationsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListPaypointsEnvelope, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(&options, &[&["BearerAuth"] as &[&str]])
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
                "v2/organizations",
                None,
                QueryBuilder::new()
                    .string("businessReference", request.business_reference.clone())
                    .string("status", request.status.clone())
                    .string("search", request.search.clone())
                    .int("limit", request.limit.clone())
                    .int("page", request.page.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Creates an organization. Provide exactly one of `business` (to create
    /// the business inline, recommended) or `businessReference` (an existing
    /// business). In intelligent boarding, an organization is a kind of paypoint, so the
    /// request takes the paypoint create fields except `paypointType`. Payabli
    /// sets the organization type itself. An organization isn't underwritten,
    /// so only the fields needed to create the records are required. New
    /// organizations start in `pending`. If you don't set
    /// `parentPaypointReference`, the organization nests under the
    /// organization your credential belongs to. Returns `409` if an
    /// organization with the same `externalId` already exists.
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
    ///         .organizations
    ///         .create_organization(
    ///             &CreateOrganizationOrganizationsRequest::CreateOrganizationWithInlineBusiness(
    ///                 CreateOrganizationWithInlineBusiness {
    ///                     organization_writable_fields_fields: OrganizationWritableFields {
    ///                         doing_business_as: "PropertyManager Pro".to_string(),
    ///                         description: Some("Property management software partner".to_string()),
    ///                         website: Some("https://example.com".to_string()),
    ///                         customer_service_phone: Some("3055551212".to_string()),
    ///                         customer_service_email: Some("support@example.com".to_string()),
    ///                         timezone: Some("America/New_York".to_string()),
    ///                         external_id: Some("pmp-org-001".to_string()),
    ///                         ..Default::default()
    ///                     },
    ///                     business: CreateBusinessRequest {
    ///                         external_id: Some("pmp-biz-001".to_string()),
    ///                         legal_name: "PropertyManager Pro LLC".to_string(),
    ///                         legal_structure: Some(CreateBusinessRequestLegalStructure::Llc),
    ///                         ..Default::default()
    ///                     },
    ///                     ..Default::default()
    ///                 },
    ///             ),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_organization(
        &self,
        request: &CreateOrganizationOrganizationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<PaypointCreatedEnvelope, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(&options, &[&["BearerAuth"] as &[&str]])
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
                "v2/organizations",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves an organization by reference.
    ///
    /// # Arguments
    ///
    /// * `organization_reference` - The organization reference.
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
    ///         .organizations
    ///         .get_organization(&"a8806f02-23b3-4f6d-bba3-42e1b94326ea".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_organization(
        &self,
        organization_reference: &str,
        options: Option<RequestOptions>,
    ) -> Result<PaypointDetailEnvelope, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(&options, &[&["BearerAuth"] as &[&str]])
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
                &format!("v2/organizations/{}", organization_reference),
                None,
                None,
                options,
            )
            .await
    }

    /// Applies a partial update to an organization. Send only the fields you want to change. You can't change an organization's type. If you send `paypointType`, the request ignores it and doesn't return an error.
    ///
    /// # Arguments
    ///
    /// * `organization_reference` - The organization reference.
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
    ///         .organizations
    ///         .update_organization(
    ///             &"a8806f02-23b3-4f6d-bba3-42e1b94326ea".to_string(),
    ///             &UpdateOrganizationRequest {
    ///                 description: Some("Updated description".to_string()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_organization(
        &self,
        organization_reference: &str,
        request: &UpdateOrganizationRequest,
        options: Option<RequestOptions>,
    ) -> Result<PaypointUpdatedEnvelope, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(&options, &[&["BearerAuth"] as &[&str]])
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
                Method::PATCH,
                &format!("v2/organizations/{}", organization_reference),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
