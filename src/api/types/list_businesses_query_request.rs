pub use crate::prelude::*;

/// Query parameters for ListBusinesses
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListBusinessesQueryRequest {
    /// Filter by business status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Filter by legal name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Filter by merchant category code.
    #[serde(rename = "merchantCatCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_cat_code: Option<String>,
    /// Filter by legal structure.
    #[serde(rename = "legalStructure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_structure: Option<String>,
    /// Number of results per page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Page number to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Field to sort by.
    #[serde(rename = "sortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Sort direction.
    #[serde(rename = "sortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

impl ListBusinessesQueryRequest {
    pub fn builder() -> ListBusinessesQueryRequestBuilder {
        <ListBusinessesQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListBusinessesQueryRequestBuilder {
    status: Option<String>,
    name: Option<String>,
    merchant_cat_code: Option<String>,
    legal_structure: Option<String>,
    limit: Option<i64>,
    page: Option<i64>,
    sort_by: Option<String>,
    sort_order: Option<String>,
}

impl ListBusinessesQueryRequestBuilder {
    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn merchant_cat_code(mut self, value: impl Into<String>) -> Self {
        self.merchant_cat_code = Some(value.into());
        self
    }

    pub fn legal_structure(mut self, value: impl Into<String>) -> Self {
        self.legal_structure = Some(value.into());
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn sort_by(mut self, value: impl Into<String>) -> Self {
        self.sort_by = Some(value.into());
        self
    }

    pub fn sort_order(mut self, value: impl Into<String>) -> Self {
        self.sort_order = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListBusinessesQueryRequest`].
    pub fn build(self) -> Result<ListBusinessesQueryRequest, BuildError> {
        Ok(ListBusinessesQueryRequest {
            status: self.status,
            name: self.name,
            merchant_cat_code: self.merchant_cat_code,
            legal_structure: self.legal_structure,
            limit: self.limit,
            page: self.page,
            sort_by: self.sort_by,
            sort_order: self.sort_order,
        })
    }
}
