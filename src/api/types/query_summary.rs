pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QuerySummary {
    #[serde(rename = "pageIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_identifier: Option<PageIdentifier>,
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<Pagesize>,
    /// Total amount for the records.
    #[serde(rename = "totalAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub total_amount: Option<f64>,
    /// Total net amount for the records.
    #[serde(rename = "totalNetAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub total_net_amount: Option<f64>,
    #[serde(rename = "totalPages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_pages: Option<Totalrecords>,
    #[serde(rename = "totalRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_records: Option<Totalrecords>,
}

impl QuerySummary {
    pub fn builder() -> QuerySummaryBuilder {
        <QuerySummaryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QuerySummaryBuilder {
    page_identifier: Option<PageIdentifier>,
    page_size: Option<Pagesize>,
    total_amount: Option<f64>,
    total_net_amount: Option<f64>,
    total_pages: Option<Totalrecords>,
    total_records: Option<Totalrecords>,
}

impl QuerySummaryBuilder {
    pub fn page_identifier(mut self, value: PageIdentifier) -> Self {
        self.page_identifier = Some(value);
        self
    }

    pub fn page_size(mut self, value: Pagesize) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn total_amount(mut self, value: f64) -> Self {
        self.total_amount = Some(value);
        self
    }

    pub fn total_net_amount(mut self, value: f64) -> Self {
        self.total_net_amount = Some(value);
        self
    }

    pub fn total_pages(mut self, value: Totalrecords) -> Self {
        self.total_pages = Some(value);
        self
    }

    pub fn total_records(mut self, value: Totalrecords) -> Self {
        self.total_records = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QuerySummary`].
    pub fn build(self) -> Result<QuerySummary, BuildError> {
        Ok(QuerySummary {
            page_identifier: self.page_identifier,
            page_size: self.page_size,
            total_amount: self.total_amount,
            total_net_amount: self.total_net_amount,
            total_pages: self.total_pages,
            total_records: self.total_records,
        })
    }
}
