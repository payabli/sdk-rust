pub use crate::prelude::*;

/// Response payload for queries related to notifications
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryResponseNotifications {
    #[serde(rename = "Records")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<QueryResponseNotificationsRecordsItem>>,
    #[serde(rename = "Summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<QuerySummary>,
}

impl QueryResponseNotifications {
    pub fn builder() -> QueryResponseNotificationsBuilder {
        <QueryResponseNotificationsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryResponseNotificationsBuilder {
    records: Option<Vec<QueryResponseNotificationsRecordsItem>>,
    summary: Option<QuerySummary>,
}

impl QueryResponseNotificationsBuilder {
    pub fn records(mut self, value: Vec<QueryResponseNotificationsRecordsItem>) -> Self {
        self.records = Some(value);
        self
    }

    pub fn summary(mut self, value: QuerySummary) -> Self {
        self.summary = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryResponseNotifications`].
    pub fn build(self) -> Result<QueryResponseNotifications, BuildError> {
        Ok(QueryResponseNotifications {
            records: self.records,
            summary: self.summary,
        })
    }
}
