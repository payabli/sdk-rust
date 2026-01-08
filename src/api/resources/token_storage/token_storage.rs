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
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn add_method(
        &self,
        request: &AddMethodRequest,
        options: Option<RequestOptions>,
    ) -> Result<AddMethodResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "TokenStorage/add",
                Some(serde_json::to_value(&request.body).unwrap_or_default()),
                QueryBuilder::new()
                    .serialize("achValidation", request.ach_validation.clone())
                    .serialize("createAnonymous", Some(request.create_anonymous.clone()))
                    .serialize(
                        "forceCustomerCreation",
                        request.force_customer_creation.clone(),
                    )
                    .serialize("temporary", Some(request.temporary.clone()))
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
    pub async fn get_method(
        &self,
        method_id: &String,
        request: &GetMethodQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetMethodResponse, ApiError> {
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
    pub async fn remove_method(
        &self,
        method_id: &String,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponsePaymethodDelete, ApiError> {
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

    /// Updates a saved payment method.
    ///
    /// # Arguments
    ///
    /// * `method_id` - The saved payment method ID.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn update_method(
        &self,
        method_id: &String,
        request: &UpdateMethodRequest,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponsePaymethodDelete, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("TokenStorage/{}", method_id),
                Some(serde_json::to_value(&request.body).unwrap_or_default()),
                QueryBuilder::new()
                    .serialize("achValidation", request.ach_validation.clone())
                    .build(),
                options,
            )
            .await
    }
}
