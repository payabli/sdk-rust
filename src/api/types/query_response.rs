pub use crate::prelude::*;

///
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
}

impl QueryResponse {
    pub fn builder() -> QueryResponseBuilder {
        <QueryResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryResponseBuilder {
    records: Option<Vec<serde_json::Value>>,
    summary: Option<String>,
}

impl QueryResponseBuilder {
    pub fn records(mut self, value: Vec<serde_json::Value>) -> Self {
        self.records = Some(value);
        self
    }

    pub fn summary(mut self, value: impl Into<String>) -> Self {
        self.summary = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueryResponse`].
    pub fn build(self) -> Result<QueryResponse, BuildError> {
        Ok(QueryResponse {
            records: self.records,
            summary: self.summary,
        })
    }
}
