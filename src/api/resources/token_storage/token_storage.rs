use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct TokenStorageClient {
    pub http_client: HttpClient,
}

impl TokenStorageClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Saves a payment method for reuse. This call exchanges sensitive payment information for a token that can be used to process future transactions. The `ReferenceId` value in the response is the `storedMethodId` to use with transactions.
    ///
    /// # Arguments
    ///
    /// * `ach_validation` - When `true`, enables real-time validation of ACH account and routing numbers. This is an add-on feature, contact Payabli for more information.
    /// * `create_anonymous` - When `true`, creates a saved method with no associated customer information. The token will be associated with customer information the first time it's used to make a payment. Defaults to `false`.
    /// * `force_customer_creation` - When `true`, the request creates a new customer record, regardless of whether customer identifiers match an existing customer. Defaults to `false`.
    /// * `temporary` - Creates a temporary, one-time-use token for the payment method that expires in 12 hours. Defaults to `false`.
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
    ///         .token_storage
    ///         .add_method(
    ///             &AddMethodRequest {
    ///                 body: RequestTokenStorage {
    ///                     customer_data: Some(PayorDataRequest {
    ///                         customer_id: Some(CustomerId(4440)),
    ///                         ..Default::default()
    ///                     }),
    ///                     entry_point: Some(Entrypointfield("8cfec329267".to_string())),
    ///                     fallback_auth: Some(true),
    ///                     fallback_auth_amount: Some(100),
    ///                     method_description: Some("Primary Visa card".to_string()),
    ///                     payment_method: Some(RequestTokenStoragePaymentMethod::TokenizeCard(
    ///                         TokenizeCard {
    ///                             method: "card".to_string(),
    ///                             cardcvv: Some(Cardcvv("123".to_string())),
    ///                             cardexp: Cardexp("12/29".to_string()),
    ///                             card_holder: Cardholder("John Doe".to_string()),
    ///                             cardnumber: Cardnumber("4111111111111111".to_string()),
    ///                             cardzip: Some(Cardzip("12345".to_string())),
    ///                             ..Default::default()
    ///                         },
    ///                     )),
    ///                     source: Some(Source("api".to_string())),
    ///                     ..Default::default()
    ///                 },
    ///                 ach_validation: None,
    ///                 create_anonymous: None,
    ///                 force_customer_creation: None,
    ///                 temporary: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn add_method(
        &self,
        request: &AddMethodRequest,
        options: Option<RequestOptions>,
    ) -> Result<AddMethodResponse, ApiError> {
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
                "TokenStorage/add",
                Some(serde_json::to_value(&request.body).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .serialize("achValidation", request.ach_validation.clone())
                    .serialize("createAnonymous", request.create_anonymous.clone())
                    .serialize(
                        "forceCustomerCreation",
                        request.force_customer_creation.clone(),
                    )
                    .serialize("temporary", request.temporary.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Retrieves details for a saved payment method.
    ///
    /// # Arguments
    ///
    /// * `method_id` - The saved payment method ID.
    /// * `card_expiration_format` - Format for card expiration dates in the response.
    ///
    /// Accepted values:
    ///
    /// - 0: default, no formatting. Expiration dates are returned in the format they're saved in.
    ///
    /// - 1: MMYY
    ///
    /// - 2: MM/YY
    /// * `include_temporary` - When `true`, the request will include temporary tokens in the search and return details for a matching temporary token. The default behavior searches only for permanent tokens.
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
    ///         .token_storage
    ///         .get_method(
    ///             &"32-8877drt00045632-678".to_string(),
    ///             &GetMethodQueryRequest {
    ///                 card_expiration_format: Some(1),
    ///                 include_temporary: Some(false),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_method(
        &self,
        method_id: &str,
        request: &GetMethodQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetMethodResponse, ApiError> {
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
                &format!("TokenStorage/{}", method_id),
                None,
                QueryBuilder::new()
                    .int(
                        "cardExpirationFormat",
                        request.card_expiration_format.clone(),
                    )
                    .bool("includeTemporary", request.include_temporary.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Updates a saved payment method.
    ///
    /// # Arguments
    ///
    /// * `method_id` - The saved payment method ID.
    /// * `ach_validation` - When `true`, enables real-time validation of ACH account and routing numbers. This is an add-on feature, contact Payabli for more information.
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
    ///         .token_storage
    ///         .update_method(
    ///             &"32-8877drt00045632-678".to_string(),
    ///             &UpdateMethodRequest {
    ///                 body: RequestTokenStorage {
    ///                     customer_data: Some(PayorDataRequest {
    ///                         customer_id: Some(CustomerId(4440)),
    ///                         ..Default::default()
    ///                     }),
    ///                     entry_point: Some(Entrypointfield("8cfec329267".to_string())),
    ///                     fallback_auth: Some(true),
    ///                     payment_method: Some(RequestTokenStoragePaymentMethod::TokenizeCard(
    ///                         TokenizeCard {
    ///                             method: "card".to_string(),
    ///                             cardcvv: Some(Cardcvv("123".to_string())),
    ///                             cardexp: Cardexp("12/29".to_string()),
    ///                             card_holder: Cardholder("John Doe".to_string()),
    ///                             cardnumber: Cardnumber("4111111111111111".to_string()),
    ///                             cardzip: Some(Cardzip("12345".to_string())),
    ///                             ..Default::default()
    ///                         },
    ///                     )),
    ///                     ..Default::default()
    ///                 },
    ///                 ach_validation: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_method(
        &self,
        method_id: &str,
        request: &UpdateMethodRequest,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponsePaymethodDelete, ApiError> {
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
                &format!("TokenStorage/{}", method_id),
                Some(serde_json::to_value(&request.body).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .serialize("achValidation", request.ach_validation.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Deletes a saved payment method.
    ///
    /// # Arguments
    ///
    /// * `method_id` - The saved payment method ID.
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
    ///         .token_storage
    ///         .remove_method(&"32-8877drt00045632-678".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn remove_method(
        &self,
        method_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponsePaymethodDelete, ApiError> {
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
                &format!("TokenStorage/{}", method_id),
                None,
                None,
                options,
            )
            .await
    }
}
