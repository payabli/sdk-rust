pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryResponseNotificationReports {
    #[serde(rename = "Records")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<QueryResponseNotificationReportsRecordsItem>>,
    #[serde(rename = "Summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<QuerySummary>,
}

impl QueryResponseNotificationReports {
    pub fn builder() -> QueryResponseNotificationReportsBuilder {
        <QueryResponseNotificationReportsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryResponseNotificationReportsBuilder {
    records: Option<Vec<QueryResponseNotificationReportsRecordsItem>>,
    summary: Option<QuerySummary>,
}

impl QueryResponseNotificationReportsBuilder {
    pub fn records(mut self, value: Vec<QueryResponseNotificationReportsRecordsItem>) -> Self {
        self.records = Some(value);
        self
    }

    pub fn summary(mut self, value: QuerySummary) -> Self {
        self.summary = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryResponseNotificationReports`].
    pub fn build(self) -> Result<QueryResponseNotificationReports, BuildError> {
        Ok(QueryResponseNotificationReports {
            records: self.records,
            summary: self.summary,
        })
    }
}
