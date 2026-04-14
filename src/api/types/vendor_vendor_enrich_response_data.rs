pub use crate::prelude::*;

/// Enrichment result details.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct VendorEnrichResponseData {
    /// Unique identifier for this enrichment run. Format: `enrich-{vendorId}-{8-char hex}`.
    #[serde(rename = "enrichmentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enrichment_id: Option<String>,
    /// Final enrichment status. Values are `completed` (vendor is payout-ready), `completed_from_network` (vendor was already enriched in the Payabli vendor network, no AI processing needed), or `insufficient` (all stages ran but the vendor still lacks sufficient payment data).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Stages that ran successfully. A stage is only listed here if it returned a successful response. Failed stages are excluded.
    #[serde(rename = "stagesTriggered")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stages_triggered: Option<Vec<String>>,
    /// `true` if the vendor now has sufficient payment data to process a payout (ACH, card email, or check remit address).
    #[serde(rename = "vendorPayoutReady")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_payout_ready: Option<bool>,
    /// Raw extraction results from the enrichment stages that ran.
    #[serde(rename = "enrichmentData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enrichment_data: Option<VendorEnrichmentData>,
}

impl VendorEnrichResponseData {
    pub fn builder() -> VendorEnrichResponseDataBuilder {
        <VendorEnrichResponseDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VendorEnrichResponseDataBuilder {
    enrichment_id: Option<String>,
    status: Option<String>,
    stages_triggered: Option<Vec<String>>,
    vendor_payout_ready: Option<bool>,
    enrichment_data: Option<VendorEnrichmentData>,
}

impl VendorEnrichResponseDataBuilder {
    pub fn enrichment_id(mut self, value: impl Into<String>) -> Self {
        self.enrichment_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn stages_triggered(mut self, value: Vec<String>) -> Self {
        self.stages_triggered = Some(value);
        self
    }

    pub fn vendor_payout_ready(mut self, value: bool) -> Self {
        self.vendor_payout_ready = Some(value);
        self
    }

    pub fn enrichment_data(mut self, value: VendorEnrichmentData) -> Self {
        self.enrichment_data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VendorEnrichResponseData`].
    pub fn build(self) -> Result<VendorEnrichResponseData, BuildError> {
        Ok(VendorEnrichResponseData {
            enrichment_id: self.enrichment_id,
            status: self.status,
            stages_triggered: self.stages_triggered,
            vendor_payout_ready: self.vendor_payout_ready,
            enrichment_data: self.enrichment_data,
        })
    }
}
