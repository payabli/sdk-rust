pub use crate::prelude::*;

/// Query parameters for ListTemplates
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BoardingTemplatesListTemplatesQueryRequest {
    /// The numeric identifier of the organization to list templates for.
    #[serde(rename = "orgId")]
    #[serde(default)]
    pub org_id: i64,
    /// Filter by template status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TemplateStatus>,
    /// Filter by exact template name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Filter by exact template slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    /// Filter by exact owning organization name.
    #[serde(rename = "orgName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_name: Option<String>,
    /// Filter by whether the template is inherited from an ancestor organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inherited: Option<bool>,
    /// The number of records to skip before collecting the result set.
    #[serde(rename = "fromRecord")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_record: Option<i64>,
    /// The maximum number of records to return. Use `0` or a negative value to return all records.
    #[serde(rename = "limitRecord")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_record: Option<i64>,
}

impl BoardingTemplatesListTemplatesQueryRequest {
    pub fn builder() -> BoardingTemplatesListTemplatesQueryRequestBuilder {
        <BoardingTemplatesListTemplatesQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BoardingTemplatesListTemplatesQueryRequestBuilder {
    org_id: Option<i64>,
    status: Option<TemplateStatus>,
    name: Option<String>,
    slug: Option<String>,
    org_name: Option<String>,
    inherited: Option<bool>,
    from_record: Option<i64>,
    limit_record: Option<i64>,
}

impl BoardingTemplatesListTemplatesQueryRequestBuilder {
    pub fn org_id(mut self, value: i64) -> Self {
        self.org_id = Some(value);
        self
    }

    pub fn status(mut self, value: TemplateStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn slug(mut self, value: impl Into<String>) -> Self {
        self.slug = Some(value.into());
        self
    }

    pub fn org_name(mut self, value: impl Into<String>) -> Self {
        self.org_name = Some(value.into());
        self
    }

    pub fn inherited(mut self, value: bool) -> Self {
        self.inherited = Some(value);
        self
    }

    pub fn from_record(mut self, value: i64) -> Self {
        self.from_record = Some(value);
        self
    }

    pub fn limit_record(mut self, value: i64) -> Self {
        self.limit_record = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BoardingTemplatesListTemplatesQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`org_id`](BoardingTemplatesListTemplatesQueryRequestBuilder::org_id)
    pub fn build(self) -> Result<BoardingTemplatesListTemplatesQueryRequest, BuildError> {
        Ok(BoardingTemplatesListTemplatesQueryRequest {
            org_id: self
                .org_id
                .ok_or_else(|| BuildError::missing_field("org_id"))?,
            status: self.status,
            name: self.name,
            slug: self.slug,
            org_name: self.org_name,
            inherited: self.inherited,
            from_record: self.from_record,
            limit_record: self.limit_record,
        })
    }
}
