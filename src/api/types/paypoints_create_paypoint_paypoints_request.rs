pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum CreatePaypointPaypointsRequest {
    CreatePaypointWithBusinessReference(CreatePaypointWithBusinessReference),

    CreatePaypointWithInlineBusiness(CreatePaypointWithInlineBusiness),

    CreatePaypointAllFields(CreatePaypointAllFields),
}

impl CreatePaypointPaypointsRequest {
    pub fn is_create_paypoint_with_business_reference(&self) -> bool {
        matches!(self, Self::CreatePaypointWithBusinessReference(_))
    }

    pub fn is_create_paypoint_with_inline_business(&self) -> bool {
        matches!(self, Self::CreatePaypointWithInlineBusiness(_))
    }

    pub fn is_create_paypoint_all_fields(&self) -> bool {
        matches!(self, Self::CreatePaypointAllFields(_))
    }

    pub fn as_create_paypoint_with_business_reference(
        &self,
    ) -> Option<&CreatePaypointWithBusinessReference> {
        match self {
            Self::CreatePaypointWithBusinessReference(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_create_paypoint_with_business_reference(
        self,
    ) -> Option<CreatePaypointWithBusinessReference> {
        match self {
            Self::CreatePaypointWithBusinessReference(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_create_paypoint_with_inline_business(
        &self,
    ) -> Option<&CreatePaypointWithInlineBusiness> {
        match self {
            Self::CreatePaypointWithInlineBusiness(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_create_paypoint_with_inline_business(
        self,
    ) -> Option<CreatePaypointWithInlineBusiness> {
        match self {
            Self::CreatePaypointWithInlineBusiness(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_create_paypoint_all_fields(&self) -> Option<&CreatePaypointAllFields> {
        match self {
            Self::CreatePaypointAllFields(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_create_paypoint_all_fields(self) -> Option<CreatePaypointAllFields> {
        match self {
            Self::CreatePaypointAllFields(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for CreatePaypointPaypointsRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CreatePaypointWithBusinessReference(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::CreatePaypointWithInlineBusiness(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::CreatePaypointAllFields(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
