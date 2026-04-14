pub use crate::prelude::*;

/// Container for enrichment stage results.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct VendorEnrichmentData {
    /// Results from the invoice scan stage, if it ran.
    #[serde(rename = "invoiceScan")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_scan: Option<VendorEnrichmentInvoiceScan>,
    /// Results from the web search stage, if it ran.
    #[serde(rename = "webSearch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_search: Option<VendorEnrichmentWebSearch>,
}

impl VendorEnrichmentData {
    pub fn builder() -> VendorEnrichmentDataBuilder {
        <VendorEnrichmentDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VendorEnrichmentDataBuilder {
    invoice_scan: Option<VendorEnrichmentInvoiceScan>,
    web_search: Option<VendorEnrichmentWebSearch>,
}

impl VendorEnrichmentDataBuilder {
    pub fn invoice_scan(mut self, value: VendorEnrichmentInvoiceScan) -> Self {
        self.invoice_scan = Some(value);
        self
    }

    pub fn web_search(mut self, value: VendorEnrichmentWebSearch) -> Self {
        self.web_search = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VendorEnrichmentData`].
    pub fn build(self) -> Result<VendorEnrichmentData, BuildError> {
        Ok(VendorEnrichmentData {
            invoice_scan: self.invoice_scan,
            web_search: self.web_search,
        })
    }
}
