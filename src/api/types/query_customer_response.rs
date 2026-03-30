pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryCustomerResponse {
    #[serde(rename = "Records")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<CustomerQueryRecords>>,
    #[serde(rename = "Summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<QuerySummary>,
}

impl QueryCustomerResponse {
    pub fn builder() -> QueryCustomerResponseBuilder {
        <QueryCustomerResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryCustomerResponseBuilder {
    records: Option<Vec<CustomerQueryRecords>>,
    summary: Option<QuerySummary>,
}

impl QueryCustomerResponseBuilder {
    pub fn records(mut self, value: Vec<CustomerQueryRecords>) -> Self {
        self.records = Some(value);
        self
    }

    pub fn summary(mut self, value: QuerySummary) -> Self {
        self.summary = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryCustomerResponse`].
    pub fn build(self) -> Result<QueryCustomerResponse, BuildError> {
        Ok(QueryCustomerResponse {
            records: self.records,
            summary: self.summary,
        })
    }
}
