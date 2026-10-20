use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct PaymentMethodsClient {
    pub http_client: HttpClient,
}

impl PaymentMethodsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists payment methods with optional filtering and pagination.
    ///
    /// # Arguments
    ///
    /// * `account_owner` - Filter by the entity that owns the payment method.
    /// * `owner_type` - Filter by owner type.
    /// * `status` - Filter by payment method status.
    /// * `type_` - Filter by payment method type.
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
    ///         .payment_methods
    ///         .list_payment_methods(
    ///             &ListPaymentMethodsQueryRequest {
    ///                 limit: Some(1),
    ///                 page: Some(1),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_payment_methods(
        &self,
        request: &ListPaymentMethodsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListPaymentMethodsEnvelope, ApiError> {
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
                "v2/payment-methods",
                None,
                QueryBuilder::new()
                    .string("accountOwner", request.account_owner.clone())
                    .serialize("ownerType", request.owner_type.clone())
                    .string("status", request.status.clone())
                    .string("type", request.r#type.clone())
                    .int("limit", request.limit.clone())
                    .int("page", request.page.clone())
                    .string("sortBy", request.sort_by.clone())
                    .string("sortOrder", request.sort_order.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Creates one or more payment methods for an entity in a single call.
    /// Set each payment method's `accountOwner` to the entity it belongs to.
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
    ///         .payment_methods
    ///         .create_payment_methods(
    ///             &CreatePaymentMethodsRequest {
    ///                 data: vec![CreatePaymentMethodRequest {
    ///                     r#type: CreatePaymentMethodRequestType::BankAccount,
    ///                     account_owner: "019fed1a-3d6e-75d3-a81e-cba3874b0a55".to_string(),
    ///                     owner_type: PaymentMethodOwnerType::Business,
    ///                     nickname: Some("Operating account".to_string()),
    ///                     financial_institution: Some("Bank of Example".to_string()),
    ///                     account_type: "checking".to_string(),
    ///                     account_number: "123456789".to_string(),
    ///                     routing_number: "021000021".to_string(),
    ///                     usage: vec![PaymentMethodUsage::Deposits, PaymentMethodUsage::Billing],
    ///                     is_default: None,
    ///                 }],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_payment_methods(
        &self,
        request: &CreatePaymentMethodsRequest,
        options: Option<RequestOptions>,
    ) -> Result<PaymentMethodsCreatedEnvelope, ApiError> {
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
                "v2/payment-methods",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves a payment method by reference. Sensitive fields such as the account and routing numbers are masked.
    ///
    /// # Arguments
    ///
    /// * `payment_method_reference` - The payment method reference.
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
    ///         .payment_methods
    ///         .get_payment_method(&"019ff142-454b-7721-96a3-c8628b9d568f".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_payment_method(
        &self,
        payment_method_reference: &str,
        options: Option<RequestOptions>,
    ) -> Result<PaymentMethodDetailEnvelope, ApiError> {
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
                &format!("v2/payment-methods/{}", payment_method_reference),
                None,
                None,
                options,
            )
            .await
    }

    /// Deactivates a payment method. This is a soft delete: the payment
    /// method moves to `INACTIVE` and the record is kept.
    ///
    /// # Arguments
    ///
    /// * `payment_method_reference` - The payment method reference.
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
    ///         .payment_methods
    ///         .deactivate_payment_method(&"019ff142-454b-7721-96a3-c8628b9d568f".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn deactivate_payment_method(
        &self,
        payment_method_reference: &str,
        options: Option<RequestOptions>,
    ) -> Result<PaymentMethodDeactivatedEnvelope, ApiError> {
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
                &format!("v2/payment-methods/{}", payment_method_reference),
                None,
                None,
                options,
            )
            .await
    }

    /// Applies a partial update to a payment method. Send only the fields you
    /// want to change. Only payment methods in `PENDING` status can be updated.
    /// Once a payment method is active, change it through the bank account
    /// change process instead.
    ///
    /// # Arguments
    ///
    /// * `payment_method_reference` - The payment method reference.
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
    ///         .payment_methods
    ///         .update_payment_method(
    ///             &"019ff142-454b-7721-96a3-c8628b9d568f".to_string(),
    ///             &UpdatePaymentMethodRequest {
    ///                 nickname: Some("Primary operating account".to_string()),
    ///                 usage: Some(vec![
    ///                     PaymentMethodUsage::Deposits,
    ///                     PaymentMethodUsage::Billing,
    ///                 ]),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_payment_method(
        &self,
        payment_method_reference: &str,
        request: &UpdatePaymentMethodRequest,
        options: Option<RequestOptions>,
    ) -> Result<PaymentMethodUpdatedEnvelope, ApiError> {
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
                &format!("v2/payment-methods/{}", payment_method_reference),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
