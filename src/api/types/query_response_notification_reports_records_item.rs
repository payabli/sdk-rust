pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryResponseNotificationReportsRecordsItem {
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<CreatedAt>,
    /// Unique identifier for the report.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Indicator of whether the report can be downloaded.
    #[serde(rename = "isDownloadable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_downloadable: Option<bool>,
    /// Name of the report.
    #[serde(rename = "reportName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_name: Option<String>,
}

impl QueryResponseNotificationReportsRecordsItem {
    pub fn builder() -> QueryResponseNotificationReportsRecordsItemBuilder {
        <QueryResponseNotificationReportsRecordsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryResponseNotificationReportsRecordsItemBuilder {
    created_at: Option<CreatedAt>,
    id: Option<i64>,
    is_downloadable: Option<bool>,
    report_name: Option<String>,
}

impl QueryResponseNotificationReportsRecordsItemBuilder {
    pub fn created_at(mut self, value: CreatedAt) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn is_downloadable(mut self, value: bool) -> Self {
        self.is_downloadable = Some(value);
        self
    }

    pub fn report_name(mut self, value: impl Into<String>) -> Self {
        self.report_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueryResponseNotificationReportsRecordsItem`].
    pub fn build(self) -> Result<QueryResponseNotificationReportsRecordsItem, BuildError> {
        Ok(QueryResponseNotificationReportsRecordsItem {
            created_at: self.created_at,
            id: self.id,
            is_downloadable: self.is_downloadable,
            report_name: self.report_name,
        })
    }
}
