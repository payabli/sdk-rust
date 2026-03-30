pub use crate::prelude::*;

/// Subscription query response body.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QuerySubscriptionResponse {
    #[serde(rename = "Records")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<SubscriptionQueryRecords>>,
    #[serde(rename = "Summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<QuerySummary>,
}

impl QuerySubscriptionResponse {
    pub fn builder() -> QuerySubscriptionResponseBuilder {
        <QuerySubscriptionResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QuerySubscriptionResponseBuilder {
    records: Option<Vec<SubscriptionQueryRecords>>,
    summary: Option<QuerySummary>,
}

impl QuerySubscriptionResponseBuilder {
    pub fn records(mut self, value: Vec<SubscriptionQueryRecords>) -> Self {
        self.records = Some(value);
        self
    }

    pub fn summary(mut self, value: QuerySummary) -> Self {
        self.summary = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QuerySubscriptionResponse`].
    pub fn build(self) -> Result<QuerySubscriptionResponse, BuildError> {
        Ok(QuerySubscriptionResponse {
            records: self.records,
            summary: self.summary,
        })
    }
}
