pub use crate::prelude::*;

/// Response body for queries about chargebacks.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryChargebacksResponse {
    #[serde(rename = "Records")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<QueryChargebacksResponseRecordsItem>>,
    #[serde(rename = "Summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<QuerySummary>,
}

impl QueryChargebacksResponse {
    pub fn builder() -> QueryChargebacksResponseBuilder {
        <QueryChargebacksResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryChargebacksResponseBuilder {
    records: Option<Vec<QueryChargebacksResponseRecordsItem>>,
    summary: Option<QuerySummary>,
}

impl QueryChargebacksResponseBuilder {
    pub fn records(mut self, value: Vec<QueryChargebacksResponseRecordsItem>) -> Self {
        self.records = Some(value);
        self
    }

    pub fn summary(mut self, value: QuerySummary) -> Self {
        self.summary = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryChargebacksResponse`].
    pub fn build(self) -> Result<QueryChargebacksResponse, BuildError> {
        Ok(QueryChargebacksResponse {
            records: self.records,
            summary: self.summary,
        })
    }
}
