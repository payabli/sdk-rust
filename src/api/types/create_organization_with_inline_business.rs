pub use crate::prelude::*;

/// Create an organization and its business together. Provide `business` plus `doingBusinessAs`. All other fields are optional.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateOrganizationWithInlineBusiness {
    #[serde(flatten)]
    pub organization_writable_fields_fields: OrganizationWritableFields,
    /// The business to create inline for the organization. An organization isn't underwritten, so the business needs only the fields that creating a business requires. Include basic details such as `addressDetails`, `emails`, and `phone` when you have them.
    #[serde(default)]
    pub business: CreateBusinessRequest,
}

impl CreateOrganizationWithInlineBusiness {
    pub fn builder() -> CreateOrganizationWithInlineBusinessBuilder {
        <CreateOrganizationWithInlineBusinessBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateOrganizationWithInlineBusinessBuilder {
    organization_writable_fields_fields: Option<OrganizationWritableFields>,
    business: Option<CreateBusinessRequest>,
}

impl CreateOrganizationWithInlineBusinessBuilder {
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

    /// Consumes the builder and constructs a [`CreateOrganizationWithInlineBusiness`].
    /// This method will fail if any of the following fields are not set:
    /// - [`organization_writable_fields_fields`](CreateOrganizationWithInlineBusinessBuilder::organization_writable_fields_fields)
    /// - [`business`](CreateOrganizationWithInlineBusinessBuilder::business)
    pub fn build(self) -> Result<CreateOrganizationWithInlineBusiness, BuildError> {
        Ok(CreateOrganizationWithInlineBusiness {
            organization_writable_fields_fields: self
                .organization_writable_fields_fields
                .ok_or_else(|| BuildError::missing_field("organization_writable_fields_fields"))?,
            business: self
                .business
                .ok_or_else(|| BuildError::missing_field("business"))?,
        })
    }
}
