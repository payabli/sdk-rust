pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
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

impl ApplePayOrganizationUpdateData {
    pub fn builder() -> ApplePayOrganizationUpdateDataBuilder {
        <ApplePayOrganizationUpdateDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApplePayOrganizationUpdateDataBuilder {
    created_at: Option<CreatedAt>,
    id: Option<ApplePayId>,
    job_id: Option<JobId>,
    job_status: Option<JobStatus>,
    organization_id: Option<OrganizationId>,
    r#type: Option<ApplePayType>,
    updated_at: Option<LastModified>,
    updates: Option<OrganizationUpdates>,
}

impl ApplePayOrganizationUpdateDataBuilder {
    pub fn created_at(mut self, value: CreatedAt) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn id(mut self, value: ApplePayId) -> Self {
        self.id = Some(value);
        self
    }

    pub fn job_id(mut self, value: JobId) -> Self {
        self.job_id = Some(value);
        self
    }

    pub fn job_status(mut self, value: JobStatus) -> Self {
        self.job_status = Some(value);
        self
    }

    pub fn organization_id(mut self, value: OrganizationId) -> Self {
        self.organization_id = Some(value);
        self
    }

    pub fn r#type(mut self, value: ApplePayType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn updated_at(mut self, value: LastModified) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn updates(mut self, value: OrganizationUpdates) -> Self {
        self.updates = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ApplePayOrganizationUpdateData`].
    pub fn build(self) -> Result<ApplePayOrganizationUpdateData, BuildError> {
        Ok(ApplePayOrganizationUpdateData {
            created_at: self.created_at,
            id: self.id,
            job_id: self.job_id,
            job_status: self.job_status,
            organization_id: self.organization_id,
            r#type: self.r#type,
            updated_at: self.updated_at,
            updates: self.updates,
        })
    }
}
