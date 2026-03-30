pub use crate::prelude::*;

/// Payout subscription query response body.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryPayoutSubscriptionResponse {
    #[serde(rename = "Summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<QuerySummary>,
    #[serde(rename = "Records")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<PayoutSubscriptionQueryRecordPascal>>,
}

impl QueryPayoutSubscriptionResponse {
    pub fn builder() -> QueryPayoutSubscriptionResponseBuilder {
        <QueryPayoutSubscriptionResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryPayoutSubscriptionResponseBuilder {
    summary: Option<QuerySummary>,
    records: Option<Vec<PayoutSubscriptionQueryRecordPascal>>,
}

impl QueryPayoutSubscriptionResponseBuilder {
    pub fn summary(mut self, value: QuerySummary) -> Self {
        self.summary = Some(value);
        self
    }

    pub fn records(mut self, value: Vec<PayoutSubscriptionQueryRecordPascal>) -> Self {
        self.records = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryPayoutSubscriptionResponse`].
    pub fn build(self) -> Result<QueryPayoutSubscriptionResponse, BuildError> {
        Ok(QueryPayoutSubscriptionResponse {
            summary: self.summary,
            records: self.records,
        })
    }
}
