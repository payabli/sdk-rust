pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct VCardSummary {
    #[serde(rename = "totalPages")]
    #[serde(default)]
    pub total_pages: Totalpages,
    #[serde(rename = "totalRecords")]
    #[serde(default)]
    pub total_records: Totalrecords,
    /// Total amount for the records.
    #[serde(rename = "totalAmount")]
    #[serde(default)]
    pub total_amount: f64,
    /// Total number of active vCards.
    #[serde(default)]
    pub totalactive: i64,
    /// Total amount of active vCards.
    #[serde(default)]
    pub totalamounteactive: f64,
    /// Total balance of active vCards.
    #[serde(default)]
    pub totalbalanceactive: f64,
    #[serde(rename = "pageIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_identifier: Option<PageIdentifier>,
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<Pagesize>,
}

impl VCardSummary {
    pub fn builder() -> VCardSummaryBuilder {
        <VCardSummaryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VCardSummaryBuilder {
    total_pages: Option<Totalpages>,
    total_records: Option<Totalrecords>,
    total_amount: Option<f64>,
    totalactive: Option<i64>,
    totalamounteactive: Option<f64>,
    totalbalanceactive: Option<f64>,
    page_identifier: Option<PageIdentifier>,
    page_size: Option<Pagesize>,
}

impl VCardSummaryBuilder {
    pub fn total_pages(mut self, value: Totalpages) -> Self {
        self.total_pages = Some(value);
        self
    }

    pub fn total_records(mut self, value: Totalrecords) -> Self {
        self.total_records = Some(value);
        self
    }

    pub fn total_amount(mut self, value: f64) -> Self {
        self.total_amount = Some(value);
        self
    }

    pub fn totalactive(mut self, value: i64) -> Self {
        self.totalactive = Some(value);
        self
    }

    pub fn totalamounteactive(mut self, value: f64) -> Self {
        self.totalamounteactive = Some(value);
        self
    }

    pub fn totalbalanceactive(mut self, value: f64) -> Self {
        self.totalbalanceactive = Some(value);
        self
    }

    pub fn page_identifier(mut self, value: PageIdentifier) -> Self {
        self.page_identifier = Some(value);
        self
    }

    pub fn page_size(mut self, value: Pagesize) -> Self {
        self.page_size = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VCardSummary`].
    /// This method will fail if any of the following fields are not set:
    /// - [`total_pages`](VCardSummaryBuilder::total_pages)
    /// - [`total_records`](VCardSummaryBuilder::total_records)
    /// - [`total_amount`](VCardSummaryBuilder::total_amount)
    /// - [`totalactive`](VCardSummaryBuilder::totalactive)
    /// - [`totalamounteactive`](VCardSummaryBuilder::totalamounteactive)
    /// - [`totalbalanceactive`](VCardSummaryBuilder::totalbalanceactive)
    pub fn build(self) -> Result<VCardSummary, BuildError> {
        Ok(VCardSummary {
            total_pages: self
                .total_pages
                .ok_or_else(|| BuildError::missing_field("total_pages"))?,
            total_records: self
                .total_records
                .ok_or_else(|| BuildError::missing_field("total_records"))?,
            total_amount: self
                .total_amount
                .ok_or_else(|| BuildError::missing_field("total_amount"))?,
            totalactive: self
                .totalactive
                .ok_or_else(|| BuildError::missing_field("totalactive"))?,
            totalamounteactive: self
                .totalamounteactive
                .ok_or_else(|| BuildError::missing_field("totalamounteactive"))?,
            totalbalanceactive: self
                .totalbalanceactive
                .ok_or_else(|| BuildError::missing_field("totalbalanceactive"))?,
            page_identifier: self.page_identifier,
            page_size: self.page_size,
        })
    }
}
