use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct PaymentMethodDomainClient {
    pub http_client: HttpClient,
}

impl PaymentMethodDomainClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Add a payment method domain to an organization or paypoint.
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
    ///         .payment_method_domain
    ///         .add_payment_method_domain(
    ///             &AddPaymentMethodDomainRequest {
    ///                 apple_pay: Some(AddPaymentMethodDomainRequestApplePay {
    ///                     is_enabled: Some(IsEnabled(true)),
    ///                     ..Default::default()
    ///                 }),
    ///                 google_pay: Some(AddPaymentMethodDomainRequestGooglePay {
    ///                     is_enabled: Some(IsEnabled(true)),
    ///                     ..Default::default()
    ///                 }),
    ///                 domain_name: Some(DomainName("checkout.example.com".to_string())),
    ///                 entity_id: Some(EntityId(109)),
    ///                 entity_type: Some(EntityType("paypoint".to_string())),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn add_payment_method_domain(
        &self,
        request: &AddPaymentMethodDomainRequest,
        options: Option<RequestOptions>,
    ) -> Result<AddPaymentMethodDomainApiResponse, ApiError> {
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
                "PaymentMethodDomain",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Cascades a payment method domain to all child entities. All paypoints and suborganization under this parent will inherit this domain and its settings.
    ///
    /// # Arguments
    ///
    /// * `domain_id` - The payment method domain's ID in Payabli.
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
    ///         .payment_method_domain
    ///         .cascade_payment_method_domain(&"pmd_b8237fa45c964d8a9ef27160cd42b8c5".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn cascade_payment_method_domain(
        &self,
        domain_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<PaymentMethodDomainGeneralResponse, ApiError> {
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
                &format!("PaymentMethodDomain/{}/cascade", domain_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Get the details for a payment method domain.
    ///
    /// # Arguments
    ///
    /// * `domain_id` - The payment method domain's ID in Payabli.
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
    ///         .payment_method_domain
    ///         .get_payment_method_domain(&"pmd_b8237fa45c964d8a9ef27160cd42b8c5".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_payment_method_domain(
        &self,
        domain_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<PaymentMethodDomainApiResponse, ApiError> {
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
                &format!("PaymentMethodDomain/{}", domain_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Delete a payment method domain. You can't delete an inherited domain, you must delete a domain at the organization level.
    ///
    /// # Arguments
    ///
    /// * `domain_id` - The payment method domain's ID in Payabli.
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
    ///         .payment_method_domain
    ///         .delete_payment_method_domain(&"pmd_b8237fa45c964d8a9ef27160cd42b8c5".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_payment_method_domain(
        &self,
        domain_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<DeletePaymentMethodDomainResponse, ApiError> {
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
                &format!("PaymentMethodDomain/{}", domain_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update a payment method domain's configuration values.
    ///
    /// # Arguments
    ///
    /// * `domain_id` - The payment method domain's ID in Payabli.
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
    ///         .payment_method_domain
    ///         .update_payment_method_domain(
    ///             &"pmd_b8237fa45c964d8a9ef27160cd42b8c5".to_string(),
    ///             &UpdatePaymentMethodDomainRequest {
    ///                 apple_pay: Some(UpdatePaymentMethodDomainRequestWallet {
    ///                     is_enabled: Some(IsEnabled(false)),
    ///                     ..Default::default()
    ///                 }),
    ///                 google_pay: Some(UpdatePaymentMethodDomainRequestWallet {
    ///                     is_enabled: Some(IsEnabled(false)),
    ///                     ..Default::default()
    ///                 }),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_payment_method_domain(
        &self,
        domain_id: &str,
        request: &UpdatePaymentMethodDomainRequest,
        options: Option<RequestOptions>,
    ) -> Result<PaymentMethodDomainGeneralResponse, ApiError> {
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
                Method::PATCH,
                &format!("PaymentMethodDomain/{}", domain_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get a list of payment method domains that belong to a PSP, organization, or paypoint.
    ///
    /// # Arguments
    ///
    /// * `entity_id` - Identifier for the organization or paypoint.
    /// - For organization, provide the organization ID - For paypoint, provide the paypoint ID
    /// * `entity_type` - The type of entity. Valid values:
    /// - organization
    /// - paypoint
    /// - psp
    /// * `from_record` - Number of records to skip. Defaults to `0`.
    /// * `limit_record` - Max number of records for query response. Defaults to `20`.
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
    ///         .payment_method_domain
    ///         .list_payment_method_domains(
    ///             &ListPaymentMethodDomainsQueryRequest {
    ///                 entity_id: Some(1147),
    ///                 entity_type: Some("paypoint".to_string()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_payment_method_domains(
        &self,
        request: &ListPaymentMethodDomainsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListPaymentMethodDomainsResponse, ApiError> {
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
                "PaymentMethodDomain/list",
                None,
                QueryBuilder::new()
                    .int("entityId", request.entity_id.clone())
                    .string("entityType", request.entity_type.clone())
                    .int("fromRecord", request.from_record.clone())
                    .int("limitRecord", request.limit_record.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Verify a new payment method domain. If verification is successful, Apple Pay is automatically activated for the domain.
    ///
    /// # Arguments
    ///
    /// * `domain_id` - The payment method domain's ID in Payabli.
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
    ///         .payment_method_domain
    ///         .verify_payment_method_domain(&"pmd_b8237fa45c964d8a9ef27160cd42b8c5".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn verify_payment_method_domain(
        &self,
        domain_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<PaymentMethodDomainGeneralResponse, ApiError> {
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
                &format!("PaymentMethodDomain/{}/verify", domain_id),
                None,
                None,
                options,
            )
            .await
    }
}
