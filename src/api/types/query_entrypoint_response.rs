pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryEntrypointResponse {
    #[serde(rename = "Records")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<QueryEntrypointResponseRecordsItem>>,
    #[serde(rename = "Summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<QuerySummary>,
}

impl QueryEntrypointResponse {
    pub fn builder() -> QueryEntrypointResponseBuilder {
        <QueryEntrypointResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryEntrypointResponseBuilder {
    records: Option<Vec<QueryEntrypointResponseRecordsItem>>,
    summary: Option<QuerySummary>,
}

impl QueryEntrypointResponseBuilder {
    pub fn records(mut self, value: Vec<QueryEntrypointResponseRecordsItem>) -> Self {
        self.records = Some(value);
        self
    }

    pub fn summary(mut self, value: QuerySummary) -> Self {
        self.summary = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryEntrypointResponse`].
    pub fn build(self) -> Result<QueryEntrypointResponse, BuildError> {
        Ok(QueryEntrypointResponse {
            records: self.records,
            summary: self.summary,
        })
    }
}
