pub use crate::prelude::*;

/// A page of boarding templates.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListTemplatesResponse {
    /// Pagination metadata for the result set.
    #[serde(rename = "Summary")]
    #[serde(default)]
    pub summary: TemplatesSummary,
    /// The templates on this page.
    #[serde(rename = "Records")]
    #[serde(default)]
    pub records: Vec<Template>,
}

impl ListTemplatesResponse {
    pub fn builder() -> ListTemplatesResponseBuilder {
        <ListTemplatesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListTemplatesResponseBuilder {
    summary: Option<TemplatesSummary>,
    records: Option<Vec<Template>>,
}

impl ListTemplatesResponseBuilder {
    pub fn summary(mut self, value: TemplatesSummary) -> Self {
        self.summary = Some(value);
        self
    }

    pub fn records(mut self, value: Vec<Template>) -> Self {
        self.records = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListTemplatesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`summary`](ListTemplatesResponseBuilder::summary)
    /// - [`records`](ListTemplatesResponseBuilder::records)
    pub fn build(self) -> Result<ListTemplatesResponse, BuildError> {
        Ok(ListTemplatesResponse {
            summary: self
                .summary
                .ok_or_else(|| BuildError::missing_field("summary"))?,
            records: self
                .records
                .ok_or_else(|| BuildError::missing_field("records"))?,
        })
    }
}
