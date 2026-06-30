pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ScheduleEnrichmentCallRequest {
    /// ID of the vendor to call. Must be active and belong to the entrypoint in the path.
    #[serde(rename = "vendorId")]
    #[serde(default)]
    pub vendor_id: i64,
    /// Vendor phone number to call, digits only. Optional. When omitted, Payabli uses the phone number on the vendor's record. If the vendor has no phone on record, the request returns an error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// ID of the originating enrichment run to associate with this call. Optional. When omitted, Payabli generates a standalone call schedule and skips the enrichment lookup. The bill due-date check only runs when both `enrichmentId` and `billId` are supplied.
    #[serde(rename = "enrichmentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enrichment_id: Option<String>,
    /// Bill ID used for the due-date check. When the bill is due in fewer than three days, the call is skipped and the fallback method is applied. Only evaluated when `enrichmentId` is also supplied.
    #[serde(rename = "billId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bill_id: Option<i64>,
    /// Payment method to apply to the vendor record if the call can't determine a preference or all retries are exhausted. Values are `check` (the default) or `managed`.
    #[serde(rename = "fallbackMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback_method: Option<String>,
    /// Number of times to retry the call if the vendor doesn't answer. Defaults to 3. Maximum is 5. The get outreach call status response reports this value as `maxAttempts`.
    #[serde(rename = "maxRetries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_retries: Option<i64>,
    /// IANA timezone identifier used to schedule the call in the vendor's local time. Defaults to `America/New_York`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// When `true`, dispatches the call immediately and bypasses the business-hours window and the bill due-date check. Defaults to `false`.
    #[serde(rename = "sendNow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_now: Option<bool>,
}

impl ScheduleEnrichmentCallRequest {
    pub fn builder() -> ScheduleEnrichmentCallRequestBuilder {
        <ScheduleEnrichmentCallRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScheduleEnrichmentCallRequestBuilder {
    vendor_id: Option<i64>,
    phone: Option<String>,
    enrichment_id: Option<String>,
    bill_id: Option<i64>,
    fallback_method: Option<String>,
    max_retries: Option<i64>,
    timezone: Option<String>,
    send_now: Option<bool>,
}

impl ScheduleEnrichmentCallRequestBuilder {
    pub fn vendor_id(mut self, value: i64) -> Self {
        self.vendor_id = Some(value);
        self
    }

    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(value.into());
        self
    }

    pub fn enrichment_id(mut self, value: impl Into<String>) -> Self {
        self.enrichment_id = Some(value.into());
        self
    }

    pub fn bill_id(mut self, value: i64) -> Self {
        self.bill_id = Some(value);
        self
    }

    pub fn fallback_method(mut self, value: impl Into<String>) -> Self {
        self.fallback_method = Some(value.into());
        self
    }

    pub fn max_retries(mut self, value: i64) -> Self {
        self.max_retries = Some(value);
        self
    }

    pub fn timezone(mut self, value: impl Into<String>) -> Self {
        self.timezone = Some(value.into());
        self
    }

    pub fn send_now(mut self, value: bool) -> Self {
        self.send_now = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ScheduleEnrichmentCallRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`vendor_id`](ScheduleEnrichmentCallRequestBuilder::vendor_id)
    pub fn build(self) -> Result<ScheduleEnrichmentCallRequest, BuildError> {
        Ok(ScheduleEnrichmentCallRequest {
            vendor_id: self
                .vendor_id
                .ok_or_else(|| BuildError::missing_field("vendor_id"))?,
            phone: self.phone,
            enrichment_id: self.enrichment_id,
            bill_id: self.bill_id,
            fallback_method: self.fallback_method,
            max_retries: self.max_retries,
            timezone: self.timezone,
            send_now: self.send_now,
        })
    }
}
