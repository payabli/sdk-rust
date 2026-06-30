pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VendorEnrichRequest {
    /// ID of the vendor to enrich. Must be active and belong to the given entrypoint.
    #[serde(rename = "vendorId")]
    #[serde(default)]
    pub vendor_id: i64,
    /// Enrichment stages to run. Valid values are `invoice_scan` and `web_search`. Stages run in order: invoice scan first, then web search. If the vendor becomes payout-ready after invoice scan, web search is skipped.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<Vec<String>>,
    /// When `true` (the default), extracted data is automatically written to the vendor record. Only empty fields are populated, existing values are never overwritten. When `false`, the vendor record isn't modified. In both cases, `enrichmentData` in the response contains the extracted results. Use `false` for UI flows where users review and confirm changes before applying them with the update vendor endpoint.
    #[serde(rename = "applyEnrichmentData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_enrichment_data: Option<bool>,
    /// When `true`, Payabli schedules an AI outreach call to the vendor if the enrichment stages return insufficient payment acceptance info. The call collects the vendor's preferred payment method and contact email. This is the third enrichment stage and is opt-in at the org level. See the schedule outreach call endpoint for behavior and requirements.
    #[serde(rename = "scheduleCallIfNeeded")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_call_if_needed: Option<bool>,
    /// PDF invoice file, Base64-encoded. Required when `scope` includes `invoice_scan`.
    #[serde(rename = "invoiceFile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_file: Option<FileContent>,
    /// Bill ID to associate with this enrichment request.
    #[serde(rename = "billId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bill_id: Option<i64>,
    /// Payment method to apply if enrichment can't find payment details. Values are `check`, `ach`, or `card`.
    #[serde(rename = "fallbackMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback_method: Option<String>,
}

impl VendorEnrichRequest {
    pub fn builder() -> VendorEnrichRequestBuilder {
        <VendorEnrichRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VendorEnrichRequestBuilder {
    vendor_id: Option<i64>,
    scope: Option<Vec<String>>,
    apply_enrichment_data: Option<bool>,
    schedule_call_if_needed: Option<bool>,
    invoice_file: Option<FileContent>,
    bill_id: Option<i64>,
    fallback_method: Option<String>,
}

impl VendorEnrichRequestBuilder {
    pub fn vendor_id(mut self, value: i64) -> Self {
        self.vendor_id = Some(value);
        self
    }

    pub fn scope(mut self, value: Vec<String>) -> Self {
        self.scope = Some(value);
        self
    }

    pub fn apply_enrichment_data(mut self, value: bool) -> Self {
        self.apply_enrichment_data = Some(value);
        self
    }

    pub fn schedule_call_if_needed(mut self, value: bool) -> Self {
        self.schedule_call_if_needed = Some(value);
        self
    }

    pub fn invoice_file(mut self, value: FileContent) -> Self {
        self.invoice_file = Some(value);
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

    /// Consumes the builder and constructs a [`VendorEnrichRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`vendor_id`](VendorEnrichRequestBuilder::vendor_id)
    pub fn build(self) -> Result<VendorEnrichRequest, BuildError> {
        Ok(VendorEnrichRequest {
            vendor_id: self
                .vendor_id
                .ok_or_else(|| BuildError::missing_field("vendor_id"))?,
            scope: self.scope,
            apply_enrichment_data: self.apply_enrichment_data,
            schedule_call_if_needed: self.schedule_call_if_needed,
            invoice_file: self.invoice_file,
            bill_id: self.bill_id,
            fallback_method: self.fallback_method,
        })
    }
}
