pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BillQueryResponse {
    /// Summary statistics for the bill query response.
    #[serde(rename = "Summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<BillQueryResponseSummary>,
    /// Array of bill records returned by the query.
    #[serde(rename = "Records")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<BillQueryRecord2>>,
}

impl BillQueryResponse {
    pub fn builder() -> BillQueryResponseBuilder {
        <BillQueryResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BillQueryResponseBuilder {
    summary: Option<BillQueryResponseSummary>,
    records: Option<Vec<BillQueryRecord2>>,
}

impl BillQueryResponseBuilder {
    pub fn summary(mut self, value: BillQueryResponseSummary) -> Self {
        self.summary = Some(value);
        self
    }

    pub fn records(mut self, value: Vec<BillQueryRecord2>) -> Self {
        self.records = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BillQueryResponse`].
    pub fn build(self) -> Result<BillQueryResponse, BuildError> {
        Ok(BillQueryResponse {
            summary: self.summary,
            records: self.records,
        })
    }
}
