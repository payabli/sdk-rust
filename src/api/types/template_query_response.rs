pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TemplateQueryResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<TemplateQueryRecord>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<QuerySummary>,
}
