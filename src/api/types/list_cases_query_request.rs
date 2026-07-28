pub use crate::prelude::*;

/// Query parameters for ListCases
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListCasesQueryRequest {
    /// The zero-based index of the first record to return.
    #[serde(rename = "fromRecord")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_record: Option<i64>,
    /// The maximum number of records to return (1 to 200).
    #[serde(rename = "limitRecord")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_record: Option<i64>,
    /// Sort expression, such as `desc(createdAt)` or `asc(state)`. Defaults to `desc(createdAt)`.
    #[serde(rename = "sortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
}

impl ListCasesQueryRequest {
    pub fn builder() -> ListCasesQueryRequestBuilder {
        <ListCasesQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListCasesQueryRequestBuilder {
    from_record: Option<i64>,
    limit_record: Option<i64>,
    sort_by: Option<String>,
}

impl ListCasesQueryRequestBuilder {
    pub fn from_record(mut self, value: i64) -> Self {
        self.from_record = Some(value);
        self
    }

    pub fn limit_record(mut self, value: i64) -> Self {
        self.limit_record = Some(value);
        self
    }

    pub fn sort_by(mut self, value: impl Into<String>) -> Self {
        self.sort_by = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListCasesQueryRequest`].
    pub fn build(self) -> Result<ListCasesQueryRequest, BuildError> {
        Ok(ListCasesQueryRequest {
            from_record: self.from_record,
            limit_record: self.limit_record,
            sort_by: self.sort_by,
        })
    }
}
