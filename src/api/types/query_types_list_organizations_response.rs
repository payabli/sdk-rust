pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListOrganizationsResponse {
    #[serde(rename = "Records")]
    pub records: Vec<OrganizationQueryRecord>,
    #[serde(rename = "Summary")]
    pub summary: QuerySummary,
}
