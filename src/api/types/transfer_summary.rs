pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TransferSummary {
    #[serde(rename = "totalPages")]
    #[serde(default)]
    pub total_pages: Totalpages,
    #[serde(rename = "totalRecords")]
    #[serde(default)]
    pub total_records: Totalrecords,
    #[serde(rename = "pageSize")]
    #[serde(default)]
    pub page_size: Pagesize,
}

impl TransferSummary {
    pub fn builder() -> TransferSummaryBuilder {
        <TransferSummaryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransferSummaryBuilder {
    total_pages: Option<Totalpages>,
    total_records: Option<Totalrecords>,
    page_size: Option<Pagesize>,
}

impl TransferSummaryBuilder {
    pub fn total_pages(mut self, value: Totalpages) -> Self {
        self.total_pages = Some(value);
        self
    }

    pub fn total_records(mut self, value: Totalrecords) -> Self {
        self.total_records = Some(value);
        self
    }

    pub fn page_size(mut self, value: Pagesize) -> Self {
        self.page_size = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TransferSummary`].
    /// This method will fail if any of the following fields are not set:
    /// - [`total_pages`](TransferSummaryBuilder::total_pages)
    /// - [`total_records`](TransferSummaryBuilder::total_records)
    /// - [`page_size`](TransferSummaryBuilder::page_size)
    pub fn build(self) -> Result<TransferSummary, BuildError> {
        Ok(TransferSummary {
            total_pages: self
                .total_pages
                .ok_or_else(|| BuildError::missing_field("total_pages"))?,
            total_records: self
                .total_records
                .ok_or_else(|| BuildError::missing_field("total_records"))?,
            page_size: self
                .page_size
                .ok_or_else(|| BuildError::missing_field("page_size"))?,
        })
    }
}
