pub use crate::prelude::*;

/// Describes the response for settlement queries.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryResponseSettlements {
    #[serde(rename = "Records")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<QueryResponseSettlementsRecordsItem>>,
    #[serde(rename = "Summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<QueryResponseSettlementsSummary>,
}

impl QueryResponseSettlements {
    pub fn builder() -> QueryResponseSettlementsBuilder {
        <QueryResponseSettlementsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryResponseSettlementsBuilder {
    records: Option<Vec<QueryResponseSettlementsRecordsItem>>,
    summary: Option<QueryResponseSettlementsSummary>,
}

impl QueryResponseSettlementsBuilder {
    pub fn records(mut self, value: Vec<QueryResponseSettlementsRecordsItem>) -> Self {
        self.records = Some(value);
        self
    }

    pub fn summary(mut self, value: QueryResponseSettlementsSummary) -> Self {
        self.summary = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryResponseSettlements`].
    pub fn build(self) -> Result<QueryResponseSettlements, BuildError> {
        Ok(QueryResponseSettlements {
            records: self.records,
            summary: self.summary,
        })
    }
}
