use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct BoardingTemplatesClient {
    pub http_client: HttpClient,
}

impl BoardingTemplatesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists the boarding templates an organization owns, plus templates
    /// inherited from ancestor organizations. Scope the listing with the
    /// required `orgId` query parameter. You can filter by `status`, `name`,
    /// `slug`, `orgName`, and `inherited`, and page through results.
    ///
    /// # Arguments
    ///
    /// * `org_id` - The numeric identifier of the organization to list templates for.
    /// * `status` - Filter by template status.
    /// * `name` - Filter by exact template name.
    /// * `slug` - Filter by exact template slug.
    /// * `org_name` - Filter by exact owning organization name.
    /// * `inherited` - Filter by whether the template is inherited from an ancestor organization.
    /// * `from_record` - The number of records to skip before collecting the result set.
    /// * `limit_record` - The maximum number of records to return. Use `0` or a negative value to return all records.
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
    ///         .boarding_templates
    ///         .list_templates(
    ///             &BoardingTemplatesListTemplatesQueryRequest {
    ///                 org_id: 123,
    ///                 status: None,
    ///                 name: None,
    ///                 slug: None,
    ///                 org_name: None,
    ///                 inherited: None,
    ///                 from_record: None,
    ///                 limit_record: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_templates(
        &self,
        request: &BoardingTemplatesListTemplatesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListTemplatesResponse, ApiError> {
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
                "v2/templates",
                None,
                QueryBuilder::new()
                    .int("orgId", request.org_id.clone())
                    .serialize("status", request.status.clone())
                    .string("name", request.name.clone())
                    .string("slug", request.slug.clone())
                    .string("orgName", request.org_name.clone())
                    .bool("inherited", request.inherited.clone())
                    .int("fromRecord", request.from_record.clone())
                    .int("limitRecord", request.limit_record.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Retrieves a boarding template by UUID, including its current content version.
    ///
    /// # Arguments
    ///
    /// * `uuid` - The template's UUID.
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
    ///         .boarding_templates
    ///         .get_template(&"019ff129-d393-7761-9a31-e5afe85b4081".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_template(
        &self,
        uuid: &str,
        options: Option<RequestOptions>,
    ) -> Result<Template, ApiError> {
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
                &format!("v2/templates/{}", uuid),
                None,
                None,
                options,
            )
            .await
    }

    /// Archives a boarding template by UUID. Archiving is a soft delete: the
    /// template stays readable with `status: archived`, so archiving an
    /// already-archived template returns `409`, not another success. Unknown
    /// templates return `404`.
    ///
    /// # Arguments
    ///
    /// * `uuid` - The template's UUID.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
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
    ///         .boarding_templates
    ///         .archive_template(&"019ff129-d393-7761-9a31-e5afe85b4081".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn archive_template(
        &self,
        uuid: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
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
                Method::DELETE,
                &format!("v2/templates/{}", uuid),
                None,
                None,
                options,
            )
            .await
    }

    /// Retrieves a specific immutable content version of a template.
    ///
    /// # Arguments
    ///
    /// * `uuid` - The template's UUID.
    /// * `version_uuid` - The version's UUID.
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
    ///         .boarding_templates
    ///         .get_template_version(
    ///             &"019ff129-d393-7761-9a31-e5afe85b4081".to_string(),
    ///             &"019ff129-d396-74c4-86da-ae3d082d7ef9".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_template_version(
        &self,
        uuid: &str,
        version_uuid: &str,
        options: Option<RequestOptions>,
    ) -> Result<TemplateVersion, ApiError> {
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
                &format!("v2/templates/{}/versions/{}", uuid, version_uuid),
                None,
                None,
                options,
            )
            .await
    }
}
