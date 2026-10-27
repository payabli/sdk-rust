pub use crate::prelude::*;

/// Create an organization under an existing business. Provide `businessReference` plus `doingBusinessAs`. All other fields are optional.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateOrganizationWithBusinessReference {
    #[serde(flatten)]
    pub organization_writable_fields_fields: OrganizationWritableFields,
    /// The reference of the existing business the organization belongs to.
    #[serde(rename = "businessReference")]
    #[serde(default)]
    pub business_reference: String,
}

impl CreateOrganizationWithBusinessReference {
    pub fn builder() -> CreateOrganizationWithBusinessReferenceBuilder {
        <CreateOrganizationWithBusinessReferenceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateOrganizationWithBusinessReferenceBuilder {
    organization_writable_fields_fields: Option<OrganizationWritableFields>,
    business_reference: Option<String>,
}

impl CreateOrganizationWithBusinessReferenceBuilder {
    pub fn organization_writable_fields_fields(
        mut self,
        value: OrganizationWritableFields,
    ) -> Self {
        self.organization_writable_fields_fields = Some(value);
        self
    }

    pub fn business_reference(mut self, value: impl Into<String>) -> Self {
        self.business_reference = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateOrganizationWithBusinessReference`].
    /// This method will fail if any of the following fields are not set:
    /// - [`organization_writable_fields_fields`](CreateOrganizationWithBusinessReferenceBuilder::organization_writable_fields_fields)
    /// - [`business_reference`](CreateOrganizationWithBusinessReferenceBuilder::business_reference)
    pub fn build(self) -> Result<CreateOrganizationWithBusinessReference, BuildError> {
        Ok(CreateOrganizationWithBusinessReference {
            organization_writable_fields_fields: self
                .organization_writable_fields_fields
                .ok_or_else(|| BuildError::missing_field("organization_writable_fields_fields"))?,
            business_reference: self
                .business_reference
                .ok_or_else(|| BuildError::missing_field("business_reference"))?,
        })
    }
}
