use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct BusinessesClient {
    pub http_client: HttpClient,
}

impl BusinessesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists businesses with optional filtering and pagination.
    ///
    /// # Arguments
    ///
    /// * `status` - Filter by business status.
    /// * `name` - Filter by legal name.
    /// * `merchant_cat_code` - Filter by merchant category code.
    /// * `legal_structure` - Filter by legal structure.
    /// * `limit` - Number of results per page.
    /// * `page` - Page number to return.
    /// * `sort_by` - Field to sort by.
    /// * `sort_order` - Sort direction.
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
    ///         .businesses
    ///         .list_businesses(
    ///             &ListBusinessesQueryRequest {
    ///                 limit: Some(1),
    ///                 page: Some(1),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_businesses(
        &self,
        request: &ListBusinessesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListBusinessesEnvelope, ApiError> {
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
                "v2/businesses",
                None,
                QueryBuilder::new()
                    .string("status", request.status.clone())
                    .string("name", request.name.clone())
                    .string("merchantCatCode", request.merchant_cat_code.clone())
                    .string("legalStructure", request.legal_structure.clone())
                    .int("limit", request.limit.clone())
                    .int("page", request.page.clone())
                    .string("sortBy", request.sort_by.clone())
                    .string("sortOrder", request.sort_order.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Creates a business, the legal entity behind a paypoint. New businesses
    /// start in `Draft`. Set `parentReference` to the organization
    /// the business belongs to, or omit it to use the organization your
    /// credential belongs to.
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
    ///         .businesses
    ///         .create_business(
    ///             &CreateBusinessRequest {
    ///                 external_id: Some("rg-001".to_string()),
    ///                 legal_name: "Rose Gardens LLC".to_string(),
    ///                 legal_structure: Some(CreateBusinessRequestLegalStructure::Llc),
    ///                 website: Some("https://example.com".to_string()),
    ///                 merchant_cat_code: Some("5814".to_string()),
    ///                 phone: Some(vec!["3055551212".to_string()]),
    ///                 emails: Some(vec!["ops@example.com".to_string()]),
    ///                 payment_methods: Some(vec![
    ///                     BusinessPaymentMethod {
    ///                         r#type: Some("bank_account".to_string()),
    ///                         name: Some("Operating account".to_string()),
    ///                         financial_institution: Some("Bank of Example".to_string()),
    ///                         owner_type: Some("business".to_string()),
    ///                         account_type: Some("checking".to_string()),
    ///                         account_number: Some("123456789".to_string()),
    ///                         routing_number: Some("021000021".to_string()),
    ///                         usage: Some(vec!["deposits".to_string(), "billing".to_string()]),
    ///                         is_default: Some(true),
    ///                         ..Default::default()
    ///                     },
    ///                     BusinessPaymentMethod {
    ///                         r#type: Some("bank_account".to_string()),
    ///                         name: Some("Payout account".to_string()),
    ///                         financial_institution: Some("Bank of Example".to_string()),
    ///                         owner_type: Some("business".to_string()),
    ///                         account_type: Some("checking".to_string()),
    ///                         account_number: Some("987654321".to_string()),
    ///                         routing_number: Some("021000021".to_string()),
    ///                         usage: Some(vec!["payOutFunding".to_string()]),
    ///                         is_default: Some(false),
    ///                         ..Default::default()
    ///                     },
    ///                 ]),
    ///                 tax_reference: Some("477498993".to_string()),
    ///                 parent_reference: Some("8cfec329267".to_string()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_business(
        &self,
        request: &CreateBusinessRequest,
        options: Option<RequestOptions>,
    ) -> Result<BusinessCreatedEnvelope, ApiError> {
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
                "v2/businesses",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves a business by reference. Sensitive fields such as `taxReference` are masked.
    ///
    /// # Arguments
    ///
    /// * `business_reference` - The business reference (`businessUuid`).
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
    ///         .businesses
    ///         .get_business(&"019fed1a-3d6e-75d3-a81e-cba3874b0a55".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_business(
        &self,
        business_reference: &str,
        options: Option<RequestOptions>,
    ) -> Result<BusinessDetailEnvelope, ApiError> {
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
                &format!("v2/businesses/{}", business_reference),
                None,
                None,
                options,
            )
            .await
    }

    /// Deactivates a business. This is a soft delete: the business moves to
    /// `Deactivated` and the record is kept.
    ///
    /// # Arguments
    ///
    /// * `business_reference` - The business reference (`businessUuid`).
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
    ///         .businesses
    ///         .deactivate_business(
    ///             &"019fed33-615d-77b6-afaa-46e7ac9bf36e".to_string(),
    ///             &DeactivateBusinessRequest {
    ///                 reason: Some("Duplicate record".to_string()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn deactivate_business(
        &self,
        business_reference: &str,
        request: &DeactivateBusinessRequest,
        options: Option<RequestOptions>,
    ) -> Result<BusinessDeactivatedEnvelope, ApiError> {
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
                &format!("v2/businesses/{}", business_reference),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Applies a partial update to a business. Send only the fields you want to change.
    ///
    /// # Arguments
    ///
    /// * `business_reference` - The business reference (`businessUuid`).
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
    ///         .businesses
    ///         .update_business(
    ///             &"019fed1a-3d6e-75d3-a81e-cba3874b0a55".to_string(),
    ///             &UpdateBusinessRequest {
    ///                 website: Some("https://example.org".to_string()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_business(
        &self,
        business_reference: &str,
        request: &UpdateBusinessRequest,
        options: Option<RequestOptions>,
    ) -> Result<BusinessUpdatedEnvelope, ApiError> {
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
                &format!("v2/businesses/{}", business_reference),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves a business by reference with sensitive fields unmasked.
    ///
    /// # Arguments
    ///
    /// * `business_reference` - The business reference (`businessUuid`).
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
    ///         .businesses
    ///         .get_business_unmasked(&"019fed1a-3d6e-75d3-a81e-cba3874b0a55".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_business_unmasked(
        &self,
        business_reference: &str,
        options: Option<RequestOptions>,
    ) -> Result<BusinessDetailEnvelope, ApiError> {
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
                &format!("v2/businesses/{}/unmasked", business_reference),
                None,
                None,
                options,
            )
            .await
    }

    /// Lists the addresses on a business. Each address is identified by its `businessAddressReference`.
    ///
    /// # Arguments
    ///
    /// * `business_reference` - The business reference (`businessUuid`).
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
    ///         .businesses
    ///         .list_business_addresses(&"019fed1a-3d6e-75d3-a81e-cba3874b0a55".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn list_business_addresses(
        &self,
        business_reference: &str,
        options: Option<RequestOptions>,
    ) -> Result<BusinessAddressListEnvelope, ApiError> {
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
                &format!("v2/businesses/{}/addresses", business_reference),
                None,
                None,
                options,
            )
            .await
    }

    /// Adds an address to a business. Returns the new address and its `businessAddressReference`.
    ///
    /// # Arguments
    ///
    /// * `business_reference` - The business reference (`businessUuid`).
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
    ///         .businesses
    ///         .create_business_address(
    ///             &"019fed1a-3d6e-75d3-a81e-cba3874b0a55".to_string(),
    ///             &CreateBusinessAddressRequest {
    ///                 r#type: AddressType::Mailing,
    ///                 location: Some(BusinessAddressLocation::Warehouse),
    ///                 address_line_1: "202 Warehouse Ln".to_string(),
    ///                 city_locality: "Austin".to_string(),
    ///                 state_province: "TX".to_string(),
    ///                 postal_code: "73302".to_string(),
    ///                 country: "US".to_string(),
    ///                 address_line_2: None,
    ///                 floor: None,
    ///                 suite_apart_number: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_business_address(
        &self,
        business_reference: &str,
        request: &CreateBusinessAddressRequest,
        options: Option<RequestOptions>,
    ) -> Result<BusinessAddressEnvelope, ApiError> {
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
                &format!("v2/businesses/{}/addresses", business_reference),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves a single business address by its `businessAddressReference`.
    ///
    /// # Arguments
    ///
    /// * `business_reference` - The business reference (`businessUuid`).
    /// * `business_address_reference` - The business address reference.
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
    ///         .businesses
    ///         .get_business_address(
    ///             &"019fed1a-3d6e-75d3-a81e-cba3874b0a55".to_string(),
    ///             &"019fed1a-84a6-7043-a6c0-e8aa2d4bec36".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_business_address(
        &self,
        business_reference: &str,
        business_address_reference: &str,
        options: Option<RequestOptions>,
    ) -> Result<BusinessAddressEnvelope, ApiError> {
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
                &format!(
                    "v2/businesses/{}/addresses/{}",
                    business_reference, business_address_reference
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Removes an address from a business.
    ///
    /// # Arguments
    ///
    /// * `business_reference` - The business reference (`businessUuid`).
    /// * `business_address_reference` - The business address reference.
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
    ///         .businesses
    ///         .delete_business_address(
    ///             &"019fed1a-3d6e-75d3-a81e-cba3874b0a55".to_string(),
    ///             &"019fed1a-67d6-768a-b4cc-29d4299e21d9".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete_business_address(
        &self,
        business_reference: &str,
        business_address_reference: &str,
        options: Option<RequestOptions>,
    ) -> Result<BusinessAddressDeletedEnvelope, ApiError> {
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
                &format!(
                    "v2/businesses/{}/addresses/{}",
                    business_reference, business_address_reference
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Applies a partial update to a business address. Send only the fields you want to change; omitted fields are kept.
    ///
    /// # Arguments
    ///
    /// * `business_reference` - The business reference (`businessUuid`).
    /// * `business_address_reference` - The business address reference.
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
    ///         .businesses
    ///         .update_business_address(
    ///             &"019fed1a-3d6e-75d3-a81e-cba3874b0a55".to_string(),
    ///             &"019fed1a-67d6-768a-b4cc-29d4299e21d9".to_string(),
    ///             &UpdateBusinessAddressRequest {
    ///                 r#type: Some(AddressType::Billing),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_business_address(
        &self,
        business_reference: &str,
        business_address_reference: &str,
        request: &UpdateBusinessAddressRequest,
        options: Option<RequestOptions>,
    ) -> Result<BusinessAddressEnvelope, ApiError> {
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
                &format!(
                    "v2/businesses/{}/addresses/{}",
                    business_reference, business_address_reference
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Lists the people linked to a business, each with the relationship's role, ownership, and relationship-scoped contact details.
    ///
    /// # Arguments
    ///
    /// * `business_reference` - The business reference (`businessUuid`).
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
    ///         .businesses
    ///         .list_business_people(&"019fed1a-3d6e-75d3-a81e-cba3874b0a55".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn list_business_people(
        &self,
        business_reference: &str,
        options: Option<RequestOptions>,
    ) -> Result<BusinessPersonListEnvelope, ApiError> {
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
                &format!("v2/businesses/{}/people", business_reference),
                None,
                None,
                options,
            )
            .await
    }

    /// Links an existing person to a business, setting the relationship's role, ownership, primary-controller flag, contact fields, and addresses.
    ///
    /// # Arguments
    ///
    /// * `business_reference` - The business reference (`businessUuid`).
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
    ///         .businesses
    ///         .link_business_person(
    ///             &"019fed1a-3d6e-75d3-a81e-cba3874b0a55".to_string(),
    ///             &LinkBusinessPersonRequest {
    ///                 person_reference: "01a0014d-ce1a-7110-9bac-c4c59647a88d".to_string(),
    ///                 person_type: Some(PersonType::Owner),
    ///                 ownership_percentage: Some(100.0),
    ///                 is_primary_controller: Some(true),
    ///                 is_signer: Some(true),
    ///                 title: Some("CEO".to_string()),
    ///                 primary_email: Some("contact@example.com".to_string()),
    ///                 secondary_email: Some("contact.alt@example.net".to_string()),
    ///                 primary_phone_number: Some("3055551212".to_string()),
    ///                 secondary_phone_number: Some("3055559876".to_string()),
    ///                 billing_address: Some(PersonAddressRequest {
    ///                     address_line_1: Some("123 Main Street".to_string()),
    ///                     city_locality: Some("Miami".to_string()),
    ///                     state_province: Some("FL".to_string()),
    ///                     postal_code: Some("33101".to_string()),
    ///                     country: Some("US".to_string()),
    ///                     r#type: Some(PersonAddressType::Billing),
    ///                     ..Default::default()
    ///                 }),
    ///                 mailing_address: Some(PersonAddressRequest {
    ///                     address_line_1: Some("456 Oak Avenue".to_string()),
    ///                     city_locality: Some("Miami".to_string()),
    ///                     state_province: Some("FL".to_string()),
    ///                     postal_code: Some("33102".to_string()),
    ///                     country: Some("US".to_string()),
    ///                     r#type: Some(PersonAddressType::Mailing),
    ///                     ..Default::default()
    ///                 }),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn link_business_person(
        &self,
        business_reference: &str,
        request: &LinkBusinessPersonRequest,
        options: Option<RequestOptions>,
    ) -> Result<BusinessPersonEnvelope, ApiError> {
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
                &format!("v2/businesses/{}/people", business_reference),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves a single business-person link by the person's reference.
    ///
    /// # Arguments
    ///
    /// * `business_reference` - The business reference (`businessUuid`).
    /// * `person_reference` - The person reference.
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
    ///         .businesses
    ///         .get_business_person(
    ///             &"019fed1a-3d6e-75d3-a81e-cba3874b0a55".to_string(),
    ///             &"01a0014d-ce1a-7110-9bac-c4c59647a88d".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_business_person(
        &self,
        business_reference: &str,
        person_reference: &str,
        options: Option<RequestOptions>,
    ) -> Result<BusinessPersonEnvelope, ApiError> {
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
                &format!(
                    "v2/businesses/{}/people/{}",
                    business_reference, person_reference
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Removes the link between a person and a business. The person record itself isn't deleted.
    ///
    /// # Arguments
    ///
    /// * `business_reference` - The business reference (`businessUuid`).
    /// * `person_reference` - The person reference.
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
    ///         .businesses
    ///         .unlink_business_person(
    ///             &"019fed1a-3d6e-75d3-a81e-cba3874b0a55".to_string(),
    ///             &"01a0014d-ce1a-7110-9bac-c4c59647a88d".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn unlink_business_person(
        &self,
        business_reference: &str,
        person_reference: &str,
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
                &format!(
                    "v2/businesses/{}/people/{}",
                    business_reference, person_reference
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Applies a partial update to a business-person link. Send only the fields you want to change; omitted fields are kept.
    ///
    /// # Arguments
    ///
    /// * `business_reference` - The business reference (`businessUuid`).
    /// * `person_reference` - The person reference.
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
    ///         .businesses
    ///         .update_business_person(
    ///             &"019fed1a-3d6e-75d3-a81e-cba3874b0a55".to_string(),
    ///             &"01a0014d-ce1a-7110-9bac-c4c59647a88d".to_string(),
    ///             &UpdateBusinessPersonRequest {
    ///                 is_primary_controller: Some(true),
    ///                 primary_email: Some("updated.contact@example.com".to_string()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_business_person(
        &self,
        business_reference: &str,
        person_reference: &str,
        request: &UpdateBusinessPersonRequest,
        options: Option<RequestOptions>,
    ) -> Result<BusinessPersonEnvelope, ApiError> {
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
                &format!(
                    "v2/businesses/{}/people/{}",
                    business_reference, person_reference
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
