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
    pub async fn add_payment_method_domain(
        &self,
        request: &AddPaymentMethodDomainRequest,
        options: Option<RequestOptions>,
    ) -> Result<AddPaymentMethodDomainApiResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "PaymentMethodDomain",
                Some(serde_json::to_value(request).unwrap_or_default()),
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
    pub async fn cascade_payment_method_domain(
        &self,
        domain_id: &String,
        options: Option<RequestOptions>,
    ) -> Result<PaymentMethodDomainGeneralResponse, ApiError> {
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
    pub async fn delete_payment_method_domain(
        &self,
        domain_id: &String,
        options: Option<RequestOptions>,
    ) -> Result<DeletePaymentMethodDomainResponse, ApiError> {
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
    pub async fn get_payment_method_domain(
        &self,
        domain_id: &String,
        options: Option<RequestOptions>,
    ) -> Result<PaymentMethodDomainApiResponse, ApiError> {
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
    pub async fn list_payment_method_domains(
        &self,
        request: &ListPaymentMethodDomainsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListPaymentMethodDomainsResponse, ApiError> {
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
    pub async fn update_payment_method_domain(
        &self,
        domain_id: &String,
        request: &UpdatePaymentMethodDomainRequest,
        options: Option<RequestOptions>,
    ) -> Result<PaymentMethodDomainGeneralResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("PaymentMethodDomain/{}", domain_id),
                Some(serde_json::to_value(request).unwrap_or_default()),
                None,
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
    pub async fn verify_payment_method_domain(
        &self,
        domain_id: &String,
        options: Option<RequestOptions>,
    ) -> Result<PaymentMethodDomainGeneralResponse, ApiError> {
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
