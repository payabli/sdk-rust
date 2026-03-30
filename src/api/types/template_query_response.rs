pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TemplateQueryResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<TemplateQueryRecord>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<QuerySummary>,
}

impl TemplateQueryResponse {
    pub fn builder() -> TemplateQueryResponseBuilder {
        <TemplateQueryResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TemplateQueryResponseBuilder {
    records: Option<Vec<TemplateQueryRecord>>,
    summary: Option<QuerySummary>,
}

impl TemplateQueryResponseBuilder {
    pub fn records(mut self, value: Vec<TemplateQueryRecord>) -> Self {
        self.records = Some(value);
        self
    }

    pub fn summary(mut self, value: QuerySummary) -> Self {
        self.summary = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TemplateQueryResponse`].
    pub fn build(self) -> Result<TemplateQueryResponse, BuildError> {
        Ok(TemplateQueryResponse {
            records: self.records,
            summary: self.summary,
        })
    }
}
