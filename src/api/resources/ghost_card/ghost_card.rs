use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct GhostCardClient {
    pub http_client: HttpClient,
}

impl GhostCardClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Creates a ghost card, a multi-use virtual debit card issued to a vendor for recurring or discretionary spend.
    ///
    /// Unlike single-use virtual cards issued as part of a payout transaction, ghost cards aren't tied to a specific payout. They're issued directly to a vendor and can be reused up to a configurable number of times within the card's spending limits.
    ///
    /// Only one ghost card can exist per vendor per paypoint. To issue a new card to the same vendor, cancel the existing card first.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn create_ghost_card(
        &self,
        entry: &Entry,
        request: &CreateGhostCardRequestBody,
        options: Option<RequestOptions>,
    ) -> Result<CreateGhostCardResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("MoneyOutCard/GhostCard/{}", entry.0),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Updates the status of a virtual card (including ghost cards) under a paypoint.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn update_card(
        &self,
        entry: &Entry,
        request: &UpdateCardRequestBody,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("MoneyOutCard/card/{}", entry.0),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
