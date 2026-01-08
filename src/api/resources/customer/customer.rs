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

    /// Creates a customer in an entrypoint. An identifier is required to create customer records. Change your identifier settings in Settings > Custom Fields in PartnerHub.
    /// If you don't include an identifier, the record is rejected.
    ///
    /// # Arguments
    ///
    /// * `force_customer_creation` - When `true`, the request creates a new customer record, regardless of whether customer identifiers match an existing customer.
    /// * `replace_existing` - Flag indicating to replace existing customer with a new record. Possible values: 0 (don't replace), 1 (replace). Default is `0`.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn add_customer(
        &self,
        entry: &Entrypointfield,
        request: &AddCustomerRequest,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponseCustomerQuery, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("Customer/single/{}", entry.0),
                Some(serde_json::to_value(&request.body).unwrap_or_default()),
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

    /// Delete a customer record.
    ///
    /// # Arguments
    ///
    /// * `customer_id` - Payabli-generated customer ID. Maps to "Customer ID" column in PartnerHub.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn delete_customer(
        &self,
        customer_id: i64,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponse00Responsedatanonobject, ApiError> {
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

    /// Retrieves a customer's record and details.
    ///
    /// # Arguments
    ///
    /// * `customer_id` - Payabli-generated customer ID. Maps to "Customer ID" column in PartnerHub.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_customer(
        &self,
        customer_id: i64,
        options: Option<RequestOptions>,
    ) -> Result<CustomerQueryRecords, ApiError> {
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

    /// Links a customer to a transaction by ID.
    ///
    /// # Arguments
    ///
    /// * `customer_id` - Payabli-generated customer ID. Maps to "Customer ID" column in PartnerHub.
    /// * `trans_id` - ReferenceId for the transaction (PaymentId).
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn link_customer_transaction(
        &self,
        customer_id: i64,
        trans_id: &String,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponse00Responsedatanonobject, ApiError> {
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

    /// Sends the consent opt-in email to the customer email address in the customer record.
    ///
    /// # Arguments
    ///
    /// * `customer_id` - Payabli-generated customer ID. Maps to "Customer ID" column in PartnerHub.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn request_consent(
        &self,
        customer_id: i64,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponse00Responsedatanonobject, ApiError> {
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

    /// Update a customer record. Include only the fields you want to change.
    ///
    /// # Arguments
    ///
    /// * `customer_id` - Payabli-generated customer ID. Maps to "Customer ID" column in PartnerHub.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn update_customer(
        &self,
        customer_id: i64,
        request: &CustomerData,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponse00Responsedatanonobject, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("Customer/{}", customer_id),
                Some(serde_json::to_value(request).unwrap_or_default()),
                None,
                options,
            )
            .await
    }
}
