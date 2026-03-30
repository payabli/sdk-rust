pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryInvoiceResponse {
    #[serde(rename = "Records")]
    #[serde(default)]
    pub records: Vec<QueryInvoiceResponseRecordsItem>,
    #[serde(rename = "Summary")]
    #[serde(default)]
    pub summary: QuerySummary,
}

impl QueryInvoiceResponse {
    pub fn builder() -> QueryInvoiceResponseBuilder {
        <QueryInvoiceResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryInvoiceResponseBuilder {
    records: Option<Vec<QueryInvoiceResponseRecordsItem>>,
    summary: Option<QuerySummary>,
}

impl QueryInvoiceResponseBuilder {
    pub fn records(mut self, value: Vec<QueryInvoiceResponseRecordsItem>) -> Self {
        self.records = Some(value);
        self
    }

    pub fn summary(mut self, value: QuerySummary) -> Self {
        self.summary = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryInvoiceResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`records`](QueryInvoiceResponseBuilder::records)
    /// - [`summary`](QueryInvoiceResponseBuilder::summary)
    pub fn build(self) -> Result<QueryInvoiceResponse, BuildError> {
        Ok(QueryInvoiceResponse {
            records: self
                .records
                .ok_or_else(|| BuildError::missing_field("records"))?,
            summary: self
                .summary
                .ok_or_else(|| BuildError::missing_field("summary"))?,
        })
    }
}
