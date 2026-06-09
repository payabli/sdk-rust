pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryBoardingLinksResponse {
    #[serde(rename = "Records")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<QueryBoardingLinksResponseRecordsItem>>,
    #[serde(rename = "Summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<QuerySummary>,
}

impl QueryBoardingLinksResponse {
    pub fn builder() -> QueryBoardingLinksResponseBuilder {
        <QueryBoardingLinksResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryBoardingLinksResponseBuilder {
    records: Option<Vec<QueryBoardingLinksResponseRecordsItem>>,
    summary: Option<QuerySummary>,
}

impl QueryBoardingLinksResponseBuilder {
    pub fn records(mut self, value: Vec<QueryBoardingLinksResponseRecordsItem>) -> Self {
        self.records = Some(value);
        self
    }

    pub fn summary(mut self, value: QuerySummary) -> Self {
        self.summary = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryBoardingLinksResponse`].
    pub fn build(self) -> Result<QueryBoardingLinksResponse, BuildError> {
        Ok(QueryBoardingLinksResponse {
            records: self.records,
            summary: self.summary,
        })
    }
}
