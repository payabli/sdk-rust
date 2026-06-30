use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct VendorClient {
    pub http_client: HttpClient,
}

impl VendorClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Creates a vendor in an entrypoint.
    ///
    /// # Arguments
    ///
    /// * `entry` - Entrypoint identifier.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn add_vendor(
        &self,
        entry: &str,
        request: &VendorData,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponseVendors, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("Vendor/single/{}", entry),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves a vendor's details, including enrichment status and payment acceptance info when available.
    ///
    /// # Arguments
    ///
    /// * `id_vendor` - Vendor ID.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_vendor(
        &self,
        id_vendor: i64,
        options: Option<RequestOptions>,
    ) -> Result<VendorQueryRecord, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("Vendor/{}", id_vendor),
                None,
                None,
                options,
            )
            .await
    }

    /// Updates a vendor's information. Send only the fields you need to update.
    ///
    /// # Arguments
    ///
    /// * `id_vendor` - Vendor ID.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn edit_vendor(
        &self,
        id_vendor: i64,
        request: &VendorData,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponseVendors, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("Vendor/{}", id_vendor),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Delete a vendor.
    ///
    /// # Arguments
    ///
    /// * `id_vendor` - Vendor ID.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn delete_vendor(
        &self,
        id_vendor: i64,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponseVendors, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("Vendor/{}", id_vendor),
                None,
                None,
                options,
            )
            .await
    }

    /// Triggers AI-powered vendor enrichment for an existing vendor. Runs one or more enrichment stages (invoice scan, web search) based on the `scope` parameter. Can automatically apply extracted payment acceptance info and vendor contact information to the vendor record, or return raw results for manual review. Contact Payabli to enable this feature.
    ///
    /// # Arguments
    ///
    /// * `entry` - Entrypoint identifier.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn enrich_vendor(
        &self,
        entry: &str,
        request: &VendorEnrichRequest,
        options: Option<RequestOptions>,
    ) -> Result<VendorEnrichResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("Vendor/enrich/{}", entry),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Schedules an AI outreach call to a vendor to collect their preferred payment method and contact email. This is the third enrichment stage. Calls are scheduled for the next business day at around 9 AM in the vendor's timezone, with retries on no-answer and a fallback payment method applied when retries are exhausted. This feature is opt-in at the org level. Contact your Payabli representative to enable it, provision a phone number, and discuss pricing.
    ///
    /// # Arguments
    ///
    /// * `entry` - Entrypoint identifier.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn schedule_enrichment_call(
        &self,
        entry: &str,
        request: &ScheduleEnrichmentCallRequest,
        options: Option<RequestOptions>,
    ) -> Result<VendorScheduleCallResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("Vendor/enrich/schedule_call/{}", entry),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns the latest AI outreach call activity for a vendor. The response is a composite object with a `state` discriminator (`none`, `scheduled`, `successful`, or `failed`); the block that matches the current state is populated. When the vendor has no call activity, `state` is `none` and the response returns HTTP 200.
    ///
    /// # Arguments
    ///
    /// * `id_vendor` - ID of the vendor to read call status for.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_enrichment_call_status(
        &self,
        id_vendor: i64,
        options: Option<RequestOptions>,
    ) -> Result<VendorCallStatusResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("Vendor/{}/enrichment/call-status", id_vendor),
                None,
                None,
                options,
            )
            .await
    }
}
