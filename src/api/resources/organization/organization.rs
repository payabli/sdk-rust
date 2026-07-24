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
    ///         .organization
    ///         .add_organization(
    ///             &AddOrganizationRequest {
    ///                 billing_info: Some(Instrument {
    ///                     ach_account: Achaccount("123123123".to_string()),
    ///                     ach_routing: Achrouting("123123123".to_string()),
    ///                     billing_address: Some(BillingAddressNullable("123 Walnut Street".to_string())),
    ///                     billing_city: Some(BillingCityNullable("Johnson City".to_string())),
    ///                     billing_country: Some(BillingCountryNullable("US".to_string())),
    ///                     billing_state: Some(BillingStateNullable("TN".to_string())),
    ///                     billing_zip: Some(BillingZip("37615".to_string())),
    ///                     ..Default::default()
    ///                 }),
    ///                 contacts: Some(ContactsField(vec![Contacts {
    ///                     contact_email: Some(Email("herman@hermanscoatings.com".to_string())),
    ///                     contact_name: Some("Herman Martinez".to_string()),
    ///                     contact_phone: Some("3055550000".to_string()),
    ///                     contact_title: Some("Owner".to_string()),
    ///                     ..Default::default()
    ///                 }])),
    ///                 has_billing: Some(true),
    ///                 has_residual: Some(true),
    ///                 org_address: Some(Orgaddress("123 Walnut Street".to_string())),
    ///                 org_city: Some(Orgcity("Johnson City".to_string())),
    ///                 org_country: Some(Orgcountry("US".to_string())),
    ///                 org_entry_name: Some(Orgentryname("pilgrim-planner".to_string())),
    ///                 org_id: Some(Orgidstring("123".to_string())),
    ///                 org_logo: Some(FileContent {
    ///                     f_content: Some("TXkgdGVzdCBmaWxlHJ==...".to_string()),
    ///                     filename: Some("my-doc.pdf".to_string()),
    ///                     ftype: Some(FileContentFtype::Pdf),
    ///                     furl: Some("https://mysite.com/my-doc.pdf".to_string()),
    ///                     ..Default::default()
    ///                 }),
    ///                 org_name: Orgname("Pilgrim Planner".to_string()),
    ///                 org_parent_id: Some(OrgParentId(236)),
    ///                 org_state: Some(Orgstate("TN".to_string())),
    ///                 org_timezone: Some(Orgtimezone(-5)),
    ///                 org_type: Orgtype(0),
    ///                 org_website: Some(Orgwebsite("www.pilgrimageplanner.com".to_string())),
    ///                 org_zip: Some(Orgzip("37615".to_string())),
    ///                 reply_to_email: ReplyToEmail("email@example.com".to_string()),
    ///                 services: None,
    ///             },
    ///             Some(
    ///                 RequestOptions::new()
    ///                     .additional_header("idempotencyKey", "6B29FC40-CA47-1067-B31D-00DD010662DA"),
    ///             ),
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn add_organization(
        &self,
        request: &AddOrganizationRequest,
        options: Option<RequestOptions>,
    ) -> Result<AddOrganizationResponse, ApiError> {
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
                "Organization",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
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
    ///         .organization
    ///         .edit_organization(
    ///             123,
    ///             &OrganizationData {
    ///                 contacts: Some(ContactsField(vec![Contacts {
    ///                     contact_email: Some(Email("herman@hermanscoatings.com".to_string())),
    ///                     contact_name: Some("Herman Martinez".to_string()),
    ///                     contact_phone: Some("3055550000".to_string()),
    ///                     contact_title: Some("Owner".to_string()),
    ///                     ..Default::default()
    ///                 }])),
    ///                 org_address: Some(Orgaddress("123 Walnut Street".to_string())),
    ///                 org_city: Some(Orgcity("Johnson City".to_string())),
    ///                 org_country: Some(Orgcountry("US".to_string())),
    ///                 org_entry_name: Some(Orgentryname("pilgrim-planner".to_string())),
    ///                 organization_data_org_id: Some(Orgidstring("123".to_string())),
    ///                 org_name: Some(Orgname("Pilgrim Planner".to_string())),
    ///                 org_state: Some(Orgstate("TN".to_string())),
    ///                 org_timezone: Some(Orgtimezone(-5)),
    ///                 org_type: Some(Orgtype(0)),
    ///                 org_website: Some(Orgwebsite("www.pilgrimageplanner.com".to_string())),
    ///                 org_zip: Some(Orgzip("37615".to_string())),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn edit_organization(
        &self,
        org_id: i64,
        request: &OrganizationData,
        options: Option<RequestOptions>,
    ) -> Result<EditOrganizationResponse, ApiError> {
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
                Method::PUT,
                &format!("Organization/{}", org_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
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
    ///     client.organization.delete_organization(123, None).await;
    /// }
    /// ```
    pub async fn delete_organization(
        &self,
        org_id: i64,
        options: Option<RequestOptions>,
    ) -> Result<DeleteOrganizationResponse, ApiError> {
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
                Method::DELETE,
                &format!("Organization/{}", org_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Gets an organization's basic information by entry name (entrypoint identifier).
    ///
    /// # Arguments
    ///
    /// * `entry` - The paypoint's entrypoint identifier. [Learn more](/developers/api-reference/api-overview#entrypoint-vs-entry)
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
    ///         .organization
    ///         .get_basic_organization(&"8cfec329267".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_basic_organization(
        &self,
        entry: &str,
        options: Option<RequestOptions>,
    ) -> Result<OrganizationQueryRecord, ApiError> {
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
                &format!("Organization/basic/{}", entry),
                None,
                None,
                options,
            )
            .await
    }

    /// Gets an organization's basic details by org ID.
    ///
    /// # Arguments
    ///
    /// * `org_id` - The numeric identifier for organization, assigned by Payabli.
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
    ///         .organization
    ///         .get_basic_organization_by_id(123, None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_basic_organization_by_id(
        &self,
        org_id: i64,
        options: Option<RequestOptions>,
    ) -> Result<OrganizationQueryRecord, ApiError> {
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
    ///     client.organization.get_organization(123, None).await;
    /// }
    /// ```
    pub async fn get_organization(
        &self,
        org_id: i64,
        options: Option<RequestOptions>,
    ) -> Result<OrganizationQueryRecord, ApiError> {
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
    ///         .organization
    ///         .get_settings_organization(123, None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_settings_organization(
        &self,
        org_id: i64,
        options: Option<RequestOptions>,
    ) -> Result<SettingsQueryRecord, ApiError> {
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
                &format!("Organization/settings/{}", org_id),
                None,
                None,
                options,
            )
            .await
    }
}
