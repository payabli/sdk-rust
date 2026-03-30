pub use crate::prelude::*;

/// Response payload for queries related to vendors.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryResponseVendors {
    #[serde(rename = "Records")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<VendorQueryRecord>>,
    #[serde(rename = "Summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<QuerySummary>,
}

impl QueryResponseVendors {
    pub fn builder() -> QueryResponseVendorsBuilder {
        <QueryResponseVendorsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryResponseVendorsBuilder {
    records: Option<Vec<VendorQueryRecord>>,
    summary: Option<QuerySummary>,
}

impl QueryResponseVendorsBuilder {
    pub fn records(mut self, value: Vec<VendorQueryRecord>) -> Self {
        self.records = Some(value);
        self
    }

    pub fn summary(mut self, value: QuerySummary) -> Self {
        self.summary = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryResponseVendors`].
    pub fn build(self) -> Result<QueryResponseVendors, BuildError> {
        Ok(QueryResponseVendors {
            records: self.records,
            summary: self.summary,
        })
    }
}
