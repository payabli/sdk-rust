pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryUserResponse {
    #[serde(rename = "Records")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<UserQueryRecord>>,
    #[serde(rename = "Summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<QuerySummary>,
}

impl QueryUserResponse {
    pub fn builder() -> QueryUserResponseBuilder {
        <QueryUserResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryUserResponseBuilder {
    records: Option<Vec<UserQueryRecord>>,
    summary: Option<QuerySummary>,
}

impl QueryUserResponseBuilder {
    pub fn records(mut self, value: Vec<UserQueryRecord>) -> Self {
        self.records = Some(value);
        self
    }

    pub fn summary(mut self, value: QuerySummary) -> Self {
        self.summary = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryUserResponse`].
    pub fn build(self) -> Result<QueryUserResponse, BuildError> {
        Ok(QueryUserResponse {
            records: self.records,
            summary: self.summary,
        })
    }
}
