pub use crate::prelude::*;

/// All fields accepted when creating an organization. Provide exactly one of `business` or `businessReference`.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateOrganizationAllFields {
    #[serde(flatten)]
    pub organization_writable_fields_fields: OrganizationWritableFields,
    /// The business to create inline for the organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business: Option<CreateBusinessRequest>,
    /// The reference of an existing business the organization belongs to.
    #[serde(rename = "businessReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_reference: Option<String>,
}

impl CreateOrganizationAllFields {
    pub fn builder() -> CreateOrganizationAllFieldsBuilder {
        <CreateOrganizationAllFieldsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateOrganizationAllFieldsBuilder {
    organization_writable_fields_fields: Option<OrganizationWritableFields>,
    business: Option<CreateBusinessRequest>,
    business_reference: Option<String>,
}

impl CreateOrganizationAllFieldsBuilder {
    pub fn organization_writable_fields_fields(
        mut self,
        value: OrganizationWritableFields,
    ) -> Self {
        self.organization_writable_fields_fields = Some(value);
        self
    }

    pub fn business(mut self, value: CreateBusinessRequest) -> Self {
        self.business = Some(value);
        self
    }

    pub fn business_reference(mut self, value: impl Into<String>) -> Self {
        self.business_reference = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateOrganizationAllFields`].
    /// This method will fail if any of the following fields are not set:
    /// - [`organization_writable_fields_fields`](CreateOrganizationAllFieldsBuilder::organization_writable_fields_fields)
    pub fn build(self) -> Result<CreateOrganizationAllFields, BuildError> {
        Ok(CreateOrganizationAllFields {
            organization_writable_fields_fields: self
                .organization_writable_fields_fields
                .ok_or_else(|| BuildError::missing_field("organization_writable_fields_fields"))?,
            business: self.business,
            business_reference: self.business_reference,
        })
    }
}
