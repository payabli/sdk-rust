pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QuerySummaryNoAmt {
    #[serde(rename = "pageIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_identifier: Option<PageIdentifier>,
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<Pagesize>,
    #[serde(rename = "totalPages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_pages: Option<Totalrecords>,
    #[serde(rename = "totalRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_records: Option<Totalrecords>,
}

impl QuerySummaryNoAmt {
    pub fn builder() -> QuerySummaryNoAmtBuilder {
        <QuerySummaryNoAmtBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QuerySummaryNoAmtBuilder {
    page_identifier: Option<PageIdentifier>,
    page_size: Option<Pagesize>,
    total_pages: Option<Totalrecords>,
    total_records: Option<Totalrecords>,
}

impl QuerySummaryNoAmtBuilder {
    pub fn page_identifier(mut self, value: PageIdentifier) -> Self {
        self.page_identifier = Some(value);
        self
    }

    pub fn page_size(mut self, value: Pagesize) -> Self {
        self.page_size = Some(value);
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

    /// Consumes the builder and constructs a [`QuerySummaryNoAmt`].
    pub fn build(self) -> Result<QuerySummaryNoAmt, BuildError> {
        Ok(QuerySummaryNoAmt {
            page_identifier: self.page_identifier,
            page_size: self.page_size,
            total_pages: self.total_pages,
            total_records: self.total_records,
        })
    }
}
