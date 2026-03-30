pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryBoardingAppsListResponse {
    #[serde(rename = "Records")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<ApplicationQueryRecord>>,
    #[serde(rename = "Summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<QuerySummary>,
}

impl QueryBoardingAppsListResponse {
    pub fn builder() -> QueryBoardingAppsListResponseBuilder {
        <QueryBoardingAppsListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryBoardingAppsListResponseBuilder {
    records: Option<Vec<ApplicationQueryRecord>>,
    summary: Option<QuerySummary>,
}

impl QueryBoardingAppsListResponseBuilder {
    pub fn records(mut self, value: Vec<ApplicationQueryRecord>) -> Self {
        self.records = Some(value);
        self
    }

    pub fn summary(mut self, value: QuerySummary) -> Self {
        self.summary = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryBoardingAppsListResponse`].
    pub fn build(self) -> Result<QueryBoardingAppsListResponse, BuildError> {
        Ok(QueryBoardingAppsListResponse {
            records: self.records,
            summary: self.summary,
        })
    }
}
