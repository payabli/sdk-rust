use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct BillingClient {
    pub http_client: HttpClient,
}

impl BillingClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns every billing profile that belongs to an organization. This is
    /// the data behind the Profile Library table in the Payabli Portal.
    ///
    /// Requires a token with the `billing_profile_read` permission; a token
    /// without it gets `403 Forbidden`.
    ///
    /// # Arguments
    ///
    /// * `org_id` - The organization's numeric identifier.
    /// * `profile_name` - Filter to profiles whose name contains this string.
    /// * `fee_type` - Filter by fee type. Repeatable to match more than one. Send the enum
    /// value (`1` Flat, `2` ICP).
    /// * `service_vertical` - Filter by billing vertical. Repeatable to match more than one. Send
    /// the enum value (`1` PayIn, `2` PayOut, `3` PayOps).
    /// * `profile_id` - Filter to a single profile by its identifier.
    /// * `limit_record` - Page size. Defaults to `20`. Passing `0` returns no records — use a
    /// positive value to page through results.
    /// * `from_record` - Zero-based offset into the result set. Defaults to `0`.
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
    ///         .billing
    ///         .list_profiles(
    ///             123,
    ///             &ListProfilesQueryRequest {
    ///                 limit_record: Some(20),
    ///                 from_record: Some(0),
    ///                 profile_name: None,
    ///                 fee_type: vec![],
    ///                 service_vertical: vec![],
    ///                 profile_id: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_profiles(
        &self,
        org_id: i64,
        request: &ListProfilesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<BillingProfileQueryResponse, ApiError> {
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
                &format!("billing/configuration/org/{}", org_id),
                None,
                QueryBuilder::new()
                    .string("profileName", request.profile_name.clone())
                    .int_array("feeType", request.fee_type.clone())
                    .int_array("serviceVertical", request.service_vertical.clone())
                    .int("profileId", request.profile_id.clone())
                    .int("limitRecord", request.limit_record.clone())
                    .int("fromRecord", request.from_record.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Returns the billing profile currently assigned to an entity, including
    /// its billable events and fee schedules. Use it to read the pricing terms
    /// in effect for an organization, paypoint, template, or application.
    ///
    /// Requires a token with the `billing_profile_read` permission and access
    /// to the requested entity; otherwise the call gets `403 Forbidden`.
    ///
    /// If the entity exists but has no profile assigned, the call returns
    /// `404 Not Found`.
    ///
    /// # Arguments
    ///
    /// * `service_group` - The billing vertical. Only `PayIn` and `PayOut` are accepted; any
    /// other value returns `400 Bad Request`.
    /// * `entity_type` - The owning entity type: `Organization`, `Paypoint`, `Template`, or
    /// `Application`.
    /// * `entity_id` - The numeric identifier of the entity.
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
    ///         .billing
    ///         .get_profile(
    ///             &GetProfileBillingRequestServiceGroup::PayIn,
    ///             &GetProfileBillingRequestEntityType::Organization,
    ///             123,
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_profile(
        &self,
        service_group: &GetProfileBillingRequestServiceGroup,
        entity_type: &GetProfileBillingRequestEntityType,
        entity_id: i64,
        options: Option<RequestOptions>,
    ) -> Result<BillingProfileResponse, ApiError> {
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
                &format!(
                    "billing/configuration/{}/{}/{}",
                    service_group, entity_type, entity_id
                ),
                None,
                None,
                options,
            )
            .await
    }
}
