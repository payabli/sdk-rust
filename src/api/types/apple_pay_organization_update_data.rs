pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ApplePayOrganizationUpdateData {
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<CreatedAt>,
    /// Internal ID for the Apple Pay organization update.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<ApplePayId>,
    #[serde(rename = "jobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<JobId>,
    #[serde(rename = "jobStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<JobStatus>,
    #[serde(rename = "organizationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<OrganizationId>,
    /// The record type, in this context it will always be `ApplePayOrganizationUpdate`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ApplePayType>,
    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<LastModified>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updates: Option<OrganizationUpdates>,
}
