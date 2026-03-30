pub use crate::prelude::*;

/// Response for line item queries
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryResponseItems {
    #[serde(rename = "Records")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<QueryResponseItemsRecordsItem>>,
    #[serde(rename = "Summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<QuerySummary>,
}

impl QueryResponseItems {
    pub fn builder() -> QueryResponseItemsBuilder {
        <QueryResponseItemsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryResponseItemsBuilder {
    records: Option<Vec<QueryResponseItemsRecordsItem>>,
    summary: Option<QuerySummary>,
}

impl QueryResponseItemsBuilder {
    pub fn records(mut self, value: Vec<QueryResponseItemsRecordsItem>) -> Self {
        self.records = Some(value);
        self
    }

    pub fn summary(mut self, value: QuerySummary) -> Self {
        self.summary = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryResponseItems`].
    pub fn build(self) -> Result<QueryResponseItems, BuildError> {
        Ok(QueryResponseItems {
            records: self.records,
            summary: self.summary,
        })
    }
}
