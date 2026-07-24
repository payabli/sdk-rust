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
    /// * `entry` - The entity's entrypoint identifier. [Learn more](/developers/api-reference/api-overview#entrypoint-vs-entry)
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
    ///         .ghost_card
    ///         .create_ghost_card(
    ///             &Entry("8cfec329267".to_string()),
    ///             &CreateGhostCardRequestBody {
    ///                 vendor_id: 456,
    ///                 expense_limit: 500.0,
    ///                 amount: 500.0,
    ///                 max_number_of_uses: 3,
    ///                 exact_amount: false,
    ///                 expense_limit_period: "monthly".to_string(),
    ///                 billing_cycle: "monthly".to_string(),
    ///                 billing_cycle_day: "1".to_string(),
    ///                 daily_transaction_count: 5,
    ///                 daily_amount_limit: 200.0,
    ///                 transaction_amount_limit: 100,
    ///                 mcc: Some("5411".to_string()),
    ///                 tcc: Some("R".to_string()),
    ///                 misc_1: Some("PO-98765".to_string()),
    ///                 misc_2: Some("Dept-Finance".to_string()),
    ///                 expiration_date: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_ghost_card(
        &self,
        entry: &Entry,
        request: &CreateGhostCardRequestBody,
        options: Option<RequestOptions>,
    ) -> Result<CreateGhostCardResponse, ApiError> {
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
    /// * `entry` - The entity's entrypoint identifier. [Learn more](/developers/api-reference/api-overview#entrypoint-vs-entry)
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
    ///         .ghost_card
    ///         .update_card(
    ///             &Entry("8cfec329267".to_string()),
    ///             &UpdateCardRequestBody {
    ///                 card_token: "gc_abc123def456".to_string(),
    ///                 status: Some(CardStatus::Cancelled),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_card(
        &self,
        entry: &Entry,
        request: &UpdateCardRequestBody,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponse, ApiError> {
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
                &format!("MoneyOutCard/card/{}", entry.0),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
