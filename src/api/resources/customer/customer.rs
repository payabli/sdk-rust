use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct CustomerClient {
    pub http_client: HttpClient,
}

impl CustomerClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Creates a customer in an entrypoint. An identifier is required to create customer records. Change your identifier settings in Settings > Custom Fields in the Payabli Portal.
    /// If you don't include an identifier, the record is rejected.
    ///
    /// # Arguments
    ///
    /// * `entry` - The entrypoint identifier.
    /// * `force_customer_creation` - When `true`, the request creates a new customer record, regardless of whether customer identifiers match an existing customer.
    /// * `replace_existing` - Flag indicating to replace existing customer with a new record. Possible values: 0 (don't replace), 1 (replace). Default is `0`.
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
    ///         .customer
    ///         .add_customer(
    ///             &Entrypointfield("8cfec329267".to_string()),
    ///             &AddCustomerRequest {
    ///                 body: CustomerData {
    ///                     customer_number: Some(CustomerNumberNullable("C-90010".to_string())),
    ///                     firstname: Some("Irene".to_string()),
    ///                     lastname: Some("Canizales".to_string()),
    ///                     email: Some(Email("irene@canizalesconcrete.com".to_string())),
    ///                     address_1: Some("123 Bishop's Trail".to_string()),
    ///                     city: Some("Mountain City".to_string()),
    ///                     state: Some("TN".to_string()),
    ///                     zip: Some("37612".to_string()),
    ///                     country: Some("US".to_string()),
    ///                     time_zone: Some(Timezone(-5)),
    ///                     identifier_fields: Some(Identifierfields(vec!["email".to_string()])),
    ///                     ..Default::default()
    ///                 },
    ///                 force_customer_creation: None,
    ///                 replace_existing: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn add_customer(
        &self,
        entry: &Entrypointfield,
        request: &AddCustomerRequest,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponseCustomerQuery, ApiError> {
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
                &format!("Customer/single/{}", entry.0),
                Some(serde_json::to_value(&request.body).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .bool(
                        "forceCustomerCreation",
                        request.force_customer_creation.clone(),
                    )
                    .int("replaceExisting", request.replace_existing.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Retrieves a customer's record and details.
    ///
    /// # Arguments
    ///
    /// * `customer_id` - Payabli-generated customer ID. Maps to "Customer ID" column in the Payabli Portal.
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
    ///     client.customer.get_customer(4440, None).await;
    /// }
    /// ```
    pub async fn get_customer(
        &self,
        customer_id: i64,
        options: Option<RequestOptions>,
    ) -> Result<CustomerQueryRecords, ApiError> {
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
                &format!("Customer/{}", customer_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update a customer record. Include only the fields you want to change.
    ///
    /// # Arguments
    ///
    /// * `customer_id` - Payabli-generated customer ID. Maps to "Customer ID" column in the Payabli Portal.
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
    ///         .customer
    ///         .update_customer(
    ///             4440,
    ///             &CustomerData {
    ///                 firstname: Some("Irene".to_string()),
    ///                 lastname: Some("Canizales".to_string()),
    ///                 address_1: Some("145 Bishop's Trail".to_string()),
    ///                 city: Some("Mountain City".to_string()),
    ///                 state: Some("TN".to_string()),
    ///                 zip: Some("37612".to_string()),
    ///                 country: Some("US".to_string()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_customer(
        &self,
        customer_id: i64,
        request: &CustomerData,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponse00Responsedatanonobject, ApiError> {
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
                &format!("Customer/{}", customer_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Delete a customer record.
    ///
    /// # Arguments
    ///
    /// * `customer_id` - Payabli-generated customer ID. Maps to "Customer ID" column in the Payabli Portal.
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
    ///     client.customer.delete_customer(4440, None).await;
    /// }
    /// ```
    pub async fn delete_customer(
        &self,
        customer_id: i64,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponse00Responsedatanonobject, ApiError> {
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
                &format!("Customer/{}", customer_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Sends the consent opt-in email to the customer email address in the customer record.
    ///
    /// # Arguments
    ///
    /// * `customer_id` - Payabli-generated customer ID. Maps to "Customer ID" column in the Payabli Portal.
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
    ///     client.customer.request_consent(4440, None).await;
    /// }
    /// ```
    pub async fn request_consent(
        &self,
        customer_id: i64,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponse00Responsedatanonobject, ApiError> {
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
                &format!("Customer/{}/consent", customer_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Links a customer to a transaction by ID.
    ///
    /// # Arguments
    ///
    /// * `customer_id` - Payabli-generated customer ID. Maps to "Customer ID" column in the Payabli Portal.
    /// * `trans_id` - ReferenceId for the transaction (PaymentId).
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
    ///         .customer
    ///         .link_customer_transaction(4440, &"45-as456777hhhhhhhhhh77777777-324".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn link_customer_transaction(
        &self,
        customer_id: i64,
        trans_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponse00Responsedatanonobject, ApiError> {
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
                &format!("Customer/link/{}/{}", customer_id, trans_id),
                None,
                None,
                options,
            )
            .await
    }
}
