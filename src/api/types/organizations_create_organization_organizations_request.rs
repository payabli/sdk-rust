pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum CreateOrganizationOrganizationsRequest {
    CreateOrganizationWithInlineBusiness(CreateOrganizationWithInlineBusiness),

    CreateOrganizationWithBusinessReference(CreateOrganizationWithBusinessReference),

    CreateOrganizationAllFields(CreateOrganizationAllFields),
}

impl CreateOrganizationOrganizationsRequest {
    pub fn is_create_organization_with_inline_business(&self) -> bool {
        matches!(self, Self::CreateOrganizationWithInlineBusiness(_))
    }

    pub fn is_create_organization_with_business_reference(&self) -> bool {
        matches!(self, Self::CreateOrganizationWithBusinessReference(_))
    }

    pub fn is_create_organization_all_fields(&self) -> bool {
        matches!(self, Self::CreateOrganizationAllFields(_))
    }

    pub fn as_create_organization_with_inline_business(
        &self,
    ) -> Option<&CreateOrganizationWithInlineBusiness> {
        match self {
            Self::CreateOrganizationWithInlineBusiness(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_create_organization_with_inline_business(
        self,
    ) -> Option<CreateOrganizationWithInlineBusiness> {
        match self {
            Self::CreateOrganizationWithInlineBusiness(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_create_organization_with_business_reference(
        &self,
    ) -> Option<&CreateOrganizationWithBusinessReference> {
        match self {
            Self::CreateOrganizationWithBusinessReference(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_create_organization_with_business_reference(
        self,
    ) -> Option<CreateOrganizationWithBusinessReference> {
        match self {
            Self::CreateOrganizationWithBusinessReference(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_create_organization_all_fields(&self) -> Option<&CreateOrganizationAllFields> {
        match self {
            Self::CreateOrganizationAllFields(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_create_organization_all_fields(self) -> Option<CreateOrganizationAllFields> {
        match self {
            Self::CreateOrganizationAllFields(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for CreateOrganizationOrganizationsRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CreateOrganizationWithInlineBusiness(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::CreateOrganizationWithBusinessReference(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::CreateOrganizationAllFields(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
